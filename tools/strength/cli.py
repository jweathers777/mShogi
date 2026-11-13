"""Engine strength test harness (MSH-163): bench, blunders, match, build-engine.

Run from the repository root:
    python3 -m tools.strength.cli bench --engine target/release/mshogi --out FILE
    python3 -m tools.strength.cli blunders --engine target/release/mshogi --movetime 1000 --out FILE
    python3 -m tools.strength.cli match --candidate A --baseline B --out DIR
    python3 -m tools.strength.cli build-engine REV
See docs/m5/strength-harness.md. Results are evidence for the recorded machine only.
"""
import argparse
import concurrent.futures
import datetime
import gzip
import json
import os
import platform
import shutil
import signal
import statistics
import subprocess
import sys
import threading
import time
from pathlib import Path

from . import stats
from .uci import Engine, default_workers, file_sha256

REPO = Path(__file__).resolve().parents[2]
DATA = Path(__file__).resolve().parent / 'data'
CLAIM_OPTION = ('ClaimPolicy', 'auto')


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='seconds')


def git(*args):
    out = subprocess.run(['git', '-C', str(REPO), *args], capture_output=True, text=True)
    return out.stdout.strip() if out.returncode == 0 else None


def environment():
    return {'utc': utc(), 'host': platform.node(), 'system': platform.system(), 'release': platform.release(),
            'machine': platform.machine(), 'processor': _cpu_name(), 'python': platform.python_version(),
            'harness_revision': git('rev-parse', 'HEAD'),
            'harness_dirty': bool(git('status', '--porcelain', '--', 'tools/strength'))}


def _cpu_name():
    try:
        out = subprocess.run(['sysctl', '-n', 'machdep.cpu.brand_string'], capture_output=True, text=True, timeout=5)
        if out.returncode == 0:
            return out.stdout.strip()
    except OSError:
        pass
    return platform.processor()


def engine_identity(path, engine=None):
    ident = {'path': str(path), 'sha256': file_sha256(path)}
    side = Path(str(path) + '.identity.json')
    if side.exists():
        ident['build'] = json.loads(side.read_text())
    if engine is not None:
        ident['uci_id'] = engine.identity
    return ident


def write_new(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, 'x') as stream:
        json.dump(value, stream, indent=1)
        stream.write('\n')


def load_json(path):
    path = Path(path)
    return json.load(gzip.open(path)) if path.suffix == '.gz' else json.loads(path.read_text())


# ---------------------------------------------------------------- bench

