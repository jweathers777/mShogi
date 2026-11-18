"""MSH-191 (Shogi Strength Anchor): clocked games against floodgate-rated engines.

`prepare` writes one batch manifest; `submit` runs it on the existing Google
Cloud Batch path (eight c3-standard-4 workers, one game at a time per VM);
`worker` runs on each VM; `summarize` audits retrieved batches, replays every
report with the local native runner and fits the joint estimate with
`anchor_stats` (MSH-185 method). The native runner (`mshogi-match shogi-match`)
owns legality, clocks and results; unscored games are retained as failures.
No cloud action occurs on import.
"""
import argparse
import gzip
import json
import math
import os
import re
import resource
import signal
import subprocess
import tempfile
import time
from pathlib import Path

from tools.experiments.core import decode, digest, encode, file_hash
from tools.strength import anchor_stats

PREFIX = 'msh191'
CAMPAIGN_SECONDS = 6 * 3600
COMPUTE_USD = 15
RATE = .201608
STORE = 'gs://mshogi-experiments-216675633801/msh191/'
CLOCK = {'base_ms': 60000, 'increment_ms': 1000}
# Reviewed clocks: the anchor control and the same 60:1 ratio for fast tests.
CLOCKS = ({'base_ms': 60000, 'increment_ms': 1000}, {'base_ms': 15000, 'increment_ms': 250})
GRACE_MS = 500
PROFILE = 'mshogi.shogi.csa36.v1'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def policy():
    from tools.strength import anchor_cloud
    return anchor_cloud.CampaignPolicy(PREFIX, CAMPAIGN_SECONDS, COMPUTE_USD, 'tools.strength.shogi_anchor',
                                       ('worker',))


def engine_config(build, name):
    entry = build['engines'][name]
    return {'path': entry['path'], 'args': entry.get('args', []), 'options': entry.get('options', []),
            'model_path': None, 'model_identity': name, 'deterministic': False}


def clock_of(m):
    return m.get('clock', CLOCK)


def match_config(build, spec, clock=CLOCK):
    """Native clocked-game config: engine 0 is mShogi, engine 1 the opponent."""
    return {'schema_version': 1, 'profile': PROFILE, 'id': spec['id'],
            'engines': [engine_config(build, 'candidate'), engine_config(build, spec['opponent'])],
            'opening': {'id': spec['opening']['id'], 'sfen': None, 'moves': spec['opening']['moves']},
            'sente': 1 if spec['swapped'] else 0, 'clock': dict(clock), 'grace_ms': GRACE_MS,
            'max_game_ms': 1_800_000,
            'watchdogs': {'handshake_ms': 120000, 'ready_ms': 120000, 'response_ms': 60000,
                          'write_ms': 2000, 'cleanup_ms': 5000},
            'io': {'line_bytes': 8192, 'queued_lines': 256, 'transcript_bytes': 4194304,
                   'stderr_bytes': 65536, 'command_bytes': 4096}}


def validate_manifest(m):
    require(m.get('schema') == 1 and re.fullmatch(PREFIX + r'-[a-z0-9-]{1,50}', m.get('experiment', '')),
            'wrong campaign identity')
    require(m.get('phase') in ('bracket', 'main', 'smoke', 'sprt'), 'phase')
    require(clock_of(m) in CLOCKS, 'reviewed clock')
    for field in ('campaign_started_epoch', 'deadline_epoch', 'estimated_compute_before_usd'):
        require(type(m.get(field)) in (int, float) and math.isfinite(m[field]), 'finite ' + field)
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + CAMPAIGN_SECONDS, 'fixed campaign deadline')
    require(0 <= m['estimated_compute_before_usd'] < COMPUTE_USD, 'compute cap reached')
    build = m['build']
    require(re.fullmatch(r'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/[^@]+@sha256:[a-f0-9]{64}',
                         build.get('runtime_image', '')) is not None, 'frozen worker image')
    engines = build['engines']
    require({'runner', 'candidate'} <= set(engines), 'runner and candidate required')
    for name, e in engines.items():
        require(Path(e['path']).is_absolute() and re.fullmatch('[a-f0-9]{64}', e['sha256']) is not None
                and bool(e.get('source')), 'engine identity: ' + name)
        for f in e.get('files', []):
            require(Path(f['path']).is_absolute() and re.fullmatch('[a-f0-9]{64}', f['sha256']) is not None,
                    'data file identity: ' + name)
        if name not in ('runner', 'candidate'):
            require(isinstance(e.get('family'), str) and type(e.get('rating')) in (int, float)
                    and isinstance(e.get('floodgate_player'), str), 'opponent anchor fields: ' + name)
    bins = m.get('bins')
    require(isinstance(bins, list) and 1 <= len(bins) <= 8 and all(isinstance(b, list) and b for b in bins),
            'one to eight nonempty bins')
    seen, pairs = set(), {}
    for spec in (s for b in bins for s in b):
        require(re.fullmatch(r'[a-z0-9][a-z0-9-]{0,63}', spec['id']) is not None and spec['id'] not in seen,
                'unique safe game id')
        seen.add(spec['id'])
        require(spec['opponent'] in engines and spec['opponent'] not in ('runner', 'candidate'), 'opponent')
        require(type(spec['swapped']) is bool, 'colour flag')
        pair = pairs.setdefault(spec['pair_id'], [])
        require(all(o['opponent'] == spec['opponent'] and o['opening'] == spec['opening']
                    and o['swapped'] != spec['swapped'] for o in pair), 'pair consistency')
        pair.append(spec)
        require(1 <= len(spec['opening']['moves']) <= 32, 'catalog opening prefix')
    require(len(seen) <= 640, 'game cap')


