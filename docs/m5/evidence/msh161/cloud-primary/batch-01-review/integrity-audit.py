#!/usr/bin/env python3
"""Offline complete scientific execution audit; no native/engine/network invocation."""
import collections,gzip,hashlib,json,math,sys,time
from pathlib import Path
HERE=Path(__file__).resolve().parent;REPO=HERE.parents[5];STUDY=HERE.parents[1]
sys.path[:0]=[str(REPO),str(STUDY)]
import chess,analysis,phase_tags,screen_v1
from tools.experiments import core,cli
read=lambda p:json.loads(Path(p).read_text())
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
index=read(HERE/'review-index.json');manifest=read(HERE.parent/'batch-01-manifest.json');store=core.LocalStore(HERE.parent/'local-cache')
stages=collections.defaultdict(collections.Counter);flags=collections.defaultdict(collections.Counter);counts=collections.Counter();outcomes=collections.defaultdict(collections.Counter);games_out=[];strata=[];triggers=[];expected_jobs=[];environments={};intervals=collections.defaultdict(list)
def norm(x):
 if isinstance(x,dict):return {k:(Path(v).name if k=='path' and isinstance(v,str) else norm(v)) for k,v in x.items() if k not in ['receipt','source_directory']}
 if isinstance(x,list):return [norm(v) for v in x]
 return x

def checked(item):
 job=item['job'];assert sha(item['success'])==item['success_sha256'];r=core.verify_result(store,job);assert r['job']==job
 assert r['status']=='complete' and r['result']['complete'];kind=job['kind'];stages[kind]['jobs']+=1;stages[kind]['wall_seconds']+=r['wall_seconds'];stages[kind]['cpu_seconds']+=r['child_cpu_seconds']
 assert job['source_revision']=='51351c48ea1adf9ac96011aec1440e44ceb741a4'
 assert job['study_freeze_sha256']==sha(STUDY/'main-freeze.json')
 for path,h in job['source_files'].items():
  if not path.startswith('tools/experiments/'):assert sha(REPO/path)==h
 env=r['environment'];assert env['architecture']=='x86_64';assert env['gcp']['machine-type'].endswith('/c3-standard-4');assert env['batch']['BATCH_TASK_RETRY_ATTEMPT']=='0'
 environments[env['gcp']['id']]=env['gcp'];intervals[env['gcp']['id']].append((r['started_epoch'],r['ended_epoch']))
 for k,a in item['artifacts'].items():assert a['sha256']==r['artifacts'][k]['sha256'] and Path(a['local']).read_bytes()==store.get(a['key'])
 return r

def queries(item,decisions,forced,nodes,stage):
 arts=item['artifacts'];m=read(arts['analysis/manifest.json']['local']);assert m['nodes']==nodes and m['options']==analysis.OPTIONS
 assert m['engine_sha256']==item['job']['build']['engines']['stockfish19']['sha256'];assert m['planned_decisions']==len(decisions)
 qs=[read(a['local']) for k,a in arts.items() if k.startswith('analysis/g') and '-attempt' in k]
 assert len(qs)==len(decisions)*(1 if forced else 2)
 expected={(d['game'],d['ply']):d for d in decisions};seen=set();ev={}
 for q in qs:
  d=q['decision'];assert d==expected[(d['game'],d['ply'])];assert q['nodes_requested']==nodes and q['options']==analysis.OPTIONS and q['engine_sha256']==m['engine_sha256']
  assert q['status']=='complete' and q['exit_code']==0 and q['process_group_settled'] and not q['raw_truncated'];assert 0<q['wall_seconds']<65
  f=q['forced_move'];assert f==forced if forced else f in [None,d['played']]
  key=(d['game'],d['ply'],f);assert key not in seen;seen.add(key)
  board=analysis.board_for(d);score=q['assessment']['score']
  if score:
   assert q['pv_independent_legal']
   for u in score['pv']:board.push_uci(u)
  else:assert 'no_nonbound_score_aligned_with_bestmove' in q['assessment']['uncertainty']
  b=analysis.board_for(d);assert chess.Move.from_uci(q['assessment']['bestmove']) in b.legal_moves
  if f:assert q['assessment']['bestmove']==f
  commands=[r['line'] for r in q['raw_uci'] if r['direction']=='send'];assert 'ucinewgame' in commands and 'setoption name Clear Hash' in commands
  assert 'position fen '+d['initial_fen']+(' moves '+' '.join(d['history']) if d['history'] else '') in commands
  assert [s for s in commands if s.startswith('go ')]==[f'go nodes {nodes}'+(' searchmoves '+f if f else '')]
  stages[stage]['queries']+=1;stages[stage]['query_wall_seconds']+=q['wall_seconds'];stages[stage]['query_cpu_seconds']+=q['child_cpu_seconds'];flags[stage].update(q['assessment']['uncertainty'])
  v=screen_v1.describe(q);flags[stage]['directional_bound_conflict']+=bool(v['directional_bound_conflicts']);flags[stage]['stable_report_heuristic']+=v['stable_report_heuristic'];ev[key]=v
 resources=read(arts['analysis/resources.json']['local']);assert resources['failures']==0 and resources['attempts']==len(qs)
 for name,qname in [('wall_seconds','wall_seconds'),('child_cpu_seconds','child_cpu_seconds')]:assert abs(resources[name]-sum(q[qname] for q in qs))<1e-5
 return ev

