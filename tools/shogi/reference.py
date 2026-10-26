#!/usr/bin/env python3
"""Developer-only pinned cshogi LegalAll comparison; never a product dependency.

JSONL probe contract: stdin {root_sfen, moves:[USI,...]}, stdout exactly one
{sfen, legal_moves:[USI,...], in_check:bool}. Each invocation starts from root.
Only enumerated LegalAll moves are pushed; permissive parsers, is_legal,
is_draw and is_game_over never determine expected legality or outcomes.
"""
import argparse
import hashlib
import importlib.metadata
import json
import os
from pathlib import Path
import platform
import random
import re
import shlex
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
PIN = '6985fe5b516c34072e27c63cf429847f03193522'
FIXTURES = ROOT / 'tests/fixtures/shogi/board-rules.json'
MANIFEST = ROOT / 'docs/m4/reference/cshogi-admission.json'
USI = re.compile(r'(?:[1-9][a-i][1-9][a-i]\+?|[PLNSGBR]\*[1-9][a-i])\Z')


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def git(source, *args):
    return subprocess.check_output(['git', '-C', str(source), *args], text=True).strip()


def admit(source, path, build_log):
    import cshogi
    import cshogi._cshogi as extension
    if git(source, 'rev-parse', 'HEAD') != PIN or git(source, 'status', '--porcelain', '--untracked-files=no'):
        raise ValueError('reference source must be the exact clean pinned tracked revision')
    files = ['setup.py', 'LICENSE', 'cshogi/_cshogi.pyx', 'src/cshogi.h', 'src/generateMoves.cpp', 'src/generateMoves.hpp', 'src/position.cpp', 'src/usi.cpp', 'src/ifdef.hpp', 'src/position.hpp', 'cshogi/dlshogi/__init__.py']
    log = Path(build_log).read_text()
    if 'BAN_BLACK_REPETITION' in log or 'BAN_WHITE_REPETITION' in log:
        raise ValueError('history-ban build flags are incompatible with board-only comparison')
    compile_lines = [line.strip() for line in log.splitlines() if (' -c ' in line or ' -shared ' in line or ' -bundle ' in line)]
    if not compile_lines:
        raise ValueError('verbose successful build log with actual compiler commands is required')
    manifest = {'format': 'mshogi.shogi.reference-admission.v1', 'revision': PIN,
                'upstream': 'https://github.com/TadaoYamaoka/cshogi',
                'license': 'GPL-3.0 (LICENSE); source headers GPL-3.0-or-later; separate developer process only',
                'source_sha256': {name: sha(source / name) for name in files},
                'platform': platform.platform(), 'machine': platform.machine(),
                'python': sys.version, 'packages': {name: importlib.metadata.version(name) for name in ['cshogi', 'Cython', 'numpy', 'setuptools', 'wheel']},
                'installed_direct_url': json.loads(importlib.metadata.distribution('cshogi').read_text('direct_url.json')),
                'extension_sha256': sha(extension.__file__), 'extension_name': Path(extension.__file__).name,
                'compiler': subprocess.check_output(shlex.split(os.environ.get('CXX', 'clang++')) + ['--version'], text=True).strip(),
                'compiler_commands': compile_lines, 'build_log_sha256': sha(build_log),
                'banned_history_macros_defined': False,
                'runtime_surface': 'Board.legal_moves -> MoveList<LegalAll>; move_to_usi; push/pop enumerated integers; board state serialization/check/hash; public dlshogi raw geometric attack planes only for reference-root domain admission',
                'excluded_as_oracles': ['MoveList<Legal>', 'is_legal', 'move_from_usi', 'push_usi', 'is_draw', 'is_game_over', 'is_nyugyoku']}
    path.write_text(json.dumps(manifest, indent=2) + '\n')
    return manifest


def reference(manifest_path):
    import cshogi
    import cshogi._cshogi as extension
    manifest = json.loads(manifest_path.read_text())
    if manifest['revision'] != PIN or importlib.metadata.version('cshogi') != '1.1.0':
        raise ValueError('unadmitted reference revision/version')
    if sha(extension.__file__) != manifest['extension_sha256']:
        raise ValueError('reference binary differs: reproduce exact source build and explicitly admit its manifest')
    return cshogi


