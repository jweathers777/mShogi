import math
import unittest

from tools.strength.xiangqi_judge import analyze_games, native_reply, win_probability


class JudgeRulesTest(unittest.TestCase):
    def test_win_probability_and_mates(self):
        self.assertAlmostEqual(win_probability(('cp', 0)), 0.5)
        self.assertAlmostEqual(win_probability(('cp', 600)), 1 / (1 + math.exp(-1)))
        self.assertEqual(win_probability(('mate', 3)), 1.0)
        self.assertEqual(win_probability(('mate', -2)), 0.0)

    def test_clear_blunder_and_loss_from_the_movers_view(self):
        # Before the move the mover is even (p 0.5). After it the opponent
        # scores +600 (p 0.731), so the mover has 0.269: a drop of 0.231 is a
        # loss but not a clear blunder (needs 0.30). After a second move the
        # opponent is mated in 1 for the mover's side: no loss.
        games = [{'moves': ['a3a4', 'a6a5', 'b0c2'], 'turns': [
            {'player': 'x', 'index': 0, 'move': 'a3a4'},
            {'player': 'y', 'index': 1, 'move': 'a6a5'}]}]
        judged = {'': {'score': ('cp', 0)}, 'a3a4': {'score': ('cp', 600)},
                  'a3a4 a6a5': {'score': ('cp', -3000)}}
        r = analyze_games(games, judged)
        self.assertEqual(r['x']['moves'], 1)
        self.assertEqual(r['x']['clear_blunders'], 0)
        self.assertAlmostEqual(r['x']['mean_win_probability_loss'], round(0.5 - (1 - 1 / (1 + math.exp(-1))), 4))
        # y: before 0.731 (its view of 'a3a4'), after 1 - p(-3000) ~ 0.993: no loss.
        self.assertEqual(r['y']['clear_blunders'], 0)
        self.assertEqual(r['y']['mean_win_probability_loss'], 0)
        # A large drop is a clear blunder.
        judged['a3a4'] = {'score': ('cp', 2000)}
        self.assertEqual(analyze_games(games, judged)['x']['clear_blunders'], 1)

    def test_fairy_ranks_are_translated_and_resign_ends(self):
        self.assertEqual(native_reply('h3e3', {'rank_offset': 1}), 'h2e2')
        self.assertEqual(native_reply('h2e2', {}), 'h2e2')
        self.assertIsNone(native_reply('resign', {}))


if __name__ == '__main__':
    unittest.main()
