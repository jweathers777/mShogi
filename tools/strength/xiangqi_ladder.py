"""Xiangqi reference ladder (MSH-197, Xiangqi Strength Anchor).

A frozen set of players (mShogi and independent engines, each with exact
options and wire format) plays a schedule of paired games through the native
clocked route (`mshogi-match xiangqi-match`) on Google Cloud Batch. Native
rules decide every move and result; unscored stops (clock exhaustion,
failures, pending adjudication) are recorded per engine and carry no score.

`fit` estimates relative ratings of all players together (Bradley-Terry
maximum likelihood on game scores, draws as half points, one player fixed at
0) with a pair-cluster bootstrap interval. No rating on a human or published
scale is claimed.

Commands: manifest, submit, worker (Batch), summary, fit.
"""
import argparse
import concurrent.futures
import gzip
import json
import math
import random
import re
import subprocess
import time
from pathlib import Path

from tools.strength.xiangqi_games import IO, PROFILE, WATCHDOGS

PREFIX = 'msh197'
CAMPAIGN_SECONDS = 6 * 3600
COMPUTE_USD = 15


def policy():
    from tools.strength import anchor_cloud
    return anchor_cloud.CampaignPolicy(PREFIX, CAMPAIGN_SECONDS, COMPUTE_USD, 'tools.strength.xiangqi_ladder',
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
    require(set(p) == {'base_ms', 'increment_ms', 'concurrency', 'pair_ms', 'task_seconds'}, 'protocol fields')
    require(all(type(v) is int for v in p.values()), 'integer protocol values')
    require(1000 <= p['base_ms'] <= 600000 and 0 <= p['increment_ms'] <= 10000 and 1 <= p['concurrency'] <= 4
            and 60000 <= p['pair_ms'] <= 7200000 and p['task_seconds'] <= CAMPAIGN_SECONDS, 'protocol bounds')
    require(digest(encode(m['openings'])) == m['openings_sha256'], 'opening catalog hash differs')
    players = m.get('players', {})
    require(2 <= len(players) <= 12 and all(re.fullmatch(r'[a-z0-9-]{1,32}', k) for k in players), 'players')
    for pl in players.values():
        require(Path(pl['engine']['path']).is_absolute() and re.fullmatch('[a-f0-9]{64}', pl['sha256']), 'player')
        for f in pl.get('files', []):
            require(Path(f['path']).is_absolute() and re.fullmatch('[a-f0-9]{64}', f['sha256']), 'player file')
    require(set(m['build']) >= {'runner', 'runtime_image'}, 'build')
    require(re.fullmatch(r'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/[^@]+@sha256:[a-f0-9]{64}',
                         m['build']['runtime_image']) is not None, 'frozen worker image')
    schedule = {s['id']: s for s in m['schedule']}
    require(len(schedule) == len(m['schedule']), 'unique schedule ids')
    for s in m['schedule']:
        require(len(s['players']) == 2 and all(x in players for x in s['players'])
                and s['players'][0] != s['players'][1] and 0 <= s['opening'] < len(m['openings']['openings']),
                'schedule entry')
    bins = m.get('bins')
    require(isinstance(bins, list) and 1 <= len(bins) <= 8 and all(b for b in bins), 'bins')
    flat = [i for b in bins for i in b]
    require(sorted(flat) == sorted(schedule) and len(flat) == len(set(flat)), 'bins cover the schedule once')


def game_config(m, entry, red):
    o = m['openings']['openings'][entry['opening']]
    p = m['protocol']
    a, b = (m['players'][x] for x in entry['players'])
    return {'schema_version': 1, 'profile': PROFILE, 'id': f"{entry['id']}-red{red}"[:128],
            'engines': [a['engine'], b['engine']], 'wire': [a.get('wire', {}), b.get('wire', {})],
            'opening': {'id': o['id'], 'fen': o.get('fen'), 'canonical': None, 'moves': o['moves']},
            'red': red, 'clock': {'base_ms': p['base_ms'], 'increment_ms': p['increment_ms']},
            'grace_ms': 1000, 'max_game_ms': p['pair_ms'] // 2, 'watchdogs': WATCHDOGS, 'io': IO}


def cloud_pair(m, entry, directory):
    from tools.experiments.core import digest, encode, file_hash
    directory.mkdir()
    receipt = {'id': entry['id'], 'players': entry['players'], 'complete': False, 'started_epoch': time.time(),
               'manifest_sha256': digest(encode(m)), 'games': []}
    try:
        for name in entry['players']:
            pl = m['players'][name]
            if file_hash(pl['engine']['path']) != pl['sha256']:
                raise RuntimeError('binary hash changed: ' + name)
            for f in pl.get('files', []):
                if file_hash(f['path']) != f['sha256']:
                    raise RuntimeError('player file changed: ' + name)
        for red in (0, 1):
            config = game_config(m, entry, red)
            cpath = directory / f'red{red}.config.json'
            cpath.write_text(json.dumps(config))
            rpath = directory / f'red{red}.report.json'
            done = subprocess.run([m['build']['runner']['path'], 'xiangqi-match', str(cpath), str(rpath)],
                                  capture_output=True, text=True, timeout=config['max_game_ms'] / 1000 + 300)
            row = {'red': red, 'returncode': done.returncode, 'stdout': done.stdout[-500:],
                   'stderr': done.stderr[-500:]}
            if done.returncode == 0:
                raw = rpath.read_bytes()
                report = json.loads(raw)
                row.update(stop=report['stop'], engine_score=report['engine_score'], plies=len(report['turns']),
                           exhaustion=report['exhaustion'], failure=report['failure'])
                Path(str(rpath) + '.gz').write_bytes(gzip.compress(raw, mtime=0))
                rpath.unlink()
            receipt['games'].append(row)
            cpath.unlink()
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
    schedule = {s['id']: s for s in m['schedule']}
    try:
        receipt['environment'] = cloud_environment()
        end = min(m['deadline_epoch'] - 300, time.time() + p['task_seconds'] - 120)

        def one(entry_id, directory):
            if time.time() + p['pair_ms'] / 1000 + 60 >= end:
                return {'id': entry_id, 'complete': False, 'skipped': 'time'}
            dest = Path(directory) / entry_id
            result = cloud_pair(m, schedule[entry_id], dest)
            for path in sorted(f for f in dest.iterdir() if f.name != 'receipt.json') + [dest / 'receipt.json']:
                put_new(store, f'pairs/{entry_id}/' + path.name, path.read_bytes())
            for path in dest.iterdir():
                path.unlink()
            dest.rmdir()
            return {'id': entry_id, 'complete': result['complete']}

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
    """Frozen manifest from a ladder definition (players, pairings, pairs per pairing)."""
    import hashlib
    from tools.experiments.core import digest, encode
    ladder = json.loads(Path(args.ladder).read_text())
    catalog = json.loads(Path(args.openings).read_text())
    local = Path(args.engines_dir)
    h = lambda name: hashlib.sha256((local / name).read_bytes()).hexdigest()
    players = {}
    for name, pl in ladder['players'].items():
        entry = {'engine': pl['engine'], 'wire': pl.get('wire', {}), 'sha256': h(pl['binary']),
                 'source': pl['source'], 'files': [{'path': f'/opt/engines/{f}', 'sha256': h(f)}
                                                    for f in pl.get('files', [])]}
        players[name] = entry
    schedule = []
    count = len(catalog['openings'])
    rng = random.Random(197)
    for pairing in ladder['pairings']:
        a, b, pairs = pairing['players'][0], pairing['players'][1], pairing['pairs']
        start = rng.randrange(count)
        for k in range(pairs):
            schedule.append({'id': f'{a}-{b}-{k + 1:03d}', 'players': [a, b], 'opening': (start + k) % count})
    order = [s['id'] for s in schedule]
    rng.shuffle(order)
    start = time.time()
    m = {'schema': 1, 'experiment': args.experiment, 'campaign_started_epoch': start,
         'deadline_epoch': start + CAMPAIGN_SECONDS, 'estimated_compute_before_usd': args.before,
         'build': {'runner': {'path': '/opt/engines/mshogi-match', 'sha256': h('mshogi-match')},
                   'runtime_image': args.image},
         'players': players, 'openings': catalog, 'openings_sha256': digest(encode(catalog)),
         'schedule': schedule, 'bins': [order[i::8] for i in range(8)],
         'protocol': {'base_ms': ladder['base_ms'], 'increment_ms': ladder['increment_ms'], 'concurrency': 4,
                      'pair_ms': ladder['pair_ms'], 'task_seconds': args.task_seconds}}
    validate_manifest(m)
    Path(args.out).write_text(json.dumps(m))
    print(len(schedule), 'pairs')


def load_games(run_dirs):
    """Scored and unscored games from retrieved runs (receipts)."""
    games = []
    for run in run_dirs:
        for path in sorted(Path(run).glob('raw/pairs/*/receipt.json')):
            r = json.loads(path.read_text())
            for g in r['games']:
                a, b = r['players']
                games.append({'pair': r['id'], 'a': a, 'b': b, 'red': g['red'],
                              'score': (g['engine_score'][0] / 2) if g.get('engine_score') else None,
                              'stop': g.get('stop'), 'exhaustion': g.get('exhaustion'),
                              'failure': (g.get('failure') or {}).get('kind') if g.get('failure') else None,
                              'returncode': g['returncode']})
    return games


def bradley_terry(games, players, anchor, iterations=2000):
    """Maximum-likelihood Elo ratings from scored games (draws count half);
    `anchor` is fixed at 0. Minorization-maximization on strengths."""
    strength = {p: 1.0 for p in players}
    wins = {p: 0.0 for p in players}
    pairs = {}
    for g in games:
        wins[g['a']] += g['score']
        wins[g['b']] += 1 - g['score']
        key = tuple(sorted((g['a'], g['b'])))
        pairs[key] = pairs.get(key, 0) + 1
    for _ in range(iterations):
        new = {}
        for p in players:
            denom = sum(n / (strength[p] + strength[q]) for (x, y), n in pairs.items() for q in [y if x == p else x]
                        if p in (x, y))
            new[p] = max(wins[p], 1e-3) / denom if denom else strength[p]
        scale = new[anchor]
        strength = {p: v / scale for p, v in new.items()}
    return {p: 400 * math.log10(v) for p, v in strength.items()}


def fit(args):
    games = [g for g in load_games(args.runs) if g['score'] is not None]
    players = sorted({g['a'] for g in games} | {g['b'] for g in games})
    point = bradley_terry(games, players, args.anchor)
    by_pair = {}
    for g in games:
        by_pair.setdefault(g['pair'], []).append(g)
    keys = sorted(by_pair)
    rng = random.Random(197)
    samples = {p: [] for p in players}
    for _ in range(args.bootstrap):
        draw = [g for _ in keys for g in by_pair[keys[rng.randrange(len(keys))]]]
        if {g['a'] for g in draw} | {g['b'] for g in draw} != set(players):
            continue
        r = bradley_terry(draw, players, args.anchor, iterations=300)
        for p in players:
            samples[p].append(r[p])
    result = {}
    for p in players:
        s = sorted(samples[p])
        result[p] = {'elo': round(point[p], 1),
                     'low': round(s[int(0.025 * len(s))], 1) if s else None,
                     'high': round(s[int(0.975 * len(s)) - 1], 1) if s else None}
    print(json.dumps({'anchor': args.anchor, 'games': len(games), 'bootstrap': len(samples[players[0]]),
                      'ratings': result}, indent=1))


def summary(args):
    games = load_games(args.runs)
    table = {}
    for g in games:
        for me, other, s in ((g['a'], g['b'], g['score']), (g['b'], g['a'], None if g['score'] is None else 1 - g['score'])):
            t = table.setdefault(me, {}).setdefault(other, {'points': 0.0, 'scored': 0, 'unscored': 0})
            if s is None:
                t['unscored'] += 1
            else:
                t['points'] += s
                t['scored'] += 1
    unscored = {}
    for g in games:
        if g['score'] is None:
            e = g['exhaustion']
            who = None
            if e:
                who = g['a'] if e['engine'] == 0 else g['b']
            unscored.setdefault(who or 'other', []).append({'pair': g['pair'], 'stop': g['stop'],
                                                            'failure': g['failure']})
    print(json.dumps({'games': len(games), 'table': table,
                      'unscored_by_engine': {k: len(v) for k, v in unscored.items()},
                      'unscored_examples': {k: v[:3] for k, v in unscored.items()}}, indent=1))


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
    for name in ('ladder', 'engines-dir', 'image', 'experiment', 'openings', 'out'):
        k.add_argument('--' + name, required=True)
    k.add_argument('--task-seconds', type=int, default=18000)
    k.add_argument('--before', type=float, default=0)
    m = sub.add_parser('summary')
    m.add_argument('runs', nargs='+')
    f = sub.add_parser('fit')
    f.add_argument('runs', nargs='+')
    f.add_argument('--anchor', required=True)
    f.add_argument('--bootstrap', type=int, default=500)
    a = p.parse_args()
    {'worker': lambda: worker(a.manifest, a.store), 'submit': lambda: submit(a),
     'manifest': lambda: make_manifest(a), 'summary': lambda: summary(a), 'fit': lambda: fit(a)}[a.command]()


if __name__ == '__main__':
    main()
