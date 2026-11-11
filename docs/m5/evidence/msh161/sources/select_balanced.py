#!/usr/bin/env python3
"""Strict v0 screening ledger and qualification panel, no engine execution.
Main90 selection remains unfinished if exact final-score certainty/coverage fail.
Run from any directory; input identities and every status are retained.
"""
import gzip,hashlib,itertools,json
from collections import Counter,defaultdict
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
read=lambda p:json.loads(p.read_text())
save=lambda p,v:p.write_text(json.dumps(v,indent=2)+'\n')
roots=json.loads(gzip.decompress((HERE/'opening-candidates.json.gz').read_bytes()))
byid={r['id']:r for r in roots};receipts={};records=[]
screen_manifest=read(ROOT/'balance-screen/manifest.json')
for p in sorted((ROOT/'balance-screen').glob('g*-attempt0000.json')):
 x=read(p);d=x['decision'];r=byid[d['opening_id']]
 assert x['engine_sha256']==screen_manifest['engine_sha256'] and x['options']==screen_manifest['options'] and x['nodes_requested']==screen_manifest['nodes']
 assert d['history']==r['moves'] and d['mover']=='white' and d['origin']=='supplied_root'
 a=x.get('assessment') or {};s=a.get('score') or {};score=s.get('score') or {};e=a.get('expected_score_mover');reasons=list(a.get('uncertainty',[]))
 if x.get('status')!='complete' or x.get('exit_code')!=0:reasons.append('query_not_complete_success')
 if not x.get('process_group_settled'):reasons.append('process_cleanup_unsettled')
 if x.get('raw_truncated'):reasons.append('truncated_transcript')
 if not x.get('pv_independent_legal'):reasons.append('pv_not_verified_legal')
 if s.get('lowerbound') or s.get('upperbound'):reasons.append('selected_score_bound')
 if not s.get('pv') or s['pv'][0]!=a.get('bestmove'):reasons.append('selected_pv_not_aligned')
 if e is None:reasons.append('missing_expected_score')
 if s.get('wdl') is not None:
  w,draw,loss=s['wdl'];assert w+draw+loss==1000
  if e is not None:assert abs(e-(w+draw/2)/1000)<1e-12
 status='uncertain' if reasons else 'accepted' if score.get('kind')=='cp' and abs(score['value'])<=60 and .40<=e<=.60 else 'outside_balance'
 rec={'id':r['id'],'family':r['family'],'candidate_rank':r['candidate_rank'],'status':status,'uncertainty':reasons,'white_score':score,'expected_score_white_proxy':e,'depth':s.get('depth'),'nodes_requested':x['nodes_requested'],'receipt':str(p.relative_to(ROOT)),'receipt_sha256':sha(p),'tags':r['tags']}
 records.append(rec);receipts[r['id']]=rec
assert len(records)==len(roots)==330
# Qualification anchors chosen by declared structural contrasts, then source rank;
# never by competitive game results. All pass strict v0 balance acceptance.
def first(family,predicate):
 return min((r for r in roots if r['family']==family and receipts[r['id']]['status']=='accepted' and predicate(r['tags'])),key=lambda r:r['candidate_rank'])
panel=[first('spanish',lambda t:t['pawn_structure_mirror_symmetric']),first('french',lambda t:t['center_regime']=='closed'),first('slav-semi-slav',lambda t:len({c['wing'] for c in t['castles']})==2),first('sicilian-najdorf-dragon-scheveningen-rauzer',lambda t:t['center_regime']=='open' and not t['pawn_structure_mirror_symmetric'])]
save(HERE/'qualification-roots.json',[{k:r[k] for k in ['id','fen','moves']} for r in panel])
save(HERE/'qualification-selection.json',{'status':'accepted_for_opponent_qualification_only','method':'first strict-accepted source-rank root within four predeclared contrasts: symmetric Spanish, closed French, opposite-castled Slav, asymmetric open Sicilian','roots':[dict(receipts[r['id']],opening_name=r['opening_name'],moves=r['moves'],source_game=r['source_game']) for r in panel],'source_candidates_sha256':sha(HERE/'opening-candidates.json.gz'),'qualification_roots_sha256':sha(HERE/'qualification-roots.json')})
# A minimal conservative confirmation shortlist fills original per-family
# structural quotas. This is NOT a run request or permission to clear uncertainty.
m=read(HERE/'manifest.json');groups=defaultdict(list)
for r in roots:
 if receipts[r['id']]['status']!='outside_balance':groups[r['family']].append(r)
def conditions(r):
 t=r['tags'];return {'symmetric':t['pawn_structure_mirror_symmetric'],'opposite_castled':len({c['wing'] for c in t['castles']})==2,'closed':t['center_regime']=='closed','open':t['center_regime']=='open'}
provisional=[];gaps=[]
for fam,q in m['structural_quotas'].items():
 combos=[c for c in itertools.combinations(groups[fam],4) if all(sum(conditions(r)[k] for r in c)>=n for k,n in q.items())]
 if not combos:gaps.append(fam);continue
 chosen=min(combos,key=lambda c:(sum(receipts[r['id']]['status']=='uncertain' for r in c),tuple(sorted(r['candidate_rank'] for r in c))))
 provisional.extend(chosen)
# Two declared gap fills would be selected later, after method review and all
# structural acceptance. Do not publish an unsupported frozen90 at this stage.
shortlist=[r for r in provisional if receipts[r['id']]['status']=='uncertain']
assert len(shortlist)<=100
save(HERE/'confirmation-shortlist.json',{'status':'proposal_only_wait_for_method_review_not_automatic_engine_request','rationale':'Minimum unresolved roots needed for four/family while preserving original per-family structural quotas; certainty first, then lexicographic source ranks. Two gap-fill roots not selected.','nodes_if_requested':5000000,'roots':[{'id':r['id'],'fen':r['fen'],'moves':r['moves'],'family':r['family'],'candidate_rank':r['candidate_rank'],'uncertainty':receipts[r['id']]['uncertainty']} for r in shortlist]})
counts=Counter(r['status'] for r in records);family={fam:dict(Counter(r['status'] for r in records if r['family']==fam)) for fam in m['family_available']}
result={'status':'main_sample_unfinished_not_frozen','screen_method':'strict_v0','acceptance':'successful complete untruncated settled query; legal PV aligned to bestmove; no uncertainty or score bound; abs White cp <=60 AND White WDL expected-score proxy in[.40,.60]','uncertainty_policy':'Final-score flags retained; not called bad. Any method amendment must preserve this original ledger and pass independent review.','source_candidates_sha256':sha(HERE/'opening-candidates.json.gz'),'screen_manifest_sha256':sha(ROOT/'balance-screen/manifest.json'),'counts':dict(counts),'families':family,'confirmation_proposal_count':len(shortlist),'family_structural_infeasibility':gaps,'main_selection':None,'qualification_ids':[r['id'] for r in panel],'records':records}
save(HERE/'selection.json',result)
print(json.dumps({k:result[k] for k in ['status','counts','confirmation_proposal_count','family_structural_infeasibility','qualification_ids']},indent=2))
