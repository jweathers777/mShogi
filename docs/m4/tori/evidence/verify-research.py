from pathlib import Path
import datetime,hashlib,json,re,subprocess
root=Path(__file__).resolve().parents[4];out=root/'docs/m4/tori';baseline='6a9d24dbff8e883fdc23dabf74216b91cc194ac0';result={'baseline':baseline,'kind':'research artifact integrity; not rule conformance or product tests'}
f=json.loads((out/'fixtures.json').read_text());rows=f['fixtures'];assert len({r['id'] for r in rows})==len(rows)
for r in rows:
 assert r['sources'] and all(s['id'] in f['source_ids'] and s['locator'] for s in r['sources'])
 if r['status']=='blocked':assert r['expected'] is None and r['gate'] in ['P01','P02','P03','P04','P05','P06']
 else:assert r['expected'] is not None and r['gate'] is None
assert f['profile_status']=='unratified' and f['executable_conformance'] is False
result.update(fixtures=len(rows),supported_partial_or_contract=sum(r['status']!='blocked' for r in rows),blocked=sum(r['status']=='blocked' for r in rows),fixture_sha256=hashlib.sha256((out/'fixtures.json').read_bytes()).hexdigest())
hashes=json.loads((out/'evidence/historical-images.json').read_text())
for h in hashes:
 p=Path(h['file']);assert p.stat().st_size==h['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==h['sha256']
for i in [4,7,9]:assert hashlib.sha256((out/f'evidence/tohoku-image{i:02}.jpg').read_bytes()).hexdigest()==hashes[i-1]['sha256']
for h in json.loads((out/'evidence/kanare-manifest.json').read_text()):
 p=Path('/private/tmp/msh81-root')/(h['id']+'.pdf');assert p.stat().st_size==h['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==h['sha256']
result.update(historical_image_hashes=15,retained_original_images=3,kanare_original_pdf_hashes=3)
records={}; paths={}
for p in [*root.glob('backlog/tasks/*.md'),*root.glob('backlog/archive/tasks/*.md')]:
 s=p.read_text();front=s.split('---',2)[1];m=re.search(r'^id: (MSH-\d+)$',front,re.M)
 if not m:continue
 ident=m.group(1);assert ident not in records;deps=re.search(r'^dependencies:\n((?:  - .*\n)*)',front,re.M)
 records[ident]=re.findall(r'  - (MSH-\d+)',deps.group(1)) if deps else [];paths[ident]=p
for ident,deps in records.items():assert len(set(deps))==len(deps) and ident not in deps and all(d in records for d in deps)
visiting=set();done=set()
def walk(i):
 assert i not in visiting
 if i in done:return
 visiting.add(i)
 for d in records[i]:walk(d)
 visiting.remove(i);done.add(i)
for i in records:walk(i)
assert records['MSH-81']==['MSH-62'] and records['MSH-85']==['MSH-81'] and records['MSH-77']==['MSH-69','MSH-72','MSH-73','MSH-85']
for i in ['MSH-42','MSH-77','MSH-81','MSH-85']:
 p=paths[i];old=subprocess.check_output(['git','show',baseline+':'+str(p.relative_to(root))],cwd=root,text=True);new=p.read_text()
 for a,b in [('<!-- AC:BEGIN -->','<!-- AC:END -->'),('<!-- DOD:BEGIN -->','<!-- DOD:END -->')]:assert old.split(a)[1].split(b)[0]==new.split(a)[1].split(b)[0]
for i in ['MSH-62','MSH-121']:
 p=paths[i];assert p.read_bytes()==subprocess.check_output(['git','show',baseline+':'+str(p.relative_to(root))],cwd=root)
result.update(task_records=len(records),dependency_edges=sum(map(len,records.values())),unique_resolved_acyclic_graph=True,protected_criteria_and_dod_unchanged=True,completed62_and121_byte_unchanged=True)
links=0
for p in [*out.glob('*.md'),*out.glob('evidence/*.md')]:
 for x in re.findall(r'\[[^\]]*\]\(([^)]+)\)',p.read_text()):
  if x.startswith(('http','mailto:','#')):continue
  target=x.split('#')[0]
  assert (p.parent/target).exists(),(p,x);links+=1
result['local_markdown_links']=links
codepaths=['crates','desktop','src','Cargo.toml','Cargo.lock','rust-toolchain.toml','.github/workflows','AGENTS.md']
assert not subprocess.check_output(['git','diff','--name-only',baseline,'--',*codepaths],cwd=root,text=True).strip()
result['product_build_workflow_instructions_diff']=[];result['at']=datetime.datetime.now(datetime.timezone.utc).isoformat();result['result']='passed'
(out/'evidence/validation.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
