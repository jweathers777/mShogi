"""Evidence boundaries in the common strength workflow."""
import json
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

from tools.strength import workflow
from tools.strength.common import profile
from tools.strength.native_match import calibration_failure


class StrengthWorkflowTests(unittest.TestCase):
    def test_every_profile_is_distinct_and_tenjiku_requires_selection(self):
        self.assertEqual(len({profile(game, None) for game in ('chess', 'shogi', 'xiangqi', 'chu', 'tori')}
                             | {profile('tenjiku', name) for name in ('adams', 'muller')}), 7)
        with self.assertRaises(ValueError):
            profile('tenjiku', None)

    def test_calibration_requires_both_applied_turns(self):
        row = {'seed': 1, 'plies': [1, 1], 'stops': ['step_limit', 'native_concluded'],
               'turns': [[{'applied': True}], [{'applied': True}]]}
        self.assertIsNone(calibration_failure([row]))
        row['turns'][1][0]['applied'] = False
        self.assertIn('one accepted move', calibration_failure([row]))

    def test_separate_heldout_and_development_answers_and_source_groups(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            args = SimpleNamespace(source=root/'input', out=root/'suite', game='chess', profile=None, proof='supplied')
            rows = [{'id': str(i), 'group': grp, 'split': part, 'source': 'native reference proof'}
                    for i, (grp, part) in enumerate([('g1', 'held-out'), ('g1', 'held-out'), ('g2', 'development')])]
            def native_position(args, row, **ignored):
                return {'position': row['id']+' b - - 0 1', 'answers': ['e2e4'],
                        'score_first_twice': None, 'root': {'record': row['id']}}
            with patch.object(workflow, 'position', native_position):
                args.source.write_text(json.dumps({'positions': rows}))
                workflow.suite(args)
                result = json.loads(args.out.read_text())
                self.assertEqual([r['split'] for r in result['positions']], ['held-out', 'held-out', 'development'])
                rows[1]['split'] = 'development'
                args.source.write_text(json.dumps({'positions': rows}))
                with self.assertRaisesRegex(ValueError, 'conflicting'):
                    workflow.suite(args)

    def test_judge_uses_movers_terminal_score_and_writes_reusable_blunder(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            args = SimpleNamespace(game='chess', profile=None, samples=1, out=root/'quality')
            step = {'game_id': 'abc/0', 'opening_id': 'test-group', 'ply': 4, 'mover_first': False,
                    'terminal_score': 2, 'before': 'before', 'after': 'after', 'player': 0,
                    'observation': {'kind': 'chess', 'result': {'kind': 'move', 'action': 'e2e4',
                                    'telemetry': {'completed_depth': 1, 'nodes': 100}}}}
            before = {'kind': 'move', 'action': 'd2d4', 'telemetry': {'score': 0, 'nodes': 500, 'completed_depth': 3}}
            with patch.object(workflow, 'evidence', return_value={}), patch.object(workflow, 'projected', return_value=[step]), \
                    patch.object(workflow, 'search', return_value=before) as search:
                workflow.judge(args)
            self.assertEqual(search.call_count, 1)
            evidence = json.loads(args.out.read_text())
            self.assertEqual((evidence['clear_blunders_per_100'], evidence['mean_win_probability_loss']), (100, .5))
            self.assertEqual(evidence['deeper_than_player'], 1)
            error, = evidence['positions']
            self.assertEqual((error['record'], error['played'], error['judge_move']), ('before', 'e2e4', 'd2d4'))
            self.assertEqual(error['split'], workflow.partition('test-group'))

    def test_missing_search_scores_stay_unjudged(self):
        self.assertIsNone(workflow.probability({'telemetry': {'score': None}}))
        self.assertEqual(workflow.probability({'telemetry': {'score': -999999}}), 0)
        self.assertEqual(workflow.probability({'telemetry': {'score': 999999}}), 1)
