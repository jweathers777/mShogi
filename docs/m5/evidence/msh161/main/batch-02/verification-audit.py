#!/usr/bin/env python3
"""Independent read-only batch02 integrity/coverage audit; no engines/network.
Writes only audit.json after all assertions pass. Run with existing python-chess.
"""
import collections,datetime,gzip,hashlib,json,math,platform,statistics,sys
from pathlib import Path
BATCH=Path(__file__).resolve().parent;HERE=BATCH.parents[1]
sys.path.insert(0,str(HERE))
import chess,analysis,phase_tags,screen_v1
read=lambda p:json.loads(Path(p).read_text())
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def stamp(s):return datetime.datetime.fromisoformat(s).timestamp()
def close(a,b):assert abs(a-b)<1e-5,(a,b)
def measures(values):
 s=sorted(values)
 return {'n':len(s),'min':s[0] if s else None,'median':statistics.median(s) if s else None,'p95':s[math.ceil(.95*len(s))-1] if s else None,'max':s[-1] if s else None,'sum':sum(s)}
freeze=read(HERE/'main-freeze.json');build=read(HERE/'build.json');chain=read(BATCH/'chain.json');native_batch=read(BATCH/'games/batch.json')
assert sha(HERE/'main-freeze.json')==chain['freeze_sha256']
for name,h in freeze['files'].items():assert sha(HERE/name)==h,name
for s in freeze['schedules']:assert sha(s['path'])==s['sha256']
for e in build['engines'].values():assert sha(e['path'])==e['sha256']
schedule=read(next(s['path'] for s in freeze['schedules'] if s['index']==2))
assert len(schedule['pairs'])==len(native_batch['pairs'])==10
assert chain['status']=='completed_needs_independent_review' and not chain.get('inflight')
assert native_batch['status']=='completed_needs_review' and not native_batch['inflight']
assert all(c['returncode']==0 for c in chain['completed'])
assert not list(BATCH.rglob('inflight.json')) and not list(BATCH.rglob('*.tmp'))
intervals=[];totals=collections.defaultdict(collections.Counter);times=collections.defaultdict(list);conditions=collections.defaultdict(collections.Counter)
results=collections.defaultdict(lambda:collections.Counter({'wins':0,'draws':0,'losses':0}));responses=collections.defaultdict(list)
canonical={};expected_plan=[];strata=[];native_records=[];query_count=0;manifest_count=0
expected_options=analysis.OPTIONS

def query_check(path,manifest,expected,forced,nodes,stage):
 global query_count
 x=read(path);assert x['decision']==expected,(path,'history/context')
 assert x['engine_sha256']==build['engines']['stockfish19']['sha256']==manifest['engine_sha256']
 assert x['nodes_requested']==manifest['nodes']==nodes and x['options']==manifest['options']==expected_options
 assert x['forced_move']==forced and x['status']=='complete' and x['exit_code']==0 and x['process_group_settled']
 assert not x['raw_truncated']
 if x.get('assessment',{}).get('score') is not None:assert x.get('pv_independent_legal')
 else:
  assert 'no_nonbound_score_aligned_with_bestmove' in x['assessment']['uncertainty']
  assert chess.Move.from_uci(x['assessment']['bestmove']) in analysis.board_for(expected).legal_moves
  conditions[stage]['no_selected_pv_report']+=1
 if forced:assert x['assessment']['bestmove']==forced
 assert not x.get('residual_process_group') and not x.get('error')
 commands=[r['line'] for r in x['raw_uci'] if r['direction']=='send']
 position='position fen '+expected['initial_fen']+(' moves '+' '.join(expected['history']) if expected['history'] else '')
 assert position in commands and 'ucinewgame' in commands and 'setoption name Clear Hash' in commands
 assert [s for s in commands if s.startswith('go ')]==[f'go nodes {nodes}'+(' searchmoves '+forced if forced else '')]
 assert 0<x['wall_seconds']<65 and x['child_cpu_seconds']>=0
 totals[stage]['attempts']+=1;totals[stage]['wall_seconds']+=x['wall_seconds'];totals[stage]['cpu_seconds']+=x['child_cpu_seconds']
 times[stage].append(x['wall_seconds']);conditions[stage].update((x.get('assessment') or {}).get('uncertainty',[]))
 v=screen_v1.describe(x)
 conditions[stage]['stable_report_heuristic']+=v['stable_report_heuristic'];conditions[stage]['directional_bound_conflict_queries']+=bool(v['directional_bound_conflicts'])
 conditions[stage]['missing_numeric_estimate']+=v['expected_score_mover'] is None
 intervals.append((stamp(x['started_utc']),stamp(x['finished_utc']),str(path.relative_to(BATCH))))
 query_count+=1;return x

