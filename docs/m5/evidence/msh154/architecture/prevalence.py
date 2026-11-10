import collections,json,chess,pathlib
out=pathlib.Path('/tmp/msh-adversarial-architecture-inputs')
games=json.load(open('/tmp/msh-gameplay-games.json'))
panel={(p['pair'],p['game'],p['ply']) for p in json.load(open('/tmp/msh-gameplay-panel.json'))}
def identity(b):return ' '.join(b.fen(en_passant='legal').split()[:4])
rows=[]; summaries=[]
for game in games:
 b=chess.Board(); all_counts=collections.Counter([identity(b)]); segment=all_counts.copy(); grow=[]; last_reset=0
 for d in game['decisions']:
  glob=any(c>=2 for c in all_counts.values()); live=any(c>=2 for c in segment.values())
  row=dict(pair=game['pair'],game=game['game'],ply=d['ply'],role=d['role'],global_repeat=glob,live_reversible_repeat=live,obsolete_only=glob and not live,halfmove=b.halfmove_clock,last_irreversible_ply=last_reset,panel=(game['pair'],game['game'],d['ply']) in panel)
  if d['role']=='msh':grow.append(row);rows.append(row)
  move=chess.Move.from_uci(d['uci']); assert move in b.legal_moves
  rights=b.castling_rights; zero=b.is_zeroing(move); b.push(move)
  k=identity(b);all_counts[k]+=1
  if zero or b.castling_rights!=rights:segment=collections.Counter();last_reset=d['ply']+1
  segment[k]+=1
 summaries.append(dict(pair=game['pair'],game=game['game'],msh_decisions=len(grow),global_repeat=sum(r['global_repeat'] for r in grow),live_repeat=sum(r['live_reversible_repeat'] for r in grow),obsolete_only=sum(r['obsolete_only'] for r in grow),first_obsolete=next((r['ply'] for r in grow if r['obsolete_only']),None)))
result=dict(method='python-chess1.11.2 legal-EP identity; reset segment after pawn move/capture/castling-rights loss exactly matching native irreversible concept; complete history occurrence counts kept',games=summaries,total_msh_decisions=len(rows),total_global_repeat=sum(r['global_repeat'] for r in rows),total_live_repeat=sum(r['live_reversible_repeat'] for r in rows),total_obsolete_only=sum(r['obsolete_only'] for r in rows),panel=[r for r in rows if r['panel']],rows=rows)
(out/'prevalence.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k not in ('rows','method')},indent=2))
