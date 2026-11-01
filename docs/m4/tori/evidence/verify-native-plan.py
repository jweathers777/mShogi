"""MSH-85 static task/ownership planning checks; never native rule conformance."""
from pathlib import Path
import collections
import datetime
import hashlib
import json
import re
import subprocess

ROOT = Path(__file__).resolve().parents[4]
OUT = ROOT / 'docs/m4/tori'
BASE = '6893f1249dff0414cdbe81ce99fac3a3c0c0e7dc'

def old(path):
    return subprocess.check_output(['git', 'show', BASE + ':' + str(path)], cwd=ROOT)

def section(text, begin, end):
    return text.split(begin)[1].split(end)[0]

def deps(front):
    match = re.search(r'^dependencies:\n((?:  - .*\n)*)', front, re.M)
    return re.findall(r'  - (MSH-\d+)', match[1]) if match else []

corpus_path = OUT / 'fixtures.json'
assert corpus_path.read_bytes() == old('docs/m4/tori/fixtures.json')
assert (OUT / 'profile.md').read_bytes() == old('docs/m4/tori/profile.md')
corpus = json.loads(corpus_path.read_text())
fixture_rows = {r['id']: r for r in corpus['fixtures']}
manifest = json.loads((OUT / 'native-fixture-ownership.json').read_text())
entries = manifest['fixtures']
assert manifest['corpus_sha256'] == hashlib.sha256(corpus_path.read_bytes()).hexdigest()
assert manifest['profile'] == corpus['profile'] == 'mshogi.tori.modern.v1'
assert len(entries) == len(fixture_rows) == 103
assert {e['fixture_id'] for e in entries} == set(fixture_rows)
assert len({e['fixture_id'] for e in entries}) == 103
counts = collections.Counter()
for entry in entries:
    row = fixture_rows[entry['fixture_id']]
    assert entry['corpus_owner'] == 'MSH-126'
    assert entry['primary_native_owner'] in {'MSH-127', 'MSH-128', 'MSH-129'}
    assert entry['native_boundary']
    assert entry['expected_fields'] == list(row['expected'])
    counts[entry['primary_native_owner']] += 1
    for check in entry['supplementary_native_checks']:
        assert check['owner'] in {'MSH-127', 'MSH-128', 'MSH-129'} and check['scope']
    for check in entry['later_consumer_checks']:
        assert check['owner'] in {'MSH-77', 'MSH-42'} and check['scope']
identity = next(e for e in entries if e['fixture_id'] == 'TORI-C-IDENTITY')
assert any(c['owner'] == 'MSH-77' and 'process-generation' in c['scope'] for c in identity['later_consumer_checks'])
assert counts == {'MSH-127': 14, 'MSH-128': 69, 'MSH-129': 20}
records, texts, statuses = {}, {}, {}
for path in [*ROOT.glob('backlog/tasks/*.md'), *ROOT.glob('backlog/archive/tasks/*.md')]:
    text = path.read_text()
    front = text.split('---', 2)[1]
    match = re.search(r'^id: (MSH-\d+)$', front, re.M)
    if not match:
        continue
    ident = match[1]
    assert ident not in records
    records[ident] = deps(front)
    texts[ident] = (path, text)
    statuses[ident] = re.search(r'^status: (.+)$', front, re.M)[1]
for ident, requirements in records.items():
    assert len(set(requirements)) == len(requirements)
    assert ident not in requirements and all(d in records for d in requirements)
visiting, done = set(), set()
def visit(ident):
    assert ident not in visiting
    if ident in done:
        return
    visiting.add(ident)
    for dependency in records[ident]:
        visit(dependency)
    visiting.remove(ident)
    done.add(ident)
for ident in records:
    visit(ident)
