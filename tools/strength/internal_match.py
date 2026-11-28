"""Paired game predecessor tests through the experiment controller.

The Rust runner owns all rules, histories, scores and report validation. This
module schedules independent color games and pairs their scores for the pentanomial SPRT.
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


CONFIG_SECONDS = 120
REPLAY_SECONDS = 60
RUNNER_GRACE_MS = 45_000
KILL_GRACE_SECONDS = 10
EVIDENCE_SECONDS = 5


def summarize_game(report, swapped):
    """Use only a replayed game's concluded score."""
    games = report['games']
    if len(games) != 1 or games[0]['pairing']['swapped'] != swapped:
        raise ValueError('expected one game with the scheduled color')
    game = games[0]
    if game.get('cleanup'):
        raise ValueError('mShogi engine cleanup failed')
    score = game['score_first_twice']
    turns = [[], []]
    for ply, step in enumerate(game['steps']):
        t = telemetry(step['observation'])
        if t is not None:
            turns[step['player']].append({
                'ply_after_opening': ply, 'seconds': step['elapsed_ns'] / 1e9,
                'depth': t['completed_depth'], 'nodes': t['nodes'],
                'search_id': t.get('search_id'), 'applied': step['applied'],
            })
    return {'score_twice': 2 - score if swapped and score is not None else score,
            'stop': game['stop'], 'turns': turns, 'plies': len(game['steps'])}


def score_pair(seed, games):
    scores = [game['score_twice'] for game in games]
    return {'seed': seed, 'scores_twice': scores,
            'pair_score': None if None in scores else sum(scores) / 4,
            'stops': [game['stop'] for game in games],
            'turns': [games[0]['turns'][player] + games[1]['turns'][player] for player in range(2)],
            'plies': [game['plies'] for game in games],
            'games': [{k: v for k, v in game.items() if k != 'turns'} for game in games]}


def wait_for_game(process, game_wall_ms):
    # OS wait calls may have narrower timeout types than the experiment budget.
    deadline = time.monotonic_ns() + (game_wall_ms + RUNNER_GRACE_MS) * 1_000_000
    while True:
        remaining_ns = deadline - time.monotonic_ns()
        try:
            return process.wait(timeout=max(0, min(remaining_ns / 1e9, 86400)))
        except subprocess.TimeoutExpired:
            if time.monotonic_ns() >= deadline:
                raise


def signal_group(pid, sig):
    try:
        os.killpg(pid, sig)
    except ProcessLookupError:
        pass