def audit(report, spec, build, clock=CLOCK):
    """Check the effective configuration and identities; return the score."""
    require(report['config'] == match_config(build, spec, clock), 'effective config differs from manifest')
    for index, name in enumerate(('candidate', spec['opponent'])):
        require(report['identities'][index]['executable']['sha256'] == build['engines'][name]['sha256'],
                'participant binary differs: ' + name)
    healthy = all(p['observation']['exit'] is not None and p['observation']['exit']['success']
                  and not p['observation']['cleanup_failures'] for p in report['processes'])
    complete = report['stop'] == 'native_conclusion' and report['engine_score'] is not None and healthy
    for turn in report['turns']:
        increment = clock['increment_ms']
        require(turn['go'].startswith('go btime ') and turn['go'].endswith(f' binc {increment} winc {increment}'),
                'unexpected clock command')
    return {'complete': complete, 'healthy': healthy, 'stop': report['stop'],
            'score': report['engine_score'][0] / 2 if complete else None,
            'conclusion': report['conclusion'], 'plies': len(report['turns']),
            'exhaustion': report.get('exhaustion'), 'failure': report.get('failure')}


def run_game(build, spec, dest, deadline_epoch, clock=CLOCK):
    dest.mkdir()
    cfg = match_config(build, spec, clock)
    (dest / 'config.json').write_bytes(encode(cfg))
    receipt = {'id': spec['id'], 'pair_id': spec['pair_id'], 'opponent': spec['opponent'],
               'swapped': spec['swapped'], 'started_epoch': time.time(), 'complete': False, 'score': None}
    start = time.monotonic()
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    try:
        for name in ('runner', 'candidate', spec['opponent']):
            e = build['engines'][name]
            require(file_hash(e['path']) == e['sha256'], 'binary changed: ' + name)
            for f in e.get('files', []):
                require(file_hash(f['path']) == f['sha256'], 'data file changed: ' + f['path'])
        timeout = min(1900, deadline_epoch - time.time() - 120)
        require(timeout > 0, 'campaign deadline leaves no execution time')
        runner = build['engines']['runner']['path']
        command = [runner, 'shogi-match', str(dest / 'config.json'), str(dest / 'report.json')]
        with (dest / 'stdout.log').open('wb') as out, (dest / 'stderr.log').open('wb') as err:
            process = subprocess.Popen(command, stdout=out, stderr=err, start_new_session=True)
            try:
                receipt['returncode'] = process.wait(timeout=timeout)
            except subprocess.TimeoutExpired:
                receipt['outer_timeout'] = True
                os.killpg(process.pid, signal.SIGKILL)
                process.wait(timeout=10)
        report = dest / 'report.json'
        if report.exists():
            raw = report.read_bytes()
            receipt['report_sha256'] = digest(raw)
            (dest / 'report.json.gz').write_bytes(gzip.compress(raw, mtime=0))
            replay = subprocess.run([runner, 'shogi-match-replay', str(report)], capture_output=True,
                                    text=True, timeout=120)
            receipt['replay'] = {'returncode': replay.returncode, 'stdout': replay.stdout[-2000:],
                                 'stderr': replay.stderr[-2000:]}
            receipt.update(audit(decode(raw), spec, build, clock))
            receipt['complete'] = bool(receipt['complete'] and replay.returncode == 0
                                       and receipt.get('returncode') == 0)
            report.unlink()
    except Exception as error:
        receipt.update(error=repr(error), complete=False)
    finally:
        if not receipt['complete']:
            receipt['score'] = None
        end = resource.getrusage(resource.RUSAGE_CHILDREN)
        receipt.update(ended_epoch=time.time(), wall_seconds=time.monotonic() - start,
                       cpu_seconds=end.ru_utime + end.ru_stime - usage.ru_utime - usage.ru_stime)
        (dest / 'receipt.json').write_bytes(encode(receipt))
    return receipt


