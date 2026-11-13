import pickle,collections,sys,statistics as st
sys.path.insert(0,sys.argv[2]); import chess
rows,games=pickle.load(open(sys.argv[1],'rb'))
C=collections.Counter
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
res={}
for cls,cond in (('level->lost',lambda eb,ep:.30<=eb<=.70 and ep<.15),('ahead->level-or-worse',lambda eb,ep:eb>=.85 and ep<=.60),('all moves (baseline)',lambda eb,ep:True)):
  for side in (True,False):
    t=C();n=0;gs=set()
    for k,rs in by.items():
        g=games[k]
        if g['family'][0]=='unaided': continue
        b=chess.Board(g['fen']); start=len(g['hist'])-len(rs)
        for m in g['hist'][:start]: b.push_uci(m)
        for r in rs:
            eb,ep=r['uq']['E'],r['pq']['E']
            if r['cand']==side and eb is not None and ep is not None and cond(eb,ep) and (cls.startswith('all') or r['nstatus']=='supported_finite'):
                n+=1; gs.add(k)
                m=chess.Move.from_uci(r['played']); pt=b.piece_type_at(m.from_square)
                ksq=b.king(b.turn)
                if pt==chess.PAWN and not b.is_capture(m) and abs(chess.square_file(m.from_square)-chess.square_file(ksq))<=1 and abs(chess.square_rank(m.from_square)-chess.square_rank(ksq))<=2 and r['material']>26: t['pawn push next to own king (mat>26)']+=1
                if b.is_capture(m): t['played capture']+=1
                if pt==chess.KING and not b.is_castling(m): t['played king move']+=1
                if b.is_check(): t['mover in check']+=1
                alt=r['nalt'] or r['uq']['best']
                try:
                    am=chess.Move.from_uci(alt)
                    if am in b.legal_moves:
                        if b.gives_check(am): t['reference alt gives check']+=1
                        if b.is_capture(am): t['reference alt is capture']+=1
                        if b.is_capture(am) and not b.is_capture(m): t['declined capture (alt capture, played non-capture)']+=1
                except Exception: pass
            b.push_uci(r['played'])
    res[(cls,side)]=(n,len(gs),t)
keys=sorted({x for v in res.values() for x in v[2]})
for cls in ('level->lost','ahead->level-or-worse','all moves (baseline)'):
    a,bb=res[(cls,True)],res[(cls,False)]
    print('==',cls,'| mShogi n',a[0],'games',a[1],'| Oxidation n',bb[0],'games',bb[1])
    for x in keys: print('   %-52s mShogi %5.1f%%   Oxidation %5.1f%%'%(x,100*a[2][x]/a[0],100*bb[2][x]/bb[0]))
