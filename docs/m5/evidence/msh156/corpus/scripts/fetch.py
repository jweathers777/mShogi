import urllib.request,hashlib,json,time
from pathlib import Path
p=Path('/tmp/msh156-data');rows=[]
for name,url,cap in [('puzzles-prefix.zst','https://database.lichess.org/lichess_db_puzzle.csv.zst',16*1024**2),('evals-prefix.zst','https://database.lichess.org/lichess_db_eval.jsonl.zst',32*1024**2),('sts-v3.epd','https://raw.githubusercontent.com/fsmosca/STS-Rating/master/STS1-STS15_LAN_v3.epd',1024**2),('sts-LICENSE','https://raw.githubusercontent.com/fsmosca/STS-Rating/master/LICENSE',65536),('lichess-source.html','https://database.lichess.org/',2*1024**2)]:
 req=urllib.request.Request(url,headers={'Range':f'bytes=0-{cap-1}','User-Agent':'mShogi-research/1.0'})
 try:
  with urllib.request.urlopen(req,timeout=45) as r:
   b=r.read(cap);headers=dict(r.headers);status=r.status;final=r.url
  (p/name).write_bytes(b);rows.append(dict(name=name,url=url,final_url=final,status=status,headers=headers,bytes=len(b),cap=cap,sha256=hashlib.sha256(b).hexdigest(),retrieved=time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())))
 except Exception as e:rows.append(dict(name=name,url=url,error=str(e)))
 (p/'fetch-manifest.json').write_text(json.dumps(rows,indent=2)+'\n')
 print(name,rows[-1].get('bytes',rows[-1].get('error')),flush=True)
