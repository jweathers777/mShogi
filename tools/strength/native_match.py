"""Paired native-game predecessor tests through the native experiment controller.

The Rust runner owns all rules, histories, scores and report validation. This
module schedules complete color pairs and applies the existing pentanomial SPRT.
Use --calibrate for one move by each engine from each identical opening root.
"""
import argparse
import concurrent.futures
import gzip
import hashlib
import json
import os
import signal
import statistics
import subprocess
import time
from pathlib import Path

from . import stats
from .cli import engine_identity, environment, write_new


def telemetry(value):
    """Find the game telemetry in the versioned observation envelope."""
    if isinstance(value, dict):
        if 'search_id' in value and 'completed_depth' in value:
            return value
        for child in value.values():
            found = telemetry(child)
            if found is not None:
                return found
    return None


def summarize(report):
    """Use only native concluded scores; never convert a cutoff to a draw."""
    games = report['games']
    if len(games) != 2 or [g['pairing']['swapped'] for g in games] != [False, True]:
        raise ValueError('expected one complete ordered color pair')
    scores = []
    turns = [[], []]
    for game in games:
        if game.get('cleanup'):
            raise ValueError('native engine cleanup failed; retained report is unscored')
        score = game['score_first_twice']
        scores.append(None if score is None else (2 - score if game['pairing']['swapped'] else score))
        for step in game['steps']:
            t = telemetry(step['observation'])
            if t is not None:
                turns[step['player']].append({
                    'seconds': step['elapsed_ns'] / 1e9,
                    'depth': t['completed_depth'], 'nodes': t['nodes'],
                    'search_id': t['search_id'], 'applied': step['applied'],
                })
    return {'scores_twice': scores,
            'pair_score': None if None in scores else sum(scores) / 4,
            'stops': [g['stop'] for g in games], 'turns': turns,
            'plies': [len(g['steps']) for g in games]}


def run_pair(args, seed, out):
    config = out / f'pair-{seed:04d}.config.json'
    report_path = out / f'pair-{seed:04d}.report.json'
    start = time.monotonic()
    command = [args.config_writer, args.candidate, args.baseline, str(seed),
               str(args.candidate_nodes), str(args.baseline_depth), str(args.baseline_nodes),
               '1' if args.calibrate else str(args.max_steps), str(args.pair_wall_ms), str(config)]
    if getattr(args, 'base', None) is not None:
        command.extend(['--clock-base-ms', str(round(args.base * 1000)),
                        '--clock-increment-ms', str(round(args.increment * 1000))])
    if getattr(args, 'openings', None):
        command.extend(['--openings', args.openings, '--opening-index',
                        str((seed - 1) % args.opening_count)])
        if getattr(args, 'random_plies', 0):
            command.extend(['--random-plies', str(args.random_plies)])
    if args.game == 'tenjiku':
        command.append(args.profile)
    if args.watchdog_ms is not None:
        command.extend(['--watchdog-ms', str(args.watchdog_ms)])
    with open(out / f'pair-{seed:04d}.config.log', 'x') as log:
        subprocess.run(command, check=True, stdout=log, stderr=subprocess.STDOUT, timeout=15)
    command = [args.runner, 'experiment-run', str(config), str(report_path)]
    with open(out / f'pair-{seed:04d}.log', 'x') as log:
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
        try:
            code = process.wait(timeout=args.pair_wall_ms / 1000 + 45)
        except BaseException:
            # The runner owns its children. At the outer hard watchdog, stop
            # the entire session so a child cannot survive a killed controller.
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait(timeout=5)
            raise
    if code != 0:
        raise RuntimeError(f'pair {seed}: runner exit {code}; see retained log')
    with open(out / f'pair-{seed:04d}.replay.log', 'x') as log:
        subprocess.run([args.runner, 'experiment-replay', str(report_path)], check=True,
                       stdout=log, stderr=subprocess.STDOUT, timeout=60)
    raw = report_path.read_bytes()
    result = summarize(json.loads(raw))
    result.update({'seed': seed, 'wall_seconds': time.monotonic() - start,
                   'report_sha256': hashlib.sha256(raw).hexdigest(),
                   'report': report_path.name + '.gz'})
    with gzip.open(str(report_path) + '.gz', 'xb') as stream:
        stream.write(raw)
    report_path.unlink()
    write_new(out / f'pair-{seed:04d}.summary.json', result)
    return result


