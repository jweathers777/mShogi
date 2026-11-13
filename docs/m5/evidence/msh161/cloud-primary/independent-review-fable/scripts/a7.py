import pickle,collections,sys,json,statistics as st
rows,games=pickle.load(open(sys.argv[1],'rb'))
C=collections.Counter
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
print('== depth/nodes by side and phase (median depth, median nodes, median seldepth)')
agg=collections.defaultdict(lambda:([],[],[]))
for rs in by.values():
    for r in rs:
        o=r.get('own') or {}
        if 'depth' in o:
            a=agg[('cand' if r['cand'] else 'opp',r['phase'])]; a[0].append(o['depth']);a[1].append(o.get('nodes',0));a[2].append(o.get('seldepth',0))
for k,a in sorted(agg.items()):
    q=st.quantiles(a[0],n=10)
    print(k,'n',len(a[0]),'depth p10 %d med %d p90 %d'%(q[0],st.median(a[0]),q[8]),'nodes med %d'%st.median(a[1]),'seldepth med %d'%st.median(a[2]))
print('== claim draws: last move (completes repetition). mover, own score, ref E best/played')
t=C()
for k,g in sorted(games.items()):
    if not g['claim']: continue
    r=by[k][-1]; o=r.get('own') or {}
    who='cand' if r['cand'] else 'opp'
    # previous candidate/opp own scores
    prevc=[x for x in by[k][-8:] if x['cand']][-1]; prevo=[x for x in by[k][-8:] if not x['cand']][-1]
    eb=r['uq']['E']; ep=r['pq']['E']
    cE=None if eb is None else (eb if r['cand'] else 1-eb)
    state='?' if cE is None else 'candWin' if cE>=.75 else 'candLost' if cE<=.25 else 'level'
    t[(who,state)]+=1
    if state!='level' or who=='opp': print(k,who,'last mover; own score',o.get('score'),'candOwn',prevc['own'].get('score'),'oppOwn',prevo['own'].get('score'),'refE(cand)',cE,'loss',r['loss'])
print(sorted(t.items()))
