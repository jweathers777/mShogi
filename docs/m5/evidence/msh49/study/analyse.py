#!/usr/bin/env python3
"""One fixed-sample MSH-49 analysis. Exact paired-sign tails, no chess rules."""
import argparse
import copy
import gzip
import hashlib
import json
import math
from pathlib import Path
import subprocess
import tempfile
import sys
sys.dont_write_bytecode = True
from run_matches import (audit_budget, audit_catalog, audit_identities, eligible,
                         expected_config, ledger_lock)

HERE = Path(__file__).resolve().parent


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def read(path):
    return json.loads(Path(path).read_text())


def sign_test(differences):
    positive = sum(d > 0 for d in differences)
    negative = sum(d < 0 for d in differences)
    n = positive+negative
    numerator = sum(math.comb(n, k) for k in range(positive, n+1)) if n else 1
    denominator = 2**n if n else 1
    return {'positive': positive, 'negative': negative, 'ties': len(differences)-n, 'non_ties': n,
            'tail_numerator': numerator, 'tail_denominator': denominator,
            'p': numerator/denominator, 'passes_0_05': 20*numerator <= denominator}


def analyse(args):
    ledger = read(args.ledger)
    freeze = read(args.freeze)
    protocol = read(HERE/'protocol-v1.json')
    pf = read(HERE/'protocol-freeze.json')
    assert ledger['protocol_sha256'] == freeze['protocol_sha256'] == pf['protocol_sha256'] == sha((HERE/'protocol-v1.json').read_bytes())
    assert sha(Path(args.runner).read_bytes()) == freeze['executables_sha256']['runner']
    assert freeze['executables_sha256']['baseline'] == protocol['baseline']['executable_sha256']
    assert freeze['executables_sha256']['stockfish'] == protocol['stockfish']['executable_sha256']
    formal = [r for r in ledger['runs'] if r['phase'] == 'formal']
    reasons = []
    if ledger['inflight']:
        reasons.append('unsettled invocation')
    try:
        spent = audit_budget(ledger, pf)
    except ValueError as error:
        spent = None
        reasons.append('budget accounting invalid: '+str(error))
    if spent is not None and spent > protocol['budget']['local_match_wall_seconds']:
        reasons.append('match budget exceeded')
    if len(formal) != 2 or {r['arm'] for r in formal} != {'relative', 'external'}:
        reasons.append('two fixed formal arms not complete')
    if any(not r['complete'] or any(not p['eligible'] for p in r['pairs']) for r in formal):
        reasons.append('incomplete/ineligible arm')
    if reasons:
        return {'eligible': False, 'passed': False, 'reasons': reasons, 'spent_seconds': spent,
                'no_acceptance_inference': 'Incomplete data never passes; no game/pair replaced or omitted'}
    results = {}
    for run in formal:
        assert run['executables_sha256'] == freeze['executables_sha256']
        assert run['catalog_sha256'] == freeze['catalog_sha256'][run['arm']]
        raw_catalog = Path(freeze['catalog_paths'][run['arm']]).read_bytes()
        catalog = json.loads(raw_catalog)
        audit_catalog(catalog, protocol, pf, run['arm'], freeze, raw_catalog)
        paths = run['executables_paths']
        assert set(paths) == set(freeze['executables_sha256'])
        count = 64 if run['arm'] == 'relative' else 32
        expected = [(i, p) for i in range(count) for p in
                    (['candidate'] if run['arm'] == 'relative' else ['baseline', 'candidate'])]
        assert [(p['sample'], p['participant']) for p in run['pairs']] == expected
        values = {}
        for receipt in run['pairs']:
            assert receipt['returncode'] == 0 and not receipt['stderr'] and receipt['eligible'] is True
            folder = Path(args.reports_root)/run['arm'] if args.reports_root else Path(run['output'])
            raw = gzip.decompress((folder/receipt['report']).read_bytes())
            assert sha(raw) == receipt['report_sha256']
            outer = json.loads(raw)
            assert outer['protocol'] == 'uci'
            assert sha(outer['report_json'].encode()) == outer['report_sha256']
            report = json.loads(outer['report_json'])
            expected_input = expected_config(protocol, paths, 'formal', run['arm'],
                                            receipt['sample'], receipt['participant'],
                                            catalog['samples'][receipt['sample']])
            audit_identities(report, expected_input, freeze['executables_sha256'],
                             run['arm'], receipt['participant'])
            # Independent native replay is read-only; no engine is launched here.
            with tempfile.TemporaryDirectory(prefix='msh49-replay-') as temporary:
                path = Path(temporary)/'report.json'
                path.write_bytes(raw)
                result = subprocess.run([args.runner, 'experiment-external-replay', str(path)], capture_output=True)
                assert result.returncode == 0, result.stderr.decode(errors='replace')
            hp, reasons = eligible(report, expected_input)
            assert not reasons and hp == receipt['halfpoints'], reasons
            values[(receipt['sample'], receipt['participant'])] = hp
        if run['arm'] == 'relative':
            scores = [values[(i, 'candidate')] for i in range(count)]
            total = sum(scores)
            test = sign_test([n-2 for n in scores])
            score_gate = total >= 180
            results['relative'] = {'pairs': count, 'candidate_halfpoints': total, 'score_fraction': total/256,
                                   'score_gate': score_gate, 'sign_test': test,
                                   'passed': score_gate and test['passes_0_05'],
                                   'pentanomial': [scores.count(k) for k in range(5)]}
        else:
            candidate = [values[(i, 'candidate')] for i in range(count)]
            baseline = [values[(i, 'baseline')] for i in range(count)]
            difference = [a-b for a, b in zip(candidate, baseline)]
            test = sign_test(difference)
            score_gate = sum(candidate) >= 32
            mean_gate = sum(difference) > 0
            results['external'] = {'roots': count, 'candidate_halfpoints': sum(candidate),
                                  'baseline_halfpoints': sum(baseline),
                                  'candidate_score_fraction': sum(candidate)/128,
                                  'baseline_score_fraction': sum(baseline)/128,
                                  'difference_halfpoints': sum(difference), 'score_gate': score_gate,
                                  'positive_mean_gate': mean_gate, 'sign_test': test,
                                  'passed': score_gate and mean_gate and test['passes_0_05'],
                                  'root_differences': difference}
    return {'eligible': True, 'passed': all(r['passed'] for r in results.values()),
            'spent_seconds': spent, 'results': results,
            'scope': 'Approved finite-resource opening-distribution study; exact tests concern nonzero paired signs, not mean CIs or human Elo'}


