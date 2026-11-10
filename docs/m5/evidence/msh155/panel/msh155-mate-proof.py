import sys, json, time, chess, chess.pgn, io, functools, signal
start=time.process_time(); nodes=0
signal.signal(signal.SIGALRM, lambda *args: (_ for _ in ()).throw(TimeoutError('30-second wall ceiling')))
signal.alarm(30)
reports=[]
for pid,plies in [('004X6',3),('SfkRC',5)]:
 j=json.load(open('/tmp/msh155-puzzle'+pid+'.json')); b=chess.pgn.read_game(io.StringIO(j['game']['pgn'])).end().board(); attacker=b.turn
 @functools.lru_cache(None)
 def mate(fen,remaining):
  global nodes
  nodes+=1; pos=chess.Board(fen)
  if pos.is_checkmate():return pos.turn!=attacker
  if pos.is_stalemate() or pos.is_insufficient_material() or remaining==0:return False
  moves=list(pos.legal_moves);moves.sort(key=lambda m: (not pos.gives_check(m),not pos.is_capture(m),m.uci()))
  if pos.turn==attacker:
   for m in moves:
    pos.push(m);yes=mate(pos.fen(),remaining-1);pos.pop()
    if yes:return True
   return False
  for m in moves:
   pos.push(m);yes=mate(pos.fen(),remaining-1);pos.pop()
   if not yes:return False
  return True
 rootmap={}
 for m in list(b.legal_moves):
  b.push(m);rootmap[m.uci()]=mate(b.fen(),plies-1);b.pop()
 report={'id':pid,'root_fen':b.fen(),'plies':plies,'all_legal_root_moves':rootmap,'mate_root_moves':[m for m,v in rootmap.items() if v],'nodes_cumulative':nodes,'cpu_seconds_cumulative':time.process_time()-start,'cache':str(mate.cache_info())}
 reports.append(report);print(json.dumps(report),flush=True)
 signal.alarm(0)
json.dump({'method':'Exact AND/OR mate-within-N-plies over every legal move; attacker existential, defender universal; stalemate and insufficient material are failures; no pruning except logical short-circuit and exact memoization','python_chess':chess.__version__,'results':reports,'cpu_seconds':time.process_time()-start,'nodes':nodes},open('/tmp/msh155-mate-proof.json','w'),indent=2)
