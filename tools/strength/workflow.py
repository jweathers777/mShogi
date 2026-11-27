"""Opening screening, tactical suites, report sampling and classical data export."""
import argparse
import gzip
import json
import math
import statistics
import subprocess
import tempfile
import time
import shutil
from pathlib import Path

from tools.experiments.core import digest, encode, file_hash
from . import texel_data
from .common import DATA, engine_identity, game_options, load_json, profile, write_new
from .internal_match import telemetry


def positive(value):
    n = int(value)
    if n <= 0:
        raise argparse.ArgumentTypeError('expected a positive integer')
    return n


def inputs(path, field, selected):
    data = load_json(path)
    if data.get('profile') not in (None, selected):
        raise ValueError('input belongs to another rules profile')
    rows = data[field]
    if not rows or len({row['id'] for row in rows}) != len(rows):
        raise ValueError('expected nonempty records with unique ids')
    return rows


def run_experiment(args, operation, payload, *settings):
    command = [args.helper, operation, *map(str, settings)]
    result = subprocess.run(command, input=encode(payload), capture_output=True, timeout=75)
    if result.returncode:
        raise ValueError(result.stderr.decode(errors='replace').strip() or 'helper failed')
    return json.loads(result.stdout)


def position(args, row, *, seed=0, plies=0, prove=False):
    return run_experiment(args, 'position', row, profile(args.game, args.profile), seed, plies,
                  'prove' if prove else 'prepare')


def search(args, record, engine=None, depth=None):
    limits = [depth or args.depth, args.nodes]
    if getattr(args, 'movetime', None):
        limits.append(args.movetime)
    result = run_experiment(args, 'search', record, engine or args.engine, *limits)
    if result['kind'] != args.game:
        raise ValueError(f"engine returned {result['kind']} for {args.game}")
    return result['result']


def board_action(result, game):
    if game == 'chess':
        return result.get('action')
    choice = result.get('choice')
    if choice is None or game not in ('shogi', 'xiangqi'):
        return choice
    return choice.get('action')


def evidence(args):
    return {'profile': profile(args.game, args.profile), 'helper': engine_identity(args.helper),
            'engine': engine_identity(args.engine), 'depth': args.depth, 'nodes': args.nodes}


def openings(args):
    if not 0 <= args.plies <= 128 or args.balance < 0 or args.seed < 0:
        raise ValueError('opening plies must be 0..128; seed and balance must be nonnegative')
    selected = profile(args.game, args.profile)
    source = inputs(args.source, 'openings', selected) if args.source else None
    seen = set()
    accepted = []
    attempts = 0
    for index in range(min(args.attempts, len(source)) if source else args.attempts):
        attempts += 1
        seed = args.seed + index
        item = source[index] if source else {'id': f'opening-{seed}'}
        row = position(args, item, seed=seed, plies=0 if source else args.plies)
        if row.get('stopped') or row['score_first_twice'] is not None:
            continue
        key = texel_data.key(row['position'], args.game)
        if key in seen:
            continue
        seen.add(key)
        record = row['root']['record']
        result = search(args, record)
        t = result.get('telemetry', {})
        score = t.get('score')
        if (board_action(result, args.game) is not None and t['completed_depth'] == args.depth
                and score is not None and abs(score) <= args.balance):
            accepted.append({'id': item['id'], 'group': item.get('group', item['id']),
                             'record': record, 'screen_score': score})
        if len(accepted) == args.count:
            break
    write_new(args.out, dict(evidence(args), openings=accepted, attempts=attempts, requested=args.count,
                             balance=args.balance, plies=0 if source else args.plies, seed=args.seed,
                             source_sha256=file_hash(args.source) if source else None))
    if len(accepted) < args.count:
        raise ValueError(f'only {len(accepted)} of {args.count} balanced openings within the attempt limit')


def partition(group):
    return 'held-out' if int(digest(group.encode())[:2], 16) % 4 == 0 else 'development'