for pair,p in index.items():
 game=p['game_pair'];gr=checked(game);spec=game['job']['spec'];assert spec['id']==pair and p['opening']==spec['opening']
 raw=gzip.decompress(Path(game['artifacts']['pair/report.json.gz']['local']).read_bytes());outer=json.loads(raw);r=json.loads(outer['report_json']);receipt=read(game['artifacts']['pair/receipt.json']['local'])
 assert hashlib.sha256(raw).hexdigest()==receipt['report_sha256'];assert hashlib.sha256(outer['report_json'].encode()).hexdigest()==outer['report_sha256']
 assert receipt['complete'] and receipt['healthy'] and receipt['process_group_settled'] and receipt['returncode']==receipt['replay']['returncode']==0
 cfg=r['config'];assert cfg==read(game['artifacts']['pair/config.json']['local']);assert cfg['openings']==[p['opening']] and cfg['paired'] and cfg['claim_policy']=='auto_claim';assert cfg['work']=={'depth':None,'nodes':None,'movetime_ms':5000}
 for n in ['candidate','oxidation060']:
  i=0 if n=='candidate' else 1;assert r['engine_identities'][i]['executable']['sha256']==game['job']['build']['engines'][n]['sha256']
 ds,gs=analysis.extract_decisions(r);canonical={(d['game'],d['ply']):d for d in ds};assert len(gs)==len(r['games'])==2
 for i,(g,gm) in enumerate(zip(r['games'],gs)):
  assert g['stop']=='chess_conclusion' and not g['failure'] and not g['diagnostic_failures'] and not g['transcript']['truncated'];assert g['engines_by_color']==([0,1] if i==0 else [1,0])
  for e in g['engines']:assert e['exit']['success'] and not e['cleanup_failures'] and not e['stderr']['truncated']
  b=chess.Board(gm['initial_fen'])
  for u in gm['complete_history']:b.push_uci(u)
  moves=[e['text'].split()[1] for e in g['transcript']['entries'] if e['direction']=='response' and e['text'].startswith('bestmove ')];assert gm['complete_history']==p['opening']['moves']+moves
  assert all(e['text']=='go movetime 5000' for e in g['transcript']['entries'] if e['direction']=='command' and e['text'].startswith('go '))
  games_out.append({'pair':pair,'game':i,'conclusion':g['conclusion'],'history_plies':len(gm['complete_history'])})
 summary=r['summaries'][0];assert summary==receipt['summary'] and summary['unscored']==summary['failures']==0
 for k in ['all',spec['phase']]:outcomes[k].update({'wins':summary['wins'][0],'draws':summary['draws'],'losses':summary['wins'][1]})
 screen=p['reference_screening'];sr=checked(screen);assert screen['job'] in manifest['jobs'];assert game['job'] in manifest['jobs']
 for item in [screen]+p['confirmations']:
  a=item['artifacts'];rr=read(a['replay.json']['local']);assert rr['returncode']==0 and rr['input_sha256']==game['artifacts']['pair/report.json.gz']['sha256'];assert rr['uncompressed_sha256']==hashlib.sha256(raw).hexdigest();assert a['replay-input.json']['sha256']==hashlib.sha256(raw).hexdigest()
 queries(screen,ds,None,500000,'reference_screening')
 saved=read(screen['artifacts']['screen-v1.json']['local']);regenerated=screen_v1.summarize(Path(screen['artifacts']['analysis/manifest.json']['local']).parent);assert norm(saved)==norm(regenerated)
 phases={g['game']:phase_tags.tag_history(g['initial_fen'],g['complete_history'],next(d['history_origin'] for d in ds if d['game']==g['game'])) for g in gs};assert {str(k):v for k,v in phases.items()}==read(screen['artifacts']['phases.json']['local'])
 selected={};cells=collections.defaultdict(list)
 for row in saved['decisions']:
  d=row['decision'];counts[d['origin']+'_decisions']+=1
  if d['origin']!='played':continue
  phase=phases[d['game']]['positions'][d['ply']]['phase']
  if row['followup_required']:selected[(d['game'],d['ply'])]=(row,'v1-followup',phase)
  else:cells[(d['game'],d['engine_index'],phase)].append(row)
 for cell,rows in cells.items():
  ranked=sorted(rows,key=lambda r:hashlib.sha256(('msh161-main-audit-v1/'+pair+'/'+str(r['decision']['game'])+'/'+str(r['decision']['ply'])).encode()).hexdigest());n=max(1,math.ceil(.1*len(rows)));strata.append({'pair':pair,'cell':cell,'population':len(rows),'selected':n})
  for row in ranked[:n]:selected[(row['decision']['game'],row['decision']['ply'])]=(row,'stratified-nonflagged-audit',cell[2])
 expected=[]
 for _,(row,reason,phase) in sorted(selected.items()):
  d=row['decision'];expected.append(dict(pair=pair,game=d['game'],ply=d['ply'],played=d['played'],alternative=row['queries'].get('unrestricted',{}).get('bestmove'),phase=phase,reason=reason,initial_flags=row['flags'],initial_raw_loss=row['raw_expected_score_loss']))
 assert expected==read(screen['artifacts']['confirmation-plan.json']['local']);derived=cli.derived_confirmations(screen['job'],sr,store);assert sorted(j['id'] for j in derived)==sorted(v['job']['id'] for v in p['confirmations']);expected_jobs.extend(derived)
 evidence={}
 for item in p['confirmations']:
  checked(item);j=item['job'];assert j in derived;d=canonical[(j['game'],j['ply'])];evidence.update(queries(item,[d],j['move'],5000000,'confirmation'))
 for q in expected:
  counts[q['reason']]+=1;counts['confirmation_decisions']+=1;counts['same_choice_controls']+=q['played']==q['alternative'];counts['missing_alternatives']+=not q['alternative']
  if q['reason']=='stratified-nonflagged-audit' and q['played']!=q['alternative']:
   vs=[evidence[(q['game'],q['ply'],m)] for m in [q['played'],q['alternative']]];stable=all(v['stable_report_heuristic'] and not v['directional_bound_conflicts'] and not(set(v['flags'])-{'last_score_update_not_selected_exact_score'}) for v in vs);counts['stable_two_choice_audits']+=stable;counts['uncertain_two_choice_audits']+=not stable
   if stable and vs[1]['expected_score_mover']-vs[0]['expected_score_mover']>=.05:triggers.append({'decision':q,'loss':vs[1]['expected_score_mover']-vs[0]['expected_score_mover'],'requires_importance_adjudication':True})
