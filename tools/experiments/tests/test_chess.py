"""Adapter contract tests: fake process results, actual frozen extraction/screening."""
import copy
import importlib.util
import json
from pathlib import Path
import tempfile
import shutil
import unittest
from unittest.mock import patch

REPO = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location('study_chess_adapter', REPO / 'tools/experiments/chess.py')
adapter = importlib.util.module_from_spec(spec)
spec.loader.exec_module(adapter)


def report():
    fen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
    initial = {'rules_profile': 'mshogi.chess.match.v1', 'initial_fen': fen,
               'history_origin': 'standard_start', 'events': []}
    record = {'rules_profile': initial['rules_profile'], 'initial_record_json': json.dumps(initial),
              'events': [{'request': {'actor': side, 'command': {'move': move}}, 'reply': {}}
                         for side, move in [('white', 'e2e4'), ('black', 'e7e5')]]}
    return {'schema_version': 1, 'config': {'id': 'pair-a', 'openings': [{'fen': None, 'moves': []}]},
            'games': [{'match_record_json': json.dumps(record), 'opening_index': 0, 'opening_id': 'root-a',
                       'pairing': i, 'engines_by_color': [i, 1-i], 'stop': 'chess_conclusion',
                       'failure': None, 'diagnostic_failures': []} for i in range(2)]}


def query(binary, decision, nodes, forced, inflight):
    best = forced or decision['played']
    selected = {'depth': 8, 'pv': [best], 'score': {'kind': 'cp', 'value': 20}, 'wdl': [100, 850, 50]}
    return {'status': 'complete', 'process_group_settled': True, 'decision': decision,
            'assessment': {'bestmove': best, 'score': selected, 'uncertainty': ['finite_search']},
            'score_updates': [dict(selected, depth=7), selected], 'pv_independent_legal': True,
            'options': {'Threads': 1, 'Hash': 64, 'UCI_ShowWDL': 'true',
                        'UCI_LimitStrength': 'false', 'Skill Level': 20, 'Ponder': 'false'},
            'wall_seconds': 1, 'child_cpu_seconds': .9, 'forced_move': forced,
            'raw_uci': [{'direction': 'send', 'line': f'go nodes {nodes}'}]}


class AdapterTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.binary = self.root / 'engine'
        self.binary.write_bytes(b'fake fixture executable')
        identity = {'path': str(self.binary), 'sha256': adapter._sha(self.binary)}
        self.build = {'source': 'frozen-native-revision',
                      'engines': {key: copy.deepcopy(identity) for key in ('candidate', 'runner', 'stockfish19', 'opponent')}}
        self.report = self.root / 'report.json'
        self.report.write_text(json.dumps(report()))
        self.modules = adapter._modules(REPO)

    def tearDown(self):
        self.temp.cleanup()

    def execute(self, job, side_effect=query):
        with patch.object(adapter, '_modules', return_value=self.modules), \
             patch.object(adapter, '_replay', return_value=self.report), \
             patch.object(self.modules['analysis'], 'query', side_effect=side_effect) as call:
            result = adapter.execute(dict(build=self.build, **job), self.root / 'out', {'report': self.report}, REPO)
        return result, call

    def test_all_moves_both_players_and_uncertainty_retained(self):
        result, call = self.execute({'kind': 'reference_screening'})
        self.assertEqual((result['decisions'], result['queries']), (4, 8))
        self.assertEqual(call.call_count, 8)
        identities = {(c.args[1]['game'], c.args[1]['engine_index']) for c in call.call_args_list}
        self.assertEqual(identities, {(0, 0), (0, 1), (1, 0), (1, 1)})
        self.assertTrue(all(c.args[2] == 500000 for c in call.call_args_list))
        screen = json.loads((self.root / 'out/screen-v1.json').read_text())
        self.assertTrue(screen['complete_query_coverage'])
        self.assertTrue(all('finite_search' in row['flags'] for row in screen['decisions']))
        self.assertEqual(len(json.loads((self.root / 'out/confirmation-plan.json').read_text())), 4)

    def test_exact_confirmation_preserves_forced_move_and_nodes(self):
        result, call = self.execute({'kind': 'confirmation', 'game': 1, 'ply': 1, 'move': 'c7c5', 'nodes': 5000000})
        self.assertEqual(result['queries'], 1)
        self.assertEqual(call.call_args.args[2:4], (5000000, 'c7c5'))
        value = json.loads(next((self.root / 'out/analysis').glob('g*-attempt*.json')).read_text())
        self.assertEqual(value['decision']['history'], ['e2e4'])
        self.assertEqual(value['score_updates'][-1]['pv'], ['c7c5'])

    def test_query_failure_is_not_success_and_receipt_survives(self):
        def failed(*args):
            return dict(query(*args), status='failed', error='interrupted')
        with self.assertRaisesRegex(RuntimeError, 'query failed'):
            self.execute({'kind': 'reference_screening'}, failed)
        value = json.loads(next((self.root / 'out/analysis').glob('g*-attempt*.json')).read_text())
        self.assertEqual(value['status'], 'failed')
        self.assertFalse((self.root / 'out/screen-v1.json').exists())

    def test_invalid_selector_and_changed_screen_limit_rejected(self):
        for job in [{'kind': 'confirmation', 'game': 2, 'ply': 0},
                    {'kind': 'reference_screening', 'nodes': 1}]:
            with self.subTest(job=job), self.assertRaises(ValueError):
                self.execute(job)

    def test_engine_replacement_rejected(self):
        self.binary.write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError, 'identity mismatch'):
            self.execute({'kind': 'reference_screening'})

    def test_replay_failure_is_fatal_and_recorded(self):
        out = self.root / 'replay'
        out.mkdir()
        with patch.object(adapter.subprocess, 'run') as call:
            call.return_value.returncode = 1
            with self.assertRaisesRegex(RuntimeError, 'replay rejected'):
                adapter._replay(self.report, str(self.binary), out)
        self.assertEqual(json.loads((out / 'replay.json').read_text())['returncode'], 1)
        self.assertEqual(call.call_args.args[0][1], 'experiment-external-replay')

    def test_pair_failure_is_not_draw_or_success(self):
        with patch.object(adapter, '_modules', return_value=self.modules), \
             patch.object(self.modules['run_batch'], 'run_pair', return_value={'complete': False}):
            with self.assertRaisesRegex(RuntimeError, 'pair incomplete'):
                adapter.execute({'kind': 'game_pair', 'build': self.build,
                                 'spec': {'opponent': 'opponent'}}, self.root / 'pair-out', {}, REPO)

    def test_frozen_source_tamper_rejected_before_execution(self):
        repo = self.root / 'changed-repo'
        study = repo / 'docs/m5/evidence/msh161'
        study.mkdir(parents=True)
        source = REPO / 'docs/m5/evidence/msh161'
        for name in ('main-freeze.json', 'run_batch.py', 'analysis.py', 'screen_v1.py', 'phase_tags.py'):
            shutil.copyfile(source / name, study / name)
        with (study / 'run_batch.py').open('a') as stream:
            stream.write('# changed source')
        with self.assertRaisesRegex(ValueError, 'Frozen study helper changed'):
            adapter._modules(repo)

    def test_incomplete_native_report_never_starts_analysis(self):
        value = report()
        value['games'][1]['stop'] = 'ply_limit'
        self.report.write_text(json.dumps(value))
        with patch.object(adapter, '_modules', return_value=self.modules), \
             patch.object(adapter, '_replay', return_value=self.report), \
             patch.object(self.modules['analysis'], 'query') as query_call:
            with self.assertRaisesRegex(ValueError, 'complete healthy native pair'):
                adapter.execute({'kind': 'reference_screening', 'build': self.build},
                                self.root / 'out', {'report': self.report}, REPO)
        query_call.assert_not_called()

    def test_wrong_report_claim_policy_never_starts_analysis(self):
        value = report(); value['config']['claim_policy'] = 'explicit_only'
        self.report.write_text(json.dumps(value))
        with patch.object(adapter, '_modules', return_value=self.modules), \
             patch.object(adapter, '_replay', return_value=self.report), \
             patch.object(self.modules['analysis'], 'query') as query_call:
            with self.assertRaisesRegex(ValueError, 'claim policy differs'):
                adapter.execute({'kind': 'reference_screening', 'build': self.build},
                                self.root / 'out', {'report': self.report}, REPO)
        query_call.assert_not_called()

    def test_frozen_config_keeps_equal_time_and_pairing(self):
        cfg = self.modules['run_batch'].config(self.build, {'id': 'pair', 'opponent': 'opponent', 'opening': {'id': 'opening', 'fen': None, 'moves': []}})
        self.assertEqual(cfg['work']['movetime_ms'], 5000)
        self.assertTrue(cfg['paired'])
        self.assertEqual(cfg['max_plies'], 512)
        self.assertEqual(cfg['claim_policy'], 'auto_claim')

