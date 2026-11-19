"""Clocked Xiangqi games through the native runner (MSH-194, Xiangqi Strength Harness).

Each pair plays one opening of the catalog twice with colours reversed. Every
game runs `mshogi-match xiangqi-match`: persistent UCI processes, controller
Fischer clocks, native admission of every reply and native replay of the
report. Only native conclusions are scored; clock exhaustion, pending
adjudication, watchdog and failures are recorded unscored.

Local: python3 -m tools.strength.xiangqi_games run --runner R --a ENGINE --b ENGINE \
    --openings CATALOG --pairs N --base-ms 5000 --inc-ms 100 --out DIR [--workers 8]
Summary: python3 -m tools.strength.xiangqi_games summary DIR
Sequential test (engine A is the candidate):
    python3 -m tools.strength.xiangqi_games sprt DIR [DIR ...] --h1 15
Batches continue the opening order with --first-pair.
"""
import argparse
import concurrent.futures
import json
import subprocess
import time
from pathlib import Path

PROFILE = 'mshogi.xiangqi.wxf2018.v1'
WATCHDOGS = {'handshake_ms': 10000, 'ready_ms': 10000, 'response_ms': 10000, 'write_ms': 2000,
             'cleanup_ms': 5000}
IO = {'line_bytes': 8192, 'queued_lines': 256, 'transcript_bytes': 4194304, 'stderr_bytes': 65536,
      'command_bytes': 4096}


def engine(path, label):
    return {'path': str(Path(path).resolve()), 'args': ['--game', 'xiangqi', 'uci'], 'options': [],
            'model_path': None, 'model_identity': label, 'deterministic': False}


def spec(path_or_engine, label, json_path):
    """Engine config and wire format: an mShogi binary, or a JSON spec
    {"engine": EngineConfig, "wire": Wire} for an external engine."""
    if json_path:
        d = json.load(open(json_path))
        return d['engine'], d.get('wire', {})
    return engine(path_or_engine, label), {}


def configs(args):
    catalog = json.load(open(args.openings))['openings']
    a, wa = spec(args.a, 'A', args.a_json)
    b, wb = spec(args.b, 'B', args.b_json)
    for e, mb in ((a, args.a_hash), (b, args.b_hash)):
        if mb:
            e['options'] = e['options'] + [{'name': 'Hash', 'value': str(mb)}]
    odds = None
    if args.odds:
        ab, ai, bb, bi = (int(x) for x in args.odds.split(','))
        odds = [{'base_ms': ab, 'increment_ms': ai}, {'base_ms': bb, 'increment_ms': bi}]
    out = []
    for pair in range(args.first_pair, args.first_pair + args.pairs):
        o = catalog[(pair - 1) % len(catalog)]
        for red in (0, 1):
            out.append({
                'schema_version': 1, 'profile': PROFILE, 'id': f'pair{pair:04d}-{o["id"]}-red{red}'[:128],
                'engines': [a, b], 'wire': [wa, wb], 'odds': odds,
                'opening': {'id': o['id'], 'fen': o.get('fen'), 'canonical': None, 'moves': o['moves']},
                'red': red, 'clock': {'base_ms': args.base_ms, 'increment_ms': args.inc_ms},
                'grace_ms': args.grace_ms, 'max_game_ms': args.max_game_ms,
                'watchdogs': WATCHDOGS, 'io': IO})
    return out


def play(runner, config, directory):
    path = directory / f'{config["id"]}.config.json'
    path.write_text(json.dumps(config))
    report = directory / f'{config["id"]}.report.json'
    started = time.time()
    done = subprocess.run([runner, 'xiangqi-match', str(path), str(report)], capture_output=True, text=True,
                          timeout=config['max_game_ms'] / 1000 + 120)
    return {'id': config['id'], 'returncode': done.returncode, 'stdout': done.stdout[-2000:],
            'stderr': done.stderr[-2000:], 'seconds': round(time.time() - started, 2)}


def run(args):
    directory = Path(args.out)
    directory.mkdir(parents=True)
    work = configs(args)
    receipts = []
    with concurrent.futures.ThreadPoolExecutor(args.workers) as pool:
        for r in pool.map(lambda c: play(args.runner, c, directory), work):
            receipts.append(r)
            print(r['id'], r['returncode'], r['stdout'].strip()[:200], flush=True)
    (directory / 'receipts.json').write_text(json.dumps(receipts, indent=1))


def load(directory):
    games = []
    for path in sorted(Path(directory).glob('*.report.json')):
        games.append(json.loads(path.read_text()))
    return games


def sprt(args):
    from tools.strength import stats
    pairs, unscored = {}, 0
    for directory in args.dirs:
        for g in load(directory):
            pair = g['config']['id'].split('-')[0]
            if g['engine_score'] is None:
                unscored += 1
                continue
            pairs.setdefault(pair, []).append(g['engine_score'][0] / 2)
    complete = [sum(v) / 2 for _, v in sorted(pairs.items()) if len(v) == 2]
    counts = stats.pentanomial(complete)
    test = stats.Sprt(0, args.h1, .05, .05, 30)
    print(json.dumps({'h1': args.h1, 'pentanomial': counts, 'complete_pairs': len(complete),
                      'unscored_games': unscored, **test.status(counts),
                      'estimate': stats.estimate(counts)}, indent=1))


def summary(args):
    games = load(args.dir)
    stops, score, plies, unscored = {}, [0, 0], [], []
    for g in games:
        stops[g['stop']] = stops.get(g['stop'], 0) + 1
        if g['engine_score']:
            score[0] += g['engine_score'][0]
            score[1] += g['engine_score'][1]
        else:
            unscored.append(g['config']['id'])
        plies.append(len(g['turns']))
    plies.sort()
    print(json.dumps({'games': len(games), 'stops': stops, 'half_points_A_B': score,
                      'median_plies': plies[len(plies) // 2] if plies else None,
                      'unscored': unscored[:20]}, indent=1))


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest='command', required=True)
    r = sub.add_parser('run')
    r.add_argument('--runner', required=True)
    r.add_argument('--a')
    r.add_argument('--b')
    r.add_argument('--a-json', help='external engine spec for engine A')
    r.add_argument('--b-json', help='external engine spec for engine B')
    r.add_argument('--openings', required=True)
    r.add_argument('--pairs', type=int, required=True)
    r.add_argument('--base-ms', type=int, default=5000)
    r.add_argument('--inc-ms', type=int, default=100)
    r.add_argument('--grace-ms', type=int, default=500)
    r.add_argument('--max-game-ms', type=int, default=1_800_000)
    r.add_argument('--workers', type=int, default=8)
    r.add_argument('--first-pair', type=int, default=1)
    r.add_argument('--a-hash', type=int, help='Hash option (MB) for engine A')
    r.add_argument('--b-hash', type=int, help='Hash option (MB) for engine B')
    r.add_argument('--odds', help='time odds: A_BASE_MS,A_INC_MS,B_BASE_MS,B_INC_MS')
    r.add_argument('--out', required=True)
    s = sub.add_parser('summary')
    s.add_argument('dir')
    t = sub.add_parser('sprt')
    t.add_argument('dirs', nargs='+')
    t.add_argument('--h1', type=float, default=15)
    args = p.parse_args()
    {'run': run, 'summary': summary, 'sprt': sprt}[args.command](args)


if __name__ == '__main__':
    main()