def manifest_check(directory,report,nodes):
 global manifest_count
 m=read(directory/'manifest.json');assert m['report_sha256']==sha(report)
 assert m['engine_sha256']==build['engines']['stockfish19']['sha256'] and m['script_sha256']==freeze['files']['analysis.py']
 assert m['build_sha256']==sha(HERE/'build.json') and m['protocol_sha256']==freeze['files']['../../competitive-gameplay-diagnosis.md']
 assert m['nodes']==nodes and m['options']==expected_options
 manifest_count+=1;return m

for spec,receipt in zip(schedule['pairs'],native_batch['pairs']):
 assert spec['id']==receipt['id'] and receipt['opponent']=='oxidation060' and receipt['phase']==spec['phase']
 assert receipt['complete'] and receipt['healthy'] and receipt['returncode']==0 and receipt['process_group_settled']
 assert receipt['replay']['returncode']==0 and not receipt.get('residual_process_group')
 d=BATCH/'games'/spec['id'];rp=d/'report.json.gz';outer=json.loads(gzip.decompress(rp.read_bytes()))
 assert hashlib.sha256(gzip.decompress(rp.read_bytes())).hexdigest()==receipt['report_sha256']
 assert hashlib.sha256(outer['report_json'].encode()).hexdigest()==outer['report_sha256'];r=json.loads(outer['report_json']);cfg=r['config']
 assert cfg==read(d/'config.json') and cfg['openings']==[spec['opening']]
 assert cfg['work']=={'depth':None,'nodes':None,'movetime_ms':5000} and cfg['paired'] and cfg['claim_policy']=='auto_claim'
 assert cfg['max_plies']==512 and cfg['watchdogs']['response_ms']==15000
 assert outer['environment']['harness_revision']==build['source'] and outer['environment']['harness_dirty_sha256']==hashlib.sha256(b'').hexdigest()
 for i,name in enumerate(['candidate','oxidation060']):assert r['engine_identities'][i]['executable']['sha256']==build['engines'][name]['sha256']
 ds,gs=analysis.extract_decisions(r);canonical[spec['id']]={(x['game'],x['ply']):x for x in ds}
 assert len(gs)==len(r['games'])==2
 pair_responses=collections.defaultdict(list)
 for gi,(g,gm) in enumerate(zip(r['games'],gs)):
  assert g['stop']=='chess_conclusion' and not g['failure'] and not g['diagnostic_failures'] and not g['transcript']['truncated']
  assert g['engines_by_color']==([0,1] if gi==0 else [1,0])
  for engine in g['engines']:assert engine['exit']['success'] and not engine['cleanup_failures'] and not engine['identity_truncated'] and not engine['stderr']['truncated']
  board=chess.Board(gm['initial_fen'])
  for u in gm['complete_history']:board.push_uci(u)
  transcript_moves=[e['text'].split()[1] for e in g['transcript']['entries'] if e['direction']=='response' and e['text'].startswith('bestmove ')]
  assert gm['complete_history']==spec['opening']['moves']+transcript_moves
  active={}
  for event in g['transcript']['entries']:
   if event['direction']=='command' and event['text'].startswith('go '):
    assert event['text']=='go movetime 5000' and event['engine'] not in active;active[event['engine']]=event['at_ns']
   if event['direction']=='response' and event['text'].startswith('bestmove '):
    elapsed=(event['at_ns']-active.pop(event['engine']))/1e9;responses[event['engine']].append(elapsed);pair_responses[event['engine']].append(elapsed)
  assert not active
  native_records.append({'pair':spec['id'],'game':gi,'stratum':spec['phase'],'candidate_color':'white' if gi==0 else 'black','conclusion':g['conclusion'],'full_history_plies':len(gm['complete_history']),'prepared_plies':len(spec['opening']['moves'])})
 for i in [0,1]:assert pair_responses[i]==receipt['response_seconds_by_engine'][i]
 s=r['summaries'][0];assert s==receipt['summary'] and s['unscored']==s['failures']==0 and sum(s['wins'])+s['draws']==2
 for target in ['all',spec['phase']]:results[target].update({'wins':s['wins'][0],'draws':s['draws'],'losses':s['wins'][1]})
 intervals.append((stamp(receipt['started_utc']),stamp(receipt['ended_utc']),spec['id']+'/native-pair'))
 for k in ['wall_seconds','cpu_seconds','active_response_seconds']:totals['native'][k]+=receipt[k]
 analysis_dir=d/'analysis';m=manifest_check(analysis_dir,rp,500000);assert m['planned_decisions']==len(ds)
 attempts=list(analysis_dir.glob('g*-attempt*.json'));assert len(attempts)==2*len(ds)
 for x in ds:
  for kind,forced in [('unrestricted',None),('played',x['played'])]:
   path=analysis_dir/f'g{x["game"]:03d}-p{x["ply"]:03d}-{kind}-attempt0000.json';query_check(path,m,x,forced,500000,'initial_reference')
 resources=read(analysis_dir/'resources.json');assert resources['attempts']==len(attempts) and resources['failures']==0
 close(resources['wall_seconds'],sum(read(p)['wall_seconds'] for p in attempts));close(resources['child_cpu_seconds'],sum(read(p)['child_cpu_seconds'] for p in attempts))
 screen=screen_v1.summarize(analysis_dir);assert screen==read(d/'screen-v1.json') and screen['complete_query_coverage']
 phases=read(d/'phases.json')
 for gm in gs:
  origin=next(x['history_origin'] for x in ds if x['game']==gm['game'])
  assert phases[str(gm['game'])]==phase_tags.tag_history(gm['initial_fen'],gm['complete_history'],origin)
 cells=collections.defaultdict(list);selected={}
 for row in screen['decisions']:
  dec=row['decision'];totals['coverage'][dec['origin']+'_decisions']+=1
  if dec['origin']!='played':continue
  phase=phases[str(dec['game'])]['positions'][dec['ply']]['phase']
  totals['initial_screen']['uncertain_rows']+=bool(row['flags']);totals['initial_screen']['raw_threshold_rows']+=row['raw_expected_score_loss'] is not None and row['raw_expected_score_loss']>=.05
  totals['initial_screen']['negative_rows']+=row['raw_expected_score_loss'] is not None and row['raw_expected_score_loss']<0
  if row['followup_required']:selected[(dec['game'],dec['ply'])]=(row,'v1-followup',phase)
  else:cells[(dec['game'],dec['engine_index'],phase)].append(row)
 for cell,rows in cells.items():
  ranked=sorted(rows,key=lambda r:hashlib.sha256(('msh161-main-audit-v1/'+spec['id']+'/'+str(r['decision']['game'])+'/'+str(r['decision']['ply'])).encode()).hexdigest());n=max(1,math.ceil(.1*len(rows)))
  for row in ranked[:n]:selected[(row['decision']['game'],row['decision']['ply'])]=(row,'stratified-nonflagged-audit',cell[2])
  strata.append({'pair':spec['id'],'game':cell[0],'engine':cell[1],'phase':cell[2],'stable_population':len(rows),'audit_selected':n})
 for key,(row,reason,phase) in sorted(selected.items()):
  dec=row['decision'];expected_plan.append({'pair':spec['id'],'report':str(rp),'game':dec['game'],'ply':dec['ply'],'played':dec['played'],'alternative':row['queries'].get('unrestricted',{}).get('bestmove'),'phase':phase,'reason':reason,'initial_flags':row['flags'],'initial_raw_loss':row['raw_expected_score_loss'],'source_screen':str(d/'screen-v1.json')})

