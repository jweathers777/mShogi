# Offline numerical derivation only. Reproduce from the archived through-Batch-06 cumulative bytes, not over a later cumulative record. Qualitative category annotations are separately documented in closure.md.
import collections,gzip,hashlib,json
from pathlib import Path
H=Path(__file__).resolve().parents[1]
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,v):p.write_text(json.dumps(v,indent=2)+'\n')
p=H/'primary-cumulative.json';raw=p.read_bytes();v=json.loads(raw);assert v['batches_reviewed']==['01','02','03','04','05','06']
archive=H/'batch-07-review/primary-cumulative-through-batch06.json.gz';archive.write_bytes(gzip.compress(raw,mtime=0));assert gzip.decompress(archive.read_bytes())==raw
plan=read(H/'batch-07-adjudication/plan.json');roots={(j['spec']['id'],j['game'],j['ply']):j['adjudication']['decision'] for j in plan['jobs']}
rows=[];sources=[]
allowed={'supported_finite','supported_subthreshold','same_choice_control','unresolved','claim_policy_limited'}
for name in ('final-dispositions-00-04.json','final-dispositions-05-09.json'):
 source='batch-07-adjudication/'+name;data=read(H/source);sources.append(source)
 for row in data['decisions']:
  key=(row['pair'],row['game'],row['ply']);actual=roots[key];assert row['played']==actual['played'] and row['candidate']==(actual['engine_index']==0);assert row['status'] in allowed
  item={k:row.get(k) for k in ('pair','game','ply','candidate','played','alternative','status','margin','severity','secondary','threshold_sensitivity')};item.update(batch='07',source=source,source_row_key={'pair':key[0],'game':key[1],'ply':key[2]},first_largest_final=False,uncertainty_and_flags='Retained in exact final overlay and immutable prior ledgers; unresolved earlier/linked events remain. Secondary comparison support is not a fresh primary loss cause.')
  if item['status']=='supported_finite':assert item['margin']>=.05
  rows.append(item)
assert len(rows)==len({(x['pair'],x['game'],x['ply']) for x in rows})==633
idx=json.loads(gzip.decompress((H/'batch-07-review/review-index.json.gz').read_bytes()));games=[];claims=[];out=collections.Counter();prepared=collections.Counter();unaided=collections.Counter()
for pair,entry in idx.items():
 art=entry['game_pair']['artifacts']['pair/report.json.gz'];path=H/'local-cache'/art['key'];assert sha(path)==art['sha256'];outer=json.loads(gzip.decompress(path.read_bytes()));report=json.loads(outer['report_json']);assert report['config']['claim_policy']=='auto_claim'
 ref={'path':str(path.relative_to(H)),'key':art['key'],'sha256':art['sha256']}
 for g in report['games']:
  assert g['stop']=='chess_conclusion' and not g['failure'];record=json.loads(g['match_record_json']);events=record['events'];moves=[e for e in events if isinstance(e['request']['command'],dict) and 'move' in e['request']['command']];conclusion=g['conclusion'];chess=conclusion['chess'];prefix=len(report['config']['openings'][g['opening_index']]['moves']);candidate_color='white' if g['engines_by_color'][0]==0 else 'black'
  result='draws' if 'draw' in chess else ('wins' if chess['checkmate']['winner']==candidate_color else 'losses');out[result]+=1;(unaided if pair.endswith('unaided') else prepared)[result]+=1
  games.append({'batch':'07','pair':pair,'game':g['pairing'],'candidate_result':result,'played_decisions':len(moves),'supplied_context_decisions':prefix,'native_report':ref,'native_conclusion':conclusion,'controller_policy':'auto_claim','optional_claim_policy_potentially_affected':True,'search_policy_potentially_affected':True,'actual_optional_claim_draw':any(isinstance(e['request']['command'],dict) and 'claim' in e['request']['command'] for e in events)})
  for ei,e in enumerate(events):
   cmd=e['request']['command']
   if isinstance(cmd,dict) and 'claim' in cmd:
    assert not e['reply']['rejection'];claims.append({'batch':'07','pair':pair,'game':g['pairing'],'event_index':ei,'played_plies_including_preparation':prefix+len(moves),'reason':cmd['claim']['reason'],'actor':e['request']['actor'],'intended_move_unplayed':cmd['claim'].get('intended'),'request_version':e['request']['version'],'reply_version':e['reply']['version'],'native_report':ref,'native_event':e,'policy_potentially_affected':True,'interpretation':'Actual native draw preserved; intended claim move unplayed. Analyze preceding choices; draw alone is not weakness.'})
assert dict(out)=={'wins':11,'losses':5,'draws':4} and len(claims)==3
assert sum(g['played_decisions'] for g in games)==2325 and sum(g['supplied_context_decisions'] for g in games)==288
sources+=['batch-07-review/integrity-review.json','batch-07-review/review-index.json.gz','batch-07-adjudication/plan.json','batch-07-adjudication/review20m-00-04.md','batch-07-adjudication/review20m-05-09.md','batch-07-followup/plan.json','batch-07-followup/review-00-04.json','batch-07-followup/review-05-09.json','batch-07-followup/runtime-integrity-review.json','batch-07-followup/forecast-update.json','autoclaim-restoration.json']
def counts(items):return dict(collections.Counter(x['status'] for x in items))
def stats(items):
 cand=[x for x in items if x['candidate']];sup=[x for x in cand if x['status']=='supported_finite'];return {'selected20m_decisions':len(items),'selected20m_statuses_both_players':counts(items),'selected20m_candidate_decisions':len(cand),'candidate_statuses':counts(cand),'candidate_supported_severity':dict(collections.Counter(x['severity'] for x in sup)),'candidate_supported_games':len({(x['pair'],x['game']) for x in sup}),'candidate_selected_finite_sensitivity_counts':{str(t):sum(x['status'] in ('supported_finite','supported_subthreshold') and x.get('margin') is not None and x['margin']>=t for x in cand) for t in (.04,.05,.06)}}
