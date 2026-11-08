#!/usr/bin/env python3
"""Offline orchestration only: Rust owns engines, legal commands, outcomes and replay.

One invocation schedules color pairs. Formal inference is a separate final pass,
never a stopping rule. All invocation wall time is charged, including failures.
"""
import argparse
from contextlib import contextmanager
import datetime
import gzip
import hashlib
import json
import math
from pathlib import Path
import subprocess
import time

HERE = Path(__file__).resolve().parent
PROTOCOL = HERE / 'protocol-v1.json'
PROTOCOL_FREEZE = HERE / 'protocol-freeze.json'
WATCHDOGS = {'handshake_ms': 5000, 'ready_ms': 5000, 'response_ms': 10000,
             'write_ms': 2000, 'cleanup_ms': 1000}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def load(path):
    return json.loads(Path(path).read_text())


def expected_config(protocol, paths, phase, arm, index, participant, sample):
    """Reconstruct the actual complete study config, independent of a report."""
    first = paths[participant]
    second = paths['baseline'] if arm == 'relative' else paths['stockfish']
    identity = ('candidate conventional native; source/artifacts recorded in study manifest'
                if participant == 'candidate' else
                'frozen mshogi.chess.material.v1 / mshogi.chess.full-width.v1 source05eeba3')

    def engine(path, model, sf=False):
        return {'path': path, 'args': [] if sf else ['uci'],
                'options': [{'name': k, 'value': v} for k, v in protocol['stockfish']['settings'].items()] if sf else [],
                'model_path': None, 'model_identity': model, 'deterministic': True}

    own = protocol['candidate_and_baseline_limits']
    other = protocol['stockfish_limits'] if arm == 'external' else own
    key = f'{phase}-{arm}-{index:03}-{participant}'
    return {'schema_version': 1, 'id': 'msh49-'+key,
            'engines': [engine(first, identity), engine(second,
                        'Stockfish19 embedded nn-1a298aa575a0.nnue' if arm == 'external' else
                        'frozen material/fullwidth baseline source05eeba3', arm == 'external')],
            'openings': [sample['opening']], 'paired': True, 'claim_policy': 'auto_claim',
            'time_control': 'untimed', 'work': own, 'engine_work': [own, other],
            'max_plies': 512, 'watchdogs': dict(WATCHDOGS), 'io': protocol['io']}


def audit_budget(ledger, protocol_freeze):
    """Recompute all charged wall time, including development and failed receipts."""
    initial = protocol_freeze['pretarget_calibration_charged_seconds']
    if ledger.get('calibration_charged_seconds') != initial:
        raise ValueError('calibration charge differs from frozen protocol')
    charges = [initial]
    for run in ledger['runs']:
        for receipt in run['pairs']:
            seconds = receipt['seconds']
            if type(seconds) not in (int, float) or not math.isfinite(seconds) or seconds < 0:
                raise ValueError('nonfinite/negative receipt charge')
            charges.append(seconds)
    recomputed = math.fsum(charges)
    stored = ledger['spent_seconds']
    if type(stored) not in (int, float) or not math.isfinite(stored) or stored < 0:
        raise ValueError('nonfinite/negative total charge')
    if not math.isclose(stored, recomputed, rel_tol=1e-12, abs_tol=1e-6):
        raise ValueError('stored budget differs from all receipt charges')
    return recomputed


