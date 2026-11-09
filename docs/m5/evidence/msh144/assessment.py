#!/usr/bin/env python3
"""Bounded MSH-144 orchestration adapted from the preserved MSH-139 helper. Rules/outcomes/replay remain in the native runner."""
import argparse
from contextlib import contextmanager
from datetime import datetime, timezone
import gzip
import hashlib
import io
import json
import math
import os
from pathlib import Path
import random
import signal
import subprocess
import time
import zipfile

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
WEIGHTS = [0, .25, .5, .75, 1]
SEED = 'msh144-progress-2026-11-09-v1'
DEFAULT_RESOURCE_CAP_SECONDS = 900
CLEANUP_SECONDS = 5
REPLAY_SECONDS = 30
FINALIZE_SECONDS = 5
MIN_PAIR_SECONDS = 180
MIN_SMOKE_SECONDS = 20


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(path):
    return json.loads(Path(path).read_text())


def save(path, value):
    path = Path(path)
    temporary = path.with_suffix(path.suffix + '.tmp')
    with temporary.open('w') as f:
        json.dump(value, f, indent=2, allow_nan=False)
        f.write('\n'); f.flush(); os.fsync(f.fileno())
    temporary.replace(path)


def utc():
    return datetime.now(timezone.utc).isoformat()


@contextmanager
def lock():
    import fcntl
    # OS releases this advisory lock on interruption. Durable inflight receipt
    # separately prevents stale work being silently forgotten after a crash.
    with (HERE/'.assessment.lock').open('a+') as f:
        fcntl.flock(f, fcntl.LOCK_EX | fcntl.LOCK_NB)
        try:
            yield
        finally:
            fcntl.flock(f, fcntl.LOCK_UN)


def prepare_book(archive, destination):
    import chess.pgn
    z = zipfile.ZipFile(archive)
    text = io.StringIO(z.read('8moves_v3.pgn').decode())
    roots, seen, excluded = [], set(), {}
    while (game := chess.pgn.read_game(text)) is not None:
        moves = list(game.mainline_moves()); board = game.end().board()
        reason = ('nonstandard_start' if game.board().fen() != chess.STARTING_FEN or game.headers.get('Variant', 'Standard') not in ['Standard','Chess'] else
                  'parse' if game.errors else 'length' if len(moves) != 16 else
                  'terminal' if board.is_game_over(claim_draw=True) else None)
        key = ' '.join(board.fen(en_passant='legal').split()[:4])
        reason = reason or ('duplicate_final_position' if key in seen else None)
        if reason:
            excluded[reason] = excluded.get(reason, 0) + 1
            continue
        seen.add(key)
        encoded = ' '.join(m.uci() for m in moves)
        h = hashlib.sha256((SEED + encoded).encode()).hexdigest()
        roots.append({'id': 'book-' + h[:16], 'fen': None,
                      'moves': encoded.split(), 'phase': 'discovery' if int(h, 16) % 2 == 0 else 'confirmation'})
    Path(destination).write_bytes(gzip.compress(json.dumps(roots).encode(), mtime=0))
    return {'archive_sha256': digest(archive), 'roots_sha256': digest(destination),
            'roots': len(roots), 'excluded': excluded,
            'phases': {p: sum(r['phase'] == p for r in roots) for p in ['discovery','confirmation']}}


def draw_root(book, phase, opponent, ordinal):
    roots = [r for r in book if r['phase'] == phase]
    # Seed each stream, advance without consuming a different opponent's draws.
    rng = random.Random(SEED + '/' + phase + '/' + opponent)
    selected = None
    for _ in range(ordinal + 1):
        selected = rng.choice(roots)
    return {k: v for k, v in selected.items() if k != 'phase'}