summary={'batch':'07','controller_policy':'auto_claim','uci_search_policy':'original claim-capable optional choices; recorded legacy auto option','games_completed':20,'outcomes':dict(out),'prepared_outcomes':dict(prepared),'unaided_outcomes':dict(unaided),'played_decisions':2325,'supplied_context_decisions':288,'initial_queries':5226,'matched5m_queries':2008,'initial_panel_jobs20m':968,'initial_panel_queries20m':992,'additional_followup_jobs20m':9,'additional_followup_queries20m':9,'queries20m':1001,'discovery_roots':24,'new_distinct_discovery_alternatives':3,'new_linked_PV_alternatives':6,'remaining_missing_singletons':0,'sources':[{'path':n,'sha256':sha(H/n)} for n in sources],**stats(rows)}
v['batch_summaries'].append(summary);v['batches_reviewed'].append('07');v['normalized_selected_events']+=rows;allrows=v['normalized_selected_events'];assert len(allrows)==3202 and len({(x['pair'],x['game'],x['ply']) for x in allrows})==3202
v.update(stats(allrows));v['supported_candidate_examples']=[x for x in allrows if x['candidate'] and x['status']=='supported_finite'];v['native_games']+=games;v['optional_claim_draws']+=claims
for k in ('outcomes','prepared_outcomes','unaided_outcomes'):
 v[k]={c:sum(b[k].get(c,0) for b in v['batch_summaries']) for c in ('wins','draws','losses')}
for k in ('games_completed','played_decisions','supplied_context_decisions','initial_queries','matched5m_queries','queries20m','discovery_roots','additional_followup_queries20m'):
 v[k]=sum(b.get(k,0) for b in v['batch_summaries'])
v['new_distinct_discovery_alternatives']+=3;v['new_linked_PV_alternatives']+=6;v['remaining_missing_singletons']=0
assert v['outcomes']=={'wins':58,'draws':41,'losses':41} and v['queries20m']==5384 and len(v['native_games'])==140 and len(v['optional_claim_draws'])==35
v['scope']='Fresh primary standardized cloud only; original AutoClaim conditions throughout batches 01–07. Selected comparison support is not independent error prevalence or final priority.'
v['closure_status']='Batches 01–07 fully reviewed through required 20M and distinct followups; remaining 60 original-condition games proceed in reviewed batches.'
v['policy_strata_source']='autoclaim-restoration.json';v['historical_cancelled_transition_record']='policy-transition/policy-strata.json';v['native_policy_verification']='All 70 native pair configs and 140 games use original AutoClaim; abandoned transition supplied zero primary games.'
v['previous_cumulative_archive']={'path':str(archive.relative_to(H)),'sha256':sha(archive),'uncompressed_sha256':hashlib.sha256(raw).hexdigest()}
v.setdefault('secondary_tag_coverage_history',[]).append(v.get('secondary_tag_coverage'));v['secondary_tag_coverage']={'batch07_explicit_rows':sum(bool(x.get('secondary')) for x in rows),'batch07_supported_secondary_comparisons':sum(bool(x.get('secondary')) and x['status']=='supported_finite' for x in rows),'prior_batches':'Native contextual secondary limitations remain in original ledgers; absent normalized tag does not mean no secondary events.'}
v['interpretation_limits']=['Selected decisions within a game are correlated; overlapping categories and secondary comparisons are not independent loss causes.','Earlier unqueried or unresolved events prevent absolute first/largest claims.','All 35 native automatic-claim draws remain authoritative; intended claim moves are unplayed. Draws alone are not evidence of weakness.','Automatic claims limit observations of unrestricted conversion; analyze preceding choices without inventing counterfactual outcomes.','Finite Stockfish expectation is not empirical win probability or tablebase proof.','Mac, pilot and failed transition evidence are separate. No final rating, strength or engineering priority inferred.']
for n in sources:v['sources'][n]=sha(H/n)
v['derivation']['batch07_normalization']='Final reviewed overlays expose normalized threshold support and secondary tags separately; unresolved/threshold-straddling and claim-policy-limited comparisons remain distinct.'
save(p,v);save(H/'batch-07-review/closure-summary.json',{'status':'closed','batch':summary,'cumulative_outcomes':v['outcomes'],'cumulative_selected_counts':v['selected20m_statuses_both_players'],'cumulative_candidate_counts':v['candidate_statuses'],'cumulative_candidate_severity':v['candidate_supported_severity'],'cumulative_sha256':sha(p),'previous_cumulative_archive':v['previous_cumulative_archive'],'secondary_tag_coverage':v['secondary_tag_coverage']});print(json.dumps({'batch07':stats(rows),'cumulative':stats(allrows),'outcomes':v['outcomes'],'secondary':v['secondary_tag_coverage']}))