def bench(args):
    spec = load_json(args.positions)
    engine = Engine(args.engine, ['uci'], [CLAIM_OPTION])
    rounds = []
    try:
        for _ in range(args.repeat):
            rows = []
            for p in spec['positions']:
                r = engine.search(p.get('fen'), p.get('moves', []), 'depth %d' % p['depth'], args.timeout)
                info = r['info']
                rows.append({'id': p['id'], 'depth': p['depth'], 'reached_depth': info.get('depth'),
                             'seldepth': info.get('seldepth'), 'nodes': info.get('nodes'),
                             'engine_ms': info.get('time'), 'wall_seconds': r['wall_seconds'], 'bestmove': r['bestmove']})
            short = [x['id'] for x in rows if x['reached_depth'] != x['depth']]
            if short:
                raise SystemExit('fixed depth not reached (time to depth invalid): ' + ', '.join(short))
            nodes = sum(x['nodes'] for x in rows)
            seconds = sum(x['wall_seconds'] for x in rows)
            engine_seconds = sum(x['engine_ms'] or 0 for x in rows) / 1000 or float('nan')
            rounds.append({'positions': rows, 'nodes': nodes, 'seconds': seconds, 'nps': nodes / seconds,
                           'engine_seconds': engine_seconds, 'engine_nps': nodes / engine_seconds})
        ident = engine_identity(args.engine, engine)
    finally:
        engine.close()
    signatures = {r['nodes'] for r in rounds}
    per_position = []
    for i, p in enumerate(spec['positions']):
        times = [r['positions'][i]['wall_seconds'] for r in rounds]
        nodes = [r['positions'][i]['nodes'] for r in rounds]
        per_position.append({'id': p['id'], 'depth': p['depth'], 'nodes': nodes[0] if len(set(nodes)) == 1 else nodes,
                             'median_seconds_to_depth': statistics.median(times),
                             'median_nps': statistics.median(n / t for n, t in zip(nodes, times))})
    result = {'kind': 'mshogi-strength-bench', 'schema': 1, 'positions_file': str(Path(args.positions).relative_to(REPO)
              if Path(args.positions).resolve().is_relative_to(REPO) else args.positions),
              'positions_sha256': file_sha256(args.positions), 'set': spec['name'],
              'engine': ident, 'environment': environment(), 'repeat': args.repeat,
              'summary': {'bench_nodes': rounds[0]['nodes'], 'deterministic_nodes': len(signatures) == 1,
                          'median_seconds': statistics.median(r['seconds'] for r in rounds),
                          'median_nps': statistics.median(r['nps'] for r in rounds),
                          'min_nps': min(r['nps'] for r in rounds), 'max_nps': max(r['nps'] for r in rounds),
                          'median_engine_nps': statistics.median(r['engine_nps'] for r in rounds),
                          'nps_note': 'nps uses driver wall time per search (includes UCI I/O); engine_nps uses the engine-reported time'},
              'per_position': per_position, 'rounds': rounds}
    if args.out:
        write_new(args.out, result)
    s = result['summary']
    print('bench %s: nodes %d (%s), median %.2f s, median nps %.0f [%.0f..%.0f], engine-time nps %.0f' % (
        spec['name'], s['bench_nodes'], 'deterministic' if s['deterministic_nodes'] else 'NOT deterministic',
        s['median_seconds'], s['median_nps'], s['min_nps'], s['max_nps'], s['median_engine_nps']))
    for p in per_position:
        print('  %-36s depth %2d  %7.3f s  %9.0f nps' % (p['id'], p['depth'], p['median_seconds_to_depth'], p['median_nps']))
    return result


# ---------------------------------------------------------------- blunders

def _blunder_worker(path, positions, movetime, results, lock):
    engine = Engine(path, ['uci'], [CLAIM_OPTION])
    try:
        for p in positions:
            r = engine.search(p['initial_fen'], p['history'], 'movetime %d' % movetime, movetime / 1000 * 3 + 30)
            row = {'id': p['id'], 'phase': p['phase'], 'blunder_move': p['blunder_move'],
                   'reference_move': p['reference_move'], 'bestmove': r['bestmove'],
                   'avoided': r['bestmove'] != p['blunder_move'],
                   'matched_reference': r['bestmove'] == p['reference_move'],
                   'depth': r['info'].get('depth'), 'nodes': r['info'].get('nodes'),
                   'score': r['info'].get('score'), 'wall_seconds': r['wall_seconds']}
            with lock:
                results.append(row)
        return engine.identity
    finally:
        engine.close()


def blunders(args):
    suite = load_json(args.suite)
    positions = suite['positions']
    workers = max(1, min(args.workers, len(positions)))
    results, lock = [], threading.Lock()
    started = time.monotonic()
    with concurrent.futures.ThreadPoolExecutor(workers) as pool:
        chunks = [positions[i::workers] for i in range(workers)]
        ids = [f.result() for f in [pool.submit(_blunder_worker, args.engine, c, args.movetime, results, lock) for c in chunks]]
    order = {p['id']: i for i, p in enumerate(positions)}
    results.sort(key=lambda r: order[r['id']])
    by_phase = {}
    for r in results:
        b = by_phase.setdefault(r['phase'], {'positions': 0, 'avoided': 0, 'matched_reference': 0})
        b['positions'] += 1
        b['avoided'] += r['avoided']
        b['matched_reference'] += r['matched_reference']
    summary = {'positions': len(results), 'avoided': sum(r['avoided'] for r in results),
               'matched_reference': sum(r['matched_reference'] for r in results),
               'repeated_blunder': sum(not r['avoided'] for r in results), 'by_phase': by_phase,
               'median_depth': statistics.median(r['depth'] for r in results if r['depth'] is not None),
               'median_nodes': statistics.median(r['nodes'] for r in results if r['nodes'] is not None)}
    result = {'kind': 'mshogi-strength-blunders', 'schema': 1, 'suite': suite['name'],
              'metric_note': 'avoided means only bestmove != the confirmed blunder; another move may also lose. '
                             'matched_reference compares with one 20M reference alternative.',
              'suite_sha256': file_sha256(args.suite), 'movetime_ms': args.movetime, 'workers': workers,
              'engine': {**engine_identity(args.engine), 'uci_id': ids[0]}, 'environment': environment(),
              'wall_seconds': time.monotonic() - started, 'summary': summary, 'results': results}
    if args.out:
        write_new(args.out, result)
    print('blunders %s at %d ms: avoided %d/%d, matched reference %d, median depth %s' % (
        suite['name'], args.movetime, summary['avoided'], summary['positions'], summary['matched_reference'],
        summary['median_depth']))
    for phase, b in sorted(by_phase.items()):
        print('  %-10s avoided %d/%d, reference %d' % (phase, b['avoided'], b['positions'], b['matched_reference']))
    return result