def fresh_book(bookpath, historical_ledger, destination):
    """Exclude every prior attempted root before drawing; preserve old partitions.

    Input is the frozen MSH-139 population, not a newly seeded reparse. Its IDs
    already identify distinct final board states, including side/rights/EP.
    Smoke, failed and incomplete historical attempts count as prior exposure.
    """
    if Path(destination).resolve() in [Path(bookpath).resolve(), Path(historical_ledger).resolve()]:
        raise ValueError('Fresh population must not overwrite historical evidence')
    history = read(historical_ledger)
    if digest(bookpath) != history['book']['roots_sha256']:
        raise ValueError('Historical opening population changed')
    roots = json.loads(gzip.decompress(Path(bookpath).read_bytes()))
    seen = {r['opening'] for r in history['runs']}
    ids = {r['id'] for r in roots}
    if not seen <= ids:
        raise ValueError('Historical attempted root missing from frozen population')
    fresh = [r for r in roots if r['id'] not in seen]
    if not all(any(r['phase'] == phase for r in fresh) for phase in ['discovery', 'confirmation']):
        raise ValueError('Fresh opening phase is empty')
    Path(destination).write_bytes(gzip.compress(json.dumps(fresh).encode(), mtime=0))
    return {'path': str(Path(destination).resolve()), 'roots_sha256': digest(destination),
            'parent_roots_sha256': digest(bookpath),
            'historical_ledger_sha256': digest(historical_ledger),
            'excluded_prior_root_ids': sorted(seen), 'roots': len(fresh),
            'phases': {p: sum(r['phase'] == p for r in fresh) for p in ['discovery', 'confirmation']}}


def resource_remaining(ledger):
    """One cumulative allowance, never reset by opponent, phase or failed run.

    Root records any separately executed preflight/diagnostic work in
    resource_extra_seconds. This helper accounts every launch, replay and cleanup
    in runs[].seconds, including failed and smoke attempts.
    """
    cap = ledger.get('resource_cap_seconds', DEFAULT_RESOURCE_CAP_SECONDS)
    extra = ledger.get('resource_extra_seconds', 0)
    durations = [r['seconds'] for r in ledger['runs']]
    if not all(isinstance(n, (int, float)) and not isinstance(n, bool)
               and math.isfinite(n) and n >= 0 for n in [cap, extra, *durations]):
        raise ValueError('Invalid cumulative resource accounting')
    return cap - extra - sum(durations)


def admit_resource(ledger, smoke=False):
    remaining = resource_remaining(ledger)
    reserve = CLEANUP_SECONDS + REPLAY_SECONDS + FINALIZE_SECONDS
    minimum = MIN_SMOKE_SECONDS if smoke else MIN_PAIR_SECONDS
    if remaining < minimum + reserve:
        raise ValueError('Insufficient remaining aggregate budget for a new pair; do not launch or reset allowance')
    return remaining


def configuration(build, opponent, opening, smoke=False):
    player = build['candidate']; ref = build['references'][opponent]
    return {'schema_version': 1, 'id': 'msh144-' + opponent + '-' + opening['id'],
            'engines': [
                {'path': player['path'], 'args': ['uci'], 'options': [], 'model_path': None,
                 'model_identity': player['model_identity'], 'deterministic': False},
                {'path': ref['path'], 'args': ref.get('args', []), 'options': ref['options'],
                 'model_path': None, 'model_identity': ref['name'] + '/' + ref['source'], 'deterministic': False}],
            'openings': [opening], 'paired': True, 'claim_policy': 'auto_claim', 'time_control': 'untimed',
            'work': {'depth': None, 'nodes': None, 'movetime_ms': 1000}, 'max_plies': 20 if smoke else 512,
            'watchdogs': {'handshake_ms': 5000, 'ready_ms': 5000, 'response_ms': 10000, 'write_ms': 2000, 'cleanup_ms': 1000},
            'io': {'line_bytes': 4096, 'queued_lines': 32, 'transcript_bytes': 2097152, 'stderr_bytes': 16384, 'command_bytes': 4096}}


