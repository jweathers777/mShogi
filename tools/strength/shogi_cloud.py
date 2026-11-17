"""Bounded MSH-187 native predecessor pairs on the existing Batch/GCS path.

No cloud action occurs on import. `submit` runs one reviewed batch; it never
launches a next batch. `worker` keeps immutable claims/results and uses the
existing native paired runner, replay and statistics. No external engines.
"""
import argparse
import gzip
import json
import math
import os
import io
import subprocess
from pathlib import Path
import re
import resource
import tempfile
import time
from types import SimpleNamespace

from tools.experiments import gcp
from tools.experiments.core import decode, digest, encode, file_hash
from tools.strength import anchor_cloud, native_match, stats
from tools.strength.anchor_worker import cloud_environment, put_new, require

POLICY = anchor_cloud.CampaignPolicy('msh187', 3600, 2, 'tools.strength.shogi_cloud', ('worker',))
PREDECESSOR = '1d19a544cc3e1ca3e92a0e8f53268758967329bf'
RATE = .201608
MAX_PAIRS = 600
PAIR_MS = 180000


def seeds(manifest):
    return sorted(n for rows in manifest['bins'] for n in rows)


def validate_manifest(m):
    require(m.get('schema') == 1 and re.fullmatch(r'msh187-[a-z0-9-]{1,60}', m.get('experiment', '')),
            'wrong Shogi campaign identity')
    for field in ('campaign_started_epoch', 'deadline_epoch', 'estimated_compute_before_usd'):
        require(type(m.get(field)) in (int, float) and math.isfinite(m[field]), 'finite ' + field)
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + POLICY.seconds, 'fixed campaign deadline required')
    require(0 <= m['estimated_compute_before_usd'] < POLICY.compute_usd, 'compute cap reached')
    require(m.get('protocol') == {'base_ms': 5000, 'increment_ms': 200, 'elo0': 0, 'elo1': 15,
                                 'alpha': .05, 'beta': .05, 'min_pairs': 30, 'max_pairs': MAX_PAIRS},
            'fixed reviewed comparison protocol required')
    bins = m.get('bins')
    require(isinstance(bins, list) and 1 <= len(bins) <= 8, 'one to eight bins')
    require(all(isinstance(b, list) and 1 <= len(b) <= 3 for b in bins), 'one to three pairs per VM')
    flat = [n for b in bins for n in b]
    require(all(type(n) is int and 1 <= n <= MAX_PAIRS for n in flat), 'invalid pair number')
    require(len(flat) == len(set(flat)) and seeds(m) == list(range(min(flat), max(flat)+1)),
            'unique contiguous pair numbers required')
    require(m.get('task_seconds') == 900, 'fixed 900-second task bound required')
    build = m['build']
    require(re.fullmatch('[a-f0-9]{40}', build['source']) is not None, 'frozen harness revision required')
    require(set(build['engines']) == {'candidate', 'baseline', 'runner', 'config_writer'}, 'four named native binaries required')
    for e in build['engines'].values():
        require(Path(e['path']).is_absolute() and re.fullmatch('[a-f0-9]{64}', e['sha256']) is not None,
                'absolute binary path and hash required')
        require(re.fullmatch('[a-f0-9]{40}', e['source']) is not None, 'frozen source required')
    require(build['engines']['baseline']['source'] == PREDECESSOR, 'wrong task-start predecessor')
    require(all(build['engines'][k]['source'] == build['source'] for k in ('runner', 'config_writer')),
            'runner/writer must match frozen harness source')
    opening_bytes = encode(m['openings'])
    require(digest(opening_bytes) == m['openings_sha256'], 'opening catalog hash differs')
    rows = m['openings']['openings']
    require(len(rows) == 6 and len({r['id'] for r in rows}) == 6, 'six frozen opening prefixes required')
    require(all(r.get('sfen') is None and isinstance(r['moves'], list) and 1 <= len(r['moves']) <= 32
                for r in rows), 'full standard-root prefixes required')


def identity(m):
    """Properties that cannot change between batches in one comparison."""
    return {k: m[k] for k in ('experiment', 'campaign_started_epoch', 'deadline_epoch',
                             'protocol', 'build', 'openings_sha256', 'task_seconds')}