def stop_game(process):
    if os.name == 'nt':
        subprocess.run(['taskkill', '/F', '/T', '/PID', str(process.pid)], check=False,
                       timeout=KILL_GRACE_SECONDS // 2)
    else:
        signal_group(process.pid, signal.SIGTERM)
    try:
        process.wait(timeout=KILL_GRACE_SECONDS // 2)
    except subprocess.TimeoutExpired:
        pass
    finally:
        if os.name == 'posix':
            # The runner can exit before an engine in its group does.
            signal_group(process.pid, signal.SIGKILL)
    process.wait(timeout=KILL_GRACE_SECONDS // 2)


def run_game(args, seed, swapped, out):
    name = f'game-{seed:04d}-{"second" if swapped else "first"}'
    config = out / f'{name}.config.json'
    report_path = out / f'{name}.report.json'
    start = time.monotonic()
    command = [args.helper, 'config', args.candidate, args.baseline, str(seed),
               str(args.candidate_nodes), str(args.baseline_depth), str(args.baseline_nodes),
               '1' if args.calibrate else str(args.max_steps), str(args.game_wall_ms), str(config),
               '--swapped', str(swapped).lower()]
    if getattr(args, 'depth', None) is not None:
        command.extend(['--candidate-depth', str(args.depth)])
    if args.base is not None:
        command.extend(['--clock-base-ms', str(round(args.base * 1000)),
                        '--clock-increment-ms', str(round(args.increment * 1000))])
        if args.candidate_time_factor != 1:
            command.extend(['--candidate-time-factor', str(args.candidate_time_factor)])
    if args.openings:
        command.extend(['--openings', args.openings, '--opening-index',
                        str((seed - 1) % args.opening_count)])
        if args.random_plies:
            command.extend(['--random-plies', str(args.random_plies)])
    command.extend(['--profile', profile(args.game, args.profile)])
    if args.watchdog_ms is not None:
        command.extend(['--watchdog-ms', str(args.watchdog_ms)])
    try:
        with open(out / f'{name}.config.log', 'x') as log:
            subprocess.run(command, check=True, stdout=log, stderr=subprocess.STDOUT,
                           timeout=CONFIG_SECONDS)
        command = [args.runner, 'experiment-run', str(config), str(report_path)]
        with open(out / f'{name}.log', 'x') as log:
            # Each POSIX runner has its own group. On Batch it stays within
            # the enclosing command's session for the worker's outer watchdog.
            process_options = {'process_group': 0} if os.name == 'posix' else {}
            process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT,
                                       **process_options)
            try:
                code = wait_for_game(process, args.game_wall_ms)
            except BaseException:
                stop_game(process)
                raise
        if code != 0:
            if os.name == 'posix':
                signal_group(process.pid, signal.SIGKILL)
            raise RuntimeError(f'runner exit {code}; see {name}.log')
        with open(out / f'{name}.replay.log', 'x') as log:
            subprocess.run([args.runner, 'experiment-replay', str(report_path)], check=True,
                           stdout=log, stderr=subprocess.STDOUT, timeout=REPLAY_SECONDS)
        raw = report_path.read_bytes()
        result = summarize_game(json.loads(raw), swapped)
        with gzip.open(str(report_path) + '.gz', 'xb') as stream:
            stream.write(raw)
        report_path.unlink()
        result.update(report_sha256=hashlib.sha256(raw).hexdigest(), report=report_path.name + '.gz')
    except (OSError, subprocess.SubprocessError, ValueError, RuntimeError) as error:
        result = {'score_twice': None, 'stop': 'runner_error', 'turns': [[], []], 'plies': 0,
                  'error': str(error), 'report': report_path.name if report_path.is_file() else None}
    result.update(swapped=swapped, wall_seconds=time.monotonic() - start)
    write_new(out / f'{name}.summary.json', result)
    return result


def calibration_failure(rows):
    for row in rows:
        if (row['plies'] != [1, 1] or any(stop not in ('game_concluded', 'step_limit') for stop in row['stops'])
                or any(len(turns) != 1 or not turns[0]['applied'] for turns in row['turns'])):
            return f"pair {row['seed']}: calibration requires one accepted move by each engine"
    return None


def summary(args, rows, start, failure=None):
    counts = stats.pentanomial(row['pair_score'] for row in rows if row['pair_score'] is not None)
    sprt = stats.Sprt(args.elo0, args.elo1, args.alpha, args.beta, args.min_pairs)
    players = []
    for player in range(2):
        turns = [t for row in rows for t in row['turns'][player]]
        late_depths = [t['depth'] for t in turns if t['applied'] and t['ply_after_opening'] >= args.late_ply]
        players.append({'turns': len(turns), 'late_turns': len(late_depths),
                        'median_late_depth': statistics.median(late_depths) if late_depths else None,
                        'median_seconds': statistics.median(t['seconds'] for t in turns) if turns else None,
                        'median_depth': statistics.median(t['depth'] for t in turns) if turns else None,
                        'total_request_seconds': sum(t['seconds'] for t in turns)})
    scores = [s for row in rows for s in row['scores_twice'] if s is not None]
    return {'pairs': len(rows), 'scored_pairs': sum(counts), 'pentanomial': counts,
            'unscored_games': sum(s is None for row in rows for s in row['scores_twice']),
            'decision': 'calibration' if args.calibrate else 'data collection' if args.selfplay else sprt.status(counts)['decision'] or 'no decision',
            'wins_draws_losses': [scores.count(2), scores.count(1), scores.count(0)],
            'sprt': sprt.status(counts), 'estimate': stats.estimate(counts),
            'wall_seconds': time.monotonic() - start,
            'sum_game_wall_seconds': sum(game['wall_seconds'] for row in rows for game in row['games']),
            'players': players, 'failure': failure, 'rows': rows}


def options(parser, *, selfplay=False):
    game_options(parser)
    if selfplay:
        parser.add_argument('--engine', required=True)
        parser.add_argument('--depth', type=int, help='fixed search depth for both self-play participants')
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
    parser.add_argument('--late-ply', type=int, default=40, help='first post-opening ply for late-depth statistics')
    parser.add_argument('--workers', type=int, default=4)
    parser.add_argument('--max-pairs', type=int, default=600)
    parser.add_argument('--max-steps', type=int, help='positive game event budget per game; unfinished games stay unscored')
    parser.add_argument('--min-pairs', type=int, default=30)
    parser.add_argument('--first-seed', type=int, default=1)
    parser.add_argument('--wall-seconds', type=int, help='campaign budget; default fits all scheduled games')
    parser.add_argument('--game-wall-ms', type=int, help='per-game wall; default uses the clock or 180 seconds for fixed work')
    parser.add_argument('--elo0', type=float, default=0)
    parser.add_argument('--elo1', type=float, default=15)
    parser.add_argument('--alpha', type=float, default=.05)
    parser.add_argument('--beta', type=float, default=.05)
    parser.add_argument('--calibrate', action='store_true')
    parser.add_argument('--candidate-time-factor', type=int, default=1,
                        help='time odds: the candidate clock is this multiple of --base/--increment')
    parser.set_defaults(func=campaign, selfplay=selfplay)


def game_slot_ms(args):
    overhead = CONFIG_SECONDS + REPLAY_SECONDS + KILL_GRACE_SECONDS + EVIDENCE_SECONDS
    return args.game_wall_ms + RUNNER_GRACE_MS + overhead * 1000


def projected_seconds(args):
    batches = (2 * args.max_pairs + args.workers - 1) // args.workers
    return (batches * game_slot_ms(args) + 999) // 1000 + 1


def configure(args, parser):
    """Resolve settings identically for play and Batch planning."""
    if args.late_ply < 0:
        parser.error('--late-ply must be nonnegative')
    if args.selfplay:
        args.candidate = args.baseline = args.engine
        if args.baseline_nodes is not None or args.baseline_depth is not None or args.candidate_time_factor != 1:
            parser.error('self-play uses identical work for both participants')
        max_depth = 8 if args.game == 'chess' else 64
        if args.depth is not None and (args.base is not None or not 1 <= args.depth <= max_depth):
            parser.error(f'--depth requires fixed-work self-play and a depth in 1..{max_depth}')
        if args.base is None:
            args.baseline_nodes = args.candidate_nodes
            args.baseline_depth = args.depth if args.depth is not None else max_depth
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
        if (round(args.base * 1000) * args.candidate_time_factor > 3_600_000
                or round(args.increment * 1000) * args.candidate_time_factor > 60_000):
            parser.error('time odds exceed the runner clock limits')
        if args.candidate_nodes is not None or args.baseline_nodes is not None or args.baseline_depth is not None:
            parser.error('clock mode uses symmetric safety caps; do not supply fixed work')
        args.candidate_nodes = args.baseline_nodes = 5_000_000
        args.baseline_depth = 8 if args.game == 'chess' else 64
    elif args.candidate_nodes is None or args.increment != 0:
        parser.error('fixed-work mode requires --candidate-nodes; increment requires --base')
    if not 0 <= args.first_seed <= 2**64 - args.max_pairs:
        parser.error('seed range exceeds unsigned 64-bit values')
    if args.baseline_depth is None:
        args.baseline_depth = depth
    if args.baseline_nodes is None:
        args.baseline_nodes = nodes
    if args.game_wall_ms is None:
        args.game_wall_ms = 180_000
        if args.base is not None:
            plies = 1 if args.calibrate else args.max_steps
            factor = args.candidate_time_factor
            # Either participant can move first from the opening. For odd
            # limits, give the larger clock the extra increment.
            increments = (plies // 2) * (1 + factor) + (plies % 2) * factor
            charge = round(args.base * 1000) * (1 + factor) + round(args.increment * 1000) * increments
            args.game_wall_ms = charge + max(30_000, (charge + 9) // 10)
    if not (1 <= args.workers <= 8 and 1 <= args.max_pairs <= 10000
            and 0 < args.game_wall_ms < 2**64):
        parser.error('invalid finite campaign bounds')
    if args.wall_seconds is None:
        args.wall_seconds = projected_seconds(args)
    if args.wall_seconds * 1000 < ((2 + args.workers - 1) // args.workers) * game_slot_ms(args):
        parser.error('campaign budget is too short for both colors of one opening')
    if args.watchdog_ms is not None and not 1 <= args.watchdog_ms <= 60000:
        parser.error('watchdog must be within 1..60000 ms')
    stats.Sprt(args.elo0, args.elo1, args.alpha, args.beta, args.min_pairs)


def campaign(args):
    configure(args, argparse.ArgumentParser(prog='strength ' + args.command))
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
    write_new(out / 'manifest.json', {'kind': f'{args.game}-paired-predecessor', 'schema': 2,
              'settings': {k:v for k,v in vars(args).items() if k != 'func'}, 'environment': environment(),
              'candidate': engine_identity(args.candidate), 'baseline': engine_identity(args.baseline),
              'runner': engine_identity(args.runner), 'helper': engine_identity(args.helper),
              'timing': 'runner request/launch/search/receipt, including process overhead',
              'sampling': ('fixed catalog index (seed-1) modulo count; both colors; source frozen before play'
                           if args.openings else 'one seeded opening per pair; no discarded unfinished pairs'),
              'stopping': 'check SPRT after each seed-ordered batch; all launched games retained'})
    start = time.monotonic()
    rows = []
    failure = None
    # Retain deterministic batches and wait for their evidence before a decision.
    # Workers each run one game at a time, independent of its opposite color.
    try:
        with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
            while len(rows) < args.max_pairs:
                remaining_ms = args.wall_seconds * 1000 - int((time.monotonic() - start) * 1000)
                available_waves = remaining_ms // game_slot_ms(args)
                count = min(args.max_pairs - len(rows), args.workers, available_waves * args.workers // 2)
                if count < 1:
                    break
                seeds = range(args.first_seed + len(rows), args.first_seed + len(rows) + count)
                games = {(seed, swapped): pool.submit(run_game, args, seed, swapped, out)
                         for seed in seeds for swapped in (False, True)}
                rows.extend(score_pair(seed, [games[seed, swapped].result() for swapped in (False, True)])
                            for seed in seeds)
                result = summary(args, rows, start)
                write_new(out / f'checkpoint-{len(rows):04d}.json', result)
                print(json.dumps({k: result[k] for k in ('pairs', 'wins_draws_losses', 'sprt', 'wall_seconds')}), flush=True)
                if not (args.calibrate or args.selfplay) and result['sprt']['decision']:
                    break
    except Exception as error:
        failure = str(error)
    if args.calibrate and failure is None:
        failure = calibration_failure(rows)
    result = summary(args, rows, start, failure)
    write_new(out / 'summary.json', result)
    if failure or not rows:
        raise SystemExit(failure or 'campaign budget allowed no games')