def audit_catalog(catalog, protocol, protocol_freeze, arm, freeze, raw):
    """Require the frozen arm, sampler, entropy seed and fixed development exclusion."""
    assert freeze['protocol_sha256'] == protocol_freeze['protocol_sha256']
    assert freeze['sampler_sha256'] == protocol_freeze['sampler_sha256']
    assert freeze['development_catalog_sha256'] == protocol_freeze['development_catalog_sha256']
    assert catalog['population'] == protocol['population']['id'] and catalog['label'] == arm
    assert catalog['seed'] == freeze['holdout_seed']
    assert catalog['excluded_sha256'] == protocol_freeze['development_catalog_sha256']
    required = 64 if arm == 'relative' else 32
    assert len(catalog['samples']) == required
    assert freeze['catalog_sha256'][arm] == digest(raw)
    for index, sample in enumerate(catalog['samples']):
        opening = sample['opening']
        assert opening['id'] == f'{arm}-{index:03}' and opening['fen'] is None
        family = protocol['population']['families'][sample['family']]
        assert 4 <= sample['extra_plies'] <= 8 and 1 <= sample['attempts'] <= 10000
        assert opening['moves'][:len(family)] == family
        assert len(opening['moves']) == len(family)+sample['extra_plies']
        events = sample['record']['payload']['events']
        assert sample['record']['payload']['claim_policy'] == 'auto_claim'
        assert [e['event']['action'] for e in events] == opening['moves']
        assert all(e['ply'] == i and e['event']['kind'] == 'move' for i, e in enumerate(events))


def audit_identities(report, expected, hashes, arm, participant):
    """Bind each historical report's actual identities to its frozen participants."""
    roles = [participant, 'baseline' if arm == 'relative' else 'stockfish']
    identities = report['engine_identities']
    if len(identities) != 2:
        raise ValueError('two actual engine identities required')
    for actual, config, role in zip(identities, expected['engines'], roles):
        executable = actual['executable']
        if (executable['path'] != config['path'] or executable['error'] is not None
                or executable['sha256'] != hashes[role]
                or actual['model'] is not None or actual['model_identity'] != config['model_identity']):
            raise ValueError('actual report participant identity differs from frozen artifact')


def eligible(report, expected):
    """Check experimental eligibility, not chess legality (strict Rust replay does that)."""
    reasons = []
    if report['config'] != expected:
        reasons.append('effective configuration differs')
    if len(report['games']) != 2 or len(report['summaries']) != 1:
        reasons.append('not one complete color pair')
    for pairing, g in enumerate(report['games']):
        if g['pairing'] != pairing or g['engines_by_color'] != ([0, 1] if pairing == 0 else [1, 0]):
            reasons.append('color/participant pairing differs')
        if g['stop'] != 'chess_conclusion' or g['failure'] or g['diagnostic_failures']:
            reasons.append('nonconcluded/failed game')
        if g['transcript']['truncated'] or g['transcript']['dropped_bytes']:
            reasons.append('truncated transcript')
        pending = {}
        for entry in g['transcript']['entries']:
            who, text = entry['engine'], entry['text']
            if who not in [0, 1] or entry['truncated']:
                reasons.append('invalid participant/truncated entry')
                continue
            work = expected['engine_work'][who]
            if entry['direction'] == 'command' and entry['phase'] == 'go':
                if who in pending or entry['write_completed'] is not True:
                    reasons.append('missing/overlapping go receipt')
                pending[who] = {'depth': 0, 'nodes': 0, 'score': False}
            elif entry['direction'] == 'response' and text.startswith('info depth '):
                tokens = text.split()
                if who not in pending:
                    reasons.append('search info without go')
                    continue
                try:
                    depth = int(tokens[2])
                    nodes = int(tokens[tokens.index('nodes')+1])
                    i = tokens.index('score')
                    int(tokens[i+2])
                    if tokens[i+1] not in ['cp', 'mate'] or depth < 0 or nodes < 0:
                        raise ValueError('invalid completed-search info')
                    if ((work['depth'] is not None and depth > work['depth'])
                            or (work['nodes'] is not None and nodes > work['nodes'])):
                        reasons.append('reported work exceeds requested ceiling')
                    pending[who] = {'depth': depth, 'nodes': nodes, 'score': True}
                except (ValueError, IndexError):
                    reasons.append('invalid completed-search info')
            elif entry['direction'] == 'response' and text.startswith('bestmove '):
                value = pending.pop(who, None)
                if not value or value['depth'] <= 0 or value['nodes'] <= 0 or not value['score']:
                    reasons.append('bestmove without completed iteration')
        if pending:
            reasons.append('unanswered search')
        if len(g['engines']) != 2:
            reasons.append('two process observations required')
        for engine in g['engines']:
            if not engine['exit'] or not engine['exit']['success'] or engine['cleanup_failures']:
                reasons.append('process exit/cleanup not successful')
            if engine['identity_truncated']:
                reasons.append('truncated process identity')
            if engine['stderr']['observed_bytes'] or engine['stderr']['truncated']:
                reasons.append('stderr/fallback diagnostic')
    summary = report['summaries'][0] if len(report['summaries']) == 1 else None
    if not summary or summary['unscored'] or summary['failures'] or summary['games'] != 2:
        reasons.append('unscored/incomplete pair')
    halfpoints = None if reasons else 2*summary['wins'][0] + summary['draws']
    return halfpoints, sorted(set(reasons))


