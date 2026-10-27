#!/usr/bin/env python3
"""Executable admission/failure sentinels for the developer reference boundary."""
import argparse
import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

import compare
import reference

INITIAL = 'rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1'
MANIFEST = reference.MANIFEST


class AdmissionSentinels(unittest.TestCase):
    def test_initial_independent_count_and_known_memberships(self):
        result = reference.query({'root_fen': INITIAL, 'perft_depth': 2}, MANIFEST)
        # Count/grouping is the independently reviewed MSH80 opening fact.
        self.assertEqual(len(result['legal_moves']), 44)
        self.assertFalse(result['in_check'])
        self.assertTrue(result['undo_restored'])
        self.assertIn('b2b9', result['legal_moves'])
        self.assertIn('h2h9', result['legal_moves'])
        self.assertNotIn('b2b8', result['legal_moves'])
        self.assertEqual(result['perft'], sum(entry['nodes'] for entry in result['divide']))

    def test_reference_sequence_restores_every_prefix(self):
        result = reference.query({'root_fen': INITIAL, 'moves': ['b0a2', 'b9a7', 'a0b0', 'a9b9'], 'perft_depth': 2}, MANIFEST)
        self.assertTrue(result['undo_restored'])

    def test_recorded_prefix_does_not_alias_later_sequence_work(self):
        sequence = ['b0a2']
        result = compare.run_case({'id': 'prefix-provenance', 'fen': INITIAL}, MANIFEST, None, 20, moves=sequence)
        self.assertEqual(result['errors'], [])
        sequence.append('b9a7')
        self.assertEqual(result['request']['moves'], ['b0a2'])
        self.assertEqual(result['reference_observation']['fen'].split(' ')[1], 'b')

    def test_foreign_check10_clock_is_excluded_from_board_comparison(self):
        root = '4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1'
        cycle = ['e9f9', 'f8e8', 'f9e9', 'e8f8']
        result = reference.query({'root_fen': root, 'moves': cycle * 6}, MANIFEST)
        # 24 accepted noncapture plies, 12 Red checks: independently count raw
        # transitions. The pinned foreign engine's emitted counter is adjusted.
        raw = result['fen'].split(' ')
        self.assertEqual(int(raw[4]), 20)  # source-pin behavior, NOT WXF truth
        raw[4] = '24'
        self.assertEqual(compare.local_fen(result['fen']), compare.local_fen(' '.join(raw)))
        self.assertTrue(result['undo_restored'])

    def test_expected_facts_preserved_on_deliberate_mismatch(self):
        result = reference.query({'root_fen': INITIAL}, MANIFEST)
        expected = {'id': 'deliberate-failure', 'fen': INITIAL,
                    'moves_from': {'e0': []}, 'must_include': ['b2b8']}
        original = copy.deepcopy(expected)
        errors = compare.fact_errors(expected, result)
        self.assertEqual(expected, original)
        self.assertIn({'origin': 'e0', 'missing': [], 'unexpected': ['e0e1']}, errors)
        self.assertIn({'membership': {'missing': ['b2b8'], 'unexpected': []}}, errors)

    def test_strict_duplicate_unknown_and_nonfinite_parsing(self):
        for text in ['{"root_fen":"a","root_fen":"b"}', '{"root_fen":NaN}']:
            with self.assertRaises(ValueError):
                reference.strict_json(text)
        for request in [{'root_fen': INITIAL, 'unknown': 1}, {'moves': []},
                        {'root_fen': INITIAL, 'perft_depth': True},
                        {'root_fen': INITIAL, 'moves': ['e0e0']},
                        {'root_fen': INITIAL, 'moves': ['a0a1'] * 257}]:
            with self.assertRaises(ValueError):
                reference.validate_request(request)

    def test_invalid_and_unsupported_reference_domains(self):
        for fen in [INITIAL.replace(' w ', ' red '), INITIAL.replace(' - - ', ' KQ - '),
                    INITIAL.replace('0 1', '120 1'), INITIAL.replace('0 1', '0 0'),
                    INITIAL.replace('RNBAKABNR', 'RNBA1ABNR'),
                    INITIAL.replace('RNBAKABNR', 'RNBAKAANR'),
                    INITIAL.replace('9/9/P1P1P1P1P', '9/9/1P7'),
                    INITIAL.replace('9/9/P1P1P1P1P', '9/18/P1P1P1P1P')]:
            with self.subTest(fen=fen), self.assertRaises(ValueError):
                reference.fen_domain(fen)

    def test_upstream_semantic_domain_rejects_opponent_already_in_check(self):
        with self.assertRaises(reference.ProbeFailure) as caught:
            reference.query({'root_fen': '4Rk3/9/9/9/9/9/9/9/9/3K5 w - - 0 1'}, MANIFEST)
        self.assertEqual(caught.exception.kind, 'process_failure')
        self.assertIn('Unsupported', caught.exception.detail)

    def test_nonlegal_complete_action_rejected(self):
        with self.assertRaises(reference.ProbeFailure) as caught:
            reference.query({'root_fen': INITIAL, 'moves': ['b2b8']}, MANIFEST)
        self.assertIn('absent from complete legal list', caught.exception.detail)

    def test_timeout(self):
        with self.assertRaises(reference.ProbeFailure) as caught:
            reference.bounded_run([sys.executable, '-c', 'import time; time.sleep(5)'], b'', 0.05)
        self.assertEqual(caught.exception.kind, 'timeout')

    def test_exited_leader_does_not_leave_pipe_holding_descendant(self):
        with tempfile.TemporaryDirectory(prefix='msh91-descendant-') as directory:
            marker = str(Path(directory) / 'survived')
            child = f'import time,pathlib; time.sleep(.4); pathlib.Path({marker!r}).touch()'
            leader = f'import subprocess,sys; subprocess.Popen([sys.executable,"-c",{child!r}])'
            with self.assertRaises(reference.ProbeFailure) as caught:
                reference.bounded_run([sys.executable, '-c', leader], b'', .1)
            self.assertEqual(caught.exception.kind, 'timeout')
            time.sleep(.45)
            self.assertFalse(Path(marker).exists(), 'descendant survived failed request retirement')

    def test_process_failure_is_explicit(self):
        with self.assertRaises(reference.ProbeFailure) as caught:
            reference.bounded_run([sys.executable, '-c', 'import sys; print("intentional", file=sys.stderr); sys.exit(7)'], b'')
        self.assertEqual(caught.exception.kind, 'process_failure')
        self.assertIn('exit=7', caught.exception.detail)
        self.assertIn('intentional', caught.exception.detail)

    def test_response_and_diagnostic_limits(self):
        for stream in ('stdout', 'stderr'):
            script = f'import sys; sys.{stream}.write("x"*100000)'
            with self.subTest(stream=stream), self.assertRaises(reference.ProbeFailure) as caught:
                reference.bounded_run([sys.executable, '-c', script], b'')
            self.assertEqual(caught.exception.kind, 'output_limit')

    def test_malformed_native_responses_remain_failures(self):
        for raw in ['{}', '{"fen":1,"fen":2}', 'no JSON']:
            with self.subTest(raw=raw), self.assertRaises(reference.ProbeFailure):
                compare.native_query([sys.executable, '-c', f'print({raw!r})'], {'root_fen': INITIAL}, 2)

    def test_wrong_binary_identity_rejected(self):
        manifest = reference.strict_json(Path(MANIFEST).read_text())
        with tempfile.TemporaryDirectory(prefix='msh91-identity-') as directory:
            path = Path(directory) / 'wrong.json'
            manifest['binary_sha256'] = '0' * 64
            path.write_text(json.dumps(manifest))
            with self.assertRaises(ValueError):
                reference.admitted_binary(path)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', type=Path, default=MANIFEST)
    args = parser.parse_args()
    MANIFEST = args.manifest
    unittest.main(argv=[sys.argv[0]], verbosity=2)
