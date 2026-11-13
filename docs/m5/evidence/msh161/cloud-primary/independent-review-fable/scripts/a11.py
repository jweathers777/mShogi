import pickle,collections,sys,statistics as st
rows,games=pickle.load(open(sys.argv[1],'rb'))
ev={(e['key'],e['ply']):e for e in pickle.load(open(sys.argv[2],'rb'))}
C=collections.Counter
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
def cp(s):
    if not s: return None
    k,v=(s['kind'],s['value']) if isinstance(s,dict) else s
    return v if k=='cp' else (3000 if v>0 else -3000)
out=[]
def crossing(k,thr,direction):
    """last candidate move with root E>=thr and played E<thr (falling through thr)"""
    last=None
    for r in by[k]:
        if not r['cand'] or r['uq']['E'] is None or r['pq']['E'] is None: continue
        if r['uq']['E']>=thr and r['pq']['E']<thr-0.10: last=r
    return last
print('LOSSES (60): last candidate move falling from E>=.30 to <.20')
tl=C();rowsL=[]
for k,g in sorted(games.items()):
    if g['result']!='losses': continue
    r=crossing(k,.30,0)
    if r is None: tl['no single crossing (gradual/opp-move reveal)']+=1; rowsL.append((k,None)); continue
    e=ev.get((k,r['ply']))
    rowsL.append((k,r,e))
print(dict(tl))
def desc(k,r,e):
    g=games[k]; o=r.get('own') or {}
    nxt=[x for x in by[k] if x['cand'] and x['ply']>r['ply']][:3]
    return '%s g%d %-10s ply%3d %-10s mat%2d E %.2f->%.2f sfcp %s->%s own %s d%s next %s | %s%s vs %s [%s/%s] 20M:%s %s sw%s fav=%s'%(k[0][5:],k[1],g['family'][0][:10],r['ply'],r['phase'],r['material'],r['uq']['E'],r['pq']['E'],cp(r['uq']['score']),cp(r['pq']['score']),cp(o.get('score')),o.get('depth'),[cp((x.get('own') or {}).get('score')) for x in nxt], '' , e['san'] if e else r['played'], (e['asan'] if e else r['uq']['best']), e['ptype'] if e else '', e['atype'] if e else '', r['nstatus'], ('%.2f'%r['nmargin']) if r['nmargin'] is not None else '', (e['sw'][:8] if e else ''), g['fav'])
for x in rowsL:
    if x[1] is None: print(x[0],'none'); continue
    print(desc(*x))
