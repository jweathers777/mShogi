#!/usr/bin/env python3
"""One MSH-49.2 initial diagnostic, using the existing native external runner unchanged."""
import datetime
import gzip
import hashlib
import json
import math
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(path):
    return json.loads(Path(path).read_text())


def save(path, value):
    temporary = Path(str(path) + '.tmp')
    temporary.write_text(json.dumps(value, indent=2) + '\n')
    temporary.replace(path)


def verify_build(build):
    for artifact in build['artifacts'].values():
        if digest(artifact['path']) != artifact['sha256']:
            raise ValueError('Frozen executable/network hash mismatch')


def configuration(protocol, build, opening, smoke=False):
    return {
        'schema_version': 1, 'id': protocol['id'] + ('-smoke' if smoke else '') + '-' + opening['id'],
        'engines': [
            {'path': build['artifacts']['mshogi']['path'], 'args': ['uci'], 'options': [],
             'model_path': None, 'model_identity': 'mshogi.chess.conventional.v1 / mshogi.chess.conventional-tapered.v1', 'deterministic': False},
            {'path': build['artifacts']['stockfish']['path'], 'args': [],
             'options': [{'name': k, 'value': v} for k, v in protocol['resources']['stockfish_options'].items()],
             'model_path': build['artifacts']['network']['path'], 'model_identity': 'Stockfish19 pinned default nn-1a298aa575a0.nnue', 'deterministic': False}],
        'openings': [opening], 'paired': True, 'claim_policy': 'auto_claim', 'time_control': 'untimed',
        'work': {'depth': None, 'nodes': None, 'movetime_ms': protocol['time']['movetime_ms']},
        'max_plies': 4 if smoke else protocol['stopping']['max_total_plies_per_game'],
        'watchdogs': {'handshake_ms': 5000, 'ready_ms': 5000, 'response_ms': 10000, 'write_ms': 2000, 'cleanup_ms': 1000},
        'io': {'line_bytes': 4096, 'queued_lines': 32, 'transcript_bytes': 2097152, 'stderr_bytes': 16384, 'command_bytes': 4096}}


def audit_report(outer, expected, build):
    raw = outer['report_json'].encode()
    if outer['protocol'] != 'uci' or hashlib.sha256(raw).hexdigest() != outer['report_sha256']:
        raise ValueError('Report digest/protocol mismatch')
    report = json.loads(raw)
    if report['config'] != expected or len(report['games']) != 2:
        raise ValueError('Different input or missing color pair')
    environment = outer['environment']
    if environment['harness_revision'] != build['source'] or environment['harness_dirty_sha256'] != hashlib.sha256(b'').hexdigest():
        raise ValueError('Different/dirty harness build')
    for index, label in enumerate(['mshogi', 'stockfish']):
        identity = report['engine_identities'][index]
        if identity['executable']['sha256'] != build['artifacts'][label]['sha256']:
            raise ValueError('Different player executable')
        if label == 'stockfish' and identity['model']['sha256'] != build['artifacts']['network']['sha256']:
            raise ValueError('Different Stockfish network')
    healthy = True
    for index, game in enumerate(report['games']):
        if game['pairing'] != index or game['engines_by_color'] != ([0, 1] if index == 0 else [1, 0]):
            raise ValueError('Different color assignment')
        healthy &= not game['failure'] and not game['diagnostic_failures'] and not game['transcript']['truncated']
        healthy &= len(game['engines']) == 2
        for child in game['engines']:
            healthy &= not child['identity_truncated'] and not child['cleanup_failures']
            healthy &= child['stderr']['observed_bytes'] == 0 and not child['stderr']['truncated']
            healthy &= bool(child['exit'] and child['exit']['success'])
        commands = [e['text'] for e in game['transcript']['entries'] if e['direction'] == 'command' and e['text'].startswith('go')]
        if not commands or any(c != f"go movetime {expected['work']['movetime_ms']}" for c in commands):
            raise ValueError('Missing/different effective time controls')
    summary = report['summaries'][0]
    complete = healthy and all(g['stop'] == 'chess_conclusion' for g in report['games']) and summary['unscored'] == 0 and sum(summary['wins']) + summary['draws'] == 2
    return {'healthy': bool(healthy), 'complete': bool(complete), 'summary': summary,
            'pair_score': (summary['wins'][0] + summary['draws'] / 2) / 2 if complete else None,
            'stops': [g['stop'] for g in report['games']]}


