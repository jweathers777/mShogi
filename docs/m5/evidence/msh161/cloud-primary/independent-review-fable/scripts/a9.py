import pickle,collections,sys,json,statistics as st
sys.path.insert(0,sys.argv[2]); import chess
rows,games=pickle.load(open(sys.argv[1],'rb'))
C=collections.Counter
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
VAL={chess.PAWN:1,chess.KNIGHT:3,chess.BISHOP:3,chess.ROOK:5,chess.QUEEN:9,chess.KING:0}
def mat(b,color): return sum(VAL[p.piece_type]*(1 if p.color==color else -1) for p in b.piece_map().values())
def mtype(b,m):
    if b.is_castling(m): return 'castle'
    t=[]
    if b.is_capture(m): t.append('capture')
    if b.gives_check(m): t.append('check')
    pt=b.piece_type_at(m.from_square)
    if not t: t.append({1:'pawn',2:'minor',3:'minor',4:'rook',5:'queen',6:'king'}[pt]+'-quiet')
    return '+'.join(t)
def cp(score):
    if not score: return None
    if isinstance(score,dict): k,v=score['kind'],score['value']
    else: k,v=score
    return v if k=='cp' else (3000 if v>0 else -3000)
ev=[]
for k,rs in by.items():
    g=games[k]
    b=chess.Board(g['fen'])
    start=len(g['hist'])-len(rs)
    for m in g['hist'][:start]: b.push_uci(m)
    for i,r in enumerate(rs):
        assert g['hist'][start+i]==r['played']
        if r['cand'] and r['nstatus']=='supported_finite':
            m=chess.Move.from_uci(r['played']); alt=chess.Move.from_uci(r['nalt']) if r['nalt'] else None
            eb=r['uq']['E']; ep=r['pq']['E']
            # material swing along reference PV after played move
            bb=b.copy(); m0=mat(bb,b.turn); sw=[]
            pv=r['pq']['pv']
            try:
                for j,u in enumerate(pv[:9]):
                    bb.push_uci(u); sw.append(mat(bb,b.turn)-m0)
            except Exception: pass
            # alt pv swing
            ba=b.copy(); swa=[]
            try:
                for u in (r['uq']['pv'] or [])[:9]:
                    ba.push_uci(u); swa.append(mat(ba,b.turn)-m0)
            except Exception: pass
            own=cp((r.get('own') or {}).get('score'))
            nxt=[x for x in rs[i+1:] if x['cand']][:3]
            own_next=[cp((x.get('own') or {}).get('score')) for x in nxt]
            opp_reply=rs[i+1] if i+1<len(rs) else None
            ev.append(dict(key=k,ply=r['ply'],phase=r['phase'],material=r['material'],margin=r['nmargin'],sev=r['nsev'],eb=eb,ep=ep,
                played=r['played'],alt=r['nalt'],ptype=mtype(b,m),atype=mtype(b,alt) if alt and alt in b.legal_moves else None,
                sw=sw,swa=swa,own=own,own_next=own_next,depth=(r.get('own') or {}).get('depth'),
                opp_loss=(opp_reply or {}).get('loss'),result=g['result'],fam=g['family'][0],fen=b.fen(),incheck=b.is_check(),
                san=b.san(m),asan=b.san(alt) if alt and alt in b.legal_moves else None,color='w' if b.turn else 'b'))
        b.push_uci(r['played'])
pickle.dump(ev,open(sys.argv[3],'wb'))
print(len(ev),'supported candidate events in',len({e['key'] for e in ev}),'games')
def band(e): return 'ahead' if e>=.75 else 'behind' if e<=.25 else 'balanced'
def after(e): return 'ahead' if e>=.75 else 'behind' if e<=.25 else 'balanced'
ok=[e for e in ev if e['eb'] is not None and e['ep'] is not None]
print('state before -> after (screening E), events, games')
t=C();tg=collections.defaultdict(set)
for e in ok: kk=(band(e['eb']),after(e['ep'])); t[kk]+=1; tg[kk].add(e['key'])
for kk,v in sorted(t.items()): print(kk,v,len(tg[kk]))
print('by stratum:',C(('U' if e['fam']=='unaided' else 'P') for e in ev))