def worker(manifest_uri, store_uri):
    from tools.experiments.gcp import GCSStore, parse_uri
    from tools.strength.anchor_worker import cloud_environment, put_new
    bucket, key = parse_uri(manifest_uri)
    raw = GCSStore('gs://' + bucket).get(key)
    m = decode(raw)
    validate_manifest(m)
    store = GCSStore(store_uri)
    index = int(os.environ['BATCH_TASK_INDEX'])
    require(0 <= index < len(m['bins']), 'invalid task index')
    require(os.environ.get('BATCH_TASK_RETRY_ATTEMPT', '0') == '0', 'no worker retries')
    claim = {'manifest_sha256': digest(raw), 'task_index': index, 'started_epoch': time.time()}
    put_new(store, f'tasks/{index}/started.json', encode(claim))
    receipt = dict(claim, complete=False, games=[])
    try:
        receipt['environment'] = cloud_environment()
        with tempfile.TemporaryDirectory(prefix=PREFIX + '-') as directory:
            for spec in m['bins'][index]:
                require(time.time() < m['deadline_epoch'] - 300, 'campaign deadline reached')
                prefix = 'games/' + spec['id']
                put_new(store, prefix + '/launch.json', encode(claim | {'spec': spec}))
                dest = Path(directory) / spec['id']
                result = run_game(m['build'], spec, dest, m['deadline_epoch'], clock_of(m))
                files = sorted(p for p in dest.iterdir() if p.name != 'receipt.json') + [dest / 'receipt.json']
                for path in files:
                    put_new(store, prefix + '/' + path.name, path.read_bytes())
                receipt['games'].append({k: result.get(k) for k in ('id', 'complete', 'score', 'stop')})
        receipt['complete'] = True
    except Exception as error:
        receipt['error'] = repr(error)
        raise
    finally:
        receipt['ended_epoch'] = time.time()
        put_new(store, f'tasks/{index}/receipt.json', encode(receipt))


def prepare(args):
    build = json.loads(Path(args.build).read_text())
    openings = json.loads(Path(args.openings).read_text())['openings']
    campaign = json.loads(Path(args.campaign).read_text())
    require(0 <= args.first_pair and args.first_pair + args.pairs <= len(openings), 'opening allocation')
    games = []
    for i in range(args.pairs):
        pair = args.first_pair + i
        opponent = args.opponents[i % len(args.opponents)]
        opening = {'id': openings[pair]['id'], 'moves': openings[pair]['moves']}
        for swapped in (False, True):
            games.append({'id': f'p{pair:03d}-g{int(swapped)}', 'pair_id': f'p{pair:03d}',
                          'opponent': opponent, 'opening': opening, 'swapped': swapped})
    bins = [games[i::8] for i in range(min(8, len(games)))]
    m = {'schema': 1, 'experiment': args.name, 'phase': args.phase, 'build': build, 'bins': bins,
         'campaign_started_epoch': campaign['started_epoch'],
         'deadline_epoch': campaign['started_epoch'] + CAMPAIGN_SECONDS,
         'estimated_compute_before_usd': args.compute_before, 'selection_reason': args.reason,
         **({'clock': {'base_ms': args.clock[0], 'increment_ms': args.clock[1]}} if args.clock else {}),
         'source_files': {n: digest(Path(n).read_bytes()) for n in
                          ('tools/strength/shogi_anchor.py', 'tools/strength/anchor_stats.py',
                           'docs/m5/evidence/msh191/method.md')}}
    validate_manifest(m)
    with Path(args.output).open('x') as f:
        json.dump(m, f, indent=1)


def submit(args):
    from tools.experiments import gcp
    from tools.strength import anchor_cloud
    m = decode(Path(args.manifest).read_bytes())
    validate_manifest(m)
    require(args.image == m['build']['runtime_image'], 'image differs from frozen worker')
    config = gcp.validate_config(decode(Path(args.config).read_bytes()))
    require(config.get('vm_hourly_usd') == RATE, 'reviewed compute rate')
    require(config['store_uri'] == STORE + m['experiment'], 'canonical batch prefix')
    anchor_cloud.run(args, policy=policy(), manifest_validator=validate_manifest)