expected_deps = {
    'MSH-126': ['MSH-81'], 'MSH-127': ['MSH-126'],
    'MSH-128': ['MSH-127'], 'MSH-129': ['MSH-128'],
    'MSH-85': ['MSH-81', 'MSH-126', 'MSH-127', 'MSH-128', 'MSH-129'],
    'MSH-77': ['MSH-69', 'MSH-72', 'MSH-73', 'MSH-85'],
    'MSH-42': ['MSH-39', 'MSH-72', 'MSH-73', 'MSH-77', 'MSH-81', 'MSH-85'],
}
for ident, expected in expected_deps.items():
    assert records[ident] == expected
for ident in ['MSH-42', 'MSH-77', 'MSH-85']:
    path, current = texts[ident]
    before = old(path.relative_to(ROOT)).decode()
    for begin, end in [('<!-- AC:BEGIN -->', '<!-- AC:END -->'), ('<!-- DOD:BEGIN -->', '<!-- DOD:END -->')]:
        assert section(before, begin, end).replace('[x]', '[ ]') == section(current, begin, end).replace('[x]', '[ ]')
    checked = re.findall(r'^- \[x\] #(\d+)', section(current, '<!-- AC:BEGIN -->', '<!-- AC:END -->'), re.M)
    assert checked in ([], ['4']) if ident == 'MSH-85' else checked == []
    assert '[x]' not in section(current, '<!-- DOD:BEGIN -->', '<!-- DOD:END -->')
assert statuses['MSH-85'] in {'In Progress', 'Ready'}
if statuses['MSH-85'] == 'Ready':
    assert '- [x] #4 ' in texts['MSH-85'][1]
for ident in ['MSH-126', 'MSH-127', 'MSH-128', 'MSH-129']:
    path, text = texts[ident]
    assert statuses[ident] == 'Ready' and 'assignee: []' in text
    assert 'milestone: m-4' in text and 'owner-msh-85' in text
    assert 'SECTION:PLAN:BEGIN' not in text and '[x]' not in text
    assert all(d not in {'MSH-85', 'MSH-77', 'MSH-42'} for d in records[ident])
for ident in ['MSH-81', 'MSH-82', 'MSH-83', 'MSH-45', 'MSH-51', 'MSH-61', 'MSH-121']:
    path, text = texts[ident]
    assert text.encode() == old(path.relative_to(ROOT))
ready = sorted(i for i in records if statuses[i] == 'Ready' and all(statuses[d] == 'Done' for d in records[i]))
assert ready == ['MSH-126'], ready
links = 0
for path in [*OUT.glob('*.md'), OUT / 'evidence/README.md']:
    for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)', path.read_text()):
        if target.startswith(('http', 'mailto:', '#')):
            continue
        assert (path.parent / target.split('#')[0]).exists(), (path, target)
        links += 1
# Check every local task documentation/reference target, preserving all old refs.
for ident in ['MSH-42', 'MSH-77', 'MSH-85', 'MSH-126', 'MSH-127', 'MSH-128', 'MSH-129']:
    text = texts[ident][1]
    for target in re.findall(r'^  - (docs/.*)$', text, re.M):
        assert (ROOT / target).exists(), (ident, target)
production = ['crates', 'desktop', 'src', 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', '.github/workflows', 'AGENTS.md']
assert not subprocess.check_output(['git', 'diff', '--name-only', BASE, '--', *production], cwd=ROOT, text=True).strip()
result = {
    'kind': 'MSH-85 static planning/task/fixture ownership checks; not implementation evidence',
    'baseline': BASE, 'profile': corpus['profile'], 'fixture_specs': 103,
    'fixture_and_profile_bytes_unchanged': True, 'corpus_sha256': manifest['corpus_sha256'],
    'corpus_owner': 'MSH-126', 'primary_native_owners': dict(counts),
    'all_expected_fields_owned': True, 'consumer_facets_explicit': True,
    'task_records': len(records), 'dependency_edges': sum(map(len, records.values())),
    'unique_resolved_acyclic_graph': True, 'original_criteria_preserved': True,
    'first_dependency_ready_task': ready, 'native_implementation_started': False,
    'production_build_workflow_instructions_diff': [], 'current_local_links': links,
    'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'result': 'passed',
}
(OUT / 'evidence/native-plan-validation.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
