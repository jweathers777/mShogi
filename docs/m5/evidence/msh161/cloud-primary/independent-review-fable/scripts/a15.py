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
print('== 1. own score when reference is level (|ref cp|<=30, stable flags none), by side/phase: quantiles of own cp; share |own|>=100, >=200')
agg=collections.defaultdict(list)
for k,rs in by.items():
    for r in rs:
        v=cp(r['uq']['score']); o=cp((r.get('own') or {}).get('score'))
        if v is None or o is None or abs(v)>30: continue
        mb='m<=14' if r['material']<=14 else 'm15-26' if r['material']<=26 else 'm>26'
        agg[('cand' if r['cand'] else 'opp',mb)].append(o)
for kk,v in sorted(agg.items()):
    q=st.quantiles(v,n=10)
    print(kk,'n',len(v),'p10 %d p25 %d med %d p75 %d p90 %d'%(q[0],st.quantiles(v,n=4)[0],st.median(v),st.quantiles(v,n=4)[2],q[8]),'|own|>=100: %.1f%%  >=200: %.1f%%'%(100*sum(abs(x)>=100 for x in v)/len(v),100*sum(abs(x)>=200 for x in v)/len(v)))
print('== 1b. own score when reference says clearly winning for mover (ref cp>=300): share own<=+100')
agg=collections.defaultdict(list)
for k,rs in by.items():
    for r in rs:
        v=cp(r['uq']['score']); o=cp((r.get('own') or {}).get('score'))
        if v is None or o is None or v<300: continue
        agg[('cand' if r['cand'] else 'opp',r['phase'])].append(o)
for kk,v in sorted(agg.items()): print(kk,'n',len(v),'median own %d'%st.median(v),'own<=100: %.1f%%'%(100*sum(x<=100 for x in v)/len(v)),'own<=0: %.1f%%'%(100*sum(x<=0 for x in v)/len(v)))
print('== 2. at level->lost collapses (root E .30-.70 -> <.15): own-score change of erring side over next 2 own moves; other side own-score jump at reply')
for side in (True,False):
    a=C();n=0
    for k,rs in by.items():
        for i,r in enumerate(rs):
            if r['cand']!=side: continue
            eb,ep=r['uq']['E'],r['pq']['E']
            if eb is None or ep is None or not(.30<=eb<=.70) or ep>=.15: continue
            o0=cp((r.get('own') or {}).get('score'))
            mine=[cp((x.get('own') or {}).get('score')) for x in rs[i+1:] if x['cand']==side][:2]
            othb=[cp((x.get('own') or {}).get('score')) for x in rs[:i] if x['cand']!=side][-1:]
            otha=[cp((x.get('own') or {}).get('score')) for x in rs[i+1:] if x['cand']!=side][:1]
            if o0 is None or len(mine)<2 or not othb or not otha or None in mine+othb+otha: continue
            n+=1
            selfsees=min(mine)-o0<=-100; othsees=otha[0]-othb[0]>=100
            a[('erring side sees within 2' if selfsees else 'erring side blind', 'other side sees at reply' if othsees else 'other side no jump')]+=1
    print('erring side =', 'mShogi' if side else 'Oxidation', 'n',n, sorted(a.items()))
print('== 4. games with mShogi >=+300 (4 successive roots) not won: draws/losses, how ended')
for k,g in sorted(games.items()):
    if g['peak'][300][0] and g['result']!='wins':
        rs=by[k]
        # last give-back
        last=None
        for i,r in enumerate(rs):
            a,b=cp(r['uq']['score']),cp(r['pq']['score'])
            if r['cand'] and a is not None and b is not None and a>=200 and b<=60: last=(i,r,a,b)
        s='' if last is None else 'last giveback ply%d %s mat%s cp %s->%s own %s 20M:%s'%(last[1]['ply'],last[1]['phase'],last[1]['material'],last[2],last[3],cp((last[1].get('own') or {}).get('score')),last[1]['nstatus'])
        print(k[0][5:],k[1],g['family'][0][:9],g['result'],'claim' if g['claim'] else list(g['conclusion']['chess'].values())[0] if g['result']=='draws' else '','|',s)
print('== 7. 20M status of screening-detected collapses, by side')
print(C(('cand' if r['cand'] else 'opp',r['nstatus'] or r['tier']) for rs in by.values() for r in rs if r['uq']['E'] is not None and r['pq']['E'] is not None and .30<=r['uq']['E']<=.70 and r['pq']['E']<.15))
