import pickle,collections,sys,statistics as st
rows,games=pickle.load(open(sys.argv[1],'rb'))
C=collections.Counter
by=collections.defaultdict(list)
for r in rows: by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
def concl(g):
    c=g['conclusion']['chess']; k=list(c.keys())[0] if isinstance(c,dict) else c
    return k
print('termination x result');
t=C((('unaided' if g['family'][0]=='unaided' else 'prep'),concl(g),g['result']) for g in games.values())
for k,v in sorted(t.items()): print(k,v)
# candidate color
def ccolor(k):
    for r in by[k]:
        if r['origin']=='played': return r['mover'] if r['cand'] else ('black' if r['mover']=='white' else 'white')
print('color x result', sorted(C((('U' if games[k]['family'][0]=='unaided' else 'P'),ccolor(k),games[k]['result']) for k in games).items()))
# pair outcomes prepared
po=C()
for k in games:
    if k[1]%2==0 or True: pass
pairs=collections.defaultdict(dict)
for k,g in games.items(): pairs[k[0]][ccolor(k)]=g['result']
print('pair outcome (white,black) prepared:', sorted(C((v.get('white'),v.get('black')) for p,v in pairs.items() if games[(p,list(k[1] for k in games if k[0]==p)[0])]['family'][0]!='unaided').items(), key=lambda x:-x[1]))
print('pair outcome unaided:', sorted(C((v.get('white'),v.get('black')) for p,v in pairs.items() if games[(p,list(k[1] for k in games if k[0]==p)[0])]['family'][0]=='unaided').items(), key=lambda x:-x[1]))
# family table
ft=collections.defaultdict(C)
for k,g in games.items(): ft[g['family'][0]][g['result']]+=1
for f,v in sorted(ft.items(), key=lambda x:(x[1]['wins']+.5*x[1]['draws'])/sum(x[1].values())): print(f'{f:48s} W{v["wins"]} D{v["draws"]} L{v["losses"]}')
# length
L=collections.defaultdict(list)
for k,g in games.items(): L[g['result']].append(len(g['hist']))
for r,v in L.items(): print(r,'plies median',st.median(v),'min',min(v),'max',max(v))
