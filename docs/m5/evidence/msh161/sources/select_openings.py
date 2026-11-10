#!/usr/bin/env python3
"""Prepare frozen candidate ordering from public master-game facts, without engines.
Usage: python select_openings.py /private/tmp/msh161-sources OUTPUT_DIR
Requires python-chess; source archive identities are bound in manifest.json.
"""
import csv, gzip, hashlib, io, json, sys, zipfile
from collections import Counter, defaultdict
from pathlib import Path
import chess, chess.pgn
raw, out = map(Path, sys.argv[1:]); out.mkdir(parents=True, exist_ok=True)
SEED='msh161-independent-master-coverage-v1'
NAMES=['Caruana','Karpov','PolgarJ','Kramnik','Anand','Carlsen']
def sha(b): return hashlib.sha256(b).hexdigest()
def key(b): return ' '.join(b.fen(en_passant='legal').split()[:4])
def family(eco):
 if not eco: return None
 letter,n=eco[0],int(eco[1:])
 if letter=='C':
  if n<=19:return 'french'
  if n>=60:return 'spanish'
  if 50<=n<=59:return 'italian'
  if 40<=n<=49:return 'petroff-scotch-four-knights'
  return 'other-open-games'
 if letter=='B':
  if 10<=n<=19:return 'caro-kann'
  if 20<=n<=29:return 'sicilian-closed-anti'
  if 30<=n<=59:return 'sicilian-classical-sveshnikov-taimanov'
  if n>=60:return 'sicilian-najdorf-dragon-scheveningen-rauzer'
  return 'pirc-modern-other-semiopen'
 if letter=='D':
  if n<=9:return 'queens-pawn-other'
  if n<=19 or 43<=n<=49:return 'slav-semi-slav'
  if n<=29:return 'queens-gambit-accepted'
  if n<=69:return 'queens-gambit-declined'
  return 'grunfeld'
 if letter=='E':
  if n<=9:return 'catalan'
  if n<=19:return 'queens-indian-bogo'
  if n<=59:return 'nimzo-indian'
  return 'kings-indian'
 if letter=='A':
  if 10<=n<=39:return 'english'
  if 4<=n<=9:return 'reti'
  return 'other-flank-indian-dutch'
FAMILIES=['spanish','italian','petroff-scotch-four-knights','french','caro-kann','sicilian-closed-anti','sicilian-classical-sveshnikov-taimanov','sicilian-najdorf-dragon-scheveningen-rauzer','pirc-modern-other-semiopen','slav-semi-slav','queens-gambit-accepted','queens-gambit-declined','grunfeld','catalan','queens-indian-bogo','nimzo-indian','kings-indian','english','reti','other-flank-indian-dutch','queens-pawn-other','other-open-games']
# Root classification uses a recognized board actually reached by ply 16, never
# the source game's later ECO tag; deterministic tie order handles aliases.
eco_map={}
for vol in 'abcde':
 for row in csv.DictReader((raw/(vol+'.tsv')).open(),delimiter='\t'):
  g=chess.pgn.read_game(io.StringIO(row['pgn'])); b=g.end().board()
  eco_map.setdefault(key(b),row)
roots={}; counts=Counter(); rejected=Counter()
for name in NAMES:
 z=zipfile.ZipFile(raw/(name+'.zip')); text=io.StringIO(z.read(z.namelist()[0]).decode('utf-8-sig',errors='replace'))
 ordinal=0
 while (g:=chess.pgn.read_game(text)) is not None:
  ordinal+=1; counts[name]+=1
  try: elo=min(int(g.headers.get(k,'')) for k in ['WhiteElo','BlackElo'])
  except ValueError: rejected['missing_elo']+=1; continue
  if elo<2400: rejected['either_elo_below_2400']+=1; continue
  ms=list(g.mainline_moves())
  if g.errors or g.board().fen()!=chess.STARTING_FEN or len(ms)<24:
   rejected['invalid_nonstandard_or_short']+=1; continue
  b=g.board(); label=None; san=[]; castles=[]; captures=checks=0
  for ply,m in enumerate(ms[:16],1):
   san.append(b.san(m)); captures+=int(b.is_capture(m));
   if b.is_castling(m):castles.append({'side':'white' if b.turn else 'black','wing':'king' if chess.square_file(m.to_square)>chess.square_file(m.from_square) else 'queen','ply':ply})
   b.push(m); checks+=int(b.is_check())
   if key(b) in eco_map: label=eco_map[key(b)]
  fam=family(label['eco']) if label else None
  if fam not in FAMILIES: rejected['unclassified']+=1; continue
  k=key(b); moves=[m.uci() for m in ms[:16]]
  identity={'archive':name+'.zip','ordinal':ordinal,'headers':dict(g.headers),'source_moves_uci':[m.uci() for m in ms]}
  if k in roots:
   roots[k]['archive_occurrences']+=1
   continue
  wp=set(b.pieces(chess.PAWN,chess.WHITE)); bp=set(b.pieces(chess.PAWN,chess.BLACK))
  locked=[chess.square_name(s) for s in wp if s+8 in bp and chess.square_file(s) in [2,3,4,5]]
  entry={'id':'master-'+sha(' '.join(moves).encode())[:16],'family':fam,'eco':label['eco'],'opening_name':label['name'],'fen':None,'moves':moves,'san':san,'root_fen':b.fen(en_passant='fen'),'root_key':k,'archive_occurrences':1,'source_game':identity,'tags':{'central_locked_white_pawns':locked,'pawn_structure_mirror_symmetric':{chess.square_mirror(s) for s in wp}==bp,'white_central_pawns':sum(chess.square_file(s) in [3,4] for s in wp),'black_central_pawns':sum(chess.square_file(s) in [3,4] for s in bp),'opening_captures':captures,'opening_checks':checks,'castles':castles,'white_minor_home_squares':sum(b.piece_at(s)==chess.Piece(t,True) for s,t in [(1,2),(2,3),(5,3),(6,2)]),'black_minor_home_squares':sum(b.piece_at(s)==chess.Piece(t,False) for s,t in [(57,2),(58,3),(61,3),(62,2)])}}
  # Explicit board descriptors: a single blocked pawn pair is not a closed center.
  central_locks=[sq for sq in locked if sq[0] in 'de']
  central_count=sum(chess.square_file(sq) in [3,4] for sq in wp|bp)
  entry['tags']['center_regime']=('closed' if len(central_locks)==2 else 'open' if not central_locks and central_count<=2 else 'mixed')
  entry['selection_hash']=sha((SEED+k).encode()); roots[k]=entry
