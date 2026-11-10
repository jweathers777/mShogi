#!/usr/bin/env python3
"""MSH-161 all-move reference analysis; native report replay is a prerequisite.

Example (use an existing python-chess environment):
  python analysis.py report.json.gz output-directory
  python analysis.py report.json.gz confirmation --game 0 --ply 34 --nodes 5000000
  python analysis.py report.json.gz alternative --game 0 --ply 34 --move e2e4 --nodes 5000000

Ply is zero-based from the canonical initial position, INCLUDING prepared moves.
No engine execution occurs at import. This is an offline diagnostic consumer,
not a match/rules authority. Never run concurrently with competitive games.
"""
import argparse
import contextlib
import datetime
import fcntl
import gzip
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import selectors
import signal
import subprocess
import time

HERE = Path(__file__).resolve().parent
DEFAULT_ENGINE = '/private/tmp/msh49/opponent/stockfish/stockfish-macos-universal'
START_FEN = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
MOVE = re.compile(r'^[a-h][1-8][a-h][1-8][qrbn]?$')
MAX_REPORT = 128 * 1024 * 1024
RAW_LIMIT = 2 * 1024 * 1024
OPTIONS = {'Threads': 1, 'Hash': 64, 'UCI_ShowWDL': 'true',
           'UCI_LimitStrength': 'false', 'Skill Level': 20, 'Ponder': 'false'}


