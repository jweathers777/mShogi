"""Offline conservative merge of reviewed behavior packets and correction overlays."""
import collections,copy,gzip,json
from pathlib import Path
import native_statistics as ns
H=ns.H;F=Path(__file__).resolve().parent
key=lambda r:(r['pair'],r['game'],r['ply'])
def unique(rows):
    out={}
    for r in rows:
        if key(r) in out:raise ValueError('Duplicate coded root')
        out[key(r)]=r
    return out

def run():
    v=ns.read(H/'primary-cumulative.json');ns.closure_guard(v)
    expected=unique([r for r in v['normalized_selected_events'] if r['candidate'] and r['status']=='supported_finite'])
    native=ns.read(F/'native-statistics.json');games={(g['pair'],g['game']):g for g in native['games']}
    phases=unique(json.loads(gzip.decompress((F/'phase-summary.json.gz').read_bytes()))['played_roots'])
    audit=ns.read(F/'audit-threshold-normalization.json');thresholds=unique(audit['rows'])
    bindings={};entries={};packet_counts={}
    for label in ['01-04','05-07','08-10']:
        path=F/f'behavior-coding-{label}.json';d=ns.read(path);bindings[path.name]=ns.sha(path)
        source=d.get('events',d.get('entries'));source=[r for r in source if r.get('inclusion',r.get('status'))=='supported_finite'];packet_counts[label]=len(source)
        for r in source:
            k=key(r)
            if k in entries:raise ValueError('Packets overlap')
            n=expected[k]
            if (r['played'],r['alternative'])!=(n['played'],n['alternative']):raise ValueError('Coded choice mismatch')
            g=games[k[:2]];p=phases[k]
            entries[k]=dict(n,categories=copy.deepcopy(r['categories']),packet=label,packet_source=path.name,rationale=r['rationale'],phase=p['phase'],phase_variants=p['variants'],family=g['family'],prepared=g['prepared'],color=g['candidate_color'],result=g['result'],corrections=[])
    if set(entries)!=set(expected):raise ValueError('Incomplete supported coding')
    prior=ns.read(F/'behavior-coding-challenge.json');fresh=ns.read(F/'audit-behavior-corrections.json')
    for name in ['behavior-coding-challenge.json','audit-behavior-corrections.json','audit-threshold-normalization.json','native-statistics.json','phase-summary.json.gz']:bindings[name]=ns.sha(F/name)
    corrections=prior['additive_correction_overlay']['rows']+fresh['new_corrections']
    for r in corrections:
        e=entries[key(r)]
        if e['categories']!=r['before_categories']:raise ValueError('Correction does not match source tags')
        e['categories']=r['after_categories'];e['corrections'].append(r)
    categories=sorted({c for r in entries.values() for c in r['categories']}|{'planning'})
    families=sorted({g['family'] for g in games.values() if g['prepared']})
    def table(rows):
        return {'moves':len(rows),'games':len({(r['pair'],r['game']) for r in rows}),'severity_moves':dict(collections.Counter(r['severity'] for r in rows)),'phase_moves':dict(collections.Counter(r['phase'] for r in rows)),'color_moves':dict(collections.Counter(r['color'] for r in rows)),'result_moves':dict(collections.Counter(r['result'] for r in rows)),'family_moves':dict(collections.Counter(r['family'] for r in rows)),'prepared_moves':sum(r['prepared'] for r in rows),'unaided_moves':sum(not r['prepared'] for r in rows),'secondary_explicit_moves':sum(bool(r.get('secondary')) for r in rows)}
    summaries={}
    for c in categories:
        rows=[r for r in entries.values() if c in r['categories']];prepared=[r for r in rows if r['prepared']]
        s=table(rows);s['prepared_games']=len({(r['pair'],r['game']) for r in prepared});s['unaided_games']=len({(r['pair'],r['game']) for r in rows if not r['prepared']});s['prepared_families']=len({r['family'] for r in prepared});s['families_by_phase_color_result']={f:table([r for r in rows if r['family']==f]) for f in sorted({r['family'] for r in rows})}
        s['threshold_supported_coded_moves']={t:sum(thresholds[key(r)]['literal_float'][t]=='supported' for r in rows) for t in ['0.04','0.05','0.06']}
        s['leave_one_family_out']={f:{'affected_prepared_games':len({(r['pair'],r['game']) for r in prepared if r['family']!=f}),'remaining_prepared_games':sum(g['prepared'] and g['family']!=f for g in games.values()),'remaining_coded_moves':sum(r['family']!=f for r in prepared)} for f in families}
        s['phase_sensitivity']={variant:dict(collections.Counter(r['phase_variants'][variant] for r in rows)) for variant in ['material24','material28','boundary_minus2','boundary_plus2']}
        summaries[c]=s
    for c,expected_counts in fresh['bounded_corrected_descriptive_counts'].items():
        if any(summaries[c][k]!=expected_counts[k] for k in ['moves','games']):raise ValueError('Independent corrected tally mismatch')
    out={'status':'complete_needs_independent_aggregate_review','implementation_sha256':ns.sha(__file__),'cumulative_sha256':ns.sha(H/'primary-cumulative.json'),'source_bindings':bindings,'supported_events':len(entries),'supported_games':len({(r['pair'],r['game']) for r in entries.values()}),'packet_counts':packet_counts,'correction_rows_applied':len(corrections),'category_summaries':summaries,'unresolved_sensitivity':{'source':'audit-threshold-normalization.json','baseline_support':589,'literal_supported_counts':audit['literal_float']['candidate'] if 'candidate' in audit['literal_float'] else audit['literal_float'],'uniform_tolerance_separate':True,'behavioral_limit':'Stable nonfinite straddlers remain separate from589 supported events and mostly lack common-rule category coding. Do not distribute657 unresolved candidate roots into invented categories or claim exact category upper bounds.'},'interpretation_limits':['Overlapping reviewer-dependent descriptive tags, not mutually exclusive defects or all-move error rates.','Unknown86 after bounded corrections; packet granularity differs and has not been exhaustively recoded to one common rule. Category numeric order is not a precise defect-frequency ranking.','Affected games and margins are correlated. Do not sum margins or treat each root as an independent cause.','No secondary tag is not proof of primary causation; historical source caveats remain.','One-sided labels do not establish opponent-created opportunities without supported predecessor/response.','Three reviewed synthesis narratives are illustrative bounded episodes, not an exhaustive conversion or episode census.','Favorable episode criteria, native outcomes and claim censoring remain separate.'],'events':[entries[k] for k in sorted(entries)]}
    with (F/'behavior-aggregate.json').open('x') as f:json.dump(out,f,indent=2);f.write('\n')
    print(json.dumps({'events':len(entries),'corrections':len(corrections),'categories':{c:{k:s[k] for k in ['moves','games','prepared_families']} for c,s in summaries.items()}}))
if __name__=='__main__':run()
