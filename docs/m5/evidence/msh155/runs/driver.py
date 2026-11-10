#!/usr/bin/env python3
"""Bounded sequential MSH-155 diagnostic. prepare/next/status; worker is internal.
No engine search is made by prepare. Every next invocation runs exactly one cell.
"""
import argparse,fcntl,hashlib,json,logging,math,os,pathlib,platform,re,resource,signal,subprocess,sys,time,traceback
import chess,chess.engine
ROOT=pathlib.Path(__file__).resolve().parent
REPO=pathlib.Path('/Users/johnweathers/.codex/worktrees/chess-adversarial-assessment/mShogi')
BINS={'mshogi':'/Users/johnweathers/Development/Personal/mShogi/target/release/examples/chess_frontier_profile','goldfish':'/tmp/msh139/goldfish113/build/goldfish.x','oxidation':'/tmp/msh139/oxidation060/target/release/oxidation'}
MEMORY={'mshogi':{'table_entries':4096,'entry_bytes_not_equated_to_reference_hash':True},'goldfish':{'Hash':16},'oxidation':{'Hash':64}}
def read(p):return json.loads(pathlib.Path(p).read_text())
def write(p,v):pathlib.Path(p).write_text(json.dumps(v,indent=2)+'\n')
def digest(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def board_for(p):
 b=chess.Board(p.get('fen',chess.STARTING_FEN))
 for u in p.get('history',[]):b.push_uci(u)
 if p.get('expected_root_fen'):assert b.fen(en_passant='fen')==p['expected_root_fen'],(p['id'],b.fen(en_passant='fen'),p['expected_root_fen'])
 assert b.is_valid(),p['id']
 return b

def validate_pv(board,moves):
 b=board.copy();san=[]
 for u in moves:
  m=chess.Move.from_uci(u);assert m in b.legal_moves,('illegal_pv',u,b.fen());san.append(b.san(m));b.push(m)
 return {'legal':True,'san':san,'final_fen':b.fen(en_passant='fen')}

def serialize(value):
 if isinstance(value,chess.Move):return value.uci()
 if isinstance(value,chess.engine.PovScore):return {'cp':value.relative.score(),'mate':value.relative.mate(),'perspective':'root_side'}
 if isinstance(value,dict):return {str(k):serialize(v) for k,v in value.items()}
 if isinstance(value,(list,tuple)):return [serialize(v) for v in value]
 if value is None or isinstance(value,(str,int,float,bool)):return value
 return str(value)

class TimedUciLog(logging.FileHandler):
 def __init__(self,path):
  super().__init__(path);self.go=None;self.bestmove=None
 def emit(self,record):
  message=record.getMessage()
  if ': << go ' in message:self.go=time.monotonic()
  if ': >> bestmove ' in message:self.bestmove=time.monotonic()
  super().emit(record)

def worker(cellpath):
 cell=read(cellpath);root=cell['root'];b=board_for(root);engine=cell['engine'];base=pathlib.Path(cellpath).with_suffix('');seconds=cell['seconds']
 resource.setrlimit(resource.RLIMIT_FSIZE,(2*1024*1024,2*1024*1024))
 resource.setrlimit(resource.RLIMIT_CPU,(math.ceil(seconds)+10,math.ceil(seconds)+12))
 if engine=='mshogi':
  inp={**root,'time_ms':round(seconds*1000),'table_entries':4096};write(str(base)+'.native-input.json',[inp])
  command=[BINS[engine],str(base)+'.native-input.json','128','timed','none']
  with open(str(base)+'.native.jsonl','w') as out,open(str(base)+'.native.stderr','w') as err:
   proc=subprocess.run(command,stdout=out,stderr=err,timeout=seconds+8)
  assert proc.returncode==0,proc.returncode
  rows=[json.loads(x) for x in pathlib.Path(str(base)+'.native.jsonl').read_text().splitlines()];assert len(rows)==1
  result=rows[0];assert result['node_limit'] is None and result['root_unchanged'] and result['pv_native_admitted']
  assert result['fen']==b.fen(en_passant='fen')
  assert result['mode']=='Conventional' and result['depth_limit']==128
  assert result['time_ms']==round(seconds*1000) and result['table_entries']==4096
  assert 'true' not in result['diagnostics'] and result['diagnostics'].count('false')==6
  pv=[x['uci'] for x in result['pv'] if x['kind']=='move'];validation=validate_pv(b,pv)
  if result['move']:
   assert chess.Move.from_uci(result['move']) in b.legal_moves
   if pv:assert pv[0]==result['move']
  stats=lambda text:{k:int(v) for k,v in re.findall(r'(\w+): (\d+)',text)}
  result['stats_parsed']=stats(result['stats'])
  for i in result['iterations']:i['stats_parsed']=stats(i['stats'])
  return {'engine':engine,'command':command,'active_search_seconds':result['elapsed_ms']/1000,'result':result,'pv_validation':validation,'completed_semantics':'native explicit completed iterations; stopped result retains last completed decision'}
 logger=logging.getLogger('chess.engine');logger.setLevel(logging.DEBUG);logger.propagate=False
 handler=TimedUciLog(str(base)+'.raw-uci.log');handler.setFormatter(logging.Formatter('%(relativeCreated).3f %(levelname)s %(message)s'));logger.addHandler(handler)
 e=None;events=[];t=None
 try:
  e=chess.engine.SimpleEngine.popen_uci(BINS[engine],timeout=4)
  options={name:{'type':opt.type,'default':opt.default,'min':opt.min,'max':opt.max,'var':opt.var} for name,opt in e.options.items()}
  config={k:v for k,v in MEMORY[engine].items() if k in e.options}
  for k,v in [('Threads',1),('SyzygyPath',''),('UCI_LimitStrength',False)]:
   if k in e.options:config[k]=v
  e.configure(config);e.ping();t=time.monotonic()
  with e.analysis(b,chess.engine.Limit(time=seconds),game=object(),info=chess.engine.INFO_ALL) as analysis:
   for info in analysis:
    event={'received_search_seconds':time.monotonic()-t,'info':serialize(info)}
    if info.get('pv'):event['pv_validation']=validate_pv(b,[m.uci() for m in info['pv']])
    events.append(event)
    with open(str(base)+'.events.jsonl','a') as stream:stream.write(json.dumps(event)+'\n')
   best=analysis.wait()
  elapsed=time.monotonic()-t
  assert handler.go is not None and handler.bestmove is not None,'missing go/bestmove timing'
  active=handler.bestmove-handler.go
  if best.move:assert best.move in b.legal_moves
  else:assert b.is_game_over(claim_draw=True),'null bestmove on nonterminal root'
  scored=[x for x in events if 'score' in x['info']]
  unbound=[x for x in scored if x['info'].get('multipv',1)==1 and not x['info'].get('lowerbound') and not x['info'].get('upperbound')]
  matching=[x for x in unbound if x['info'].get('pv') and best.move and x['info']['pv'][0]==best.move.uci()]
  return {'engine':engine,'engine_id':e.id,'available_options':options,'configured_options':config,'active_search_seconds':active,'analysis_request_seconds':elapsed,'bestmove':best.move.uci() if best.move else None,'ponder':best.ponder.uci() if best.ponder else None,'events':events,'last_scored_event':scored[-1] if scored else None,'last_nonbound_scored_event':unbound[-1] if unbound else None,'last_nonbound_scored_event_matching_bestmove':matching[-1] if matching else None,'completed_semantics':'UCI info has no completed-iteration flag; last nonbound score is an inference, not proof of fully completed depth; all bound and partial events retained'}
 finally:
  if e:
   try:e.quit()
   except Exception:e.close()
  handler.close();logger.removeHandler(handler)

def prepare(panel):
 assert not (ROOT/'manifest.json').exists(),'manifest exists; do not overwrite evidence'
 content=read(panel);roots=content if isinstance(content,list) else content['roots'];assert len({p['id'] for p in roots})==len(roots)
 for p in roots:board_for(p)
 provenance=read(REPO/'docs/m5/evidence/msh154/architecture/provenance.json')
 for filename,h in provenance['source_hashes'].items():assert digest(REPO/filename)==h,('source changed',filename)
 assert digest(BINS['mshogi'])==provenance['binary_sha256']
 cells=[];engines=list(BINS)
 for ti,seconds in enumerate([1,5,15]):
  for pi in range(len(roots)):
   # Rotate root order and engine order across budgets, deterministically.
   ri=(pi+ti)%len(roots)
   order=engines[(ri+ti)%3:]+engines[:(ri+ti)%3]
   for e in order:cells.append({'index':len(cells),'root':roots[ri],'engine':e,'seconds':seconds})
 assert sum(c['seconds'] for c in cells)<=900
 manifest={'schema':1,'native_provenance':provenance,'reference_source_revisions':{'goldfish':'b7aed1c1b9188c55f4d773fd4fac43de30c83cbb','oxidation':'46a705b80e7191e7c302838eb8b85053b5994d65'},'panel_path':str(pathlib.Path(panel).resolve()),'panel_sha256':digest(panel),'driver_sha256':digest(__file__),'python':sys.version,'python_chess':chess.__version__,'host':platform.platform(),'binary_sha256':{k:digest(v) for k,v in BINS.items()},'binary_paths':BINS,'memory_policy':MEMORY,'threads':1,'cold_policy':'fresh process per engine/root/budget; UCI ucinewgame via new game object; native fresh table','history_policy':'entire supplied origin plus history replayed; puzzle FEN has only supplied history','reference_tb_policy':'SyzygyPath empty when supported; no online oracle','budget_seconds':900,'planned_seconds':sum(c['seconds'] for c in cells),'cells':cells}
 write(ROOT/'manifest.json',manifest);print(json.dumps({'cells':len(cells),'planned_seconds':manifest['planned_seconds'],'hashes':manifest['binary_sha256']}))

def ledger():
 p=ROOT/'runs.jsonl';return [json.loads(l) for l in p.read_text().splitlines()] if p.exists() else []
def next_cell():
 with open(ROOT/'run.lock','w') as lock:
  fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
  m=read(ROOT/'manifest.json');assert digest(__file__)==m['driver_sha256'],'driver changed since freeze'
  assert digest(m['panel_path'])==m['panel_sha256'],'panel changed since freeze'
  for k,h in m['binary_sha256'].items():assert digest(BINS[k])==h,('binary changed',k)
  runs=ledger();done={r['index'] for r in runs};remaining=[c for c in m['cells'] if c['index'] not in done]
  if not remaining:print('ALL CELLS ATTEMPTED');return
  cell=remaining[0];used=sum(r['charged_active_seconds'] for r in runs)
  assert used+cell['seconds']+15<=m['budget_seconds'],'budget reserve exhausted'
  base=ROOT/f"cell-{cell['index']:03d}-{cell['engine']}";write(str(base)+'.json',cell)
  # Crash residue is intentionally not overwritten. Resolve orphan evidence explicitly.
  assert not pathlib.Path(str(base)+'.worker.stdout').exists(),'orphan attempt; inspect before retry'
  before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.monotonic();status=None
  command=[sys.executable,str(pathlib.Path(__file__).resolve()),'worker',str(base)+'.json']
  with open(str(base)+'.worker.stdout','w') as out,open(str(base)+'.worker.stderr','w') as err:
   p=subprocess.Popen(command,stdout=out,stderr=err,start_new_session=True)
   try:status=p.wait(timeout=cell['seconds']+15)
   except subprocess.TimeoutExpired:
    os.killpg(p.pid,signal.SIGKILL);p.wait();status='watchdog'
  # Remove any escaped cleanup work in this task-owned process group, including worker failures.
  try:os.killpg(p.pid,signal.SIGKILL)
  except ProcessLookupError:pass
  wall=time.monotonic()-start;after=resource.getrusage(resource.RUSAGE_CHILDREN)
  resultpath=pathlib.Path(str(base)+'.result.json');result=None
  try:result=read(resultpath) if resultpath.exists() else None
  except (ValueError,OSError):status='malformed_result'
  charged=result['active_search_seconds'] if status==0 and result else wall
  row={'index':cell['index'],'root_id':cell['root']['id'],'engine':cell['engine'],'seconds':cell['seconds'],'status':status,'process_wall_seconds':wall,'cpu_seconds':after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,'charged_active_seconds':charged,'aggregate_charged_seconds':used+charged,'result_path':str(resultpath) if result else None,'artifact_sha256':{p.name:digest(p) for p in ROOT.glob(base.name+'.*')},'panel_sha256':m['panel_sha256'],'driver_sha256':m['driver_sha256'],'binary_sha256':m['binary_sha256'][cell['engine']]}
  with open(ROOT/'runs.jsonl','a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
  print(json.dumps(row))
  if status!=0:raise SystemExit('cell failed; preserved artifacts; inspect before continuing')

if __name__=='__main__':
 command=sys.argv[1]
 if command=='prepare':prepare(sys.argv[2])
 elif command=='next':next_cell()
 elif command=='status':print(json.dumps(ledger(),indent=2))
 elif command=='worker':
  try:write(str(pathlib.Path(sys.argv[2]).with_suffix(''))+'.result.json',worker(sys.argv[2]))
  except BaseException:traceback.print_exc();raise
 else:raise SystemExit('prepare PANEL | next | status')
