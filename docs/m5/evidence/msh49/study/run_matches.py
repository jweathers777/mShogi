#!/usr/bin/env python3
"""Offline orchestration only: Rust owns engines, legal commands, outcomes and replay.

One invocation schedules color pairs. Formal inference is a separate final pass,
never a stopping rule. All invocation wall time is charged, including failures.
"""
import argparse
import datetime
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import time

HERE = Path(__file__).resolve().parent
PROTOCOL = HERE / 'protocol-v1.json'
PROTOCOL_FREEZE = HERE / 'protocol-freeze.json'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def load(path):
    return json.loads(Path(path).read_text())


def eligible(report, expected):
    """Check experimental eligibility, not chess legality (strict Rust replay does that)."""
    reasons = []
    if report['config'] != expected:
        # Rust supplies effective watchdog/I/O defaults; compare explicitly set fields.
        if any(report['config'].get(k) != v for k, v in expected.items()):
            reasons.append('effective configuration differs')
    if len(report['games']) != 2 or len(report['summaries']) != 1:
        reasons.append('not one complete color pair')
    for g in report['games']:
        if g['stop'] != 'chess_conclusion' or g['failure'] or g['diagnostic_failures']:
            reasons.append('nonconcluded/failed game')
        if g['transcript']['truncated'] or g['transcript']['dropped_bytes']:
            reasons.append('truncated transcript')
        pending = {}
        for entry in g['transcript']['entries']:
            who, text = entry['engine'], entry['text']
            if entry['direction'] == 'command' and entry['phase'] == 'go':
                if who in pending or entry['write_completed'] is not True:
                    reasons.append('missing/overlapping go receipt')
                pending[who] = {'depth': 0, 'score': False}
            elif entry['direction'] == 'response' and text.startswith('info depth '):
                tokens = text.split()
                if who not in pending:
                    reasons.append('search info without go')
                    continue
                try:
                    depth = int(tokens[2])
                    i = tokens.index('score')
                    int(tokens[i+2])
                    if tokens[i+1] not in ['cp', 'mate']:
                        raise ValueError('unknown score')
                    pending[who] = {'depth': depth, 'score': True}
                except (ValueError, IndexError):
                    reasons.append('invalid completed-search info')
            elif entry['direction'] == 'response' and text.startswith('bestmove '):
                value = pending.pop(who, None)
                if not value or value['depth'] <= 0 or not value['score']:
                    reasons.append('bestmove without completed iteration')
        if pending:
            reasons.append('unanswered search')
        for engine in g['engines']:
            if not engine['exit'] or not engine['exit']['success'] or engine['cleanup_failures']:
                reasons.append('process exit/cleanup not successful')
            if engine['stderr']['observed_bytes'] or engine['stderr']['truncated']:
                reasons.append('stderr/fallback diagnostic')
    summary = report['summaries'][0] if len(report['summaries']) == 1 else None
    if not summary or summary['unscored'] or summary['failures'] or summary['games'] != 2:
        reasons.append('unscored/incomplete pair')
    halfpoints = None if reasons else 2*summary['wins'][0] + summary['draws']
    return halfpoints, sorted(set(reasons))


