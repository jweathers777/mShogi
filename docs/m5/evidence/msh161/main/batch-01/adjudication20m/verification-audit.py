#!/usr/bin/env python3
"""Independent offline20M adjudication receipt audit; never launches engines."""
import collections,datetime,hashlib,json,platform,statistics,sys
from pathlib import Path
OUT=Path(__file__).resolve().parent;BATCH=OUT.parent;HERE=BATCH.parents[1]
sys.path.insert(0,str(HERE))
import chess,analysis,screen_v1
read=lambda p:json.loads(Path(p).read_text())
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def timestamp(s):return datetime.datetime.fromisoformat(s).timestamp()
def close(a,b):assert abs(a-b)<1e-5,(a,b)
chain=read(OUT/'chain.json');plan=read(BATCH/'adjudication-plan.json');freeze=read(HERE/'main-freeze.json');build=read(HERE/'build.json')
assert chain['status']=='completed_needs_independent_review' and not chain.get('inflight')
assert len(plan)==len(chain['results'])==73 and chain['nodes']==20000000
assert chain['plan_sha256']==sha(BATCH/'adjudication-plan.json') and chain['freeze_sha256']==sha(HERE/'main-freeze.json')
assert chain['script_sha256']==sha(HERE/'review_checks.py')
for name,h in freeze['files'].items():assert sha(HERE/name)==h
for schedule in freeze['schedules']:assert sha(schedule['path'])==schedule['sha256']
assert not list(OUT.rglob('inflight.json')) and not list(OUT.rglob('*.tmp'))
reports={};seen_decisions=set();referenced=set();directories=set();bydirectory=collections.defaultdict(list);intervals=[];flags=collections.Counter();statistics_counts=collections.Counter();walls=[];cpus=[]
for path,h in chain['report_hashes'].items():
 assert sha(path)==h
 ds,_=analysis.extract_decisions(analysis.load_report(path));reports[str(Path(path).resolve())]={(d['game'],d['ply']):d for d in ds}
assert len(reports)==10
for index,(row,result) in enumerate(zip(plan,chain['results'])):
 assert result['decision']==row and result['requires_independent_adjudication']
 key=(str(Path(row['report']).resolve()),row['game'],row['ply']);assert key not in seen_decisions;seen_decisions.add(key)
 decision=reports[key[0]][key[1:]];board=analysis.board_for(decision)
 for move in row.get('moves',[]):assert chess.Move.from_uci(move) in board.legal_moves
 if not row.get('discovery'):assert decision['played'] in row['moves']
 actual_forced=[];kind_map={}
 for q in result['queries']:
  path=Path(q['receipt']);assert path.is_relative_to(OUT) and path not in referenced and sha(path)==q['sha256'];referenced.add(path)
  value=read(path);directory=path.parent;directories.add(directory);bydirectory[directory].append(value);manifest=read(directory/'manifest.json')
  assert manifest['report_sha256']==sha(row['report']) and manifest['script_sha256']==freeze['files']['analysis.py']
  assert manifest['build_sha256']==sha(HERE/'build.json') and manifest['protocol_sha256']==freeze['files']['../../competitive-gameplay-diagnosis.md']
  assert manifest['engine_sha256']==value['engine_sha256']==build['engines']['stockfish19']['sha256']
  assert manifest['nodes']==value['nodes_requested']==20000000 and value['options']==manifest['options']==analysis.OPTIONS
  assert manifest['planned_decisions']==1 and manifest['game']==row['game'] and manifest['ply']==row['ply']
  assert value['decision']==decision and value['status']=='complete' and value['exit_code']==0 and value['process_group_settled']
  assert not value['raw_truncated'] and not value.get('residual_process_group') and not value.get('error')
  assert q['finite_evidence']==screen_v1.describe(value)
  forced=value['forced_move'];kind=value['query'].rsplit('-',1)[1];assert q['kind']==kind
  if kind=='alternative':assert manifest['move']==forced;actual_forced.append(forced)
  elif kind=='played':assert forced==decision['played'] and manifest['move'] is None;kind_map[kind]=value
  else:assert kind=='unrestricted' and forced is None and manifest['move'] is None;kind_map[kind]=value
  if forced:assert value['assessment']['bestmove']==forced
  assert chess.Move.from_uci(value['assessment']['bestmove']) in board.legal_moves
  selected=value['assessment'].get('score')
  if selected:
   assert not selected['lowerbound'] and not selected['upperbound'] and selected['pv'][0]==value['assessment']['bestmove'] and value.get('pv_independent_legal')
   pvboard=board.copy(stack=True)
   for move in selected['pv']:pvboard.push_uci(move)
  else:assert 'no_nonbound_score_aligned_with_bestmove' in value['assessment']['uncertainty']
  commands=[e['line'] for e in value['raw_uci'] if e['direction']=='send'];position='position fen '+decision['initial_fen']+(' moves '+' '.join(decision['history']) if decision['history'] else '')
  assert position in commands and 'ucinewgame' in commands and 'setoption name Clear Hash' in commands
  assert [c for c in commands if c.startswith('go ')]==['go nodes 20000000'+(' searchmoves '+forced if forced else '')]
  assert 0<value['wall_seconds']<65 and value['child_cpu_seconds']>=0
  intervals.append((timestamp(value['started_utc']),timestamp(value['finished_utc']),str(path.relative_to(OUT))))
  walls.append(value['wall_seconds']);cpus.append(value['child_cpu_seconds']);flags.update(value['assessment']['uncertainty'])
  ev=q['finite_evidence'];statistics_counts['stable_report_heuristic_queries']+=ev['stable_report_heuristic'];statistics_counts['directional_bound_conflict_queries']+=bool(ev['directional_bound_conflicts']);statistics_counts['missing_numeric_estimate_queries']+=ev['expected_score_mover'] is None
 if row.get('discovery'):
  statistics_counts['discovery_decisions']+=1;assert set(kind_map)=={'played','unrestricted'}
  found=kind_map['unrestricted']['assessment']['bestmove'];played=decision['played'];expected=list(dict.fromkeys(row.get('moves',[])+([found] if found!=played else [])));expected=[m for m in expected if m!=played]
  assert actual_forced==expected
 else:
  assert not kind_map and actual_forced==list(dict.fromkeys(row['moves']))
