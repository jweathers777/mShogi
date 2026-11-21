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
import hashlib
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
# The per-task spending cap of AGENTS.md; `estimated_compute_before_usd`
# carries the earlier spend of the task.
COMPUTE_USD = 10
# Frozen data in the worker image; a local smoke run may point elsewhere.
DATA = os.environ.get('MSH204_DATA', '/opt/data')
ENGINES = ('engine', 'runner', 'writer', 'helper')
# Optional frozen predecessor (MSH-200): its engine and helper, for match pairs
# and same-machine profiles and probes of both builds.
BASELINE = ('baseline', 'baseline_helper')
# Must equal FULL_VISITS and PROVER_VERSION in crates/mshogi-runner/examples/tori_strength.rs.
PROOF_BOUND = 20_000_000
PROVER_VERSION = 'tori-mate-proof-v2'


def policy(prefix=PREFIX):
    from tools.strength import anchor_cloud
    return anchor_cloud.CampaignPolicy(prefix, CAMPAIGN_SECONDS, COMPUTE_USD, 'tools.strength.tori_cloud',
                                       ('worker',))


def validate_task(t):
    from tools.strength.anchor_worker import require
    kind = t.get('kind')
    if kind == 'pairs':
        fields = {'kind', 'label', 'seeds', 'base_ms', 'increment_ms', 'factor', 'concurrency', 'pair_ms'}
        require(set(t) in (fields, fields | {'opponent'}), 'pairs fields')
        require(t.get('opponent', 'self') in ('self', 'baseline'), 'pairs opponent')
        require((t['label'] == 'match') == (t.get('opponent') == 'baseline'), 'match pairs play the baseline')
        require(t['label'] in ('control', 'odds', 'match') and isinstance(t['seeds'], list) and t['seeds']
                and all(type(s) is int and s >= 1 for s in t['seeds']), 'pairs seeds')
        require(1000 <= t['base_ms'] <= 60000 and 0 <= t['increment_ms'] <= 5000 and t['factor'] in (1, 2)
                and t['concurrency'] in (1, 2) and 60000 <= t['pair_ms'] <= 600000, 'pairs bounds')
    elif kind == 'probe':
        require(set(t) - {'build'} == {'kind', 'ms', 'shard', 'shards'}, 'probe fields')
        require(t.get('build', 'candidate') in ('candidate', 'baseline'), 'probe build')
        require(t['ms'] in (20, 200, 1000) and 0 <= t['shard'] < t['shards'] <= 8, 'probe bounds')
    elif kind == 'movegen':
        require(set(t) - {'build'} == {'kind', 'repeats'} and type(t['repeats']) is int
                and 1 <= t['repeats'] <= 10000, 'movegen fields')
        require(t.get('build', 'candidate') in ('candidate', 'baseline'), 'movegen build')
    elif kind == 'depth':
        require(set(t) == {'kind', 'ms', 'min_ply'}, 'depth fields')
        require(t['ms'] in (50, 100, 200) and 0 <= t['min_ply'] <= 400, 'depth bounds')
    else:
        require(False, 'unknown task kind')


def validate_manifest(m):
    from tools.strength.anchor_worker import require
    require(m.get('schema') == 1 and re.fullmatch(r'msh[0-9]+-[a-z0-9-]{1,50}',
                                                  m.get('experiment', '')), 'wrong campaign identity')
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
    engines = set(m['build']['engines'])
    require(engines in (set(ENGINES), set(ENGINES + BASELINE)), 'four binaries, or six with a baseline')
    require(engines == set(ENGINES + BASELINE) or not any(
        t.get('opponent') == 'baseline' or t.get('build') == 'baseline' for t in tasks), 'baseline binaries')
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
        config_writer=e['writer']['path'], candidate=e['engine']['path'],
        baseline=e['baseline' if t.get('opponent') == 'baseline' else 'engine']['path'],
        candidate_nodes=5_000_000, baseline_depth=64, baseline_nodes=5_000_000, calibrate=False,
        max_steps=1000, pair_wall_ms=t['pair_ms'], base=t['base_ms'] / 1000, increment=t['increment_ms'] / 1000,
        game='tori', candidate_time_factor=t['factor'], openings=f'{DATA}/tori-openings-v1.json',
        opening_count=len(json.loads(Path(f'{DATA}/tori-openings-v1.json').read_text())['openings']),
        random_plies=0, profile=None, watchdog_ms=None, runner=e['runner']['path'])


