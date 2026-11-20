"""MSH-204 (Tori Strength Harness) measurements on the existing Batch/GCS path.

One frozen manifest lists eight bins of tasks, one bin per VM, run in order:

- `pairs`: native self-play pairs through the clocked native route
  (`native_match.run_pair`: config writer, `mshogi-match experiment-run`,
  native replay). The candidate may get a multiple of the clock (time odds).
- `probe`: `tori_strength probe` of one shard of the tactical suite at one
  budget, starting from the frozen proof store.
- `depth`: `tori_strength depth` on the frozen late-game source reports.

No cloud action occurs on import. `submit` runs one reviewed campaign and
never launches a next one. Results are interpreted locally (`collect`).

Usage:
  python3 -m tools.strength.tori_cloud manifest --bin-dir D --image I --experiment msh204-... --out M
  python3 -m tools.strength.tori_cloud submit --manifest M --config C --image I --job J --out DIR
  python3 -m tools.strength.tori_cloud worker --manifest URI --store URI   (Batch worker)
"""
import argparse
import concurrent.futures
import json
import math
import os
import re
import subprocess
import tempfile
import time
from pathlib import Path
from types import SimpleNamespace

PREFIX = 'msh204'
CAMPAIGN_SECONDS = 2 * 3600
COMPUTE_USD = 5
# Frozen data in the worker image; a local smoke run may point elsewhere.
DATA = os.environ.get('MSH204_DATA', '/opt/data')
ENGINES = ('engine', 'runner', 'writer', 'helper')


def policy():
    from tools.strength import anchor_cloud
    return anchor_cloud.CampaignPolicy(PREFIX, CAMPAIGN_SECONDS, COMPUTE_USD, 'tools.strength.tori_cloud',
                                       ('worker',))


def validate_task(t):
    from tools.strength.anchor_worker import require
    kind = t.get('kind')
    if kind == 'pairs':
        require(set(t) == {'kind', 'label', 'seeds', 'base_ms', 'increment_ms', 'factor', 'concurrency',
                           'pair_ms'}, 'pairs fields')
        require(t['label'] in ('control', 'odds') and isinstance(t['seeds'], list) and t['seeds']
                and all(type(s) is int and s >= 1 for s in t['seeds']), 'pairs seeds')
        require(1000 <= t['base_ms'] <= 60000 and 0 <= t['increment_ms'] <= 5000 and t['factor'] in (1, 2)
                and t['concurrency'] in (1, 2) and 60000 <= t['pair_ms'] <= 600000, 'pairs bounds')
    elif kind == 'probe':
        require(set(t) == {'kind', 'ms', 'shard', 'shards'}, 'probe fields')
        require(t['ms'] in (20, 200, 1000) and 0 <= t['shard'] < t['shards'] <= 8, 'probe bounds')
    elif kind == 'depth':
        require(set(t) == {'kind', 'ms', 'min_ply'}, 'depth fields')
        require(t['ms'] in (50, 100, 200) and 0 <= t['min_ply'] <= 400, 'depth bounds')
    else:
        require(False, 'unknown task kind')


def validate_manifest(m):
    from tools.strength.anchor_worker import require
    require(m.get('schema') == 1 and re.fullmatch(PREFIX + r'-[a-z0-9-]{1,50}', m.get('experiment', '')),
            'wrong campaign identity')
    for field in ('campaign_started_epoch', 'deadline_epoch', 'estimated_compute_before_usd'):
        require(type(m.get(field)) in (int, float) and math.isfinite(m[field]), 'finite ' + field)
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + CAMPAIGN_SECONDS, 'fixed campaign deadline')
    require(0 <= m['estimated_compute_before_usd'] < COMPUTE_USD, 'compute cap reached')
    require(type(m.get('task_seconds')) is int and 600 <= m['task_seconds'] <= CAMPAIGN_SECONDS, 'task seconds')
    bins = m.get('bins')
    require(isinstance(bins, list) and 1 <= len(bins) <= 8 and all(isinstance(b, list) and b for b in bins),
            'one to eight nonempty bins')
    tasks = [t for b in bins for t in b]
    require(len(tasks) <= 64, 'at most 64 tasks')
    for t in tasks:
        validate_task(t)
    seeds = [(t['label'], s) for t in tasks if t['kind'] == 'pairs' for s in t['seeds']]
    require(len(seeds) == len(set(seeds)), 'unique pair seeds per label')
    require(set(m['build']['engines']) == set(ENGINES), 'four binaries')
    for e in m['build']['engines'].values():
        require(Path(e['path']).is_absolute() and re.fullmatch('[a-f0-9]{64}', e['sha256']) is not None
                and re.fullmatch('[a-f0-9]{40}', e['source']) is not None, 'binary identity')
    require(re.fullmatch('[a-f0-9]{64}', m['build'].get('data_sha256', '')) is not None, 'data identity')
    require(re.fullmatch(r'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/[^@]+@sha256:[a-f0-9]{64}',
                         m['build'].get('runtime_image', '')) is not None, 'frozen worker image')


