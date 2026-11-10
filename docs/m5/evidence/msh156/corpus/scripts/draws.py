exec(open('/tmp/msh156-data/tablebases.py').read().split("counts=collections.Counter();attempts=[]")[0].replace("sel=json.loads((P/'selected-pre-tb.json').read_text())","sel=json.loads((P/'selected-pre-tb.json').read_text())"))
# use full selected corpus and build blocked roots
sel=json.loads((P/'selected-all.json').read_text());used=set();games=set()
for x in sel:used.update(keys(chess.Board(x['fen'])));games.add(x.get('game_id'))
candidates=[]
for c in json.loads((P/'candidates-tb.json').read_text()):
 if 'equality' in c['row']['Themes'].split():candidates.append(c)
# Additional public evaluation endgames with independent finite draw opinions only nominate candidates.
with (P/'evals-prefix.jsonl').open() as f:
 for i,s in enumerate(f,1):
  try:
   r=json.loads(s);b=chess.Board(r['fen']+' 0 1')
   if len(b.piece_map())>5 or not b.is_valid() or b.castling_rights or b.is_game_over():continue
   if not any(e['depth']>=22 and e['pvs'][0].get('cp')==0 for e in r['evals']):continue
   pieces=[p.piece_type for p in b.piece_map().values() if p.piece_type!=chess.KING];kind='pawn' if all(p==chess.PAWN for p in pieces) else 'rook' if chess.ROOK in pieces else 'minor' if any(p in [chess.KNIGHT,chess.BISHOP] for p in pieces) else 'queen'
   candidates.append(dict(row=r,fen=b.fen(),gid=None,kind=kind,eval_line=i))
  except:pass
  if len(candidates)>=150:break
attempts=[];accepted=0
for c in candidates:
 b=chess.Board(c['fen']);kind=c['kind'];gid=c['gid'];row=c['row']
 if keys(b)&used or (gid and gid in games):continue
 id=(row['PuzzleId'] if 'PuzzleId' in row else hashlib.sha256(row['fen'].encode()).hexdigest()[:16]);url='https://tablebase.lichess.org/standard?'+urllib.parse.urlencode({'fen':b.fen()});fn=P/'tb-raw'/('draw-'+id+'.json')
 try:
  with urllib.request.urlopen(url,timeout=20) as resp:raw=resp.read(200000)
  fn.write_bytes(raw);d=json.loads(raw);attempts.append(dict(id=id,url=url,sha256=hashlib.sha256(raw).hexdigest()));time.sleep(.15)
 except Exception as e:
  attempts.append(dict(id=id,url=url,error=str(e)))
  if '429' in str(e):break
  continue
 if d['category']!='draw' or any(m['category']not in ['win','draw','loss'] for m in d['moves']):continue
 vals={m['uci']:-{'win':1,'draw':0,'loss':-1}[m['category']] for m in d['moves']}
 if max(vals.values())!=0 or min(vals.values())!=-1 or set(vals)!={m.uci() for m in b.legal_moves}:continue
 # replace an exact win of same material, maintaining total/domain quotas
 replace=next((i for i in range(len(sel)-1,-1,-1) if sel[i]['source_kind']=='tablebase' and sel[i]['stratum']==kind and sel[i]['expectation']['root_wdl']==1),None)
 if replace is None:continue
 x=dict(id='syzygy-draw-'+id,source_kind='tablebase',source_id=id,stratum=kind,fen=b.fen(),initial_fen=b.fen(),moves=[],history_status='unavailable; explicitly root-only history',family_key=gid or 'eval-'+id,source_url=url,source_row=row,tablebase_file=str(fn),expectation={'tier':'exact-tablebase-wdl','root_wdl':0,'preferred_moves':[u for u,v in vals.items() if v==0],'move_wdl':vals,'assumptions':'Supplied halfmove clock; explicitly root-only history, no claim about original-game repetitions; uncertain/cursed/blessed categories excluded.','source_sha256':hashlib.sha256(raw).hexdigest()})
 if gid:x['game_id']=gid
 else:x['synthetic_clock_policy']='Source four-field FEN; halfmove 0/fullmove 1 are explicit test initialization, not historical facts.'
 sel[replace]=x;used.update(keys(b));games.add(gid);accepted+=1
 print('draw accepted',accepted,kind,flush=True)
 if accepted==8 or len(attempts)>=100:break
(P/'draw-attempts.json').write_text(json.dumps(attempts,indent=2));(P/'selected-all.json').write_text(json.dumps(sel,indent=2));print('draws',accepted,'requests',len(attempts),flush=True)