def audit_budget(ledger):
    budget = ledger['budget']
    seconds = [r['seconds'] for r in budget['runs']]
    if any(not math.isfinite(n) or n < 0 for n in seconds):
        raise ValueError('Invalid invocation charge')
    if not math.isfinite(budget['charged_seconds']) or abs(sum(seconds)-budget['charged_seconds']) > 1e-6:
        raise ValueError('Aggregate charge differs from invocation receipts')
    if budget['inflight']:
        raise ValueError('Unsettled prior invocation')


def invoke(protocol, build, ledger, config, phase, folder, timeout):
    if ledger['budget']['inflight']:
        raise ValueError('Unsettled prior invocation; never reset accounting')
    folder.mkdir(parents=True, exist_ok=False)
    config_path, report_path = folder/'config.json', folder/'report.json'
    save(config_path, config)
    ledger['budget']['inflight'] = {'phase': phase, 'folder': str(folder), 'started_utc': datetime.datetime.now(datetime.timezone.utc).isoformat()}
    save(HERE/'verification.json', ledger)
    started = time.monotonic()
    receipt = {'phase': phase, 'folder': str(folder), 'timeout_seconds': timeout, 'report': None, 'healthy': False, 'complete': False}
    child = None
    cleanup_deadline = None

    def stop_child():
        nonlocal cleanup_deadline
        if cleanup_deadline is None:
            cleanup_deadline = time.monotonic() + 5
        try:
            os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        # All error paths share this one cleanup allowance. No nested timeout
        # can start another window or turn interruption into a scored result.
        return child.communicate(timeout=max(0, cleanup_deadline-time.monotonic()))

    try:
        child = subprocess.Popen([build['artifacts']['runner']['path'], 'experiment-external', 'uci', str(config_path), str(report_path)], stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
        try:
            stdout, stderr = child.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            receipt['outer_timeout'] = True
            stdout, stderr = stop_child()
        receipt.update({'returncode': child.returncode, 'stdout': stdout.decode(errors='replace'), 'stderr': stderr.decode(errors='replace')})
    except BaseException as error:
        receipt['interruption'] = repr(error)
        if child is not None:
            stop_child()
        raise
    finally:
        # Charge every launch attempt, even interruption/error; unresolved work
        # stays visible and prevents another invocation until reconciled.
        receipt['seconds'] = time.monotonic()-started
        ledger['budget']['charged_seconds'] += receipt['seconds']
        ledger['budget']['runs'].append(receipt)
        ledger['budget']['inflight'] = None if child is None or child.poll() is not None else ledger['budget']['inflight']
        save(HERE/'verification.json', ledger)
    if report_path.exists():
        raw = report_path.read_bytes()
        receipt['report_sha256'] = hashlib.sha256(raw).hexdigest()
        receipt['report'] = str(folder/'report.json.gz')
        (folder/'report.json.gz').write_bytes(gzip.compress(raw, mtime=0))
        try:
            receipt.update(audit_report(json.loads(raw), config, build))
            replay = subprocess.run([build['artifacts']['runner']['path'], 'experiment-external-replay', str(report_path)], capture_output=True, text=True, timeout=30)
            receipt['replay_returncode'], receipt['replay_stderr'] = replay.returncode, replay.stderr
            if replay.returncode or replay.stderr or receipt['returncode'] or receipt['stderr']:
                receipt['healthy'] = receipt['complete'] = False
        except (ValueError, subprocess.TimeoutExpired) as error:
            receipt['audit_error'] = str(error)
            receipt['healthy'] = receipt['complete'] = False
        # The exact raw bytes remain in the compressed receipt and its digest.
        report_path.unlink()
    save(HERE/'verification.json', ledger)
    return receipt


def main(mode):
    protocol, build, ledger = [read(HERE/name) for name in ['protocol.json', 'build.json', 'verification.json']]
    verify_build(build)
    audit_budget(ledger)
    if mode == 'smoke':
        if ledger['budget']['runs']:
            raise ValueError('One smoke only; preserve failures, no automatic repeats')
        config = configuration(protocol, build, {'id': 'startup', 'fen': None, 'moves': []}, True)
        receipt = invoke(protocol, build, ledger, config, 'routine_smoke', HERE/'smoke', min(15, 900-ledger['budget']['charged_seconds']))
        print(json.dumps(receipt, indent=2))
        return
    if mode != 'diagnostic':
        raise ValueError('Use diagnostic (the formal study is superseded)')
    approval = ledger['approval']
    if not approval or approval.get('owner') != 'John' or approval.get('approved_protocol_sha256') != digest(HERE/'protocol.json'):
        raise ValueError('Explicit owner approval of this exact budget/protocol is required')
    if ledger['budget']['inflight'] or any(r['phase'] == 'diagnostic' for r in ledger['budget']['runs']):
        raise ValueError('Prior diagnostic work cannot be automatically repeated/replaced')
    if subprocess.check_output(['git', 'diff', '--binary', build['source'], '--', 'crates', 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml'], cwd=REPO):
        raise ValueError('Production source differs from accepted frozen build')
    frozen_inputs = [str(HERE/name) for name in ['study.py', 'test_study.py', 'protocol.json', 'build.json']]
    if subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=all', '--', *frozen_inputs], cwd=REPO):
        raise ValueError('Commit reviewed study inputs before diagnostic execution')
    freeze = {'protocol_sha256': digest(HERE/'protocol.json'), 'build_sha256': digest(HERE/'build.json'), 'study_code_sha256': digest(__file__), 'study_source': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=REPO, text=True).strip(), 'approval': approval}
    with (HERE/'freeze.json').open('x') as f:
        json.dump(freeze, f, indent=2)
    catalog = HERE/'roots.json'
    save(catalog, protocol['population']['openings'])
    ledger['diagnostic_roots'] = {'path': str(catalog), 'sha256': digest(catalog), 'selection': 'Authored prefixes, no random sampler or seed'}
    save(HERE/'verification.json', ledger)
    receipts = []
    for index, opening in enumerate(read(catalog)):
        remaining = protocol['stopping']['budget_seconds'] - ledger['budget']['charged_seconds']
        if remaining < 60:
            break
        receipt = invoke(protocol, build, ledger, configuration(protocol, build, opening), 'diagnostic', HERE/f'pair-{index:03}', remaining-5)
        receipts.append(receipt)
        if not receipt['healthy']:
            break
    ledger['diagnostic_results'] = {'attempted_pairs': len(receipts), 'planned_pairs': protocol['pairs'], 'completed_pairs': sum(r['complete'] for r in receipts), 'summaries': [r.get('summary') for r in receipts], 'scope': 'Descriptive initial diagnostic; no statistical significance/interval/Elo or numerical strength acceptance.'}
    save(HERE/'verification.json', ledger)
    print(json.dumps(ledger['diagnostic_results'], indent=2))


if __name__ == '__main__':
    lock = HERE/'.study.lock'
    # Exclusive lock covers receipt/config publication and the entire campaign.
    with lock.open('x') as f:
        f.write(str(os.getpid()))
    try:
        main(sys.argv[1] if len(sys.argv) == 2 else '')
    finally:
        lock.unlink()
