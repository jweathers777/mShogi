import chess,chess.engine,json,pathlib,time,resource,logging
logging.getLogger('chess.engine').setLevel(logging.CRITICAL)
P=json.loads(pathlib.Path('/tmp/msh-gameplay-panel.json').read_text());O=[];start=time.monotonic();e=None
try:
 e=chess.engine.SimpleEngine.popen_uci('/private/tmp/msh49/opponent/stockfish/stockfish-macos-universal',timeout=5);e.configure({'Threads':1,'Hash':16,'SyzygyPath':''})
 print('IDENTITY',e.id,flush=True)
 for p in P:
  b=chess.Board()
  for u in p['history']:b.push_uci(u)
  assert b.fen()==p['fen'];res={**p,'engine':e.id,'nodes_limit':500000,'searches':{}}
  for label,root in [('unrestricted',None),('played',[chess.Move.from_uci(p['played'])]),('alternative',[chess.Move.from_uci(p['alternative'])])]:
   a=e.analyse(b,chess.engine.Limit(nodes=500000),game=object(),root_moves=root)
   pv=a.get('pv',[]);bb=b.copy();san=[]
   for m in pv:assert m in bb.legal_moves;san.append(bb.san(m));bb.push(m)
   assert root is None or pv[0] in root
   res['searches'][label]={'score':a['score'].pov(b.turn).score(mate_score=100000),'mate':a['score'].pov(b.turn).mate(),'depth':a.get('depth'),'nodes':a.get('nodes'),'time':a.get('time'),'pv':[m.uci() for m in pv],'san':san}
  O.append(res);pathlib.Path('/tmp/msh-gameplay-reference.json').write_text(json.dumps(O,indent=2));print(p['pair'],p['game'],p['san'],[(k,v['score'],v['depth']) for k,v in res['searches'].items()], 'elapsed',time.monotonic()-start,flush=True)
finally:
 if e:e.quit()
 rr=resource.getrusage(resource.RUSAGE_CHILDREN);print('RESOURCE',json.dumps({'elapsed':time.monotonic()-start,'user':rr.ru_utime,'sys':rr.ru_stime}),flush=True)
