from pathlib import Path
import json,gzip,hashlib,re,subprocess
e=Path(__file__).resolve().parent;root=e.parents[3]
manifest=json.loads((e/'artifact-manifest.json').read_text())
for name,m in manifest.items():
 b=(e/name).read_bytes();assert hashlib.sha256(b).hexdigest()==m['sha256'],name
 raw=gzip.decompress(b) if name.endswith('.gz') else b
 assert hashlib.sha256(raw).hexdigest()==m['original_sha256'],name
 if m['original_name'].endswith('.json'):json.loads(raw)
 elif m['original_name'].endswith('.jsonl'):
  for l in raw.splitlines():json.loads(l)
p=json.loads((e/'architecture/provenance.json').read_text())
for name,h in p['source_hashes'].items():assert hashlib.sha256((root/name).read_bytes()).hexdigest()==h,name
assert not subprocess.check_output(['git','diff','d53f344','--','crates','Cargo.toml','Cargo.lock'],cwd=root).strip()
rows=[]
for f in (e/'architecture').glob('*.out.jsonl*'):
 b=f.read_bytes();b=gzip.decompress(b) if f.suffix=='.gz' else b
 for l in b.splitlines():
  x=json.loads(l);assert x['root_unchanged'] and x['pv_native_admitted'],f;rows.append(x)
for doc in ['docs/m5/chess-adversarial-assessment.md','docs/m5/strength-roadmap.md']:
 path=root/doc
 for link in re.findall(r'\]\(([^)]+)\)',path.read_text()):
  if '://' not in link and not link.startswith('#'):assert (path.parent/link.split('#')[0]).exists(),(doc,link)
print(json.dumps({'artifacts_verified':len(manifest),'source_hashes_verified':len(p['source_hashes']),'native_search_rows_restored_and_pv_admitted':len(rows),'production_diff_empty':True,'new_report_and_roadmap_links_valid':True},indent=2))
