import pickle,sys,json,gzip,collections,re
rows,games=pickle.load(open(sys.argv[1],'rb'))
by=collections.defaultdict(list)
for r in rows:
    if r['origin']=='played': by[(r['pair'],r['game'])].append(r)
for k in by: by[k].sort(key=lambda r:r['ply'])
cache={}
def load(p):
    if p not in cache:
        d=json.load(gzip.open(p)) if p.endswith('.gz') else json.load(open(p))
        cache[p]=json.loads(d['report_json']) if 'report_json' in d else d
    return cache[p]
def parse(info):
    o={}
    t=info.split()
    for key in ('depth','seldepth','nodes','time'):
        if key in t: o[key]=int(t[t.index(key)+1])
    if 'score' in t:
        i=t.index('score'); o['score']=(t[i+1],int(t[i+2]))
    if 'pv' in t: o['pv']=t[t.index('pv')+1:][:10]
    return o
mism=0;last=collections.Counter()
for k,g in games.items():
    d=load(g['report']); gm=d['games'][k[1]]
    ids=[' '.join(e['uci_identity']) for e in gm['engines']]
    cand_eng=[i for i,s in enumerate(ids) if 'mShogi' in s][0]
    g['cand_eng']=cand_eng
    es=gm['transcript']['entries']
    moves=[];lastinfo={};dephist={}
    for e in es:
        if e['direction']!='response': continue
        t=e['text']
        if t.startswith('info') and ' score ' in t: lastinfo[e['engine']]=t
        elif t.startswith('bestmove'):
            moves.append((e['engine'],t.split()[1],parse(lastinfo.get(e['engine'],''))))
            lastinfo.pop(e['engine'],None)
    rs=by[k]
    g['claim_last_engine']=None
    if len(moves)==len(rs)+1:
        g['claim_last_engine']='cand' if moves[-1][0]==cand_eng else 'opp'; g['claim_move']=moves[-1]
    elif len(moves)!=len(rs): mism+=1; print('len mismatch',k,len(moves),len(rs))
    for r,m in zip(rs,moves):
        if m[1]!=r['played'] or (m[0]==cand_eng)!=r['cand']: mism+=1
        r['own']=m[2]
print('mismatches',mism)
print(collections.Counter((g['result'],g['claim'],g['claim_last_engine']) for g in games.values()))
pickle.dump((rows,games),open(sys.argv[1],'wb'))
