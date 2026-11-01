from pathlib import Path
import subprocess,re,json
r=Path(__file__).resolve().parents[4];base='04225043545a7219fbbad6024eb2398c5f68aaf2'
def old(p):return subprocess.check_output(['git','show',base+':'+str(p)],text=True)
records={};texts={};statuses={}
for p in [*r.glob('backlog/tasks/*.md'),*r.glob('backlog/archive/tasks/*.md')]:
 s=p.read_text();front=s.split('---',2)[1];m=re.search(r'^id: (MSH-\d+)$',front,re.M)
 if not m:continue
 i=m[1];assert i not in records;d=re.search(r'^dependencies:\n((?:  - .*\n)*)',front,re.M)
 records[i]=re.findall(r'  - (MSH-\d+)',d[1]) if d else [];texts[i]=(p,s);statuses[i]=re.search(r'^status: (.+)$',front,re.M)[1]
 for a,b in [('<!-- SECTION:PLAN:BEGIN -->','<!-- SECTION:PLAN:END -->')]:
  if i in ['MSH-127','MSH-128','MSH-129','MSH-85','MSH-82','MSH-83','MSH-86','MSH-87','MSH-43','MSH-44']:
   prior=old(p.relative_to(r))
   if a in prior:assert s.split(a)[1].split(b)[0]==prior.split(a)[1].split(b)[0]
 before=old(p.relative_to(r))
 if 'status: Done\n' in before:assert s==before,(i,'completed task changed')
 priorfront=before.split('---',2)[1];dd=re.search(r'^dependencies:\n((?:  - .*\n)*)',priorfront,re.M)
 assert records[i]==(re.findall(r'  - (MSH-\d+)',dd[1]) if dd else [])
for i,d in records.items():assert len(d)==len(set(d)) and i not in d and all(x in records for x in d)
a=set();done=set()
def visit(i):
 assert i not in a
 if i in done:return
 a.add(i)
 for d in records[i]:visit(d)
 a.remove(i);done.add(i)
for i in records:visit(i)
for f in ['profile.md','fixtures.json','native-fixture-ownership.json']:
 p=Path('docs/m4/tori')/f;assert p.read_text()==old(p)
links=0
for p in [Path('docs/testing/conformance-policy.md'),*Path('docs/m4/tori').glob('*.md')]:
 for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)',p.read_text()):
  if target.startswith(('http','mailto:','#')):continue
  assert (p.parent/target.split('#')[0]).exists(),(p,target)
  links+=1
assert not subprocess.check_output(['git','diff','--name-only',base,'--','.github/workflows','desktop','crates/mshogi-core','crates/mshogi-engine','crates/mshogi-search','crates/mshogi-runner','crates/mshogi-desktop-session','crates/mshogi-console'],text=True).strip()
out={'baseline':base,'task_records':len(records),'dependency_edges':sum(map(len,records.values())),'graph_unique_resolved_acyclic':True,'all_dependencies_unchanged':True,'completed_tasks_unchanged':True,'future_implementation_plans_unchanged':True,'profile_corpus_ownership_unchanged':True,'local_document_links':links,'existing_product_and_workflows_unchanged':True,'kind':'metadata integrity, not native conformance'}
Path('docs/m4/tori/evidence/harness-integrity.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
