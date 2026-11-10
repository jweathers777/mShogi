import csv,json,hashlib,re,collections,urllib.request,time,io
from pathlib import Path
import chess,chess.pgn
P=Path('/tmp/msh156-data'); selected=[]; used=set(); games=set();raw={}; seed='MSH156-independent-v1'
def rank(x):return hashlib.sha256((seed+str(x)).encode()).hexdigest()
def keys(b):
 return {' '.join(q.fen().split()[:4]) for q in [b,b.mirror(),b.transform(chess.flip_horizontal),b.mirror().transform(chess.flip_horizontal)]}
def valid(b):return b.is_valid() and not b.is_game_over(claim_draw=False) and len(list(b.legal_moves))>1
for x in json.load(open('docs/m5/chess-decision-benchmark.json')):
 used.update(keys(chess.Board(x['fen'])))
def accept(x,b,line=[]):
 ks=keys(b);q=b.copy()
 for u in line:
  try:q.push_uci(u);ks.update(keys(q))
  except: return False
 if used.intersection(ks) or x.get('game_id') in games:return False
 # conservative same-material nearby-root exclusion, including symmetries
 for old in selected:
  ob=chess.Board(old['fen'])
  if sorted(str(p) for p in ob.piece_map().values())!=sorted(str(p) for p in b.piece_map().values()):continue
  for bb in [b,b.mirror(),b.transform(chess.flip_horizontal),b.mirror().transform(chess.flip_horizontal)]:
   if bb.turn==ob.turn and sum(bb.piece_at(i)!=ob.piece_at(i) for i in range(64))<=4:return False
 used.update(ks)
 if x.get('game_id'):games.add(x['game_id'])
 x.update(fen=b.fen(),initial_fen=b.fen(),moves=[],history_status='unavailable; root-only context, no invented repetition history',family_key=x.get('game_id',x['id']))
 selected.append(x);return True
puzzles=[]
with (P/'puzzles-prefix.csv').open() as f:
 for row in csv.DictReader(f):
  try:
   if int(row['Popularity'])<80 or int(row['NbPlays'])<100 or int(row['RatingDeviation'])>100:continue
   b=chess.Board(row['FEN']);moves=row['Moves'].split();b.push_uci(moves[0]);rating=int(row['Rating']);gid=row['GameUrl'].split('/game/')[-1].split('/')[0]
   if not valid(b):continue
   puzzles.append((row,b,moves[1:],gid))
  except:pass
puzzles.sort(key=lambda p:rank(p[0]['PuzzleId']))
# Twelve strata, eight roots each; same game and near-root families globally excluded.
counts=collections.Counter()
for row,b,line,gid in puzzles:
 themes=row['Themes'].split(); band='easy' if int(row['Rating'])<1400 else 'medium' if int(row['Rating'])<2100 else 'hard'
 kind='defensive' if any(t in themes for t in ['defensiveMove','equality']) else 'mate' if 'mate' in themes else 'quiet' if any(t in themes for t in ['quietMove','intermezzo']) else 'tactical'
 s=(kind,band)
 if counts[s]>=8:continue
 x=dict(id='lichess-puzzle-'+row['PuzzleId'],source_kind='puzzle',source_id=row['PuzzleId'],game_id=gid,stratum='/'.join(s),rating=int(row['Rating']),themes=themes,source_url='https://lichess.org/training/'+row['PuzzleId'],source_row=row,expectation={'tier':'published-puzzle-preference','preferred_moves':[line[0]],'continuation':line,'unlisted':'unresolved; published only-move assertion is not an exhaustive proof'})
 if accept(x,b,line):counts[s]+=1
print('puzzles',sum(counts.values()),dict(counts),flush=True)
# STS four per theme, strongest disjoint theme split assigned later.
sts=collections.defaultdict(list)
for n,s in enumerate((P/'sts-v3.epd').read_text().splitlines(),1):
 try:
  b,ops=chess.Board.from_epd(s);id=ops['id'];theme=id.rsplit('.',1)[0];pts=dict(zip(str(ops['c9']).split(),map(int,str(ops['c8']).split())))
  sts[theme].append((rank(id),n,s,b,ops,pts))
 except:pass