def suite(args):
    selected = profile(args.game, args.profile)
    records = inputs(args.source, 'positions', selected)
    groups = {}
    boards = set()
    rows = []
    for item in records:
        group = item.get('group') or item.get('opening_id') or item.get('family')
        if not isinstance(group, str) or not group:
            raise ValueError(f"{item['id']}: supply the source game or opening group")
        if args.proof == 'supplied' and 'checking_answers' in item and 'answers' not in item and 'expected_moves' not in item:
            raise ValueError('incomplete checking-only answers: probe the frozen suite directly')
        if args.proof == 'supplied' and not (item.get('source') or item.get('proof')):
            raise ValueError(f"{item['id']}: supply independent answer provenance")
        row = position(args, item, prove=args.proof == 'one-ply')
        if not row['answers'] or row['score_first_twice'] is not None:
            if args.proof == 'supplied':
                raise ValueError(f"{item['id']}: no live position with independent answers")
            continue
        key = texel_data.key(row['position'], args.game)
        part = item.get('split', partition(group))
        if part not in ('development', 'held-out') or groups.setdefault(group, part) != part:
            raise ValueError(f'{group}: conflicting or unknown suite partition')
        if key in boards:
            continue
        boards.add(key)
        rows.append({'id': item['id'], 'group': group, 'split': part, 'record': row['root']['record'],
                     'answers': row['answers'], 'proof': item.get('proof') or item.get('source')
                     if args.proof == 'supplied' else 'every game board move ending in a win for its mover'})
    if {r['split'] for r in rows} != {'development', 'held-out'}:
        raise ValueError('suite requires independently grouped development and held-out rows')
    write_new(args.out, {'profile': selected, 'source_sha256': file_hash(args.source), 'positions': rows,
                         'proof': args.proof,
                         'split_rule': 'Use the explicit source split when present. Otherwise the first byte '
                                       'of SHA-256(group) modulo 4 selects held-out for 0.'})


def probe(args):
    selected = profile(args.game, args.profile)
    proofs = {}
    if args.game == 'tori':
        proofs = load_json(DATA / 'tori-tactics-v1-proofs.json')['proofs']
    rows = []
    for item in inputs(args.suite, 'positions', selected):
        if item.get('split', 'development') != args.split:
            continue
        row = position(args, item)
        if not row['answers']:
            raise ValueError(f"{item['id']}: expected independent answers")
        result = search(args, row['root']['record'])
        choice = board_action(result, args.game)
        passed = choice in row['answers']
        verdict = 'listed' if passed else 'incorrect'
        if not passed and choice is not None and 'checking_answers' in item and args.game == 'tori':
            payload = {'diagram': item['diagram'], 'action': choice}
            spelling = run_experiment(args, 'unlisted-tori', payload, item['mate_plies'], 0)['token']
            key = digest(encode(item['diagram']).rstrip(b'\n'))
            cached = proofs.get(f"{key}|{spelling}|{item['mate_plies']}|20000000|tori-mate-proof-v2")
            if cached and cached['verdict'] != 'unknown':
                verdict = cached['verdict']
            else:
                proof = run_experiment(args, 'unlisted-tori', payload, item['mate_plies'], args.proof_visits)
                verdict = {None: 'unknown', False: 'refuted', True: 'proved_win'}[proof['win']]
            passed = verdict == 'proved_win'
        rows.append({'id': item['id'], 'choice': choice, 'passed': passed, 'verdict': verdict,
                     'igui_contract_passed': row.get('igui_contract_passed'), 'telemetry': result.get('telemetry')})
    if not rows:
        raise ValueError('empty requested suite partition')
    write_new(args.out, dict(evidence(args), split=args.split, suite_sha256=file_hash(args.suite),
                             passed=sum(r['passed'] for r in rows), total=len(rows), rows=rows))


def projected(args, mode, count):
    selected = profile(args.game, args.profile)
    seen = set()
    for path in args.reports:
        # Spool the projection one report at a time. Late replay failures must
        # not deliver a partial report to the statistics or training corpus.
        with tempfile.TemporaryFile() as projection, tempfile.TemporaryFile() as expanded:
            with (gzip.open(path, 'rb') if path.suffix == '.gz' else path.open('rb')) as source:
                shutil.copyfileobj(source, expanded)
            expanded.seek(0)
            subprocess.run([args.helper, 'report', selected, mode, str(count)], stdin=expanded,
                           stdout=projection, check=True)
            projection.seek(0)
            for line in projection:
                row = json.loads(line)
                identity = (row['game_id'], row.get('ply'))
                if identity in seen:
                    raise ValueError('duplicate report in sample')
                seen.add(identity)
                yield row


