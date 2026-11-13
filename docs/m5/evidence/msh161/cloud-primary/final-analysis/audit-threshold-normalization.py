"""Offline independent threshold audit. Does not change any retained disposition."""
import collections
if not __debug__:
    raise RuntimeError("Optimized Python disables audit checks")
import hashlib
import json
from pathlib import Path

F = Path(__file__).resolve().parent
H = F.parent
THRESHOLDS = (0.04, 0.05, 0.06)


def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def numeric_range(value):
    return isinstance(value, list) and len(value) == 2 and all(isinstance(v, (int, float)) for v in value)


def margin_range(row):
    """Return only a retained stable comparison; do not use selected loss."""
    ranges = row.get('ranges', row.get('comparison_ranges'))
    if isinstance(ranges, dict) and all(numeric_range(ranges.get(k)) for k in ('played', 'alternative')):
        if 'blocking_flags' in row:
            stable = not row['blocking_flags']
        else:
            stable = old_queries_stable(row)
        if stable:
            p, a = ranges['played'], ranges['alternative']
            return [a[0] - p[1], a[1] - p[0]]
    if 'stable_finite_comparison' in row:
        if row['stable_finite_comparison'] and numeric_range(row.get('range')):
            return row['range']
        return None
    if numeric_range(row.get('sensitivity_range')):
        sens = row.get('threshold_sensitivity', {})
        if any(x in ('supported', 'below', 'straddles') for x in sens.values()):
            return row['sensitivity_range']
    if old_queries_stable(row):
        p, a = old_pair(row)
        return [a['range'][0] - p['range'][1], a['range'][1] - p['range'][0]]
    return None


def old_pair(row):
    moves = [row.get('played'), row.get('compared_alternative')]
    result = []
    for move in moves:
        matches = [q for q in row.get('queries', []) if q.get('forced_move') == move and move]
        if not matches:
            return None
        # Followup source order is retained. Distinct receipts for a move must agree.
        signatures = {(tuple(q.get('range', [])), q.get('stable_eligible')) for q in matches}
        if len(signatures) != 1:
            raise ValueError(('ambiguous old singleton', row['pair'], row['game'], row['ply']))
        result.append(matches[-1])
    return result


def old_queries_stable(row):
    pair = old_pair(row)
    return bool(pair and all(q.get('stable_eligible') is True and numeric_range(q.get('range')) for q in pair))


def classify_range(rng, threshold, epsilon=0):
    if rng is None:
        return 'unresolved'
    lo, hi = rng
    return 'supported' if lo + epsilon >= threshold else 'below' if hi + epsilon < threshold else 'straddles'


def audit(row, normalized_status, threshold, epsilon=0):
    if normalized_status == 'claim_policy_limited':
        return 'claim_limited'
    if normalized_status == 'same_choice_control':
        return 'same_choice'
    return classify_range(margin_range(row), threshold, epsilon)


def fixtures():
    # Independent hand-derived range: [.56-.50,.57-.49] == [.06,.08].
    old = {'played': 'a2a3', 'compared_alternative': 'a2a4', 'queries': [
        {'forced_move': 'a2a3', 'range': [.49,.50], 'stable_eligible': True},
        {'forced_move': 'a2a4', 'range': [.56,.57], 'stable_eligible': True}]}
    assert audit(old, 'supported_finite', .05) == 'supported'
    assert audit(old, 'claim_policy_limited', .05) == 'claim_limited'
    assert audit(old, 'same_choice_control', .05) == 'same_choice'
    old['queries'][1]['stable_eligible'] = False
    assert audit(old, 'unresolved', .05) == 'unresolved'
    assert classify_range([.04,.06], .05) == 'straddles'
    assert classify_range([.03,.0395], .04) == 'below'
    assert classify_range([.04999999999999999,.051], .05) == 'straddles'
    assert classify_range([.04999999999999999,.051], .05, 1e-9) == 'supported'
    assert margin_range({'ranges': {'played': [.4,.4], 'alternative': [.5,.5]}, 'blocking_flags':['bound conflict']}) is None
    assert margin_range({'range':[.1,.2], 'stable_finite_comparison':False}) is None
    return 10


def run():
    v = json.loads((H/'primary-cumulative.json').read_text())
    assert v['games_completed'] == 200 and v['remaining_missing_singletons'] == 0
    cache, bindings, rows = {}, {}, []
    for n in v['normalized_selected_events']:
        source = n['source']
        if source not in cache:
            path = H/source
            d = json.loads(path.read_text())
            cache[source] = {(r['pair'], r['game'], r['ply']): r for r in d['decisions']}
            bindings[source] = sha(path)
        row = cache[source][(n['pair'],n['game'],n['ply'])]
        rows.append({k:n.get(k) for k in ('pair','game','ply','candidate','status','secondary','source')} | {
            'source_status':row['status'], 'stable_margin_range':margin_range(row),
            'source_thresholds':row.get('threshold_sensitivity',row.get('sensitivity',{})),
            'literal_float':{str(t):audit(row,n['status'],t) for t in THRESHOLDS},
            'uniform_1e9_tolerance':{str(t):audit(row,n['status'],t,1e-9) for t in THRESHOLDS}})
    counts = lambda field,predicate:{str(t):dict(collections.Counter(r[field][str(t)] for r in rows if predicate(r))) for t in THRESHOLDS}
    differences = [r for r in rows if r['literal_float'] != r['uniform_1e9_tolerance']]
    # Original baseline labels remain authoritative even if arithmetic is on a float edge.
    baseline_diff = [r for r in rows if (r['status']=='supported_finite') != (r['literal_float']['0.05']=='supported')]
    nonfinite_straddlers=[r for r in rows if r['candidate'] and r['status']!='supported_finite' and 'straddles' in r['uniform_1e9_tolerance'].values()]
    out={'scope':'All closed selected roots, both players; offline range sensitivity only',
        'cumulative_sha256':sha(H/'primary-cumulative.json'),'implementation_sha256':sha(__file__),
        'fixtures_passed':fixtures(),'source_bindings':bindings,
        'original_candidate_supported_finite':sum(r['candidate'] and r['status']=='supported_finite' for r in rows),
        'literal_float':{'candidate':counts('literal_float',lambda r:r['candidate']),'both_players':counts('literal_float',lambda r:True)},
        'uniform_1e9_tolerance':{'candidate':counts('uniform_1e9_tolerance',lambda r:r['candidate']),'both_players':counts('uniform_1e9_tolerance',lambda r:True)},
        'original_baseline_vs_literal_disagreements':baseline_diff,'float_boundary_cases':differences,
        'nonfinite_candidate_stable_straddlers':nonfinite_straddlers,'rows':rows,
        'limits':['Original status remains unchanged. Uniform tolerance is an explicit sensitivity, not a source upgrade.',
                  'Margin ranges are observed-report envelopes, not statistical confidence bounds.',
                  'Selected-root counts are not all-move error rates. Claim and same-choice roots stay separate.',
                  'No engine query, cloud action or source change was used.']}
    path=F/'audit-threshold-normalization.json'
    path.write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:v for k,v in out.items() if k not in ('rows','source_bindings','nonfinite_candidate_stable_straddlers','float_boundary_cases','original_baseline_vs_literal_disagreements')},indent=2))
    print('baseline differences',[(r['pair'],r['game'],r['ply'],r['status'],r['literal_float']['0.05']) for r in baseline_diff])
    print('float boundaries',len(differences),'nonfinite candidate straddlers',len(nonfinite_straddlers))

if __name__=='__main__':
    run()
