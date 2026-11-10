#!/usr/bin/env python3
"""Native-only adaptation of MSH-155 driver; prepare never searches.
Each next invocation attempts one cell, preserving failures and refusing orphans.
"""
import argparse, datetime, fcntl, hashlib, json, math, os, pathlib, platform, re, resource, signal, subprocess, sys, time, traceback
import chess
DIAGNOSTICS=('disable_null','disable_lmr','disable_quiet_omissions','disable_see_pruning','disable_delta_pruning','endgame_lmr')
def read(p): return json.loads(pathlib.Path(p).read_text())
def write(p,v): pathlib.Path(p).write_text(json.dumps(v,indent=2)+'\n')
def digest(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def board_for(p):
 b=chess.Board(p.get('fen',chess.STARTING_FEN))
 for u in p.get('history',[]): b.push_uci(u)
 if p.get('expected_root_fen'): assert b.fen(en_passant='fen')==p['expected_root_fen'],p['id']
 assert b.is_valid(),p['id']
 return b

def worker(cellpath):
 c=read(cellpath); root=c['root']; b=board_for(root); base=pathlib.Path(cellpath).with_suffix(''); seconds=c['seconds']
 resource.setrlimit(resource.RLIMIT_FSIZE,(8*1024*1024,8*1024*1024))
 resource.setrlimit(resource.RLIMIT_CPU,(math.ceil(seconds)+10,math.ceil(seconds)+12))
 inp={**root,**c['diagnostics'],'mode':'conventional','depth':128,'time_ms':round(seconds*1000),'table_entries':c['table_entries']}
 write(str(base)+'.native-input.json',[inp]); command=[c['binary'],str(base)+'.native-input.json','128','timed','none']
 search_start=utc()
 with open(str(base)+'.native.jsonl','x') as out,open(str(base)+'.native.stderr','x') as err:
  proc=subprocess.run(command,stdout=out,stderr=err,timeout=seconds+8)
 search_end=utc(); assert proc.returncode==0,proc.returncode
 rows=[json.loads(x) for x in pathlib.Path(str(base)+'.native.jsonl').read_text().splitlines()]; assert len(rows)==1
 r=rows[0]; assert r['id']==root['id'],('wrong_root_id',r['id'],root['id']); assert r['node_limit'] is None and r['root_unchanged'] and r['pv_native_admitted']
 assert r['fen']==b.fen(en_passant='fen') and r['mode']=='Conventional' and r['depth_limit']==128
 assert r['time_ms']==round(seconds*1000) and r['table_entries']==c['table_entries']
 observed=dict(re.findall(r'(\w+): (true|false)',r['diagnostics']))
 assert observed=={k:str(v).lower() for k,v in c['diagnostics'].items()},observed
 pv=[x['uci'] for x in r['pv'] if x['kind']=='move']; replay=b.copy();san=[]
 for u in pv:
  m=chess.Move.from_uci(u);assert m in replay.legal_moves,('illegal_pv',u);san.append(replay.san(m));replay.push(m)
 if r['move']:
  assert chess.Move.from_uci(r['move']) in b.legal_moves
  if pv:assert pv[0]==r['move']
 stats=lambda s:{k:int(v) for k,v in re.findall(r'(\w+): (\d+)',s)}
 r['stats_parsed']=stats(r['stats'])
 for i in r['iterations']:i['stats_parsed']=stats(i['stats'])
 active=r['elapsed_ms']/1000;assert math.isfinite(active) and active>=0
 return {'engine':'mshogi','command':command,'native_process_start_utc':search_start,'native_process_end_utc':search_end,'active_search_seconds':active,'result':r,'pv_validation':{'legal':True,'san':san,'final_fen':replay.fen(en_passant='fen')},'completed_semantics':'native explicit completed iterations; stopped result retains last completed decision'}

def registry_lock(path):
 path=pathlib.Path(path).resolve();path.parent.mkdir(parents=True,exist_ok=True)
 lock=open(str(path)+'.lock','a');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB);return lock

