import pickle,collections,sys,statistics as st
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
print('== level->lost single-move collapses (root E .30-.70, played E<.15), prepared; rate per 1000 level-root moves; games affected')
def matband(m): return 'm>=45' if m>=45 else 'm27-44' if m>26 else 'm15-26' if m>=15 else 'm<=14'
for keyf,name in ((lambda r:r['phase'],'phase'),(lambda r:matband(r['material']),'material')):
    n=C();d=C();gs=collections.defaultdict(set);conf=C()
    for k,rs in by.items():
        if games[k]['family'][0]=='unaided': continue
        for r in rs:
            eb,ep=r['uq']['E'],r['pq']['E']
            if eb is None or ep is None or not(.30<=eb<=.70): continue
            kk=('cand' if r['cand'] else 'opp',keyf(r)); n[kk]+=1
            if ep<.15:
                d[kk]+=1; gs[kk].add(k); conf[kk]+= r['nstatus']=='supported_finite'
    for kk in sorted(n): print(name,kk,'level moves',n[kk],'collapses',d[kk],'rate/1000 %.1f'%(1000*d[kk]/n[kk]),'games',len(gs[kk]),'20M-supported',conf[kk])
print('== advantage give-backs (root E>=.85, played E<=.60), prepared')
for keyf,name in ((lambda r:r['phase'],'phase'),):
    n=C();d=C();gs=collections.defaultdict(set);conf=C()
    for k,rs in by.items():
        if games[k]['family'][0]=='unaided': continue
        for r in rs:
            eb,ep=r['uq']['E'],r['pq']['E']
            if eb is None or ep is None or eb<.85: continue
            kk=('cand' if r['cand'] else 'opp',keyf(r)); n[kk]+=1
            if ep<=.60:
                d[kk]+=1; gs[kk].add(k); conf[kk]+= r['nstatus']=='supported_finite'
    for kk in sorted(n): print(name,kk,'ahead moves',n[kk],'give-backs',d[kk],'rate/1000 %.1f'%(1000*d[kk]/n[kk]),'games',len(gs[kk]),'20M-supported',conf[kk])
print('== same with reference cp: root cp>=+300 and played cp<=+80')
n=C();d=C();gs=collections.defaultdict(set)
for k,rs in by.items():
    if games[k]['family'][0]=='unaided': continue
    for r in rs:
        a,b=cp(r['uq']['score']),cp(r['pq']['score'])
        if a is None or b is None or a<300: continue
        kk=('cand' if r['cand'] else 'opp',r['phase']); n[kk]+=1
        if b<=80: d[kk]+=1; gs[kk].add(k)
for kk in sorted(n): print(kk,'moves at>=300',n[kk],'give-backs',d[kk],'rate/1000 %.1f'%(1000*d[kk]/n[kk]),'games',len(gs[kk]))
