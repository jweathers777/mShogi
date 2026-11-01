"""MSH-85 acceptance metadata audit; not a product/conformance dependency.

Run the real native adapter first (see native-acceptance.md), then pass its report:
  python3 docs/m4/tori/evidence/verify-native-acceptance.py /tmp/tori85-report.json
"""
from pathlib import Path
import collections
import hashlib
import json
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
BASE = '558402b4426a1d8faff3931f65643bcda95dd2fe'
PRODUCTION = '994521bec0637d6f8f17103f1106c0025d950753'

def previous(path):
    return subprocess.check_output(['git', 'show', BASE + ':' + str(path)], cwd=ROOT)

def task_id(text):
    return re.search(r'^id: (MSH-\d+)$', text, re.M)[1]

def dependencies(text):
    match = re.search(r'^dependencies:\n((?:  - .*\n)*)', text, re.M)
    return re.findall(r'  - (MSH-\d+)', match[1]) if match else []

report_path = Path(sys.argv[1])
report_bytes = report_path.read_bytes()
report = json.loads(report_bytes)
manifest_path = ROOT / 'docs/m4/tori/native-fixture-ownership.json'
manifest = json.loads(manifest_path.read_text())
assert (report['fixtures_validated'], report['executed'], report['passed'], report['pending'], report['failed']) == (103, 713, 713, 14, 0)
assert len(manifest['fixtures']) == 103
assert all(a['status'] == 'passed' for a in report['assertions'] if a['owner'] in {'MSH-127', 'MSH-128', 'MSH-129'})
pending = [a for a in report['assertions'] if a['status'] == 'pending']
assert all(a['owner'] in {'MSH-77', 'MSH-42'} for a in pending)
archived = json.loads((ROOT / 'docs/m4/tori/evidence/native-game-fixtures.json').read_text())
assert pending == archived['pending_assertions']
expected = [(f['fixture_id'], r['owner'], r['scope']) for f in manifest['fixtures'] for r in f['later_consumer_checks']]
actual = [(r['fixture_id'], r['owner'], r['scope']) for r in report['later_consumer_requirements']]
assert len(actual) == 83 and len(set(actual)) == 83
assert sorted(actual) == sorted(expected)
assert all(r['status'] == 'pending' for r in report['later_consumer_requirements'])
assert collections.Counter(a['owner'] for a in pending) == {'MSH-77': 13, 'MSH-42': 1}
assert collections.Counter(owner for _, owner, _ in actual) == {'MSH-77': 82, 'MSH-42': 1}

records = {}
for p in [*ROOT.glob('backlog/tasks/*.md'), *ROOT.glob('backlog/archive/tasks/*.md')]:
    text = p.read_text()
    identifier = task_id(text)
    assert identifier not in records
    deps = dependencies(text)
    prior = previous(p.relative_to(ROOT)).decode()
    assert deps == dependencies(prior), (identifier, 'dependencies changed')
    if identifier != 'MSH-85':
        assert text == prior, (identifier, 'unrelated/completed task changed')
    else:
        for begin, end in [('<!-- AC:BEGIN -->', '<!-- AC:END -->'), ('<!-- DOD:BEGIN -->', '<!-- DOD:END -->')]:
            normalize = lambda s: re.sub(r'\[[ x]\]', '[ ]', s.split(begin)[1].split(end)[0])
            assert normalize(text) == normalize(prior), 'acceptance wording changed'
    records[identifier] = deps
for identifier, deps in records.items():
    assert len(deps) == len(set(deps)) and identifier not in deps and all(d in records for d in deps)
active, completed = set(), set()
def visit(identifier):
    assert identifier not in active, identifier
    if identifier in completed:
        return
    active.add(identifier)
    for dep in records[identifier]:
        visit(dep)
    active.remove(identifier)
    completed.add(identifier)
for identifier in records:
    visit(identifier)
assert records['MSH-85'] == ['MSH-81', 'MSH-126', 'MSH-127', 'MSH-128', 'MSH-129']
for dep in records['MSH-85']:
    paths = [p for p in ROOT.glob('backlog/tasks/*.md') if p.read_text().startswith('---\nid: ' + dep + '\n')]
    assert len(paths) == 1 and '\nstatus: Done\n' in paths[0].read_text()

pins = {}
for name in ['profile.md', 'fixtures.json', 'native-fixture-ownership.json']:
    p = Path('docs/m4/tori') / name
    raw = (ROOT / p).read_bytes()
    assert raw == previous(p)
    pins[str(p)] = hashlib.sha256(raw).hexdigest()
production_paths = ['crates', 'desktop', '.github', 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'AGENTS.md', 'docs/design.md', 'docs/testing/conformance-policy.md']
assert not subprocess.check_output(['git', 'diff', '--name-only', PRODUCTION, '--', *production_paths], cwd=ROOT, text=True).strip()
links = 0
for name in ['native-acceptance.md', 'handoff.md', 'native-plan.md', 'README.md']:
    p = ROOT / 'docs/m4/tori' / name
    for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)', p.read_text()):
        if target.startswith(('http', '#')):
            continue
        resolved = (p.parent / target.split('#')[0]).resolve()
        assert resolved.exists() or resolved == ROOT / 'docs/m4/tori/evidence/native-acceptance-audit.json', (p, target)
        links += 1

out = {
    'kind': 'MSH-85 exhaustive metadata/ownership acceptance; native execution remains the separate report',
    'baseline': BASE, 'tested_production': PRODUCTION,
    'live_report_sha256': hashlib.sha256(report_bytes).hexdigest(),
    'counts': {k: report[k] for k in ['fixtures_validated', 'executed', 'passed', 'pending', 'failed', 'documentary_annotations']},
    'native_pending': 0,
    'pending_assertions': pending,
    'later_consumer_requirements': [dict(fixture_id=f, owner=o, scope=s, native_handoff_verdict='available; actual consumer verification remains pending') for f, o, s in actual],
    'consumer_groups': [dict(owner=o, scope=s, count=count) for (o, s), count in collections.Counter((o, s) for _, o, s in actual).items()],
    'task_records': len(records), 'dependency_edges': sum(map(len, records.values())),
    'dependencies_unique_resolved_acyclic_and_unchanged': True,
    'all_other_task_records_and_original85criteria_unchanged': True,
    'production_unchanged_from_tested_revision': True,
    'immutable_profile_corpus_manifest_sha256': pins, 'links_checked': links,
}
path = ROOT / 'docs/m4/tori/evidence/native-acceptance-audit.json'
path.write_text(json.dumps(out, indent=2) + '\n')
print(json.dumps({k: v for k, v in out.items() if k not in ['pending_assertions', 'later_consumer_requirements']}))
