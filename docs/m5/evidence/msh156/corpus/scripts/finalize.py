import json,hashlib,collections,urllib.request,time,io
from pathlib import Path
import chess,chess.pgn
P=Path('/tmp/msh156-data');sel=json.loads((P/'selected-all.json').read_text());(P/'games').mkdir(exist_ok=True);receipts=[]
for x in sel:
 if not x.get('game_id'):continue
 gid=x['game_id'];fn=P/'games'/(gid+'.pgn');url='https://lichess.org/game/export/'+gid+'?clocks=false&evals=false&opening=false'
 try:
  if fn.exists():raw=fn.read_bytes()
  else:
   with urllib.request.urlopen(urllib.request.Request(url,headers={'Accept':'application/x-chess-pgn','User-Agent':'mShogi-benchmark-research/1.0'}),timeout=12) as r:raw=r.read(262144)
   fn.write_bytes(raw);time.sleep(.1)
  game=chess.pgn.read_game(io.StringIO(raw.decode()));b=game.board();moves=[];target=chess.Board(x['fen'])
  for m in game.mainline_moves():
   b.push(m);moves.append(m.uci())
   if b.fen()==target.fen():break
  if b.fen()!=target.fen():raise ValueError('target exact FEN not reached')
  x.update(initial_fen=game.board().fen(),moves=moves,history_status='complete source PGN history through root, exact six-field FEN verified',history_file=str(fn))
  # Reject source repetition claims for exact TB classification rather than infer from board.
  if x['source_kind']=='tablebase' and (b.can_claim_threefold_repetition() or b.is_fivefold_repetition()):raise ValueError('history has repetition claim; TB exact contract invalid')
  receipts.append(dict(game_id=gid,url=url,bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),matched=True))
 except Exception as e:
  x.update(initial_fen=x['fen'],moves=[],history_status='source history unavailable or not exactly matched; explicit root-only context')
  receipts.append(dict(game_id=gid,url=url,error=str(e)))
  if '429' in str(e):
   print('rate-limit; stop history fetch',flush=True);break
 print('history',len(receipts),flush=True)
# Exact mate-in-one proof: enumerate all legal moves, retain every immediate mate.
for x in sel:
 if x['source_kind']=='puzzle' and 'mateIn1' in x.get('themes',[]):
  b=chess.Board(x['fen']);mates=[]
  for m in list(b.legal_moves):
   b.push(m)
   if b.is_checkmate():mates.append(m.uci())
   b.pop()
  assert mates
  x['expectation']={'tier':'exact-mate-in-one','preferred_moves':mates,'all_legal_roots_enumerated':len(list(b.legal_moves)),'proof':'Every legal root move applied using python-chess 1.11.2; terminal checkmate tested. All mating alternatives accepted.'}
# Group STS themes, distribute other strata deterministically; global near-root/game exclusion already holds.
def rank(s):return hashlib.sha256(('MSH156-split-v1'+s).encode()).hexdigest()
sthemes=sorted({x['stratum']for x in sel if x['source_kind']=='sts'},key=rank);devthemes=set(sthemes[:10]);dev=[];held=[]
for kind in ['puzzle','evaluation','tablebase']:
 strata=collections.defaultdict(list)
 for x in sel:
  if x['source_kind']==kind:strata[x['stratum']].append(x)
 for name,arr in sorted(strata.items()):
  arr.sort(key=lambda x:rank(x['family_key']));n=(len(arr)*2+1)//3
  dev+=arr[:n];held+=arr[n:]
for x in sel:
 if x['source_kind']=='sts':(dev if x['stratum']in devthemes else held).append(x)
# Puzzle 8 roots per stratum yields 5dev3held=>60/36 vs desired64/32; deterministic balance4 strata.
while len(dev)<160:
 x=sorted([x for x in held if x['source_kind']=='puzzle'],key=lambda x:rank(x['id']))[0];held.remove(x);dev.append(x)
assert len(dev)==160 and len(held)==80,(len(dev),len(held))
for name,arr in [('development',dev),('heldout',held)]:
 for x in arr:x['partition']=name
 (P/(name+'.json')).write_text(json.dumps(sorted(arr,key=lambda x:x['id']),indent=2)+'\n')
(P/'history-receipts.json').write_text(json.dumps(receipts,indent=2)+'\n')
meta={name:{'count':len(arr),'sources':dict(collections.Counter(x['source_kind']for x in arr)),'tiers':dict(collections.Counter(x['expectation']['tier']for x in arr)),'history':dict(collections.Counter(x['history_status']for x in arr)),'side':dict(collections.Counter('white'if chess.Board(x['fen']).turn else 'black'for x in arr)),'tb_outcomes':dict(collections.Counter(x['expectation'].get('root_wdl')for x in arr if x['source_kind']=='tablebase')),'sha256':hashlib.sha256((P/(name+'.json')).read_bytes()).hexdigest()}for name,arr in [('development',dev),('heldout',held)]}
(P/'metadata.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps(meta),flush=True)
