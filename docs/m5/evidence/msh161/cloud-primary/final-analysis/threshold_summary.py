"""Retain heterogeneous source threshold judgments without clearing uncertainty."""
import collections,json,hashlib
from pathlib import Path
import native_statistics as ns
H=ns.H;F=Path(__file__).resolve().parent

def classify(row,t):
    status=row['status'];ts=row.get('threshold_sensitivity',row.get('sensitivity',{}));value=ts.get(str(t))
    if status=='claim_policy_limited':return 'claim_limited'
    if isinstance(value,str):
        return {'supported':'supported','straddles':'straddles','below':'below','unresolved':'unresolved','same_choice_control':'same_choice'}.get(value,'unresolved')
    if status=='same_choice_control':return 'same_choice'
    if value is True:return 'supported'
    if value is None:return 'unresolved'
    # A false source boolean includes both below-threshold and straddling ranges;
    # distinguish them only if the source has no independent blocking flag.
    if row.get('blocking_flags'):return 'unresolved'
    ranges=row.get('ranges')
    if isinstance(ranges,dict) and all(isinstance(ranges.get(k),list) and len(ranges[k])==2 and all(isinstance(x,(int,float)) for x in ranges[k]) for k in ['played','alternative']):
        lo=ranges['alternative'][0]-ranges['played'][1];hi=ranges['alternative'][1]-ranges['played'][0]
        return 'supported' if lo>=t else ('below' if hi<t else 'straddles')
    return 'unresolved'

def run():
    v=ns.read(H/'primary-cumulative.json');ns.closure_guard(v);cache={};bindings={};rows=[]
    for n in v['normalized_selected_events']:
        source=n['source']
        if source not in cache:
            path=H/source;d=ns.read(path);cache[source]={(r['pair'],r['game'],r['ply']):r for r in d['decisions']};bindings[source]=ns.sha(path)
        row=cache[source][(n['pair'],n['game'],n['ply'])]
        rows.append({'pair':n['pair'],'game':n['game'],'ply':n['ply'],'candidate':n['candidate'],'original_status':n['status'],'secondary':n.get('secondary'),'source':source,'thresholds':{str(t):classify(row,t) for t in [.04,.05,.06]}})
    result={'status':'complete_needs_independent_threshold_review','implementation_sha256':ns.sha(__file__),'cumulative_sha256':ns.sha(H/'primary-cumulative.json'),'source_bindings':bindings,'normalization':'Explicit source string judgments retained. True boolean means supported; false with blocking flags stays unresolved. False with no blocking flags and numeric retained ranges is separated into below/straddles using conservative lower and upper differences. Missing/None judgment stays unresolved. Native claim limitations and same-choice controls remain separate. Counts are selected-root sensitivity, not all-move error rates.','both_players':{t:dict(collections.Counter(r['thresholds'][t] for r in rows)) for t in ['0.04','0.05','0.06']},'candidate':{t:dict(collections.Counter(r['thresholds'][t] for r in rows if r['candidate'])) for t in ['0.04','0.05','0.06']},'rows':rows}
    with (F/'threshold-summary.json').open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(json.dumps(result['candidate']))
if __name__=='__main__':run()
