"""Native Xiangqi self-play for evaluation corpora (MSH-196, Xiangqi Evaluation Round).

One frozen engine plays both colours of each opening pair through the clocked
native route (`mshogi-match xiangqi-match`): persistent UCI processes,
controller Fischer clocks, native admission and native replay of every report.
Pair N uses opening (N-1) mod catalog size; timing makes repeated openings
diverge. Each report is exported with the native Red result for every
`every`-th position (`xiangqi_strength export`). Labels come only from native
outcomes, never search scores. Unscored games give no labels.

Cloud: python3 -m tools.strength.xiangqi_selfplay submit --manifest M --config C \
    --image I --job msh196-... --out DIR   (the Batch worker runs `worker`)
"""
import argparse
import concurrent.futures
import gzip
import json
import math
import re
import subprocess
import time
from pathlib import Path

from tools.strength.xiangqi_games import IO, PROFILE, WATCHDOGS

PREFIX = 'msh196'
CAMPAIGN_SECONDS = 3 * 3600
COMPUTE_USD = 5


def policy():
    from tools.strength import anchor_cloud
    return anchor_cloud.CampaignPolicy(PREFIX, CAMPAIGN_SECONDS, COMPUTE_USD, 'tools.strength.xiangqi_selfplay',
                                       ('worker',))


def validate_manifest(m):
    from tools.experiments.core import digest, encode
    from tools.strength.anchor_worker import require
    require(m.get('schema') == 1 and re.fullmatch(PREFIX + r'-[a-z0-9-]{1,50}', m.get('experiment', '')),
            'wrong campaign identity')
    for field in ('campaign_started_epoch', 'deadline_epoch', 'estimated_compute_before_usd'):
        require(type(m.get(field)) in (int, float) and math.isfinite(m[field]), 'finite ' + field)
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + CAMPAIGN_SECONDS, 'fixed campaign deadline')
    require(0 <= m['estimated_compute_before_usd'] < COMPUTE_USD, 'compute cap reached')
    p = m.get('protocol', {})
    require(set(p) == {'base_ms', 'increment_ms', 'concurrency', 'pair_ms', 'task_seconds', 'every'},
            'protocol fields')
    require(1000 <= p['base_ms'] <= 60000 and 0 <= p['increment_ms'] <= 5000 and 1 <= p['concurrency'] <= 4
            and 1 <= p['every'] <= 16 and 60000 <= p['pair_ms'] <= 900000 and p['task_seconds'] <= CAMPAIGN_SECONDS,
            'protocol bounds')
    require(digest(encode(m['openings'])) == m['openings_sha256'], 'opening catalog hash differs')
    bins = m.get('bins')
    require(isinstance(bins, list) and 1 <= len(bins) <= 8 and all(isinstance(b, list) and b for b in bins),
            'one to eight nonempty bins')
    flat = sorted(n for b in bins for n in b)
    require(len(flat) == len(set(flat)) and all(type(n) is int and n >= 1 for n in flat), 'unique pair numbers')
    require(set(m['build']['engines']) == {'engine', 'runner', 'helper'}, 'three binaries')
    for e in m['build']['engines'].values():
        require(Path(e['path']).is_absolute() and re.fullmatch('[a-f0-9]{64}', e['sha256']) is not None
                and re.fullmatch('[a-f0-9]{40}', e['source']) is not None, 'binary identity')
    require(re.fullmatch(r'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/[^@]+@sha256:[a-f0-9]{64}',
                         m['build'].get('runtime_image', '')) is not None, 'frozen worker image')