# ---------------------------------------------------------------- match

def match_config(args, opening, label):
    def engine(path, options, name):
        return {'path': str(Path(path).resolve()), 'args': ['uci'],
                'options': [{'name': n, 'value': v} for n, v in [CLAIM_OPTION, *options]],
                'model_path': None, 'model_identity': name, 'deterministic': False}
    if args.base is None:
        time_control, work = 'untimed', {'depth': None, 'nodes': None, 'movetime_ms': args.movetime}
        response_ms = args.movetime + 10000
    else:
        base_ns, inc_ns = int(args.base * 1e9), int(args.increment * 1e9)
        time_control = {'increment': {'initial_ns': [base_ns, base_ns], 'increment_ns': inc_ns}}
        work = {'depth': None, 'nodes': None, 'movetime_ms': None}
        response_ms = int((args.base + args.increment) * 1000) + 10000
    return {'schema_version': 1, 'id': label,
            'engines': [engine(args.candidate, args.candidate_option, 'candidate'),
                        engine(args.baseline, args.baseline_option, 'baseline')],
            'openings': [{'id': opening['id'], 'fen': opening.get('fen'), 'moves': opening['moves']}],
            'paired': True, 'claim_policy': 'auto_claim',
            'time_control': time_control, 'work': work, 'max_plies': 512,
            'watchdogs': {'handshake_ms': 10000, 'ready_ms': 10000, 'response_ms': response_ms,
                          'write_ms': 2000, 'cleanup_ms': 2000},
            'io': {'line_bytes': 8192, 'queued_lines': 64, 'transcript_bytes': 1048576,
                   'stderr_bytes': 16384, 'command_bytes': 4096}}


def _moves(record_json):
    """Opening prefix and played UCI moves from a native match record."""
    record = json.loads(record_json)
    initial = json.loads(record['initial_record_json'])
    prefix = [e['event']['action'] for e in initial['events'] if e['event'].get('kind') == 'move']
    played = [e['request']['command']['move'] for e in record['events']
              if isinstance(e['request']['command'], dict) and 'move' in e['request']['command']]
    remaining = record['events'][-1]['reply'].get('remaining_ns') if record['events'] else None
    return {'initial_fen': initial['initial_fen'], 'prefix': prefix, 'played': played,
            'final_remaining_ns': remaining}


def game_winner_engine(game):
    """Configuration engine index of the winner, 'draw', or None, from the native conclusion."""
    # Mirrors MatchConclusion::white_score_twice: Chess(Draw)/Draw score as draws,
    # Chess(Checkmate)/Win score for the winner, Unresolved is unscored.
    c = game['conclusion']
    if not isinstance(c, dict):
        return None
    if 'draw' in c or ('chess' in c and 'draw' in c['chess']):
        return 'draw'
    value = c.get('win') or (c.get('chess') or {}).get('checkmate')
    if isinstance(value, dict) and value.get('winner') in ('white', 'black'):
        return game['engines_by_color'][0 if value['winner'] == 'white' else 1]
    return None


