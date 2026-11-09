"""Three explicitly authorized independent endgame queries; no engine execution."""
import json,datetime,urllib.parse,urllib.request,hashlib,chess
from pathlib import Path
B=Path('/private/tmp/msh151');p=B/'diagnostic-oracles.json'
assert not p.exists(),'Preserve prior receipts; do not silently repeat queries'
chosen={'scout-3-46-Kg5','confirmation-002-0-66-Kd3','confirmation-002-1-50-h2'}
rows=[r for r in json.load(open(B/'diagnostic-selected.json'))['candidates'] if r['id'] in chosen]
assert len(rows)==3
out={'method':'Lichess Syzygy independent board-WDL/DTZ; complete-history native claims remain separate. Child categories are from the child side to move and inverted before comparison. No claim of the first losing move.', 'board_validator':'python-chess '+chess.__version__,'queries':[]}
inverse={'win':'loss','loss':'win','draw':'draw','cursed-win':'blessed-loss','blessed-loss':'cursed-win','unknown':'unknown'}
for row in rows:
 board=chess.Board()
 for move in row['history']:board.push_uci(move)
 assert board.fen()==row['fen'];assert board.is_valid();assert len(board.piece_map())<=7
 url='https://tablebase.lichess.ovh/standard?'+urllib.parse.urlencode({'fen':row['fen']})
 receipt={'context':row,'url':url,'requested_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'full_history_replay_matches':True,'piece_count':len(board.piece_map()),'history_claim_diagnostics_independent_only':{'threefold_current':board.is_repetition(3),'can_claim_threefold':board.can_claim_threefold_repetition(),'can_claim_fifty_moves':board.can_claim_fifty_moves(),'halfmove_clock':board.halfmove_clock}}
 try:
  with urllib.request.urlopen(url,timeout=15) as r:
   raw=r.read();receipt['http_status']=r.status
  data=json.loads(raw);receipt['raw_response_utf8']=raw.decode();receipt['response_sha256']=hashlib.sha256(raw).hexdigest();receipt['response']=data
  assert {m['uci'] for m in data['moves']}=={m.uci() for m in board.legal_moves}
  selected=next(m for m in data['moves'] if m['uci']==row['move'])
  receipt.update(complete_legal_set_matches=True,root_category=data['category'],root_dtz=data.get('dtz'),selected_move=selected,selected_category_from_root=inverse[selected['category']],selected_preserves_root_category=inverse[selected['category']]==data['category'])
  print(row['id'],data['category'],'DTZ',data.get('dtz'),'selected_child',selected['category'],'root_perspective',inverse[selected['category']],'preserves',receipt['selected_preserves_root_category'])
 except Exception as error:
  receipt['error']=repr(error);print(row['id'],'ERROR',error)
 out['queries'].append(receipt);p.write_text(json.dumps(out,indent=2)+'\n')
