import pickle,collections,sys,json,statistics as st
rows,games=pickle.load(open(sys.argv[1],'rb'))
cum=json.load(open('primary-cumulative.json'))
ccol=lambda rs:[r['mover'] for r in rs if r['cand']][0]
C=collections.Counter
f=lambda e:'  ? ' if e is None else '%.2f'%e
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
def ce(r):
    e=r['uq']['E']
    return None if e is None else (e if r['cand'] else 1-e)
claims={(x['pair'],x['game']):x for x in cum['optional_claim_draws']}
print('draws: stratum pair game plies | draw kind | E at 25/50/75% and last8 roots (cand persp) | max min')
for k,g in sorted(games.items()):
    if g['result']!='draws': continue
    rs=by[k]; es=[ce(r) for r in rs]
    cl=claims.get(k)
    kind=json.dumps(g['conclusion'])[:60] if not cl else cl['reason']+' by '+cl['actor']+' cand='+ccol(rs)
    n=len(es)
    print(k[0],k[1],g['family'][0][:10],len(g['hist']),'|',kind,'|',' '.join(f(es[i]) for i in (n//4,n//2,3*n//4)),'|',' '.join(f(e) for e in es[-6:]),'| max %.2f min %.2f'%(g['maxE'],g['minE']))
