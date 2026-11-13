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
import tarfile
import io
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
            nodes = sum(x['nodes'] for x in rows)
            seconds = sum(x['wall_seconds'] for x in rows)
            rounds.append({'positions': rows, 'nodes': nodes, 'seconds': seconds, 'nps': nodes / seconds})
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
                          'min_nps': min(r['nps'] for r in rounds), 'max_nps': max(r['nps'] for r in rounds)},
              'per_position': per_position, 'rounds': rounds}
    if args.out:
        write_new(args.out, result)
    s = result['summary']
    print('bench %s: nodes %d (%s), median %.2f s, median nps %.0f [%.0f..%.0f]' % (
        spec['name'], s['bench_nodes'], 'deterministic' if s['deterministic_nodes'] else 'NOT deterministic',
        s['median_seconds'], s['median_nps'], s['min_nps'], s['max_nps']))
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


def summarize_pair(report, cfg):
    """Pair score for engine 0 plus compact per-game evidence. None score if not fully scored."""
    s = report['summaries'][0]
    games = []
    for g in report['games']:
        search = [[], []]
        for e in g['transcript']['entries']:
            if e['direction'] == 'response' and e['text'].startswith('info ') and ' nodes ' in e['text']:
                tokens = e['text'].split()
                depth = int(tokens[tokens.index('depth') + 1]) if 'depth' in tokens else None
                search[e['engine']].append((depth, int(tokens[tokens.index('nodes') + 1])))
        games.append({'pairing': g['pairing'], 'white': g['engines_by_color'][0], 'stop': g['stop'],
                      'conclusion': g['conclusion'], 'failure': (g['failure'] or {}).get('kind'),
                      'record': _moves(g['match_record_json']),
                      'median_depth': [statistics.median(d for d, _ in x if d is not None) if x else None for x in search],
                      'median_nodes': [statistics.median(n for _, n in x) if x else None for x in search],
                      'searches': [len(x) for x in search]})
    complete = s['unscored'] == 0 and s['games'] == 2
    score = (s['wins'][0] + 0.5 * s['draws']) / 2 if complete else None
    return {'opening': s['opening_id'], 'wins': s['wins'], 'draws': s['draws'], 'unscored': s['unscored'],
            'failures': s['failures'], 'score': score, 'games': games,
            'engine_sha256': [i['executable']['sha256'] for i in report['engine_identities']]}


