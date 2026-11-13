import pickle,collections,sys,json,statistics as st
rows,games=pickle.load(open(sys.argv[1],'rb'))
cum=json.load(open('primary-cumulative.json'))
C=collections.Counter
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
def ce(r):
    e=r['uq']['E']
    return None if e is None else (e if r['cand'] else 1-e)
t=C();lst=[]
for x in cum['optional_claim_draws']:
    k=(x['pair'],x['game']); rs=by[k]
    ccol=[r['mover'] for r in rs if r['cand']][0]
    es=[ce(r) for r in rs[-6:] if ce(r) is not None]
    m=st.median(es)
    state='cand_winning' if m>=.75 else 'cand_losing' if m<=.25 else 'level'
    actor='cand' if x['actor']==ccol else 'opp'
    t[(x['reason'],state,'claim_actor='+actor)]+=1
    if state!='level': lst.append((k,x['reason'],state,actor,len(games[k]['hist']),[round(e,2) for e in es], rs[-1]['uq']['score'], rs[-1]['material']))
for k,v in sorted(t.items()): print(k,v)
for l in lst: print(l)
