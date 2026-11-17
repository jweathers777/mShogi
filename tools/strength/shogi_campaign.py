"""Shogi predecessor game tests on the existing Google Cloud Batch path.

MSH-190 (Shogi Selectivity Round) generalization of the MSH-187 worker
(`shogi_cloud.py`, kept unchanged for its frozen evidence). One manifest is one
batch: at most eight c3-standard-4 VMs, no retries, each VM runs its assigned
pairs with a fixed concurrency. Pair N uses opening index N-1 of the frozen
catalog, so a campaign never repeats an opening while max_pairs <= catalog size.
The native runner owns rules, clocks, histories and scores; every report is
replayed by the native authority on the worker and again on the host. An
unfinished pair stops the campaign; it is never scored. No cloud action occurs
on import. `submit` runs one reviewed batch and never launches the next one.
"""
import argparse
import concurrent.futures
import gzip
import io
import math
import os
import re
import resource
import subprocess
import tempfile
import time
from pathlib import Path
from types import SimpleNamespace

from tools.experiments import gcp
from tools.experiments.core import decode, digest, encode, file_hash
from tools.strength import anchor_cloud, native_match, stats
from tools.strength.anchor_worker import cloud_environment, put_new, require

PREFIX = 'msh190'
# Campaign window and compute cap of one comparison (all batches together).
POLICY = anchor_cloud.CampaignPolicy(PREFIX, 2 * 3600, 4, 'tools.strength.shogi_campaign', ('worker',))
RATE = .201608
STORE = 'gs://mshogi-experiments-216675633801/msh190/'
MAX_STEPS = 320


def seeds(m):
    return sorted(n for rows in m['bins'] for n in rows)


def validate_manifest(m):
    require(m.get('schema') == 1 and re.fullmatch(PREFIX + r'-[a-z0-9-]{1,50}', m.get('experiment', '')),
            'wrong campaign identity')
    for field in ('campaign_started_epoch', 'deadline_epoch', 'estimated_compute_before_usd'):
        require(type(m.get(field)) in (int, float) and math.isfinite(m[field]), 'finite ' + field)
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + POLICY.seconds, 'fixed campaign deadline required')
    require(0 <= m['estimated_compute_before_usd'] < POLICY.compute_usd, 'compute cap reached')
    p = m.get('protocol', {})
    require(set(p) == {'base_ms', 'increment_ms', 'elo0', 'elo1', 'alpha', 'beta', 'min_pairs', 'max_pairs',
                       'pair_ms', 'concurrency', 'task_seconds'}, 'fixed protocol fields required')
    require(1000 <= p['base_ms'] <= 600000 and 0 <= p['increment_ms'] <= 10000, 'clock bounds')
    require(p['alpha'] == p['beta'] == .05 and p['elo0'] == 0 and p['elo1'] in (10, 15), 'reviewed SPRT required')
    rows = m['openings']['openings']
    require(digest(encode(m['openings'])) == m['openings_sha256'], 'opening catalog hash differs')
    require(len({r['id'] for r in rows}) == len(rows) >= 100, 'at least 100 unique openings required')
    require(all(r.get('sfen') is None and 1 <= len(r['moves']) <= 32 for r in rows), 'standard-root prefixes')
    require(30 <= p['min_pairs'] <= p['max_pairs'] <= len(rows), 'no opening repeat within one comparison')
    require(1 <= p['concurrency'] <= 2 and 60000 <= p['pair_ms'] <= 1800000, 'pair bounds')
    bins = m.get('bins')
    require(isinstance(bins, list) and 1 <= len(bins) <= 8 and all(isinstance(b, list) and b for b in bins),
            'one to eight nonempty worker bins')
    flat = seeds(m)
    require(len(flat) == len({n for b in bins for n in b}) and flat == list(range(flat[0], flat[-1] + 1))
            and 1 <= flat[0] and flat[-1] <= p['max_pairs'], 'unique contiguous pair numbers required')
    waves = max(math.ceil(len(b) / p['concurrency']) for b in bins)
    require(p['task_seconds'] >= waves * (p['pair_ms'] / 1000 + 120) + 300, 'task bound below pair bound')
    build = m['build']
    require(re.fullmatch('[a-f0-9]{40}', build['source']) is not None, 'frozen harness revision required')
    require(set(build['engines']) == {'candidate', 'baseline', 'runner', 'config_writer'}, 'four named binaries')
    for e in build['engines'].values():
        require(Path(e['path']).is_absolute() and re.fullmatch('[a-f0-9]{64}', e['sha256']) is not None
                and re.fullmatch('[a-f0-9]{40}', e['source']) is not None, 'binary path, hash and source')
    require(all(build['engines'][k]['source'] == build['source'] for k in ('runner', 'config_writer')),
            'runner/writer must match frozen harness source')
    require(re.fullmatch(r'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/[^@]+@sha256:[a-f0-9]{64}',
                         build.get('runtime_image', '')) is not None, 'frozen worker image required')


