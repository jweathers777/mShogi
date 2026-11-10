import chess,chess.engine,json,pathlib,logging,time,resource
log=logging.getLogger('chess.engine');log.setLevel(logging.DEBUG);log.addHandler(logging.FileHandler('/tmp/msh-gameplay-bound-safe-key03-raw-uci.log'));log.propagate=False
P=json.loads(pathlib.Path('/tmp/msh-gameplay-panel.json').read_text())
S=[(P[0],{'played':P[0]['played'],'human_h3':P[0]['alternative'],'prior_reference_Rab1':'a1b1','current_Bf5':'d3f5'}),(P[3],{'played':P[3]['played'],'prior_reference_Ne4':'d6e4','current_Rad8':'a8d8'})]
S=S[1:];start=time.monotonic();O=[];e=chess.engine.SimpleEngine.popen_uci('/private/tmp/msh49/opponent/stockfish/stockfish-macos-universal',timeout=5);e.configure({'Threads':1,'Hash':16,'SyzygyPath':''})
try:
 for p,alternatives in S:
  b=chess.Board()
  for u in p['history']:b.push_uci(u)
  assert b.fen()==p['fen'];r={**p,'engine':e.id,'searches':{},'node_limit':2000000}
  for name,uci in {'unrestricted':None,**alternatives}.items():
   root=None if uci is None else [chess.Move.from_uci(uci)]; assert root is None or root[0] in b.legal_moves
   snapshots=[]
   with e.analysis(b,chess.engine.Limit(nodes=2000000),game=object(),root_moves=root) as analysis:
    for info in analysis:
     if 'score' not in info:continue
     bb=b.copy();pv=info.get('pv',[]);san=[]
     for m in pv:assert m in bb.legal_moves;san.append(bb.san(m));bb.push(m)
     assert not pv or root is None or pv[0] in root
     snapshots.append({'depth':info.get('depth'),'seldepth':info.get('seldepth'),'nodes':info.get('nodes'),'time':info.get('time'),'score':info['score'].pov(b.turn).score(mate_score=100000),'mate':info['score'].pov(b.turn).mate(),'lowerbound':bool(info.get('lowerbound',False)),'upperbound':bool(info.get('upperbound',False)),'pv':[m.uci() for m in pv],'san':san})
   exact=[i for i in snapshots if not i['lowerbound'] and not i['upperbound']];r['searches'][name]={'last_exact_iteration':exact[-1] if exact else None,'last_scored_info':snapshots[-1] if snapshots else None,'scored_snapshots':snapshots}
  O.append(r);pathlib.Path('/tmp/msh-gameplay-bound-safe-key03.json').write_text(json.dumps(O,indent=2));print(p['pair'],p['game'],[(k,v['last_exact_iteration']) for k,v in r['searches'].items()],flush=True)
finally:
 e.quit();rr=resource.getrusage(resource.RUSAGE_CHILDREN);print('RESOURCE',json.dumps({'elapsed':time.monotonic()-start,'user':rr.ru_utime,'sys':rr.ru_stime}),flush=True)
