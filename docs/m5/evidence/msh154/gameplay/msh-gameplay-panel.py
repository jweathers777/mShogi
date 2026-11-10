import json,pathlib,chess
G=json.loads(pathlib.Path('/tmp/msh-gameplay-games.json').read_text())
# One primary each, selected before usable reference queries. Includes neutral phase controls.
S=[('000',0,22,'h2h3','possible compensation/coordination deterioration; fallback 21.a3'),('000',1,27,'c7d6','winning-game neutral middlegame control'),('001',0,24,'g2g3','winning-game opposite-flank strategy control'),('001',1,23,'e6f7','suspected attack underestimation, fallback 19...f5'),('002',0,52,'f4d3','endgame pawn/king coordination, fallback 55.Nf4'),('002',1,48,'c2c4','drawn rook-ending passers/control'),('003',0,13,'f5h3','compensation judgment, fallback 27.e4'),('003',1,66,'h2h1q','late tactical liquidation versus passer promotion; earlier deterioration unresolved'),('004',0,22,'c3a4','winning-game bishop pair/coordination control'),('004',1,21,'h8h6','central break opens king lines, fallback 31...Ke8'),('005',0,34,'e4e3','winning rook ending trade/passers control'),('005',1,13,'d8d6','winning-game central passed pawn/attack control')]
P=[]
for pair,idx,number,alt,why in S:
 g=next(g for g in G if g['pair'].startswith('pair-'+pair) and g['game']==idx);ply=2*(number-1)+idx;d=g['decisions'][ply];b=chess.Board(d['fen']);a=chess.Move.from_uci(alt)
 if a not in b.legal_moves: print('ILLEGAL ALT',pair,idx,number,alt);continue
 P.append({'pair':g['pair'],'game':idx,'ply':ply,'played':d['uci'],'san':d['san'],'alternative':alt,'alternative_san':b.san(a),'reason':why,'fen':d['fen'],'history':[q['uci'] for q in g['decisions'][:ply]]})
pathlib.Path('/tmp/msh-gameplay-panel.json').write_text(json.dumps(P,indent=2));print(json.dumps([{k:p[k] for k in ['pair','game','ply','san','alternative_san','reason']} for p in P],indent=2))