def identity(m):
    """Properties that cannot change between batches of one comparison."""
    return {k: m[k] for k in ('experiment', 'campaign_started_epoch', 'deadline_epoch', 'protocol', 'build',
                             'openings_sha256')}


def report_bytes(data):
    with gzip.GzipFile(fileobj=io.BytesIO(data)) as stream:
        raw = stream.read(128 * 1024 * 1024 + 1)
    require(len(raw) <= 128 * 1024 * 1024, 'native report exceeds evidence cap')
    return raw


def audit_report(report, config, m, seed):
    p = m['protocol']
    require(report['config'] == config and config['seed'] == seed, 'wrong effective pair config')
    require(config['clock'] == {'base_ms': p['base_ms'], 'increment_ms': p['increment_ms']} and
            config['max_steps'] == MAX_STEPS and config['wall_budget_ms'] == p['pair_ms'], 'wrong clock/pair bound')
    require(config['rounds'] == 1 and len(config['openings']) == 1 and len(config['players']) == 2,
            'expected one native opening pair')
    index = seed - 1
    selected = m['openings']['openings'][index]
    opening = config['openings'][0]
    require(opening['id'] == selected['id'] and opening['provenance'].startswith(
        f"Frozen catalog SHA256 {m['openings_sha256']}; index {index};"), 'wrong opening assignment')
    native = opening['root']['record']['payload']
    require(native['root']['category'] == 'standard' and native['root']['known_history'] == 'initial',
            'opening must retain the ordinary initial root')
    require([e['command'] for e in native['events']] == [{'kind': 'play', 'action': mv} for mv in selected['moves']],
            'opening full prefix differs')
    env = report['environment']
    require(env['harness_revision'] == m['build']['source'] and env['harness_dirty_sha256'] == digest(b'')
            and env['os'] == 'linux' and env['arch'] == 'x86_64', 'wrong or dirty native runner build')
    for i, name in enumerate(('candidate', 'baseline')):
        e = m['build']['engines'][name]
        require(config['players'][i]['executable'] == e['path'] and
                report['identities'][i]['executable']['sha256'] == e['sha256'], 'participant identity differs')
        require(config['players'][i]['limits'] == [{'profile': 'mshogi.shogi.csa36.v1',
                'work': {'depth': 64, 'nodes': 5000000}}], 'asymmetric or wrong safety caps')
    result = native_match.summarize(report)
    require(all(g['stop'] == 'native_concluded' and g['native_outcome'] is not None for g in report['games'])
            and result['pair_score'] is not None, 'unfinished native pair')
    return result


def pair_args(m, opening_path):
    e, p = m['build']['engines'], m['protocol']
    return SimpleNamespace(config_writer=e['config_writer']['path'], candidate=e['candidate']['path'],
                           baseline=e['baseline']['path'], runner=e['runner']['path'], candidate_nodes=5000000,
                           baseline_depth=64, baseline_nodes=5000000, calibrate=False, max_steps=MAX_STEPS,
                           pair_wall_ms=p['pair_ms'], game='shogi', watchdog_ms=60000, base=p['base_ms'] / 1000,
                           increment=p['increment_ms'] / 1000, openings=str(opening_path),
                           opening_count=len(m['openings']['openings']))


def execute_pair(m, seed, directory, task_index=None):
    """Always retain a receipt and the native runner's artifacts."""
    directory.mkdir()
    opening_path = directory / 'openings.json'
    opening_path.write_bytes(encode(m['openings']))
    receipt = {'seed': seed, 'task_index': task_index, 'manifest_sha256': digest(encode(m)),
               'complete': False, 'score': None, 'started_epoch': time.time()}
    start_cpu = time.process_time()
    try:
        for name, engine in m['build']['engines'].items():
            require(file_hash(engine['path']) == engine['sha256'], 'binary hash changed: ' + name)
        result = native_match.run_pair(pair_args(m, opening_path), seed, directory)
        raw = report_bytes((directory / result['report']).read_bytes())
        cfg = decode((directory / f'pair-{seed:04d}.config.json').read_bytes())
        checked = audit_report(decode(raw), cfg, m, seed)
        require(checked['pair_score'] == result['pair_score'] and digest(raw) == result['report_sha256'],
                'pair summary differs from native result')
        receipt.update(complete=True, score=result['pair_score'], summary=result)
    except Exception as error:
        receipt['error'] = repr(error)
    finally:
        receipt.update(ended_epoch=time.time(), controller_cpu_seconds=time.process_time() - start_cpu)
        receipt['artifacts'] = {f.name: {'sha256': file_hash(f), 'bytes': f.stat().st_size}
                                for f in sorted(directory.iterdir()) if f.is_file()}
        (directory / 'receipt.json').write_bytes(encode(receipt))
    return receipt


