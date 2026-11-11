#!/usr/bin/env python3
"""One unattended dependency chain: completed native batch -> reference results.
A shared OS lock waits for the batch; there is no model or per-move polling.
"""
import fcntl, gzip, hashlib, json, os, signal, subprocess, sys, time
from pathlib import Path
HERE=Path(__file__).resolve().parent
PYTHON='/private/tmp/msh139/venv/bin/python'
def save(p,v):
 tmp=p.with_suffix('.tmp');tmp.write_text(json.dumps(v,indent=2)+'\n');tmp.replace(p)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def validate_smoke(checks):
 assert len(checks)==2
 for r in checks:
  assert r['status']=='complete'
  assert set(r['assessment']['uncertainty']) <= {'last_score_update_not_selected_exact_score'}
  score=r['assessment']['score']
  assert score and score['wdl'] and not score['lowerbound'] and not score['upperbound']
  assert score['pv'][0]==r['assessment']['bestmove']
  assert r['pv_independent_legal'] and not r['raw_truncated']
  assert r['nodes_requested']==500000 and r['decision']['history']==[]
  assert 0<r['wall_seconds']<65 and r['process_group_settled']

def main():
 status=HERE/'postbatch-status.json'
 if status.exists():raise RuntimeError('Existing chain state: inspect before explicit restart')
 frozen={str(p):digest(p) for p in [HERE/'analysis.py',HERE/'summarize_analysis.py',HERE/'sources/balance-roots.json',HERE/'../../competitive-gameplay-diagnosis.md']}
 receipt={'status':'waiting_for_qualification','pid':os.getpid(),'started_epoch':time.time(),'steps':[],'frozen':frozen}
 save(status,receipt)
 try:
  def expired(*_):raise TimeoutError('Qualification dependency did not release within12600seconds')
  signal.signal(signal.SIGALRM,expired);signal.alarm(12600)
  with (HERE/'.execution.lock').open('a') as lock:
   fcntl.flock(lock,fcntl.LOCK_EX)
   batch=json.loads((HERE/'qualification/batch.json').read_text())
   if batch.get('status')!='completed_needs_review' or not all(p['complete'] for p in batch['pairs']):
    raise RuntimeError('Qualification incomplete/failed; preserved for review')
  signal.alarm(0)
  assert all(digest(Path(p))==h for p,h in frozen.items()),'Frozen analysis/source/protocol changed while waiting'
  receipt['status']='reference_smoke';save(status,receipt)
  first=HERE/'qualification'/batch['pairs'][0]['id']/'report.json.gz'
  smoke=HERE/'reference-smoke'
  cmd=[PYTHON,str(HERE/'analysis.py'),str(first),str(smoke),'--game','0','--ply','0']
  t=time.monotonic()
  with (HERE/'reference-smoke.stdout.log').open('xb') as out,(HERE/'reference-smoke.stderr.log').open('xb') as err:
   code=subprocess.call(cmd,stdout=out,stderr=err)
  receipt['steps'].append({'command':cmd,'returncode':code,'wall_seconds':time.monotonic()-t});save(status,receipt)
  assert code==0,'Reference smoke execution failed'
  checks=[json.loads(p.read_text()) for p in smoke.glob('g*-attempt*.json')]
  validate_smoke(checks)
  receipt['status']='reference_analysis';save(status,receipt)
  for pair in batch['pairs']:
   folder=HERE/'qualification'/pair['id'];report=folder/'report.json.gz'
   command=[PYTHON,str(HERE/'analysis.py'),str(report),str(folder/'analysis')]
   t=time.monotonic();step={'command':command,'started_epoch':time.time()};receipt['inflight']=step;save(status,receipt)
   with (folder/'analysis.stdout.log').open('xb') as out,(folder/'analysis.stderr.log').open('xb') as err:
    # Each query has a60s watchdog; number is bounded by canonical game length.
    code=subprocess.call(command,stdout=out,stderr=err)
   step.update(returncode=code,wall_seconds=time.monotonic()-t);receipt['steps'].append(step);receipt.pop('inflight',None);save(status,receipt)
   if code:raise RuntimeError('Reference analysis failed; inspect retained query receipt')
   subprocess.run([PYTHON,str(HERE/'summarize_analysis.py'),str(folder/'analysis'),str(folder/'analysis-summary.json')],check=True,timeout=60)
  roots=HERE/'sources/balance-roots.json'
  command=[PYTHON,str(HERE/'analysis.py'),str(roots),str(HERE/'balance-screen'),'--roots']
  t=time.monotonic();step={'command':command,'started_epoch':time.time()};receipt['inflight']=step;save(status,receipt)
  with (HERE/'balance.stdout.log').open('xb') as out,(HERE/'balance.stderr.log').open('xb') as err:
   code=subprocess.call(command,stdout=out,stderr=err)
  step.update(returncode=code,wall_seconds=time.monotonic()-t);receipt['steps'].append(step);receipt.pop('inflight',None)
  if code:raise RuntimeError('Source balance screen failed; preserve and review')
  receipt['status']='completed_needs_independent_review'
 except BaseException as exc:
  receipt.update(status='failed_needs_review',error=repr(exc));raise
 finally:
  receipt['last_epoch']=time.time();save(status,receipt)
if __name__=='__main__':main()