def legal(board, cshogi):
    # The only source of legal move membership. This wrapper is LegalAll at the pin.
    result = {cshogi.move_to_usi(move): move for move in board.legal_moves}
    if any(not USI.fullmatch(text) for text in result):
        raise ValueError('reference emitted a malformed USI board action')
    return result


def normalized_sfen(sfen):
    """Independent syntax/semantics normalization, not a reachability oracle."""
    if not isinstance(sfen, str) or not sfen.isascii() or len(sfen) > 512:
        raise ValueError('reference-domain: SFEN must be bounded ASCII, at most512bytes')
    fields = sfen.split()
    if sfen != ' '.join(fields):
        raise ValueError('reference-domain: comparison SFEN must use canonical single-space fields')
    if len(fields) != 4 or fields[1] not in ['b', 'w'] or not fields[3].isdigit() or int(fields[3]) < 1:
        raise ValueError('reference-domain: invalid SFEN fields')
    if int(fields[3]) > 2147483647:
        raise ValueError('reference-domain: next-ply exceeds cshogi signed-int32 domain; native u32 contract is separate')
    rows = fields[0].split('/')
    if len(rows) != 9:
        raise ValueError('SFEN requires nine ranks')
    pieces = []
    for rank, row in enumerate(rows):
        file = 9
        tokens = re.findall(r'\+[PLNSBRplnsbr]|[PLNSGBRKplnsgbrk]|[1-9]', row)
        if ''.join(tokens) != row:
            raise ValueError('invalid SFEN piece/promotion')
        for token in tokens:
            if token.isdigit():
                file -= int(token)
            else:
                if file < 1:
                    raise ValueError('SFEN rank overflow')
                pieces.append((str(file) + 'abcdefghi'[rank], token))
                file -= 1
        if file != 0:
            raise ValueError('SFEN rank does not have nine squares')
    if sum(p == 'K' for _, p in pieces) != 1 or sum(p == 'k' for _, p in pieces) != 1:
        raise ValueError('comparison diagrams require one king per owner')
    hands = {}
    if fields[2] != '-':
        tokens = re.findall(r'([1-9][0-9]*)?([RBGSNLPrbgsnlp])', fields[2])
        if ''.join(n + p for n, p in tokens) != fields[2]:
            raise ValueError('invalid SFEN hand')
        for n, p in tokens:
            if p in hands:
                raise ValueError('duplicate SFEN hand token')
            hands[p] = int(n or 1)
    king_squares = {piece: square for square, piece in pieces if piece in ['K', 'k']}
    bk, wk = king_squares['K'], king_squares['k']
    if abs(int(bk[0]) - int(wk[0])) <= 1 and abs(ord(bk[1]) - ord(wk[1])) <= 1:
        raise ValueError('reference-domain: adjacent kings')
    pawn_files = set()
    for square, piece in pieces:
        if piece in ['P', 'L'] and square[1] == 'a' or piece in ['p', 'l'] and square[1] == 'i':
            raise ValueError('reference-domain: dead unpromoted pawn/lance')
        if piece == 'N' and square[1] in 'ab' or piece == 'n' and square[1] in 'hi':
            raise ValueError('reference-domain: dead unpromoted knight')
        if piece in ['P', 'p']:
            key = (piece, square[0])
            if key in pawn_files:
                raise ValueError('reference-domain: nifu diagram')
            pawn_files.add(key)
    material = {}
    for _, piece in pieces:
        kind = piece.lstrip('+').upper()
        material[kind] = material.get(kind, 0) + 1
    for piece, count in hands.items():
        kind = piece.upper()
        material[kind] = material.get(kind, 0) + count
    if any(material.get(k, 0) > limit for k, limit in {'K': 2, 'R': 2, 'B': 2, 'G': 4, 'S': 4, 'N': 4, 'L': 4, 'P': 18}.items()):
        raise ValueError('SFEN exceeds standard physical piece inventory')
    return {'pieces': sorted(pieces), 'turn': fields[1], 'hands': hands, 'next_ply': int(fields[3])}