def summary(args, rows, start, failure=None):
    counts = stats.pentanomial(row['pair_score'] for row in rows if row['pair_score'] is not None)
    sprt = stats.Sprt(args.elo0, args.elo1, args.alpha, args.beta, args.min_pairs)
    players = []
    for player in range(2):
        turns = [t for row in rows for t in row['turns'][player]]
        players.append({'turns': len(turns),
                        'median_seconds': statistics.median(t['seconds'] for t in turns) if turns else None,
                        'median_depth': statistics.median(t['depth'] for t in turns) if turns else None,
                        'total_request_seconds': sum(t['seconds'] for t in turns)})
    scores = [s for row in rows for s in row['scores_twice'] if s is not None]
    return {'pairs': len(rows), 'scored_pairs': sum(counts), 'pentanomial': counts,
            'decision': 'calibration' if args.calibrate else sprt.status(counts)['decision'] or 'no decision',
            'wins_draws_losses': [scores.count(2), scores.count(1), scores.count(0)],
            'sprt': sprt.status(counts), 'estimate': stats.estimate(counts),
            'wall_seconds': time.monotonic() - start,
            'sum_pair_wall_seconds': sum(row['wall_seconds'] for row in rows),
            'players': players, 'failure': failure, 'rows': rows}


def main(default_game="shogi"):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--game', choices=['shogi', 'xiangqi', 'chu', 'tenjiku'], default=default_game)
    parser.add_argument('--profile', choices=['adams', 'muller'])
    parser.add_argument('--candidate', required=True)
    parser.add_argument('--baseline', required=True)
    parser.add_argument('--candidate-nodes', type=int)
    parser.add_argument('--base', type=float, help='Fischer initial time in seconds (Shogi)')
    parser.add_argument('--increment', type=float, default=0, help='Fischer increment in seconds')
    parser.add_argument('--openings', help='fixed Shogi opening JSON catalog')
    parser.add_argument('--baseline-depth', type=int)
    parser.add_argument('--baseline-nodes', type=int)
    parser.add_argument('--runner', default='target/release/mshogi-match')
    parser.add_argument('--config-writer')
    parser.add_argument('--watchdog-ms', type=int, help='symmetric engine watchdog; writer default is 60000 for clocks, 30000 for fixed work')
    parser.add_argument('--out', required=True)
    parser.add_argument('--workers', type=int, default=4)
    parser.add_argument('--max-pairs', type=int, default=600)
    parser.add_argument('--min-pairs', type=int, default=30)
    parser.add_argument('--first-seed', type=int, default=1)
    parser.add_argument('--wall-seconds', type=int, default=1800)
    parser.add_argument('--pair-wall-ms', type=int, default=180000)
    parser.add_argument('--elo0', type=float, default=0)
    parser.add_argument('--elo1', type=float, default=15)
    parser.add_argument('--alpha', type=float, default=.05)
    parser.add_argument('--beta', type=float, default=.05)
    parser.add_argument('--calibrate', action='store_true')
    args = parser.parse_args()
    if (args.game == 'tenjiku') != (args.profile is not None):
        parser.error('--profile adams|muller is required only for --game tenjiku')
    defaults = {'shogi': (2, 5000, 320), 'xiangqi': (1, 128, 500), 'chu': (1, 512, 500), 'tenjiku': (1, 1024, 500)}
    depth, nodes, args.max_steps = defaults[args.game]
    if args.base is not None:
        if args.game != 'shogi' or not (0.001 <= args.base <= 600 and 0 <= args.increment <= 60):
            parser.error('clock mode requires Shogi, base 0.001..600 and increment 0..60 seconds')
        if args.candidate_nodes is not None or args.baseline_nodes is not None or args.baseline_depth is not None:
            parser.error('clock mode uses symmetric native safety caps; do not supply fixed work')
        args.candidate_nodes = args.baseline_nodes = 5_000_000
        args.baseline_depth = 64
    elif args.candidate_nodes is None or args.increment != 0:
        parser.error('fixed-work mode requires --candidate-nodes; increment requires --base')
    if args.openings and args.game != 'shogi':
        parser.error('--openings currently requires Shogi')
    if args.first_seed < 0:
        parser.error('--first-seed must be nonnegative')
    if args.baseline_depth is None:
        args.baseline_depth = depth
    if args.baseline_nodes is None:
        args.baseline_nodes = nodes
    if args.config_writer is None:
        args.config_writer = f'target/release/examples/{args.game}_match_config'
    if not (1 <= args.workers <= 8 and 1 <= args.max_pairs <= 10000
            and 0 < args.wall_seconds <= 3600 and 0 < args.pair_wall_ms <= 600000):
        parser.error('invalid finite campaign bounds')
    if args.watchdog_ms is not None and not 1 <= args.watchdog_ms <= 60000:
        parser.error('watchdog must be within 1..60000 ms')
    stats.Sprt(args.elo0, args.elo1, args.alpha, args.beta, args.min_pairs)
    out = Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=False)
    if args.openings:
        raw = Path(args.openings).read_bytes()
        catalog = json.loads(raw)
        openings = catalog.get('openings')
        if not isinstance(openings, list) or not openings or len({r['id'] for r in openings}) != len(openings):
            parser.error('opening catalog must have nonempty unique records')
        args.opening_count = len(openings)
        args.openings_sha256 = hashlib.sha256(raw).hexdigest()
        frozen = out / 'openings.json'
        with frozen.open('xb') as stream:
            stream.write(raw)
        args.openings = str(frozen)
    write_new(out / 'manifest.json', {'kind': f'{args.game}-paired-predecessor', 'schema': 1,
              'settings': vars(args), 'environment': environment(),
              'candidate': engine_identity(args.candidate), 'baseline': engine_identity(args.baseline),
              'runner': engine_identity(args.runner), 'config_writer': engine_identity(args.config_writer),
              'timing': 'native runner request/launch/search/receipt, including process overhead',
              'sampling': ('fixed catalog index (seed-1) modulo count; both colors; source frozen before play'
                           if args.openings else 'one seeded opening per pair; no discarded unfinished pairs'),
              'stopping': 'check SPRT in seed order after each worker batch; all launched pairs retained'})
    start = time.monotonic()
    rows = []
    failure = None
    # Batches avoid unobserved running work after a decision. The last batch can
    # contain up to workers-1 pairs beyond the first crossing; all are reported.
    try:
        with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
            for offset in range(0, args.max_pairs, args.workers):
                remaining = args.wall_seconds - (time.monotonic() - start)
                if remaining < args.pair_wall_ms / 1000 + 45:
                    break
                seeds = range(args.first_seed + offset,
                              args.first_seed + min(offset + args.workers, args.max_pairs))
                futures = [(seed, pool.submit(run_pair, args, seed, out)) for seed in seeds]
                errors = []
                for seed, future in futures:
                    try:
                        rows.append(future.result())
                    except Exception as error:
                        errors.append(f'pair {seed}: {error}')
                if errors:
                    raise RuntimeError('; '.join(errors))
                result = summary(args, rows, start)
                write_new(out / f'checkpoint-{len(rows):04d}.json', result)
                print(json.dumps({k: result[k] for k in ('pairs', 'wins_draws_losses', 'sprt', 'wall_seconds')}), flush=True)
                if args.calibrate:
                    continue
                if any(row['pair_score'] is None for row in rows):
                    raise RuntimeError('unfinished pair: stop and diagnose; no score-based exclusion')
                if result['sprt']['decision']:
                    break
    except Exception as error:
        failure = str(error)
    result = summary(args, rows, start, failure)
    write_new(out / 'summary.json', result)
    if failure:
        raise SystemExit(failure)


if __name__ == '__main__':
    main()
