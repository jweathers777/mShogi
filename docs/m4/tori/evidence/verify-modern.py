"""Static planning evidence, not a Tori legality implementation or conformance run."""
from pathlib import Path
import datetime
import hashlib
import json
import re
import subprocess

ROOT = Path(__file__).resolve().parents[4]
OUT = ROOT / 'docs/m4/tori'
BASE = '26a2fa9ce8165b6e7ae713d12f16548b65c110da'

def old(path):
    return subprocess.check_output(['git', 'show', BASE + ':' + str(path)], cwd=ROOT)

f = json.loads((OUT / 'fixtures.json').read_text())
assert f['profile'] == 'mshogi.tori.modern.v1'
assert f['native_conformance_executed'] is False
rows = f['fixtures']
by_id = {r['id']: r for r in rows}
assert len(by_id) == len(rows)
archived = json.loads((OUT / 'archive/2026-10-31/fixtures.json').read_text())
assert len(archived['fixtures']) == 39
assert set(f['preserved_seed_ids']) == {r['id'] for r in archived['fixtures']}
assert set(f['preserved_seed_ids']) <= set(by_id)
assert all(r['expected'] is not None and r['expectation_method'] for r in rows)
for r in rows:
    assert r['clauses'] and all(re.fullmatch(r'T0[1-8]', c) for c in r['clauses'])
    assert r['sources'] and set(r['sources']) <= set(f['source_ids'])
    if r['scope'].startswith('native_admission') and r['expected']['accepted']:
        assert r['expected']['reversal_contract']['retired_execution'] == 'not_revived'
    root = r['given'].get('root')
    if root and r['scope'] != 'root_admission':
        positions = root['board']
        assert all(re.fullmatch(r'[sg]:(Ph|Fa|Ea|Cr|Pt|LQ|RQ|Sw|Go):[1-7][a-g]', p) for p in positions)
        assert len({p.split(':')[2] for p in positions}) == len(positions)
        assert sum(p.startswith('s:Ph:') for p in positions) == 1
        assert sum(p.startswith('g:Ph:') for p in positions) == 1
        assert root['provenance']['historical_reachability'] == 'not_asserted'
        for hand in root['hands'].values():
            assert set(hand) <= {'Fa', 'Cr', 'Pt', 'LQ', 'RQ', 'Sw'}
            assert all(isinstance(n, int) and n >= 0 for n in hand.values())
    if 'rotation_of' in r:
        assert r['rotation_of'] in by_id
        b = by_id[r['rotation_of']]
        assert r['expected']['accepted'] == b['expected']['accepted']
assert len(by_id['TORI-I01']['expected']['board']) == 32
assert sum(r['scope'] == 'exhaustive_local_geometry' for r in rows) == 9
assert by_id['TORI-H-FORCED-FULL']['expected']['conclusion']['kind'] == 'repetition_forfeit'
assert len(by_id['TORI-H-FORCED-FULL']['given']['actions']) == 8
assert len(by_id['TORI-H-UNEQUAL-FULL']['given']['actions']) == 16
# Exact historical checkpoint bytes; never edit older evidence into the new verdict.
for name in ['README.md', 'profile.md', 'fixtures.json', 'handoff.md', 'review.md', 'verification.md']:
    assert (OUT / 'archive/2026-10-31' / name).read_bytes() == old('docs/m4/tori/' + name)
assert (OUT / 'archive/2026-10-31/validation.json').read_bytes() == old('docs/m4/tori/evidence/validation.json')
records = {}
texts = {}
for p in [*ROOT.glob('backlog/tasks/*.md'), *ROOT.glob('backlog/archive/tasks/*.md')]:
    text = p.read_text()
    front = text.split('---', 2)[1]
    match = re.search(r'^id: (MSH-\d+)$', front, re.M)
    if not match:
        continue
    ident = match[1]
    assert ident not in records
    deps = re.search(r'^dependencies:\n((?:  - .*\n)*)', front, re.M)
    records[ident] = re.findall(r'  - (MSH-\d+)', deps[1]) if deps else []
    texts[ident] = (p, text)
for ident, deps in records.items():
    assert len(set(deps)) == len(deps) and ident not in deps and all(d in records for d in deps)
visiting, done = set(), set()
def walk(ident):
    assert ident not in visiting
    if ident in done:
        return
    visiting.add(ident)
    for dep in records[ident]:
        walk(dep)
    visiting.remove(ident)
    done.add(ident)
for ident in records:
    walk(ident)
for ident in ['MSH-42', 'MSH-77', 'MSH-81', 'MSH-85']:
    p, current = texts[ident]
    before = old(p.relative_to(ROOT)).decode()
    for begin, end in [('<!-- AC:BEGIN -->', '<!-- AC:END -->'), ('<!-- DOD:BEGIN -->', '<!-- DOD:END -->')]:
        left = before.split(begin)[1].split(end)[0]
        right = current.split(begin)[1].split(end)[0]
        if ident == 'MSH-81':
            left, right = left.replace('[x]', '[ ]'), right.replace('[x]', '[ ]')
        assert left == right
    front_before = before.split('---', 2)[1]
    front_current = current.split('---', 2)[1]
    pattern = r'^dependencies:\n((?:  - .*\n)*)'
    assert re.search(pattern, front_before, re.M)[1] == re.search(pattern, front_current, re.M)[1]
for ident in ['MSH-62', 'MSH-121', 'MSH-45', 'MSH-82', 'MSH-83']:
    p, current = texts[ident]
    assert current.encode() == old(p.relative_to(ROOT))
links = 0
for p in [*OUT.glob('*.md'), OUT / 'evidence/README.md']:
    for target in re.findall(r'\[[^\]]*\]\(([^)]+)\)', p.read_text()):
        if target.startswith(('http', 'mailto:', '#')):
            continue
        assert (p.parent / target.split('#')[0]).exists(), (p, target)
        links += 1
production = ['crates', 'desktop', 'src', 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', '.github/workflows', 'AGENTS.md']
assert not subprocess.check_output(['git', 'diff', '--name-only', BASE, '--', *production], cwd=ROOT, text=True).strip()
result = {
    'kind': 'static profile/fixture/document/task integrity; not native conformance',
    'baseline': BASE, 'profile': f['profile'], 'fixtures': len(rows),
    'preserved_original_seed_ids': 39, 'original_checkpoint_bytes_preserved': True,
    'rotated_native_admission_cases': sum('rotation_of' in r for r in rows),
    'exhaustive_geometry_tables': 9, 'native_conformance_executed': False,
    'fixture_sha256': hashlib.sha256((OUT / 'fixtures.json').read_bytes()).hexdigest(),
    'task_records': len(records), 'dependency_edges': sum(map(len, records.values())),
    'unique_resolved_acyclic_graph': True, 'original_criteria_and_dependencies_preserved': True,
    'completed62_121_45_and_inactive82_83_byte_unchanged': True,
    'current_local_markdown_links': links, 'production_build_workflow_instructions_diff': [],
    'at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'result': 'passed'
}
(OUT / 'evidence/modern-validation.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
