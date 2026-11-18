"""Native Shogi self-play for evaluation corpora (MSH-188, Shogi Evaluation Round).

One engine plays both colours of each opening pair through the native paired
runner; the native rules own every move, history and outcome, and each report
is replayed natively. Pair N uses opening (N-1) mod catalog size. Each pair
draws one fixed node budget per player from a seeded range, so repeated
openings diverge. Labels come only from native outcomes, never search scores.

Local: python3 -m tools.strength.shogi_selfplay --engine E --openings CATALOG \
    --pairs N --out DIR [--workers 8] [--nodes 30000 60000] [--first-seed 1]
Cloud: python3 -m tools.strength.shogi_selfplay submit --manifest M --config C \
    --image I --job msh188-... --out DIR   (the Batch worker runs `worker`)
"""
import argparse
import concurrent.futures
import hashlib
import json
import time
from pathlib import Path
from types import SimpleNamespace

from . import native_match
from .cli import engine_identity, environment, write_new


def budgets(seed, low, high):
    """Two reproducible per-player node budgets for one pair."""
    digest = hashlib.sha256(f'msh188-selfplay-{seed}'.encode()).digest()
    span = high - low + 1
    return (low + int.from_bytes(digest[:8], 'big') % span,
            low + int.from_bytes(digest[8:16], 'big') % span)


PREFIX = 'msh188'


def policy():
    from tools.strength import anchor_cloud
    # Campaign window and conservative compute cap (eight VMs over the window).
    return anchor_cloud.CampaignPolicy(PREFIX, 4 * 3600, 8, 'tools.strength.shogi_selfplay', ('worker',))


def validate_manifest(m):
    import math
    import re
    from tools.experiments.core import digest, encode
    from tools.strength.anchor_worker import require
    pol = policy()
    require(m.get('schema') == 1 and re.fullmatch(PREFIX + r'-[a-z0-9-]{1,50}', m.get('experiment', '')),
            'wrong campaign identity')
    for field in ('campaign_started_epoch', 'deadline_epoch', 'estimated_compute_before_usd'):
        require(type(m.get(field)) in (int, float) and math.isfinite(m[field]), 'finite ' + field)
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + pol.seconds, 'fixed campaign deadline')
    require(0 <= m['estimated_compute_before_usd'] < pol.compute_usd, 'compute cap reached')
    p = m.get('protocol', {})
    require(set(p) - {'random_plies'} == {'nodes', 'concurrency', 'pair_ms', 'task_seconds', 'every', 'max_steps'}
            and p.get('random_plies', 0) in (0, 1, 2, 3, 4), 'protocol fields')
    low, high = p['nodes']
    require(1000 <= low <= high <= 1_000_000 and 1 <= p['concurrency'] <= 4 and 1 <= p['every'] <= 16
            and p['max_steps'] == 320 and 60000 <= p['pair_ms'] <= 900000, 'protocol bounds')
    require(digest(encode(m['openings'])) == m['openings_sha256'], 'opening catalog hash differs')
    bins = m.get('bins')
    require(isinstance(bins, list) and 1 <= len(bins) <= 8 and all(isinstance(b, list) and b for b in bins),
            'one to eight nonempty bins')
    flat = sorted(n for b in bins for n in b)
    require(len(flat) == len(set(flat)) and all(type(n) is int and n >= 1 for n in flat), 'unique pair numbers')
    require(set(m['build']['engines']) == {'engine', 'runner', 'config_writer', 'helper'}, 'four binaries')
    for e in m['build']['engines'].values():
        require(Path(e['path']).is_absolute() and re.fullmatch('[a-f0-9]{64}', e['sha256']) is not None
                and re.fullmatch('[a-f0-9]{40}', e['source']) is not None, 'binary identity')
    require(re.fullmatch(r'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/[^@]+@sha256:[a-f0-9]{64}',
                         m['build'].get('runtime_image', '')) is not None, 'frozen worker image')


def cloud_pair(m, seed, directory):
    """One native pair plus native-replayed export; always a receipt."""
    import gzip
    import subprocess
    from tools.experiments.core import digest, encode, file_hash
    directory.mkdir()
    catalog = directory / 'openings.json'
    catalog.write_bytes(encode(m['openings']))
    e, p = m['build']['engines'], m['protocol']
    cand, base = budgets(seed, *p['nodes'])
    receipt = {'seed': seed, 'nodes': [cand, base], 'complete': False, 'started_epoch': time.time(),
               'manifest_sha256': digest(encode(m))}
    try:
        for name, engine in e.items():
            if file_hash(engine['path']) != engine['sha256']:
                raise RuntimeError('binary hash changed: ' + name)
        args = SimpleNamespace(config_writer=e['config_writer']['path'], candidate=e['engine']['path'],
                               baseline=e['engine']['path'], runner=e['runner']['path'], candidate_nodes=cand,
                               baseline_depth=64, baseline_nodes=base, calibrate=False,
                               max_steps=p['max_steps'], pair_wall_ms=p['pair_ms'], game='shogi',
                               watchdog_ms=None, openings=str(catalog),
                               opening_count=len(m['openings']['openings']),
                               random_plies=p.get('random_plies', 0))
        result = native_match.run_pair(args, seed, directory)
        raw = gzip.decompress((directory / result['report']).read_bytes())
        plain = directory / 'report.json'
        plain.write_bytes(raw)
        export = directory / f'pair-{seed:04d}.games.jsonl'
        subprocess.run([e['helper']['path'], 'export', str(plain), str(export), str(p['every'])],
                       check=True, capture_output=True, timeout=300)
        plain.unlink()
        receipt.update(complete=True, stops=result['stops'], plies=result['plies'],
                       scores_twice=result['scores_twice'], report_sha256=result['report_sha256'])
    except Exception as error:
        receipt['error'] = repr(error)
    finally:
        receipt['ended_epoch'] = time.time()
        receipt['artifacts'] = {f.name: {'sha256': file_hash(f), 'bytes': f.stat().st_size}
                                for f in sorted(directory.iterdir()) if f.is_file() and f.name != 'openings.json'}
        (directory / 'receipt.json').write_bytes(encode(receipt))
    return receipt


