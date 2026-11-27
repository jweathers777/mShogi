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
from .common import engine_identity, environment, write_new, game_options, profile


def telemetry(value):
    """Find the game telemetry in the versioned observation envelope."""
    if isinstance(value, dict):
        if 'nodes' in value and 'completed_depth' in value:
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
                    'search_id': t.get('search_id'), 'applied': step['applied'],
                })
    return {'scores_twice': scores,
            'pair_score': None if None in scores else sum(scores) / 4,
            'stops': [g['stop'] for g in games], 'turns': turns,
            'plies': [len(g['steps']) for g in games]}


def wait_for_pair(process, pair_wall_ms):
    # OS wait calls may have narrower timeout types than the experiment budget.
    deadline = time.monotonic_ns() + (pair_wall_ms + 45_000) * 1_000_000
    while True:
        remaining_ns = deadline - time.monotonic_ns()
        try:
            return process.wait(timeout=max(0, min(remaining_ns / 1e9, 86400)))
        except subprocess.TimeoutExpired:
            if time.monotonic_ns() >= deadline:
                raise


def run_pair(args, seed, out):
    config = out / f'pair-{seed:04d}.config.json'
    report_path = out / f'pair-{seed:04d}.report.json'
    start = time.monotonic()
    command = [args.helper, 'config', args.candidate, args.baseline, str(seed),
               str(args.candidate_nodes), str(args.baseline_depth), str(args.baseline_nodes),
               '1' if args.calibrate else str(args.max_steps), str(args.pair_wall_ms), str(config)]
    if getattr(args, 'base', None) is not None:
        command.extend(['--clock-base-ms', str(round(args.base * 1000)),
                        '--clock-increment-ms', str(round(args.increment * 1000))])
        if getattr(args, 'candidate_time_factor', 1) != 1:
            command.extend(['--candidate-time-factor', str(args.candidate_time_factor)])
    if getattr(args, 'openings', None):
        command.extend(['--openings', args.openings, '--opening-index',
                        str((seed - 1) % args.opening_count)])
        if getattr(args, 'random_plies', 0):
            command.extend(['--random-plies', str(args.random_plies)])
    command.extend(['--profile', profile(args.game, args.profile)])
    if args.watchdog_ms is not None:
        command.extend(['--watchdog-ms', str(args.watchdog_ms)])
    with open(out / f'pair-{seed:04d}.config.log', 'x') as log:
        subprocess.run(command, check=True, stdout=log, stderr=subprocess.STDOUT, timeout=120)
    command = [args.runner, 'experiment-run', str(config), str(report_path)]
    with open(out / f'pair-{seed:04d}.log', 'x') as log:
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
        try:
            code = wait_for_pair(process, args.pair_wall_ms)
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


# Rule fixed before any MSH-204 (Tori Strength Harness) result, 2026-11-20,
# as proposed to John: a Tori pair in which a game reaches the step limit
# without a native result is excluded from the test and counted; every other
# unscored stop still stops the campaign; the campaign also stops when more
# than LENGTH_LIMIT_SHARE of at least LENGTH_LIMIT_MIN_PAIRS pairs are excluded.
LENGTH_LIMIT_SHARE = 0.10
LENGTH_LIMIT_MIN_PAIRS = 20


def length_limited(args, row):
    return (args.game == 'tori' and row['pair_score'] is None
            and all(stop in ('native_concluded', 'step_limit') for stop in row['stops'])
            and 'step_limit' in row['stops'])


def unfinished(args, rows):
    """The reason to stop the campaign, or None."""
    if any(row['pair_score'] is None and not length_limited(args, row) for row in rows):
        return 'unfinished pair: stop and diagnose; no score-based exclusion'
    limited = sum(length_limited(args, row) for row in rows)
    if len(rows) >= LENGTH_LIMIT_MIN_PAIRS and limited > LENGTH_LIMIT_SHARE * len(rows):
        return f'length-limited pairs {limited} of {len(rows)} exceed {LENGTH_LIMIT_SHARE:.0%}: stop and diagnose'
    return None


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
            'length_limited_pairs': sum(length_limited(args, row) for row in rows),
            'decision': 'calibration' if args.calibrate else 'data collection' if args.selfplay else sprt.status(counts)['decision'] or 'no decision',
            'wins_draws_losses': [scores.count(2), scores.count(1), scores.count(0)],
            'sprt': sprt.status(counts), 'estimate': stats.estimate(counts),
            'wall_seconds': time.monotonic() - start,
            'sum_pair_wall_seconds': sum(row['wall_seconds'] for row in rows),
            'players': players, 'failure': failure, 'rows': rows}


