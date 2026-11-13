import pickle,collections,sys
sys.path.insert(0,sys.argv[2]); import chess
rows,games=pickle.load(open(sys.argv[1],'rb'))
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
def cp(s):
    if not s: return None
    k,v=(s['kind'],s['value']) if isinstance(s,dict) else s
    return v if k=='cp' else ('M%d'%v)
def board_at(k,ply_index):
    g=games[k]; b=chess.Board(g['fen'])
    start=len(g['hist'])-len(by[k])
    for m in g['hist'][:start+ply_index]: b.push_uci(m)
    return b
def sanline(b,pv,n=10):
    bb=b.copy();out=[]
    for u in pv[:n]:
        try:
            m=chess.Move.from_uci(u)
            if m not in bb.legal_moves: break
            out.append(bb.san(m)); bb.push(m)
        except Exception: break
    return ' '.join(out)
def show(k,i,tag=''):
    rs=by[k]; r=rs[i]; b=board_at(k,i); o=r.get('own') or {}
    nxt=[x for x in rs[i+1:] if x['cand']==r['cand']][:4]
    opp=rs[i+1] if i+1<len(rs) else None
    print('%s %s g%d ply%d %s %s mat%s %s to move | E %.2f->%.2f cp %s->%s | 20M %s %s | own %s d%s then %s | oppown %s'%(tag,k[0][5:],k[1],r['ply'],games[k]['result'],r['phase'],r['material'],r['mover'],r['uq']['E'],r['pq']['E'],cp(r['uq']['score']),cp(r['pq']['score']),r['nstatus'],r['nmargin'],cp(o.get('score')),o.get('depth'),[cp((x.get('own') or {}).get('score')) for x in nxt],cp(((opp or {}).get('own') or {}).get('score'))))
    print('   FEN',b.fen())
    print('   played:',sanline(b,r['pq']['pv'],11))
    print('   ref   :',sanline(b,r['uq']['pv'],9))
    print('   own pv:',sanline(b,o.get('pv') or [],8))
mode=sys.argv[3]
sel=[]
for k,g in sorted(games.items()):
    rs=by[k]
    if mode=='lossdrop' and g['result']=='losses':
        last=None
        for i,r in enumerate(rs):
            if r['cand'] and r['uq']['E'] is not None and r['pq']['E'] is not None and r['uq']['E']>=.30 and r['pq']['E']<.20: last=i
        if last is not None: sel.append((k,last))
lo,hi=float(sys.argv[4]),float(sys.argv[5])
sel=[(k,i) for k,i in sel if lo<=by[k][i]['material']<=hi]
step=int(sys.argv[6]) if len(sys.argv)>6 else 1
for k,i in sel[::step]: show(k,i)
print(len(sel))
