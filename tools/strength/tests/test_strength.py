"""MSH-163 harness tests: statistics properties, data files and report parsing."""
import json
import math
import random
import unittest
from pathlib import Path

from tools.strength import stats

DATA = Path(__file__).resolve().parents[1] / 'data'


class EloTests(unittest.TestCase):
    def test_known_logistic_values(self):
        # 10 ** (100 / 400) = 1.778279...; 1 / (1 + 1 / 1.778279) = 0.640065
        self.assertAlmostEqual(stats.expected_score(100), 0.6400649998, places=9)
        self.assertEqual(stats.expected_score(0), 0.5)
        self.assertAlmostEqual(stats.elo_from_score(0.75), 400 * math.log10(3), places=9)
        self.assertEqual(stats.elo_from_score(0), -math.inf)
        self.assertEqual(stats.elo_from_score(1), math.inf)

    def test_inverse(self):
        for elo in (-300, -15, 0, 7.5, 250):
            self.assertAlmostEqual(stats.elo_from_score(stats.expected_score(elo)), elo, places=9)

    def test_wald_bounds(self):
        lower, upper = stats.bounds(0.05, 0.05)
        self.assertAlmostEqual(upper, math.log(19), places=12)
        self.assertAlmostEqual(lower, -math.log(19), places=12)


class PentanomialTests(unittest.TestCase):
    def test_counts(self):
        self.assertEqual(stats.pentanomial([0, .25, .5, .5, 1]), [1, 1, 2, 0, 1])

    def test_rejects_impossible_pair_score(self):
        for bad in (0.3, -0.25, 1.25):
            with self.assertRaises(ValueError):
                stats.pentanomial([bad])

    def test_llr_hand_computed(self):
        # Pairs [0, 1, 2, 1, 0]: mean .5, sum of squared deviations 2 * .0625;
        # with the two prior pairs at .25/.75: (0.125 + 0.125) / 6.
        counts = [0, 1, 2, 1, 0]
        s0, s1 = 0.5, stats.expected_score(15)
        expected = 4 * (s1 - s0) * (1.0 - s0 - s1) / (2 * (0.25 / 6))
        self.assertAlmostEqual(stats.llr(counts, 0, 15), expected, places=12)
        self.assertLess(expected, 0)

    def test_llr_empty_is_zero(self):
        self.assertEqual(stats.llr([0, 0, 0, 0, 0], 0, 15), 0.0)

    def test_all_draw_pairs_have_defined_negative_llr(self):
        self.assertLess(stats.llr([0, 0, 9, 0, 0], 0, 15), 0.0)

    def test_no_decision_on_small_or_draw_dominated_samples(self):
        sprt = stats.Sprt(0, 15, 0.05, 0.05, min_pairs=30)
        self.assertIsNone(sprt.status([0, 0, 0, 1, 3])['decision'])   # 4 pairs
        self.assertIsNone(sprt.status([0, 1, 19, 0, 0])['decision'])  # 20 pairs
        self.assertIsNone(sprt.status([0, 1, 29, 0, 0])['decision'])  # one non-draw pair in 30

    def test_estimate_is_strict_json(self):
        e = stats.estimate([0, 0, 0, 0, 5])
        json.dumps(e, allow_nan=False)
        self.assertIsNone(e['elo'])

    def test_estimate_symmetric_at_even_score(self):
        e = stats.estimate([1, 3, 10, 3, 1])
        self.assertAlmostEqual(e['score'], 0.5)
        self.assertAlmostEqual(e['elo'], 0.0)
        self.assertAlmostEqual(e['elo_low'], -e['elo_high'], places=9)

    def test_sprt_parameter_validation(self):
        with self.assertRaises(ValueError):
            stats.Sprt(10, 5)


def simulate(true_elo, sprt, rng, max_pairs=4000):
    """Pair scores from two independent games with 60% draws at the true strength."""
    s = stats.expected_score(true_elo)
    draw = 0.6
    win = s - draw / 2
    counts = [0] * 5
    for _ in range(max_pairs):
        total = 0
        for _game in range(2):
            u = rng.random()
            total += 2 if u < win else 1 if u < win + draw else 0
        counts[total] += 1
        decision = sprt.status(counts)['decision']
        if decision:
            return decision
    return None


class SequentialPropertyTests(unittest.TestCase):
    """Operating characteristics of the stopping rule, from simulated games."""

    def test_error_rates(self):
        rng = random.Random(163)
        sprt = stats.Sprt(0, 15, 0.05, 0.05)
        null = [simulate(0, sprt, rng) for _ in range(150)]
        alt = [simulate(25, sprt, rng) for _ in range(150)]
        self.assertLessEqual(null.count('H1') / len(null), 0.10)
        self.assertGreaterEqual(alt.count('H1') / len(alt), 0.90)


class DataTests(unittest.TestCase):
    def test_blunder_suite(self):
        suite = json.loads((DATA / 'chess-blunders-msh161.json').read_text())
        self.assertEqual(suite['count'], 146)
        self.assertEqual(len(suite['positions']), 146)
        ids = set()
        for p in suite['positions']:
            self.assertNotIn(p['id'], ids)
            ids.add(p['id'])
            self.assertNotEqual(p['blunder_move'], p['reference_move'])
            self.assertEqual(p['confirmation_20m']['status'], 'supported_finite')
            self.assertTrue(.30 <= p['screen_500k']['root_E'] <= .70)
            self.assertLess(p['screen_500k']['played_E'], .15)
            self.assertEqual(len(p['history']) % 2, 0 if p['mover'] == 'white' else 1)
            self.assertIn(p['phase'], ('opening', 'middlegame', 'endgame'))

    def test_openings(self):
        data = json.loads((DATA / 'openings-msh161.json').read_text())
        self.assertEqual(data['count'], 90)
        for o in data['openings']:
            fields = o['root_fen'].split()
            self.assertEqual(len(o['moves']), 2 * (int(fields[5]) - 1) + (fields[1] == 'b'))

    def test_bench_positions(self):
        data = json.loads((DATA / 'bench-chess-v1.json').read_text())
        self.assertEqual(data['name'], 'bench-chess-v1')
        self.assertEqual(len({p['id'] for p in data['positions']}), len(data['positions']))
        for p in data['positions']:
            self.assertTrue(1 <= p['depth'] <= 40)
