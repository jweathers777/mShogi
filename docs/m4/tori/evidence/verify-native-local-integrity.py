"""MSH-128 metadata/source-boundary integrity, not rules-conformance or a runtime dependency."""
from pathlib import Path
import subprocess,re,json
r=Path(__file__).resolve().parents[4];baseline='4710a4d9c1c188293fc89870cef68b2d0232e6c7'
def old(p):return subprocess.check_output(['git','show',baseline+':'+str(p)],text=True)
records={}
for p in [*r.glob('backlog/tasks/*.md'),*r.glob('backlog/archive/tasks/*.md')]:
 s=p.read_text();front=s.split('---',2)[1];i=re.search(r'^id: (MSH-\d+)$',front,re.M)[1];assert i not in records
 d=re.search(r'^dependencies:\n((?:  - .*\n)*)',front,re.M);deps=re.findall(r'  - (MSH-\d+)',d[1]) if d else []
 before=old(p.relative_to(r));bf=before.split('---',2)[1];bd=re.search(r'^dependencies:\n((?:  - .*\n)*)',bf,re.M)
 assert deps==(re.findall(r'  - (MSH-\d+)',bd[1]) if bd else [])
 if 'status: Done\n' in before:assert s==before,(i,'completed task changed')
 if i!='MSH-128':
  begin,end='<!-- SECTION:PLAN:BEGIN -->','<!-- SECTION:PLAN:END -->'
  if begin in before:assert s.split(begin)[1].split(end)[0]==before.split(begin)[1].split(end)[0]
 records[i]=deps
for i,deps in records.items():assert len(deps)==len(set(deps)) and i not in deps and all(d in records for d in deps)
active=set();done=set()
def visit(i):
 assert i not in active
 if i in done:return
 active.add(i)
 for d in records[i]:visit(d)
 active.remove(i);done.add(i)
for i in records:visit(i)
for f in ['profile.md','fixtures.json','native-fixture-ownership.json']:
 p=Path('docs/m4/tori')/f;assert p.read_bytes()==subprocess.check_output(['git','show',baseline+':'+str(p)])
paths=['.github/workflows','desktop','Cargo.toml','Cargo.lock','crates/mshogi-core/Cargo.toml','crates/mshogi-core/src/chess','crates/mshogi-core/src/shogi','crates/mshogi-core/src/xiangqi','crates/mshogi-core/src/native.rs','crates/mshogi-engine','crates/mshogi-search','crates/mshogi-runner','crates/mshogi-desktop-session','crates/mshogi-console','crates/mshogi-conformance']
assert not subprocess.check_output(['git','diff','--name-only',baseline,'--',*paths],text=True).strip()
p=Path('crates/mshogi-core/src/tori/transition.rs');current=p.read_text();prior=old(p)
assert current.split('impl Position {')[1]==prior.split('impl Position {')[1]
assert current.split('pub(crate) fn movement')[1].split('/// Structural failure')[0]==prior.split('pub(crate) fn movement')[1].split('/// Structural failure')[0]
p=Path('crates/mshogi-core/src/tori/tests.rs');assert p.read_text()==old(p)
p=Path('crates/mshogi-core/src/tori/conformance.rs');assert p.read_text().replace('pub(super) struct StructuralAdapter','struct StructuralAdapter').replace('pub(super) fn transition_field','fn transition_field')==old(p)
links=0
for p in [Path('docs/m4/tori/native-local-evidence.md'),Path('docs/m4/tori/handoff.md'),Path('docs/m4/tori/native-plan.md'),Path('docs/m4/tori/README.md')]:
 for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)',p.read_text()):
  if target.startswith(('http','#')):continue
  assert (p.parent/target.split('#')[0]).exists(),(p,target)
  links+=1
out={'baseline':baseline,'task_records':len(records),'dependency_edges':sum(map(len,records.values())),'unique_resolved_acyclic':True,'all_dependencies_unchanged':True,'all_previously_completed_tasks_unchanged':True,'other_implementation_plans_unchanged':True,'profile_corpus_manifest_bytes_unchanged':True,'existing_games_consumers_workflows_dependencies_unchanged':True,'structural_geometry_transition_semantics_and_tests_unchanged':True,'prior_structural_adapter_only_visibility_changes':True,'local_links_checked':links,'kind':'metadata integrity, not native conformance'}
Path('docs/m4/tori/evidence/native-local-integrity.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