def report_bytes(data):
    """Bound decompression by the native evidence ceiling."""
    with gzip.GzipFile(fileobj=io.BytesIO(data)) as stream:
        raw = stream.read(128 * 1024 * 1024 + 1)
    require(len(raw) <= 128 * 1024 * 1024, "native report exceeds evidence cap")
    return raw


def audit_report(report, config, manifest, seed):
    require(report['config'] == config and config['seed'] == seed, 'wrong effective pair config')
    require(config['clock'] == {'base_ms': 5000, 'increment_ms': 200} and
            config['max_steps'] == 320 and config['wall_budget_ms'] == PAIR_MS, 'wrong clock/pair bound')
    require(config['rounds'] == 1 and len(config['openings']) == 1 and len(config['players']) == 2,
            'expected one native opening pair')
    index = (seed-1) % 6
    selected = manifest['openings']['openings'][index]
    opening = config['openings'][0]
    require(opening['id'] == selected['id'], 'wrong opening assignment')
    require(opening['provenance'].startswith(f"Frozen catalog SHA256 {manifest['openings_sha256']}; index {index};"),
            'wrong opening catalog provenance')
    record = opening['root']['record']
    require(record['rules_profile'] == 'mshogi.shogi.csa36.v1' and record['game_kind'] == 'shogi' and
            opening['root']['chess_match'] is None, 'wrong opening native profile')
    native = record['payload']
    require(native['root']['category'] == 'standard' and native['root']['sfen'] ==
            'lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1' and
            native['root']['known_history'] == 'initial' and native['root']['counter_source'] == 'initial_fixed',
            'opening must retain the ordinary initial root')
    require([e['command'] for e in native['events']] ==
            [{'kind':'play','action':move} for move in selected['moves']], 'opening full prefix differs')
    env = report['environment']
    require(env['harness_revision'] == manifest['build']['source'] and
            env['harness_dirty_sha256'] == digest(b'') and env['os'] == 'linux' and env['arch'] == 'x86_64',
            'wrong or dirty native runner build')
    for index, name in enumerate(('candidate', 'baseline')):
        e = manifest['build']['engines'][name]
        require(config['players'][index]['executable'] == e['path'] and
                report['identities'][index]['executable']['sha256'] == e['sha256'], 'participant identity differs')
        require(config['players'][index]['limits'] == [{'profile': 'mshogi.shogi.csa36.v1',
                 'work': {'depth': 64, 'nodes': 5000000}}], 'asymmetric or wrong safety caps')
    result = native_match.summarize(report)
    require(all(g['stop'] == 'native_concluded' and g['native_outcome'] is not None
                for g in report['games']) and result['pair_score'] is not None, 'unfinished native pair')
    return result


def pair_args(m, opening_path):
    e = m['build']['engines']
    return SimpleNamespace(config_writer=e['config_writer']['path'], candidate=e['candidate']['path'],
        baseline=e['baseline']['path'], runner=e['runner']['path'], candidate_nodes=5000000,
        baseline_depth=64, baseline_nodes=5000000, calibrate=False, max_steps=320,
        pair_wall_ms=PAIR_MS, game='shogi', watchdog_ms=60000, base=5, increment=.2,
        openings=str(opening_path), opening_count=6)


def execute_pair(m, seed, directory, task_index=None):
    """Always retain a receipt and the native runner's artifacts on failure."""
    directory.mkdir()
    opening_path = directory/'openings.json'
    opening_path.write_bytes(encode(m['openings']))
    receipt = {'seed': seed, 'task_index': task_index, 'manifest_sha256': digest(encode(m)),
               'complete': False, 'score': None, 'started_epoch': time.time()}
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    try:
        for name, engine in m['build']['engines'].items():
            require(file_hash(engine['path']) == engine['sha256'], 'binary hash changed: ' + name)
        result = native_match.run_pair(pair_args(m, opening_path), seed, directory)
        raw = report_bytes((directory/result['report']).read_bytes())
        cfg = decode((directory/f'pair-{seed:04d}.config.json').read_bytes())
        checked = audit_report(decode(raw), cfg, m, seed)
        require(checked['pair_score'] == result['pair_score'] and digest(raw) == result['report_sha256'],
                'pair summary differs from native result')
        receipt.update(complete=True, score=result['pair_score'], summary=result)
    except Exception as error:
        receipt['error'] = repr(error)
    finally:
        end_usage = resource.getrusage(resource.RUSAGE_CHILDREN)
        receipt.update(ended_epoch=time.time(), cpu_seconds=end_usage.ru_utime+end_usage.ru_stime
                       -usage.ru_utime-usage.ru_stime)
        receipt['artifacts'] = {p.name: {'sha256': file_hash(p), 'bytes': p.stat().st_size}
                               for p in sorted(directory.iterdir()) if p.is_file()}
        (directory/'receipt.json').write_bytes(encode(receipt))
    return receipt


