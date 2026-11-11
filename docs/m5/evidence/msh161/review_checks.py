#!/usr/bin/env python3
"""Run a frozen completed-batch adjudication plan, through notify_exit.py.

No automatic chess claims or retries. Existing analysis owns locks, process bounds,
raw UCI, full native history and resource receipts. Discovery runs unrestricted
and played, then matches the discovered alternative with a singleton query.
"""
import argparse
import json
import subprocess
import time
from pathlib import Path
import chess
import analysis
import main_batch
import screen_v1

HERE = Path(__file__).resolve().parent


def preflight(plan):
    reports, seen = {}, set()
    for row in plan:
        path = str(Path(row['report']).resolve())
        if path not in reports:
            decisions, _ = analysis.extract_decisions(analysis.load_report(path))
            reports[path] = {(d['game'], d['ply']): d for d in decisions}
        key = (path, row['game'], row['ply'])
        if key in seen:
            raise ValueError('Duplicate decision: ' + str(key))
        seen.add(key)
        decision = reports[path][key[1:]]
        board = chess.Board(decision['initial_fen'])
        for move in decision['history']:
            board.push_uci(move)
        moves = row.get('moves', [])
        if not row.get('discovery') and decision['played'] not in moves:
            raise ValueError('Matched plan omits played move')
        for move in moves:
            if chess.Move.from_uci(move) not in board.legal_moves:
                raise ValueError('Illegal adjudication choice: ' + str(key))
    return {path: analysis.digest(path) for path in reports}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('plan')
    parser.add_argument('output')
    parser.add_argument('--preflight-only', action='store_true')
    args = parser.parse_args()
    planpath = Path(args.plan).resolve()
    plan = json.loads(planpath.read_text())
    freeze = json.loads((HERE/'main-freeze.json').read_text())
    main_batch.verify(freeze)
    hashes = preflight(plan)
    if args.preflight_only:
        print(json.dumps({'decisions': len(plan), 'reports': len(hashes), 'legal': True}))
        return
    out = Path(args.output).resolve()
    out.mkdir(exist_ok=False)
    state = {'status': 'running', 'started_epoch': time.time(), 'nodes': 20000000,
             'script_sha256': analysis.digest(__file__), 'plan_sha256': analysis.digest(planpath),
             'freeze_sha256': analysis.digest(HERE/'main-freeze.json'),
             'report_hashes': hashes, 'completed': [], 'results': []}
    def save():
        analysis.save(out/'chain.json', state)
    def query(row, index, move):
        main_batch.verify(freeze)
        assert analysis.digest(planpath) == state['plan_sha256']
        assert analysis.digest(row['report']) == hashes[str(Path(row['report']).resolve())]
        dest = out/f'q{index:04d}-{move or "discovery"}'
        command = [main_batch.PYTHON, str(HERE/'analysis.py'), row['report'], str(dest),
                   '--game', str(row['game']), '--ply', str(row['ply']), '--nodes', '20000000']
        if move:
            command += ['--move', move]
        state['inflight'] = {'command': command, 'started_epoch': time.time()}
        save()
        start = time.monotonic()
        with Path(str(dest)+'.stdout.log').open('x') as stdout, Path(str(dest)+'.stderr.log').open('x') as stderr:
            code = subprocess.call(command, stdout=stdout, stderr=stderr)
        state['completed'].append({'command': command, 'returncode': code, 'wall_seconds': time.monotonic()-start})
        state.pop('inflight', None)
        save()
        if code or (dest/'inflight.json').exists():
            raise RuntimeError('Query failed or unsettled; reconcile preserved state before retry')
        receipts = []
        for path in sorted(dest.glob('g*-attempt*.json')):
            value = json.loads(path.read_text())
            assert value['status'] == 'complete' and value['process_group_settled']
            receipts.append({'receipt': str(path), 'sha256': analysis.digest(path),
                             'kind': value['query'].rsplit('-', 1)[-1],
                             'finite_evidence': screen_v1.describe(value)})
        assert len(receipts) == (1 if move else 2)
        return receipts
    save()
    try:
        for index, row in enumerate(plan):
            receipts, moves = [], list(dict.fromkeys(row.get('moves', [])))
            if row.get('discovery'):
                receipts = query(row, index, None)
                discovered = next(r for r in receipts if r['kind'] == 'unrestricted')['finite_evidence']['bestmove']
                played = next(r for r in receipts if r['kind'] == 'played')['finite_evidence']['bestmove']
                moves = list(dict.fromkeys(moves + ([discovered] if discovered != played else [])))
                moves = [move for move in moves if move != played]
            for move in moves:
                receipts += query(row, index, move)
            state['results'].append({'decision': row, 'queries': receipts, 'requires_independent_adjudication': True})
            save()
        state['status'] = 'completed_needs_independent_review'
    except BaseException as exc:
        state.update(status='failed_needs_review', error=repr(exc))
        raise
    finally:
        state['ended_epoch'] = time.time()
        save()


if __name__ == '__main__':
    main()