def registry_state(path):
 state=read(path);assert state['schema']==1 and state['budget_seconds']>0
 used=0
 for identifier,entry in state.get('external',{}).items():
  assert re.fullmatch(r'[A-Za-z0-9_-]+',identifier)
  reserved=entry['reserved_seconds'];assert math.isfinite(reserved) and reserved>0
  if entry.get('receipt_path'):
   assert digest(entry['receipt_path'])==entry['receipt_sha256'],'external receipt changed'
   receipt=read(entry['receipt_path']);validate_charge(receipt)
   used+=max(reserved,receipt['charged_active_seconds'])
  else:used+=reserved
 for directory,expected in state['directories'].items():
  out=pathlib.Path(directory);assert digest(out/'manifest.json')==expected,'registered manifest changed'
  m=read(out/'manifest.json');assert m['registry_path']==str(pathlib.Path(path).resolve())
  rows=ledger(out);assert [r['index'] for r in rows]==list(range(len(rows))) and len(rows)<=len(m['cells'])
  for row in rows:
   charge=row['charged_active_seconds'];assert isinstance(charge,(float,int)) and math.isfinite(charge) and charge>=0
   assert row['panel_sha256']==m['panel_sha256'] and row['driver_sha256']==m['driver_sha256'] and row['binary_sha256']==m['binary_sha256']
   used+=charge
  # A crash or partially launched attempt must not escape the aggregate budget.
  for receipt in out.glob('cell-*.started.json'):
   assert read(receipt)['index']<len(rows),('global orphan attempt',str(receipt))
  if len(rows)<len(m['cells']):assert not list(out.glob(f"cell-{len(rows):04d}.*")),('global orphan artifacts',directory)
 return state,used

def validate_charge(receipt):
 for key in ('charged_active_seconds','process_wall_seconds','cpu_seconds'):
  value=receipt[key];assert isinstance(value,(float,int)) and math.isfinite(value) and value>=0,key
 assert 'status' in receipt
 for key in ('start_utc','end_utc'):
  value=datetime.datetime.fromisoformat(receipt[key]);assert value.utcoffset()==datetime.timedelta(0)
 assert receipt['start_utc']<=receipt['end_utc']

def save_registry(path,state):
 temporary=pathlib.Path(str(path)+'.tmp');write(temporary,state);os.replace(temporary,path)

def external(a):
 registry=pathlib.Path(a.registry).resolve()
 with registry_lock(registry):
  state,used=registry_state(registry);entries=state.setdefault('external',{})
  assert re.fullmatch(r'[A-Za-z0-9_-]+',a.id),'invalid reservation identifier'
  if a.command=='reserve':
   assert a.id not in entries,'duplicate reservation'
   assert math.isfinite(a.seconds) and a.seconds>0 and used+a.seconds<=state['budget_seconds'],'invalid/excess reservation'
   entries[a.id]={'reserved_seconds':a.seconds,'reserved_utc':utc()}
  else:
   assert a.id in entries and not entries[a.id].get('receipt_path'),'unknown or already charged reservation'
   path=pathlib.Path(a.receipt).resolve();receipt=read(path);validate_charge(receipt);sha=digest(path)
   assert not any(e.get('receipt_sha256')==sha for e in entries.values()),'duplicate receipt'
   entries[a.id].update({'receipt_path':str(path),'receipt_sha256':sha,'finalized_utc':utc(),'actual_charge_seconds':receipt['charged_active_seconds'],'over_reservation':receipt['charged_active_seconds']>entries[a.id]['reserved_seconds']})
  # Always retain actual over-reservation expenditure; subsequent launches are blocked if exhausted.
  save_registry(registry,state);_,total=registry_state(registry)
  print(json.dumps({'id':a.id,'entry':entries[a.id],'effective_global_budget_charge':total,'reserved_unused_seconds_are_not_refunded':True}))

def prepare(a):
 with registry_lock(a.registry):
  prepare_locked(a)