def run_task(m, t, directory, upload, end):
    """Run one task in `directory`; upload every artifact. Returns a summary."""
    from tools.strength import native_match
    helper = m['build']['engines']['baseline_helper' if t.get('build') == 'baseline' else 'helper']['path']
    suffix = '-baseline' if t.get('build') == 'baseline' else ''
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
        upload(f'probe{suffix}/{out.name}', out)
        return json.loads(out.read_text())['all']
    listing = Path(directory) / 'reports.json'
    listing.write_text(json.dumps(sorted(str(p) for p in Path(f'{DATA}/control').glob('*.report.json'))))
    if t['kind'] == 'movegen':
        out = Path(directory) / f'movegen{suffix}.json'
        subprocess.run([helper, 'movegen', str(listing), str(out), str(t['repeats']),
                        f'{DATA}/tori-tactics-v1.json', f'{DATA}/tori-openings-v1.json'],
                       check=True, capture_output=True, timeout=max(60, end - time.time()))
        upload(f'movegen/{out.name}', out)
        return {'sets': list(json.loads(out.read_text())['sets'])}
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
                with tempfile.TemporaryDirectory(prefix='tori-') as directory:
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
    anchor_cloud.run(args, policy=policy(m['experiment'].split('-')[0]), manifest_validator=validate_manifest)


def campaign_bins():
    """The reviewed MSH-204 work, fixed before any result: eight bins of the
    control pairs, the time-odds pairs, the suite probes and the depth task."""
    bins = [[] for _ in range(8)]
    for i, task in enumerate(campaign_tasks()):
        bins[i % 8].append(task)
    return bins


def match_bins():
    """MSH-200, fixed before any result: one bin profiles both builds on the
    same VM; seven bins play candidate-predecessor pairs at the Tori control
    (5 s + 0.1 s). The suite probes run separately (`suite_bins`): one
    unsharded probe with proofs takes about half an hour."""
    bins = [[{'kind': 'movegen', 'repeats': 200, 'build': b} for b in ('baseline', 'candidate')]]
    for b in range(7):
        bins.append([{'kind': 'pairs', 'label': 'match', 'opponent': 'baseline', 'factor': 1,
                      'seeds': list(range(1 + 32 * b, 33 + 32 * b)), 'base_ms': 5000, 'increment_ms': 100,
                      'concurrency': 2, 'pair_ms': 600000}])
    return bins


def corpus_bins(first_seed=1):
    """MSH-206 (Tori Evaluation Round) Texel corpus: 3,200 self-play pairs of
    the frozen predecessor at 2 s + 0.05 s, 400 pairs per VM, seeds from
    `first_seed`. Seed N plays catalog opening (N - 1) mod 218; timing makes
    repeated openings differ. A later batch starts after the earlier seeds."""
    return [[{'kind': 'pairs', 'label': 'control', 'factor': 1,
              'seeds': list(range(first_seed + 400 * b, first_seed + 400 * (b + 1))),
              'base_ms': 2000, 'increment_ms': 50, 'concurrency': 2, 'pair_ms': 300000}] for b in range(8)]


def suite_bins():
    """20 ms tactical-suite probes of both builds, four shards each, one per VM."""
    return [[{'kind': 'probe', 'ms': 20, 'shard': shard, 'shards': 4, 'build': b}]
            for b in ('baseline', 'candidate') for shard in range(4)]


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
    if args.baseline_bin_dir:
        old = Path(args.baseline_bin_dir)
        old_source = (old / 'source.txt').read_text().strip()
        for k, n in (('baseline', 'mshogi'), ('baseline_helper', 'tori_strength')):
            build['engines'][k] = {'path': '/opt/engines/baseline/' + n, 'sha256': file_hash(old / n),
                                   'source': old_source}
    start = time.time()
    m = {'schema': 1, 'experiment': args.experiment, 'campaign_started_epoch': start,
         'deadline_epoch': start + CAMPAIGN_SECONDS, 'estimated_compute_before_usd': args.before,
         'task_seconds': args.task_seconds, 'build': build,
         'bins': (smoke_bins() if args.smoke else [[{'kind': 'movegen', 'repeats': 200}]] if args.movegen
                  else match_bins() if args.match else corpus_bins(args.corpus_first_seed) if args.corpus else suite_bins() if args.suite else campaign_bins())}
    validate_manifest(m)
    Path(args.out).write_text(json.dumps(m))
    print(sum(map(len, m['bins'])), 'tasks in', len(m['bins']), 'bins')


