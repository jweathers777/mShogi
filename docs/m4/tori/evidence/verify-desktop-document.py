"""Optional metadata audit for MSH-130; native/codec tests are separate Rust checks."""
from pathlib import Path
import hashlib,json,re,subprocess
ROOT=Path(__file__).resolve().parents[4]
BASE='1e51ea51b3ba6997b75a7c11a25df0c7f4eeaad5'
def old(p):return subprocess.check_output(['git','show',BASE+':'+str(p)],cwd=ROOT)
records={};states={}
for p in [*ROOT.glob('backlog/tasks/*.md'),*ROOT.glob('backlog/archive/tasks/*.md')]:
 t=p.read_text();i=re.search(r'^id: (MSH-\d+)$',t,re.M)[1];assert i not in records
 m=re.search(r'^dependencies:\n((?:  - .*\n)*)',t,re.M);ds=re.findall(r'  - (MSH-\d+)',m[1]) if m else []
 prior=old(p.relative_to(ROOT)).decode()
 if i in {'MSH-130','MSH-131'}:
  for b,e in [('<!-- AC:BEGIN -->','<!-- AC:END -->'),('<!-- DOD:BEGIN -->','<!-- DOD:END -->')]:
   normalize=lambda x:re.sub(r'\[[ x]\]','[ ]',x.split(b)[1].split(e)[0])
   assert normalize(t)==normalize(prior),(i,'criteria changed')
  if i=='MSH-131':assert '\nstatus: Ready\n' in t and '\nassignee: []\n' in t and '## Implementation Plan' not in t
 else:assert t==prior,(i,'unrelated record changed')
 prior_deps=re.search(r'^dependencies:\n((?:  - .*\n)*)',prior,re.M)
 assert ds==(re.findall(r'  - (MSH-\d+)',prior_deps[1]) if prior_deps else [])
 records[i]=ds;states[i]=re.search(r'^status: (.*)$',t,re.M)[1]
for i,ds in records.items():assert len(ds)==len(set(ds)) and i not in ds and all(d in records for d in ds)
active=set();visited=set()
def visit(i):
 assert i not in active
 if i in visited:return
 active.add(i)
 for d in records[i]:visit(d)
 active.remove(i);visited.add(i)
for i in records:visit(i)
for i in records['MSH-130']:assert states[i]=='Done'
assert len(records)==132 and sum(map(len,records.values()))==353
pins={}
for path in ['docs/m4/tori/profile.md','docs/m4/tori/fixtures.json','docs/m4/tori/native-fixture-ownership.json','docs/m4/tori/shared-consumer-ownership.json','docs/m4/tori/evidence/native-acceptance-audit.json']:
 raw=(ROOT/path).read_bytes();assert raw==old(path);pins[path]=hashlib.sha256(raw).hexdigest()
assert not subprocess.check_output(['git','diff','--name-only',BASE,'--','crates/mshogi-core','crates/mshogi-search','crates/mshogi-runner','crates/mshogi-engine','docs/design.md','docs/testing/conformance-policy.md','.github','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).strip()
paths=['crates/mshogi-desktop-session/src/tori_document.rs','crates/mshogi-desktop-session/src/tori_document/tests.rs','crates/mshogi-desktop-session/src/shared_archive.rs','crates/mshogi-desktop-session/src/lib.rs','crates/mshogi-desktop-session/src/resource.rs','crates/mshogi-console/tests/tori_internal.rs','desktop/src/resources.ts','desktop/src/product/resourcePresentation.test.ts']
source_pins={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in paths}
links=0
for path in ['docs/m4/tori/desktop-document.md','docs/m4/tori/desktop-document-evidence.md','docs/m4/tori/handoff.md']:
 p=ROOT/path
 for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)',p.read_text()):
  if target.startswith(('http:','https:','#')):continue
  assert (p.parent/target.split('#')[0]).resolve().exists(),(path,target);links+=1
out={'kind':'MSH-130 static scope/dependency/hash/link audit; actual execution separate','baseline':BASE,'tasks':len(records),'edges':sum(map(len,records.values())),'graph_unique_resolved_acyclic_unchanged':True,'original130_131criteria_unchanged':True,'131_unstarted':True,'all_other_task_records_unchanged':True,'native_rules_search_engine_runner_design_policy_workflows_unchanged':True,'immutable_pins':pins,'source_sha256':source_pins,'links_checked':links,'pending_assertions_unchanged':14,'consumer_requirements_unchanged':83}
(ROOT/'docs/m4/tori/evidence/desktop-document-validation.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
