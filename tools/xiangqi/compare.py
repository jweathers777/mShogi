#!/usr/bin/env python3
"""Compare immutable independent board facts and optional native JSONL probes.

Failures retain expected facts separately from reference/native observations;
this program never edits fixtures or chooses the winner of a disagreement.
"""
import argparse
import json
from pathlib import Path
import random
import shlex
import sys

import reference


def fact_errors(case, actual):
    errors = []
    moves = set(actual['legal_moves'])
    if 'in_check' in case and case['in_check'] != actual['in_check']:
        errors.append({'in_check': {'expected': case['in_check'], 'observed': actual['in_check']}})
    if 'legal_moves' in case and moves != set(case['legal_moves']):
        errors.append({'complete_actions': {'missing': sorted(set(case['legal_moves']) - moves),
                                            'unexpected': sorted(moves - set(case['legal_moves']))}})
    for origin, expected in case.get('moves_from', {}).items():
        observed = {move for move in moves if move[:2] == origin}
        if observed != set(expected):
            errors.append({'origin': origin, 'missing': sorted(set(expected) - observed),
                           'unexpected': sorted(observed - set(expected))})
    missing = set(case.get('must_include', [])) - moves
    unexpected = set(case.get('must_exclude', [])) & moves
    if missing or unexpected:
        errors.append({'membership': {'missing': sorted(missing), 'unexpected': sorted(unexpected)}})
    if 'perft' in actual:
        depth = case.get('_depth', 0)
        expected = case.get('perft', {}).get(str(depth))
        if expected is not None and expected != actual['perft']:
            errors.append({'independent_perft': {'depth': depth, 'expected': expected, 'observed': actual['perft']}})
        expected_divide = case.get('divide', {}).get(str(depth))
        if expected_divide is not None:
            observed_divide = {entry['action']: entry['nodes'] for entry in actual['divide']}
            if observed_divide != expected_divide:
                errors.append({'independent_divide': {'depth': depth, 'expected': expected_divide, 'observed': observed_divide}})
    return errors


def native_query(command, request, timeout):
    raw = reference.bounded_run(command, (json.dumps(request) + '\n').encode(), timeout)
    try:
        result = reference.strict_json(raw)
    except (ValueError, UnicodeError) as error:
        raise reference.ProbeFailure('malformed_response', str(error)) from error
    return reference.validate_response(result, request.get('perft_depth', 0))


def local_fen(fen):
    # Pikafish emits its rule60/check10-adjusted clock. Native position codecs
    # preserve a declared/raw no-capture counter; neither proves WXF history.
    # Compare the board, native side and move-number convention, retaining both
    # complete FENs as observations but excluding the foreign policy clock.
    fields = fen.split(' ')
    return fields[:4] + fields[5:]


def run_case(case, manifest, native, timeout, depth=0, moves=None):
    record = {'id': case['id'], 'expected_facts': {key: case[key] for key in
              ('in_check', 'legal_moves', 'moves_from', 'must_include', 'must_exclude', 'perft', 'divide') if key in case},
              'errors': []}
    if case.get('reference_eligible') is False:
        record['excluded_domain'] = case.get('domain', 'native structural diagnostic outside admitted reference domain')
        return record
    # Accepted request provenance is immutable: the caller extends its working
    # sequence after each observation, so never retain that mutable list.
    request = {'root_fen': case['fen'], 'moves': list(moves or []), 'perft_depth': depth}
    record['request'] = request
    try:
        actual = reference.query(request, manifest, timeout)
        record['reference_observation'] = actual
        expected = dict(case, _depth=depth)
        record['errors'].extend(fact_errors(expected, actual))
        if native:
            observed = native_query(native, request, timeout)
            record['native_observation'] = observed
            record['errors'].extend({'native_fixture': error} for error in fact_errors(expected, observed))
            if local_fen(actual['fen']) != local_fen(observed['fen']):
                record['errors'].append({'differential': 'local_board_side_fullmove',
                                         'reference': actual['fen'], 'native': observed['fen']})
            for key in ('legal_moves', 'in_check', 'perft', 'divide'):
                if actual.get(key) != observed.get(key):
                    record['errors'].append({'differential': key,
                                             'reference': actual.get(key), 'native': observed.get(key)})
    except (ValueError, OSError, KeyError, TypeError) as error:
        record['errors'].append({'failure': str(error), 'kind': getattr(error, 'kind', 'admission_or_schema')})
    return record


