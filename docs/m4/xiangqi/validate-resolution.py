#!/usr/bin/env python3
"""Read-only planning checks; no game, reference engine or product is executed."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--phase', choices=['review', 'closed'], default='review')
parser.add_argument('--research-root', type=Path)
parser.add_argument('--output', type=Path)
args = parser.parse_args()
BASELINE = 'a2cf50bdfa7de6b501e668cfc4bccb4b60ec7a80'
tasks = {}


def parse(text):
    fm = text.split('---', 2)[1]
    section = re.search(r'^dependencies:\n((?:  - .+\n)*)', fm, re.M)
    return {
        'id': re.search(r'^id: (MSH-\d+)$', fm, re.M)[1],
        'status': re.search(r'^status: (.+)$', fm, re.M)[1],
        'deps': re.findall(r'MSH-\d+', section[1]) if section else [],
        'text': text,
    }


def items(text, marker):
    body = text.split(f'<!-- {marker}:BEGIN -->')[1].split(f'<!-- {marker}:END -->')[0]
    return re.findall(r'^- \[[ x]\] #\d+ (.+)$', body, re.M)


for path in Path('backlog/tasks').glob('*.md'):
    task = parse(path.read_text())
    assert task['id'] not in tasks
    tasks[task['id']] = dict(task, path=path)
assert len(tasks) == 105
seen, active = set(), set()


def visit(task_id):
    assert task_id in tasks, ('missing dependency', task_id)
    assert task_id not in active, ('cycle', task_id)
    if task_id in seen:
        return
    active.add(task_id)
    for dependency in tasks[task_id]['deps']:
        visit(dependency)
    active.remove(task_id)
    seen.add(task_id)


for task_id in tasks:
    visit(task_id)
protected = 0
for task in tasks.values():
    old = subprocess.run(['git', 'show', f"{BASELINE}:{task['path'].as_posix()}"],
                         text=True, capture_output=True)
    if old.returncode:
        assert task['id'] in ['MSH-104', 'MSH-105']
        continue
    baseline = parse(old.stdout)
    assert set(baseline['deps']) <= set(task['deps']), task['id']
    assert items(old.stdout, 'DOD') == items(task['text'], 'DOD'), task['id']
    original = items(old.stdout, 'AC')
    # AC1 is explicitly reconciled with the owner's scoped alternative-source instruction.
    if task['id'] == 'MSH-80':
        original = original[1:]
    assert all(value in items(task['text'], 'AC') for value in original), task['id']
    if baseline['status'] == 'Done':
        assert task['text'] == old.stdout, ('changed completed evidence', task['id'])
        protected += 1
assert all(tasks[f'MSH-{number}']['status'] != 'Done' for number in range(42, 48))
for task_id in ['MSH-84', 'MSH-103', 'MSH-45']:
    assert 'MSH-105' in tasks[task_id]['deps']
assert tasks['MSH-105']['deps'] == ['MSH-94', 'MSH-95']
assert tasks['MSH-104']['deps'] == ['MSH-48']
assert 'MSH-104' in tasks['MSH-56']['deps']
assert 'MSH-59' not in tasks['MSH-56']['deps']
ancestors = set()


def collect(task_id):
    for dependency in tasks[task_id]['deps']:
        if dependency not in ancestors:
            ancestors.add(dependency)
            collect(dependency)


collect('MSH-45')
assert not ancestors.intersection({'MSH-42', 'MSH-43', 'MSH-44', 'MSH-46', 'MSH-47',
                                   'MSH-81', 'MSH-82', 'MSH-83'})
for task_id in ['MSH-91', 'MSH-92']:
    assert tasks[task_id]['deps'] == ['MSH-80']
    assert tasks[task_id]['status'] == ('Ready' if args.phase == 'closed' else 'To Do')
assert tasks['MSH-80']['status'] == ('Done' if args.phase == 'closed' else 'In Review')
for number in list(range(93, 104)) + [104, 105]:
    assert tasks[f'MSH-{number}']['status'] == 'To Do'
for number in range(91, 106):
    assert 'assignee: []' in tasks[f'MSH-{number}']['text']
    assert '<!-- SECTION:PLAN:BEGIN -->' not in tasks[f'MSH-{number}']['text']

links = 0
for path in list(Path('docs/m4/xiangqi').glob('*.md')) + [Path('docs/m5/opening-knowledge.md')]:
    for target in re.findall(r'\]\(([^)]+)\)', path.read_text()):
        if re.match(r'\w+://', target) or target.startswith('#'):
            continue
        assert (path.parent / target.split('#')[0]).exists(), (path, target)
        links += 1
for path in Path('docs/m4/xiangqi').glob('*.json'):
    json.loads(path.read_text())
fixture = Path('docs/m4/xiangqi/fixtures.md').read_text()
opening = fixture.split('```')[1]
actions = re.findall(r'\b[a-i][0-9][a-i][0-9]\b', opening)
assert len(actions) == len(set(actions)) == 44
old_fixture = subprocess.check_output(['git', 'show', f'{BASELINE}:docs/m4/xiangqi/fixtures.md'], text=True)
assert opening == old_fixture.split('```')[1]
for cycle in ['e9f9 f8e8 f9e9 e8f8', 'e9a9 f8f7 a9e9 f7f8']:
    assert cycle in fixture and cycle in old_fixture
hashes = 0
if args.research_root:
    manifest = json.loads(Path('docs/m4/xiangqi/source-manifest.json').read_text())
    for row, name in zip(manifest['sources'], ['wxf2018-en.pdf', 'wxf2018-zh.pdf', 'axf-en.pdf', 'axf-zh.pdf']):
        data = (args.research_root / name).read_bytes()
        assert len(data) == row['bytes'] and hashlib.sha256(data).hexdigest() == row['sha256']
        hashes += 1
    manifest = json.loads(Path('docs/m4/xiangqi/comparator-source-manifest.json').read_text())
    for candidate in manifest['candidates']:
        checkout = args.research_root / candidate['name']
        head = subprocess.check_output(['git', '-C', str(checkout), 'rev-parse', 'HEAD'], text=True).strip()
        assert head == candidate['commit']
        assert not subprocess.check_output(['git', '-C', str(checkout), 'status', '--porcelain'], text=True)
        for row in candidate['files']:
            data = (checkout / row['path']).read_bytes()
            assert len(data) == row['bytes'] and hashlib.sha256(data).hexdigest() == row['sha256']
            hashes += 1
product_diff = subprocess.check_output(['git', 'diff', BASELINE, '--', 'crates', 'desktop',
                                       'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml',
                                       '.github/workflows', 'backlog/milestones'], text=True)
assert not product_diff
changed = subprocess.check_output(['git', 'diff', '--name-only', BASELINE], text=True).splitlines()
untracked = subprocess.check_output(['git', 'ls-files', '--others', '--exclude-standard'], text=True).splitlines()
assert all(path.startswith(('docs/', 'backlog/')) or path == 'README.md'
           for path in changed + untracked), 'change outside the documentation/backlog scope'
result = dict(status='passed', baseline=BASELINE, phase=args.phase, tasks=len(tasks),
              dag='acyclic/all dependencies resolve', unchanged_completed_records=protected,
              original_criteria_and_dependencies='preserved; owner-directed80AC1 reconciliation explicit',
              local_links=links, initial44_and_paired_cycles='preserved, not executed',
              acquired_pdf_and_pinned_source_hashes=hashes,
              product_and_milestone_changes='none', opening_experiment='104 required by56; adoption optional',
              exact_verification='105 required by84/103/45',
              runtime_verification='not run; documentation-only change')
output = json.dumps(result, indent=2) + '\n'
if args.output:
    args.output.write_text(output)
print(output, end='')