def options(parser, *, selfplay=False):
    game_options(parser)
    if selfplay:
        parser.add_argument('--engine', required=True)
    else:
        parser.add_argument('--candidate', required=True)
        parser.add_argument('--baseline', required=True)
    parser.add_argument('--candidate-nodes', type=int)
    parser.add_argument('--base', type=float, help='Fischer initial time in seconds')
    parser.add_argument('--increment', type=float, default=0, help='Fischer increment in seconds')
    parser.add_argument('--openings', help='frozen opening JSON catalog')
    parser.add_argument('--baseline-depth', type=int)
    parser.add_argument('--baseline-nodes', type=int)
    parser.add_argument('--runner', default='target/release/mshogi-match')
    parser.add_argument('--random-plies', type=int, default=0, help='seeded continuations of catalog roots')
    parser.add_argument('--watchdog-ms', type=int, help='symmetric engine watchdog; writer default is 60000 for clocks, 30000 for fixed work')
    parser.add_argument('--out', required=True)
    parser.add_argument('--workers', type=int, default=4)
    parser.add_argument('--max-pairs', type=int, default=600)
    parser.add_argument('--max-steps', type=int, help='positive native event budget per game; unfinished games stay unscored')
    parser.add_argument('--min-pairs', type=int, default=30)
    parser.add_argument('--first-seed', type=int, default=1)
    parser.add_argument('--wall-seconds', type=int, default=1800)
    parser.add_argument('--pair-wall-ms', type=int, default=180000)
    parser.add_argument('--elo0', type=float, default=0)
    parser.add_argument('--elo1', type=float, default=15)
    parser.add_argument('--alpha', type=float, default=.05)
    parser.add_argument('--beta', type=float, default=.05)
    parser.add_argument('--calibrate', action='store_true')
    parser.add_argument('--candidate-time-factor', type=int, default=1,
                        help='time odds: the candidate clock is this multiple of --base/--increment')
    parser.set_defaults(func=campaign, selfplay=selfplay)


def campaign(args):
    parser = argparse.ArgumentParser(prog="strength match")
    if args.selfplay:
        args.candidate = args.baseline = args.engine
        if args.baseline_nodes is not None or args.baseline_depth is not None or args.candidate_time_factor != 1:
            parser.error('self-play uses identical work for both participants')
        if args.base is None:
            args.baseline_nodes = args.candidate_nodes
            args.baseline_depth = 64
    if args.random_plies and (not args.openings or not 0 <= args.random_plies <= 128):
        parser.error('--random-plies requires a catalog and at most 128 plies')
    if (args.game == 'tenjiku') != (args.profile is not None):
        parser.error('--profile adams|muller is required only for --game tenjiku')
    defaults = {'chess': (2, 5000, 500), 'shogi': (2, 5000, 320), 'xiangqi': (1, 128, 500), 'tori': (1, 512, 1000), 'chu': (1, 512, 500), 'tenjiku': (1, 1024, 500)}
    depth, nodes, default_steps = defaults[args.game]
    if args.max_steps is None:
        args.max_steps = default_steps
    if not 1 <= args.max_steps < 2**64:
        parser.error('--max-steps must be a positive 64-bit count')
    if args.candidate_time_factor != 1 and args.base is None:
        parser.error('--candidate-time-factor needs a clock (--base)')
    if args.base is not None:
        if not (0.001 <= args.base <= 600 and 0 <= args.increment <= 60):
            parser.error('clock mode requires base 0.001..600 and increment 0..60 seconds')
        if not 1 <= args.candidate_time_factor <= 8:
            parser.error('--candidate-time-factor must be 1..8')
        if args.candidate_nodes is not None or args.baseline_nodes is not None or args.baseline_depth is not None:
            parser.error('clock mode uses symmetric native safety caps; do not supply fixed work')
        args.candidate_nodes = args.baseline_nodes = 5_000_000
        args.baseline_depth = 64
    elif args.candidate_nodes is None or args.increment != 0:
        parser.error('fixed-work mode requires --candidate-nodes; increment requires --base')
    if not 0 <= args.first_seed <= 2**64 - args.max_pairs:
        parser.error('seed range exceeds unsigned 64-bit values')
    if args.baseline_depth is None:
        args.baseline_depth = depth
    if args.baseline_nodes is None:
        args.baseline_nodes = nodes
    if not (1 <= args.workers <= 8 and 1 <= args.max_pairs <= 10000
            and args.wall_seconds > 0 and 0 < args.pair_wall_ms < 2**64):
        parser.error('invalid finite campaign bounds')
    if args.watchdog_ms is not None and not 1 <= args.watchdog_ms <= 60000:
        parser.error('watchdog must be within 1..60000 ms')
    stats.Sprt(args.elo0, args.elo1, args.alpha, args.beta, args.min_pairs)
    out = Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=False)
    if args.openings:
        raw = Path(args.openings).read_bytes()
        catalog = json.loads(raw)
        if catalog.get('profile') not in (None, profile(args.game, args.profile)):
            parser.error('opening catalog has another profile')
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
              'settings': {k:v for k,v in vars(args).items() if k != 'func'}, 'environment': environment(),
              'candidate': engine_identity(args.candidate), 'baseline': engine_identity(args.baseline),
              'runner': engine_identity(args.runner), 'helper': engine_identity(args.helper),
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
                if args.wall_seconds * 1000 - int((time.monotonic() - start) * 1000) < args.pair_wall_ms + 45_000:
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
                if args.selfplay:
                    if any(stop not in ('native_concluded', 'step_limit', 'wall_budget',
                                        'native_pending_adjudication', 'native_no_board_actions', 'artifact_limit') for
                           row in rows for stop in row['stops']):
                        raise RuntimeError('self-play encountered an unexpected stop; see native reports')
                    continue
                reason = unfinished(args, rows)
                if reason:
                    raise RuntimeError(reason)
                if result['sprt']['decision']:
                    break
    except Exception as error:
        failure = str(error)
    result = summary(args, rows, start, failure)
    write_new(out / 'summary.json', result)
    if failure or not rows:
        raise SystemExit(failure or 'campaign budget allowed no pairs')