def admitted_root(cshogi, root, extra_plies=0):
    """Guard the source-audited geometric domain before any move enumeration/push.

    Public dlshogi feature API is used solely for its raw per-piece attack
    planes; it does not consult Legal/is_legal or cached incremental check state.
    At this pin, planes14..27 of owner0 hold all moving-side piece attacks,
    oriented to the mover; source cshogi.h:348-404 calls pos.attacksFrom.
    """
    try:
        state = normalized_sfen(root)
    except ValueError as error:
        if str(error).startswith('reference-domain:'):
            raise
        raise ValueError('reference-domain: ' + str(error)) from error
    if extra_plies < 0 or state['next_ply'] + extra_plies > 2147483647:
        raise ValueError('reference-domain: supplied prefix exceeds cshogi signed-int32 next-ply limit')
    import numpy as np
    from cshogi import dlshogi
    if dlshogi.FEATURES1_NUM != 62:
        raise ValueError('reference-domain: unexpected feature shape at admitted source pin')
    board = cshogi.Board(root)
    features1 = np.zeros((dlshogi.FEATURES1_NUM, 9, 9), dtype=np.float32)
    features2 = np.zeros((dlshogi.FEATURES2_NUM, 9, 9), dtype=np.float32)
    dlshogi.make_input_features(board, features1, features2)
    target = board.king_square(1 - board.turn)
    if board.turn == cshogi.WHITE:
        target = 80 - target
    attacks = features1.reshape(2, 31, 81)[0, 14:28, target]
    if bool(attacks.any()):
        raise ValueError('reference-domain: nonmoving king is already geometrically attacked by side to move; unsupported cshogi incremental-check root: ' + root)
    return board


def snapshot(board, cshogi):
    return {'sfen': board.sfen(), 'legal_moves': sorted(legal(board, cshogi)), 'in_check': bool(board.is_check()),
            'pieces': list(board.pieces), 'hands': [list(h) for h in board.pieces_in_hand],
            'turn': board.turn, 'move_number': board.move_number, 'history': list(board.history),
            'reference_hash': board.zobrist_hash()}


def replay(cshogi, root, moves):
    board = admitted_root(cshogi, root, len(moves))
    prefixes = [snapshot(board, cshogi)]
    for index, text in enumerate(moves):
        options = legal(board, cshogi)
        if not USI.fullmatch(text) or text not in options:
            raise ValueError(f'prefix action {index + 1} is not LegalAll: {text}')
        board.push(options[text])
        prefixes.append(snapshot(board, cshogi))
    return board, prefixes


def verify_reversal(board, prefixes, cshogi):
    for index in range(len(prefixes) - 2, -1, -1):
        want = prefixes[index]
        board.pop()
        got = snapshot(board, cshogi)
        if got != want:
            fields = [key for key in want if want[key] != got[key]]
            raise AssertionError(f'reference nested push/pop failed restoration at prefix{index}, fields={fields}, expected_sfen={want["sfen"]}, actual_sfen={got["sfen"]}')


def check_fixtures(cshogi, path):
    data = json.loads(path.read_text())
    if data.get('format') != 'mshogi.shogi.board-fixtures.v1' or data.get('rules_profile') != 'mshogi.shogi.csa36.v1':
        raise ValueError('unsupported fixture format/profile')
    ids = [case['id'] for case in data['cases']]
    if len(ids) != len(set(ids)):
        raise ValueError('duplicate fixture IDs')
    observations = []
    for case in data['cases']:
        if set(case['expected'].get('contains', [])) & set(case['expected'].get('absent', [])):
            raise ValueError('contradictory expected membership: ' + case['id'])
        if any(label not in data['sources'] for label in case['provenance']['source_labels']):
            raise ValueError('unknown fixture source label: ' + case['id'])
        board, prefixes = replay(cshogi, case['root_sfen'], case['moves'])
        choices = legal(board, cshogi)
        expected = case['expected']
        errors = []
        missing = sorted(set(expected.get('contains', [])) - choices.keys())
        extra = sorted(set(expected.get('absent', [])) & choices.keys())
        if 'sfen' in expected and board.sfen() != expected['sfen']:
            errors.append({'expected_sfen': expected['sfen'], 'actual_sfen': board.sfen()})
        if 'in_check' in expected and bool(board.is_check()) != expected['in_check']:
            errors.append({'in_check': {'expected': expected['in_check'], 'actual': bool(board.is_check())}})
        if missing or extra:
            errors.append({'missing_expected': missing, 'unexpected_forbidden': extra})
        if 'legal_moves' in expected and sorted(choices) != sorted(expected['legal_moves']):
            errors.append({'missing': sorted(set(expected['legal_moves']) - choices.keys()), 'extra': sorted(choices.keys() - set(expected['legal_moves']))})
        for move, want in expected.get('after_sfen', {}).items():
            if move not in choices:
                errors.append({'transition_not_legal': move})
                continue
            before = snapshot(board, cshogi)
            board.push(choices[move])
            got = board.sfen()
            if got != want:
                errors.append({'transition': move, 'expected_sfen': want, 'actual_sfen': got})
            board.pop()
            if snapshot(board, cshogi) != before:
                errors.append({'reversal': move})
        verify_reversal(board, prefixes, cshogi)
        observations.append({'id': case['id'], 'passed': not errors, 'errors': errors,
                             'root_sfen': case['root_sfen'], 'moves': case['moves'],
                             'reference': prefixes[-1]})
    return observations