plan=read(BATCH/'confirmation-plan.json');confirmed=read(BATCH/'confirmation-results.json');assert plan==expected_plan and len(plan)==len(confirmed)==1451
audit_triggers=[]
for index,(q,result) in enumerate(zip(plan,confirmed)):
 assert result['decision']==q and not result['confirmed_event'] and result['requires_completed_batch_adjudication']
 moves=[m for m in dict.fromkeys([q['played'],q['alternative']]) if m];assert [r['move'] for r in result['queries']]==moves
 assert result['same_choice_control']==(q['played']==q['alternative'])
 totals['confirmation_selection'][q['reason']]+=1;totals['confirmation_selection']['same_choice_controls']+=result['same_choice_control'];totals['confirmation_selection']['missing_alternatives']+=not q['alternative']
 for query in result['queries']:
  path=Path(query['receipt']);assert path.is_relative_to(BATCH/'confirmations') and sha(path)==query['sha256']
  m=manifest_check(path.parent,Path(q['report']),5000000);assert m['planned_decisions']==1 and m['game']==q['game'] and m['ply']==q['ply'] and m['move']==query['move']
  assert len(list(path.parent.glob('g*-attempt*.json')))==1
  x=query_check(path,m,canonical[q['pair']][(q['game'],q['ply'])],query['move'],5000000,'confirmation_reference')
  assert query['finite_evidence']==screen_v1.describe(x)
  rr=read(path.parent/'resources.json');assert rr['attempts']==1 and rr['failures']==0;close(rr['wall_seconds'],x['wall_seconds']);close(rr['child_cpu_seconds'],x['child_cpu_seconds'])
 if q['reason']=='stratified-nonflagged-audit' and len(result['queries'])==2:
  ev={x['move']:x['finite_evidence'] for x in result['queries']};actual=ev[q['played']];alternative=ev[q['alternative']]
  stable=all(v['stable_report_heuristic'] and not v['directional_bound_conflicts'] and not (set(v['flags'])-{'last_score_update_not_selected_exact_score'}) for v in [actual,alternative])
  if stable:
   loss=alternative['expected_score_mover']-actual['expected_score_mover'];low=alternative['last_two_expected_score_range'][0]-actual['last_two_expected_score_range'][1]
   if loss>=.05:audit_triggers.append({'pair':q['pair'],'game':q['game'],'ply':q['ply'],'phase':q['phase'],'engine':canonical[q['pair']][(q['game'],q['ply'])]['engine_index'],'expected_score_loss':loss,'last_two_range_minimum_difference':low,'threshold_stable_across_observed_reports':low>=.05,'source_initial_raw_loss':q['initial_raw_loss'],'requires_importance_adjudication_before_stratum_expansion':True,'receipt_refs':[x['receipt'] for x in result['queries']]})