class ClaimPolicyAmendmentTests(unittest.TestCase):
    def test_fresh_module_override_changes_only_policy_and_does_not_leak(self):
        modules = adapter._modules(REPO)
        native = modules['run_batch']
        build = {'source': 'a'*40, 'engines': {k: {'path': '/'+k, 'source': 'b'*40}
                 for k in ['candidate', 'oxidation060']}}
        pair = {'id': 'policy-fixture', 'opponent': 'oxidation060',
                'opening': {'id': 'root', 'fen': None, 'moves': []}}
        original = native.config(build, pair)
        adapter._configure_native(native, 'explicit_only')
        amended = native.config(build, pair)
        expected = copy.deepcopy(original); expected['claim_policy'] = 'explicit_only'
        self.assertEqual(amended, expected)
        fresh = adapter._modules(REPO)['run_batch']
        self.assertEqual(fresh.config(build, pair), original)
        self.assertEqual(original['claim_policy'], 'auto_claim')

    def test_amendment_binds_policy_source_and_bytes(self):
        with tempfile.TemporaryDirectory() as t:
            root = Path(t); path = root/'amendment.json'; source = 'a'*40
            value = {'schema': 'msh161-automatic-thresholds-v1', 'claim_policy': 'explicit_only',
                     'uci_optional_claims_available': False, 'native_source': source,
                     'original_study_freeze_sha256': 'f'*64}
            path.write_text(json.dumps(value)); sha = adapter._sha(path)
            job = {'claim_policy': 'explicit_only', 'study_amendment': {'path': 'amendment.json', 'sha256': sha},
                   'source_files': {'amendment.json': sha}, 'study_freeze_sha256': 'f'*64,
                   'build': {'source': source, 'engines': {k: {'source': source} for k in ['candidate','runner']}}}
            self.assertEqual(adapter._claim_policy(job, root), 'explicit_only')
            bad = copy.deepcopy(job); del bad['study_amendment']
            with self.assertRaises(ValueError): adapter._claim_policy(bad, root)
            bad = copy.deepcopy(job); bad['claim_policy'] = 'auto_claim'
            with self.assertRaises(ValueError): adapter._claim_policy(bad, root)
            bad = copy.deepcopy(job); bad['build']['engines']['candidate']['source'] = 'b'*40
            with self.assertRaises(ValueError): adapter._claim_policy(bad, root)
            path.write_text(json.dumps(dict(value, native_source='b'*40)))
            with self.assertRaises(ValueError): adapter._claim_policy(job, root)
            self.assertEqual(adapter._claim_policy({}, root), 'auto_claim')

    def test_package_only_cannot_launch(self):
        with self.assertRaisesRegex(ValueError, 'Package-only'):
            adapter.execute({'package_only': True}, None, {}, REPO)

if __name__ == '__main__':
    unittest.main()