def audit_report(outer, expected, build, opponent):
    raw = outer['report_json'].encode()
    if outer['protocol'] != 'uci' or hashlib.sha256(raw).hexdigest() != outer['report_sha256']:
        raise ValueError('Report protocol/digest mismatch')
    report = json.loads(raw)
    if report['config'] != expected or len(report['games']) != 2:
        raise ValueError('Different configuration or incomplete pair')
    env = outer['environment']
    if env['harness_revision'] != build['runner']['source'] or env['harness_dirty_sha256'] != hashlib.sha256(b'').hexdigest():
        raise ValueError('Unexpected harness build')
    for i, artifact in enumerate([build['candidate'], build['references'][opponent]]):
        if report['engine_identities'][i]['executable']['sha256'] != artifact['sha256']:
            raise ValueError('Different participant binary')
    healthy = True
    for index, game in enumerate(report['games']):
        if game['pairing'] != index or game['engines_by_color'] != ([0, 1] if index == 0 else [1, 0]):
            raise ValueError('Different color assignment')
        healthy &= not game['failure'] and not game['diagnostic_failures'] and not game['transcript']['truncated']
        healthy &= len(game['engines']) == 2
        for child in game['engines']:
            healthy &= not child['identity_truncated'] and not child['cleanup_failures']
            healthy &= not child['stderr']['truncated']  # ordinary bounded stderr is diagnostic, not failure
            healthy &= bool(child['exit'] and child['exit']['success'])
        commands = [e['text'] for e in game['transcript']['entries'] if e['direction'] == 'command' and e['text'].startswith('go')]
        if not commands or any(c != 'go movetime 1000' for c in commands):
            raise ValueError('Different effective search limits')
    s = report['summaries'][0]
    complete = bool(healthy and all(g['stop'] == 'chess_conclusion' for g in report['games']) and s['unscored'] == 0 and sum(s['wins']) + s['draws'] == 2)
    return {'healthy': bool(healthy), 'complete': complete, 'summary': s,
            'pair_score': (s['wins'][0] + s['draws']/2)/2 if complete else None,
            'stops': [g['stop'] for g in report['games']]}


def verify_build(build, opponent):
    if not build['references'][opponent].get('eligible', False):
        raise ValueError('Reference has not passed fair-condition preflight')
    for a in [build['candidate'], build['runner'], build['references'][opponent]]:
        if digest(a['path']) != a['sha256']:
            raise ValueError('Frozen binary changed')
    if subprocess.check_output(['git','diff','--binary',build['candidate']['source'],'--','crates','Cargo.toml','Cargo.lock','rust-toolchain.toml'],cwd=REPO):
        raise ValueError('Production changed during assessment')


def group_alive(group):
    try:
        os.killpg(group, 0)
        return True
    except ProcessLookupError:
        return False


def terminate(child, deadline):
    # The leader may already have exited while descendants still hold pipes.
    try: os.killpg(child.pid, signal.SIGKILL)
    except ProcessLookupError: pass
    output = child.communicate(timeout=max(0, deadline-time.monotonic()))
    while group_alive(child.pid) and time.monotonic() < deadline:
        time.sleep(.02)
    return output, not group_alive(child.pid)


