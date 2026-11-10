import json,hashlib,urllib.request,urllib.parse,time,collections
from pathlib import Path
import chess
P=Path('/tmp/msh156-data');sel=json.loads((P/'selected-pre-tb.json').read_text());seen3=False;out=[]
# Merge the source's spelling variant of the same STS theme; retain first four hash-selected roots.
c=0
for x in sel:
 if x['source_kind']=='sts' and x['stratum'].startswith('STS(v3.0)'):
  x['stratum']='STS(v3.0) Knight Outposts/Repositioning/Centralization';c+=1
  if c>4:continue
 out.append(x)
sel=out;assert len(sel)==192
(P/'tb-raw').mkdir(exist_ok=True)
def keys(b):return {' '.join(q.fen().split()[:4]) for q in [b,b.mirror(),b.transform(chess.flip_horizontal),b.mirror().transform(chess.flip_horizontal)]}
used=set();games=set()
for x in sel:
 b=chess.Board(x['fen']);used.update(keys(b));games.add(x.get('game_id'))
 for u in x['expectation'].get('continuation',[]):b.push_uci(u);used.update(keys(b))
counts=collections.Counter();attempts=[]
for cand in json.loads((P/'candidates-tb.json').read_text()):
 row=cand['row'];b=chess.Board(cand['fen']);kind=cand['kind'];gid=cand['gid']
 if counts[kind]>=12 or gid in games or keys(b)&used:continue
 near=False
 for x in sel:
  ob=chess.Board(x['fen'])
  if sorted(str(p) for p in ob.piece_map().values())==sorted(str(p) for p in b.piece_map().values()):
   if any(q.turn==ob.turn and sum(q.piece_at(i)!=ob.piece_at(i) for i in range(64))<=4 for q in [b,b.mirror(),b.transform(chess.flip_horizontal),b.mirror().transform(chess.flip_horizontal)]):near=True;break
 if near:continue
 url='https://tablebase.lichess.org/standard?'+urllib.parse.urlencode({'fen':b.fen()});fn=P/'tb-raw'/('puzzle-'+row['PuzzleId']+'.json')
 try:
  if fn.exists():raw=fn.read_bytes()
  else:
   with urllib.request.urlopen(urllib.request.Request(url,headers={'User-Agent':'mShogi-benchmark-research/1.0'}),timeout=20) as r:raw=r.read(200000)
   fn.write_bytes(raw);time.sleep(.15)
  data=json.loads(raw);attempts.append({'id':row['PuzzleId'],'url':url,'sha256':hashlib.sha256(raw).hexdigest()})
 except Exception as e:
  attempts.append({'id':row['PuzzleId'],'url':url,'error':str(e)})
  if '429' in str(e):break
  continue
 cats={'win':1,'draw':0,'loss':-1}
 if data['category'] not in ['win','draw'] or any(m['category'] not in cats for m in data['moves']):continue
 vals={m['uci']:-cats[m['category']] for m in data['moves']};legal={m.uci() for m in b.legal_moves}
 if set(vals)!=legal or len(set(vals.values()))<2:continue
 best=max(vals.values())
 if best!=cats[data['category']]:continue
 x=dict(id='syzygy-puzzle-'+row['PuzzleId'],source_kind='tablebase',source_id=row['PuzzleId'],game_id=gid,stratum=kind,fen=b.fen(),initial_fen=b.fen(),moves=[],history_status='unavailable; root-only context, no invented repetition history',family_key=gid,source_url=url,source_row=row,tablebase_file=str(fn),expectation={'tier':'exact-tablebase-wdl','preferred_moves':[u for u,v in vals.items() if v==best],'root_wdl':best,'move_wdl':vals,'assumptions':'Standard rules with supplied halfmove clock; no prior repetition information. Exact under explicitly initialized root-only history, not a claim about original game repetitions. No uncertain or cursed/blessed API category admitted.','source_sha256':hashlib.sha256(raw).hexdigest()})
 sel.append(x);used.update(keys(b));games.add(gid);counts[kind]+=1
 print('accepted',sum(counts.values()),dict(counts),flush=True)
 if sum(counts.values())==48:break
 if len(attempts)>=180:break
(P/'tb-attempts.json').write_text(json.dumps(attempts,indent=2));(P/'selected-all.json').write_text(json.dumps(sel,indent=2));print('FINAL',len(sel),dict(counts),'requests',len(attempts),flush=True)