@contextmanager
def ledger_lock(path):
    """One active manager per ledger; a stale crash lock is never silently stolen."""
    lock = Path(str(path)+'.lock')
    with lock.open('x') as stream:
        stream.write('active match manager; preserve/reconcile after interrupted work\n')
    try:
        yield
    finally:
        lock.unlink()


def run(args):
    protocol, pf = load(PROTOCOL), load(PROTOCOL_FREEZE)
    assert digest(PROTOCOL.read_bytes()) == pf['protocol_sha256']
    catalog_raw = Path(args.catalog).read_bytes()
    catalog = json.loads(catalog_raw)
    assert catalog['population'] == protocol['population']['id']
    if args.phase == 'formal':
        if not args.freeze or args.take is not None:
            raise ValueError('formal requires frozen artifact manifest and full fixed sample')
        freeze = load(args.freeze)
        audit_catalog(catalog, protocol, pf, args.arm, freeze, catalog_raw)
    else:
        assert digest(catalog_raw) == pf['development_catalog_sha256']
        freeze = None
    if args.take is not None and not (1 <= args.take <= len(catalog['samples'])):
        raise ValueError('invalid development sample count')
    out = Path(args.output)
    out.mkdir(parents=True, exist_ok=True)
    with ledger_lock(Path(args.ledger)):
        run_locked(args, protocol, pf, catalog, freeze, out)


