import math
import unittest

from tools.strength.xiangqi_ladder import bradley_terry


class BradleyTerryTest(unittest.TestCase):
    def test_two_players_match_the_logistic_score(self):
        # A scores 75% against B: 400 * log10(0.75 / 0.25) Elo above B.
        games = [{'a': 'a', 'b': 'b', 'score': 1.0}] * 3 + [{'a': 'a', 'b': 'b', 'score': 0.0}]
        r = bradley_terry(games, ['a', 'b'], 'b')
        self.assertAlmostEqual(r['a'], 400 * math.log10(3), places=3)
        self.assertEqual(r['b'], 0)

    def test_transitive_chain_is_additive_and_draws_count_half(self):
        # a-b 75% and b-c 75% with no a-c games: a = 2 * 190.8 above c.
        games = ([{'a': 'a', 'b': 'b', 'score': 1.0}] * 3 + [{'a': 'a', 'b': 'b', 'score': 0.0}]
                 + [{'a': 'b', 'b': 'c', 'score': 1.0}] * 2 + [{'a': 'b', 'b': 'c', 'score': 0.5}] * 2)
        r = bradley_terry(games, ['a', 'b', 'c'], 'c')
        self.assertAlmostEqual(r['b'], 400 * math.log10(3), places=2)
        self.assertAlmostEqual(r['a'], 800 * math.log10(3), places=2)


if __name__ == '__main__':
    unittest.main()
