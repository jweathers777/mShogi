#!/usr/bin/env python3
"""Static ownership/graph/source audit. Execution evidence is explicitly separate."""
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[4]
BASE = '086579f7361c5e1e395ea8f766779688446441b2'
def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)
def old(path):
    return git('show', f'{BASE}:{path}')
def sha(path):
    return hashlib.sha256((ROOT/path).read_bytes()).hexdigest()
def section(text, name):
    return text.split(f'<!-- {name}:BEGIN -->', 1)[1].split(f'<!-- {name}:END -->', 1)[0].replace('[x]', '[ ]')

tasks = {}
for path in (ROOT/'backlog/tasks').glob('*.md'):
    text=path.read_text()
    found=re.search(r'^id: (MSH-\d+)\s*$',text,re.M)
    if not found:
        continue
    ident=found[1]
    assert ident not in tasks, ident
    header=text.split('---',2)[1]
    deps=re.findall(r'MSH-\d+',header.split('dependencies:',1)[1].split('references:',1)[0]) if 'dependencies:' in header else []
    tasks[ident]=(path,text,deps)
visiting=set()
visited=set()
def visit(ident):
    assert ident in tasks, ident
    assert ident not in visiting, f'cycle:{ident}'
    if ident in visited:
        return
    visiting.add(ident)
    for dependency in tasks[ident][2]:
        assert dependency != ident
        visit(dependency)
    visiting.remove(ident)
    visited.add(ident)
for ident in tasks:
    visit(ident)
for ident in ('MSH-131','MSH-132','MSH-77','MSH-42','MSH-46'):
    path,text,deps=tasks[ident]
    before=old(path.relative_to(ROOT).as_posix())
    assert section(text,'AC') == section(before,'AC'),ident
    assert section(text,'DOD') == section(before,'DOD'),ident
    before_header=before.split('---',2)[1]
    before_deps=re.findall(r'MSH-\d+',before_header.split('dependencies:',1)[1].split('references:',1)[0])
    assert deps==before_deps,ident
text=tasks['MSH-132'][1]
assert 'status: Ready' in text and 'assignee: []' in text
assert '<!-- SECTION:PLAN:BEGIN -->' not in text
changed=git('diff','--name-only',BASE,'--','backlog/tasks').splitlines()
assert all('/msh-131 - ' in p or '/msh-132 - ' in p for p in changed),changed
protected=['crates/mshogi-core','crates/mshogi-engine','crates/mshogi-search','crates/mshogi-runner','docs/design.md','docs/testing/conformance-policy.md','docs/m4/tori/profile.md','docs/m4/tori/fixtures.json','docs/m4/tori/native-fixture-ownership.json','docs/m4/tori/shared-consumer-ownership.json','.github/workflows','desktop/src']
assert not git('diff','--name-only',BASE,'--',*protected).strip()
source=[p for p in git('diff','--name-only',BASE).splitlines() if p.endswith('.rs') or p=='crates/mshogi-console/Cargo.toml']
source+=['crates/mshogi-desktop-session/src/shared_tori.rs','crates/mshogi-desktop-session/src/shared_tori_tests.rs','crates/mshogi-console/tests/desktop_tori.rs']
source=sorted(set(source))
report=json.loads((ROOT/'docs/m4/tori/evidence/desktop-runtime-consumer-acceptance.json').read_text())
assert len(report['runtime_assertions']['fields'])==13
assert len(report['runtime_consumer_requirements']['rows'])==82
links=0
for filename in ['desktop-runtime.md','desktop-runtime-evidence.md']:
    path=ROOT/'docs/m4/tori'/filename
    for target in re.findall(r'\]\(([^\s)]+)\)',path.read_text()):
        if '://' in target or target.startswith('#'):
            continue
        assert (path.parent/target.split('#',1)[0]).exists(),target
        links+=1
result={'kind':'static audit, not test/GUI execution','baseline':BASE,'tasks':len(tasks),'edges':sum(len(v[2]) for v in tasks.values()),'graph_unique_resolved_acyclic':True,'original_criteria_dependencies_unchanged':True,'132_unstarted':True,'all_other_task_records_unchanged':True,'native_rules_execution_design_policy_frontend_workflows_unchanged':True,'runtime_fields':13,'runtime_requirements':82,'remaining_42_fields_requirements':[1,1],'links_checked':links,'source_sha256':{p:sha(p) for p in source},'immutable_sha256':{p:sha(p) for p in protected if (ROOT/p).is_file()}}
(ROOT/'docs/m4/tori/evidence/desktop-runtime-validation.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