def run_locked(args, protocol, pf, catalog, freeze, out):
    ledger_path = Path(args.ledger)
    ledger = load(ledger_path) if ledger_path.exists() else {
        'schema': 1, 'protocol_sha256': pf['protocol_sha256'], 'spent_seconds': 54,
        'calibration_charged_seconds': 54, 'runs': [], 'inflight': None}
    assert ledger['protocol_sha256'] == pf['protocol_sha256']
    ledger['spent_seconds'] = audit_budget(ledger, pf)
    if ledger['inflight']:
        raise ValueError('unsettled prior invocation; preserve it, do not blindly repeat')
    if args.phase == 'formal' and any(x['phase'] == 'formal' and x['arm'] == args.arm for x in ledger['runs']):
        raise ValueError('formal arm already attempted; no retry/fresh seed')
    paths = {k: str(Path(getattr(args, k)).resolve()) for k in ['candidate', 'baseline', 'stockfish', 'runner']}
    hashes = {k: digest(Path(v).read_bytes()) for k, v in paths.items()}
    assert hashes['baseline'] == protocol['baseline']['executable_sha256']
    assert hashes['stockfish'] == protocol['stockfish']['executable_sha256']
    if freeze:
        assert hashes == freeze['executables_sha256']
    runid = len(ledger['runs'])
    entry = {'id': runid, 'phase': args.phase, 'arm': args.arm,
             'catalog_sha256': digest(Path(args.catalog).read_bytes()), 'executables_sha256': hashes,
             'executables_paths': paths,
             'pairs': [], 'complete': False, 'output': str(out.resolve())}
    ledger['runs'].append(entry)
    persist = lambda: ledger_path.write_text(json.dumps(ledger, indent=2)+'\n')
    persist()
    samples = catalog['samples'][:args.take] if args.take is not None else catalog['samples']
    stop = False
    for index, sample in enumerate(samples):
        comparisons = ['candidate'] if args.arm == 'relative' else ['baseline', 'candidate']
        for participant in comparisons:
            if ledger['spent_seconds'] >= protocol['budget']['local_match_wall_seconds']:
                entry['stop'] = 'budget'; stop = True; break
            key = f'{args.phase}-{args.arm}-{index:03}-{participant}'
            cp, rp = out/(key+'.config.json'), out/(key+'.external.json')
            config = expected_config(protocol, paths, args.phase, args.arm, index, participant, sample)
            with cp.open('x') as stream:
                json.dump(config, stream)
            ledger['inflight'] = {'key': key, 'started_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'report': str(rp)}
            persist()
            began = time.monotonic()
            result = subprocess.run([paths['runner'], 'experiment-external', 'uci', str(cp), str(rp)], capture_output=True, text=True)
            elapsed = time.monotonic()-began
            ledger['spent_seconds'] += elapsed
            receipt = {'key': key, 'sample': index, 'participant': participant, 'seconds': elapsed,
                       'returncode': result.returncode, 'stderr': result.stderr, 'eligible': False}
            if result.returncode == 0:
                try:
                    raw = rp.read_bytes(); outer = json.loads(raw)
                    assert outer['protocol'] == 'uci'
                    assert digest(outer['report_json'].encode()) == outer['report_sha256']
                    report = json.loads(outer['report_json'])
                    audit_identities(report, config, hashes, args.arm, participant)
                    halfpoints, reasons = eligible(report, config)
                    if result.stderr:
                        reasons.append('runner stderr diagnostic')
                        halfpoints = None
                    receipt.update({'halfpoints': halfpoints, 'eligible': not reasons, 'reasons': reasons,
                                    'report_sha256': digest(raw), 'summary': report['summaries']})
                    if not reasons:
                        with Path(str(rp)+'.gz').open('xb') as stream:
                            stream.write(gzip.compress(raw, mtime=0))
                        receipt['report'] = rp.name+'.gz'
                        rp.unlink()
                except Exception as error:
                    receipt['reasons'] = ['report audit failed: '+str(error)]
                    receipt['eligible'] = False
            else:
                receipt['reasons'] = ['runner operation failed; no invented result']
            if rp.exists():
                receipt['report_raw'] = rp.name
            ledger['inflight'] = None; entry['pairs'].append(receipt); persist()
            print(json.dumps({'run': runid, 'key': key, 'seconds': round(elapsed, 3),
                              'spent': round(ledger['spent_seconds'], 3),
                              'halfpoints': receipt.get('halfpoints'), 'eligible': receipt['eligible']}), flush=True)
            if not receipt['eligible']:
                entry['stop'] = 'ineligible'; stop = True; break
        if stop:
            break
    entry['complete'] = not stop and len(entry['pairs']) == len(samples)*(1 if args.arm == 'relative' else 2)
    ledger['spent_seconds'] = audit_budget(ledger, pf)
    if ledger['spent_seconds'] > protocol['budget']['local_match_wall_seconds']:
        entry['budget_overrun_seconds'] = ledger['spent_seconds'] - protocol['budget']['local_match_wall_seconds']
        entry['complete'] = False
        entry['stop'] = 'budget_overrun_ineligible'
    persist()


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    for name in ['runner', 'candidate', 'baseline', 'stockfish', 'catalog', 'output', 'ledger']:
        parser.add_argument('--'+name, required=True)
    parser.add_argument('--phase', choices=['development', 'formal'], required=True)
    parser.add_argument('--arm', choices=['relative', 'external'], required=True)
    parser.add_argument('--take', type=int)
    parser.add_argument('--freeze')
    run(parser.parse_args())
