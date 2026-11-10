#!/usr/bin/env python3
"""Independent full source-game/root replay, no engines."""
import gzip,hashlib,json
from collections import Counter
from pathlib import Path
import chess
p=Path(__file__).resolve().parent
raw=(p/'opening-candidates.json.gz').read_bytes(); r=json.loads(gzip.decompress(raw))
for x in r:
 b=chess.Board()
 for m in x['source_game']['source_moves_uci']: b.push_uci(m)
 b=chess.Board()
 for m in x['moves']: b.push_uci(m)
 assert b.fen(en_passant='fen')==x['root_fen']
 assert x['source_game']['source_moves_uci'][:16]==x['moves']
 assert not b.is_game_over()
assert len({x['root_key'] for x in r})==len(r)==330
assert all(n==15 for n in Counter(x['family'] for x in r).values())
initial=[x for x in r if x['candidate_rank']<=4]
report={'candidate_sha256':hashlib.sha256(raw).hexdigest(),'candidate_records':len(r),'full_source_histories_legally_replayed':len(r),'unique_root_identities':len(r),'root_fens_match':len(r),'initial_four_per_family':len(initial),'initial_four_structure_counts':{'central_locked':sum(bool(x['tags']['central_locked_white_pawns']) for x in initial),'center_regimes':dict(Counter(x['tags']['center_regime'] for x in initial)),'mirror_symmetric':sum(x['tags']['pawn_structure_mirror_symmetric'] for x in initial),'opposite_wing_castles':sum(len({c['wing'] for c in x['tags']['castles']})==2 for x in initial),'both_kings_castled':sum(len(x['tags']['castles'])==2 for x in initial),'neither_castled':sum(len(x['tags']['castles'])==0 for x in initial)},'analysis':'source validation only; balance screen and native admission still required'}
c=report['initial_four_structure_counts']
assert c['mirror_symmetric']>=12 and c['opposite_wing_castles']>=10
assert c['center_regimes']['closed']>=15 and c['center_regimes']['open']>=15
(p/'source-validation.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
