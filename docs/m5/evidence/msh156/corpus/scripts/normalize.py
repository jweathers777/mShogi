from pathlib import Path
import json,chess,hashlib,collections
P=Path('/tmp/msh156-data')
for partition in ['development','heldout']:
 a=json.loads((P/(partition+'.json')).read_text())
 for x in a:
  x['source_root_fen']=x['fen']
  if not x['moves'] and 'FEN'in x.get('source_row',{}):
   x['initial_fen']=x['source_row']['FEN'];x['moves']=[x['source_row']['Moves'].split()[0]];x['history_status']='source FEN plus published opponent move; prior repetition history unavailable'
  elif not x['moves'] and 'fen'in x.get('source_row',{}):x['initial_fen']=x['source_row']['fen']+' 0 1'
  b=chess.Board(x['initial_fen'])
  for m in x['moves']:b.push_uci(m)
  assert b.board_fen()==chess.Board(x['source_root_fen']).board_fen()
  e=x['expectation'];e['preferred_moves']=[b.parse_uci(u).uci() for u in e['preferred_moves']]
  if x['source_kind']=='evaluation':e['uci_conversion']='Source UCI_Chess960 castling converted to orthodox UCI for root matching; raw source rows and PVs preserved.'
  x['fen']=x['initial_fen'];x['history']=x.pop('moves');x['expected_root_fen']=b.fen(en_passant='fen');x['time_ms']=5000
  if x['source_kind']=='tablebase':
   x['tb_moves']=x['expectation']['move_wdl'];x['tablebase_receipt']=json.loads(Path(x['tablebase_file']).read_text());x['expectation']['assumptions']='Exact API categories at supplied halfmove clock, excluding uncertain/cursed/blessed outcomes. Root/source history retained as documented; no repetition claim present in supplied known history. Unknown preceding history is not asserted absent in the original game.'
  if x['source_kind']=='sts':x['source_rights']='Reference-only repository packaging: original author license not established; execute locally from public mirror with MIT notice and upstream qualification.'
 (P/(partition+'.json')).write_text(json.dumps(a,indent=2)+'\n')
meta={}
for part in ['development','heldout']:
 a=json.loads((P/(part+'.json')).read_text());meta[part]={'count':len(a),'sources':dict(collections.Counter(x['source_kind'] for x in a)),'tiers':dict(collections.Counter(x['expectation']['tier']for x in a)),'history':dict(collections.Counter(x['history_status']for x in a)),'sha256':hashlib.sha256((P/(part+'.json')).read_bytes()).hexdigest()}
(P/'metadata.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps(meta,indent=2))