def search_reports(transcript):
    """(depth, nodes) from the last info line before each bestmove, per engine."""
    search, last = [[], []], [None, None]
    for e in transcript['entries']:
        if e['direction'] != 'response':
            continue
        if e['text'].startswith('info ') and ' nodes ' in e['text']:
            last[e['engine']] = e['text']
        elif e['text'].startswith('bestmove'):
            line, last[e['engine']] = last[e['engine']], None
            if line:
                tokens = line.split()
                depth = int(tokens[tokens.index('depth') + 1]) if 'depth' in tokens else None
                search[e['engine']].append((depth, int(tokens[tokens.index('nodes') + 1])))
    return search


def _median(values):
    values = [v for v in values if v is not None]
    return statistics.median(values) if values else None


def summarize_pair(report, cfg):
    """Pair score for engine 0 plus compact per-game evidence. None score if not fully scored.

    The native per-opening summary is cross-checked against winners derived
    independently from each game's conclusion and color assignment.
    """
    s = report['summaries'][0]
    games, wins, draws = [], [0, 0], 0
    for g in report['games']:
        winner = game_winner_engine(g)
        if winner == 'draw':
            draws += 1
        elif winner is not None:
            wins[winner] += 1
        search = search_reports(g['transcript'])
        games.append({'pairing': g['pairing'], 'white': g['engines_by_color'][0], 'stop': g['stop'],
                      'conclusion': g['conclusion'], 'failure': (g['failure'] or {}).get('kind'),
                      'record': _moves(g['match_record_json']),
                      'transcript_truncated': g['transcript'].get('truncated'),
                      'median_depth': [_median(d for d, _ in x) for x in search],
                      'median_nodes': [_median(n for _, n in x) for x in search],
                      'searches': [len(x) for x in search]})
    if wins != list(s['wins']) or draws != s['draws']:
        raise ValueError('native summary %s/%s disagrees with game conclusions %s/%s' % (s['wins'], s['draws'], wins, draws))
    complete = s['unscored'] == 0 and s['games'] == 2
    score = (s['wins'][0] + 0.5 * s['draws']) / 2 if complete else None
    return {'opening': s['opening_id'], 'wins': s['wins'], 'draws': s['draws'], 'unscored': s['unscored'],
            'failures': s['failures'], 'score': score, 'games': games,
            'engine_sha256': [i['executable']['sha256'] for i in report['engine_identities']]}


def opening_order(count, seed):
    """Fixed pseudo-random opening order, so any prefix of the run samples all families."""
    import random
    order = list(range(count))
    random.Random(seed).shuffle(order)
    return order


