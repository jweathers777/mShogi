#!/usr/bin/env python3
"""Bounded native cost comparison. prepare only writes inputs; next attempts one process.
The coordinator must reserve this separate <=60s ledger in the global round budget.
No opponent strength claims. Run only after source tests/build and timing isolation.
"""
import argparse,datetime,fcntl,hashlib,json,math,os,pathlib,platform,resource,signal,subprocess,time
P=pathlib.Path
def read(p):return json.loads(P(p).read_text())
def write(p,x):P(p).write_text(json.dumps(x,indent=2)+'\n')
def sha(p):return hashlib.sha256(P(p).read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def ledger(out):return [json.loads(x) for x in (out/'runs.jsonl').read_text().splitlines()] if (out/'runs.jsonl').exists() else []
def prepare(a):
 out=P(a.out).resolve();out.mkdir(parents=True,exist_ok=True);assert not any(out.iterdir())
 source=P(a.repo)/'docs/m5/evidence/msh154/architecture'
 fixed=read(source/'fixed-depth2.json');assert len(fixed)==33
 real=next(x for x in read(source/'real-obsolete-1s.json') if x['id']=='real002g0p112-full')
 live=read(source/'profile-panel4.json')[0]
 normal=read(source/'panel-1s.json')[0]
 for x,label in [(real,'obsolete-real'),(live,'live-repeat'),(normal,'normal-history')]:
  x['id']=label;x['time_ms']=1000;x['depth']=128;x['table_entries']=4096
 inputs={'fixed-depth2':fixed,'obsolete-real':[real],'live-repeat':[live],'normal-history':[normal]}
 for name,rows in inputs.items():write(out/(name+'.json'),rows)
 bins={k:str(P(getattr(a,k)).resolve()) for k in ['predecessor','candidate']}
 for b in bins.values():assert P(b).is_file() and os.access(b,os.X_OK)
 cells=[]
 for repeat in range(3):
  for version in (['predecessor','candidate'] if repeat%2==0 else ['candidate','predecessor']):cells.append({'kind':'fixed','input':'fixed-depth2','repeat':repeat,'version':version,'timeout':8})
 for kind in ['obsolete-real','live-repeat','normal-history']:
  for repeat in range(3):
   for version in (['predecessor','candidate'] if repeat%2==0 else ['candidate','predecessor']):cells.append({'kind':'timed','input':kind,'repeat':repeat,'version':version,'timeout':9})
 m={'schema':1,'host':platform.platform(),'prepared_utc':now(),'budget_seconds':60,'budget_scope':'separate native-cost reservation, coordinator subtracts from round allowance; never double-count as runner folders','source_revision':a.source_revision,'predecessor_source':a.predecessor_source,'binary_paths':bins,'binary_sha256':{k:sha(v) for k,v in bins.items()},'input_sha256':{k:sha(out/(k+'.json')) for k in inputs},'driver_sha256':sha(__file__),'cells':cells}
 write(out/'manifest.json',m);print(json.dumps({'prepared':len(cells),'timed_nominal_seconds':18,'searches_launched':0}))
def next_run(a):
 out=P(a.out).resolve()
 with open(out/'run.lock','a') as lock:
  fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
  m=read(out/'manifest.json');assert sha(__file__)==m['driver_sha256'];rows=ledger(out)
  assert [r['index'] for r in rows]==list(range(len(rows)))
  if len(rows)==len(m['cells']):print('ALL ATTEMPTS COMPLETE');return
  c=m['cells'][len(rows)];used=sum(r['charged_active_seconds'] for r in rows);assert used+c['timeout']+1<=m['budget_seconds']
  binary=m['binary_paths'][c['version']];assert sha(binary)==m['binary_sha256'][c['version']]
  inp=out/(c['input']+'.json');assert sha(inp)==m['input_sha256'][c['input']]
  expected=read(inp);base=out/f'cell-{len(rows):03d}';assert not list(out.glob(base.name+'.*')),'orphan preserved; reconcile before resume'
  command=[binary,str(inp),'2' if c['kind']=='fixed' else '128','untimed' if c['kind']=='fixed' else 'timed','none']
  write(str(base)+'.started.json',{'index':len(rows),'start_utc':now(),'command':command})
  start=time.monotonic();utc_start=now();before=resource.getrusage(resource.RUSAGE_CHILDREN);p=None;status=None;results=None;error=None
  def limits():
   resource.setrlimit(resource.RLIMIT_FSIZE,(8*1024*1024,8*1024*1024));resource.setrlimit(resource.RLIMIT_CPU,(c['timeout'],c['timeout']+1))
  try:
   with open(str(base)+'.stdout','x') as stdout,open(str(base)+'.stderr','x') as stderr:
    p=subprocess.Popen(command,stdout=stdout,stderr=stderr,start_new_session=True,preexec_fn=limits)
    try:status=p.wait(timeout=c['timeout'])
    except subprocess.TimeoutExpired:status='watchdog';os.killpg(p.pid,signal.SIGKILL);p.wait()
  except OSError as exc:status='launch-failed';error=str(exc)
  finally:
   if p:
    try:os.killpg(p.pid,signal.SIGKILL)
    except ProcessLookupError:pass
  wall=time.monotonic()-start;utc_end=now();after=resource.getrusage(resource.RUSAGE_CHILDREN)
  if status==0:
   try:
    results=[json.loads(x) for x in P(str(base)+'.stdout').read_text().splitlines()];assert len(results)==len(expected)
    for r,e in zip(results,expected):
     assert r['id']==e['id'] and r['fen']==e['expected_root_fen']
     assert r['root_unchanged'] and r['pv_native_admitted'] and r['node_limit'] is None
     assert r['mode']=='Conventional' and r['table_entries']==4096
     assert r['depth_limit']==(2 if c['kind']=='fixed' else 128)
     assert r['time_ms']==(None if c['kind']=='fixed' else 1000)
     if c['kind']=='fixed':assert r['status']=='complete'
     assert math.isfinite(r['elapsed_ms']) and r['elapsed_ms']>=0
    active=sum(r['elapsed_ms']/1000 for r in results);assert active<=wall+.1
   except (AssertionError,ValueError,KeyError,TypeError) as exc:status='invalid-result';results=None;error=repr(exc)
  charge=active if status==0 else wall
  row={**c,'index':len(rows),'command':command,'start_utc':utc_start,'end_utc':utc_end,'status':status,'error':error,'process_wall_seconds':wall,'cpu_seconds':after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,'charged_active_seconds':charge,'aggregate_charged_seconds':used+charge,'binary_sha256':sha(binary),'input_sha256':sha(inp),'result':results,'artifact_sha256':{p.name:sha(p) for p in out.glob(base.name+'.*')}}
  with open(out/'runs.jsonl','a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
  print(json.dumps({k:row[k] for k in ['index','version','input','status','charged_active_seconds','aggregate_charged_seconds']}))
  if status!=0:raise SystemExit('Preserved failure, stop and inspect; do not silently rerun')
def summarize(a):
 import statistics
 out=P(a.out);rows=ledger(out);m=read(out/'manifest.json');assert len(rows)==len(m['cells']) and all(r['status']==0 for r in rows)
 fixed={}
 for row in rows:
  if row['kind']=='fixed':fixed[row['version'],row['repeat']]=row['result']
 for repeat in range(3):
  for x,y in zip(fixed['predecessor',repeat],fixed['candidate',repeat]):
   for key in ['id','fen','decision','move','score','nodes','completed_depth']:assert x[key]==y[key],(repeat,x['id'],key,x[key],y[key])
 report={'fixed_exact_equivalence':True,'fixed_rows_compared':99,'active_seconds':sum(r['charged_active_seconds'] for r in rows),'cpu_seconds':sum(r['cpu_seconds'] for r in rows),'process_wall_seconds':sum(r['process_wall_seconds'] for r in rows),'comparisons':{},'limitations':'Fixed-tree equivalence and local execution cost only; selected timing roots and shallow millisecond samples do not establish playing strength. Timed decisions may differ and are reported, not graded.'}
 for name in m['input_sha256']:
  group={}
  for version in ['predecessor','candidate']:
   rs=[x for r in rows if r['input']==name and r['version']==version for x in r['result']]
   group[version]={'median_elapsed_ms':statistics.median(x['elapsed_ms'] for x in rs),'median_visits':statistics.median(x['nodes'] for x in rs),'choices':[x['move'] for x in rs],'completed_depths':[x['completed_depth'] for x in rs],'scores':[x['score'] for x in rs]}
  report['comparisons'][name]=group
 report['fixed_by_id']={}
 for root in fixed['predecessor',0]:
  key=root['id'];report['fixed_by_id'][key]={}
  for version in ['predecessor','candidate']:
   samples=[next(x for x in fixed[version,i] if x['id']==key) for i in range(3)]
   report['fixed_by_id'][key][version]={'median_elapsed_ms':statistics.median(x['elapsed_ms'] for x in samples),'nodes':samples[0]['nodes'],'move':samples[0]['move'],'score':samples[0]['score']}
 write(out/'summary.json',report);print(json.dumps(report,indent=2))
def main():
 p=argparse.ArgumentParser(description=__doc__);s=p.add_subparsers(dest='command',required=True);a=s.add_parser('prepare')
 for k in ['out','repo','predecessor','candidate','source-revision','predecessor-source']:a.add_argument('--'+k,required=True)
 for command in ['next','summarize']:a=s.add_parser(command);a.add_argument('--out',required=True)
 a=p.parse_args();{'prepare':prepare,'next':next_run,'summarize':summarize}[a.command](a)
if __name__=='__main__':main()
