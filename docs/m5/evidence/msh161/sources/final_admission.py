#!/usr/bin/env python3
"""Final opening admission after a completed 5M structure-gap screen.
Usage: python final_admission.py CONFIRMATION_DIRECTORY
No engines or competitive-game reads. Never weakens balance or coverage gates.
"""
import collections,gzip,hashlib,json,sys,argparse
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parent
sys.path.insert(0,str(ROOT));from screen_v1 import describe
read=lambda p:json.loads(p.read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
save=lambda p,v:p.write_text(json.dumps(v,indent=2)+'\n')
parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('confirmation_directory');args=parser.parse_args()
confirmation=Path(args.confirmation_directory).resolve()
if (confirmation/'inflight.json').exists():raise RuntimeError('Confirmation query still in flight')
if (HERE/'main-roots.json').exists() or (HERE/'main-selection.json').exists():raise FileExistsError('Preserve existing final admission; do not overwrite')
roots=json.loads(gzip.decompress((HERE/'opening-candidates.json.gz').read_bytes()))
supplement=json.loads(gzip.decompress((HERE/'supplement-candidates-v1.json.gz').read_bytes()))
for r in supplement:r['candidate_rank']=15+r['supplement_rank']
roots+=supplement
byid={r['id']:r for r in roots};source_order={r['id']:i for i,r in enumerate(roots)}
excluded={r['id'] for r in read(HERE/'qualification-roots.json')}
manifest=read(ROOT/'balance-screen/manifest.json');confirmation_manifest=read(confirmation/'manifest.json');records=[];byreceipt={}
assert confirmation_manifest['engine_sha256']==manifest['engine_sha256'] and confirmation_manifest['options']==manifest['options']
assert confirmation_manifest['nodes']==5000000
expected_confirm={r['id'] for r in read(HERE/'next-balance-roots-v1.json')}
actual_confirm=set();latest={};prior={}
for folder,m in [(ROOT/'balance-screen',manifest),(confirmation,confirmation_manifest)]:
 for p in sorted(folder.glob('g*-attempt*.json')):
  x=read(p);rid=x['decision']['opening_id']
  assert x['engine_sha256']==m['engine_sha256'] and x['options']==m['options'] and x['nodes_requested']==m['nodes']
  if folder==confirmation:
   assert rid in expected_confirm;actual_confirm.add(rid)
  if rid in latest:prior.setdefault(rid,[]).append({'receipt':str(latest[rid]),'sha256':sha(latest[rid])})
  latest[rid]=p
assert actual_confirm==expected_confirm, 'Incomplete32-root follow-up; wait or preserve failure, never silently fall back'
for rid,p in sorted(latest.items(),key=lambda kv:source_order[kv[0]]):
 x=read(p);d=x['decision'];r=byid[d['opening_id']];v=describe(x);s=v['selected_report']or{};sc=s.get('score',{});e=v['expected_score_mover']
 assert d['history']==r['moves'] and d['mover']=='white' and d['origin']=='supplied_root'
 flags=set(v['flags']);blocking=flags-{'last_score_update_not_selected_exact_score'}
 if x.get('exit_code')!=0:blocking.add('nonzero_exit')
 if not x.get('process_group_settled'):blocking.add('unsettled_process')
 if x.get('raw_truncated'):blocking.add('truncated_raw')
 if not x.get('pv_independent_legal'):blocking.add('unverified_legal_pv')
 if not v['stable_report_heuristic']:blocking.add('distinct_depth_stability_not_met')
 blocking.update(v['directional_bound_conflicts'])
 window=sc.get('kind')=='cp' and abs(sc['value'])<=60 and e is not None and .4<=e<=.6
 status='unresolved' if blocking else 'provisionally_accepted' if window else 'outside_balance_window'
 rec={'id':r['id'],'family':r['family'],'candidate_rank':r['candidate_rank'],'status':status,'excluded_qualification_exposure':r['id'] in excluded,'white_score':sc,'expected_score_white_proxy':e,'inside_numeric_window':window,'original_flags':v['flags'],'blocking_flags':sorted(blocking),'last_two_depth_expected_score_range':v['last_two_expected_score_range'],'last_two_spread':v['last_two_spread'],'selected_report':s,'final_report':v['final_report'],'receipt':str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p),'receipt_sha256':sha(p),'nodes_requested':x['nodes_requested'],'superseded_receipts':prior.get(rid,[])}
 records.append(rec);byreceipt[r['id']]=rec
assert len(records)==350
families=list(read(HERE/'manifest.json')['family_available'])
def tags(r):
 t=r['tags'];return {'symmetric':t['pawn_structure_mirror_symmetric'],'opposite_castled':len({c['wing'] for c in t['castles']})==2,'closed':t['center_regime']=='closed','open':t['center_regime']=='open'}