class Match:
    def __init__(self, args):
        self.args = args
        self.out = Path(args.out)
        self.out.mkdir(parents=True, exist_ok=False)
        (self.out / 'pairs').mkdir()
        self.openings = load_json(args.openings)['openings']
        self.order = opening_order(len(self.openings), args.seed)
        self.sprt = stats.Sprt(args.elo0, args.elo1, args.alpha, args.beta,
                               max(args.min_pairs, 2 * args.workers))
        self.lock = threading.Lock()
        self.pairs, self.failed, self.cancelled = [], [], []
        self.decision = None
        self.stop = threading.Event()
        self.running = {}
        self.records = gzip.open(self.out / 'games.jsonl.gz', 'wt')

    def _kill(self, p):
        try:
            os.killpg(p.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass

    def run_pair(self, index):
        opening = self.openings[self.order[index % len(self.openings)]]
        entry = {'index': index, 'opening': opening['id']}
        try:
            self._run_pair(index, opening, entry)
        except Exception as exc:  # recorded as a failed pair; the run continues
            entry.update(score=None, error='harness: %r' % exc)
            with self.lock:
                self.records.write(json.dumps(entry) + '\n')
                self.failed.append(entry)
                self.update()

    def _run_pair(self, index, opening, entry):
        if self.stop.is_set():
            return
        label = 'pair-%05d' % index
        dest = self.out / 'pairs' / label
        dest.mkdir()
        cfg = match_config(self.args, opening, label)
        (dest / 'config.json').write_text(json.dumps(cfg, indent=1))
        started = time.monotonic()
        with open(dest / 'stderr.log', 'wb') as err:
            p = subprocess.Popen([self.args.runner, 'run', str(dest / 'config.json'), str(dest / 'report.json')],
                                 stdout=subprocess.DEVNULL, stderr=err, start_new_session=True)
            with self.lock:
                self.running[index] = p
                if self.stop.is_set():
                    self._kill(p)
            _, status, usage = os.wait4(p.pid, 0)
            p.returncode = os.waitstatus_to_exitcode(status)
            with self.lock:
                self.running.pop(index, None)
        entry.update(wall_seconds=time.monotonic() - started, cpu_seconds=usage.ru_utime + usage.ru_stime,
                     returncode=p.returncode)
        if self.stop.is_set() and p.returncode != 0:
            with self.lock:
                self.cancelled.append(entry)
            shutil.rmtree(dest)
            return
        report_path = dest / 'report.json'
        if p.returncode == 0 and report_path.exists():
            raw = report_path.read_bytes()
            entry.update(summarize_pair(json.loads(raw), cfg))
            if self.args.keep_reports:
                (dest / 'report.json.gz').write_bytes(gzip.compress(raw, mtime=0))
            report_path.unlink()
        else:
            entry.update(score=None, error=(dest / 'stderr.log').read_text()[-2000:])
        if entry['score'] is not None and not self.args.keep_reports:
            shutil.rmtree(dest)
        with self.lock:
            entry['after_decision'] = self.decision is not None
            self.records.write(json.dumps(entry) + '\n')
            if entry['score'] is None:
                self.failed.append(entry)
            else:
                self.pairs.append(entry)
            self.update()

    def counts(self, pairs=None):
        return stats.pentanomial(p['score'] for p in (self.pairs if pairs is None else pairs))

    def update(self):
        counts = self.counts()
        st = self.sprt.status(counts)
        n = len(self.pairs)
        if self.decision is None:
            result = None
            if st['decision']:
                result = st['decision']
            elif n + len(self.failed) >= self.args.max_pairs:
                result = 'max_pairs'
            elif len(self.failed) > max(2, self.args.max_failure_fraction * (n + len(self.failed))):
                result = 'too_many_failures'
            if result:
                self.decision = {'result': result, 'pairs': n, 'failed_pairs': len(self.failed),
                                 'pentanomial': counts, 'sprt': st, 'estimate': stats.estimate(counts)}
                self.stop.set()
                for p in self.running.values():
                    self._kill(p)
        e = stats.estimate(counts)
        key = (n, self.decision is not None)
        if n and (n % self.args.progress == 0 or self.decision) and key != getattr(self, '_printed', None):
            self._printed = key
            fmt = lambda x: '%+.1f' % x if x is not None else 'inf'
            print('[%s] pairs %d failed %d penta %s score %.3f elo %s [%s, %s] llr %.2f (%.2f, %.2f)' % (
                time.strftime('%H:%M:%S'), n, len(self.failed), counts, e['score'], fmt(e['elo']), fmt(e['elo_low']),
                fmt(e['elo_high']), st['llr'], st['lower'], st['upper']), flush=True)

    def run(self):
        started_utc, started = utc(), time.monotonic()
        idents = [engine_identity(self.args.candidate), engine_identity(self.args.baseline)]
        workers = self.args.workers
        try:
            with concurrent.futures.ThreadPoolExecutor(workers) as pool:
                futures = set()
                index = 0
                while not self.stop.is_set():
                    while len(futures) < workers and not self.stop.is_set():
                        futures.add(pool.submit(self.run_pair, index))
                        index += 1
                    done, futures = concurrent.futures.wait(futures, return_when=concurrent.futures.FIRST_COMPLETED)
                    for f in done:
                        f.result()
                for f in futures:
                    f.result()
        finally:
            self.stop.set()
            with self.lock:
                for p in self.running.values():
                    self._kill(p)
            self.records.close()
        wall = time.monotonic() - started
        counts = self.counts()
        games = [g for p in self.pairs for g in p['games']]
        everything = self.pairs + self.failed + self.cancelled
        result = {
            'kind': 'mshogi-strength-match', 'schema': 2, 'label': self.args.label,
            'candidate': idents[0], 'baseline': idents[1], 'runner': engine_identity(self.args.runner),
            'openings_file': self.args.openings, 'openings_sha256': file_sha256(self.args.openings),
            'opening_order_seed': self.args.seed,
            'time_control': ({'movetime_ms': self.args.movetime} if self.args.base is None else
                             {'base_seconds': self.args.base, 'increment_seconds': self.args.increment}),
            'sprt': {'elo0': self.args.elo0, 'elo1': self.args.elo1, 'alpha': self.args.alpha, 'beta': self.args.beta,
                     'min_pairs': self.sprt.min_pairs,
                     'model': 'logistic Elo; GSPRT normal approximation on paired-opening scores; '
                              'variance regularized with two prior pairs at .25 and .75'},
            'max_pairs': self.args.max_pairs, 'workers': workers, 'environment': environment(),
            'started_utc': started_utc, 'ended_utc': utc(),
            'decision': self.decision,
            'all_completed': {'note': 'includes pairs that finished after the decision; the decision is authoritative',
                              'pentanomial': counts, 'sprt': self.sprt.status(counts),
                              'estimate': stats.estimate(counts),
                              'pairs_after_decision': sum(p.get('after_decision', False) for p in self.pairs)},
            'games': {'scored_pairs': len(self.pairs), 'scored_games': 2 * len(self.pairs),
                      'candidate_wins': sum(p['wins'][0] for p in self.pairs),
                      'baseline_wins': sum(p['wins'][1] for p in self.pairs),
                      'draws': sum(p['draws'] for p in self.pairs),
                      'failed_pairs': len(self.failed), 'cancelled_pairs': len(self.cancelled)},
            'search': {'candidate_median_depth': _median(g['median_depth'][0] for g in games),
                       'baseline_median_depth': _median(g['median_depth'][1] for g in games),
                       'candidate_median_nodes': _median(g['median_nodes'][0] for g in games),
                       'baseline_median_nodes': _median(g['median_nodes'][1] for g in games)},
            'compute': {'wall_seconds': wall,
                        'pair_wall_seconds': sum(p.get('wall_seconds', 0) for p in everything),
                        'cpu_seconds': sum(p.get('cpu_seconds', 0) for p in everything),
                        'cancelled_cpu_seconds': sum(p.get('cpu_seconds', 0) for p in self.cancelled),
                        'note': 'CPU is rusage of each mshogi-match process and its reaped engines, cancelled pairs included'},
            'failures': [{k: f.get(k) for k in ('index', 'opening', 'returncode', 'error', 'unscored', 'failures')}
                         for f in self.failed],
        }
        write_new(self.out / 'result.json', result)
        d = self.decision or {}
        e = d.get('estimate') or {}
        fmt = lambda x: '%+.1f' % x if x is not None else 'inf'
        print('match %s: decision %s at %d pairs, elo %s [%s, %s]; all completed +%d -%d =%d; wall %.0f s, cpu %.0f s' % (
            self.args.label, d.get('result'), d.get('pairs', 0), fmt(e.get('elo')), fmt(e.get('elo_low')),
            fmt(e.get('elo_high')), result['games']['candidate_wins'], result['games']['baseline_wins'],
            result['games']['draws'], wall, result['compute']['cpu_seconds']))
        return result

def match(args):
    for path in (args.candidate, args.baseline, args.runner):
        if not os.access(path, os.X_OK):
            raise SystemExit('not executable: ' + path)
    return Match(args).run()


# ---------------------------------------------------------------- build-engine

def build_engine(args):
    rev = git('rev-parse', '--verify', args.revision + '^{commit}')
    if not rev:
        raise SystemExit('unknown revision ' + args.revision)
    root = REPO / 'target' / 'strength'
    dest = root / 'engines' / rev[:12]
    exe = dest / 'mshogi'
    if exe.exists() and not args.force:
        print(exe)
        return
    src = root / 'src' / rev[:12]
    if src.exists():
        shutil.rmtree(src)
    src.mkdir(parents=True)
    archive = subprocess.Popen(['git', '-C', str(REPO), 'archive', '--format=tar', rev], stdout=subprocess.PIPE)
    extract = subprocess.Popen(['tar', '-x', '-f', '-', '-C', str(src)], stdin=archive.stdout)
    archive.stdout.close()  # git archive receives SIGPIPE if tar stops early
    if extract.wait() != 0 or archive.wait() != 0:
        raise SystemExit('source extraction failed')
    env = dict(os.environ, CARGO_TARGET_DIR=str(root / 'build-target'))
    subprocess.run(['cargo', 'build', '--release', '--locked', '-p', 'mshogi-console', '--bin', 'mshogi'],
                   cwd=src, env=env, check=True)
    rustc = subprocess.run(['rustc', '--version'], cwd=src, capture_output=True, text=True).stdout.strip()
    dest.mkdir(parents=True, exist_ok=True)
    shutil.copy2(root / 'build-target' / 'release' / 'mshogi', exe)
    Path(str(exe) + '.identity.json').write_text(json.dumps(
        {'source_revision': rev, 'rustc': rustc, 'profile': 'release', 'built_utc': utc(),
         'sha256': file_sha256(exe), 'host': platform.machine() + '-' + platform.system()}, indent=1) + '\n')
    shutil.rmtree(src)
    print(exe)


# ---------------------------------------------------------------- main

def main(argv=None):
    ap = argparse.ArgumentParser(prog='python3 -m tools.strength.cli', description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest='command', required=True)

    b = sub.add_parser('bench', help='nodes per second and time to fixed depth on a fixed position set')
    b.add_argument('--engine', default='target/release/mshogi')
    b.add_argument('--positions', default=str(DATA / 'bench-chess-v1.json'))
    b.add_argument('--repeat', type=int, default=3)
    b.add_argument('--timeout', type=float, default=300.0)
    b.add_argument('--out')
    b.set_defaults(func=bench)

    u = sub.add_parser('blunders', help='development blunder suite at a fixed move time')
    u.add_argument('--engine', default='target/release/mshogi')
    u.add_argument('--suite', default=str(DATA / 'chess-blunders-msh161.json'))
    u.add_argument('--movetime', type=int, default=1000, help='milliseconds per position')
    u.add_argument('--workers', type=int, default=4)
    u.add_argument('--out')
    u.set_defaults(func=blunders)

    m = sub.add_parser('match', help='candidate against frozen predecessor, paired openings, sequential stop')
    m.add_argument('--candidate', required=True)
    m.add_argument('--baseline', required=True)
    m.add_argument('--candidate-option', action='append', default=[], type=lambda s: tuple(s.split('=', 1)),
                   metavar='NAME=VALUE')
    m.add_argument('--baseline-option', action='append', default=[], type=lambda s: tuple(s.split('=', 1)),
                   metavar='NAME=VALUE')
    m.add_argument('--runner', default='target/release/mshogi-match')
    m.add_argument('--openings', default=str(DATA / 'openings-msh161.json'))
    m.add_argument('--movetime', type=int, default=200, help='fixed milliseconds per move (default control)')
    m.add_argument('--base', type=float, help='clock mode: initial seconds per side (replaces --movetime)')
    m.add_argument('--increment', type=float, default=0.0, help='clock mode: Fischer increment seconds')
    m.add_argument('--elo0', type=float, default=0.0)
    m.add_argument('--elo1', type=float, default=15.0)
    m.add_argument('--alpha', type=float, default=0.05)
    m.add_argument('--beta', type=float, default=0.05)
    m.add_argument('--max-pairs', type=int, default=600, help='cap on attempted pairs (scored or failed)')
    m.add_argument('--min-pairs', type=int, default=30, help='no SPRT decision before this many scored pairs (and 2 x workers)')
    m.add_argument('--seed', type=int, default=163, help='opening order seed')
    m.add_argument('--max-failure-fraction', type=float, default=0.02)
    m.add_argument('--workers', type=int, default=default_workers())
    m.add_argument('--progress', type=int, default=20, help='print status every N scored pairs')
    m.add_argument('--keep-reports', action='store_true', help='keep full native reports for every pair')
    m.add_argument('--label', default='match')
    m.add_argument('--out', required=True, help='new directory for this run')
    m.set_defaults(func=match)

    e = sub.add_parser('build-engine', help='build a frozen mshogi engine from a Git revision into target/strength')
    e.add_argument('revision')
    e.add_argument('--force', action='store_true')
    e.set_defaults(func=build_engine)

    args = ap.parse_args(argv)
    args.func(args)


if __name__ == '__main__':
    main()
