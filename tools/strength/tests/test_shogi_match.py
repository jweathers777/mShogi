"""The match wrapper must preserve native scoring and incomplete pairs."""
import unittest

from tools.strength.shogi_match import summarize


class ShogiPairScoring(unittest.TestCase):
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


if __name__ == '__main__':
    unittest.main()