def worker(manifest_uri, store_uri):
    bucket, key = gcp.parse_uri(manifest_uri)
    raw = gcp.GCSStore('gs://'+bucket).get(key)
    m = decode(raw); validate_manifest(m)
    store = gcp.GCSStore(store_uri)
    index = int(os.environ['BATCH_TASK_INDEX'])
    require(0 <= index < len(m['bins']), 'invalid task index')
    require(os.environ.get('BATCH_TASK_RETRY_ATTEMPT', '0') == '0', 'no worker retries')
    claim = {'manifest_sha256': digest(raw), 'task_index': index, 'started_epoch': time.time()}
    prefix = f'tasks/{index}'
    put_new(store, prefix+'/started.json', encode(claim))
    receipt = dict(claim, complete=False, pairs=[])
    try:
        receipt['environment'] = cloud_environment()
        require(receipt['environment'].get('gcp', {}).get('cpu-platform') == 'Intel Sapphire Rapids',
                'CPU platform differs from frozen comparison hardware')
        end = min(m['deadline_epoch']-300, time.time()+m['task_seconds']-120)
        with tempfile.TemporaryDirectory(prefix='msh187-') as directory:
            for seed in m['bins'][index]:
                require(not store.exists('abort.json'), 'another worker failed; no new pair')
                require(time.time()+330 < end, 'insufficient time for a bounded pair and upload')
                pair_prefix = f'pairs/{seed:04d}'
                put_new(store, pair_prefix+'/launch.json', encode(claim | {'seed': seed}))
                dest = Path(directory)/f'{seed:04d}'
                result = execute_pair(m, seed, dest, index)
                files = sorted(p for p in dest.iterdir() if p.name != 'receipt.json') + [dest/'receipt.json']
                for path in files:
                    put_new(store, pair_prefix+'/'+path.name, path.read_bytes())
                receipt['pairs'].append({'seed': seed, 'complete': result['complete']})
                require(result['complete'], 'unscored pair; stop worker')
        receipt['complete'] = True
    except Exception as error:
        receipt['error'] = repr(error)
        # A shared immutable stop marker prevents queued work after any failure.
        store.put('abort.json', encode(claim | {'error': repr(error)}))
        raise
    finally:
        receipt['ended_epoch'] = time.time()
        put_new(store, prefix+'/receipt.json', encode(receipt))