def reference_domain_sentinels(cshogi):
    """Admission rejection evidence, never a legality/outcome oracle."""
    quiet = '8k/9/9/9/9/9/9/9/K8 b - 1'
    old = json.loads((ROOT / 'tests/fixtures/shogi/board-rules-v1.json').read_text())
    samples = [(case['id'], case['root_sfen'], 0) for case in old['cases']
               if case['id'].startswith('SH-F022:evasion-B-')]
    samples += [
        ('adjacent-kings', '9/9/9/9/9/9/9/8k/8K b - 1', 0),
        ('missing-king', '8k/9/9/9/9/9/9/9/9 b - 1', 0),
        ('dead-pawn', '4P3k/9/9/9/9/9/9/9/K8 b - 1', 0),
        ('dead-knight', '8k/4N4/9/9/9/9/9/9/K8 b - 1', 0),
        ('nifu-root', '8k/9/9/4P4/4P4/9/9/9/K8 b - 1', 0),
        ('excess-hand-material', '8k/9/9/9/9/9/9/9/K8 b 19P 1', 0),
        ('non-shogi-piece', '8k/9/9/4Q4/9/9/9/9/K8 b - 1', 0),
        ('counter-int32-overflow', quiet[:-1] + '2147483648', 0),
        ('prefix-counter-overflow', quiet[:-1] + '2147483647', 1),
        ('non-ascii', quiet.replace(' b ', '\u00a0b '), 0),
        ('oversized-input', quiet + ' ' * 513, 0),
    ]
    observed = []
    for name, root, extra_plies in samples:
        try:
            admitted_root(cshogi, root, extra_plies)
        except ValueError as error:
            if not str(error).startswith('reference-domain:'):
                raise AssertionError('unlabeled reference admission rejection: ' + str(error))
            observed.append({'id': name, 'root_sfen': root, 'extra_plies': extra_plies,
                             'rejected': True, 'diagnostic': str(error)})
        else:
            raise AssertionError('reference domain sentinel unexpectedly admitted: ' + name)
    return observed


def audit_gap_sentinels(cshogi):
    """Observe audited API gaps only; never use these helpers to decide rules."""
    out = []
    for name, root, text in [
            ('dead-pawn-drop', '8k/9/9/9/9/9/9/9/K8 b P 1', 'P*5a'),
            ('forced-pawn-nonpromotion', '8k/4P4/9/9/9/9/9/9/K8 b - 1', '5b5a')]:
        board = cshogi.Board(root)
        enumerated = text in legal(board, cshogi)
        parsed = board.move_from_usi(text)  # Deliberately audited permissive helper, not oracle.
        permissive = bool(board.is_legal(parsed))
        if enumerated or not parsed or not permissive:
            raise AssertionError('pinned permissive-API gap changed: ' + name)
        out.append({'id': name, 'root_sfen': root, 'candidate': text,
                    'independently_expected_legal': False,
                    'LegalAll_contains': enumerated, 'parser_returned_move': bool(parsed),
                    'is_legal_returned': permissive,
                    'interpretation': 'parser/is_legal acceptance is not a negative-legality oracle'})
    root = '8k/9/9/9/9/9/9/9/K8 b - 1'
    cycle = ['9i9h', '1a1b', '9h9i', '1b1a']
    for count in [1, 3]:
        board, prefixes = replay(cshogi, root, cycle * count)
        helper = board.is_draw()  # Deliberate gap observation; no native outcome inferred.
        if helper != cshogi.REPETITION_DRAW or not legal(board, cshogi):
            raise AssertionError('pinned repetition helper/board-query scope changed')
        verify_reversal(board, prefixes, cshogi)
        out.append({'id': 'repetition-helper-' + str(count) + '-cycles',
                    'root_sfen': root, 'moves': cycle * count,
                    'independently_counted_root_occurrences': count + 1,
                    'is_draw_returned': int(helper), 'REPETITION_DRAW': int(cshogi.REPETITION_DRAW),
                    'LegalAll_still_contains_board_choices': True,
                    'interpretation': 'first prior match is already reported; board choices do not adjudicate fourfold or match outcomes; MSH-67 owns those expectations'})
    return out


