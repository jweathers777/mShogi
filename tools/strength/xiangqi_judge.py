"""MSH-197 (Xiangqi Strength Anchor): judged move quality of ladder games.

Full-strength Pikafish (fixed nodes, one thread) scores the position before
every reply of every scored ladder game. It is a measurement tool only:
native rules decide moves and results. Definitions are those of MSH-193
(Shogi Blunder Screen): win probability p = 1/(1+exp(-cp/600)), mate scores
map to 1 or 0; a clear blunder is a move with p_before >= 0.30 and
p_after <= p_before - 0.30 from the mover's view; the loss of a move is
max(0, p_before - p_after).

Commands: prepare (from retrieved ladder runs), submit, worker (Batch),
collect, analyze. No cloud action on import.
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

PREFIX = 'msh197'
CAMPAIGN_SECONDS = 6 * 3600
COMPUTE_USD = 15


def require(condition, message):
    if not condition:
        raise ValueError(message)


def policy():
    from tools.strength import anchor_cloud
    return anchor_cloud.CampaignPolicy(PREFIX, CAMPAIGN_SECONDS, COMPUTE_USD, 'tools.strength.xiangqi_judge',
                                       ('worker',))


def shift(token, delta):
    m = re.fullmatch(r'([a-i])(\d+)([a-i])(\d+)', token)
    require(m is not None, 'move token ' + token)
    return f'{m[1]}{int(m[2]) + delta}{m[3]}{int(m[4]) + delta}'


def native_reply(token, wire):
    """Native action of a reply token, or None for a resignation."""
    if token == 'resign':
        return None
    return shift(token, -int(wire.get('rank_offset', 0))) if wire.get('rank_offset') else token


def extract(runs):
    """Scored games as native move lists, with the mover of each reply."""
    games = []
    for run in runs:
        for receipt_path in sorted(Path(run).glob('raw/pairs/*/receipt.json')):
            receipt = json.loads(receipt_path.read_text())
            for red in (0, 1):
                path = receipt_path.parent / f'red{red}.report.json.gz'
                if not path.exists():
                    continue
                report = json.loads(gzip.decompress(path.read_bytes()))
                if report['engine_score'] is None:
                    continue
                c = report['config']
                players = receipt['players']
                moves = list(c['opening']['moves'])
                start = len(moves)
                turns = []
                for t in report['turns']:
                    words = t['reply'].split()
                    if words[-1] in ('resign', 'draw') and c['wire'][t['engine']].get('protocol') == 'ucci':
                        if words[-1] == 'resign':
                            break
                        words = words[:-1]
                    action = native_reply(words[1], c['wire'][t['engine']])
                    if action is None:
                        break
                    side = 0 if t['actor'] == 'red' else 1
                    turns.append({'player': players[t['engine']], 'index': len(moves), 'move': action,
                                  'clock_ms': t['clock_before_ms'][side]})
                    moves.append(action)
                require(c['opening'].get('fen') is None, 'catalog openings start from the standard root')
                games.append({'id': f"{receipt['id']}-red{red}", 'players': players, 'red': c['red'],
                              'engine_score': report['engine_score'], 'moves': moves, 'start': start,
                              'turns': turns, 'conclusion': report['conclusion']})
    return games


def judge_positions(engine, sequences, out):
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
            lines.append(line.strip())
            if line.startswith(token):
                return lines
    send('uci')
    until('uciok')
    for name, value in engine['options']:
        send(f'setoption name {name} value {value}')
    send('isready')
    until('readyok')
    for seq in sequences:
        send('ucinewgame')
        send('position startpos' + (' moves ' + seq if seq else ''))
        send(f"go nodes {engine['nodes']}")
        lines = until('bestmove')
        score = None
        for line in lines:
            words = line.split()
            if words[:1] == ['info'] and 'score' in words and 'pv' in words:
                i = words.index('score')
                score = (words[i + 1], int(words[i + 2]))
        out.append({'moves': seq, 'score': score, 'bestmove': lines[-1].split()[1]})
    send('quit')
    process.wait(timeout=30)


def worker(manifest_uri, store_uri):
    import os
    from tools.experiments import gcp
    from tools.experiments.core import decode, digest, encode, file_hash
    from tools.strength.anchor_worker import cloud_environment, put_new
    bucket, name = gcp.parse_uri(manifest_uri)
    raw = gcp.GCSStore('gs://' + bucket).get(name)
    m = decode(raw)
    validate_manifest(m)
    store = gcp.GCSStore(store_uri)
    index = int(os.environ['BATCH_TASK_INDEX'])
    claim = {'manifest_sha256': digest(raw), 'task_index': index, 'started_epoch': time.time()}
    put_new(store, f'tasks/{index}/started.json', encode(claim))
    receipt = dict(claim, complete=False)
    try:
        receipt['environment'] = cloud_environment()
        judge = m['judge']
        require(file_hash(judge['path']) == judge['sha256'], 'judge binary changed')
        for f in judge['files']:
            require(file_hash(f['path']) == f['sha256'], 'judge data changed')
        seqs = m['bins'][index]
        chunks = [seqs[i::m['parallel']] for i in range(m['parallel'])]
        outputs = [[] for _ in chunks]
        with concurrent.futures.ThreadPoolExecutor(m['parallel']) as pool:
            for future in [pool.submit(judge_positions, judge, c, o) for c, o in zip(chunks, outputs)]:
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
    require(m.get('schema') == 1 and re.fullmatch(PREFIX + r'-judge[a-z0-9-]{0,40}', m.get('experiment', '')),
            'identity')
    require(m['deadline_epoch'] == m['campaign_started_epoch'] + CAMPAIGN_SECONDS, 'deadline')
    require(0 <= m['estimated_compute_before_usd'] < COMPUTE_USD, 'compute cap')
    require(1 <= len(m['bins']) <= 8 and all(m['bins']), 'bins')
    require(1 <= m['parallel'] <= 4, 'parallel')
    require(type(m['judge']['nodes']) is int and 1 <= m['judge']['nodes'] <= 100_000_000, 'judge nodes')
    require(re.fullmatch(r'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/[^@]+@sha256:[a-f0-9]{64}',
                         m.get('runtime_image', '')) is not None, 'image')


def prepare(args):
    import hashlib
    games = extract(args.runs)
    seqs = set()
    for g in games:
        for t in g['turns']:
            seqs.add(' '.join(g['moves'][:t['index']]))
            seqs.add(' '.join(g['moves'][:t['index'] + 1]))
    seqs = sorted(seqs)
    h = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
    local = Path(args.engines_dir)
    judge = {'path': '/opt/engines/pikafish', 'sha256': h(local / 'pikafish'), 'nodes': args.nodes,
             'options': [['Threads', '1'], ['Hash', '256'], ['EvalFile', '/opt/engines/pikafish.nnue']],
             'files': [{'path': '/opt/engines/pikafish.nnue', 'sha256': h(local / 'pikafish.nnue')}]}
    start = time.time()
    m = {'schema': 1, 'experiment': args.experiment, 'campaign_started_epoch': start,
         'deadline_epoch': start + CAMPAIGN_SECONDS, 'estimated_compute_before_usd': args.before,
         'runtime_image': args.image, 'judge': judge, 'bins': [seqs[i::8] for i in range(8)], 'parallel': 4,
         'positions': len(seqs), 'games': len(games)}
    validate_manifest(m)
    Path(args.output).write_text(json.dumps(m))
    Path(args.games).write_text(json.dumps(games))
    print(len(games), 'games', len(seqs), 'positions')


def submit(args):
    from tools.experiments import gcp
    from tools.experiments.core import decode
    from tools.strength import anchor_cloud
    m = decode(Path(args.manifest).read_bytes())
    validate_manifest(m)
    require(args.image == m['runtime_image'], 'image differs')
    gcp.validate_config(decode(Path(args.config).read_bytes()))
    anchor_cloud.run(args, policy=policy(), manifest_validator=validate_manifest)


def collect(args):
    results = {}
    for path in sorted(Path(args.run).glob('raw/tasks/*/results.jsonl.gz')):
        for line in gzip.decompress(path.read_bytes()).decode().splitlines():
            row = json.loads(line)
            results[row['moves']] = row
    Path(args.output).write_text(json.dumps(results))
    print(len(results), 'judged positions')


def win_probability(score):
    if score is None:
        return None
    kind, value = score
    if kind == 'mate':
        return 1.0 if value > 0 else 0.0
    return 1 / (1 + math.exp(-value / 600))


def analyze_games(games, judged):
    per = {}
    for g in games:
        for t in g['turns']:
            before = judged.get(' '.join(g['moves'][:t['index']]))
            after = judged.get(' '.join(g['moves'][:t['index'] + 1]))
            if not before or not after:
                continue
            pb, pa_opp = win_probability(before['score']), win_probability(after['score'])
            if pb is None or pa_opp is None:
                continue
            pa = 1 - pa_opp
            s = per.setdefault(t['player'], {'moves': 0, 'blunders': 0, 'loss': 0.0})
            s['moves'] += 1
            s['loss'] += max(0.0, pb - pa)
            s['blunders'] += int(pb >= 0.30 and pa <= pb - 0.30)
    return {p: {'moves': s['moves'], 'clear_blunders': s['blunders'],
                'blunders_per_100_moves': round(100 * s['blunders'] / max(s['moves'], 1), 2),
                'mean_win_probability_loss': round(s['loss'] / max(s['moves'], 1), 4)}
            for p, s in sorted(per.items())}


def analyze(args):
    games = json.loads(Path(args.games).read_text())
    judged = json.loads(Path(args.judged).read_text())
    result = analyze_games(games, judged)
    Path(args.output).write_text(json.dumps(result, indent=1))
    print(json.dumps(result, indent=1))


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='command', required=True)
    w = sub.add_parser('worker')
    w.add_argument('--manifest', required=True)
    w.add_argument('--store', required=True)
    a = sub.add_parser('prepare')
    a.add_argument('runs', nargs='+')
    for k in ('engines-dir', 'image', 'experiment', 'output', 'games'):
        a.add_argument('--' + k, required=True)
    a.add_argument('--nodes', type=int, default=1_000_000)
    a.add_argument('--before', type=float, default=0)
    s = sub.add_parser('submit')
    for k in ('manifest', 'config', 'image', 'job', 'out'):
        s.add_argument('--' + k, required=True)
    s.add_argument('--subscription', default='projects/mshogi/subscriptions/mshogi-pilot-completion')
    c = sub.add_parser('collect')
    c.add_argument('run')
    c.add_argument('--output', required=True)
    z = sub.add_parser('analyze')
    for k in ('games', 'judged', 'output'):
        z.add_argument('--' + k, required=True)
    args = p.parse_args()
    {'worker': lambda: worker(args.manifest, args.store), 'prepare': lambda: prepare(args),
     'submit': lambda: submit(args), 'collect': lambda: collect(args), 'analyze': lambda: analyze(args)}[args.command]()


if __name__ == '__main__':
    main()