def summarize_runs(paths, replay_runner=None):
    """Audit retrieved receipts and replay through the local native authority."""
    require(not paths or replay_runner is not None, "local native replay runner required")
    rows = []; manifests = []; upper = 0.; failure = []
    for name in paths:
        root = Path(name)
        m = decode((root/'manifest.json').read_bytes()); validate_manifest(m)
        if manifests:
            require(identity(m) == identity(manifests[0]), 'comparison identity changed')
        manifests.append(m)
        expected_hash = digest(encode(m))
        intent = decode((root/'intent.json').read_bytes())
        require(intent['manifest_sha256'] == expected_hash, 'resource intent belongs to another manifest')
        cleanup = decode((root/'cleanup.json').read_bytes())
        require(cleanup.get('verified_absent') is True, 'cleanup not verified')
        resources = decode((root/'resources.json').read_bytes())
        require(resources['started_epoch'] == intent['started_epoch'] and
                resources.get('cleanup_error') is None and resources['rate_usd_vm_hour'] == RATE,
                'wrong resource start/rate or cleanup uncertainty')
        start = resources['started_epoch']; end = resources['allocation_end_epoch']
        require(m['campaign_started_epoch'] <= start <= cleanup['at_epoch'] <= end <= resources['ended_epoch']
                and end <= m['deadline_epoch'], 'resource interval outside campaign or verified cleanup')
        hours = 8 * (end-start) / 3600
        cost = hours * RATE
        require(math.isclose(resources['conservative_vm_hours_upper_bound'], hours, rel_tol=1e-12, abs_tol=1e-12)
                and math.isclose(resources['conservative_compute_usd_upper_bound'], cost, rel_tol=1e-12, abs_tol=1e-12),
                'resource upper bound differs from observed interval')
        upper += cost
        for i, assigned in enumerate(m['bins']):
            task_folder = root/'raw'/'tasks'/str(i)
            started = task_folder/'started.json'
            if started.exists():
                claim = decode(started.read_bytes())
                require(claim['manifest_sha256'] == expected_hash and claim['task_index'] == i, 'wrong task launch claim')
            task = task_folder/'receipt.json'
            if task.exists():
                task_receipt = decode(task.read_bytes())
                require(started.exists() and all(task_receipt[k] == claim[k]
                        for k in ('manifest_sha256','task_index','started_epoch')), 'wrong task receipt claim')
                hardware = task_receipt.get('environment', {}).get('gcp', {})
                require(hardware.get('cpu-platform') == 'Intel Sapphire Rapids' and
                        hardware.get('machine-type', '').endswith('/c3-standard-4') and
                        hardware.get('zone', '').rsplit('/',1)[-1].startswith('us-central1-') and hardware.get('id'),
                        'missing or different worker hardware receipt')
                attempted = [r['seed'] for r in task_receipt['pairs']]
                require(attempted == assigned[:len(attempted)] and
                        (not task_receipt['complete'] or attempted == assigned), 'task completed pairs differ from bin')
                for pair in task_receipt['pairs']:
                    bound = decode((root/'raw'/'pairs'/f"{pair['seed']:04d}"/'receipt.json').read_bytes())
                    require(bound['complete'] == pair['complete'], 'task/pair completion mismatch')
        for n in seeds(m):
            folder = root/'raw'/'pairs'/f'{n:04d}'
            if not (folder/'receipt.json').exists():
                failure.append({'seed': n, 'reason': 'missing or unstarted pair receipt'}); continue
            receipt = decode((folder/'receipt.json').read_bytes())
            index = next(i for i,b in enumerate(m['bins']) if n in b)
            claim = decode((root/'raw'/'tasks'/str(index)/'started.json').read_bytes())
            launch = decode((folder/'launch.json').read_bytes())
            require(launch == claim | {'seed':n}, 'wrong pair launch claim')
            require(receipt['seed'] == n and receipt['task_index'] == index and
                    receipt['manifest_sha256'] == expected_hash, 'wrong pair receipt identity')
            require(claim['started_epoch'] <= receipt['started_epoch'] <= receipt['ended_epoch'] <= m['deadline_epoch'],
                    'pair receipt time outside launch/campaign')
            for file, expected in receipt['artifacts'].items():
                gcp._key(file)
                require('/' not in file and (folder/file).stat().st_size == expected['bytes'] and
                        file_hash(folder/file) == expected['sha256'], 'artifact integrity differs')
            if not receipt['complete']:
                failure.append({'seed': n, 'reason': receipt.get('error', 'unscored pair')}); continue
            summary = receipt['summary']
            required = {f'pair-{n:04d}.config.json', f'pair-{n:04d}.report.json.gz',
                        f'pair-{n:04d}.replay.log', f'pair-{n:04d}.summary.json', 'openings.json'}
            require(required <= set(receipt['artifacts']), 'successful pair lacks required evidence')
            require((folder/f'pair-{n:04d}.replay.log').read_text().strip() ==
                    'replayed 2 native experiment games', 'missing worker native replay acknowledgement')
            require((folder/'openings.json').read_bytes() == encode(m['openings']), 'frozen opening bytes differ')
            require(summary['report'] == f'pair-{n:04d}.report.json.gz', 'wrong report artifact path')
            raw = report_bytes((folder/summary['report']).read_bytes())
            require(digest(raw) == summary['report_sha256'], 'native report hash differs')
            with tempfile.NamedTemporaryFile(suffix='.json') as replay_file:
                replay_file.write(raw); replay_file.flush()
                subprocess.run([str(replay_runner), 'experiment-replay', replay_file.name],
                               check=True, capture_output=True, timeout=60)
            cfg = decode((folder/f'pair-{n:04d}.config.json').read_bytes())
            checked = audit_report(decode(raw), cfg, m, n)
            require(checked['pair_score'] == receipt['score'], 'receipt score differs')
            rows.append({'seed': n, 'pair_score': checked['pair_score'], 'scores_twice': checked['scores_twice']})
        if any((root/f).exists() for f in ('failure.json', 'retrieval-error.json')):
            failure.append({'batch': str(root), 'reason': 'controller or retrieval failure'})
        for i in range(len(m['bins'])):
            task = root/'raw'/'tasks'/str(i)/'receipt.json'
            if not task.exists() or not decode(task.read_bytes()).get('complete'):
                failure.append({'task': str(task), 'reason': 'worker incomplete'})
    flat = [n for m in manifests for n in seeds(m)]
    require(flat == list(range(1, len(flat)+1)), 'prior runs must be ordered, complete and nonduplicated')
    counts = stats.pentanomial(r['pair_score'] for r in sorted(rows,key=lambda r:r['seed']))
    scores = [s for r in rows for s in r['scores_twice']]
    return {'pairs_attempted': len(flat), 'scored_pairs': len(rows), 'pentanomial': counts,
            'wins_draws_losses': [scores.count(2),scores.count(1),scores.count(0)],
            'sprt': stats.Sprt().status(counts), 'estimate': stats.estimate(counts),
            'failures': failure, 'conservative_compute_usd_upper_bound': upper, 'rows': rows}


