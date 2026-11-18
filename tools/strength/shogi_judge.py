"""MSH-193 (Shogi Blunder Screen): judge positions with a reference USI engine.

The judge (YaneuraOu v4.79 + elmo WCSC27, admitted in MSH-191) is a
measurement tool only: it scores positions; native rules and records decide
everything else. `prepare` extracts positions from retrieved MSH-191 runs,
`submit` runs one Batch job (eight c3-standard-4 VMs), `worker` drives
several judge processes per VM, `collect` merges results. No cloud action
occurs on import.
"""
import argparse
import concurrent.futures
import gzip
import json
import math
import os
import re
import subprocess
import tempfile
import time
from pathlib import Path

from tools.experiments.core import decode, digest, encode, file_hash

PREFIX = 'msh193'
CAMPAIGN_SECONDS = 4 * 3600
COMPUTE_USD = 8
RATE = .201608
STORE = 'gs://mshogi-experiments-216675633801/msh193/'
NODES = 3_000_000


def require(condition, message):
    if not condition:
        raise ValueError(message)


def policy():
    from tools.strength import anchor_cloud
    return anchor_cloud.CampaignPolicy(PREFIX, CAMPAIGN_SECONDS, COMPUTE_USD, 'tools.strength.shogi_judge',
                                       ('worker',))


def key(sfen):
    return ' '.join(sfen.split()[:3])


def extract(runs):
    """Positions before every engine reply of every complete MSH-191 game."""
    games, positions = [], {}
    for run in runs:
        for receipt_path in sorted(Path(run).glob('raw/games/*/receipt.json')):
            receipt = json.loads(receipt_path.read_text())
            if not receipt.get('complete'):
                continue
            raw = gzip.decompress((receipt_path.parent / 'report.json.gz').read_bytes())
            report = json.loads(raw)
            record = json.loads(report['game'])['payload']
            events = record['events']
            root = record['root']['sfen']
            sfens = [root] + [e['after_sfen'] for e in events]
            turns = []
            for turn in report['turns']:
                before = sfens[turn['version']]
                after = sfens[turn['version'] + 1] if turn['version'] + 1 < len(sfens) else None
                positions.setdefault(key(before), before)
                if after is not None:
                    positions.setdefault(key(after), after)
                turns.append({'version': turn['version'], 'engine': turn['engine'], 'actor': turn['actor'],
                              'reply': turn['reply'], 'clock_before_ms': turn['clock_before_ms'],
                              'elapsed_ms': turn['elapsed_ms'], 'before': key(before),
                              'after': key(after) if after else None})
            games.append({'id': receipt['id'], 'pair_id': receipt['pair_id'], 'opponent': receipt['opponent'],
                          'candidate_sente': not receipt['swapped'], 'score': receipt['score'],
                          'conclusion': report['conclusion'], 'report_sha256': digest(raw), 'turns': turns})
    return games, positions


def judge_positions(build, sfens, threads_out):
    """Run one judge process over `sfens`; yields result dicts."""
    engine = build['judge']
    process = subprocess.Popen([engine['path']], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                               stderr=subprocess.DEVNULL, text=True, bufsize=1)

    def send(text):
        process.stdin.write(text + '\n')
        process.stdin.flush()

    def until(token):
        lines = []
        while True:
            line = process.stdout.readline()
            require(line != '', 'judge exited')
            line = line.strip()
            lines.append(line)
            if line.startswith(token):
                return lines
    send('usi')
    until('usiok')
    for name, value in engine['options']:
        send(f'setoption name {name} value {value}')
    send('isready')
    until('readyok')
    for sfen in sfens:
        send('usinewgame')
        send('position sfen ' + sfen)
        started = time.monotonic()
        send('go btime 0 wtime 0 byoyomi 600000')
        lines = until('bestmove')
        score, pv, depth, nodes = None, [], None, None
        for line in lines:
            if line.startswith('info') and ' score ' in line and ' pv ' in line:
                words = line.split()
                i = words.index('score')
                score = (words[i + 1], int(words[i + 2]))
                pv = words[words.index('pv') + 1:]
                if 'depth' in words:
                    depth = int(words[words.index('depth') + 1])
                if 'nodes' in words:
                    nodes = int(words[words.index('nodes') + 1])
        best = lines[-1].split()[1]
        threads_out.append({'sfen': sfen, 'bestmove': best, 'score': score, 'pv': pv[:12], 'depth': depth,
                            'nodes': nodes, 'seconds': time.monotonic() - started})
    send('quit')
    process.wait(timeout=30)


