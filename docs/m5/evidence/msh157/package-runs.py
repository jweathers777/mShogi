from pathlib import Path
import json,hashlib,tarfile,io,gzip,copy
BASE=Path('/tmp/msh157-runs');DEST=Path('/Users/johnweathers/.codex/worktrees/chess-adversarial-assessment/mShogi/docs/m5/evidence/msh157')
def sha(b):return hashlib.sha256(b).hexdigest()
def dump(x):return (json.dumps(x,indent=2)+'\n').encode()
def issts(root):return str(root.get('id','')).startswith('sts-') or root.get('source_kind')=='sts'
def ref(root):return {'id':root['id'],'source_kind':'sts','state_and_expectation':'Reconstruct the frozen source fixture via MSH-156 corpus references; upstream source rights not established.','source_id':root.get('source_id'),'source_line':root.get('source_line'),'partition':root.get('partition')}
def no_fen(x):
 if isinstance(x,dict):return {k:no_fen(v)for k,v in x.items()if 'fen'not in k.lower()}
 if isinstance(x,list):return [no_fen(v)for v in x]
 return x
entries={};mapping={};omitted={}
def add(name,p,data=None,reason=None):
 original=p.read_bytes();data=original if data is None else data;entries[name]=data;mapping[name]={'sha256':sha(data),'original_sha256':sha(original),'mode':reason or 'original-bytes'}
def skip(name,p,why):omitted[name]={'original_sha256':sha(p.read_bytes()),'reason':why}
for folder in sorted(BASE.iterdir()):
 if not folder.is_dir()or not(folder/'manifest.json').exists():continue
 manifest=json.loads((folder/'manifest.json').read_text());cells=manifest['cells'];sts={c['index']for c in cells if issts(c['root'])};prefix='native/'+folder.name+'/'
 for p in sorted(folder.iterdir()):
  if not p.is_file()or p.name.endswith('.lock')or p.name=='grades.json':continue
  name=prefix+p.name
  if p.name=='manifest.json':
   m=copy.deepcopy(manifest)
   for c in m['cells']:
    if issts(c['root']):c['root']=ref(c['root'])
   add(name,p,dump(m),'STS source roots replaced with retrieval references'if sts else 'JSON formatting only');continue
  if p.name=='runs.jsonl':
   rows=[]
   for line in p.read_text().splitlines():
    r=json.loads(line)
    # First verify every original runtime fingerprint before packaging.
    for f,h in r['artifact_sha256'].items():assert sha((folder/f).read_bytes())==h,(folder,f)
    if r['index']in sts:r=no_fen(r)
    rows.append(json.dumps(r))
   add(name,p,('\n'.join(rows)+'\n').encode(),'STS generated result FEN fields omitted; original artifact hashes retained');continue
  if p.name.startswith('cell-'):
   idx=int(p.name.split('-')[1].split('.')[0])
   if idx in sts:
    if p.name.endswith('.result.json'):
     add(name,p,dump(no_fen(json.loads(p.read_text()))),'STS generated result FEN fields omitted');continue
    if p.name.endswith('.native.jsonl'):
     add(name,p,('\n'.join(json.dumps(no_fen(json.loads(x)))for x in p.read_text().splitlines())+'\n').encode(),'STS generated result FEN fields omitted');continue
    if p.name.endswith('.started.json'):add(name,p);continue
    skip(name,p,'STS source input/log omitted; reconstruct fixture and retain original execution fingerprint');continue
  add(name,p)
# Reference source inputs and raw UCI position commands need the same STS treatment.
for folder in sorted(Path('/tmp').glob('msh157-reference-*')):
 if not folder.is_dir()or not(folder/'manifest.json').exists():continue
 m=json.loads((folder/'manifest.json').read_text());sts={q['query_index']for q in m['queries']if q['root_id'].startswith('sts-')};prefix='reference/'+folder.name+'/'
 for p in sorted(folder.iterdir()):
  if not p.is_file()or p.name.endswith('.lock'):continue
  name=prefix+p.name
  if p.name=='manifest.json':
   x=copy.deepcopy(m)
   for k,r in x['roots'].items():
    if issts(r):x['roots'][k]=ref(r)
   add(name,p,dump(x),'STS source roots replaced with retrieval references');continue
  if p.name.startswith('query-'):
   idx=int(p.name.split('-')[1].split('.')[0])
   if idx in sts:
    if p.name.endswith('.result.json'):add(name,p,dump(no_fen(json.loads(p.read_text()))),'STS generated result FEN fields omitted');continue
    if p.name.endswith('.events.jsonl'):add(name,p,('\n'.join(json.dumps(no_fen(json.loads(x)))for x in p.read_text().splitlines())+'\n').encode(),'STS generated PV FEN fields omitted');continue
    if p.name.endswith('.pending.json'):add(name,p);continue
    skip(name,p,'STS source input/raw position command omitted; original fingerprint retained');continue
  if p.name=='runs.jsonl':
   for line in p.read_text().splitlines():
    r=json.loads(line)
    for f,h in r['artifact_sha256'].items():assert sha((folder/f).read_bytes())==h,(folder,f)
  add(name,p)
# Top-level files have no source rows except the execution fast-panel, excluded.
for name in ['registry.json','runner.py','grade.py','fast-selection.json','quiet-confirmation-selection.json']:
 p=BASE/name
 if p.exists():add('control/'+name,p)
for name in ['msh157-batch.py','msh157-paired-batch.py','msh157-final-campaign.py','msh157-depth-runner.py','msh157-reference-driver.py','msh157-reference-candidates.json','msh157-depth-plan.json','msh157-child-plan.json','msh157-fast-panel.py','msh157-quiet-confirmation.py','msh157-reference-final-driver.py','msh157-reference-final-candidates.json']:
 p=Path('/tmp')/name
 if p.exists():add('control/'+name,p)
DEST.mkdir(parents=True,exist_ok=True)
buf=io.BytesIO()
with tarfile.open(fileobj=buf,mode='w')as t:
 for name,data in sorted(entries.items()):
  info=tarfile.TarInfo(name);info.size=len(data);info.mtime=0;info.mode=0o644;t.addfile(info,io.BytesIO(data))
compressed=gzip.compress(buf.getvalue(),mtime=0);(DEST/'decision-runs.tar.gz').write_bytes(compressed)
(DEST/'decision-bundle-manifest.json').write_text(json.dumps({'archive':'decision-runs.tar.gz','sha256':sha(compressed),'entries':mapping,'omitted_original_artifacts':omitted,'note':'Archived hashes authenticate stored bytes; original hashes authenticate inspected local execution artifacts. STS source state/inputs are intentionally reference-only, not claimed byte-preserved in archive.'},indent=2)+'\n')
print(json.dumps({'archived_files':len(entries),'omitted_STS_source_artifacts':len(omitted),'compressed_bytes':len(compressed)}))