def submit(args):
    m = decode(Path(args.manifest).read_bytes()); validate_manifest(m)
    previous = summarize_runs(args.previous, args.replay_runner)
    require(not previous['failures'] and previous['sprt']['decision'] is None, 'campaign already stopped')
    require(seeds(m)[0] == previous['pairs_attempted']+1, 'next batch skips/repeats pair numbers')
    require(m['estimated_compute_before_usd'] == previous['conservative_compute_usd_upper_bound'], 'wrong prior cost')
    for root in args.previous:
        require(identity(decode((Path(root)/'manifest.json').read_bytes())) == identity(m), 'changed campaign identity')
    config = gcp.validate_config(decode(Path(args.config).read_bytes()))
    require(config['max_run_seconds'] == m['task_seconds'] and config.get('vm_hourly_usd') == RATE,
            'fixed task limit and reviewed compute rate required')
    require(config['store_uri'] == 'gs://mshogi-experiments-216675633801/msh187/' +
            m['experiment'] + f'/batches/{seeds(m)[0]:04d}', 'canonical immutable batch prefix required')
    require(config['service_account'] == 'mshogi-batch-worker@mshogi.iam.gserviceaccount.com' and
            config.get('notification_topic') == 'projects/mshogi/topics/mshogi-batch-events' and
            config.get('network') == 'projects/mshogi/global/networks/mshogi-batch' and
            config.get('subnetwork') == 'projects/mshogi/regions/us-central1/subnetworks/mshogi-batch-us-central1',
            'reuse the configured service account, events and network')
    require(args.subscription == 'projects/mshogi/subscriptions/mshogi-pilot-completion', 'reuse completion subscription')
    require(args.image.startswith('us-central1-docker.pkg.dev/mshogi/mshogi-experiments/'), 'wrong existing registry')
    for root in args.previous:
        require(decode((Path(root)/'intent.json').read_bytes())['image'] == args.image, 'image changed')
    anchor_cloud.run(args, policy=POLICY, manifest_validator=validate_manifest)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='command', required=True)
    w = sub.add_parser('worker'); w.add_argument('--manifest',required=True); w.add_argument('--store',required=True)
    s = sub.add_parser('submit')
    for name in ('manifest','config','image','job','out'): s.add_argument('--'+name,required=True)
    s.add_argument('--subscription',default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    s.add_argument('--previous', nargs='*', default=[])
    s.add_argument('--replay-runner', default='target/release/mshogi-match')
    a = sub.add_parser('summarize'); a.add_argument('runs', nargs='+'); a.add_argument('--out',required=True)
    a.add_argument('--replay-runner', default='target/release/mshogi-match')
    args = p.parse_args()
    if args.command == 'worker': worker(args.manifest,args.store)
    elif args.command == 'submit': submit(args)
    else: anchor_cloud.save(args.out,summarize_runs(args.runs,args.replay_runner))


if __name__ == '__main__':
    main()