for v in intervals.values():
 v.sort();assert all(a[1]<=b[0]+.001 for a,b in zip(v,v[1:]))
alljobs=manifest['jobs']+expected_jobs;assert len(alljobs)==2410 and len(expected_jobs)==2390
assert len(list((HERE.parent/'local-cache'/manifest['experiment']).glob('jobs/*/success.json')))==len(alljobs)
receipts=list((HERE.parent/'local-cache'/manifest['experiment']).glob('jobs/*/attempts/*/receipt.json'));assert len(receipts)==len(alljobs) and all(read(p)['status']=='complete' for p in receipts)
assert not list((HERE.parent/'local-cache'/manifest['experiment']).glob('jobs/*/claim.json'))
result={'status':'passed','scope':'Independent offline full artifact/hash/native history/query/PV/selection audit; existing native replay receipts reused, no engines or native replay launched','script_sha256':sha(__file__),'review_index_sha256':sha(HERE/'review-index.json'),'manifest_sha256':sha(HERE.parent/'batch-01-manifest.json'),'counts':dict(counts),'stages':dict(stages),'flags':dict(flags),'outcomes_candidate':dict(outcomes),'games':games_out,'audit_strata':strata,'stable_audit_miss_triggers':triggers,'worker_environments':environments,'failed_job_attempts':0,'unresolved_claims':0,'verified_jobs':len(alljobs),'resource_note':'Parent job CPU/wall includes native replay and helper query setup/cleanup. Query subtotals overlap job totals; never add both. VM lifecycle additionally includes idle/provisioning.','limitations':['Finite analysis remains unadjudicated; cloud20M required for consequential decisions','Recorded cleanup is not a new live process census','CPU/platform conditions and identities verified; no Mac/cloud pooling']}
(HERE/'integrity-audit.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:result[k] for k in ['status','counts','stages','flags','outcomes_candidate','stable_audit_miss_triggers']},indent=2))