def game_config(m, seed, red):
    catalog = m['openings']['openings']
    o = catalog[(seed - 1) % len(catalog)]
    p = m['protocol']
    engine = {'path': m['build']['engines']['engine']['path'], 'args': ['--game', 'xiangqi', 'uci'],
              'options': [], 'model_path': None, 'model_identity': 'self-play', 'deterministic': False}
    return {'schema_version': 1, 'profile': PROFILE, 'id': f'pair{seed:05d}-red{red}',
            'engines': [engine, engine],
            'opening': {'id': o['id'], 'fen': o.get('fen'), 'canonical': None, 'moves': o['moves']},
            'red': red, 'clock': {'base_ms': p['base_ms'], 'increment_ms': p['increment_ms']},
            'grace_ms': 500, 'max_game_ms': p['pair_ms'] // 2, 'watchdogs': WATCHDOGS, 'io': IO}


def cloud_pair(m, seed, directory):
    """One native pair plus native-replayed export; always a receipt."""
    from tools.experiments.core import digest, encode, file_hash
    directory.mkdir()
    e = m['build']['engines']
    receipt = {'seed': seed, 'complete': False, 'started_epoch': time.time(),
               'manifest_sha256': digest(encode(m)), 'games': []}
    try:
        for name, engine in e.items():
            if file_hash(engine['path']) != engine['sha256']:
                raise RuntimeError('binary hash changed: ' + name)
        reports = []
        for red in (0, 1):
            config = game_config(m, seed, red)
            cpath = directory / f'red{red}.config.json'
            cpath.write_text(json.dumps(config))
            rpath = directory / f'red{red}.report.json'
            done = subprocess.run([e['runner']['path'], 'xiangqi-match', str(cpath), str(rpath)],
                                  capture_output=True, text=True, timeout=config['max_game_ms'] / 1000 + 120)
            row = {'red': red, 'returncode': done.returncode, 'stdout': done.stdout[-500:],
                   'stderr': done.stderr[-500:]}
            if done.returncode == 0:
                reports.append(str(rpath))
            receipt['games'].append(row)
            cpath.unlink()
        if reports:
            listing = directory / 'reports.json'
            listing.write_text(json.dumps(reports))
            export = directory / f'pair-{seed:05d}.games.jsonl'
            subprocess.run([e['helper']['path'], 'export', str(listing), str(export), str(m['protocol']['every'])],
                           check=True, capture_output=True, timeout=300)
            listing.unlink()
            for r in reports:
                raw = Path(r).read_bytes()
                Path(r + '.gz').write_bytes(gzip.compress(raw, mtime=0))
                Path(r).unlink()
        receipt['complete'] = True
    except Exception as error:
        receipt['error'] = repr(error)
    finally:
        receipt['ended_epoch'] = time.time()
        receipt['artifacts'] = {f.name: {'sha256': file_hash(f), 'bytes': f.stat().st_size}
                                for f in sorted(directory.iterdir()) if f.is_file()}
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
            for path in sorted(f for f in dest.iterdir() if f.name != 'receipt.json') + [dest / 'receipt.json']:
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


def make_manifest(args):
    """Frozen manifest from built binaries (bin dir), image digest and bins."""
    import hashlib
    from tools.experiments.core import digest, encode
    catalog = json.loads(Path(args.openings).read_text())
    h = lambda n: hashlib.sha256((Path(args.bin_dir) / n).read_bytes()).hexdigest()
    source = (Path(args.bin_dir) / 'source.txt').read_text().strip()
    build = {'source': source, 'runtime_image': args.image,
             'compiler': (Path(args.bin_dir) / 'rustc.txt').read_text().strip() + '; -C target-cpu=x86-64; --locked release',
             'engines': {'engine': {'path': '/opt/engines/mshogi', 'sha256': h('mshogi'), 'source': source},
                         'runner': {'path': '/opt/engines/mshogi-match', 'sha256': h('mshogi-match'), 'source': source},
                         'helper': {'path': '/opt/engines/xiangqi_strength', 'sha256': h('xiangqi_strength'),
                                    'source': source}}}
    pairs = list(range(args.first_pair, args.first_pair + args.pairs))
    start = time.time()
    m = {'schema': 1, 'experiment': args.experiment, 'campaign_started_epoch': start,
         'deadline_epoch': start + CAMPAIGN_SECONDS, 'estimated_compute_before_usd': args.before,
         'build': build, 'openings': catalog, 'openings_sha256': digest(encode(catalog)),
         'bins': [pairs[i::8] for i in range(8)],
         'protocol': {'base_ms': args.base_ms, 'increment_ms': args.inc_ms, 'concurrency': 4,
                      'pair_ms': args.pair_ms, 'task_seconds': args.task_seconds, 'every': args.every}}
    validate_manifest(m)
    Path(args.out).write_text(json.dumps(m))
    print(len(pairs), 'pairs')


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
    for name in ('bin-dir', 'image', 'experiment', 'openings', 'out'):
        k.add_argument('--' + name, required=True)
    k.add_argument('--pairs', type=int, required=True)
    k.add_argument('--first-pair', type=int, default=1)
    k.add_argument('--base-ms', type=int, default=2000)
    k.add_argument('--inc-ms', type=int, default=50)
    k.add_argument('--pair-ms', type=int, default=240000)
    k.add_argument('--task-seconds', type=int, default=9000)
    k.add_argument('--every', type=int, default=1)
    k.add_argument('--before', type=float, default=0)
    a = p.parse_args()
    if a.command == 'worker':
        worker(a.manifest, a.store)
    elif a.command == 'submit':
        submit(a)
    else:
        make_manifest(a)


if __name__ == '__main__':
    main()
