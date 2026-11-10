import json,hashlib,collections
from pathlib import Path
import chess
import argparse
parser=argparse.ArgumentParser();parser.add_argument('--corpus',type=Path,required=True);parser.add_argument('--repo',type=Path,required=True);args=parser.parse_args();P=args.corpus;a=json.loads((P/'development.json').read_text());b=json.loads((P/'heldout.json').read_text());roots=a+b;errors=[]
def keys(b):return {' '.join(q.fen().split()[:4]) for q in [b,b.mirror(),b.transform(chess.flip_horizontal),b.mirror().transform(chess.flip_horizontal)]}
def linekeys(x):
 b=chess.Board(x['expected_root_fen']);ks=keys(b);lines=[];e=x['expectation']
 if e.get('continuation'):lines.append(e['continuation'])
 for pv in e.get('published_analysis',{}).get('pvs',[]):lines.append(pv['line'].split())
 for line in lines:
  c=b.copy()
  for u in line:c.push_uci(u);ks.update(keys(c))
 return ks
for x in roots:
 c=chess.Board(x['fen'])
 for u in x['history']:
  assert not c.is_game_over(claim_draw=False), 'source history continues after automatic terminal'
  c.push_uci(u)
 assert c.fen(en_passant='fen')==x['expected_root_fen']
 assert c.is_valid()
 assert c.fen()==chess.Board(x['source_root_fen']).fen(), 'all canonical source fields must match'
 legal={m.uci()for m in c.legal_moves};e=x['expectation'];assert set(e['preferred_moves'])<=legal
 if x['source_kind']=='tablebase':
  assert set(x['tb_moves'])==legal
  assert not c.can_claim_threefold_repetition() and not c.is_fivefold_repetition()
  assert not c.can_claim_fifty_moves() and not c.is_seventyfive_moves()
  raw=x['tablebase_receipt'];cats={'win':1,'draw':0,'loss':-1};assert raw['category']in cats
  vals={m['uci']:-cats[m['category']]for m in raw['moves']};assert vals==x['tb_moves']
  assert e['root_wdl']==max(vals.values())
 if e['tier']=='exact-mate-in-one':
  mates=[]
  for m in list(c.legal_moves):
   c.push(m)
   if c.is_checkmate():mates.append(m.uci())
   c.pop()
  assert set(mates)==set(e['preferred_moves'])
linecache={x["id"]:linekeys(x) for x in roots}
rootcache={x["id"]:keys(chess.Board(x["expected_root_fen"])) for x in roots}
for i,x in enumerate(roots):
 for y in roots[i+1:]:
  if x['family_key']==y['family_key'] and x['partition']!=y['partition']:errors.append('same family across split')
  xb=chess.Board(x['expected_root_fen']);yb=chess.Board(y['expected_root_fen'])
  if x['partition']!=y['partition'] and sorted(map(str,xb.piece_map().values()))==sorted(map(str,yb.piece_map().values())):
   if any(q.turn==yb.turn and sum(q.piece_at(j)!=yb.piece_at(j) for j in range(64))<=4 for q in [xb,xb.mirror(),xb.transform(chess.flip_horizontal),xb.mirror().transform(chess.flip_horizontal)]):errors.append('near family across split')
  if rootcache[x['id']]&linecache[y['id']] or rootcache[y['id']]&linecache[x['id']]:errors.append('root/line overlap')
# Old eight roots and declared continuation; root FEN may be reached through history.
old=json.load(open(args.repo/'docs/m5/chess-decision-benchmark.json'));oldkeys=set()
for x in old:
 c=chess.Board(x.get('expected_root_fen',x['fen']));oldkeys.update(keys(c))
 for u in x.get('expected_line',[]):c.push_uci(u);oldkeys.update(keys(c))
for x in roots:
 if linekeys(x)&oldkeys:errors.append('old regression overlap')
assert not errors,collections.Counter(errors)
assert len(a)==160 and len(b)==80
res={'status':'passed','roots':240,'development':160,'heldout':80,'history_replay':240,'legal_expectations':240,'complete_exact_tb_maps':48,'exhaustive_mate1':3,'cross_partition_family_overlap':0,'canonical_symmetry_root_overlap':0,'root_known_continuation_overlap':0,'old_eight_root_and_declared_line_overlap':0,'limitations':['Unknown-game evaluation rows cannot prove absence of all latent game/family relations. Conservative near-board filtering and known-line exclusion reduce but cannot eliminate this risk.','No engine candidate evaluated. Published and finite opinions remain separate from exact outcomes.']}
(P/'validation.json').write_text(json.dumps(res,indent=2)+'\n');print(json.dumps(res,indent=2))