def worker(manifest_uri, store_uri):
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
    receipt = dict(claim, complete=False, pairs=[])
    p = m['protocol']
    try:
        receipt['environment'] = cloud_environment()
        require(receipt['environment']['gcp'].get('cpu-platform') == 'Intel Sapphire Rapids',
                'CPU platform differs from frozen comparison hardware')
        end = min(m['deadline_epoch'] - 300, time.time() + p['task_seconds'] - 120)

        def one(seed, directory):
            require(not store.exists('abort.json'), 'another worker failed; no new pair')
            require(time.time() + p['pair_ms'] / 1000 + 120 < end, 'insufficient time for a bounded pair')
            prefix = f'pairs/{seed:04d}'
            put_new(store, prefix + '/launch.json', encode(claim | {'seed': seed}))
            dest = Path(directory) / f'{seed:04d}'
            result = execute_pair(m, seed, dest, index)
            for path in sorted(f for f in dest.iterdir() if f.name != 'receipt.json') + [dest / 'receipt.json']:
                put_new(store, prefix + '/' + path.name, path.read_bytes())
            if not result['complete']:
                store.put('abort.json', encode(claim | {'seed': seed, 'error': result.get('error')}))
            return {'seed': seed, 'complete': result['complete']}

        with tempfile.TemporaryDirectory(prefix=PREFIX + '-') as directory:
            with concurrent.futures.ThreadPoolExecutor(p['concurrency']) as pool:
                futures = [pool.submit(one, seed, directory) for seed in m['bins'][index]]
                for future in futures:
                    try:
                        receipt['pairs'].append(future.result())
                    except Exception as error:
                        receipt['pairs'].append({'error': repr(error), 'complete': False})
        require(all(r['complete'] for r in receipt['pairs']), 'unscored or unstarted pair; stop')
        receipt['complete'] = True
    except Exception as error:
        receipt['error'] = repr(error)
        store.put('abort.json', encode(claim | {'error': repr(error)}))
        raise
    finally:
        receipt['ended_epoch'] = time.time()
        put_new(store, f'tasks/{index}/receipt.json', encode(receipt))


