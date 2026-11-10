import json,gzip,pathlib,chess,chess.pgn
allgames=[]
for p in sorted(pathlib.Path('docs/m5/evidence/msh151').glob('pair-*')):
 r=json.loads(json.load(gzip.open(p/'report.json.gz'))['report_json'])
 for idx,g in enumerate(r['games']):
  mr=json.loads(g['match_record_json']); ir=json.loads(mr['initial_record_json']); moves=[e['event']['action'] for e in ir['events'] if e['event']['kind']=='move']
  moves += [e['request']['command']['move'] for e in mr['events'] if isinstance(e['request']['command'],dict) and 'move' in e['request']['command'] and e['reply']['rejection'] is None]
  b=chess.Board(ir['initial_fen']); game=chess.pgn.Game.from_board(b);node=game
  game.headers.update({'Event':p.name,'White':'mShogi' if g['engines_by_color'][0]==0 else r['config']['engines'][1]['model_identity'].split('/')[0],'Black':'mShogi' if g['engines_by_color'][1]==0 else r['config']['engines'][1]['model_identity'].split('/')[0],'Result': '1/2-1/2' if 'draw' in g['conclusion']['chess'] else '1-0' if g['conclusion']['chess'].get('checkmate',{}).get('winner')=='white' else '0-1'})
  decisions=[]
  for ply,u in enumerate(moves):
   m=chess.Move.from_uci(u);assert m in b.legal_moves,(p,idx,ply,u,b.fen())
   san=b.san(m);role='msh' if g['engines_by_color'][int(not b.turn)]==0 else 'opp'
   values={chess.PAWN:1,chess.KNIGHT:3,chess.BISHOP:3,chess.ROOK:5,chess.QUEEN:9,chess.KING:0}
   mat=sum(values[k]*len(b.pieces(k,chess.WHITE))-values[k]*len(b.pieces(k,chess.BLACK)) for k in values)
   decisions.append({'ply':ply,'uci':u,'san':san,'role':role if ply>=len(ir['events']) else 'book','fen':b.fen(),'mat_white':mat})
   b.push(m);node=node.add_variation(m)
  print(f'\n### {p.name} g{idx} {game.headers["White"]} vs {game.headers["Black"]} {game.headers["Result"]} {len(moves)} plies, book {len(ir["events"])}')
  print(game.accept(chess.pgn.StringExporter(headers=False,variations=False,comments=False)))
  print('MATERIAL CHANGES',[(d['ply']+1,d['san'],d['role'],d['mat_white']) for i,d in enumerate(decisions) if i and d['mat_white']!=decisions[i-1]['mat_white']])
  print('END',b.fen())
  allgames.append({'pair':p.name,'game':idx,'headers':dict(game.headers),'decisions':decisions,'book_plies':len(ir['events']),'end_fen':b.fen()})
pathlib.Path('/tmp/msh-gameplay-games.json').write_text(json.dumps(allgames,indent=2))
