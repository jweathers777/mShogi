import json,gzip,pathlib,re
G=json.loads(pathlib.Path('/tmp/msh-gameplay-games.json').read_text())
for p in sorted(pathlib.Path('docs/m5/evidence/msh151').glob('pair-*')):
 r=json.loads(json.load(gzip.open(p/'report.json.gz'))['report_json'])
 for idx,g in enumerate(r['games']):
  gg=next(a for a in G if a['pair']==p.name and a['game']==idx); last={}; rows=[]
  for e in g['transcript']['entries']:
   if e['engine']!=0:continue
   t=e['text']
   if t.startswith('go '):last={}
   if t.startswith('info '):
    vals={k:re.search(r'\b'+k+r' (-?\d+)',t) for k in ['depth','nodes','time','cp','mate']};last={k:int(v[1]) for k,v in vals.items() if v}
   if t.startswith('bestmove'):
    d=[d for d in gg['decisions'] if d['role']=='msh'][len(rows)];rows.append((d['ply']+1,d['san'],last))
  print('\n',p.name,idx);print(rows)