def summarize_runs(paths, replay_runner):
    """Audit retrieved batches in seed order and replay with the local native authority."""
    rows, manifests, failures = [], [], []
    upper = 0.
    for name in paths:
        root = Path(name)
        m = decode((root / 'manifest.json').read_bytes())
        validate_manifest(m)
        if manifests:
            require(identity(m) == identity(manifests[0]), 'comparison identity changed')
        manifests.append(m)
        cleanup = decode((root / 'cleanup.json').read_bytes())
        require(cleanup.get('verified_absent') is True, 'cleanup not verified')
        resources = decode((root / 'resources.json').read_bytes())
        require(resources.get('cleanup_error') is None and resources['rate_usd_vm_hour'] == RATE,
                'cleanup uncertainty or wrong rate')
        upper += resources['conservative_compute_usd_upper_bound']
        expected = digest(encode(m))
        for i in range(len(m['bins'])):
            task = root / 'raw' / 'tasks' / str(i) / 'receipt.json'
            if not task.exists() or not decode(task.read_bytes()).get('complete'):
                failures.append({'task': str(task), 'reason': 'worker incomplete'})
            else:
                hw = decode(task.read_bytes())['environment']['gcp']
                require(hw['cpu-platform'] == 'Intel Sapphire Rapids' and
                        hw['machine-type'].endswith('/c3-standard-4'), 'worker hardware differs')
        for n in seeds(m):
            folder = root / 'raw' / 'pairs' / f'{n:04d}'
            if not (folder / 'receipt.json').exists():
                failures.append({'seed': n, 'reason': 'missing pair receipt'})
                continue
            receipt = decode((folder / 'receipt.json').read_bytes())
            require(receipt['seed'] == n and receipt['manifest_sha256'] == expected, 'wrong pair receipt identity')
            for file, info in receipt['artifacts'].items():
                require((folder / file).stat().st_size == info['bytes'] and file_hash(folder / file) == info['sha256'],
                        'artifact integrity differs')
            if not receipt['complete']:
                failures.append({'seed': n, 'reason': receipt.get('error', 'unscored pair')})
                continue
            summary = receipt['summary']
            raw = report_bytes((folder / summary['report']).read_bytes())
            require(digest(raw) == summary['report_sha256'], 'native report hash differs')
            with tempfile.NamedTemporaryFile(suffix='.json') as replay:
                replay.write(raw)
                replay.flush()
                subprocess.run([str(replay_runner), 'experiment-replay', replay.name], check=True,
                               capture_output=True, timeout=120)
            cfg = decode((folder / f'pair-{n:04d}.config.json').read_bytes())
            checked = audit_report(decode(raw), cfg, m, n)
            require(checked['pair_score'] == receipt['score'], 'receipt score differs')
            late = [[t['depth'] for t in turns[len(turns) // 2:]] for turns in checked['turns']]
            rows.append({'seed': n, 'pair_score': checked['pair_score'], 'scores_twice': checked['scores_twice'],
                         'plies': checked['plies'], 'late_depths': late,
                         'wall_seconds': receipt['ended_epoch'] - receipt['started_epoch']})
    flat = [n for m in manifests for n in seeds(m)]
    require(flat == list(range(1, len(flat) + 1)), 'batches must be ordered, complete and nonduplicated')
    p = manifests[0]['protocol']
    sprt = stats.Sprt(p['elo0'], p['elo1'], p['alpha'], p['beta'], p['min_pairs'])
    ordered = sorted(rows, key=lambda r: r['seed'])
    counts = stats.pentanomial(r['pair_score'] for r in ordered)
    scores = [s for r in rows for s in r['scores_twice']]
    late = [[d for r in rows for d in r['late_depths'][i]] for i in range(2)]
    return {'pairs_attempted': len(flat), 'scored_pairs': len(rows), 'pentanomial': counts,
            'wins_draws_losses': [scores.count(2), scores.count(1), scores.count(0)],
            'sprt': sprt.status(counts), 'estimate': stats.estimate(counts), 'failures': failures,
            'late_game_median_depth': [sorted(d)[len(d) // 2] if d else None for d in late],
            'conservative_compute_usd_upper_bound': upper, 'rows': ordered}


def submit(args):
    m = decode(Path(args.manifest).read_bytes())
    validate_manifest(m)
    require(args.image == m['build']['runtime_image'], 'submitted image differs from frozen worker')
    previous = summarize_runs(args.previous, args.replay_runner) if args.previous else None
    if previous:
        require(not previous['failures'] and previous['sprt']['decision'] is None, 'campaign already stopped')
        require(seeds(m)[0] == previous['pairs_attempted'] + 1, 'next batch skips/repeats pair numbers')
        require(math.isclose(m['estimated_compute_before_usd'], previous['conservative_compute_usd_upper_bound']),
                'wrong prior cost')
        for root in args.previous:
            require(identity(decode((Path(root) / 'manifest.json').read_bytes())) == identity(m), 'changed identity')
    else:
        require(seeds(m)[0] == 1 and m['estimated_compute_before_usd'] == 0, 'first batch starts at pair 1')
    config = gcp.validate_config(decode(Path(args.config).read_bytes()))
    require(config['max_run_seconds'] == m['protocol']['task_seconds'] and config.get('vm_hourly_usd') == RATE,
            'fixed task limit and reviewed compute rate required')
    require(config['store_uri'] == STORE + m['experiment'] + f'/batches/{seeds(m)[0]:04d}',
            'canonical immutable batch prefix required')
    anchor_cloud.run(args, policy=POLICY, manifest_validator=validate_manifest)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    w = sub.add_parser('worker')
    w.add_argument('--manifest', required=True)
    w.add_argument('--store', required=True)
    s = sub.add_parser('submit')
    for name in ('manifest', 'config', 'image', 'job', 'out'):
        s.add_argument('--' + name, required=True)
    s.add_argument('--subscription', default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    s.add_argument('--previous', nargs='*', default=[])
    s.add_argument('--replay-runner', default='target/release/mshogi-match')
    a = sub.add_parser('summarize')
    a.add_argument('runs', nargs='+')
    a.add_argument('--out', required=True)
    a.add_argument('--replay-runner', default='target/release/mshogi-match')
    args = parser.parse_args()
    if args.command == 'worker':
        worker(args.manifest, args.store)
    elif args.command == 'submit':
        submit(args)
    else:
        anchor_cloud.save(args.out, summarize_runs(args.runs, args.replay_runner))


if __name__ == '__main__':
    main()
