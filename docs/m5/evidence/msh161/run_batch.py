#!/usr/bin/env python3
"""Study-specific unattended native pair batches; adapted from MSH-151 supervision.
No rules, move selection, chess evaluation or outcomes are implemented here.
"""
import argparse, fcntl, gzip, hashlib, json, os, resource, signal, subprocess, time
from datetime import datetime, timezone
from pathlib import Path
HERE=Path(__file__).resolve().parent
REPO=HERE.parents[3]
def read(p): return json.loads(Path(p).read_text())
def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def save(p,v):
 p=Path(p); tmp=p.with_suffix(p.suffix+'.tmp')
 with tmp.open('w') as f: json.dump(v,f,indent=2,allow_nan=False); f.write('\n'); f.flush(); os.fsync(f.fileno())
 tmp.replace(p)
def utc(): return datetime.now(timezone.utc).isoformat()
def limits():
 resource.setrlimit(resource.RLIMIT_CORE,(0,0))
 resource.setrlimit(resource.RLIMIT_FSIZE,(128*1024*1024,128*1024*1024))
def config(build, spec):
 opponent=spec['opponent']; engines=build['engines']
 return {'schema_version':1,'id':spec['id'],'engines':[
  {'path':engines[k]['path'],'args':['uci'] if k=='candidate' else [],'options':[], 'model_path':None,
   'model_identity':k+'/'+engines[k].get('source',build['source']),'deterministic':False} for k in ['candidate',opponent]],
  'openings':[spec['opening']],'paired':True,'claim_policy':'auto_claim','time_control':'untimed',
  'work':{'depth':None,'nodes':None,'movetime_ms':5000},'max_plies':512,
  'watchdogs':{'handshake_ms':5000,'ready_ms':5000,'response_ms':15000,'write_ms':2000,'cleanup_ms':1000},
  'io':{'line_bytes':4096,'queued_lines':32,'transcript_bytes':2097152,'stderr_bytes':16384,'command_bytes':4096}}
def audit(outer,cfg,build):
 assert outer['protocol']=='uci'
 assert hashlib.sha256(outer['report_json'].encode()).hexdigest()==outer['report_sha256']
 r=json.loads(outer['report_json']);assert r['config']==cfg and len(r['games'])==2
 assert outer['environment']['harness_revision']==build['source']
 assert outer['environment']['harness_dirty_sha256']==hashlib.sha256(b'').hexdigest()
 for i,e in enumerate(cfg['engines']):
  frozen=[v for v in build['engines'].values() if v['path']==e['path']]
  assert len(frozen)==1
  assert r['engine_identities'][i]['executable']['sha256']==frozen[0]['sha256']
 healthy=True;timings=[[],[]];searched=0
 for i,g in enumerate(r['games']):
  assert g['pairing']==i and g['engines_by_color']==([0,1] if i==0 else [1,0])
  healthy &= not g['failure'] and not g['diagnostic_failures'] and not g['transcript']['truncated']
  assert len(g['engines'])==2
  completed_before=searched
  for e in g['engines']:
   healthy &= not e['identity_truncated'] and not e['cleanup_failures'] and not e['stderr']['truncated'] and bool(e['exit'] and e['exit']['success'])
  active={}
  for e in g['transcript']['entries']:
   if e['direction']=='command' and e['text'].startswith('go'):
    assert e['text']=='go movetime 5000'; active[e['engine']]=e['at_ns']
   if e['direction']=='response' and e['text'].startswith('bestmove '):
    assert e['engine'] in active
    timings[e['engine']].append((e['at_ns']-active.pop(e['engine']))/1e9);searched+=1
  if active or searched==completed_before:healthy=False
 s=r['summaries'][0]
 complete=bool(healthy and all(g['stop']=='chess_conclusion' for g in r['games']) and s['unscored']==0 and sum(s['wins'])+s['draws']==2)
 return {'healthy':bool(healthy),'complete':complete,'summary':s,'stops':[g['stop'] for g in r['games']],
         'searched_plies':searched,'response_seconds_by_engine':timings,'active_response_seconds':sum(map(sum,timings))}
def group_alive(pid):
 try:os.killpg(pid,0);return True
 except ProcessLookupError:return False