def worker(manifest_uri, store_uri):
    from tools.experiments.gcp import GCSStore, parse_uri
    from tools.strength.anchor_worker import cloud_environment, put_new
    bucket, name = parse_uri(manifest_uri)
    raw = GCSStore('gs://' + bucket).get(name)
    m = decode(raw)
    store = GCSStore(store_uri)
    index = int(os.environ['BATCH_TASK_INDEX'])
    claim = {'manifest_sha256': digest(raw), 'task_index': index, 'started_epoch': time.time()}
    put_new(store, f'tasks/{index}/started.json', encode(claim))
    receipt = dict(claim, complete=False)
    try:
        receipt['environment'] = cloud_environment()
        judge = m['build']['judge']
        require(file_hash(judge['path']) == judge['sha256'], 'judge binary changed')
        for f in judge['files']:
            require(file_hash(f['path']) == f['sha256'], 'judge data changed')
        sfens = m['bins'][index]
        parallel = m['parallel']
        chunks = [sfens[i::parallel] for i in range(parallel)]
        outputs = [[] for _ in chunks]
        with concurrent.futures.ThreadPoolExecutor(parallel) as pool:
            for future in [pool.submit(judge_positions, m['build'], c, o) for c, o in zip(chunks, outputs)]:
                future.result()
        rows = [r for out in outputs for r in out]
        put_new(store, f'tasks/{index}/results.jsonl.gz',
                gzip.compress(''.join(json.dumps(r) + '\n' for r in rows).encode(), mtime=0))
        receipt.update(complete=True, positions=len(rows))
    except Exception as error:
        receipt['error'] = repr(error)
        raise
    finally:
        receipt['ended_epoch'] = time.time()
        put_new(store, f'tasks/{index}/receipt.json', encode(receipt))


def validate_manifest(m):
    require(m.get('schema') == 1 and re.fullmatch(PREFIX + r'-[a-z0-9-]{1,40}', m.get('experiment', '')),
            'identity')
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + CAMPAIGN_SECONDS, 'deadline')
    require(0 <= m['estimated_compute_before_usd'] < COMPUTE_USD, 'compute cap')
    require(1 <= len(m['bins']) <= 8 and all(m['bins']), 'bins')
    require(1 <= m['parallel'] <= 4, 'parallel')
    require(re.fullmatch(r'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/[^@]+@sha256:[a-f0-9]{64}',
                         m['build'].get('runtime_image', '')) is not None, 'image')


def prepare(args):
    games, positions = extract(args.runs)
    build = json.loads(Path(args.build).read_text())
    sfens = sorted(positions.values())
    bins = [sfens[i::8] for i in range(8)]
    start = time.time()
    m = {'schema': 1, 'experiment': args.name, 'campaign_started_epoch': start,
         'deadline_epoch': start + CAMPAIGN_SECONDS, 'estimated_compute_before_usd': 0,
         'build': build, 'bins': bins, 'parallel': 4, 'nodes': NODES,
         'positions': len(sfens), 'games': len(games)}
    validate_manifest(m)
    Path(args.output).write_text(json.dumps(m))
    Path(args.games).write_text(json.dumps(games))
    print(len(games), 'games', len(sfens), 'positions')


def submit(args):
    from tools.experiments import gcp
    from tools.strength import anchor_cloud
    m = decode(Path(args.manifest).read_bytes())
    validate_manifest(m)
    config = gcp.validate_config(decode(Path(args.config).read_bytes()))
    require(config['store_uri'] == STORE + m['experiment'] and config.get('vm_hourly_usd') == RATE, 'config')
    anchor_cloud.run(args, policy=policy(), manifest_validator=validate_manifest)


def collect(args):
    results = {}
    for path in sorted(Path(args.run).glob('raw/tasks/*/results.jsonl.gz')):
        for line in gzip.decompress(path.read_bytes()).decode().splitlines():
            row = json.loads(line)
            results[key(row['sfen'])] = row
    Path(args.output).write_text(json.dumps(results))
    print(len(results), 'judged positions')


def win_probability(score):
    """Side-to-move win probability from a judge score."""
    if score is None:
        return None
    kind, value = score
    if kind == 'mate':
        return 1.0 if value > 0 else 0.0
    return 1 / (1 + math.exp(-value / 600))


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='command', required=True)
    w = sub.add_parser('worker')
    w.add_argument('--manifest', required=True)
    w.add_argument('--store', required=True)
    a = sub.add_parser('prepare')
    a.add_argument('runs', nargs='+')
    for k in ('build', 'name', 'output', 'games'):
        a.add_argument('--' + k, required=True)
    s = sub.add_parser('submit')
    for k in ('manifest', 'config', 'image', 'job', 'out'):
        s.add_argument('--' + k, required=True)
    s.add_argument('--subscription', default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    c = sub.add_parser('collect')
    c.add_argument('run')
    c.add_argument('--output', required=True)
    args = p.parse_args()
    {'worker': lambda: worker(args.manifest, args.store), 'prepare': lambda: prepare(args),
     'submit': lambda: submit(args), 'collect': lambda: collect(args)}[args.command]()


if __name__ == '__main__':
    main()