def digest(path):
    h = hashlib.sha256()
    with open(path, 'rb') as f:
        for block in iter(lambda: f.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def save(path, value):
    path = Path(path)
    temporary = path.with_name(path.name + '.tmp')
    with temporary.open('w') as f:
        json.dump(value, f, sort_keys=True, indent=2)
        f.write('\n')
        f.flush()
        os.fsync(f.fileno())
    os.replace(temporary, path)


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError('Duplicate JSON key: ' + key)
        result[key] = value
    return result


def decode(value):
    return json.loads(value, object_pairs_hook=unique_object) if isinstance(value, (str, bytes)) else value


def load_report(path):
    opener = gzip.open if str(path).endswith('.gz') else open
    with opener(path, 'rb') as f:
        raw = f.read(MAX_REPORT + 1)
    if len(raw) > MAX_REPORT:
        raise ValueError('Report exceeds bounded decoder size')
    outer = decode(raw)
    if 'report_json' in outer:
        nested = outer['report_json']
        if 'report_sha256' in outer and isinstance(nested, str):
            if hashlib.sha256(nested.encode()).hexdigest() != outer['report_sha256']:
                raise ValueError('Wrapper report hash mismatch')
        return decode(nested)
    return outer


def check_move(move):
    if not isinstance(move, str) or not MOVE.fullmatch(move):
        raise ValueError('Invalid coordinate action: ' + repr(move))
    return move


def extract_decisions(report):
    """Extract accepted native actions; never treat intended draw claims as played."""
    if report.get('schema_version') != 1:
        raise ValueError('Unknown report schema')
    decisions, games = [], []
    for gi, game in enumerate(report['games']):
        if not game.get('match_record_json'):
            games.append({'game': gi, 'stop': game.get('stop'), 'failure': game.get('failure'),
                          'decisions': 0, 'missing_match_record': True})
            continue
        record = decode(game['match_record_json'])
        initial = decode(record['initial_record_json'])
        if record.get('rules_profile') != 'mshogi.chess.match.v1' or initial.get('rules_profile') != record['rules_profile']:
            raise ValueError('Unsupported rules profile')
        fen = initial['initial_fen']
        fields = fen.split()
        if len(fields) != 6 or fields[1] not in ('w', 'b'):
            raise ValueError('Malformed initial FEN')
        opening = report['config']['openings'][game['opening_index']]
        if (opening.get('fen') or START_FEN) != fen:
            raise ValueError('Opening FEN and canonical initial FEN disagree')
        initial_side = 0 if fields[1] == 'w' else 1
        history = []
        start = len(decisions)
        def append(move, origin, event_index=None):
            check_move(move)
            side = (initial_side + len(history)) % 2
            decisions.append({'game': gi, 'opening_id': game['opening_id'],
                              'pairing': game['pairing'], 'ply': len(history),
                              'initial_fen': fen, 'history': history.copy(),
                              'history_origin': initial.get('history_origin'),
                              'mover': ('white', 'black')[side],
                              'engine_index': None if origin == 'prepared' else game['engines_by_color'][side],
                              'origin': origin, 'played': move, 'native_event': event_index})
            history.append(move)
        for event in initial['events']:
            if event['ply'] != len(history) or event['event'].get('kind') != 'move':
                raise ValueError('Prepared canonical history must contain consecutive moves')
            append(event['event']['action'], 'prepared')
        if history != opening['moves']:
            raise ValueError('Prepared canonical history differs from configuration')
        rejected = []
        for ei, event in enumerate(record['events']):
            command = event['request']['command']
            if event['reply'].get('rejection') is not None:
                rejected.append(ei)
                continue
            if isinstance(command, dict) and 'move' in command:
                expected = ('white', 'black')[(initial_side + len(history)) % 2]
                if event['request']['actor'] != expected:
                    raise ValueError('Native accepted move actor disagrees with history')
                append(command['move'], 'played', ei)
        games.append({'game': gi, 'opening_id': game['opening_id'], 'pairing': game['pairing'],
                      'stop': game.get('stop'), 'conclusion': game.get('conclusion'),
                      'failure': game.get('failure'), 'rejected_events': rejected,
                      'decisions': len(decisions) - start, 'prepared_plies': len(opening['moves']),
                      'complete_history': history, 'initial_fen': fen})
    return decisions, games


def extract_roots(values):
    """Balance-screen input: [{id, fen: null|FEN, moves: complete-prefix}]."""
    rows = []
    for index, value in enumerate(values):
        history = [check_move(x) for x in value['moves']]
        fen = value.get('fen') or START_FEN
        fields = fen.split()
        if len(fields) != 6 or fields[1] not in ('w', 'b'):
            raise ValueError('Malformed root FEN')
        side = ((0 if fields[1] == 'w' else 1) + len(history)) % 2
        rows.append({'game': index, 'opening_id': value['id'], 'pairing': None,
                     'ply': len(history), 'initial_fen': fen, 'history': history,
                     'history_origin': 'standard_start' if not value.get('fen') else 'imported',
                     'mover': ('white', 'black')[side], 'engine_index': None,
                     'origin': 'supplied_root', 'played': None, 'native_event': None})
    if len({r['opening_id'] for r in rows}) != len(rows):
        raise ValueError('Duplicate supplied root ID')
    return rows, []


def parse_info(line):
    """One aligned info line only: never combine score/PV/WDL across iterations."""
    words = line.split()
    if not words or words[0] != 'info' or 'score' not in words or 'pv' not in words:
        return None
    try:
        i = words.index('score')
        kind, value = words[i + 1], int(words[i + 2])
        if kind not in ('cp', 'mate'):
            return None
        result = {'score': {'kind': kind, 'value': value}, 'pv': words[words.index('pv') + 1:],
                  'lowerbound': 'lowerbound' in words, 'upperbound': 'upperbound' in words}
        for key in ('depth', 'seldepth', 'nodes', 'time', 'multipv', 'tbhits', 'hashfull', 'nps'):
            if key in words:
                result[key] = int(words[words.index(key) + 1])
        if 'wdl' in words:
            i = words.index('wdl')
            result['wdl'] = [int(x) for x in words[i + 1:i + 4]]
            if len(result['wdl']) != 3 or sum(result['wdl']) != 1000 or min(result['wdl']) < 0:
                raise ValueError('Invalid WDL triple')
        if result.get('multipv', 1) != 1 or not result['pv']:
            return None
        return result
    except (ValueError, IndexError):
        return None


def expected_score(wdl, mover='white', perspective='mover'):
    value = (wdl[0] + wdl[1] / 2) / 1000
    if perspective == 'mover' or perspective == mover:
        return value
    if perspective not in ('white', 'black'):
        raise ValueError('Unknown score perspective')
    return 1 - value


def select_score(infos, bestmove, forced=None):
    aligned = [x for x in infos if x['pv'][0] == bestmove
               and not x['lowerbound'] and not x['upperbound']]
    result = {'bestmove': bestmove, 'score': aligned[-1] if aligned else None, 'uncertainty': []}
    if not aligned:
        result['uncertainty'].append('no_nonbound_score_aligned_with_bestmove')
    elif aligned[-1] is not infos[-1]:
        result['uncertainty'].append('last_score_update_not_selected_exact_score')
    if forced and bestmove != forced:
        result['uncertainty'].append('forced_move_not_returned')
    if result['score'] is not None:
        if 'wdl' not in result['score']:
            result['uncertainty'].append('missing_wdl')
        else:
            result['expected_score_mover'] = expected_score(result['score']['wdl'])
    return result


def compare(unrestricted, played):
    identity_keys = ('engine_sha256', 'nodes_requested', 'options')
    root_keys = ('initial_fen', 'history', 'mover')
    if (any(k not in unrestricted or k not in played or unrestricted[k] != played[k] for k in identity_keys)
            or any(k not in unrestricted.get('decision', {}) or k not in played.get('decision', {})
                   or unrestricted['decision'][k] != played['decision'][k] for k in root_keys)):
        return {'loss': None, 'uncertain': True, 'reason': 'comparison_identity_mismatch_or_missing'}
    a, b = unrestricted.get('assessment', {}), played.get('assessment', {})
    if any(x.get('status') != 'complete' for x in (unrestricted, played)) or any(x.get('uncertainty') for x in (a, b)):
        return {'loss': None, 'uncertain': True}
    if 'expected_score_mover' not in a or 'expected_score_mover' not in b:
        return {'loss': None, 'uncertain': True}
    loss = a['expected_score_mover'] - b['expected_score_mover']
    return {'loss': loss, 'uncertain': loss < 0, 'negative_loss_preserved': loss < 0}


def board_for(decision):
    import chess  # Independent diagnostic validation; native replay owns legality/outcome.
    board = chess.Board(decision['initial_fen'])
    for move in decision['history']:
        board.push_uci(move)
    return board


def query(binary, decision, nodes, forced, inflight_path, timeout=60):
    started, usage0 = time.monotonic(), resource.getrusage(resource.RUSAGE_CHILDREN)
    receipt = {'schema': 1, 'started_utc': utc(), 'decision': decision, 'nodes_requested': nodes,
               'forced_move': forced, 'score_perspective': 'root_side_to_move',
               'status': 'failed', 'raw_uci': [], 'raw_truncated': False, 'options': OPTIONS}
    raw_bytes, info_bytes, infos, process, selector = 0, 0, [], None, selectors.DefaultSelector()
    buffers = {'stdout': b'', 'stderr': b''}
    def retain(direction, line):
        nonlocal raw_bytes
        size = len(line.encode()) + len(direction) + 32
        if raw_bytes + size <= RAW_LIMIT:
            receipt['raw_uci'].append({'direction': direction, 'line': line})
            raw_bytes += size
        else:
            receipt['raw_truncated'] = True
    def send(line):
        if len(line.encode()) > 8192:
            raise ValueError('Command exceeds bounded size')
        retain('send', line)
        process.stdin.write((line + '\n').encode())
        process.stdin.flush()
    def wait_for(token):
        nonlocal info_bytes
        while True:
            remaining = timeout - (time.monotonic() - started)
            if remaining <= 0:
                raise TimeoutError('Per-query wall limit expired')
            for key, _ in selector.select(min(remaining, 0.2)):
                chunk = os.read(key.fileobj.fileno(), 65536)
                name = key.data
                if not chunk:
                    selector.unregister(key.fileobj)
                    if name == 'stdout':
                        raise RuntimeError('Engine stdout closed before ' + token)
                    continue
                buffers[name] += chunk
                if b'\n' not in buffers[name] and len(buffers[name]) > 65536:
                    raise ValueError('Engine line exceeds bounded size')
                while b'\n' in buffers[name]:
                    line, buffers[name] = buffers[name].split(b'\n', 1)
                    if len(line) > 65536:
                        raise ValueError('Engine line exceeds bounded size')
                    line = line.decode('utf-8', errors='replace').rstrip('\r')
                    retain(name, line)
                    if name == 'stdout':
                        info = parse_info(line)
                        if info is not None:
                            info_bytes += len(line.encode())
                            if info_bytes > RAW_LIMIT:
                                raise ValueError('Score updates exceed bounded retained size')
                            infos.append(info)
                            if len(infos) > 10000:
                                raise ValueError('Too many score updates')
                        if line == token or line.startswith(token + ' '):
                            return line
    try:
        board = board_for(decision)
        if forced is not None:
            check_move(forced)
            if forced not in [m.uci() for m in board.legal_moves]:
                raise ValueError('Requested diagnostic root move is not legal')
        process = subprocess.Popen([str(binary)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, start_new_session=True)
        save(inflight_path, {'pid': process.pid, 'started_utc': receipt['started_utc'],
                             'decision': decision, 'forced_move': forced, 'nodes': nodes})
        for name in buffers:
            stream = getattr(process, name)
            os.set_blocking(stream.fileno(), False)
            selector.register(stream, selectors.EVENT_READ, name)
        send('uci')
        wait_for('uciok')
        for name, value in OPTIONS.items():
            send(f'setoption name {name} value {value}')
        send('isready')
        wait_for('readyok')
        send('ucinewgame')
        send('setoption name Clear Hash')
        send('isready')
        wait_for('readyok')
        position = 'position fen ' + decision['initial_fen']
        if decision['history']:
            position += ' moves ' + ' '.join(decision['history'])
        send(position)
        send(f'go nodes {nodes}' + (' searchmoves ' + forced if forced else ''))
        best = wait_for('bestmove').split()[1]
        receipt['assessment'] = select_score(infos, best, forced)
        receipt['score_updates'] = infos
        exact = receipt['assessment']['score']
        if exact is not None:
            pv_board = board.copy(stack=True)
            try:
                for move in exact['pv']:
                    pv_board.push_uci(move)
                receipt['pv_independent_legal'] = True
            except ValueError:
                receipt['pv_independent_legal'] = False
                receipt['assessment']['uncertainty'].append('illegal_pv')
        receipt['status'] = 'complete'
    except BaseException as exc:
        receipt['error'] = repr(exc)
        receipt['score_updates'] = infos
        if isinstance(exc, (KeyboardInterrupt, SystemExit)):
            receipt['interrupted'] = True
    finally:
        selector.close()
        if process is not None:
            try:
                send('quit')
                process.wait(timeout=1)
            except (BrokenPipeError, OSError, subprocess.TimeoutExpired):
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait(timeout=2)
            receipt['exit_code'] = process.returncode
            if process.returncode != 0:
                receipt['status'] = 'failed'
                receipt.setdefault('error', 'Engine did not exit successfully')
            for stream in (process.stdin, process.stdout, process.stderr):
                stream.close()
            def group_alive():
                try:
                    os.killpg(process.pid, 0)
                    return True
                except ProcessLookupError:
                    return False
            if group_alive():
                receipt['residual_process_group'] = True
                receipt['status'] = 'failed'
                receipt.setdefault('error', 'Residual process group required termination')
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                cleanup_deadline = time.monotonic() + 2
                while group_alive() and time.monotonic() < cleanup_deadline:
                    time.sleep(.02)
            receipt['process_group_settled'] = not group_alive()
            if not receipt['process_group_settled']:
                receipt['status'] = 'failed'
                receipt['error'] = 'Process group survives cleanup; inflight marker retained'
        else:
            receipt['process_group_settled'] = True
        usage1 = resource.getrusage(resource.RUSAGE_CHILDREN)
        receipt['wall_seconds'] = time.monotonic() - started
        receipt['child_cpu_seconds'] = (usage1.ru_utime + usage1.ru_stime) - (usage0.ru_utime + usage0.ru_stime)
        receipt['finished_utc'] = utc()
        receipt['raw_retained_bytes'] = raw_bytes
        if receipt['raw_truncated'] and 'assessment' in receipt:
            receipt['assessment']['uncertainty'].append('raw_uci_truncated')
    return receipt


@contextlib.contextmanager
def execution_lock():
    """Shared with run_batch.py: reference work cannot overlap match work."""
    with (HERE / '.execution.lock').open('a') as handle:
        fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
        yield


@contextlib.contextmanager
def locked(directory):
    directory.mkdir(parents=True, exist_ok=True)
    with (directory / '.lock').open('a') as handle:
        fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
        if (directory / 'inflight.json').exists():
            raise RuntimeError('Inflight/orphan marker exists: inspect process and preserve failure before explicit recovery')
        yield


def file_identity(path):
    value = Path(path).stat()
    return [value.st_dev, value.st_ino, value.st_size, value.st_mtime_ns, value.st_ctime_ns]


def run(args):
    binary, directory = Path(args.engine).resolve(), Path(args.output).resolve()
    build_path = HERE / 'build.json'
    frozen = decode(build_path.read_text())['engines']['stockfish19']
    if digest(binary) != frozen['sha256']:
        raise ValueError('Engine differs from study Stockfish19 freeze')
    decisions, games = (extract_roots(decode(Path(args.report).read_text())) if args.roots
                        else extract_decisions(load_report(args.report)))
    import chess  # Fail missing diagnostic dependency before creating an inflight marker.
    if args.game is not None:
        decisions = [x for x in decisions if x['game'] == args.game]
    if args.ply is not None:
        decisions = [x for x in decisions if x['ply'] == args.ply]
    if args.move is not None and (args.game is None or args.ply is None):
        raise ValueError('--move requires an exact --game and --ply')
    if not decisions:
        raise ValueError('No matching decisions')
    manifest = {'schema': 1, 'report_sha256': digest(args.report), 'script_sha256': digest(__file__),
                'engine_sha256': digest(binary), 'engine_path': str(binary), 'nodes': args.nodes,
                'engine_file_identity': file_identity(binary),
                'build_sha256': digest(build_path),
                'protocol_sha256': digest(HERE / '../../competitive-gameplay-diagnosis.md'),
                'options': OPTIONS, 'timeout_seconds': 60, 'game': args.game, 'ply': args.ply,
                'python_chess_version': chess.__version__, 'roots_input': args.roots,
                'move': args.move, 'native_replay_required': True,
                'games': games, 'planned_decisions': len(decisions)}
    with execution_lock(), locked(directory):
        path = directory / 'manifest.json'
        if path.exists():
            if decode(path.read_text()) != manifest:
                raise ValueError('Pinned source, report, engine or method changed; use a new output directory')
        else:
            save(path, manifest)
        prior_results = [decode(p.read_text()) for p in directory.glob('g*-attempt*.json')]
        usage = {'attempts': len(prior_results),
                 'wall_seconds': sum(x['wall_seconds'] for x in prior_results),
                 'child_cpu_seconds': sum(x['child_cpu_seconds'] for x in prior_results),
                 'failures': sum(x['status'] != 'complete' for x in prior_results)}
        for decision in decisions:
            queries = [('alternative', args.move)] if args.move else ([('unrestricted', None)] if args.roots else [('unrestricted', None), ('played', decision['played'])])
            for kind, forced in queries:
                stem = f'g{decision["game"]:03d}-p{decision["ply"]:03d}-{kind}'
                previous = sorted(directory.glob(stem + '-attempt*.json'))
                if previous:
                    latest = decode(previous[-1].read_text())
                    if latest['status'] == 'complete':
                        continue
                    if not args.retry_failed:
                        raise RuntimeError('Prior failed query retained; inspect then explicitly --retry-failed: ' + stem)
                if file_identity(binary) != manifest['engine_file_identity']:
                    raise ValueError('Engine file identity changed between queries; batch stopped before launch')
                target = directory / f'{stem}-attempt{len(previous):04d}.json'
                inflight = directory / 'inflight.json'
                save(inflight, {'pid': None, 'query': stem, 'started_utc': utc()})
                result = query(binary, decision, args.nodes, forced, inflight)
                result.update(query=stem, engine_sha256=manifest['engine_sha256'], nodes_requested=args.nodes)
                save(target, result)
                if result['process_group_settled']:
                    inflight.unlink()
                usage['attempts'] += 1
                usage['wall_seconds'] += result['wall_seconds']
                usage['child_cpu_seconds'] += result['child_cpu_seconds']
                usage['failures'] += result['status'] != 'complete'
                usage['artifact_bytes'] = sum(p.stat().st_size for p in directory.iterdir() if p.is_file())
                save(directory / 'resources.json', usage)
                print(json.dumps({'query': stem, 'status': result['status'], 'wall_seconds': result['wall_seconds']}), flush=True)
                if result['status'] != 'complete':
                    raise RuntimeError('Query failed; preserved receipt: ' + str(target))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('report')
    parser.add_argument('output')
    parser.add_argument('--engine', default=DEFAULT_ENGINE)
    parser.add_argument('--roots', action='store_true', help='Input is a JSON list of {id,fen,moves}; unrestricted endpoint screening only')
    parser.add_argument('--nodes', type=int, default=500000)
    parser.add_argument('--game', type=int)
    parser.add_argument('--ply', type=int)
    parser.add_argument('--move')
    parser.add_argument('--retry-failed', action='store_true')
    args = parser.parse_args()
    if not 1 <= args.nodes <= 20000000:
        parser.error('--nodes must be between 1 and 20000000')
    run(args)


if __name__ == '__main__':
    main()