def run_pair(build,spec,dest):
 dest.mkdir(); cfg=config(build,spec);save(dest/'config.json',cfg)
 receipt={'id':spec['id'],'phase':spec['phase'],'opponent':spec['opponent'],'started_utc':utc(),'complete':False}
 start=time.monotonic();cpu=resource.getrusage(resource.RUSAGE_CHILDREN);p=None
 save(dest/'receipt.json',receipt)
 try:
  with (dest/'stdout.log').open('wb') as out,(dest/'stderr.log').open('wb') as err:
   p=subprocess.Popen([build['engines']['runner']['path'],'experiment-external','uci',str(dest/'config.json'),str(dest/'report.json')],stdout=out,stderr=err,start_new_session=True,preexec_fn=limits)
   receipt['pid']=p.pid;save(dest/'receipt.json',receipt)
   try:receipt['returncode']=p.wait(timeout=6000)
   except subprocess.TimeoutExpired:receipt['outer_timeout']=True;os.killpg(p.pid,signal.SIGKILL);p.wait(timeout=10)
  if group_alive(p.pid):os.killpg(p.pid,signal.SIGKILL);receipt['residual_process_group']=True
  receipt['process_group_settled']=not group_alive(p.pid)
  if (dest/'report.json').exists():
   receipt.update(audit(read(dest/'report.json'),cfg,build))
   replay=subprocess.run([build['engines']['runner']['path'],'experiment-external-replay',str(dest/'report.json')],capture_output=True,text=True,timeout=60)
   receipt['replay']={'returncode':replay.returncode,'stdout':replay.stdout,'stderr':replay.stderr}
   receipt['complete'] &= replay.returncode==0 and receipt.get('returncode')==0 and receipt['process_group_settled'] and not receipt.get('residual_process_group')
   raw=(dest/'report.json').read_bytes();receipt['report_sha256']=hashlib.sha256(raw).hexdigest()
   (dest/'report.json.gz').write_bytes(gzip.compress(raw,mtime=0));(dest/'report.json').unlink()
 except BaseException as exc:
  receipt['error']=repr(exc);receipt['complete']=False
  if p:
   try:os.killpg(p.pid,signal.SIGKILL)
   except ProcessLookupError:pass
   p.wait(timeout=10)
 finally:
  endcpu=resource.getrusage(resource.RUSAGE_CHILDREN)
  receipt.update(ended_utc=utc(),wall_seconds=time.monotonic()-start,cpu_seconds=endcpu.ru_utime+endcpu.ru_stime-cpu.ru_utime-cpu.ru_stime)
  receipt['artifact_bytes']=sum(p.stat().st_size for p in dest.iterdir() if p.is_file())
  save(dest/'receipt.json',receipt)
 return receipt

def main():
 ap=argparse.ArgumentParser();ap.add_argument('schedule');ap.add_argument('output');a=ap.parse_args()
 schedule=Path(a.schedule).resolve();out=Path(a.output).resolve();out.mkdir(exist_ok=True)
 with (HERE/'.execution.lock').open('a') as lock:
  fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
  plan=read(schedule);build=read(HERE/'build.json')
  assert plan['stage'] in ['qualification','pilot','main'] and 1<=len(plan['pairs'])<=15
  assert len({s['id'] for s in plan['pairs']})==len(plan['pairs'])
  for e in build['engines'].values():assert sha(e['path'])==e['sha256']
  assert not subprocess.check_output(['git','diff',build['source'],'--','crates','Cargo.toml','Cargo.lock','rust-toolchain.toml'],cwd=REPO)
  statepath=out/'batch.json';identity={'schedule_sha256':sha(schedule),'build_sha256':sha(HERE/'build.json'),'helper_sha256':sha(__file__),'protocol_sha256':sha(HERE/'../../competitive-gameplay-diagnosis.md')}
  if statepath.exists():
   state=read(statepath);assert state['identity']==identity and not state.get('inflight'),'Changed freeze or unreconciled interrupted pair'
   assert all(r['complete'] for r in state['pairs']),'Investigate previous failure before continuation'
  else:state={'identity':identity,'stage':plan['stage'],'started_utc':utc(),'pairs':[],'inflight':None}
  save(statepath,state)
  for spec in plan['pairs'][len(state['pairs']):]:
   for k in ['candidate','runner',spec['opponent']]:
    e=build['engines'][k];assert sha(e['path'])==e['sha256']
   dest=out/spec['id'];assert not dest.exists(),'Orphan output requires reconciliation'
   state['inflight']=spec;save(statepath,state)
   result=run_pair(build,spec,dest);state['pairs'].append(result);state['inflight']=None
   state['wall_seconds']=sum(r['wall_seconds'] for r in state['pairs']);state['cpu_seconds']=sum(r['cpu_seconds'] for r in state['pairs'])
   state['active_response_seconds']=sum(r.get('active_response_seconds',0) for r in state['pairs']);save(statepath,state)
   print(json.dumps({'id':spec['id'],'complete':result['complete'],'summary':result.get('summary'),'wall_seconds':result['wall_seconds']}),flush=True)
   if not result['complete']:raise RuntimeError('Pair incomplete: batch stopped; evidence retained')
  state['status']='completed_needs_review';state['ended_utc']=utc();save(statepath,state)
if __name__=='__main__':main()