assert len(list((BATCH/'confirmations').glob('*/g*-attempt*.json')))==totals['confirmation_reference']['attempts']
assert len(chain['completed'])==1+2*10+totals['confirmation_reference']['attempts']
for k in ['wall_seconds','cpu_seconds','active_response_seconds']:close(totals['native'][k],native_batch[k])
close(sum(map(sum,responses.values())),native_batch['active_response_seconds'])
intervals.sort();overlaps=[{'left':a[2],'right':b[2],'seconds':a[1]-b[0]} for a,b in zip(intervals,intervals[1:]) if a[1]>b[0]+.001];assert not overlaps,overlaps[:3]
notification_path=HERE/'notifications/main-batch-02.json'
notification=read(notification_path)
assert notification['status']=='success' and notification['computation_returncode']==notification['queue_returncode']==0
assert notification['queue_attempted'] and notification['queue_accepted'] and notification['process_group_settled'] and notification['residual_group_detected'] is False
assert Path(notification['state_artifact'])==BATCH/'chain.json' and notification['command'][-1]=='2'
assert notification['started_epoch']<=chain['started_epoch']<=chain['last_epoch']<=notification['finished_epoch']
assert not (HERE/'main/batch-03').exists(), 'Unexpected batch03 artifacts require reconciliation'
artifact_bytes=sum(p.stat().st_size for p in BATCH.rglob('*') if p.is_file() and p.name not in ['audit.json','verification-review.md'])
output={'status':'passed','scope':'Independent offline batch02 integrity, complete coverage and resource audit; no engines launched',
 'notification_sha256':sha(notification_path),'notification_status':notification['status'],'notification_queue_accepted':True,'batch03_artifacts_present':False,'paused_scope':'Batch02 chain exited successfully; no live process census performed. Engine receipts and wrapper record settled groups.',
 'auditor_script_sha256':sha(__file__),'python':platform.python_version(),'python_chess':chess.__version__,'platform':platform.platform(),
 'freeze_sha256':sha(HERE/'main-freeze.json'),'chain_sha256':sha(BATCH/'chain.json'),'confirmation_plan_sha256':sha(BATCH/'confirmation-plan.json'),'confirmation_results_sha256':sha(BATCH/'confirmation-results.json'),
 'outcomes_candidate':dict(results),'native_games':len(native_records),'native_pairs':10,'native_replay':'all10existing frozen-runner receipts passed and exact raw report hashes matched; no new native invocation',
 'stage_commands':len(chain['completed']),'stage_failures':0,'all_query_attempts':query_count,'query_failures':0,'query_manifests_verified':manifest_count,'unreconciled_inflight_or_temporary_files':0,
 'totals':dict(totals),'reference_quality_flags':dict(conditions),'query_wall_seconds':{k:measures(v) for k,v in times.items()},
 'native_response_seconds_by_engine':{str(k):measures(v) for k,v in responses.items()},'serial_interval_overlap_count':0,
 'chain_elapsed_seconds':chain['last_epoch']-chain['started_epoch'],'sum_stage_wall_seconds':sum(c['wall_seconds'] for c in chain['completed']),
 'resource_accounting':'Native pair wall/CPU include replay/cleanup; reference wall/childCPU include process setup/cleanup; stage and chain elapsed overlap these and must not be added again.',
 'artifact_bytes_at_audit':artifact_bytes,'host_load_start':chain['host_load_start'],'host_load_end':chain['host_load_end'],
 'stable_audit_threshold_crossings':audit_triggers,'audit_strata':strata,'native_game_records':native_records,
 'recommendation':'Owner pause/cloud migration controls: preserve completed batch02; do not launch20M, batch03 or engines. No integrity/coverage execution failure found; uncertainties remain unadjudicated.',
 'remaining_obligations':['Consequential20M adjudication and legal continuation checks','Every played decision final disposition','Favorable episode boundaries and strong-play examples follow-up','Theme frequency/uncertainty and contrary examples across all planned batches','Main corpus remains200games; batch01 and batch02 total40 completed native games; owner pause controls further computation']}
analysis.save(BATCH/'audit.json',output)
print(json.dumps({k:output[k] for k in ['status','outcomes_candidate','native_games','all_query_attempts','totals','reference_quality_flags','chain_elapsed_seconds','artifact_bytes_at_audit']},indent=2))
