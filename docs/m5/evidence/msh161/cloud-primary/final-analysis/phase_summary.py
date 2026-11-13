"""Offline descriptive phase sensitivity over completed native histories."""
import collections,gzip,hashlib,json,sys
from pathlib import Path
import native_statistics as ns
sys.path.insert(0,str(ns.STUDY))
import phase_tags
H=ns.H;F=Path(__file__).resolve().parent
sha=ns.sha

def run():
    v=ns.read(H/'primary-cumulative.json');ns.closure_guard(v)
    supported={(r['pair'],r['game'],r['ply']) for r in v['normalized_selected_events'] if r['candidate'] and r['status']=='supported_finite'}
    rows=[];games=[];cache={};counts=collections.defaultdict(collections.Counter);changes=collections.Counter()
    for item in v['native_games']:
        pair=item['pair'];gi=item['game'];a=item['native_report'];path=H/'local-cache'/a['key']
        if pair not in cache:
            if sha(path)!=a['sha256']:raise ValueError('Native source changed')
            report=ns.analysis.load_report(path);ds,gs=ns.analysis.extract_decisions(report);cache[pair]=(report,ds,gs)
        report,ds,gs=cache[pair];g=gs[gi];record=ns.analysis.decode(report['games'][gi]['match_record_json']);initial=ns.analysis.decode(record['initial_record_json'])
        tagged=phase_tags.tag_history(g['initial_fen'],g['complete_history'],initial['history_origin']);boundary=tagged['opening_boundary_ply'];positions=tagged['positions']
        games.append({'pair':pair,'game':gi,'opening_boundary_ply':boundary,'unresolved':tagged['opening_boundary_unresolved'],'reason':tagged['opening_boundary_reason']})
        for d in ds:
            if d['game']!=gi or d['origin']!='played':continue
            p=positions[d['ply']];material=p['indicators']['nonpawn_nonking_points'];base=p['phase'];variants={}
            for label,cutoff,shift in [('material24',24,0),('material28',28,0),('boundary_minus2',26,-2),('boundary_plus2',26,2)]:
                variants[label]='endgame' if material<=cutoff else ('middlegame' if boundary is not None and d['ply']>=max(0,boundary+shift) else ('opening' if p['indicators']['original_minor_identities_known'] else 'opening_or_middlegame_unresolved'))
            supported_root=(pair,gi,d['ply']) in supported
            row={'pair':pair,'game':gi,'ply':d['ply'],'candidate':d['engine_index']==0,'phase':base,'material':material,'variants':variants,'supported_candidate':supported_root};rows.append(row)
            counts['all_played'][base]+=1
            if row['candidate']:counts['candidate_played'][base]+=1
            if supported_root:counts['supported_candidate'][base]+=1
            for label,phase in variants.items():
                counts[label+'_all_played'][phase]+=1
                if supported_root:counts[label+'_supported_candidate'][phase]+=1
                if phase!=base:
                    changes[label+'_all_played']+=1
                    if supported_root:changes[label+'_supported_candidate']+=1
    if len(rows)!=v['played_decisions'] or sum(r['supported_candidate'] for r in rows)!=len(supported):raise ValueError('Coverage differs')
    out={'status':'complete_needs_independent_phase_review','implementation_sha256':sha(__file__),'phase_helper_sha256':sha(Path(phase_tags.__file__)),'native_statistics_sha256':sha(Path(ns.__file__)),'cumulative_sha256':sha(H/'primary-cumulative.json'),'method':'Frozen complete-history mechanical phase; material26 override. Sensitivity24/28 and resolved opening boundary +/-2plies. Context moves excluded from counts. No position-quality claim.','counts':dict(counts),'changed_assignments':dict(changes),'unresolved_opening_boundaries':sum(g['unresolved'] for g in games),'games':games,'played_roots':rows}
    path=F/'phase-summary.json.gz'
    with path.open('xb') as f:f.write(gzip.compress(json.dumps(out,indent=2).encode(),mtime=0))
    print(json.dumps({k:out[k] for k in ['status','counts','changed_assignments','unresolved_opening_boundaries']}))
if __name__=='__main__':run()
