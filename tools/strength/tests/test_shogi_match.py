"""The match wrapper must preserve native scoring and incomplete pairs."""
import unittest
import contextlib
import io
from unittest import mock

from tools.strength.shogi_match import summarize
from tools.strength import native_match


class ShogiPairScoring(unittest.TestCase):
    def test_shogi_entry_point_uses_shared_native_scorer(self):
        self.assertIs(summarize, native_match.summarize)

    def test_tenjiku_requires_an_explicit_immutable_profile(self):
        common = ['native_match', '--candidate', 'unused', '--baseline', 'unused',
                  '--candidate-nodes', '1000', '--out', 'unused']
        for extra in [['--game', 'tenjiku'], ['--game', 'chu', '--profile', 'adams']]:
            with self.subTest(extra=extra), mock.patch('sys.argv', common + extra):
                with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as raised:
                    native_match.main()
                self.assertEqual(raised.exception.code, 2)

    def report(self, first, second):
        return {'games': [
            {'pairing': {'swapped': False}, 'score_first_twice': first,
             'steps': [], 'stop': 'native'},
            {'pairing': {'swapped': True}, 'score_first_twice': second,
             'steps': [], 'stop': 'native'},
        ]}

    def test_colors_are_reversed_before_pair_statistics(self):
        for first, second, scores, pair in [
            (2, 0, [2, 2], 1), (0, 2, [0, 0], 0),
            (2, 2, [2, 0], .5), (1, 0, [1, 2], .75),
        ]:
            result = summarize(self.report(first, second))
            self.assertEqual(result['scores_twice'], scores)
            self.assertEqual(result['pair_score'], pair)

    def test_cutoff_is_not_a_draw_or_a_scored_pair(self):
        result = summarize(self.report(2, None))
        self.assertEqual(result['scores_twice'], [2, None])
        self.assertIsNone(result['pair_score'])

    def test_incomplete_schedule_is_refused(self):
        report = self.report(2, 0)
        report['games'].pop()
        with self.assertRaises(ValueError):
            summarize(report)

    def test_pending_xiangqi_adjudication_stays_unscored(self):
        report = self.report(None, None)
        report['games'][0]['stop'] = 'native_pending_adjudication'
        result = native_match.summarize(report)
        self.assertIsNone(result['pair_score'])
        self.assertEqual(result['stops'][0], 'native_pending_adjudication')

    def test_chu_capacity_stop_does_not_become_a_draw(self):
        report = self.report(2, None)
        report['games'][1]['stop'] = 'native_stop:ResourceLimit'
        result = native_match.summarize(report)
        self.assertIsNone(result['pair_score'])
        self.assertEqual(result['scores_twice'], [2, None])
        self.assertEqual(result['stops'][1], 'native_stop:ResourceLimit')


if __name__ == '__main__':
    unittest.main()
