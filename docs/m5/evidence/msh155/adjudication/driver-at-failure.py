#!/usr/bin/env python3
"""prepare is read-only to engines; next requires an explicit coordinator slot release."""
import sys,pathlib,json,hashlib,time,os,signal,subprocess,resource,math,logging,fcntl,traceback,importlib.util
import chess,chess.engine
ROOT=pathlib.Path('/tmp/msh155-adjudication')
BINARY=pathlib.Path('/private/tmp/msh49/opponent/stockfish/stockfish-macos-universal')
PANEL=pathlib.Path('/tmp/msh155-panel.json')
EXPECTED_PANEL='993785b5c6f1f82c17652ebd0078b3e642132207ac10db5ec0abde15d2c4ec85'
def read(p):return json.loads(pathlib.Path(p).read_text())
def write(p,j):pathlib.Path(p).write_text(json.dumps(j,indent=2)+'\n')
def digest(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def board_for(r):
 b=chess.Board(r['fen'])
 for u in r['history']:b.push_uci(u)
 assert b.fen(en_passant='fen')==r['expected_root_fen'] and b.is_valid()
 return b
class TimedLog(logging.FileHandler):
 def __init__(self,p):super().__init__(p);self.go=None;self.best=None
 def emit(self,record):
  s=record.getMessage()
  if ': << go ' in s:self.go=time.monotonic()
  if ': >> bestmove ' in s:self.best=time.monotonic()
  super().emit(record)
def validate(b,pv):
 x=b.copy();san=[]
 for m in pv:assert m in x.legal_moves;san.append(x.san(m));x.push(m)
 return {'san':san,'final_fen':x.fen(en_passant='fen'),'legal':True}
def worker(qfile):
 q=read(qfile);base=pathlib.Path(qfile).with_suffix('');r=q['root'];b=board_for(r);rootmove=q['query']['root_move'];limit=q['query']['seconds']
 resource.setrlimit(resource.RLIMIT_CPU,(math.ceil(limit)+4,math.ceil(limit)+5));resource.setrlimit(resource.RLIMIT_FSIZE,(2*1024*1024,2*1024*1024))
 handler=TimedLog(str(base)+'.raw-uci.log');handler.setFormatter(logging.Formatter('%(relativeCreated).3f %(levelname)s %(message)s'));logger=logging.getLogger('chess.engine');logger.setLevel(logging.DEBUG);logger.propagate=False;logger.addHandler(handler)
 e=None;events=[];started=time.monotonic()
 try:
  e=chess.engine.SimpleEngine.popen_uci(str(BINARY),timeout=4)
  assert e.id.get('name')=='Stockfish 19',e.id
  config={k:v for k,v in {'Threads':1,'Hash':16,'SyzygyPath':'','UCI_LimitStrength':False}.items() if k in e.options};e.configure(config);e.ping()
  forced=None if rootmove is None else [chess.Move.from_uci(rootmove)];assert forced is None or forced[0] in b.legal_moves
  with e.analysis(b,chess.engine.Limit(time=limit),game=object(),root_moves=forced,info=chess.engine.INFO_ALL) as analysis:
   for info in analysis:
    pv=info.get('pv',[])
    if pv:assert forced is None or pv[0]==forced[0]
    ev={'received_process_seconds':time.monotonic()-started,**{k:info.get(k) for k in ['depth','seldepth','nodes','time','nps','hashfull','multipv'] if k in info},'lowerbound':bool(info.get('lowerbound')),'upperbound':bool(info.get('upperbound')),'pv':[m.uci() for m in pv]}
    if 'score' in info:ev['score']={'cp':info['score'].pov(b.turn).score(),'mate':info['score'].pov(b.turn).mate(),'perspective':'root_side'}
    if pv:ev['pv_validation']=validate(b,pv)
    events.append(ev)
    with open(str(base)+'.events.jsonl','a') as f:f.write(json.dumps(ev)+'\n')
   best=analysis.wait()
  assert best.move in b.legal_moves and (forced is None or best.move==forced[0])
  assert handler.go is not None and handler.best is not None
  unbound=[x for x in events if 'score' in x and x.get('pv') and x.get('multipv',1)==1 and not x['lowerbound'] and not x['upperbound']]
  aligned=[x for x in unbound if x['pv'][0]==best.move.uci()];distinct=[];depths=set()
  for x in reversed(aligned):
   if x.get('depth') not in depths:distinct.append(x);depths.add(x.get('depth'))
   if len(distinct)==2:break
  return {'root_id':r['id'],'query':q['query'],'root_fen':b.fen(en_passant='fen'),'full_history_plies':len(r['history']),'binary_sha256':digest(BINARY),'engine_id':e.id,'configured_options':config,'bestmove':best.move.uci(),'active_search_seconds':handler.best-handler.go,'worker_elapsed_before_shutdown':time.monotonic()-started,'last_nonbound_event':unbound[-1] if unbound else None,'last_nonbound_event_matching_bestmove':aligned[-1] if aligned else None,'last_two_distinct_depth_aligned_nonbound_events':list(reversed(distinct)),'bestmove_alignment_issue':not aligned or (bool(unbound) and unbound[-1]['pv'][0]!=best.move.uci()),'events':events,'limits':'3seconds freshprocess/TT Threads1Hash16 noTB, singletonsearchmoves where supplied; nonboundUCI score is finite reported iteration, not proof'}
 finally:
  if e:
   try:e.quit()
   except Exception:e.close()
  logger.removeHandler(handler);handler.close()
def prepare(candidates):
 ROOT.mkdir(exist_ok=True);assert not (ROOT/'manifest.json').exists(),'preserve existing manifest';assert digest(PANEL)==EXPECTED_PANEL
 c=read(candidates);assert c['state'].startswith('FROZEN_') and not c['baseline_missing_indices'];assert len(c['queries'])<=28 and c['nominal_active_seconds']<=84
 roots={x['id']:x for x in read(PANEL)}
 for q in c['queries']:
  b=board_for(roots[q['root_id']]);assert q['root_move'] is None or chess.Move.from_uci(q['root_move']) in b.legal_moves
 m={'created_utc':__import__('datetime').datetime.now(__import__('datetime').timezone.utc).isoformat(),'candidates_path':str(pathlib.Path(candidates).resolve()),'candidates_sha256':digest(candidates),'driver_sha256':digest(__file__),'binary_path':str(BINARY),'binary_sha256':digest(BINARY),'panel_path':str(PANEL),'panel_sha256':EXPECTED_PANEL,'queries':c['queries'],'roots':roots,'nominal_active_seconds':c['nominal_active_seconds'],'hard_aggregate_active_seconds':100,'no_engine_called_by_prepare':True}
 write(ROOT/'manifest.json',m);print(json.dumps({'queries':len(c['queries']),'nominal_seconds':c['nominal_active_seconds'],'manifest':str(ROOT/'manifest.json')}))
def nextq(release):
 assert release=='coordinator-slot-released','wait for coordinator explicit release'
 with open('/tmp/msh155/run.lock','a') as baseline_lock,open(ROOT/'run.lock','a') as lock:
  fcntl.flock(baseline_lock,fcntl.LOCK_EX|fcntl.LOCK_NB);fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
  m=read(ROOT/'manifest.json');assert digest(__file__)==m['driver_sha256'];assert digest(BINARY)==m['binary_sha256'];assert digest(PANEL)==m['panel_sha256'];assert digest(m['candidates_path'])==m['candidates_sha256']
  ledger=ROOT/'runs.jsonl';runs=[json.loads(l) for l in ledger.read_text().splitlines()] if ledger.exists() else [];done={x['query_index'] for x in runs};pending=[q for q in m['queries'] if q['query_index'] not in done]
  if not pending:print('ALL APPROVED QUERIES ATTEMPTED');return
  q=pending[0];used=sum(x['charged_active_seconds'] for x in runs);assert used+q['seconds']+6<=100,'100second cap with watchdog reserve'
  base=ROOT/f"query-{q['query_index']:03d}";assert not pathlib.Path(str(base)+'.worker.stdout').exists(),'orphanattempt inspect before retry';write(str(base)+'.json',{'query':q,'root':m['roots'][q['root_id']]})
  start=time.monotonic();before=resource.getrusage(resource.RUSAGE_CHILDREN)
  with open(str(base)+'.worker.stdout','w') as out,open(str(base)+'.worker.stderr','w') as err:
   p=subprocess.Popen([sys.executable,__file__,'worker',str(base)+'.json'],stdout=out,stderr=err,start_new_session=True)
   try:status=p.wait(timeout=q['seconds']+6)
   except subprocess.TimeoutExpired:os.killpg(p.pid,signal.SIGKILL);p.wait();status='watchdog'
  try:os.killpg(p.pid,signal.SIGKILL)
  except ProcessLookupError:pass
  wall=time.monotonic()-start;after=resource.getrusage(resource.RUSAGE_CHILDREN);rp=pathlib.Path(str(base)+'.result.json');result=read(rp) if rp.exists() else None;charged=result['active_search_seconds'] if status==0 and result else wall
  row={'query_index':q['query_index'],'root_id':q['root_id'],'root_move':q['root_move'],'status':status,'charged_active_seconds':charged,'aggregate_charged_seconds':used+charged,'process_wall_seconds':wall,'cpu_seconds':after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,'result_path':str(rp) if result else None,'artifact_sha256':{p.name:digest(p) for p in ROOT.glob(base.name+'.*')}}
  with ledger.open('a') as f:f.write(json.dumps(row)+'\n');f.flush();os.fsync(f.fileno())
  print(json.dumps(row))
  if status!=0:raise SystemExit('failed query preserved; do not automatically retry')
if __name__=='__main__':
 if sys.argv[1]=='prepare':prepare(sys.argv[2])
 elif sys.argv[1]=='next':nextq(sys.argv[2])
 elif sys.argv[1]=='worker':
  try:write(str(pathlib.Path(sys.argv[2]).with_suffix(''))+'.result.json',worker(sys.argv[2]))
  except BaseException:traceback.print_exc();raise
