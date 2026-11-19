import unittest

from tools.strength.xiangqi_blunders import board_after, classify


class BlunderClassTest(unittest.TestCase):
    def test_board_tracking_from_the_standard_root(self):
        b = board_after(['h2e2', 'h9g7'])
        self.assertEqual(b['e2'], 'C')
        self.assertEqual(b['g7'], 'n')
        self.assertNotIn('h2', b)
        self.assertEqual(b['a0'], 'R')
        self.assertEqual(b['e9'], 'k')

    def test_priority_of_classes(self):
        game = {'moves': ['h2e2', 'h9g7', 'h0g2'], 'conclusion': None}
        turn = {'index': 2, 'clock_ms': 50000}
        self.assertEqual(classify(game, turn, {'score': ('cp', 300), 'bestmove': 'i9h9'}, 'red'), 'other')
        self.assertEqual(classify(game, turn, {'score': ('cp', 300), 'bestmove': 'g7g2'}, 'red'),
                         'refuted_major_capture')
        self.assertEqual(classify(game, turn, {'score': ('cp', 300), 'bestmove': 'e6e3'}, 'red'),
                         'refuted_minor_capture')
        self.assertEqual(classify(game, turn, {'score': ('mate', 3), 'bestmove': 'g7g2'}, 'red'),
                         'allowed_short_mate')
        self.assertEqual(classify(game, turn, {'score': ('mate', 5), 'bestmove': 'i9h9'}, 'red'), 'other')
        lost = dict(game, conclusion={'kind': 'win', 'winner': 'black',
                                      'reason': {'kind': 'repetition', 'fault': 'perpetual_chase'}})
        self.assertEqual(classify(lost, turn, {'score': ('mate', 3), 'bestmove': 'g7g2'}, 'red'),
                         'repetition_rule')
        self.assertEqual(classify(game, dict(turn, clock_ms=1500), {'score': ('cp', 0), 'bestmove': 'i9h9'}, 'red'),
                         'time_trouble')


if __name__ == '__main__':
    unittest.main()