class Match:
    def __init__(self, args):
        self.args = args
        self.out = Path(args.out)
        self.out.mkdir(parents=True, exist_ok=False)
        (self.out / 'pairs').mkdir()
        self.openings = load_json(args.openings)['openings']
        self.sprt = stats.Sprt(args.elo0, args.elo1, args.alpha, args.beta)
        self.lock = threading.Lock()
        self.pairs, self.failed, self.cancelled = [], [], 0
        self.decision = None
        self.stop = threading.Event()
        self.running = {}
        self.records = gzip.open(self.out / 'games.jsonl.gz', 'wt')

    def run_pair(self, index):
        if self.stop.is_set():
            return
        opening = self.openings[index % len(self.openings)]
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
            _, status, usage = os.wait4(p.pid, 0)
            p.returncode = os.waitstatus_to_exitcode(status)
            with self.lock:
                self.running.pop(index, None)
        wall = time.monotonic() - started
        cpu = usage.ru_utime + usage.ru_stime
        if self.stop.is_set() and p.returncode != 0:
            with self.lock:
                self.cancelled += 1
            shutil.rmtree(dest)
            return
        entry = {'index': index, 'wall_seconds': wall, 'cpu_seconds': cpu, 'returncode': p.returncode}
        report_path = dest / 'report.json'
        if p.returncode == 0 and report_path.exists():
            raw = report_path.read_bytes()
            entry.update(summarize_pair(json.loads(raw), cfg))
            if self.args.keep_reports:
                (dest / 'report.json.gz').write_bytes(gzip.compress(raw, mtime=0))
            report_path.unlink()
        else:
            entry.update(opening=opening['id'], score=None, error=(dest / 'stderr.log').read_text()[-2000:])
        if entry['score'] is not None and not self.args.keep_reports:
            shutil.rmtree(dest)
        with self.lock:
            self.records.write(json.dumps(entry) + '\n')
            if entry['score'] is None:
                self.failed.append(entry)
            else:
                self.pairs.append(entry)
            self.update()

    def counts(self):
        return stats.pentanomial(p['score'] for p in self.pairs)

    def update(self):
        counts = self.counts()
        st = self.sprt.status(counts)
        n = len(self.pairs)
        if self.decision is None:
            if st['decision']:
                self.decision = {'result': st['decision'], 'pairs': n, 'llr': st['llr']}
            elif n + len(self.failed) >= self.args.max_pairs:
                self.decision = {'result': 'max_pairs', 'pairs': n, 'llr': st['llr']}
            elif len(self.failed) > max(2, self.args.max_failure_fraction * (n + len(self.failed))):
                self.decision = {'result': 'too_many_failures', 'pairs': n, 'llr': st['llr']}
            if self.decision:
                self.stop.set()
                for p in self.running.values():
                    try:
                        os.killpg(p.pid, signal.SIGTERM)
                    except ProcessLookupError:
                        pass
        e = stats.estimate(counts)
        key = (n, self.decision is not None)
        if n and (n % self.args.progress == 0 or self.decision) and key != getattr(self, '_printed', None):
            self._printed = key
            print('[%s] pairs %d failed %d penta %s score %.3f elo %+.1f [%+.1f, %+.1f] llr %.2f (%.2f, %.2f)' % (
                time.strftime('%H:%M:%S'), n, len(self.failed), counts, e['score'], e['elo'], e['elo_low'],
                e['elo_high'], st['llr'], st['lower'], st['upper']), flush=True)

    def run(self):
        started_utc, started = utc(), time.monotonic()
        idents = [engine_identity(self.args.candidate), engine_identity(self.args.baseline)]
        workers = self.args.workers
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
        self.records.close()
        wall = time.monotonic() - started
        counts = self.counts()
        games = [g for p in self.pairs for g in p['games']]
        cand_depth = [g['median_depth'][0] for g in games if g['median_depth'][0] is not None]
        base_depth = [g['median_depth'][1] for g in games if g['median_depth'][1] is not None]
        result = {
            'kind': 'mshogi-strength-match', 'schema': 1, 'label': self.args.label,
            'candidate': idents[0], 'baseline': idents[1], 'runner': engine_identity(self.args.runner),
            'openings_file': self.args.openings, 'openings_sha256': file_sha256(self.args.openings),
            'time_control': ({'movetime_ms': self.args.movetime} if self.args.base is None else
                             {'base_seconds': self.args.base, 'increment_seconds': self.args.increment}),
            'sprt': {'elo0': self.args.elo0, 'elo1': self.args.elo1, 'alpha': self.args.alpha, 'beta': self.args.beta,
                     'model': 'logistic Elo, GSPRT normal approximation on paired-opening scores'},
            'max_pairs': self.args.max_pairs, 'workers': workers, 'environment': environment(),
            'started_utc': started_utc, 'ended_utc': utc(),
            'decision': self.decision, 'final_sprt': self.sprt.status(counts),
            'pentanomial': counts, 'estimate': stats.estimate(counts),
            'games': {'scored_pairs': len(self.pairs), 'scored_games': 2 * len(self.pairs),
                      'candidate_wins': sum(p['wins'][0] for p in self.pairs),
                      'baseline_wins': sum(p['wins'][1] for p in self.pairs),
                      'draws': sum(p['draws'] for p in self.pairs),
                      'failed_pairs': len(self.failed), 'cancelled_pairs': self.cancelled},
            'search': {'candidate_median_depth': statistics.median(cand_depth) if cand_depth else None,
                       'baseline_median_depth': statistics.median(base_depth) if base_depth else None},
            'compute': {'wall_seconds': wall, 'pair_wall_seconds': sum(p['wall_seconds'] for p in self.pairs + self.failed),
                        'cpu_seconds': sum(p['cpu_seconds'] for p in self.pairs + self.failed),
                        'cpu_seconds_cancelled_excluded': True},
            'failures': [{k: f.get(k) for k in ('index', 'opening', 'returncode', 'error', 'unscored', 'failures')}
                         for f in self.failed],
        }
        write_new(self.out / 'result.json', result)
        e = result['estimate']
        print('match %s: decision %s; %d pairs, +%d -%d =%d, elo %+.1f [%+.1f, %+.1f]; wall %.0f s, cpu %.0f s' % (
            self.args.label, (self.decision or {}).get('result'), len(self.pairs), result['games']['candidate_wins'],
            result['games']['baseline_wins'], result['games']['draws'], e.get('elo', 0), e.get('elo_low', 0),
            e.get('elo_high', 0), wall, result['compute']['cpu_seconds']))
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
    archive = subprocess.run(['git', '-C', str(REPO), 'archive', '--format=tar', rev], capture_output=True, check=True).stdout
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        tar.extractall(src, filter='data')
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
