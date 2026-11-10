import chess,chess.engine,json,pathlib,time,resource
G=json.loads(pathlib.Path('/tmp/msh-gameplay-games.json').read_text()); output=[]; start=time.monotonic();enginepath='/private/tmp/msh139/simbelmyne/target/release/simbelmyne'
e=chess.engine.SimpleEngine.popen_uci(enginepath, timeout=4)
print('engine',e.id, 'options',list(e.options),flush=True)
for g in G:
 b=chess.Board(); out={'pair':g['pair'],'game':g['game'],'rows':[]}
 for d in g['decisions']:
  m=chess.Move.from_uci(d['uci']); side=b.turn
  if d['role']=='msh':
   infos=[]
   for bb in [b,b.copy()]:
    if len(infos):bb.push(m)
    a=e.analyse(bb,chess.engine.Limit(nodes=20000),game=object())
    infos.append({'score':a['score'].pov(side).score(mate_score=100000),'depth':a.get('depth'),'nodes':a.get('nodes'),'pv':[x.uci() for x in a.get('pv',[])]})
   out['rows'].append({'ply':d['ply'],'san':d['san'],'uci':d['uci'],'pre':infos[0],'post':infos[1],'loss':infos[0]['score']-infos[1]['score']})
  b.push(m)
 output.append(out);pathlib.Path('/tmp/msh-gameplay-screen.json').write_text(json.dumps(output,indent=2));print(g['pair'],g['game'],'done',time.monotonic()-start,flush=True)
e.quit();print('RESOURCE',time.monotonic()-start, resource.getrusage(resource.RUSAGE_CHILDREN),flush=True)
