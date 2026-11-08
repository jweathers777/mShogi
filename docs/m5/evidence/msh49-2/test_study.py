"""Offline measurement-control tests, not native chess or strength evidence."""
import copy
import hashlib
import gzip
import importlib.util
import json
import math
import tempfile
from pathlib import Path
import unittest
from unittest.mock import Mock, patch

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('study', HERE/'study.py')
study = importlib.util.module_from_spec(spec)
spec.loader.exec_module(study)


class Controls(unittest.TestCase):
    def setUp(self):
        self.protocol = study.read(HERE/'protocol.json')
        self.build = study.read(HERE/'build.json')

    def test_time_controls_have_no_hidden_node_depth_or_skill_handicap(self):
        opening = {'id': 'control', 'fen': None, 'moves': ['e2e4']}
        c = study.configuration(self.protocol, self.build, opening)
        self.assertEqual(c['work'], {'depth': None, 'nodes': None, 'movetime_ms': 500})
        self.assertNotIn('engine_work', c)
        self.assertTrue(c['paired'])
        self.assertEqual(c['max_plies'], 512)
        options = {o['name']: o['value'] for o in c['engines'][1]['options']}
        for name, value in [('Threads', '1'), ('Skill Level', '20'), ('UCI_LimitStrength', 'false'), ('Ponder', 'false'), ('nodestime', '0')]:
            self.assertEqual(options[name], value)
        self.assertEqual(options['EvalFile'], c['engines'][1]['model_path'])
        self.assertEqual(c['openings'], [opening])
        self.assertEqual(study.configuration(self.protocol, self.build, opening, True)['max_plies'], 4)

    def test_formal_execution_refuses_missing_or_different_protocol_approval(self):
        for approval in [None, {'owner': 'John', 'approved_protocol_sha256': 'different'}, {'owner': 'Codex', 'approved_protocol_sha256': study.digest(HERE/'protocol.json')}]:
            ledger = {'approval': approval, 'budget': {'runs': [], 'charged_seconds': 0, 'inflight': None}}
            with patch.object(study, 'read', side_effect=[self.protocol, self.build, ledger]), patch.object(study, 'verify_build'), patch.object(study.subprocess, 'Popen') as launch:
                with self.assertRaisesRegex(ValueError, 'Explicit owner approval'):
                    study.main('formal')
                launch.assert_not_called()

    def test_paired_uncertainty_never_treats_all_losses_as_zero_uncertainty(self):
        result = study.interval([0] * 16)
        self.assertEqual(result['lower'], 0)
        self.assertAlmostEqual(result['upper'], 0.339525, places=5)
        result = study.interval([0.5] * 16)
        self.assertAlmostEqual(result['mean'], 0.5)
        self.assertLess(result['lower'], 0.2)
        self.assertGreater(result['upper'], 0.8)
        for bad in [[], [-1], [1.1], [float('nan')]]:
            with self.assertRaises(ValueError):
                study.interval(bad)

    def test_tampered_executable_is_refused_before_launch(self):
        wrong = copy.deepcopy(self.build)
        wrong['artifacts']['mshogi']['sha256'] = hashlib.sha256(b'not the engine').hexdigest()
        with self.assertRaisesRegex(ValueError, 'hash mismatch'):
            study.verify_build(wrong)

    def test_accounting_cannot_reset_failed_runs_or_unsettled_work(self):
        for budget in [
            {'runs': [{'seconds': 10}], 'charged_seconds': 0, 'inflight': None},
            {'runs': [], 'charged_seconds': 0, 'inflight': {'phase': 'formal'}},
            {'runs': [{'seconds': -1}], 'charged_seconds': -1, 'inflight': None},
        ]:
            with self.assertRaises(ValueError):
                study.audit_budget({'budget': budget})
        study.audit_budget({'budget': {'runs': [{'seconds': 5.5}], 'charged_seconds': 5.5, 'inflight': None}})

    def test_actual_smoke_caps_are_unscored_and_altered_controls_rejected(self):
        # Mutations of the actual transport receipt exercise the audit, not
        # a fabricated engine-strength or rules result.
        outer = json.loads(gzip.decompress((HERE/'smoke/report.json.gz').read_bytes()))
        expected = study.read(HERE/'smoke/config.json')
        result = study.audit_report(outer, expected, self.build)
        self.assertTrue(result['healthy'])
        self.assertFalse(result['complete'])
        self.assertIsNone(result['pair_score'])
        inner = json.loads(outer['report_json'])
        for mutation in ['time', 'color', 'network', 'config']:
            bad = copy.deepcopy(inner)
            if mutation == 'time':
                for game in bad['games']:
                    for entry in game['transcript']['entries']:
                        if entry['direction'] == 'command' and entry['text'].startswith('go'):
                            entry['text'] = 'go depth 2'
            elif mutation == 'color':
                bad['games'][1]['engines_by_color'] = [0, 1]
            elif mutation == 'network':
                bad['engine_identities'][1]['model']['sha256'] = 'wrong'
            else:
                bad['config']['work']['nodes'] = 10000
            envelope = copy.deepcopy(outer)
            envelope['report_json'] = json.dumps(bad)
            envelope['report_sha256'] = hashlib.sha256(envelope['report_json'].encode()).hexdigest()
            with self.assertRaises(ValueError):
                study.audit_report(envelope, expected, self.build)
        bad = copy.deepcopy(inner)
        bad['games'][0]['engines'][0]['cleanup_failures'] = ['not reaped']
        envelope = copy.deepcopy(outer)
        envelope['report_json'] = json.dumps(bad)
        envelope['report_sha256'] = hashlib.sha256(envelope['report_json'].encode()).hexdigest()
        self.assertFalse(study.audit_report(envelope, expected, self.build)['healthy'])

    def test_nested_timeout_shares_cleanup_deadline_and_preserves_unsettled_receipt(self):
        # Controlled scheduler/pipe ordering; no real engine is launched.
        child = Mock(pid=123)
        child.communicate.side_effect = [study.subprocess.TimeoutExpired('runner', 1), study.subprocess.TimeoutExpired('runner', 4), study.subprocess.TimeoutExpired('runner', 0)]
        child.poll.return_value = None
        ledger = {'budget': {'runs': [], 'inflight': None, 'charged_seconds': 0}}
        config = study.configuration(self.protocol, self.build, {'id': 'cancel', 'fen': None, 'moves': []})
        with tempfile.TemporaryDirectory() as directory, patch.object(study, 'save'), patch.object(study.subprocess, 'Popen', return_value=child), patch.object(study.os, 'killpg') as kill, patch.object(study.time, 'monotonic', side_effect=[0, 10, 11, 16, 17]):
            with self.assertRaises(study.subprocess.TimeoutExpired):
                study.invoke(self.protocol, self.build, ledger, config, 'constructed_control_test', Path(directory)/'trial', 1)
        self.assertEqual([c.kwargs['timeout'] for c in child.communicate.call_args_list], [1, 4, 0])
        self.assertEqual(kill.call_count, 2)
        self.assertEqual(ledger['budget']['charged_seconds'], 17)
        self.assertIsNotNone(ledger['budget']['inflight'])
        self.assertFalse(ledger['budget']['runs'][0]['healthy'])
        self.assertFalse(ledger['budget']['runs'][0]['complete'])


if __name__ == '__main__':
    unittest.main()
