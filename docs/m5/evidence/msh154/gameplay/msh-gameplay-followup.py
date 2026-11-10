import json,pathlib,chess,chess.engine,resource,time,logging
logging.getLogger('chess.engine').setLevel(logging.CRITICAL)
G=json.loads(pathlib.Path('/tmp/msh-gameplay-games.json').read_text());P=[]
S=[('000',0,22,'h2h3'),('001',1,23,'d8f6'),('002',0,55,'c7c8'),('003',0,27,'a1e1'),('003',1,35,'f7g6'),('004',1,9,'e8g8'),('005',0,34,'e4e3')]
# 001 candidate alternative taken from unrestricted SF19 only if manually legal alt above fails.
for pair,idx,n,alt in S:
 g=next(g for g in G if g['pair'].startswith('pair-'+pair) and g['game']==idx);ply=2*(n-1)+idx;d=g['decisions'][ply];b=chess.Board(d['fen']);a=chess.Move.from_uci(alt)
 if a not in b.legal_moves:
  if pair=='001':
   R=json.loads(pathlib.Path('/tmp/msh-gameplay-reference.json').read_text());rr=next(r for r in R if r['pair']==g['pair'] and r['game']==idx);alt=rr['searches']['unrestricted']['pv'][0]
  else:raise Exception((pair,alt))
 P.append({'pair':g['pair'],'game':idx,'ply':ply,'played':d['uci'],'san':d['san'],'alternative':alt,'history':[q['uci'] for q in g['decisions'][:ply]],'fen':d['fen']})
pathlib.Path('/tmp/msh-gameplay-followup-panel.json').write_text(json.dumps(P,indent=2))
start=time.monotonic();O=[];e=chess.engine.SimpleEngine.popen_uci('/private/tmp/msh49/opponent/stockfish/stockfish-macos-universal',timeout=5);e.configure({'Threads':1,'Hash':16,'SyzygyPath':''})
try:
 for p in P:
  b=chess.Board()
  for u in p['history']:b.push_uci(u)
  r={**p,'searches':{}}
  for label,root in [('unrestricted',None),('played',[chess.Move.from_uci(p['played'])]),('alternative',[chess.Move.from_uci(p['alternative'])])]:
   a=e.analyse(b,chess.engine.Limit(nodes=2000000),game=object(),root_moves=root);pv=a['pv'];bb=b.copy();san=[]
   for m in pv:assert m in bb.legal_moves;san.append(bb.san(m));bb.push(m)
   assert root is None or pv[0] in root
   r['searches'][label]={'score':a['score'].pov(b.turn).score(mate_score=100000),'depth':a.get('depth'),'nodes':a.get('nodes'),'time':a.get('time'),'pv':[m.uci() for m in pv],'san':san}
  O.append(r);pathlib.Path('/tmp/msh-gameplay-followup.json').write_text(json.dumps(O,indent=2));print(p['pair'],p['game'],p['san'],[(k,v['score'],v['depth']) for k,v in r['searches'].items()],flush=True)
finally:
 e.quit();rr=resource.getrusage(resource.RUSAGE_CHILDREN);print('RESOURCE',json.dumps({'elapsed':time.monotonic()-start,'user':rr.ru_utime,'sys':rr.ru_stime}),flush=True)
