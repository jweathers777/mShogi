"""MSH-204 (Tori Strength Harness): the fixed length-limit rule of native_match."""
import types
import unittest

from tools.strength import native_match


def row(score, *stops):
    return {'pair_score': score, 'stops': list(stops)}


class LengthLimitRule(unittest.TestCase):
    tori = types.SimpleNamespace(game='tori')
    shogi = types.SimpleNamespace(game='shogi')

    def test_only_tori_step_limit_pairs_are_excluded(self):
        limited = row(None, 'native_concluded', 'step_limit')
        self.assertTrue(native_match.length_limited(self.tori, limited))
        self.assertFalse(native_match.length_limited(self.shogi, limited))
        self.assertFalse(native_match.length_limited(self.tori, row(None, 'native_concluded', 'clock_limit')))
        self.assertFalse(native_match.length_limited(self.tori, row(1.0, 'native_concluded', 'native_concluded')))

    def test_other_unscored_stops_still_stop_the_campaign(self):
        self.assertIsNotNone(native_match.unfinished(self.tori, [row(None, 'step_limit', 'failure:Search: x')]))
        self.assertIsNotNone(native_match.unfinished(self.shogi, [row(None, 'native_concluded', 'step_limit')]))
        self.assertIsNone(native_match.unfinished(self.tori, [row(None, 'native_concluded', 'step_limit')]))

    def test_share_above_ten_percent_stops_after_twenty_pairs(self):
        scored = [row(0.5, 'native_concluded', 'native_concluded')]
        limited = [row(None, 'step_limit', 'native_concluded')]
        self.assertIsNone(native_match.unfinished(self.tori, scored * 18 + limited * 2))
        self.assertIsNotNone(native_match.unfinished(self.tori, scored * 17 + limited * 3))
        self.assertIsNone(native_match.unfinished(self.tori, scored * 5 + limited * 3))


if __name__ == '__main__':
    unittest.main()
