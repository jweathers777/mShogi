#!/usr/bin/env python3
"""One frozen20game batch, allmove screening and automatic matched confirmations.
Run through notify_exit.py; helpers own process limits/native game semantics.
"""
import argparse,hashlib,json,math,subprocess,time,os
from collections import defaultdict
from pathlib import Path
import analysis,phase_tags,screen_v1
HERE=Path(__file__).resolve().parent
PYTHON='/private/tmp/msh139/venv/bin/python'
def read(p):return json.loads(p.read_text())
def save(p,r):analysis.save(p,r)
def verify(freeze):
 for name,h in freeze['files'].items():assert analysis.digest(HERE/name)==h,('Frozen source changed',name)
 for schedule in freeze['schedules']:assert analysis.digest(Path(schedule['path']))==schedule['sha256'],'Frozen schedule changed'
def main():
 ap=argparse.ArgumentParser();ap.add_argument('index',type=int);a=ap.parse_args();freeze=read(HERE/'main-freeze.json');verify(freeze)
 spec=next(x for x in freeze['schedules'] if x['index']==a.index);schedule=Path(spec['path']);assert analysis.digest(schedule)==spec['sha256']
 out=HERE/'main'/f'batch-{a.index:02d}';out.mkdir(exist_ok=False)
 statepath=out/'chain.json';state={'status':'running','started_epoch':time.time(),'freeze_sha256':analysis.digest(HERE/'main-freeze.json'),'completed':[],'host_load_start':os.getloadavg(),'hardware_note':'Shared unpinned macOS host; fixed equal thinking time, not isolated CPU throughput'};save(statepath,state)
 def command(cmd,label):
  verify(freeze);i=len(state['completed']);state['inflight']={'label':label,'command':cmd,'started_epoch':time.time()};save(statepath,state);t=time.monotonic()
  with (out/f'step-{i:04d}.stdout.log').open('xb') as stdout,(out/f'step-{i:04d}.stderr.log').open('xb') as stderr:code=subprocess.call(cmd,stdout=stdout,stderr=stderr)
  state['completed'].append({'label':label,'command':cmd,'returncode':code,'wall_seconds':time.monotonic()-t});state.pop('inflight',None);save(statepath,state)
  if code:raise RuntimeError(f'{label} failed; preserve outputs and reconcile before retry')
 try:
  command([PYTHON,str(HERE/'run_batch.py'),str(schedule),str(out/'games')],'native20games')
  batch=read(out/'games/batch.json');assert batch['status']=='completed_needs_review' and all(x['complete'] for x in batch['pairs'])
  plan=[]
  for pair in read(schedule)['pairs']:
   d=out/'games'/pair['id'];report=d/'report.json.gz'
   command([PYTHON,str(HERE/'analysis.py'),str(report),str(d/'analysis')],pair['id']+'-allmove')
   command([PYTHON,str(HERE/'summarize_analysis.py'),str(d/'analysis'),str(d/'summary-v0.json')],pair['id']+'-summary')
   screen=screen_v1.summarize(d/'analysis');assert screen['complete_query_coverage'];save(d/'screen-v1.json',screen)
   native=analysis.load_report(report);decisions,games=analysis.extract_decisions(native)
   origins={d['game']:d.get('history_origin','unknown') for d in decisions}
   phases={g['game']:phase_tags.tag_history(g['initial_fen'],g['complete_history'],origins.get(g['game'],'unknown')) for g in games};save(d/'phases.json',phases)
   cells=defaultdict(list);selected={}
   for row in screen['decisions']:
    dec=row['decision']
    if dec['origin']!='played':continue
    key=(dec['game'],dec['ply']);phase=phases[dec['game']]['positions'][dec['ply']]['phase']
    if row['followup_required']:selected[key]=(row,'v1-followup',phase)
    else:cells[(dec['game'],dec['engine_index'],phase)].append(row)
   for cell,rows in cells.items():
    ranked=sorted(rows,key=lambda r:hashlib.sha256(('msh161-main-audit-v1/'+pair['id']+'/'+str(r['decision']['game'])+'/'+str(r['decision']['ply'])).encode()).hexdigest())
    for row in ranked[:max(1,math.ceil(len(rows)*.10))]:
     dec=row['decision'];selected[(dec['game'],dec['ply'])]=(row,'stratified-nonflagged-audit',cell[2])
   for key,(row,reason,phase) in sorted(selected.items()):
    dec=row['decision'];alternative=row['queries'].get('unrestricted',{}).get('bestmove')
    plan.append({'pair':pair['id'],'report':str(report),'game':dec['game'],'ply':dec['ply'],'played':dec['played'],'alternative':alternative,'phase':phase,'reason':reason,'initial_flags':row['flags'],'initial_raw_loss':row['raw_expected_score_loss'],'source_screen':str(d/'screen-v1.json')})
  save(out/'confirmation-plan.json',plan)
  confirmations=[]
  for index,q in enumerate(plan):
   queries=[]
   for move in dict.fromkeys([q['played'],q['alternative']]):
    if not move:continue
    destination=out/'confirmations'/f'q{index:04d}-{move}'
    command([PYTHON,str(HERE/'analysis.py'),q['report'],str(destination),'--game',str(q['game']),'--ply',str(q['ply']),'--move',move,'--nodes','5000000'],f'confirmation-{index:04d}-{move}')
    receipt=next(destination.glob('g*-attempt*.json'));value=read(receipt)
    queries.append({'move':move,'receipt':str(receipt),'sha256':analysis.digest(receipt),'finite_evidence':screen_v1.describe(value)})
   confirmations.append({'decision':q,'queries':queries,'same_choice_control':q['played']==q['alternative'],'confirmed_event':False,'requires_completed_batch_adjudication':True})
   save(out/'confirmation-results.json',confirmations)
  state.update(status='completed_needs_independent_review',games=20,confirmation_decisions=len(plan),note='No automatic thematic/error confirmation;20Mconsequential adjudication,episode review and additional strongplay checks remain')
 except BaseException as exc:state.update(status='failed_needs_review',error=repr(exc));raise
 finally:state['last_epoch']=time.time();state['host_load_end']=os.getloadavg();save(statepath,state)
if __name__=='__main__':main()
