"""Optional standard-library planning audit; no native or product test claim."""
from pathlib import Path
import collections
import hashlib
import json
import re
import subprocess

ROOT = Path(__file__).resolve().parents[4]
BASE = 'a3677e1e862fdf3877c784f12f56a99c4bfbf167'
def old(path):
    return subprocess.check_output(['git','show',BASE+':'+str(path)],cwd=ROOT)
def deps(text):
    m=re.search(r'^dependencies:\n((?:  - .*\n)*)',text,re.M)
    return re.findall(r'  - (MSH-\d+)',m[1]) if m else []
records={};texts={}
for p in [*ROOT.glob('backlog/tasks/*.md'),*ROOT.glob('backlog/archive/tasks/*.md')]:
    t=p.read_text();id=re.search(r'^id: (MSH-\d+)$',t,re.M)[1]
    assert id not in records,id
    records[id]=deps(t);texts[id]=t
    if id in {'MSH-130','MSH-131','MSH-132'}:
        assert '\nstatus: Ready\n' in t and '\nassignee: []\n' in t
        assert '## Implementation Plan' not in t
        assert not re.search(r'^- \[x\]',t,re.M)
        assert '\nmilestone: m-4\n' in t and 'owner-msh-77' in t
    else:
        prior=old(p.relative_to(ROOT)).decode()
        if id=='MSH-77':
            assert '\nstatus: Ready\n' in t and '\nassignee: []\n' in t
            for b,e in [('<!-- AC:BEGIN -->','<!-- AC:END -->'),('<!-- DOD:BEGIN -->','<!-- DOD:END -->')]:
                assert t.split(b)[1].split(e)[0]==prior.split(b)[1].split(e)[0],(id,'original criteria changed')
            assert not re.search(r'^- \[x\]',t,re.M)
        else: assert t==prior,(id,'unrelated task changed')
for id,ds in records.items():
    assert len(ds)==len(set(ds)) and id not in ds and all(d in records for d in ds),id
visiting=set();visited=set()
def visit(id):
    assert id not in visiting,('cycle',id)
    if id in visited:return
    visiting.add(id)
    for d in records[id]:visit(d)
    visiting.remove(id);visited.add(id)
for id in records:visit(id)
assert len(records)==132
expected={'MSH-77':['MSH-69','MSH-72','MSH-73','MSH-85','MSH-130','MSH-131','MSH-132'],
'MSH-130':['MSH-69','MSH-72','MSH-73','MSH-85'],'MSH-131':['MSH-130'],'MSH-132':['MSH-131']}
for id,ds in expected.items():assert records[id]==ds,(id,records[id])
for d in records['MSH-130']:assert '\nstatus: Done\n' in texts[d],d
for child in ['MSH-130','MSH-131','MSH-132']:assert not set(records[child])&{'MSH-77','MSH-42','MSH-46'}
manifest=json.loads((ROOT/'docs/m4/tori/native-fixture-ownership.json').read_text())
accepted=json.loads((ROOT/'docs/m4/tori/evidence/native-acceptance-audit.json').read_text())
mapping=json.loads((ROOT/'docs/m4/tori/shared-consumer-ownership.json').read_text())
fixtures={f['id']:f for f in json.loads((ROOT/'docs/m4/tori/fixtures.json').read_text())['fixtures']}
assert len(fixtures)==103
pending=mapping['pending_assertions'];later=mapping['later_consumer_requirements']
assert len(pending)==14 and len(later)==83
assert [{k:r[k] for k in accepted['pending_assertions'][0]} for r in pending]==accepted['pending_assertions']
triples=lambda rows:[(r['fixture_id'],r['owner'],r['scope']) for r in rows]
assert triples(later)==triples(accepted['later_consumer_requirements'])
expected_manifest=[(f['fixture_id'],r['owner'],r['scope']) for f in manifest['fixtures'] for r in f['later_consumer_checks']]
assert sorted(triples(later))==sorted(expected_manifest) and len(set(triples(later)))==83
assert len({(r['fixture_id'],r['field']) for r in pending})==14
for r in pending:
    assert r['independent_expected_value']==fixtures[r['fixture_id']]['expected'][r['field']]
for r in pending+later:
    assert r['status']=='pending' and r['fixture_id'] in fixtures
    assert r['implementation_owner']==('MSH-131' if r['owner']=='MSH-77' else 'MSH-42')
    assert r['executable_acceptance'] and r['expected_source']
    assert all(x in {'MSH-130','MSH-132'} for x in r['supporting_components'])
assert collections.Counter(r['implementation_owner'] for r in pending)=={'MSH-131':13,'MSH-42':1}
assert collections.Counter(r['implementation_owner'] for r in later)=={'MSH-131':82,'MSH-42':1}
pins={}
for f in ['profile.md','fixtures.json','native-fixture-ownership.json','evidence/native-acceptance-audit.json']:
    p=Path('docs/m4/tori')/f;raw=(ROOT/p).read_bytes();assert raw==old(p)
    pins[str(p)]=hashlib.sha256(raw).hexdigest()
production=['crates','desktop','.github','Cargo.toml','Cargo.lock','AGENTS.md','docs/design.md','docs/testing/conformance-policy.md']
assert not subprocess.check_output(['git','diff','--name-only',BASE,'--',*production],cwd=ROOT,text=True).strip()
links=0
for path in ['docs/m4/tori/shared-plan.md','docs/m4/tori/shared-plan-review.md','docs/m4/tori/handoff.md','docs/m4/delivery-plan.md','docs/m4/implementation-map.md']:
    p=ROOT/path
    for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)',p.read_text()):
        if target.startswith(('https:','http:','#')):continue
        assert (p.parent/target.split('#')[0]).resolve().exists(),(path,target)
        links+=1
# Exact source references in the new current plan must resolve; symbols are reviewed separately.
for path in ['crates/mshogi-core/src/native.rs','crates/mshogi-desktop-session/src/document.rs','crates/mshogi-desktop-session/src/persistence.rs','crates/mshogi-desktop-session/src/lib.rs','desktop/src-tauri/src/main.rs','desktop/src/product/useGameSession.ts','desktop/src/interaction/useNativeChoiceController.ts']:
    assert (ROOT/path).is_file(),path
out={'kind':'planning-only metadata/ownership validation, not executed consumer conformance','baseline':BASE,'tasks':len(records),'edges':sum(map(len,records.values())),'unique_resolved_acyclic':True,'original77criteria_and_other129_records_except77_unchanged':True,'complete_dependencies':expected,'only_new_dependency_ready':'MSH-130','all_children_ready_unassigned_unchecked_no_plans':True,'pending_assertions':14,'consumer_requirements':83,'native_assertions_reused_not_rerun':713,'pending_native':0,'native_profile_corpus_manifest_acceptance_pins':pins,'production_unchanged':True,'links_checked':links}
(ROOT/'docs/m4/tori/evidence/shared-plan-validation.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out))