def export(args):
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=False)
    position_field = texel_data.POSITION[args.game]
    outcome_field = texel_data.OUTCOME[args.game]
    with (out / 'games-0.jsonl').open('x') as stream:
        for game in projected(args, 'export', args.every):
            score = game.pop('score_first_twice')
            label = None if score is None else score / 2
            for sample in game['samples']:
                sample[position_field] = sample.pop('position')
                sample[outcome_field] = label
            game.update({outcome_field: label, 'truncated': label is None})
            stream.write(json.dumps(game, separators=(',', ':')) + '\n')
    texel_data.split(out, out / 'manifest.json', args.game, args.allow_empty_demonstration,
                     by_opening=True, profile=profile(args.game, args.profile))


def probability(result):
    score = result.get('telemetry', {}).get('score')
    if score is None:
        return None
    # Stable logistic function, including the mShogi engines' mate-distance scores.
    exp = math.exp(-abs(score) / 600)
    return 1 / (1 + exp) if score >= 0 else exp / (1 + exp)


def judge(args):
    rows = []
    losses = []
    deeper = 0
    for item in projected(args, 'judge', args.samples):
        before = search(args, item['before'])
        p = probability(before)
        after = None
        if item['terminal_score'] is None:
            after = search(args, item['after'])
            opponent = probability(after)
            q = None if opponent is None else 1 - opponent
        else:
            first = item['terminal_score'] / 2
            q = first if item['mover_first'] else 1 - first
        player = telemetry(item['observation'])
        loss = None
        if p is not None and q is not None:
            loss = max(0, p - q)
            losses.append(loss)
            deeper += bool(player and before['telemetry']['completed_depth'] > player['completed_depth'])
        rows.append(dict(item, judge_before=before, judge_after=after, p_before=p, p_after=q,
                         loss=loss, clear_blunder=loss is not None and p >= .30 and p - q >= .30))
    count = sum(r['clear_blunder'] for r in rows)
    errors = []
    for row in rows:
        if row['clear_blunder']:
            played = board_action(row['observation']['result'], args.game)
            reference = board_action(row['judge_before'], args.game)
            if played is not None and reference is not None:
                errors.append({'id': f"{row['game_id']}/{row['ply']}", 'group': row['opening_id'],
                               'split': partition(row['opening_id']), 'record': row['before'],
                               'played': played, 'judge_move': reference,
                               'p_before': row['p_before'], 'p_after': row['p_after']})
    write_new(args.out, dict(evidence(args), positions=errors, sampled=len(rows), unjudged=len(rows)-len(losses),
                             judged=len(losses), deeper_than_player=deeper, clear_blunders=count,
                             clear_blunders_per_100=100*count/len(losses) if losses else None,
                             mean_win_probability_loss=statistics.mean(losses) if losses else None, rows=rows))


def signature(args):
    rows = []
    for item in inputs(args.source, 'openings', profile(args.game, args.profile)):
        record = position(args, item)['root']['record']
        current = search(args, record)
        baseline = search(args, record, args.baseline)
        same = (board_action(current, args.game) is not None and current == baseline
                and current['telemetry']['completed_depth'] == args.depth)
        rows.append({'id': item['id'], 'identical': same, 'candidate': current, 'baseline': baseline})
    identical = all(r['identical'] for r in rows)
    write_new(args.out, dict(evidence(args), baseline=engine_identity(args.baseline), rows=rows, identical=identical))
    if not identical:
        raise ValueError('fixed-depth results differ or did not reach requested depth')


def bench(args):
    rows = []
    for item in inputs(args.source, 'positions', profile(args.game, args.profile)):
        record = position(args, item)['root']['record']
        depth = args.depth or item['depth']
        for repeat in range(args.repeat):
            start = time.monotonic()
            result = search(args, record, depth=depth)
            seconds = time.monotonic() - start
            t = result['telemetry']
            if t['completed_depth'] != depth:
                raise ValueError(f"{item['id']}: fixed depth {depth} was not reached")
            rows.append({'id': item['id'], 'repeat': repeat, 'depth': depth, 'seconds': seconds,
                         'nodes': t['nodes'], 'nodes_per_second': t['nodes']/seconds,
                         'choice': board_action(result, args.game)})
    write_new(args.out, dict(evidence(args), source_sha256=file_hash(args.source), rows=rows,
                             median_seconds=statistics.median(r['seconds'] for r in rows)))


