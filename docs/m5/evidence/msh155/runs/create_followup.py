import json,copy,pathlib,shutil,sys,hashlib
base=pathlib.Path(__file__).parent
name=sys.argv[1];p=base/name;p.mkdir(exist_ok=False)
m=json.loads((base/'manifest.json').read_text());roots={x['root']['id']:x['root'] for x in m['cells']};cells=[]
if name=='intermediate':
 for seconds in [2,3]:
  for rid in ['historical-22Nf5','historical-23Rf6']:cells.append({'index':len(cells),'root':roots[rid],'engine':'mshogi','seconds':seconds})
else:
 for ri,rid in enumerate(sys.argv[2:]):
  engines=['mshogi','goldfish','oxidation'];engines=engines[ri:]+engines[:ri]
  for engine in engines:cells.append({'index':len(cells),'root':roots[rid],'engine':engine,'seconds':30})
m['parent_manifest_sha256']=hashlib.sha256((base/'manifest.json').read_bytes()).hexdigest();m['cells']=cells;m['planned_seconds']=sum(c['seconds'] for c in cells);m['followup']=name;m['note']='Adaptive followup, baseline preserved. Global900s includingallrunfolders/adjudication enforced by coordinator; this localledger is not a newallowance.'
shutil.copyfile(base/'driver.py',p/'driver.py');(p/'manifest.json').write_text(json.dumps(m,indent=2)+'\n');shutil.copyfile(base/'run_batch.py',p/'run_batch.py')
print(name,len(cells),m['planned_seconds'])