def prepare_locked(a):
 out=pathlib.Path(a.out).resolve();out.mkdir(parents=True,exist_ok=True)
 assert not any(out.iterdir()),'output directory must be empty'
 panel=pathlib.Path(a.panel).resolve();content=read(panel);roots=content if isinstance(content,list) else content['roots']
 assert roots and len({p['id'] for p in roots})==len(roots),'empty/duplicate roots'
 for p in roots:board_for(p)
 assert math.isfinite(a.seconds) and a.seconds>=.001 and math.isfinite(a.budget) and a.budget>0
 assert 0<=a.table_entries<=16384 and (a.table_entries==0 or a.table_entries%4==0)
 binary=pathlib.Path(a.binary).resolve();assert binary.is_file() and os.access(binary,os.X_OK)
 diag={k:k in a.diagnostic for k in DIAGNOSTICS}
 # Do not inherit diagnostic controls or budget overrides from corpus rows.
 cells=[{'index':i,'root':p,'seconds':a.seconds,'binary':str(binary),'table_entries':a.table_entries,'diagnostics':diag} for i,p in enumerate(roots)]
 assert len(cells)*a.seconds+15<=a.budget,'budget requires nominal work plus watchdog reserve'
 manifest={'schema':1,'registry_path':str(pathlib.Path(a.registry).resolve()),'prepared_utc':utc(),'panel_path':str(panel),'panel_sha256':digest(panel),'driver_path':str(pathlib.Path(__file__).resolve()),'driver_sha256':digest(__file__),'binary_path':str(binary),'binary_sha256':digest(binary),'source_revision':a.source_revision,'source_provenance_limit':'revision supplied by coordinator; binary hash frozen, build-to-source attestation must accompany run','python':sys.version,'python_chess':chess.__version__,'host':platform.platform(),'budget_seconds':a.budget,'planned_seconds':len(cells)*a.seconds,'cold_policy':'one sequential fresh native process per cell, no persisted TT','history_policy':'only supplied history; no invented repetition history','cells':cells}
 registry=pathlib.Path(a.registry).resolve()
 if registry.exists():
  state,used=registry_state(registry);assert state['budget_seconds']==a.global_budget,'global budget mismatch'
 else:
  assert math.isfinite(a.global_budget) and a.global_budget>0
  state={'schema':1,'budget_seconds':a.global_budget,'directories':{}};used=0
 assert str(out) not in state['directories'],'already registered'
 assert used+a.seconds+15<=state['budget_seconds'],'global budget exhausted'
 write(out/'manifest.json',manifest);state['directories'][str(out)]=digest(out/'manifest.json')
 save_registry(registry,state)
 print(json.dumps({'cells':len(cells),'planned_seconds':manifest['planned_seconds'],'binary_sha256':manifest['binary_sha256']}))

def ledger(out):
 p=out/'runs.jsonl';return [json.loads(l) for l in p.read_text().splitlines()] if p.exists() else []
def next_cell(out):
 out=pathlib.Path(out).resolve();m=read(out/'manifest.json')
 with registry_lock(m['registry_path']):
  state,used=registry_state(m['registry_path']);assert str(out) in state['directories'],'unregistered output'
  next_cell_locked(out,state,used)