def invoke(ledger, opponent, phase, reason, smoke=False):
    if ledger.get('inflight'):
        raise ValueError('Unreconciled prior process; inspect PID/group and preserve accounting first')
    if any(r.get('phase') != 'smoke' and not r.get('complete') and not r.get('resolution') for r in ledger['runs']):
        raise ValueError('Prior incomplete pair needs explicit investigation, not automatic replacement')
    if ledger.get('engineering_accepted') is not True:
        raise ValueError('Combined engineering acceptance is required before any engine launch')
    remaining = admit_resource(ledger, smoke)
    played={r['opponent'] for r in ledger['runs'] if r['phase']!='smoke'}
    if not smoke and opponent not in played and len(played)>=9:
        raise ValueError('Reconcile supplemental alpha before a tenth reference')
    build = ledger['build']; verify_build(build, opponent)
    freeze = ledger['freeze']
    if digest(__file__) != freeze['code_sha256'] or digest(HERE/'../../chess-progress-assessment.md') != freeze['method_sha256']:
        raise ValueError('Frozen methods changed; review and record a superseding method before more games')
    bookpath = Path(ledger['book']['path'])
    if digest(bookpath) != ledger['book']['roots_sha256']:
        raise ValueError('Opening population changed')
    ordinal = sum(r['opponent'] == opponent and r['phase'] == phase for r in ledger['runs'])
    opening = draw_root(json.loads(gzip.decompress(bookpath.read_bytes())), 'discovery' if smoke else phase, opponent, ordinal)
    config = configuration(build, opponent, opening, smoke)
    folder = HERE/f"pair-{len(ledger['runs']):03}-{phase}-{opponent}"
    folder.mkdir(exist_ok=False); save(folder/'config.json', config)
    reportpath = folder/'report.json'
    receipt = {'phase':phase, 'opponent':opponent, 'ordinal':ordinal,'opening':opening['id'],
               'reason':reason,'started_utc':utc(),'healthy':False,'complete':False,'pair_score':None,
               'folder':folder.name, 'config_sha256':digest(folder/'config.json')}
    start = time.monotonic(); deadline = start + remaining
    child = None; settled = False; receipt_path = HERE/'verification.json'
    receipt.update(resource_cap_seconds=ledger.get('resource_cap_seconds', DEFAULT_RESOURCE_CAP_SECONDS),
                   remaining_at_admission_seconds=remaining,
                   cleanup_replay_finalize_reserve_seconds=CLEANUP_SECONDS+REPLAY_SECONDS+FINALIZE_SECONDS)
    cleanup_deadline = None
    def cleanup():
        nonlocal cleanup_deadline
        if cleanup_deadline is None:
            cleanup_deadline = min(time.monotonic()+CLEANUP_SECONDS, deadline-FINALIZE_SECONDS)
        return terminate(child, cleanup_deadline)
    ledger['inflight'] = {**receipt, 'launcher_pid':os.getpid(), 'process_group':None}
    save(receipt_path, ledger)
    try:
        run_seconds = deadline-time.monotonic()-CLEANUP_SECONDS-REPLAY_SECONDS-FINALIZE_SECONDS
        if run_seconds <= 0:
            raise TimeoutError('Aggregate allowance consumed before process launch')
        child = subprocess.Popen([build['runner']['path'],'experiment-external','uci',str(folder/'config.json'),str(reportpath)],
                                 stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
        ledger['inflight']['process_group'] = child.pid; save(receipt_path, ledger)
        try:
            run_seconds = max(0, deadline-time.monotonic()-CLEANUP_SECONDS-REPLAY_SECONDS-FINALIZE_SECONDS)
            stdout, stderr = child.communicate(timeout=min(75, run_seconds) if smoke else run_seconds)
        except subprocess.TimeoutExpired:
            receipt['outer_timeout'] = True
            receipt['budget_limited_timeout'] = not smoke
            (stdout, stderr), settled = cleanup()
        if not settled:
            settled = not group_alive(child.pid)
            if not settled:
                (stdout, stderr), settled = cleanup()
                receipt['residual_group_cleanup'] = True
        receipt.update(returncode=child.returncode, stdout=stdout.decode(errors='replace'), stderr=stderr.decode(errors='replace'))
        if reportpath.exists():
            raw = reportpath.read_bytes()
            (folder/'report.json.gz').write_bytes(gzip.compress(raw, mtime=0))
            receipt['report_sha256'] = hashlib.sha256(raw).hexdigest()
            receipt.update(audit_report(json.loads(raw), config, build, opponent))
            replay_seconds = min(REPLAY_SECONDS, deadline-time.monotonic()-FINALIZE_SECONDS)
            if replay_seconds <= 0:
                raise TimeoutError('Aggregate allowance exhausted before native replay')
            replay = subprocess.run([build['runner']['path'],'experiment-external-replay',str(reportpath)],capture_output=True,text=True,timeout=replay_seconds)
            receipt.update(replay_returncode=replay.returncode,replay_stderr=replay.stderr)
            if replay.returncode or receipt['returncode'] or not settled or receipt.get('residual_group_cleanup'):
                receipt['complete'] = receipt['healthy'] = False
                receipt['pair_score'] = None
            reportpath.unlink()
    except BaseException as error:
        receipt['error'] = repr(error); receipt['complete'] = receipt['healthy'] = False; receipt['pair_score'] = None
        if child is not None:
            try:
                _, settled = cleanup()
            except subprocess.TimeoutExpired:
                settled = False
        raise
    finally:
        receipt['seconds'] = time.monotonic()-start; receipt['ended_utc'] = utc()
        receipt['process_group_settled'] = settled if child is not None else True
        ledger['runs'].append(receipt)
        ledger['inflight'] = None if child is None or settled else ledger['inflight']
        ledger['active_seconds'] = sum(r['seconds'] for r in ledger['runs'])
        ledger['resource_total_seconds'] = ledger['active_seconds'] + ledger.get('resource_extra_seconds', 0)
        save(receipt_path, ledger)
    return receipt


def posterior(scores, anchor, prior=.5, draws=100000):
    import numpy as np
    counts = [scores.count(x) for x in WEIGHTS]
    samples = np.random.default_rng(139).dirichlet(np.array(counts)+prior, size=draws) @ np.array(WEIGHTS)
    indices = anchor + 400*np.log10(samples/(1-samples))
    return {'counts':counts,'prior_per_bin':prior,'score_mean':float(np.mean(samples)),
            'score_credible95':np.quantile(samples,[.025,.975]).tolist(),
            'index_median':float(np.median(indices)), 'index_credible95':np.quantile(indices,[.025,.975]).tolist()}


def posterior_change(baseline_scores, fresh_scores, prior=.5, draws=100000):
    """Independent paired-score posteriors; no shared draws or pooled families.

    The same exact reference release/anchor must underlie both samples. Its
    anchor cancels in this conditional difference; transfer/style and published
    rating errors do not disappear from an absolute strength interpretation.
    This is a conditional Bayesian interval, not a frequentist gain test.
    """
    import numpy as np
    if not baseline_scores or not fresh_scores:
        raise ValueError('Both baseline and fresh completed pairs are required')
    if any(score not in WEIGHTS for score in [*baseline_scores, *fresh_scores]):
        raise ValueError('Expected completed opening-pair scores')
    streams = np.random.SeedSequence(144).spawn(2)
    samples = []
    for scores, stream in zip([baseline_scores, fresh_scores], streams):
        counts = np.array([scores.count(x) for x in WEIGHTS])
        p = np.random.default_rng(stream).dirichlet(counts+prior, size=draws) @ np.array(WEIGHTS)
        samples.append((p, 400*np.log10(p/(1-p))))
    score_change = samples[1][0]-samples[0][0]
    index_change = samples[1][1]-samples[0][1]
    return {'baseline_pairs':len(baseline_scores), 'fresh_pairs':len(fresh_scores),
            'prior_per_bin':prior, 'independent_streams':True,
            'score_change_median':float(np.median(score_change)),
            'score_change_credible95':np.quantile(score_change,[.025,.975]).tolist(),
            'conditional_index_change_median':float(np.median(index_change)),
            'conditional_index_change_credible95':np.quantile(index_change,[.025,.975]).tolist(),
            'probability_positive_conditional_change':float(np.mean(index_change > 0))}


def comparison_condition_differences(baseline, ledger, opponent):
    """Only comparable reference conditions support an automatic cohort delta."""
    old = baseline['build']['references'].get(opponent, {})
    fresh = ledger['build']['references'].get(opponent, {})
    conditions = [
        ('reference_source', old.get('source'), fresh.get('source')),
        ('reference_executable_sha256', old.get('sha256'), fresh.get('sha256')),
        ('reference_options', old.get('options', []), fresh.get('options', [])),
        ('reference_args', old.get('args', []), fresh.get('args', [])),
        ('rated_anchor', baseline['catalog'].get(opponent, {}).get('rating'),
         ledger['catalog'].get(opponent, {}).get('rating')),
    ]
    return [{'condition':name, 'baseline':before, 'fresh':after}
            for name, before, after in conditions
            if before is None or after is None or before != after]


def analysis(ledger):
    result = {}
    baseline = None
    if ledger.get('baseline'):
        reference = ledger['baseline']
        if digest(reference['path']) != reference['sha256']:
            raise ValueError('Historical baseline ledger changed')
        baseline = read(reference['path'])
    if len({r['opponent'] for r in ledger['runs'] if r['phase']!='smoke'})>9:
        raise ValueError('Reconcile supplemental alpha spending before a tenth played reference')
    for opponent, ref in ledger['catalog'].items():
        for phase in ['discovery','confirmation']:
            scores = [r['pair_score'] for r in ledger['runs'] if r['opponent']==opponent and r['phase']==phase and r['complete']]
            if not scores: continue
            item = posterior(scores, ref['rating'])
            item['prior_sensitivity'] = [posterior(scores,ref['rating'],p) for p in [.2,1]]
            try:
                from confseq.predmix import predmix_empbern_twosided_cs
                import numpy as np
                lo, hi = predmix_empbern_twosided_cs(np.asarray(scores),alpha=.05/18,running_intersection=True)
                item['supplemental_anytime_score_bounds'] = [float(lo[-1]),float(hi[-1])]
            except ImportError:
                item['supplemental_anytime_score_bounds'] = 'not available; install pinned upstream source on PYTHONPATH'
            item['pairs'] = len(scores); item['actual_points'] = sum(scores)*2
            if baseline is not None:
                differences = comparison_condition_differences(baseline, ledger, opponent)
                if differences:
                    item['historical_confirmation_comparison_omitted'] = {
                        'reason':'Reference conditions differ or are missing; no automatic historical delta.',
                        'differences':differences}
                else:
                    old_scores = [r['pair_score'] for r in baseline['runs']
                                  if r['opponent']==opponent and r['phase']=='confirmation' and r['complete']]
                    if old_scores:
                        item['historical_confirmation_comparison'] = posterior_change(old_scores, scores)
                        item['historical_confirmation_comparison']['fresh_phase'] = phase
                        item['historical_confirmation_comparison']['prior_sensitivity'] = [
                            posterior_change(old_scores, scores, p) for p in [.2, 1]]
            result[opponent+'/'+phase] = item
    return result


def main():
    p=argparse.ArgumentParser();p.add_argument('mode',choices=['pair','smoke','analyze','book']);p.add_argument('--opponent');p.add_argument('--phase',choices=['discovery','confirmation'],default='discovery');p.add_argument('--reason');p.add_argument('--archive');p.add_argument('--destination');a=p.parse_args()
    if a.mode=='book': print(json.dumps(prepare_book(a.archive,a.destination),indent=2));return
    with lock():
        ledger=read(HERE/'verification.json')
        if a.mode=='analyze': print(json.dumps(analysis(ledger),indent=2));return
        if not a.reason: p.error('record an opponent/phase selection reason')
        receipt=invoke(ledger,a.opponent,'smoke' if a.mode=='smoke' else a.phase,a.reason,a.mode=='smoke')
        print(json.dumps(receipt,indent=2))


if __name__=='__main__': main()