def data_digest(directory):
    """SHA-256 over the sorted (name, file hash) list of the frozen data."""
    import hashlib
    from tools.experiments.core import file_hash
    rows = [f'{p.relative_to(directory)} {file_hash(p)}' for p in sorted(Path(directory).rglob('*'))
            if p.is_file()]
    return hashlib.sha256('\n'.join(rows).encode()).hexdigest()


def pairs_args(m, t):
    e = m['build']['engines']
    return SimpleNamespace(
        config_writer=e['writer']['path'], candidate=e['engine']['path'], baseline=e['engine']['path'],
        candidate_nodes=5_000_000, baseline_depth=64, baseline_nodes=5_000_000, calibrate=False,
        max_steps=1000, pair_wall_ms=t['pair_ms'], base=t['base_ms'] / 1000, increment=t['increment_ms'] / 1000,
        game='tori', candidate_time_factor=t['factor'], openings=f'{DATA}/tori-openings-v1.json',
        opening_count=len(json.loads(Path(f'{DATA}/tori-openings-v1.json').read_text())['openings']),
        random_plies=0, profile=None, watchdog_ms=None, runner=e['runner']['path'])


def run_task(m, t, directory, upload, end):
    """Run one task in `directory`; upload every artifact. Returns a summary."""
    from tools.strength import native_match
    helper = m['build']['engines']['helper']['path']
    if t['kind'] == 'pairs':
        args = pairs_args(m, t)
        out = Path(directory) / 'pairs'
        out.mkdir()

        def one(seed):
            if time.time() + t['pair_ms'] / 1000 + 60 >= end:
                return {'seed': seed, 'skipped': 'time'}
            try:
                row = native_match.run_pair(args, seed, out)
                return {'seed': seed, 'pair_score': row['pair_score'], 'stops': row['stops']}
            except Exception as error:
                return {'seed': seed, 'error': repr(error)}

        with concurrent.futures.ThreadPoolExecutor(t['concurrency']) as pool:
            rows = list(pool.map(one, t['seeds']))
        for f in sorted(out.iterdir()):
            upload(f'{t["label"]}/{f.name}', f)
        return {'pairs': rows}
    if t['kind'] == 'probe':
        suite = json.loads(Path(f'{DATA}/tori-tactics-v1.json').read_text())
        suite['positions'] = suite['positions'][t['shard']::t['shards']]
        shard = Path(directory) / 'shard.json'
        shard.write_text(json.dumps(suite))
        out = Path(directory) / f'probe-{t["ms"]}-{t["shard"]}.json'
        subprocess.run([helper, 'probe', str(shard), str(out), str(t['ms']), f'{DATA}/store.json'],
                       check=True, capture_output=True, timeout=max(60, end - time.time()))
        upload(f'probe/{out.name}', out)
        return json.loads(out.read_text())['all']
    listing = Path(directory) / 'reports.json'
    listing.write_text(json.dumps(sorted(str(p) for p in Path(f'{DATA}/control').glob('*.report.json'))))
    out = Path(directory) / f'depth-{t["ms"]}.json'
    subprocess.run([helper, 'depth', str(listing), str(out), str(t['ms']), str(t['min_ply'])],
                   check=True, capture_output=True, timeout=max(60, end - time.time()))
    upload(f'depth/{out.name}', out)
    return json.loads(out.read_text())


def worker(manifest_uri, store_uri):
    from tools.experiments import gcp
    from tools.experiments.core import decode, digest, encode, file_hash
    from tools.strength.anchor_worker import cloud_environment, put_new, require
    bucket, key = gcp.parse_uri(manifest_uri)
    raw = gcp.GCSStore('gs://' + bucket).get(key)
    m = decode(raw)
    validate_manifest(m)
    store = gcp.GCSStore(store_uri)
    index = int(os.environ['BATCH_TASK_INDEX'])
    require(0 <= index < len(m['bins']), 'invalid task index')
    require(os.environ.get('BATCH_TASK_RETRY_ATTEMPT', '0') == '0', 'no worker retries')
    claim = {'manifest_sha256': digest(raw), 'task_index': index, 'started_epoch': time.time()}
    put_new(store, f'tasks/{index}/started.json', encode(claim))
    receipt = dict(claim, complete=False)
    try:
        receipt['environment'] = cloud_environment()
        for name, e in m['build']['engines'].items():
            require(file_hash(e['path']) == e['sha256'], 'binary hash changed: ' + name)
        require(data_digest(DATA) == m['build']['data_sha256'], 'frozen data changed')
        end = min(m['deadline_epoch'] - 300, time.time() + m['task_seconds'] - 120)
        receipt['results'] = []
        for number, task in enumerate(m['bins'][index]):
            # One failed task is recorded; the bin goes on with the next one.
            started = time.time()
            row = {'task': task, 'started_epoch': started}
            try:
                with tempfile.TemporaryDirectory(prefix=PREFIX + '-') as directory:
                    row['result'] = run_task(
                        m, task, directory,
                        lambda name, path: put_new(store, f'tasks/{index}/{number}/{name}',
                                                   Path(path).read_bytes()), end)
            except Exception as error:
                row['error'] = repr(error)
            row['seconds'] = time.time() - started
            receipt['results'].append(row)
        receipt['complete'] = True
    except Exception as error:
        receipt['error'] = repr(error)
        raise
    finally:
        receipt['ended_epoch'] = time.time()
        put_new(store, f'tasks/{index}/receipt.json', encode(receipt))