def query_probe(command, root, moves, timeout):
    request = {'root_sfen': root, 'moves': moves}
    try:
        result = subprocess.run(shlex.split(command), input=json.dumps(request) + '\n', capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return {'probe_failure': 'timeout', 'timeout_seconds': timeout}
    if result.returncode != 0:
        return {'probe_failure': 'nonzero exit', 'exit': result.returncode, 'stderr': result.stderr[-2000:]}
    if result.stderr:
        return {'probe_failure': 'unexpected stderr', 'stderr': result.stderr[-2000:]}
    try:
        lines = result.stdout.splitlines()
        if len(lines) != 1:
            raise ValueError('expected exactly one JSONL response')
        data = json.loads(lines[0])
        if not isinstance(data.get('sfen'), str) or type(data.get('in_check')) is not bool or not isinstance(data.get('legal_moves'), list):
            raise ValueError('missing or wrong response field types')
        choices = data['legal_moves']
        if any(not isinstance(m, str) or not USI.fullmatch(m) for m in choices) or len(set(choices)) != len(choices):
            raise ValueError('malformed or duplicate legal choices')
        normalized_sfen(data['sfen'])
        return data
    except (ValueError, TypeError, AttributeError) as error:
        return {'probe_failure': 'invalid response', 'detail': str(error), 'stdout': result.stdout[-2000:]}


def difference(want, got):
    if 'probe_failure' in got:
        return got
    mismatch = {}
    if want['sfen'] != got['sfen']:
        mismatch['sfen'] = {'expected': want['sfen'], 'actual': got['sfen'], 'semantic_difference': normalized_sfen(want['sfen']) != normalized_sfen(got['sfen'])}
    if want['in_check'] != got['in_check']:
        mismatch['in_check'] = {'expected': want['in_check'], 'actual': got['in_check']}
    missing = sorted(set(want['legal_moves']) - set(got['legal_moves']))
    extra = sorted(set(got['legal_moves']) - set(want['legal_moves']))
    if missing or extra:
        mismatch['legal_moves'] = {'missing': missing, 'extra': extra}
    if got.get('undo_restored') is False:
        mismatch['undo_restored'] = False
    return mismatch or None


def compare(cshogi, command, root, moves, seed, timeout):
    board, prefixes = replay(cshogi, root, moves)
    verify_reversal(board, prefixes, cshogi)
    for length, want in enumerate(prefixes):
        got = query_probe(command, root, moves[:length], timeout)
        delta = difference(want, got)
        if delta:
            return {'root_sfen': root, 'seed': seed, 'original_moves': moves,
                    'minimized_prefix': moves[:length], 'first_failing_ply': length,
                    'minimization': 'shortest failing prefix: every earlier prefix compared successfully; root and history retained',
                    'position_sfen': want['sfen'], 'difference': delta}
    return None


def seeded_path(cshogi, root, seed, plies):
    rng = random.Random(seed)
    board = admitted_root(cshogi, root, plies)
    moves = []
    for _ in range(plies):
        choices = legal(board, cshogi)
        if not choices:
            break
        text = rng.choice(sorted(choices))
        moves.append(text)
        board.push(choices[text])
    return moves


def demo_probe(cshogi, kind):
    request = json.loads(sys.stdin.readline())
    board, prefixes = replay(cshogi, request['root_sfen'], request['moves'])
    result = {k: prefixes[-1][k] for k in ['sfen', 'legal_moves', 'in_check']}
    if len(request['moves']) >= 2:
        if kind == 'missing':
            result['legal_moves'] = result['legal_moves'][1:]
        elif kind == 'extra':
            result['legal_moves'] = sorted(set(result['legal_moves'] + ['P*5e']))
        elif kind == 'crash':
            sys.stderr.write('deliberate diagnostic test double failure\n')
            return 17
        elif kind == 'truncated':
            sys.stdout.write('{"sfen":')
            return 0
    print(json.dumps(result))
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', type=Path, default=MANIFEST)
    parser.add_argument('--admit-source', type=Path)
    parser.add_argument('--build-log', type=Path)
    parser.add_argument('--fixtures', type=Path, default=FIXTURES)
    parser.add_argument('--probe')
    parser.add_argument('--seed', type=int, default=20261026)
    parser.add_argument('--playouts', type=int, default=1)
    parser.add_argument('--plies', type=int, default=24)
    parser.add_argument('--timeout', type=float, default=10)
    parser.add_argument('--report', type=Path)
    parser.add_argument('--diagnostics-demo', action='store_true')
    parser.add_argument('--demo-probe', choices=['missing', 'extra', 'crash', 'truncated'])
    args = parser.parse_args()
    if args.admit_source:
        if not args.build_log:
            parser.error('--admit-source requires --build-log')
        admit(args.admit_source, args.manifest, args.build_log)
    cshogi = reference(args.manifest)
    if args.demo_probe:
        return demo_probe(cshogi, args.demo_probe)
    observations = check_fixtures(cshogi, args.fixtures)
    report = {'format': 'mshogi.shogi.reference-run.v1', 'revision': PIN,
              'fixture_sha256': sha(args.fixtures), 'manifest_sha256': sha(args.manifest),
              'harness_sha256': sha(__file__),
              'probe_command': shlex.split(args.probe) if args.probe else None,
              'probe_sha256': sha(shlex.split(args.probe)[0]) if args.probe else None,
              'fixtures': observations, 'reference_domain_sentinels': reference_domain_sentinels(cshogi), 'audit_gap_sentinels': audit_gap_sentinels(cshogi), 'comparisons': [], 'diagnostics_demo': []}
    failed = [c for c in observations if not c['passed']]
    if failed:
        for case in failed:
            print(json.dumps({'fixture_failure': case['id'], 'errors': case['errors']}), file=sys.stderr)
    roots = list(dict.fromkeys(c['root_sfen'] for c in json.loads(args.fixtures.read_text())['cases']))
    if args.probe and not failed:
        for index, root in enumerate(roots):
            for run in range(args.playouts):
                seed = args.seed + index * 1009 + run
                moves = seeded_path(cshogi, root, seed, args.plies)
                mismatch = compare(cshogi, args.probe, root, moves, seed, args.timeout)
                record = {'root_sfen': root, 'seed': seed, 'moves': moves, 'prefixes_compared': mismatch['first_failing_ply'] + 1 if mismatch else len(moves) + 1, 'mismatch': mismatch}
                report['comparisons'].append(record)
                if mismatch:
                    print(json.dumps(mismatch), file=sys.stderr)
                    break
            if report['comparisons'][-1]['mismatch']:
                break
    if args.diagnostics_demo and not failed:
        root = roots[0]
        moves = seeded_path(cshogi, root, args.seed, 5)
        for kind in ['missing', 'extra', 'crash', 'truncated']:
            command = shlex.join([sys.executable, str(Path(__file__).resolve()), '--manifest', str(args.manifest), '--demo-probe', kind])
            mismatch = compare(cshogi, command, root, moves, args.seed, args.timeout)
            if mismatch is None or mismatch['first_failing_ply'] != 2:
                raise AssertionError(f'diagnostic demo did not detect/minimize {kind}: {mismatch}')
            report['diagnostics_demo'].append({'injection': kind, 'observed': mismatch})
    if args.report:
        args.report.write_text(json.dumps(report, indent=2) + '\n')
    comparison_failures = sum(c['mismatch'] is not None for c in report['comparisons'])
    print(json.dumps({'fixtures_passed': len(observations) - len(failed), 'fixtures_failed': len(failed),
                      'comparison_sequences': len(report['comparisons']), 'comparison_failures': comparison_failures,
                      'reference_domain_sentinels': len(report['reference_domain_sentinels']), 'audit_gap_sentinels': len(report['audit_gap_sentinels']), 'diagnostic_demonstrations': len(report['diagnostics_demo'])}))
    return int(bool(failed or comparison_failures))


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, AssertionError, OSError) as error:
        print('FAIL: ' + str(error), file=sys.stderr)
        sys.exit(1)