minimum={'symmetric':12,'opposite_castled':10,'closed':15,'open':15}
accepted=[r for r in roots if byreceipt[r['id']]['status']=='provisionally_accepted' and r['id'] not in excluded]
available={k:sum(tags(r)[k] for r in accepted) for k in minimum}
# Source-order constrained sampling. Scarce structural kinds first; within a kind
# use original source family/rank order, never numeric score size or game results.
# Four/family cap leaves two explicit additional structural gap-fill slots.
selected=[];perfamily=collections.Counter()
def add(r,cap=4):
 if r in selected or perfamily[r['family']]>=cap:return False
 selected.append(r);perfamily[r['family']]+=1;return True
def coverage():return {k:sum(tags(r)[k] for r in selected) for k in minimum}
for kind in sorted(minimum,key=lambda k:(available[k]/minimum[k],list(minimum).index(k))):
 target=min(minimum[kind],available[kind])
 for r in accepted:
  if coverage()[kind]>=target:break
  if tags(r)[kind]:add(r)
for fam in families:
 for r in accepted:
  if perfamily[fam]>=4:break
  if r['family']==fam:add(r)
assert all(perfamily[f]==4 for f in families)
# Two gap fills maximize outstanding target coverage, then least represented
# structure among already reached kinds, then original source order.
for _ in range(2):
 c=coverage();pool=[r for r in accepted if r not in selected]
 def priority(r):
  t=tags(r);unmet=sum(t[k] for k in minimum if c[k]<minimum[k]);diversity=sum(t[k]/max(1,c[k]) for k in minimum)
  return (-unmet,-diversity,source_order[r['id']])
 add(min(pool,key=priority),cap=6)
selected.sort(key=lambda r:source_order[r['id']]);c=coverage();deficits={k:minimum[k]-c[k] for k in minimum if c[k]<minimum[k]}
main=[{k:r[k] for k in ['id','fen','moves']} for r in selected]
if deficits:
 failure={'status':'blocked_on_structural_coverage_no_final_admission','deficits':deficits,'available':available,'selected_coverage':c,'records':records}
 save(HERE/'main-admission-unresolved.json',failure)
 raise RuntimeError('Coverage not met; preserve criteria and investigate: '+str(deficits))
main_bytes=(json.dumps(main,indent=2)+'\n').encode()
out={'status':'admitted_pending_independent_review','method':'opening_balance_v1','acceptance':'Aligned nonbound abs White cp<=60 and Ewhite[.40,.60]; last two distinct-depth aligned estimates spread<=.025; no directional final-bound conflict; legal untruncated complete settled query. Sole stale-final-report flag is retained but not disqualifying. Finite screening, not proved equality.','source_candidates_sha256':sha(HERE/'opening-candidates.json.gz'),'screen_method_sha256':sha(ROOT/'screen_v1.py'),'screen_manifest_sha256':sha(ROOT/'balance-screen/manifest.json'),'qualification_excluded_ids':sorted(excluded),'counts':dict(collections.Counter(r['status'] for r in records)),'eligible_pool_structure':available,'required_minimums':minimum,'selection_size':len(selected),'selection_coverage':c,'selection_family_counts':dict(perfamily),'structural_deficits':deficits,'main_roots_sha256':hashlib.sha256(main_bytes).hexdigest(),'script_sha256':sha(Path(__file__)),'supplement_manifest_sha256':sha(HERE/'supplement-manifest-v1.json'),'confirmation_manifest_sha256':sha(confirmation/'manifest.json'),'selection_policy':'Source-order constrained sampling by scarce structural kind, original family/rank order within kind; four/family then two structural gap fills. Numerical scores are admission gates only; no competitive results read. Higher-work32-root receipts supersede earlier finite reports, which remain linked.','selected_ids':[r['id'] for r in selected],'selected_roots':[{'id':r['id'],'family':r['family'],'candidate_rank':r['candidate_rank'],'opening_name':r['opening_name'],'root_fen':r['root_fen'],'source_game':r['source_game'],'source_collection':'supplement-candidates-v1.json.gz' if 'supplement_rank' in r else 'opening-candidates.json.gz','tags':r['tags'],'screen_receipt':byreceipt[r['id']]['receipt'],'screen_receipt_sha256':byreceipt[r['id']]['receipt_sha256']} for r in selected],'records':records}
assert len(selected)==90 and not excluded.intersection(out['selected_ids'])
assert all(c[k]>=minimum[k] for k in minimum)
assert all(perfamily[f]>=4 for f in families)
assert len({byid[r['id']]['root_key'] for r in main})==90
assert all(byreceipt[r['id']]['status']=='provisionally_accepted' for r in main)
(HERE/'main-roots.json').write_bytes(main_bytes)
save(HERE/'main-selection.json',out)
print(json.dumps({k:out[k] for k in ['status','counts','eligible_pool_structure','selection_coverage','structural_deficits','selection_family_counts']},indent=2))