def worker(manifest_uri, store_uri):
    import os
    import tempfile
    from tools.experiments import gcp
    from tools.experiments.core import decode, digest, encode
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
    receipt = dict(claim, complete=False, pairs=[])
    p = m['protocol']
    try:
        receipt['environment'] = cloud_environment()
        end = min(m['deadline_epoch'] - 300, time.time() + p['task_seconds'] - 120)

        def one(seed, directory):
            # Self-play rows are independent: a failed pair is recorded, not fatal.
            if time.time() + p['pair_ms'] / 1000 + 60 >= end:
                return {'seed': seed, 'complete': False, 'skipped': 'time'}
            dest = Path(directory) / f'{seed:05d}'
            result = cloud_pair(m, seed, dest)
            for path in sorted(f for f in dest.iterdir() if f.name not in ('receipt.json', 'openings.json')) \
                    + [dest / 'receipt.json']:
                put_new(store, f'pairs/{seed:05d}/' + path.name, path.read_bytes())
            for path in dest.iterdir():
                path.unlink()
            dest.rmdir()
            return {'seed': seed, 'complete': result['complete']}

        with tempfile.TemporaryDirectory(prefix=PREFIX + '-') as directory:
            with concurrent.futures.ThreadPoolExecutor(p['concurrency']) as pool:
                for row in pool.map(lambda n: one(n, directory), m['bins'][index]):
                    receipt['pairs'].append(row)
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
    require(config['max_run_seconds'] == m['protocol']['task_seconds'], 'task limit differs')
    anchor_cloud.run(args, policy=policy(), manifest_validator=validate_manifest)


def cloud_main(argv):
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='command', required=True)
    w = sub.add_parser('worker')
    w.add_argument('--manifest', required=True)
    w.add_argument('--store', required=True)
    s = sub.add_parser('submit')
    for name in ('manifest', 'config', 'image', 'job', 'out'):
        s.add_argument('--' + name, required=True)
    s.add_argument('--subscription', default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    a = p.parse_args(argv)
    if a.command == 'worker':
        worker(a.manifest, a.store)
    else:
        submit(a)


def main():
    import sys
    if len(sys.argv) > 1 and sys.argv[1] in ('worker', 'submit'):
        return cloud_main(sys.argv[1:])
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--engine', required=True)
    p.add_argument('--openings', required=True)
    p.add_argument('--pairs', type=int, required=True)
    p.add_argument('--out', required=True)
    p.add_argument('--workers', type=int, default=8)
    p.add_argument('--nodes', type=int, nargs=2, default=[6000, 16000])
    p.add_argument('--first-seed', type=int, default=1)
    p.add_argument('--runner', default='target/release/mshogi-match')
    p.add_argument('--config-writer', default='target/release/examples/shogi_match_config')
    p.add_argument('--pair-wall-ms', type=int, default=300000)
    a = p.parse_args()
    low, high = a.nodes
    if not (1 <= a.pairs <= 20000 and 1 <= a.workers <= 16 and 1000 <= low <= high <= 1_000_000):
        p.error('invalid bounds')
    out = Path(a.out).resolve()
    out.mkdir(parents=True, exist_ok=False)
    catalog = Path(a.openings).resolve()
    count = len(json.loads(catalog.read_text())['openings'])
    write_new(out / 'generation.json', {
        'kind': 'shogi-selfplay-generation', 'engine': engine_identity(a.engine),
        'openings': str(catalog), 'openings_count': count, 'pairs': a.pairs,
        'first_seed': a.first_seed, 'node_range': [low, high], 'workers': a.workers,
        'max_steps': 320, 'environment': environment(), 'started_epoch': time.time()})

    def one(seed):
        cand, base = budgets(seed, low, high)
        args = SimpleNamespace(config_writer=a.config_writer, candidate=a.engine, baseline=a.engine,
                               runner=a.runner, candidate_nodes=cand, baseline_depth=64,
                               baseline_nodes=base, calibrate=False, max_steps=320,
                               pair_wall_ms=a.pair_wall_ms, game='shogi', watchdog_ms=None,
                               openings=str(catalog), opening_count=count)
        try:
            r = native_match.run_pair(args, seed, out)
            return {'seed': seed, 'nodes': [cand, base], 'scores_twice': r['scores_twice'],
                    'stops': r['stops'], 'plies': r['plies'], 'wall_seconds': r['wall_seconds']}
        except Exception as error:
            return {'seed': seed, 'nodes': [cand, base], 'error': repr(error)}

    start = time.monotonic()
    rows = []
    with concurrent.futures.ThreadPoolExecutor(a.workers) as pool:
        for row in pool.map(one, range(a.first_seed, a.first_seed + a.pairs)):
            rows.append(row)
    write_new(out / 'pairs.json', {'rows': rows, 'wall_seconds': time.monotonic() - start,
                                  'failures': [r for r in rows if 'error' in r]})
    print(json.dumps({'pairs': len(rows), 'failures': sum('error' in r for r in rows),
                      'wall_seconds': time.monotonic() - start}))


if __name__ == '__main__':
    main()