def run(args):
    protocol, pf = load(PROTOCOL), load(PROTOCOL_FREEZE)
    assert digest(PROTOCOL.read_bytes()) == pf['protocol_sha256']
    catalog = load(args.catalog)
    assert catalog['population'] == protocol['population']['id']
    if args.phase == 'formal':
        if not args.freeze or args.take:
            raise ValueError('formal requires frozen artifact manifest and full fixed sample')
        freeze = load(args.freeze)
        assert freeze['protocol_sha256'] == pf['protocol_sha256']
        assert freeze['development_catalog_sha256'] == pf['development_catalog_sha256']
        assert catalog['excluded_sha256'] == pf['development_catalog_sha256']
        assert catalog['label'] == args.arm
        required = 64 if args.arm == 'relative' else 32
        assert len(catalog['samples']) == required
        assert freeze['catalog_sha256'][args.arm] == digest(Path(args.catalog).read_bytes())
    else:
        assert digest(Path(args.catalog).read_bytes()) == pf['development_catalog_sha256']
        freeze = None
    out = Path(args.output)
    out.mkdir(parents=True, exist_ok=True)
    ledger_path = Path(args.ledger)
    ledger = load(ledger_path) if ledger_path.exists() else {
        'schema': 1, 'protocol_sha256': pf['protocol_sha256'], 'spent_seconds': 54,
        'calibration_charged_seconds': 54, 'runs': [], 'inflight': None}
    assert ledger['protocol_sha256'] == pf['protocol_sha256']
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
             'pairs': [], 'complete': False, 'output': str(out.resolve())}
    ledger['runs'].append(entry)
    persist = lambda: ledger_path.write_text(json.dumps(ledger, indent=2)+'\n')
    persist()
    samples = catalog['samples'][:args.take] if args.take else catalog['samples']
    stop = False
    for index, sample in enumerate(samples):
        comparisons = ['candidate'] if args.arm == 'relative' else ['baseline', 'candidate']
        for participant in comparisons:
            if ledger['spent_seconds'] >= protocol['budget']['local_match_wall_seconds']:
                entry['stop'] = 'budget'; stop = True; break
            key = f'{args.phase}-{args.arm}-{index:03}-{participant}'
            cp, rp = out/(key+'.config.json'), out/(key+'.external.json')
            first, second = paths[participant], paths['baseline'] if args.arm == 'relative' else paths['stockfish']
            identity = 'candidate conventional native; source/artifacts recorded in study manifest' if participant == 'candidate' else 'frozen mshogi.chess.material.v1 / mshogi.chess.full-width.v1 source05eeba3'
            def engine(path, model, sf=False):
                return {'path': path, 'args': [] if sf else ['uci'],
                        'options': [{'name':k,'value':v} for k,v in protocol['stockfish']['settings'].items()] if sf else [],
                        'model_path': None, 'model_identity': model, 'deterministic': True}
            own = protocol['candidate_and_baseline_limits']; other = protocol['stockfish_limits'] if args.arm == 'external' else own
            config = {'schema_version':1, 'id': 'msh49-'+key,
                      'engines':[engine(first, identity), engine(second, 'Stockfish19 embedded nn-1a298aa575a0.nnue' if args.arm == 'external' else 'frozen material/fullwidth baseline source05eeba3', args.arm == 'external')],
                      'openings':[sample['opening']], 'paired':True, 'claim_policy':'auto_claim', 'time_control':'untimed',
                      'work': own, 'engine_work':[own, other], 'max_plies':512, 'io':protocol['io']}
            with cp.open('x') as f:
                json.dump(config, f)
            ledger['inflight'] = {'key':key,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'report':str(rp)}
            persist()
            began = time.monotonic()
            result = subprocess.run([paths['runner'], 'experiment-external', 'uci', str(cp), str(rp)], capture_output=True, text=True)
            elapsed = time.monotonic()-began
            ledger['spent_seconds'] += elapsed
            receipt = {'key':key,'sample':index,'participant':participant,'seconds':elapsed,'returncode':result.returncode,'stderr':result.stderr,'eligible':False}
            if result.returncode == 0:
                raw = rp.read_bytes(); outer = json.loads(raw)
                assert digest(outer['report_json'].encode()) == outer['report_sha256']
                report = json.loads(outer['report_json'])
                for actual, path in zip(report['engine_identities'], [first, second]):
                    assert actual['executable']['sha256'] == hashes[next(k for k,v in paths.items() if v==path)]
                halfpoints, reasons = eligible(report, config)
                receipt.update({'halfpoints':halfpoints,'eligible':not reasons,'reasons':reasons,
                                'report_sha256':digest(raw),'report':rp.name+'.gz','summary':report['summaries']})
                Path(str(rp)+'.gz').write_bytes(gzip.compress(raw, mtime=0))
                rp.unlink()
            else:
                receipt['reasons'] = ['runner operation failed; no invented result']
            ledger['inflight'] = None;entry['pairs'].append(receipt);persist()
            print(json.dumps({'run':runid,'key':key,'seconds':round(elapsed,3),'spent':round(ledger['spent_seconds'],3),'halfpoints':receipt.get('halfpoints'),'eligible':receipt['eligible']}),flush=True)
            if not receipt['eligible']:
                entry['stop']='ineligible';stop=True;break
        if stop:break
    entry['complete'] = not stop and len(entry['pairs']) == len(samples)*(1 if args.arm=='relative' else 2)
    if ledger['spent_seconds'] > protocol['budget']['local_match_wall_seconds']:
        entry['budget_overrun_seconds'] = ledger['spent_seconds'] - protocol['budget']['local_match_wall_seconds']
        entry['complete'] = False
        entry['stop'] = 'budget_overrun_ineligible'
    persist()


if __name__ == '__main__':
    parser=argparse.ArgumentParser()
    for name in ['runner','candidate','baseline','stockfish','catalog','output','ledger']:
        parser.add_argument('--'+name,required=True)
    parser.add_argument('--phase',choices=['development','formal'],required=True)
    parser.add_argument('--arm',choices=['relative','external'],required=True)
    parser.add_argument('--take',type=int)
    parser.add_argument('--freeze')
    run(parser.parse_args())