def next_cell_locked(out,global_state,global_used):
 out=pathlib.Path(out).resolve()
 with open(out/'run.lock','a') as lock:
  fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
  m=read(out/'manifest.json');assert digest(__file__)==m['driver_sha256'],'driver changed'
  assert digest(m['panel_path'])==m['panel_sha256'],'panel changed';assert digest(m['binary_path'])==m['binary_sha256'],'binary changed'
  runs=ledger(out);indices=[r['index'] for r in runs];assert indices==list(range(len(indices))),'nonsequential/corrupt ledger'
  assert len(runs)<=len(m['cells'])
  if len(runs)==len(m['cells']):print('ALL CELLS ATTEMPTED');return
  c=m['cells'][len(runs)];used=sum(r['charged_active_seconds'] for r in runs)
  assert used+c['seconds']+15<=m['budget_seconds'],'budget reserve exhausted'
  assert global_used+c['seconds']+15<=global_state['budget_seconds'],'global budget reserve exhausted'
  base=out/f"cell-{c['index']:04d}";assert not list(out.glob(base.name+'.*')),'orphan attempt; inspect explicitly, never silently overwrite'
  # Durable start receipt precedes the worker. A controller crash leaves an orphan and blocks resumption.
  write(str(base)+'.json',c);write(str(base)+'.started.json',{'start_utc':utc(),'index':c['index'],'nominal_seconds':c['seconds']})
  before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.monotonic();started=utc();status=None;p=None
  command=[sys.executable,str(pathlib.Path(__file__).resolve()),'worker',str(base)+'.json']
  try:
   with open(str(base)+'.worker.stdout','x') as stdout,open(str(base)+'.worker.stderr','x') as stderr:
    p=subprocess.Popen(command,stdout=stdout,stderr=stderr,start_new_session=True)
    try:status=p.wait(timeout=c['seconds']+15)
    except subprocess.TimeoutExpired:status='watchdog';os.killpg(p.pid,signal.SIGKILL);p.wait()
  except OSError as exc:status='launch_failure';write(str(base)+'.launch-error.json',{'error':str(exc)})
  finally:
   if p:
    try:os.killpg(p.pid,signal.SIGKILL)
    except ProcessLookupError:pass
  wall=time.monotonic()-start;ended=utc();after=resource.getrusage(resource.RUSAGE_CHILDREN)
  resultpath=pathlib.Path(str(base)+'.result.json');result=None
  try:
   if status==0:
    result=read(resultpath);active=result['active_search_seconds'];assert isinstance(active,(int,float)) and math.isfinite(active) and 0<=active<=wall+1
  except (ValueError,OSError,KeyError,TypeError,AssertionError):status='malformed_result';result=None
  charged=result['active_search_seconds'] if status==0 else wall
  row={'index':c['index'],'root_id':c['root']['id'],'engine':'mshogi','seconds':c['seconds'],'diagnostics':c['diagnostics'],'status':status,'start_utc':started,'end_utc':ended,'process_wall_seconds':wall,'cpu_seconds':after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,'charged_active_seconds':charged,'aggregate_charged_seconds':used+charged,'global_aggregate_charged_seconds':global_used+charged,'result':result,'result_path':str(resultpath) if result else None,'artifact_sha256':{p.name:digest(p) for p in out.glob(base.name+'.*')},'panel_sha256':m['panel_sha256'],'driver_sha256':m['driver_sha256'],'binary_sha256':m['binary_sha256']}
  with open(out/'runs.jsonl','a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
  print(json.dumps({'index':row['index'],'root_id':row['root_id'],'status':status,'aggregate_charged_seconds':used+charged}))
  if status!=0:raise SystemExit('cell failed; preserved and charged; no silent rerun')

def main():
 parser=argparse.ArgumentParser(description=__doc__);sub=parser.add_subparsers(dest='command',required=True)
 p=sub.add_parser('prepare')
 for key in ('panel','binary','out','source-revision','registry'):p.add_argument('--'+key,required=True)
 p.add_argument('--global-budget',type=float,default=3300);p.add_argument('--seconds',type=float,default=5);p.add_argument('--budget',type=float,required=True);p.add_argument('--table-entries',type=int,default=4096);p.add_argument('--diagnostic',choices=DIAGNOSTICS,action='append',default=[])
 for command in ('next','status'):
  p=sub.add_parser(command);p.add_argument('--out',required=True)
 for command in ('reserve','charge'):
  p=sub.add_parser(command);p.add_argument('--registry',required=True);p.add_argument('--id',required=True)
  if command=='reserve':p.add_argument('--seconds',type=float,required=True)
  else:p.add_argument('--receipt',required=True)
 p=sub.add_parser('worker');p.add_argument('cell')
 a=parser.parse_args()
 if a.command=='prepare':prepare(a)
 elif a.command=='next':next_cell(a.out)
 elif a.command=='status':print(json.dumps(ledger(pathlib.Path(a.out)),indent=2))
 elif a.command in ('reserve','charge'):external(a)
 elif a.command=='worker':write(str(pathlib.Path(a.cell).with_suffix(''))+'.result.json',worker(a.cell))
if __name__=='__main__':
 try:main()
 except BaseException:traceback.print_exc();raise