def submit(args):
    from tools.experiments import gcp
    from tools.experiments.core import decode
    from tools.strength import anchor_cloud
    from tools.strength.anchor_worker import require
    m = decode(Path(args.manifest).read_bytes())
    validate_manifest(m)
    require(args.image == m['build']['runtime_image'], 'submitted image differs from frozen worker')
    config = gcp.validate_config(decode(Path(args.config).read_bytes()))
    require(config['max_run_seconds'] == m['task_seconds'], 'task limit differs')
    anchor_cloud.run(args, policy=policy(), manifest_validator=validate_manifest)


def campaign_bins():
    """The reviewed MSH-204 work, fixed before any result: eight bins of the
    control pairs, the time-odds pairs, the suite probes and the depth task."""
    bins = [[] for _ in range(8)]
    for i, task in enumerate(campaign_tasks()):
        bins[i % 8].append(task)
    return bins


def smoke_bins():
    """One small task of each kind, to check the whole path before a campaign."""
    pairs = {'kind': 'pairs', 'seeds': [113], 'base_ms': 5000, 'increment_ms': 100, 'concurrency': 1,
             'pair_ms': 600000}
    return [[dict(pairs, label='control', factor=1)], [dict(pairs, label='odds', factor=2, seeds=[201])],
            [{'kind': 'probe', 'ms': 20, 'shard': 0, 'shards': 8}], [{'kind': 'depth', 'ms': 50, 'min_ply': 300}]]


def campaign_tasks():
    tasks = []
    control = list(range(113, 169))
    for i in range(8):
        tasks.append({'kind': 'pairs', 'label': 'control', 'seeds': control[i::8], 'base_ms': 5000,
                      'increment_ms': 100, 'factor': 1, 'concurrency': 2, 'pair_ms': 600000})
    odds = list(range(201, 297))
    for i in range(8):
        tasks.append({'kind': 'pairs', 'label': 'odds', 'seeds': odds[i::8], 'base_ms': 5000,
                      'increment_ms': 100, 'factor': 2, 'concurrency': 2, 'pair_ms': 600000})
    for ms in (20, 200, 1000):
        for shard in range(4):
            tasks.append({'kind': 'probe', 'ms': ms, 'shard': shard, 'shards': 4})
    tasks.append({'kind': 'depth', 'ms': 100, 'min_ply': 100})
    return tasks


def make_manifest(args):
    from tools.experiments.core import file_hash
    bins = Path(args.bin_dir)
    source = (bins / 'source.txt').read_text().strip()
    names = {'engine': 'mshogi', 'runner': 'mshogi-match', 'writer': 'tori_match_config',
             'helper': 'tori_strength'}
    build = {'source': source, 'runtime_image': args.image, 'data_sha256': data_digest(args.data_dir),
             'compiler': (bins / 'rustc.txt').read_text().strip() + '; -C target-cpu=x86-64; --locked release',
             'engines': {k: {'path': '/opt/engines/' + n, 'sha256': file_hash(bins / n), 'source': source}
                         for k, n in names.items()}}
    start = time.time()
    m = {'schema': 1, 'experiment': args.experiment, 'campaign_started_epoch': start,
         'deadline_epoch': start + CAMPAIGN_SECONDS, 'estimated_compute_before_usd': args.before,
         'task_seconds': args.task_seconds, 'build': build,
         'bins': smoke_bins() if args.smoke else campaign_bins()}
    validate_manifest(m)
    Path(args.out).write_text(json.dumps(m))
    print(sum(map(len, m['bins'])), 'tasks in', len(m['bins']), 'bins')


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='command', required=True)
    w = sub.add_parser('worker')
    w.add_argument('--manifest', required=True)
    w.add_argument('--store', required=True)
    s = sub.add_parser('submit')
    for name in ('manifest', 'config', 'image', 'job', 'out'):
        s.add_argument('--' + name, required=True)
    s.add_argument('--subscription', default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    k = sub.add_parser('manifest')
    for name in ('bin-dir', 'data-dir', 'image', 'experiment', 'out'):
        k.add_argument('--' + name, required=True)
    k.add_argument('--task-seconds', type=int, default=3600)
    k.add_argument('--before', type=float, default=0)
    k.add_argument('--smoke', action='store_true')
    a = p.parse_args()
    if a.command == 'worker':
        worker(a.manifest, a.store)
    elif a.command == 'submit':
        submit(a)
    else:
        make_manifest(a)


if __name__ == '__main__':
    main()
