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
# reference cp from candidate perspective at every root
print('== conversion by peak reference cp (two successive roots at/above threshold), prepared | unaided')
for thr in (150,300,500,900):
  for strat in ('P','U'):
    c=C();o=C()
    for k,rs in by.items():
        g=games[k]
        if (g['family'][0]=='unaided')!=(strat=='U'): continue
        cs=[]
        for r in rs:
            v=cp(r['uq']['score'])
            if v is None: continue
            cs.append(v if r['cand'] else -v)
        up=any(cs[i]>=thr and cs[i+1]>=thr and cs[i+2]>=thr and cs[i+3]>=thr for i in range(len(cs)-3))
        dn=any(cs[i]<=-thr and cs[i+1]<=-thr and cs[i+2]<=-thr and cs[i+3]<=-thr for i in range(len(cs)-3))
        if up: c[g['result']]+=1
        if dn: o[g['result']]+=1
        g.setdefault('peak',{})[thr]=(up,dn)
    n=sum(c.values());m=sum(o.values())
    print(strat,'thr',thr,'| mShogi ahead: games',n,'won',c['wins'],'drawn',c['draws'],'lost',c['losses'],'(%.0f%%)'%(100*c['wins']/max(n,1)),'| Oxidation ahead: games',m,'opp won',o['losses'],'drawn',o['draws'],'opp lost',o['wins'],'(%.0f%%)'%(100*o['losses']/max(m,1)))
print('== claim draws among non-converted: which side was ahead >=300 at the end')
pickle.dump((rows,games),open(sys.argv[1],'wb'))
# decisive drop symmetric: for each decisive game, last mover move falling from E>=.30 to <.20, for loser
def lastdrop(k,loser_is_cand):
    last=None
    for r in by[k]:
        if r['cand']!=loser_is_cand or r['uq']['E'] is None or r['pq']['E'] is None: continue
        if r['uq']['E']>=.30 and r['pq']['E']<.20: last=r
    return last
for label,res,lc in (('mShogi losses','losses',True),('Oxidation losses (mShogi wins)','wins',False)):
    ph=C();lag=C();pre=[];post=[];n=0;mat=[]
    for k,g in games.items():
        if g['result']!=res or g['family'][0]=='unaided': continue
        r=lastdrop(k,lc)
        if r is None: ph['none']+=1; continue
        n+=1; ph[r['phase']]+=1; pre.append(cp(r['uq']['score'])); post.append(cp(r['pq']['score']))
        own0=cp((r.get('own') or {}).get('score'))
        nxt=[x for x in by[k] if x['cand']==lc and x['ply']>r['ply']]
        L=None
        for j,x in enumerate(nxt[:8]):
            v=cp((x.get('own') or {}).get('score'))
            if v is not None and own0 is not None and v<=own0-100: L=j+1;break
        lag['no own-score fall>=100 within 8 moves' if L is None else 'lag %d'%L if L<=3 else 'lag 4-8']+=1
    print(label,'prepared n',n,dict(ph)); print('   ref cp before median',st.median(pre),'after median',st.median(post)); print('   own-score awareness',sorted(lag.items()))
