from pathlib import Path
import tarfile,hashlib,json,io,re,subprocess
here=Path(__file__).resolve().parent
root=here.parents[3]
manifest=json.loads((here/'bundle-manifest.json').read_text());data={}
for name,info in manifest.items():
 b=(here/name).read_bytes();assert hashlib.sha256(b).hexdigest()==info['sha256'],name
 with tarfile.open(fileobj=io.BytesIO(b),mode='r:gz') as t:
  for member in t.getmembers():
   assert member.isfile() and not member.name.startswith('/') and '..' not in Path(member.name).parts
   x=t.extractfile(member).read();assert hashlib.sha256(x).hexdigest()==info['entries'][member.name],member.name;data[member.name]=x
 assert len(info['entries'])==sum(1 for x in data if x.startswith('runs/' if name.startswith('runs-') else 'adjudication/'))
ledger_count=0;hash_count=0;active=0
for name,b in data.items():
 if not name.endswith('/runs.jsonl'):continue
 for line in b.splitlines():
  r=json.loads(line);ledger_count+=1;active+=r['charged_active_seconds'];prefix=str(Path(name).parent)
  for filename,sha in r['artifact_sha256'].items():
   key=prefix+'/'+filename
   assert key in data,(name,key);assert hashlib.sha256(data[key]).hexdigest()==sha,key;hash_count+=1
assert ledger_count==109 and hash_count==683
assert abs(active-723.1750327887252)<.000001 and active+.132<900
m=json.loads(data['runs/manifest.json'])
for path,sha in m['native_provenance']['source_hashes'].items():assert hashlib.sha256((root/path).read_bytes()).hexdigest()==sha,path
assert not subprocess.check_output(['git','diff','d53f344','--','crates','Cargo.toml','Cargo.lock'],cwd=root).strip()
panel=json.loads((here/'panel-frozen.json').read_text());stable=json.loads((root/'docs/m5/chess-decision-benchmark.json').read_text())
# Stable public fixture only normalizes artifact source paths.
def strip_paths(x):
 if isinstance(x,dict):return {k:strip_paths(v) for k,v in x.items() if k not in ['raw_file','proof_file']}
 if isinstance(x,list):return [strip_paths(v) for v in x]
 return x
assert strip_paths(panel)==strip_paths(stable)
for name in ['chess-time-scaling.md','chess-decision-benchmark.md','strength-roadmap.md']:
 p=root/'docs/m5'/name
 for link in re.findall(r'\]\(([^)]+)\)',p.read_text()):
  if '://' not in link and not link.startswith('#'):assert (p.parent/link.split('#')[0]).exists(),(name,link)
print(json.dumps({'raw_artifacts':len(data),'ledger_attempts':ledger_count,'ledger_artifact_hashes':hash_count,'active_seconds':active,'production_unchanged':True,'benchmark_paths_only_normalized':True,'report_links_valid':True},indent=2))