by=defaultdict(list)
for r in roots.values():by[r['family']].append(r)
# Preserve all eligible records locally for auditable prefreeze selection changes.
(raw/'eligible-roots.json.gz').write_bytes(gzip.compress(json.dumps(list(roots.values())).encode(),mtime=0))
# Explicit coverage minima are assigned before screening or engine observations.
# At most one quota of each kind/family => at most four required roots/family.
# Greedy selection satisfies at least one unmet quota each turn, so first four
# satisfy all quotas whenever each required kind exists in that family's pool.
def conditions(r):
 t=r['tags']
 return {'symmetric':t['pawn_structure_mirror_symmetric'],
         'opposite_castled':len({c['wing'] for c in t['castles']})==2,
         'closed':t['center_regime']=='closed','open':t['center_regime']=='open'}
minimums={'symmetric':12,'opposite_castled':10,'closed':15,'open':15}
quotas={f:{k:0 for k in minimums} for f in FAMILIES}
available={f:{k:sum(conditions(r)[k] for r in by[f]) for k in minimums} for f in FAMILIES}
for kind,total in minimums.items():
 for fam in FAMILIES:
  if sum(quotas[f][kind] for f in FAMILIES)<total and available[fam][kind]:quotas[fam][kind]=1
 if sum(quotas[f][kind] for f in FAMILIES)<total:raise ValueError('insufficient family coverage for '+kind)
selected=[]
for fam in FAMILIES:
 pool=sorted(by[fam],key=lambda r:r['selection_hash']); ordered=[]; covered=set(); attained={k:0 for k in minimums}
 def features(r):
  t=r['tags'];wings=tuple(sorted((c['side'],c['wing']) for c in t['castles']))
  return {('symmetry',t['pawn_structure_mirror_symmetric']),('center',t['center_regime']),('castles',wings)}
 for slot in range(min(15,len(pool))):
  needed={k for k in minimums if attained[k]<quotas[fam][k]}
  chosen=min(pool,key=lambda r:(-sum(conditions(r)[k] for k in needed),-len(features(r)-covered),r['selection_hash']))
  for k in attained:attained[k]+=conditions(chosen)[k]
  covered|=features(chosen);ordered.append(chosen);pool.remove(chosen)
 selected.extend(dict(r,candidate_rank=i+1) for i,r in enumerate(ordered))
(out/'opening-candidates.json.gz').write_bytes(gzip.compress((json.dumps(selected,indent=2)+'\n').encode(),mtime=0))
files={p.name:{'sha256':sha(p.read_bytes()),'bytes':p.stat().st_size} for p in raw.iterdir() if p.suffix in ['.tsv','.zip'] or p.name in ['COPYING.txt','README.md','source-ref.json','pgnmentor-files.html','masters.headers','masters.json']}
manifest={'seed':SEED,'purpose':'Candidate sources only; not balance-cleared or frozen playing sample','filter':'standard start; at least 24 legal plies; both recorded Elo >=2400; first 16 full-history plies; recognized ECO ancestor by root; unique board/turn/rights/legal-EP roots','selection':'preassigned structural minima by family, greedy unmet coverage then unseen symmetry/center/castling features, SHA tie-break; up to15 per family; no result/mShogi/SF judgment consulted','structural_minimums':minimums,'structural_quotas':quotas,'family_structure_available':available,'counts':dict(counts),'rejected':dict(rejected),'eligible_unique_roots':len(roots),'family_available':{f:len(by[f]) for f in FAMILIES},'family_selected':dict(Counter(r['family'] for r in selected)),'files':files,'eco_revision':json.loads((raw/'source-ref.json').read_text())['sha'],'archives':{n+'.zip':'https://www.pgnmentor.com/players/'+n+'.zip' for n in NAMES}}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps({k:manifest[k] for k in ['counts','eligible_unique_roots','family_available','family_selected']},indent=2))