def collect(run, runner):
    run = Path(run)
    m = json.loads((run / 'manifest.json').read_text())
    validate_manifest(m)
    build = m['build']
    games, failures = [], []
    for spec in (s for b in m['bins'] for s in b):
        folder = run / 'raw/games' / spec['id']
        if not (folder / 'receipt.json').exists():
            failures.append({'game_id': spec['id'], 'reason': 'missing receipt'})
            continue
        receipt = json.loads((folder / 'receipt.json').read_text())
        if not (folder / 'report.json.gz').exists():
            failures.append({'game_id': spec['id'], 'reason': receipt.get('error', 'no report')})
            continue
        raw = gzip.decompress((folder / 'report.json.gz').read_bytes())
        require(digest(raw) == receipt['report_sha256'], 'report hash differs')
        with tempfile.NamedTemporaryFile(suffix='.json') as f:
            f.write(raw)
            f.flush()
            subprocess.run([runner, 'shogi-match-replay', f.name], check=True, capture_output=True, timeout=120)
        observed = audit(json.loads(raw), spec, build, clock_of(m))
        require(observed['complete'] == receipt['complete'] and observed['score'] == receipt['score'],
                'receipt disagrees with native report')
        if not observed['complete']:
            failures.append({'game_id': spec['id'], 'reason': observed['stop'],
                             'exhaustion': observed['exhaustion'], 'failure': observed['failure']})
            continue
        opponent = build['engines'][spec['opponent']]
        games.append({'game_id': spec['id'], 'pair_id': spec['pair_id'], 'opponent_id': spec['opponent'],
                      'family': opponent['family'], 'anchor': opponent['rating'], 'score': observed['score'],
                      'phase': m['phase'], 'plies': observed['plies'], 'conclusion': observed['conclusion'],
                      'candidate_sente': not spec['swapped'], 'report_sha256': digest(raw)})
    return games, failures


def summarize(args):
    games, failures = [], []
    for run in args.runs:
        g, f = collect(run, args.runner)
        games.extend(g)
        failures.extend(f)
    result = {'games': games, 'failures': failures,
              **anchor_stats.summarize(games, args.prior_mean, args.prior_sd)}
    result['opponents'] = {}
    for name in sorted({g['opponent_id'] for g in games}):
        rows = [g for g in games if g['opponent_id'] == name]
        result['opponents'][name] = {'wins': sum(g['score'] == 1 for g in rows),
                                     'draws': sum(g['score'] == .5 for g in rows),
                                     'losses': sum(g['score'] == 0 for g in rows),
                                     'score': sum(g['score'] for g in rows) / len(rows),
                                     'fit': anchor_stats.fit(rows, args.prior_mean, args.prior_sd)}
    if args.sprt:
        from tools.strength import stats
        pairs = {}
        for g in games:
            pairs.setdefault(g['pair_id'], []).append(g['score'])
        complete = [sum(v) / 2 for k, v in sorted(pairs.items()) if len(v) == 2]
        counts = stats.pentanomial(complete)
        test = stats.Sprt(0, args.sprt, .05, .05, 30)
        result['sprt'] = {'h1': args.sprt, 'pentanomial': counts, **test.status(counts),
                          'estimate': stats.estimate(counts), 'complete_pairs': len(complete)}
        print(json.dumps(result['sprt'], indent=1))
    with Path(args.output).open('x') as f:
        json.dump(result, f, indent=1)
    print(json.dumps({'joint': {k: result['joint'][k] for k in ('estimate', 'interval95', 'half_width',
                                                                  'scored_games', 'clusters')},
                      'family_spread': result['family_spread'], 'failures': len(failures),
                      'opponents': {k: {x: v[x] for x in ('wins', 'draws', 'losses')}
                                    for k, v in result['opponents'].items()}}, indent=1))


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='command', required=True)
    w = sub.add_parser('worker')
    w.add_argument('--manifest', required=True)
    w.add_argument('--store', required=True)
    a = sub.add_parser('prepare')
    for key in ('build', 'openings', 'campaign', 'name', 'reason', 'output'):
        a.add_argument('--' + key, required=True)
    a.add_argument('--phase', choices=['bracket', 'main', 'smoke', 'sprt'], required=True)
    a.add_argument('--opponents', nargs='+', required=True)
    a.add_argument('--first-pair', type=int, required=True)
    a.add_argument('--pairs', type=int, required=True)
    a.add_argument('--compute-before', type=float, required=True)
    a.add_argument('--clock', type=int, nargs=2, help='base_ms increment_ms (reviewed values only)')
    s = sub.add_parser('submit')
    for key in ('manifest', 'config', 'image', 'job', 'out'):
        s.add_argument('--' + key, required=True)
    s.add_argument('--subscription', default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    z = sub.add_parser('summarize')
    z.add_argument('runs', nargs='+')
    z.add_argument('--output', required=True)
    z.add_argument('--runner', default='target/release/mshogi-match')
    z.add_argument('--prior-mean', type=float, default=2900.0)
    z.add_argument('--prior-sd', type=float, default=500.0)
    z.add_argument('--sprt', type=float, help='also run the pentanomial SPRT (H0=0, this H1) over complete pairs')
    args = p.parse_args()
    if args.command == 'worker':
        worker(args.manifest, args.store)
    elif args.command == 'prepare':
        prepare(args)
    elif args.command == 'submit':
        submit(args)
    else:
        summarize(args)


if __name__ == '__main__':
    main()