def collect(raw, out, h1=15):
    """Summarize retrieved campaign artifacts (local, reads files only)."""
    import collections
    import gzip
    import statistics
    from tools.strength import native_match, stats
    raw = Path(raw)
    from tools.experiments.core import decode
    manifest = decode((raw / 'manifest.json').read_bytes())
    # Every task must have completed; a failed or skipped pair stops the analysis.
    for receipt in sorted(raw.glob('tasks/*/receipt.json')):
        r = json.loads(receipt.read_text())
        if not r.get('complete'):
            raise SystemExit(f'{receipt}: task incomplete: {r.get("error")}')
        for row in r['results']:
            if row.get('error'):
                raise SystemExit(f'{receipt}: {row["task"]["kind"]} failed: {row["error"]}')
            for pair in (row.get('result') or {}).get('pairs', []) if row['task']['kind'] == 'pairs' else []:
                if 'error' in pair or 'skipped' in pair:
                    raise SystemExit(f'{receipt}: pair {pair["seed"]} not played: {pair}')
    result = {}
    for label in ('control', 'odds', 'match'):
        expected = sorted(s for b in manifest['bins'] for t in b
                          if t['kind'] == 'pairs' and t['label'] == label for s in t['seeds'])
        summaries = [json.loads(f.read_text()) for f in sorted(raw.glob(f'tasks/*/*/{label}/pair-*.summary.json'))]
        if sorted(row['seed'] for row in summaries) != expected:
            raise SystemExit(f'{label}: pair summaries do not match the manifest seeds')
        reason = native_match.unfinished(SimpleNamespace(game='tori'), summaries)
        if reason:
            raise SystemExit(f'{label}: {reason}')
        stops = collections.Counter(s for row in summaries for s in row['stops'])
        plies = [n for row in summaries for n in row['plies']]
        scored = [row['pair_score'] for row in summaries if row['pair_score'] is not None]
        counts = stats.pentanomial(scored)
        # Real-game depth by ply within each game (after the opening), from the
        # native-replayed reports; both players' moves count.
        late, seconds = [], []
        for path in sorted(raw.glob(f'tasks/*/*/{label}/pair-*.report.json.gz')):
            for game in json.loads(gzip.decompress(path.read_bytes()))['games']:
                for ply, step in enumerate(game['steps']):
                    t = native_match.telemetry(step['observation']) if step['observation'] else None
                    if t is None:
                        continue
                    seconds.append(step['elapsed_ns'] / 1e9)
                    if ply >= 100:
                        late.append(t['completed_depth'])
        late.sort()
        entry = {'pairs': len(summaries), 'seeds_expected': len(expected), 'stops': dict(stops),
                 'length_limited_pairs': sum(native_match.length_limited(
                     SimpleNamespace(game='tori'), row) for row in summaries),
                 'median_plies': statistics.median(plies) if plies else None, 'max_plies': max(plies, default=None),
                 'game_depth_from_ply_100': {'moves': len(late),
                                             'median': statistics.median(late) if late else None,
                                             'p10': late[len(late) // 10] if late else None},
                 'median_move_seconds': statistics.median(seconds) if seconds else None,
                 'pentanomial': counts, 'estimate': stats.estimate(counts) if sum(counts) else None,
                 'sprt_fixed_sample': stats.Sprt(0, h1, .05, .05, 30).status(counts) if sum(counts) else None}
        if label == 'match' and scored:
            # Sequential test in seed order over every launched pair: the first
            # decision is the result; later pairs are retained, not used.
            ordered = [row['pair_score'] for row in sorted(summaries, key=lambda row: row['seed'])]
            test = stats.Sprt(0, h1, .05, .05, 30)
            for n in range(1, len(ordered) + 1):
                status = test.status(stats.pentanomial(ordered[:n]))
                if status['decision'] is not None:
                    break
            entry['sprt_sequential'] = dict(status, pairs=n)
        if summaries:
            result[label] = entry
    suite_path = Path(__file__).resolve().parents[2] / 'tools/strength/data/tori-tactics-v1.json'
    suite_bytes = suite_path.read_bytes()
    frozen = json.loads((Path(__file__).resolve().parents[2] / 'tools/strength/data/tori-cloud-data-files.json')
                        .read_text())['files']['tori-tactics-v1.json']
    if hashlib.sha256(suite_bytes).hexdigest() != frozen:
        raise SystemExit('suite differs from the frozen cloud data')
    suite = json.loads(suite_bytes)
    diagrams = {r['id']: r['diagram'] for r in suite['positions']}
    result['movegen'] = {f.stem: json.loads(f.read_text())
                         for f in sorted(raw.glob('tasks/*/*/movegen*/movegen*.json'))}
    proofs = {}
    def count(rows, split):
        sel = [r for r in rows if split is None or r['split'] == split]
        verdict = collections.Counter(r['verdict'] for r in sel)
        return {'positions': len(sel), 'passed': sum(r['pass'] for r in sel), 'verdicts': dict(verdict)}

    for folder, key in (('probe', 'suite'), ('probe-baseline', 'suite_baseline')):
        probes = collections.defaultdict(list)
        for f in sorted(raw.glob(f'tasks/*/*/{folder}/probe-*.json')):
            probes[int(f.name.split('-')[1])] += json.loads(f.read_text())['results']
        result[key] = {}
        for ms, rows in sorted(probes.items()):
            result[key][ms] = {'all': count(rows, None), 'development': count(rows, 'development'),
                               'held_out': count(rows, 'held-out')}
            for r in rows:
                if r['verdict'] in ('proved_win', 'refuted', 'unknown'):
                    # Same key as tori_strength store_key: canonical diagram digest,
                    # action, mate length, visit bound, prover version.
                    digest = hashlib.sha256(json.dumps(diagrams[r['id']], sort_keys=True,
                                                       separators=(',', ':')).encode()).hexdigest()
                    proofs[f"{digest}|{r['chosen']}|{r['mate_plies']}|{PROOF_BOUND}|{PROVER_VERSION}"] = {
                        'verdict': r['verdict'], 'id': r['id']}
    result['proofs_stored'] = len(proofs)
    depth = [json.loads(f.read_text()) for f in sorted(raw.glob('tasks/*/*/depth/depth-*.json'))]
    result['fresh_search_depth'] = depth
    Path(out).write_text(json.dumps(result, indent=1))
    Path(out).with_name('proof-store.json').write_text(json.dumps(
        {'rule': 'native full-width prover; verdict proved_win | refuted | unknown (visit bound)',
         'visit_bound': PROOF_BOUND, 'prover': PROVER_VERSION, 'proofs': proofs}, indent=1))
    print(json.dumps({k: v for k, v in result.items() if k != 'fresh_search_depth'}, indent=1)[:4000])
    print(json.dumps(depth, indent=1))


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
    k.add_argument('--movegen', action='store_true', help='MSH-200 move generation profile (one VM)')
    k.add_argument('--match', action='store_true', help='candidate against --baseline-bin-dir')
    k.add_argument('--suite', action='store_true', help='suite probes of both builds')
    k.add_argument('--corpus', action='store_true', help='MSH-206 self-play pairs for the Texel corpus')
    k.add_argument('--corpus-first-seed', type=int, default=1, help='first pair seed of a corpus batch')
    k.add_argument('--baseline-bin-dir', help='binaries of the frozen predecessor (mshogi, tori_strength)')
    c = sub.add_parser('collect')
    c.add_argument('--raw', required=True)
    c.add_argument('--out', required=True)
    c.add_argument('--h1', type=float, default=15, help='SPRT H1 in Elo (H0 is 0)')
    a = p.parse_args()
    if a.command == 'worker':
        worker(a.manifest, a.store)
    elif a.command == 'submit':
        submit(a)
    elif a.command == 'collect':
        collect(a.raw, a.out, a.h1)
    else:
        make_manifest(a)


if __name__ == '__main__':
    main()
