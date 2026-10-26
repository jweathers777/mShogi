#!/usr/bin/env python3
"""Developer-only bounded perft/divide comparison with the admitted LegalAll reference.

Independent expectations, where present, are checked separately. Other numbers
are observations of agreement, not newly invented rules/perft authorities.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import sys
import reference


def count(board, cshogi, depth):
    if depth == 0:
        return 1
    choices = reference.legal(board, cshogi)
    if depth == 1:
        return len(choices)
    before = reference.snapshot(board, cshogi)
    total = 0
    for move in choices.values():
        board.push(move)
        total += count(board, cshogi, depth - 1)
        board.pop()
        if reference.snapshot(board, cshogi) != before:
            raise AssertionError('reference perft did not restore exact state')
    return total


def run():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--probe', required=True)
    parser.add_argument('--fixtures', type=Path, action='append', required=True)
    parser.add_argument('--manifest', type=Path, default=reference.MANIFEST)
    parser.add_argument('--depth', type=int, choices=[1, 2], default=2)
    parser.add_argument('--timeout', type=float, default=20)
    parser.add_argument('--report', type=Path, required=True)
    args = parser.parse_args()
    cshogi = reference.reference(args.manifest)
    records = []
    for path in args.fixtures:
        for case in json.loads(path.read_text())['cases']:
            identity = (case['root_sfen'], tuple(case['moves']))
            # Diagnostic traversal has its own counter headroom, beyond replay.
            reference.admitted_root(cshogi, identity[0], len(identity[1]) + args.depth)
            board, prefixes = reference.replay(cshogi, *identity)
            before = reference.snapshot(board, cshogi)
            divide = {}
            for text, move in reference.legal(board, cshogi).items():
                board.push(move)
                divide[text] = count(board, cshogi, args.depth - 1)
                board.pop()
                if reference.snapshot(board, cshogi) != before:
                    raise AssertionError('reference divide did not restore exact state')
            reference.verify_reversal(board, prefixes, cshogi)
            total = sum(divide.values())
            request = {'root_sfen': identity[0], 'moves': list(identity[1]), 'perft_depth': args.depth}
            errors = []
            try:
                result = subprocess.run(shlex.split(args.probe), input=json.dumps(request) + '\n', capture_output=True, text=True, timeout=args.timeout)
                if result.returncode or result.stderr:
                    raise ValueError(f'probe exit={result.returncode}, stderr={result.stderr[-2000:]}')
                response = json.loads(result.stdout)
                native_divide = {e['action']: e['nodes'] for e in response['divide']}
                if len(native_divide) != len(response['divide']):
                    raise ValueError('duplicate divide entry')
                if response['perft'] != total:
                    errors.append({'count': {'reference': total, 'native': response['perft']}})
                differing = {text: {'reference': divide.get(text), 'native': native_divide.get(text)}
                             for text in sorted(divide.keys() | native_divide.keys())
                             if divide.get(text) != native_divide.get(text)}
                if differing:
                    errors.append({'divide': differing})
                if response['undo_restored'] is not True:
                    errors.append({'undo_restored': response.get('undo_restored')})
            except (ValueError, KeyError, TypeError, subprocess.TimeoutExpired) as error:
                errors.append({'probe_failure': str(error)})
            expected = case['expected']
            independent = expected.get('perft', {}).get(str(args.depth))
            if independent is not None and total != independent:
                errors.append({'independent_perft': independent, 'reference': total})
            expected_divide = expected.get('divide', {}).get(str(args.depth))
            if expected_divide is not None and divide != expected_divide:
                errors.append({'independent_divide_mismatch': True})
            records.append({'id': case['id'], 'root_sfen': identity[0], 'moves': list(identity[1]),
                            'depth': args.depth, 'reference_nodes': total, 'reference_divide': dict(sorted(divide.items())),
                            'independent_expected_nodes': independent, 'errors': errors})
            if errors:
                print(json.dumps(records[-1]), file=sys.stderr)
    command = shlex.split(args.probe)
    report = {'format': 'mshogi.shogi.perft-comparison.v1', 'reference_revision': reference.PIN,
              'manifest_sha256': reference.sha(args.manifest), 'tool_sha256': reference.sha(__file__), 'probe_command': command,
              'probe_sha256': hashlib.sha256(Path(command[0]).read_bytes()).hexdigest(),
              'fixtures_sha256': {str(p): reference.sha(p) for p in args.fixtures}, 'records': records,
              'interpretation': 'independent expectations checked where present; remaining counts are differential observations only'}
    args.report.write_text(json.dumps(report, indent=2) + '\n')
    failed = sum(bool(r['errors']) for r in records)
    print(json.dumps({'positions': len(records), 'failed': failed, 'depth': args.depth}))
    return int(failed != 0)


if __name__ == '__main__':
    try:
        sys.exit(run())
    except (ValueError, AssertionError, OSError) as error:
        print('FAIL: ' + str(error), file=sys.stderr)
        sys.exit(1)
