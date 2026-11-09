import json,time,hashlib,os,signal
from pathlib import Path
import chess,chess.engine
out=Path('/private/tmp/msh152');binary=Path('/private/tmp/msh49/opponent/stockfish/stockfish-macos-universal');digest=hashlib.sha256(binary.read_bytes()).hexdigest();assert digest=='8eed61129d1493c5d1f2fd9323f0c54c47ac49319911fbde18c6b9c87e8b13c5'
rows=json.load(open('docs/m5/evidence/msh152/positions.json'))[:2]
result=[];engine=None;pid=None;start=time.monotonic();err=None;settled=False
try:
 engine=chess.engine.SimpleEngine.popen_uci(str(binary),timeout=5,start_new_session=True);pid=engine.transport.get_pid();engine.configure({'Threads':1,'Hash':16,'UCI_LimitStrength':False,'Skill Level':20})
 for row in rows:
  for suffix,line in [('root',[]),('chosen',[('e3e4' if '003-0' in row['id'] else 'd7e8')]),('forecast-exchange' if '003-0' in row['id'] else 'rook-invasion',(['e3e4','a8e8','e4f5','e8e5','f4e5'] if '003-0' in row['id'] else ['d7e8','b1b8']))]:
   b=chess.Board(row['fen']);[b.push_uci(a) for a in row['history']];assert b.fen()==row['expected_root_fen'];side=b.turn
   for a in line:b.push_uci(a)
   infos=engine.analyse(b,chess.engine.Limit(time=3),multipv=2,game=object())
   result.append({'id':row['id']+'/'+suffix,'extra_history':line,'fen':b.fen(),'root_side':chess.COLOR_NAMES[side],'opinions':[{'depth':i.get('depth'),'nodes':i.get('nodes'),'time':i.get('time'),'score_root':str(i['score'].pov(side)),'pv':[m.uci() for m in i.get('pv',[])]} for i in infos]})
except BaseException as e:err=repr(e);raise
finally:
 if engine:
  try:engine.quit()
  except BaseException:
   try:os.killpg(pid,signal.SIGKILL)
   except ProcessLookupError:pass
  engine.close()
  try:os.killpg(pid,0)
  except ProcessLookupError:settled=True
 (out/'reference.json').write_text(json.dumps({'binary_sha256':digest,'engine':'Stockfish19 full strength default embedded network; reference opinions only, not product evaluation or independent tactic proof','options':{'Threads':1,'Hash':16,'MultiPV':2},'limit_seconds_per_root':3,'elapsed_seconds':time.monotonic()-start,'error':err,'process_group_settled':settled,'results':result},indent=2)+'\n')
print([(r['id'],[(x['score_root'],x['pv'][:6]) for x in r['opinions']]) for r in result])