for theme,arr in sorted(sts.items()):
 n=0
 for _,lineno,s,b,ops,pts in sorted(arr):
  x=dict(id='sts-'+ops['id'],source_kind='sts',source_id=ops['id'],stratum=theme,source_url='https://github.com/fsmosca/STS-Rating',source_line=lineno,source_sha256=hashlib.sha256(s.encode()).hexdigest(),expectation={'tier':'published-strategic-preference','preferred_moves':[u for u,v in pts.items() if v==max(pts.values())],'published_points':pts,'unlisted':'unresolved; no automatic zero'})
  if valid(b) and accept(x,b):raw[x['id']]=s;n+=1
  if n==4:break
print('after sts',len(selected),flush=True)
# Deep multi-PV positions; balanced side and material stage. No root game IDs exist.
evals=[]
with (P/'evals-prefix.jsonl').open() as f:
 for lineno,s in enumerate(f,1):
  try:
   row=json.loads(s);b=chess.Board(row['fen']+' 0 1')
   if not valid(b) or len(b.piece_map())<=5:continue
   evs=[e for e in row['evals'] if e['depth']>=22 and len(e['pvs'])>=2 and all('cp'in p for p in e['pvs'])]
   if not evs:continue
   e=max(evs,key=lambda e:(e['depth'],e.get('knodes',0)));stage='endgame' if len(b.piece_map())<=12 else 'middle' if len(b.piece_map())<=24 else 'opening';st=stage+('/white' if b.turn else '/black')
   evals.append((rank(row['fen']),lineno,row,b,e,st))
  except:pass
cs=collections.Counter()
for _,lineno,row,b,e,st in sorted(evals):
 if cs[st]>=6:continue
 # Validate every published line; use all near-best published alternatives, not forced single answer.
 good=True
 for pv in e['pvs']:
  q=b.copy()
  try:
   for u in pv['line'].split():q.push_uci(u)
  except:good=False
 if not good:continue
 sign=1 if b.turn else -1;best=max(sign*p['cp'] for p in e['pvs']);prefs=[p['line'].split()[0] for p in e['pvs'] if best-sign*p['cp']<=30]
 x=dict(id='lichess-eval-'+hashlib.sha256(row['fen'].encode()).hexdigest()[:16],source_kind='evaluation',source_id=hashlib.sha256(row['fen'].encode()).hexdigest(),stratum=st,source_url='https://database.lichess.org/#evals',source_line=lineno,source_row=row,synthetic_clock_policy='Source has four-field FEN. Test explicitly initializes halfmove 0/fullmove 1; these are not historical facts.',expectation={'tier':'finite-reference-opinion','preferred_moves':prefs,'published_analysis':e,'score_pov':'white','tolerance_cp':30,'unlisted':'unresolved; finite MultiPV list is not exhaustive'})
 if accept(x,b,e['pvs'][0]['line'].split()):cs[st]+=1
print('after eval',len(selected),dict(cs),flush=True)
# Tablebase candidate stream: actual puzzle game roots, exact map replaces puzzle answer.
tb=[]
for row,b,line,gid in puzzles:
 if len(b.piece_map())<=5 and not b.castling_rights and b.halfmove_clock<80:
  nonking=[p for p in b.piece_map().values() if p.piece_type!=chess.KING];kind='pawn' if all(p.piece_type==chess.PAWN for p in nonking) else 'rook' if any(p.piece_type==chess.ROOK for p in nonking) else 'minor' if any(p.piece_type in [chess.KNIGHT,chess.BISHOP] for p in nonking) else 'queen';tb.append((row,b,line,gid,kind))
(P/'candidates-tb.json').write_text(json.dumps([{'row':row,'fen':b.fen(),'line':line,'gid':gid,'kind':kind} for row,b,line,gid,kind in tb],indent=2))
(P/'selected-pre-tb.json').write_text(json.dumps(selected,indent=2));(P/'selected-sts-lines.json').write_text(json.dumps(raw,indent=2));print('tb candidates',len(tb),flush=True)
