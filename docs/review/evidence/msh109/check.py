#!/usr/bin/env python3
"""Read-only MSH-109 source/evidence reconciliation; no builds or game execution."""
import gzip
import hashlib
import json
from pathlib import Path
import platform
import re
import subprocess

ROOT = Path(__file__).resolve().parents[4]
SOURCE = 'e161cef2935cd9b33449757699dba4ccfa0874dd'
PRIOR = '1d3920227bc376b5cc1694928597e7b5a1d5a864'
TESTED = '9a164eb4379231dcb4fca484adb7ec5597e7c991'


def command(*args):
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def git(*args):
    return command('git', *args)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads((ROOT / path).read_text())


runtime = ['crates', 'desktop', 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', '.github']
assert not git('diff', '--name-only', SOURCE, '--', *runtime)
assert not git('diff', '--name-only', TESTED, SOURCE, '--', *runtime)
unchanged = ['crates/mshogi-core', 'crates/mshogi-search', 'crates/mshogi-engine',
             'desktop/src', 'desktop/src-tauri']
trees = {}
for path in unchanged:
    a, b = git('rev-parse', f'{PRIOR}:{path}'), git('rev-parse', f'{SOURCE}:{path}')
    assert a == b, path
    trees[path] = b

metadata = {}
for name, extra in [('workspace', []), ('tauri', ['--manifest-path', 'desktop/src-tauri/Cargo.toml'])]:
    value = json.loads(command('cargo', 'metadata', '--offline', '--locked', '--no-deps',
                               '--format-version', '1', *extra))
    metadata[name] = {p['name']: {kind: sorted(d['name'] for d in p['dependencies']
                         if (d['kind'] or 'normal') == kind)
                         for kind in ['normal', 'dev', 'build']} for p in value['packages']}
packages = metadata['workspace']
assert packages['mshogi-core']['normal'] == ['serde', 'serde_json']
assert packages['mshogi-search']['normal'] == ['mshogi-core']
assert {'mshogi-core', 'mshogi-search', 'mshogi-engine'} <= set(packages['mshogi-runner']['normal'])
assert 'mshogi-runner' in packages['mshogi-console']['dev']
assert 'mshogi-runner' not in packages['mshogi-console']['normal']
assert metadata['tauri']['mshogi-desktop']['normal'] == ['mshogi-desktop-session', 'tauri', 'tauri-plugin-dialog']

ledger = read('docs/m5/evidence/msh48/verification.json')
assert ledger['tested_source'] == TESTED and ledger['status'] == 'complete'
ci = ledger['ci']
assert ci['source'] == TESTED and ci['conclusion'] == 'success' and ci['attempt'] == 1
assert {j['name'] for j in ci['jobs']} == {'Rust (ubuntu-24.04)', 'Rust (macos-15)', 'Rust (windows-2022)'}
for job in ci['jobs']:
    assert job['head_sha'] == TESTED and job['conclusion'] == 'success' and job['run_attempt'] == 1
    assert job['runner_id'] and len(job['steps']) == 13
    assert all(s['conclusion'] == 'success' for s in job['steps'])
metrics = read('docs/m5/evidence/msh48/pilot-metrics.json')
reports = {}
for name in ['release', 'supplement', 'repeat']:
    ref = metrics[name]
    data = (ROOT / ref['report']).read_bytes()
    raw = gzip.decompress(data)
    assert sha(data) == ref['gzip_sha256'] and sha(raw) == ref['uncompressed_sha256']
    report = json.loads(raw)
    assert report['environment']['harness_revision'] == TESTED
    assert report['environment']['harness_dirty_sha256'] == sha(b'')
    reports[name] = report
assert [len(reports[n]['games']) for n in ['release', 'supplement', 'repeat']] == [56, 8, 56]
assert sum(g['score_first_twice'] is not None for g in reports['release']['games']) == 20
assert sum(g['stop'] == 'step_limit' for g in reports['release']['games']) == 28
assert sum(g['stop'] == 'incomplete:NodeLimit' for g in reports['release']['games']) == 8
assert all(g['stop'] == 'native_concluded' for g in reports['supplement']['games'])
assert sum(len(g['steps']) for g in reports['release']['games']) == 84
assert metrics['data']['rows'] == 84 and metrics['supplement_data']['rows'] == 8
assert metrics['data']['usable_action_targets'] + metrics['supplement_data']['usable_action_targets'] == 84
assert metrics['data']['terminal_labels'] + metrics['supplement_data']['terminal_labels'] == 28


def semantic_games(report):
    return [{**g, 'steps': [{k: v for k, v in s.items() if k != 'elapsed_ns'}
                            for s in g['steps']]} for g in report['games']]


assert reports['release']['config'] == reports['repeat']['config']
assert reports['release']['identities'] == reports['repeat']['identities']
assert semantic_games(reports['release']) == semantic_games(reports['repeat'])

# Preserve every other task exactly; the active task may only gain lifecycle/evidence.
for path in git('ls-tree', '-r', '--name-only', SOURCE, 'backlog/tasks').splitlines():
    if Path(path).name.startswith('msh-109 - '):
        current, original = (ROOT / path).read_text(), git('show', f'{SOURCE}:{path}')
        for section in ['AC', 'DOD', 'SECTION:DESCRIPTION']:
            if section.startswith('SECTION:'):
                begin, end = '<!-- SECTION:DESCRIPTION:BEGIN -->', '<!-- SECTION:DESCRIPTION:END -->'
            else:
                begin, end = f'<!-- {section}:BEGIN -->', f'<!-- {section}:END -->'
            clean = lambda s: re.sub(r'\[[ x]\]', '[]', s.split(begin)[1].split(end)[0]).strip()
            assert clean(current) == clean(original), section
    else:
        assert (ROOT / path).read_bytes() == subprocess.check_output(['git', 'show', f'{SOURCE}:{path}'], cwd=ROOT), path

# The active task retains the original graph/milestone/labels, not just AC prose.
active = next((ROOT / 'backlog/tasks').glob('msh-109 - *.md'))
a, b = active.read_text().split('---')[1], git('show', f'{SOURCE}:{active.relative_to(ROOT)}').split('---')[1]
for key in ['dependencies', 'milestone', 'labels', 'ordinal', 'type']:
    pattern = rf'(?m)^{key}:.*(?:\n[ \t]+.*)*'
    assert re.search(pattern, a).group() == re.search(pattern, b).group(), key

protected = ['docs/design.md', 'docs/testing/conformance-policy.md',
             'docs/review/architecture-health-108.md', 'docs/review/evidence/msh108/verification.json',
             'docs/review/runner-exit-112.md', 'docs/review/evidence/msh112/verification.json',
             'docs/review/history-view-134.md', 'docs/review/evidence/msh134/verification.json',
             'docs/m4/final-delivery.md', 'docs/m4/evidence/msh47/verification.json',
             'docs/m5/experiments.md', 'docs/m5/evidence/msh48/verification.json',
             'docs/m5/evidence/msh48/pilot-metrics.json']
protected_hashes = {}
for path in protected:
    raw = (ROOT / path).read_bytes()
    assert raw == subprocess.check_output(['git', 'show', f'{SOURCE}:{path}'], cwd=ROOT), path
    protected_hashes[path] = sha(raw)

# Spot-check precise current symbol anchors; unchanged tree hashes qualify the rest
# of the carried native map. Values are independently selected source symbols.
anchors = [
 ('crates/mshogi-runner/src/experiments/mod.rs', 154, 'pub fn validate'),
 ('crates/mshogi-runner/src/experiments/mod.rs', 247, 'pub fn schedule'),
 ('crates/mshogi-runner/src/experiments/mod.rs', 372, 'fn admit_bounded'),
 ('crates/mshogi-runner/src/experiments/mod.rs', 521, 'pub fn validate'),
 ('crates/mshogi-runner/src/experiments/native.rs', 193, 'pub fn from_root'),
 ('crates/mshogi-runner/src/experiments/native.rs', 394, 'pub fn admit'),
 ('crates/mshogi-runner/src/experiments/evidence.rs', 120, 'pub fn data_jsonl'),
 ('crates/mshogi-runner/src/experiments/evidence.rs', 222, 'pub fn run_external'),
 ('crates/mshogi-engine/src/bridge.rs', 461, 'serde_json::from_slice'),
 ('crates/mshogi-core/src/chu/record.rs', 149, 'fn replay'),
 ('crates/mshogi-search/src/chu.rs', 286, 'fn minimax'),
 ('crates/mshogi-search/src/tenjiku.rs', 318, 'fn minimax'),
 ('crates/mshogi-desktop-session/src/lib.rs', 346, 'enum ActiveDocument'),
 ('crates/mshogi-desktop-session/src/lib.rs', 1207, 'struct DesktopController'),
 ('crates/mshogi-desktop-session/src/persistence.rs', 1679, 'fn install_document'),
]
for path, line, expected in anchors:
    assert expected in (ROOT / path).read_text().splitlines()[line - 1], (path, line, expected)

report_path = ROOT / 'docs/review/architecture-health-109.md'
local_links = []
for target in re.findall(r'\[[^\]]+\]\(([^)]+)\)', report_path.read_text()):
    if '://' not in target and not target.startswith('#'):
        path = target.split('#')[0]
        assert (report_path.parent / path).exists(), target
        local_links.append(target)

print(json.dumps({'outcome': 'passed', 'reviewed_source': SOURCE, 'prior_source': PRIOR,
 'reused_tested_source': TESTED, 'unchanged_git_trees': trees, 'metadata': metadata,
 'protected_evidence_sha256': protected_hashes, 'anchor_assertions': len(anchors), 'report_local_links': local_links,
 'checks': ['production source preservation', 'unchanged prior native/application surfaces',
            'normal/dev/build dependency boundaries', 'exact-source retained three-platform CI',
            'compressed pilot hashes/counts/semantic repeat', 'all other tasks preserved',
            'original109description/AC/DoD preserved', 'protected design/evidence unchanged',
            'current symbol anchor spot checks'],
 'environment': {'system': platform.system(), 'release': platform.release(), 'machine': platform.machine(),
                 'python': platform.python_version(), 'cargo': command('cargo', '--version'),
                 'rustc': command('rustc', '--version'), 'git': command('git', '--version')},
 'new_product_tests': 'not run; diagnostic/documentation only', 'new_ci_jobs': [],
 'new_benchmarks': 'not run; retained measurements remain source-qualified'}, indent=2))