for directory,values in bydirectory.items():
 rr=read(directory/'resources.json');assert rr['attempts']==len(values) and rr['failures']==0
 close(rr['wall_seconds'],sum(v['wall_seconds'] for v in values));close(rr['child_cpu_seconds'],sum(v['child_cpu_seconds'] for v in values))
assert referenced==set(OUT.glob('q*/g*-attempt*.json'))
assert len(chain['completed'])==len(directories)==142
assert {Path(c['command'][3]) for c in chain['completed']}==directories
assert all(c['returncode']==0 for c in chain['completed'])
intervals.sort();overlaps=[(a[2],b[2]) for a,b in zip(intervals,intervals[1:]) if a[1]>b[0]+.001];assert not overlaps
notification=HERE/'notifications/main-batch-01-adjudication.json';n=read(notification)
assert n['state_artifact']==str(OUT/'chain.json') and n['status']=='success' and n['computation_returncode']==0 and n['process_group_settled'] and n['queue_returncode']==0 and n['queue_accepted']
prior=read(BATCH/'audit.json');assert chain['started_epoch']>=read(BATCH/'chain.json')['last_epoch']
draw_context=[]
for g in prior['native_game_records']:
 draw=g['conclusion'].get('chess',{}).get('draw')
 if draw is None:continue
 item=dict(g)
 if isinstance(draw,dict) and 'claimed' in draw:
  raw=analysis.load_report(BATCH/'games'/g['pair']/'report.json.gz');record=analysis.decode(raw['games'][g['game']]['match_record_json'])
  claims=[e for e in record['events'] if isinstance(e['request']['command'],dict) and 'claim' in e['request']['command'] and e['reply'].get('rejection') is None]
  assert raw['config']['claim_policy']=='auto_claim' and claims
  item['accepted_final_claim_request']=claims[-1]['request'];item['claim_policy']='auto_claim'
  item['interpretation']='Controller exercises native automatic claim; Stockfish self-play WDL does not certify conversion under this mandatory policy. Intended move is not played.'
 draw_context.append(item)
result={'status':'passed','scope':'Independent offline73-decision20M integrity, matching, legalPV and accounting audit; no engine/native invocation',
 'auditor_script_sha256':sha(__file__),'chain_sha256':sha(OUT/'chain.json'),'plan_sha256':chain['plan_sha256'],'freeze_sha256':chain['freeze_sha256'],
 'python':platform.python_version(),'python_chess':chess.__version__,'platform':platform.platform(),
 'decisions':len(plan),'reports':len(reports),'query_process_steps':len(directories),'query_attempts':len(referenced),'failures':0,'unreferenced_attempts':0,'inflight_or_partial_files':0,
 'data_quality':dict(statistics_counts),'assessment_uncertainty_flags':dict(flags),'overlapping_query_intervals':0,
 'resources':{'query_wall_seconds':sum(walls),'query_child_cpu_seconds':sum(cpus),'stage_wall_seconds':sum(c['wall_seconds'] for c in chain['completed']),'chain_elapsed_seconds':chain['ended_epoch']-chain['started_epoch'],
              'query_wall_min':min(walls),'query_wall_median':statistics.median(walls),'query_wall_max':max(walls),'artifact_bytes_before_audit':sum(p.stat().st_size for p in OUT.rglob('*') if p.is_file() and p.name not in ['audit.json','verification-review.md'])},
 'accounting_rule':'These are additional20M attempts, absent from initial batch audit; query/stage/chain wall intervals overlap and must not be summed together. Native games remain20; no games added here.',
 'initial_batch_audit_sha256':sha(BATCH/'audit.json'),'combined_unique_reference_attempts':prior['all_query_attempts']+len(referenced),
 'combined_native_and_reference_process_wall_seconds':prior['totals']['native']['wall_seconds']+prior['totals']['initial_reference']['wall_seconds']+prior['totals']['confirmation_reference']['wall_seconds']+sum(walls),
 'combined_native_and_reference_child_cpu_seconds':prior['totals']['native']['cpu_seconds']+prior['totals']['initial_reference']['cpu_seconds']+prior['totals']['confirmation_reference']['cpu_seconds']+sum(cpus),
 'native_draw_context':draw_context,'notification_receipt_sha256':sha(notification),'notification_queue_accepted':True,
 'integrity_blocker_batch02':False,'remaining_gate':'Independent finite-event/episode adjudication, dispositions and any audit expansion decision; no automatic chess conclusions from execution pass.'}
analysis.save(OUT/'audit.json',result)
print(json.dumps(result,indent=2))
