import pickle,collections,sys,statistics as st
sys.path.insert(0,sys.argv[2]); import chess
rows,games=pickle.load(open(sys.argv[1],'rb'))
C=collections.Counter
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
def cp(s):
    if not s: return None
    k,v=(s['kind'],s['value']) if isinstance(s,dict) else s
    return v if k=='cp' else (3000 if v>0 else -3000)
U=sorted(k for k in games if games[k]['family'][0]=='unaided')
def san(hist,n):
    b=chess.Board();o=[]
    for u in hist[:n]: m=chess.Move.from_uci(u); o.append(b.san(m)); b.push(m)
    return ' '.join(o)
print('== unaided games: candidate color, result, plies, first 16 plies, ref cp (cand persp) at ply 10/20/30/40')
for k in U:
    g=games[k]; rs=by[k]; col=[r['mover'] for r in rs if r['cand']][0]
    cs={r['ply']:(cp(r['uq']['score']) if r['cand'] else -cp(r['uq']['score'])) for r in rs if cp(r['uq']['score']) is not None}
    print(k[0][5:],k[1],col,g['result'],len(g['hist']),'|',san(g['hist'],14),'|',[cs.get(p) for p in (10,20,30,40,60)])
print('== shared prefix lengths among same-color unaided games')
for col in ('white','black'):
    ks=[k for k in U if [r['mover'] for r in by[k] if r['cand']][0]==col]
    L=[]
    for i in range(len(ks)):
        for j in range(i+1,len(ks)):
            a,b=games[ks[i]]['hist'],games[ks[j]]['hist']; n=0
            while n<min(len(a),len(b)) and a[n]==b[n]: n+=1
            L.append(n)
    print(col,'pairwise shared plies: min',min(L),'median',st.median(L),'max',max(L), 'distinct first-12-ply lines',len({tuple(games[k]['hist'][:12]) for k in ks}))
print('== unaided: candidate supported events in first 30 plies (20M)')
for k in U:
    for r in by[k]:
        if r['cand'] and r['ply']<30 and r['nstatus']=='supported_finite': print(k[0][5:],k[1],'ply',r['ply'],r['played'],'alt',r['nalt'],'margin %.2f'%r['nmargin'],'cp',cp(r['uq']['score']),'->',cp(r['pq']['score']),'own',cp((r.get('own') or {}).get('score')))
print('== prepared: reference cp (cand persp) trajectory means at k plies after prefix end')
for off in (0,10,20,30,40):
    v=[]
    for k,rs in by.items():
        if games[k]['family'][0]=='unaided' or len(rs)<=off: continue
        r=rs[off]; c=cp(r['uq']['score'])
        if c is None or abs(c)>=3000: continue
        v.append(max(-600,min(600,c if r['cand'] else -c)))
    print('offset',off,'n',len(v),'mean %.0f median %.0f'%(st.mean(v),st.median(v)),'share<=-100: %.2f share>=100: %.2f'%(sum(x<=-100 for x in v)/len(v),sum(x>=100 for x in v)/len(v)))
