#!/usr/bin/env python3
"""Fixed pilot confirmations, diverse qualification, then unattended allmove analysis.
Uses the existing reviewed analysis and native batch helpers unchanged.
"""
import hashlib,json,subprocess,time,os
from pathlib import Path
HERE=Path(__file__).resolve().parent
REPO=HERE.parents[3]
PYTHON='/private/tmp/msh139/venv/bin/python'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,v):
 t=p.with_suffix('.tmp');t.write_text(json.dumps(v,indent=2)+'\n');t.replace(p)
def main():
 statepath=HERE/'extension-chain.json';assert not statepath.exists(),'Inspect prior chain before resumption'
 commands=[]
 plan=json.loads((HERE/'pilot-confirmation-plan.json').read_text())
 for i,q in enumerate(plan):
  for move in dict.fromkeys([q['played'],q['alternativefromexistingPV']]):
   commands.append([PYTHON,str(HERE/'analysis.py'),str(REPO/q['report']),str(HERE/'pilot-confirmations'/f'q{i:02d}-{move}'),'--game',str(q['game']),'--ply',str(q['ply']),'--move',move,'--nodes','5000000'])
 schedule=HERE/'qualification-extension-schedule.json';out=HERE/'qualification-extension'
 commands.append([PYTHON,str(HERE/'run_batch.py'),str(schedule),str(out)])
 for pair in json.loads(schedule.read_text())['pairs']:
  d=out/pair['id'];commands.append([PYTHON,str(HERE/'analysis.py'),str(d/'report.json.gz'),str(d/'analysis')])
  commands.append([PYTHON,str(HERE/'summarize_analysis.py'),str(d/'analysis'),str(d/'analysis-summary.json')])
 frozen={str(p):digest(p) for p in [HERE/'analysis.py',HERE/'run_batch.py',HERE/'summarize_analysis.py',HERE/'pilot-confirmation-plan.json',schedule,HERE/'build.json',HERE/'../../competitive-gameplay-diagnosis.md']}
 state={'status':'running','started_epoch':time.time(),'pid':os.getpid(),'frozen':frozen,'commands':commands,'completed':[]};save(statepath,state)
 try:
  for i,command in enumerate(commands):
   assert all(digest(Path(p))==sha for p,sha in frozen.items()),'Frozen method/source changed'
   state['inflight']={'index':i,'command':command,'started_epoch':time.time()};save(statepath,state);start=time.monotonic()
   with (HERE/f'extension-step-{i:03d}.stdout.log').open('xb') as outlog,(HERE/f'extension-step-{i:03d}.stderr.log').open('xb') as err:
    code=subprocess.call(command,stdout=outlog,stderr=err)
   state['completed'].append({'index':i,'returncode':code,'wall_seconds':time.monotonic()-start});state.pop('inflight');save(statepath,state)
   if code:raise RuntimeError(f'Step{i} failed; retained evidence; no automatic retry')
  state['status']='completed_needs_review'
 except BaseException as e:state.update(status='failed_needs_review',error=repr(e));raise
 finally:state['last_epoch']=time.time();save(statepath,state)
if __name__=='__main__':main()
