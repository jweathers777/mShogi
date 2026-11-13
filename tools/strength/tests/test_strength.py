"""MSH-163 harness tests: statistics properties, data files and report parsing."""
import json
import math
import random
import unittest
from pathlib import Path
from types import SimpleNamespace

from tools.strength import stats
from tools.strength import cli
from tools.strength.uci import parse_info, position_command

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
        # Pairs [0, 1, 2, 1, 0]: mean .5, variance (2*.0625)/4 = .03125.
        counts = [0, 1, 2, 1, 0]
        s0, s1 = 0.5, stats.expected_score(15)
        expected = 4 * (s1 - s0) * (1.0 - s0 - s1) / (2 * 0.03125)
        self.assertAlmostEqual(stats.llr(counts, 0, 15), expected, places=12)
        self.assertLess(expected, 0)

    def test_llr_undefined_variance_is_zero(self):
        self.assertEqual(stats.llr([0, 0, 9, 0, 0], 0, 15), 0.0)
        self.assertEqual(stats.llr([0, 0, 0, 0, 0], 0, 15), 0.0)

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


class ParsingTests(unittest.TestCase):
    def test_parse_info(self):
        info = parse_info('info depth 9 nodes 220937 time 858 seldepth 21 score cp -21 pv b1c3 g8f6')
        self.assertEqual(info['depth'], 9)
        self.assertEqual(info['nodes'], 220937)
        self.assertEqual(info['score'], {'kind': 'cp', 'value': -21})
        self.assertEqual(info['pv'], ['b1c3', 'g8f6'])

    def test_position_command(self):
        self.assertEqual(position_command(None, []), 'position startpos')
        self.assertEqual(position_command('8/8/8/8/8/8/8/K6k w - - 0 1', ['a1a2']),
                         'position fen 8/8/8/8/8/8/8/K6k w - - 0 1 moves a1a2')

    def _game(self, white, conclusion):
        initial = {'initial_fen': 'startpos-fen', 'events': [{'ply': 0, 'event': {'kind': 'move', 'action': 'e2e4'}}]}
        record = {'initial_record_json': json.dumps(initial), 'events': [
            {'request': {'command': 'start'}, 'reply': {'remaining_ns': None}},
            {'request': {'command': {'move': 'e7e5'}}, 'reply': {'remaining_ns': None}}]}
        return {'pairing': 0 if white == 0 else 1, 'engines_by_color': [white, 1 - white], 'stop': 'chess_conclusion',
                'conclusion': conclusion, 'failure': None, 'match_record_json': json.dumps(record),
                'transcript': {'entries': [
                    {'direction': 'response', 'engine': 0, 'text': 'info depth 7 nodes 100 time 5'},
                    {'direction': 'response', 'engine': 1, 'text': 'info depth 5 nodes 50 time 5'}]}}

    def test_summarize_pair(self):
        report = {'summaries': [{'opening_id': 'o', 'games': 2, 'wins': [1, 0], 'draws': 1, 'unscored': 0,
                                 'failures': 0}],
                  'games': [self._game(0, 'w'), self._game(1, 'd')],
                  'engine_identities': [{'executable': {'sha256': 'a'}}, {'executable': {'sha256': 'b'}}]}
        s = cli.summarize_pair(report, None)
        self.assertEqual(s['score'], 0.75)
        self.assertEqual(s['games'][0]['record']['prefix'], ['e2e4'])
        self.assertEqual(s['games'][0]['record']['played'], ['e7e5'])
        self.assertEqual(s['games'][0]['median_depth'], [7, 5])

    def test_unscored_pair_has_no_score(self):
        report = {'summaries': [{'opening_id': 'o', 'games': 2, 'wins': [1, 0], 'draws': 0, 'unscored': 1,
                                 'failures': 0}],
                  'games': [self._game(0, 'w'), self._game(1, 'u')],
                  'engine_identities': [{'executable': {'sha256': 'a'}}, {'executable': {'sha256': 'b'}}]}
        self.assertIsNone(cli.summarize_pair(report, None)['score'])

    def test_match_config_controls(self):
        base = dict(candidate='/a', baseline='/b', candidate_option=[], baseline_option=[('Hash', '64')])
        cfg = cli.match_config(SimpleNamespace(**base, movetime=200, base=None, increment=0), {'id': 'x', 'moves': []}, 'p')
        self.assertEqual(cfg['time_control'], 'untimed')
        self.assertEqual(cfg['work']['movetime_ms'], 200)
        self.assertTrue(cfg['paired'])
        self.assertEqual(cfg['engines'][1]['options'][-1], {'name': 'Hash', 'value': '64'})
        cfg = cli.match_config(SimpleNamespace(**base, movetime=200, base=10, increment=0.1), {'id': 'x', 'moves': []}, 'p')
        self.assertEqual(cfg['time_control'], {'increment': {'initial_ns': [10 ** 10, 10 ** 10], 'increment_ns': 10 ** 8}})
        self.assertIsNone(cfg['work']['movetime_ms'])


if __name__ == '__main__':
    unittest.main()