def compare(fixtures, manifest, native, report, timeout, depth, walks, length):
    before = reference.sha(fixtures)
    tool_before = reference.sha(__file__)
    adapter_before = reference.sha(reference.__file__)
    inventory = reference.strict_json(fixtures.read_text())
    if inventory.get('format') != 'mshogi.xiangqi.board-fixtures.v1':
        raise ValueError('unsupported fixture format')
    cases = inventory['fixtures']
    records = [run_case(case, manifest, native, timeout, depth) for case in cases]
    # Broad sequence observations are deliberately not new expected fixtures.
    sequence_records = []
    root = next(case['fen'] for case in cases if case.get('legal_moves') and len(case['legal_moves']) == 44)
    for seed in range(walks):
        rng, moves = random.Random(seed), []
        for ply in range(length + 1):
            case = {'id': f'observed-sequence-{seed}-{ply}', 'fen': root}
            item = run_case(case, manifest, native, timeout, 0, moves)
            sequence_records.append(item)
            if item['errors']:
                break
            if ply == length:
                break
            choices = item['reference_observation']['legal_moves']
            if not choices:
                break
            moves.append(rng.choice(choices))
    if before != reference.sha(fixtures):
        raise AssertionError('fixture changed during comparison')
    if tool_before != reference.sha(__file__) or adapter_before != reference.sha(reference.__file__):
        raise AssertionError('comparison tools changed during execution; rerun stable sources')
    result = {'format': 'mshogi.xiangqi.board-comparison.v1', 'reference_revision': reference.PIN,
              'manifest_sha256': reference.sha(manifest), 'fixtures_sha256': before,
              'tool_sha256': tool_before, 'reference_adapter_sha256': adapter_before,
              'native_command': native, 'native_binary_sha256': reference.sha(native[0]) if native else None,
              'depth': depth, 'sequence_seeds': list(range(walks)), 'sequence_length': length,
              'fen_counter_exclusion': 'Pikafish rule60/check10 clock is not native raw no-capture count or WXF adjudication; full native/reference FEN observations retained; compare placement/side/fullmove only',
              'fixture_records': records, 'sequence_observations': sequence_records,
              'interpretation': 'expected facts are immutable independent inputs; reference/native outputs and unproven perft totals are observations only'}
    report.write_text(json.dumps(result, indent=2) + '\n')
    failures = sum(bool(item['errors']) for item in records + sequence_records)
    print(json.dumps({'fixtures': len(records), 'excluded': sum('excluded_domain' in item for item in records),
                      'sequence_positions': len(sequence_records), 'failed': failures, 'report': str(report)}))
    return int(bool(failures))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--fixtures', type=Path, required=True)
    parser.add_argument('--manifest', type=Path, default=reference.MANIFEST)
    parser.add_argument('--native-probe', help='quoted argv for the native JSONL probe, optional')
    parser.add_argument('--timeout', type=float, default=20)
    parser.add_argument('--depth', type=int, choices=[0, 1, 2, 3], default=0)
    parser.add_argument('--walks', type=int, default=0)
    parser.add_argument('--length', type=int, default=32)
    parser.add_argument('--report', type=Path, required=True)
    args = parser.parse_args()
    try:
        if not 0 <= args.walks <= 32 or not 0 <= args.length <= 256:
            raise ValueError('bounded walks0..32 and length0..256 required')
        sys.exit(compare(args.fixtures, args.manifest, shlex.split(args.native_probe) if args.native_probe else None,
                         args.report, args.timeout, args.depth, args.walks, args.length))
    except (ValueError, OSError, KeyError, TypeError, AssertionError) as error:
        print('COMPARISON FAILED: ' + str(error), file=sys.stderr)
        sys.exit(1)