def blunders(args):
    rows = []
    for item in inputs(args.suite, 'positions', profile(args.game, args.profile)):
        if args.split and item.get('split', 'development') != args.split:
            continue
        item = dict(item)
        if 'initial_fen' in item:
            item.update(fen=item['initial_fen'], moves=item['history'])
        played = item.get('blunder_move', item.get('played'))
        reference = item.get('reference_move', item.get('judge_move'))
        if played is None or reference is None:
            raise ValueError(f"{item['id']}: expected a played move and a reference alternative")
        item['answers'] = [played, reference]
        prepared = position(args, item)
        result = search(args, prepared['root']['record'])
        choice = board_action(result, args.game)
        wrong, alternative = prepared['answers']
        rows.append({'id': item['id'], 'choice': choice, 'avoided': choice is not None and choice != wrong,
                     'matched_reference': choice == alternative, 'telemetry': result.get('telemetry')})
    if not rows:
        raise ValueError('empty blunder screen')
    write_new(args.out, dict(evidence(args), suite_sha256=file_hash(args.suite), movetime_ms=args.movetime,
                             rows=rows, avoided=sum(r['avoided'] for r in rows),
                             matched_reference=sum(r['matched_reference'] for r in rows)))


def options(sub):
    for name, func, description in [
        ('bench', bench, 'measure time and nodes to a fixed search depth'),
        ('blunders', blunders, 'check confirmed errors and reference alternatives'),
        ('openings', openings, 'screen seeded or supplied openings at fixed depth'),
        ('suite', suite, 'construct a grouped tactical suite with independent answers'),
        ('probe', probe, 'score one partition of a tactical suite'),
        ('export', export, 'export self-play reports and split by opening'),
        ('judge', judge, 'judge sampled moves with larger fixed work'),
        ('signature', signature, 'compare fixed-depth observations from two engines'),
    ]:
        p = sub.add_parser(name, help=description)
        game_options(p)
        p.set_defaults(func=func)
        p.add_argument('--out', required=True)
        if func in (openings, probe, judge, signature, bench, blunders):
            p.add_argument('--engine', default='target/release/mshogi')
            p.add_argument('--depth', type=positive, required=func != bench)
            p.add_argument('--nodes', type=positive, required=True)
        if func in (judge, export):
            p.add_argument('reports', nargs='+', type=Path, help='match reports, optionally gzip compressed')
        if func in (openings, suite, signature, bench):
            p.add_argument('--source', required=func != openings)
        if func == bench:
            p.add_argument('--repeat', type=positive, default=3)
        elif func == blunders:
            p.add_argument('--suite', required=True)
            p.add_argument('--split', choices=['development', 'held-out'])
            p.add_argument('--movetime', type=positive, help='move budget in milliseconds')
        elif func == openings:
            p.add_argument('--seed', type=int, default=1)
            p.add_argument('--plies', type=int, default=8)
            p.add_argument('--count', type=positive, required=True)
            p.add_argument('--attempts', type=positive, required=True)
            p.add_argument('--balance', type=int, required=True, help='maximum absolute score in hundredths')
        elif func == suite:
            p.add_argument('--proof', choices=['one-ply', 'supplied'], required=True)
        elif func == probe:
            p.add_argument('--proof-visits', type=positive, default=20000, help='maximum visits for an unlisted Tori move')
            p.add_argument('--suite', required=True)
            p.add_argument('--split', required=True, choices=['development', 'held-out'])
        elif func == export:
            p.add_argument('--every', type=positive, default=4)
            p.add_argument('--allow-empty-demonstration', action='store_true')
        elif func == judge:
            p.add_argument('--samples', type=positive, default=16, help='evenly spaced moves per game')
        elif func == signature:
            p.add_argument('--baseline', required=True)
