import pickle,collections,sys,statistics as st
rows,games=pickle.load(open(sys.argv[1],'rb'))
C=collections.Counter
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
def ce(r):  # candidate-perspective root E
    e=r['uq']['E'] if r['uq'] else None
    if e is None: return None
    return e if r['cand'] else 1-e
print('== symmetric screening loss per move (root E mover in [.1,.9]) by phase, prepared only')
for strat in ('P','U'):
  agg=collections.defaultdict(lambda: [0,0.0,0,0,0])
  for k,rs in by.items():
    if (games[k]['family'][0]=='unaided')!=(strat=='U'): continue
    for r in rs:
        if r['loss'] is None or r['uq']['E'] is None: continue
        if not(.1<=r['uq']['E']<=.9): continue
        a=agg[(r['phase'],'cand' if r['cand'] else 'opp')]
        l=max(r['loss'],0)
        a[0]+=1;a[1]+=l;a[2]+=l>=.05;a[3]+=l>=.10;a[4]+=l>=.20
  for key,a in sorted(agg.items()): print(strat,key,'n',a[0],'mean loss %.4f'%(a[1]/a[0]),'>=.05 %.3f >=.10 %.3f >=.20 %.3f'%(a[2]/a[0],a[3]/a[0],a[4]/a[0]))
print('== 20M-supported finite events by side/phase (selected, so not rates)')
print(sorted(C((('cand' if r['cand'] else 'opp'),r['phase'],r['nsev']) for rs in by.values() for r in rs if r['nstatus']=='supported_finite').items()))
print('== trajectory shapes (screening root E, candidate perspective)')
shape=C(); detail=[]
for k,rs in by.items():
    g=games[k]; es=[(r['ply'],ce(r),r['cand']) for r in rs if ce(r) is not None]
    cturn=[e for p,e,c in es if c]
    fav=any(cturn[i]>=.75 and cturn[i+1]>=.75 for i in range(len(cturn)-1))
    bad=any(cturn[i]<=.25 and cturn[i+1]<=.25 for i in range(len(cturn)-1))
    lost=any(cturn[i]<=.10 and cturn[i+1]<=.10 for i in range(len(cturn)-1))
    won=any(cturn[i]>=.90 and cturn[i+1]>=.90 for i in range(len(cturn)-1))
    st_='U' if g['family'][0]=='unaided' else 'P'
    shape[(st_,g['result'],'fav' if fav else '-', 'bad' if bad else '-')]+=1
    g['fav']=fav;g['bad']=bad;g['won90']=won;g['lost10']=lost;g['E0']=es[0][1];g['maxE']=max(cturn);g['minE']=min(cturn)
for k,v in sorted(shape.items()): print(k,v)
print('won90 by result',sorted(C((('U' if g['family'][0]=='unaided' else 'P'),g['result'],g['won90'],g['lost10']) for g in games.values()).items()))
print('== first root E (cand perspective) after supplied prefix, prepared: mean', st.mean(g['E0'] for g in games.values() if g['family'][0]!='unaided'))
pickle.dump((rows,games),open(sys.argv[1],'wb'))