def self_test():
    """Constructed audit machinery examples only; no native/engine evidence."""
    assert sign_test([])['p'] == 1 and not sign_test([])['passes_0_05']
    assert sign_test([1]*4)['p'] == 1/16 and not sign_test([1]*4)['passes_0_05']
    assert sign_test([1]*5)['p'] == 1/32 and sign_test([1]*5)['passes_0_05']
    assert sign_test([1, 1, -1, -1])['p'] == 11/16
    assert sign_test([0, 1, 0, -1])['ties'] == 2
    pf = {'pretarget_calibration_charged_seconds': 54}
    ledger = {'calibration_charged_seconds': 54, 'spent_seconds': 61,
              'runs': [{'pairs': [{'seconds': 3}, {'seconds': 4}]}]}
    assert audit_budget(ledger, pf) == 61

    def refuses(call):
        try:
            call()
        except (AssertionError, ValueError):
            return
        raise AssertionError('corrupt constructed audit input was accepted')

    for mutation in [60, -1, True, float('nan'), float('inf')]:
        bad = copy.deepcopy(ledger)
        bad['spent_seconds'] = mutation
        refuses(lambda: audit_budget(bad, pf))
    for mutation in [-1, True, float('nan'), float('inf')]:
        bad = copy.deepcopy(ledger)
        bad['runs'][0]['pairs'][0]['seconds'] = mutation
        refuses(lambda: audit_budget(bad, pf))
    bad = copy.deepcopy(ledger)
    bad['calibration_charged_seconds'] = 0
    refuses(lambda: audit_budget(bad, pf))

    protocol = read(HERE/'protocol-v1.json')
    paths = {'candidate': '/candidate', 'baseline': '/baseline', 'stockfish': '/stockfish', 'runner': '/runner'}
    hashes = {role: str(i)*64 for i, role in enumerate(paths, 1)}
    sample = {'opening': {'id': 'test-root', 'fen': None, 'moves': []}}
    expected = expected_config(protocol, paths, 'formal', 'external', 0, 'candidate', sample)
    identities = [{'executable': {'path': expected['engines'][i]['path'], 'error': None,
                                  'sha256': hashes[role]},
                   'model': None, 'model_identity': expected['engines'][i]['model_identity']}
                  for i, role in enumerate(['candidate', 'stockfish'])]

    def game(pairing):
        entries = []
        for who, depth in [(0, 8), (1, 2)]:
            for direction, phase, text, completed in [
                    ('command', 'go', 'go', True),
                    ('response', 'bestmove', f'info depth {depth} nodes 100 score cp 0', None),
                    ('response', 'bestmove', 'bestmove a1a2', None)]:
                entries.append({'engine': who, 'direction': direction, 'phase': phase,
                                'text': text, 'write_completed': completed, 'truncated': False})
        return {'pairing': pairing, 'engines_by_color': [0, 1] if pairing == 0 else [1, 0],
                'stop': 'chess_conclusion', 'failure': None, 'diagnostic_failures': [],
                'transcript': {'truncated': False, 'dropped_bytes': 0, 'entries': entries},
                'engines': [{'exit': {'success': True}, 'cleanup_failures': [],
                             'identity_truncated': False,
                             'stderr': {'observed_bytes': 0, 'truncated': False}} for _ in range(2)]}

    report = {'config': expected, 'games': [game(0), game(1)], 'engine_identities': identities,
              'summaries': [{'unscored': 0, 'failures': 0, 'games': 2, 'wins': [1, 1], 'draws': 0}]}
    audit_identities(report, expected, hashes, 'external', 'candidate')
    assert eligible(report, expected) == (2, [])
    bad = copy.deepcopy(report)
    bad['engine_identities'][0]['executable']['sha256'] = 'f'*64
    refuses(lambda: audit_identities(bad, expected, hashes, 'external', 'candidate'))
    bad = copy.deepcopy(report)
    bad['engine_identities'][1]['model_identity'] = 'wrong model'
    refuses(lambda: audit_identities(bad, expected, hashes, 'external', 'candidate'))
    for field, value in [('claim_policy', 'explicit_only'), ('max_plies', 128),
                         ('io', {}), ('work', {'depth': 1}), ('time_control', 'wrong')]:
        bad = copy.deepcopy(report)
        bad['config'][field] = value
        assert eligible(bad, expected)[1], field
    bad = copy.deepcopy(report)
    bad['config']['engines'][1]['options'][0]['value'] = '2'
    assert eligible(bad, expected)[1]
    for text in ['info depth 9 nodes 100 score cp 0',
                 'info depth 8 nodes 10001 score cp 0',
                 'info depth 0 nodes 100 score cp 0',
                 'info depth 8 nodes 0 score cp 0', 'info depth 8 score cp 0']:
        bad = copy.deepcopy(report)
        bad['games'][0]['transcript']['entries'][1]['text'] = text
        assert eligible(bad, expected)[1], text
    bad = copy.deepcopy(report)
    bad['games'][0]['engines'][0]['identity_truncated'] = True
    assert eligible(bad, expected)[1]
    bad = copy.deepcopy(report)
    bad['games'][0]['transcript']['entries'][1]['truncated'] = True
    assert eligible(bad, expected)[1]
    with tempfile.TemporaryDirectory(prefix='msh49-audit-lock-') as temporary:
        path = Path(temporary)/'ledger.json'
        with ledger_lock(path):
            try:
                with ledger_lock(path):
                    raise AssertionError('overlapping ledger manager acquired lock')
            except FileExistsError:
                pass
        assert not Path(str(path)+'.lock').exists()


if __name__ == '__main__':
    self_test()
    parser = argparse.ArgumentParser()
    parser.add_argument('--self-test', action='store_true')
    parser.add_argument('--ledger')
    parser.add_argument('--freeze')
    parser.add_argument('--runner')
    parser.add_argument('--reports-root')
    args = parser.parse_args()
    if args.self_test:
        print('exact sign tails and constructed identity/config/work/truncation/budget/lock audit mutations passed')
    else:
        if not (args.ledger and args.freeze and args.runner):
            parser.error('ledger, freeze, runner required')
        print(json.dumps(analyse(args), indent=2))
