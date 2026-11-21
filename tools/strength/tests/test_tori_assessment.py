"""MSH-207 (Tori Strength Assessment): the frozen blunder rule and classes."""
import unittest

from tools.strength import tori_assessment as a
from tools.strength import tori_cloud

D = {'board': [], 'hands': {'s': {}, 'g': {}}}


def row(ply, score, **extra):
    side = 's' if ply % 2 == 0 else 'g'
    return dict({'ply': ply, 'side': side, 'diagram': dict(D, ply=ply), 'score': score,
                 'played': 'x', 'best': 'y', 'clock_ms': 3000}, **extra)


def game(*rows):
    return {'games': [{'game_id': 'f' * 64 + '/0', 'opening_id': 'o', 'stop': 'native_concluded',
                       'positions': list(rows)}], 'nodes': 5_000_000, 'evaluator_id': 'e', 'search_id': 's'}


class Rule(unittest.TestCase):
    def classes(self, *rows):
        r = a.analyze([game(*rows)])
        return [(e['kind'], e['class']) for e in r['errors']]

    def test_even_position_thrown_away_is_a_clear_blunder(self):
        # p_before 0.5; after the move the opponent is +1000: p_after about 0.16.
        self.assertEqual(self.classes(row(0, 0), row(1, 1000)), [('clear_blunder', 'other')])

    def test_small_loss_is_no_error_and_a_mistake_is_not_a_blunder(self):
        self.assertEqual(self.classes(row(0, 0), row(1, 100)), [])
        self.assertEqual(self.classes(row(0, 0), row(1, 400)), [('mistake', 'other')])

    def test_classes_by_priority(self):
        self.assertEqual(self.classes(row(0, 0, clock_ms=900), row(1, 1000)), [('clear_blunder', 'time_trouble')])
        self.assertEqual(self.classes(row(0, 0), row(1, a.MATE_SCORE - 7)),
                         [('clear_blunder', 'allowed_short_mate')])
        self.assertEqual(self.classes(row(0, 0), row(1, a.MATE_SCORE - 11)), [('clear_blunder', 'other')])
        self.assertEqual(self.classes(row(0, 0, best_captures='Sw'), row(1, 1000)),
                         [('clear_blunder', 'missed_capture')])
        self.assertEqual(self.classes(row(0, 0, best_captures='Sw', played_captures='Go'), row(1, 1000)),
                         [('clear_blunder', 'other')])
        self.assertEqual(self.classes(row(0, 0), row(1, 1000, best_drop=True)),
                         [('clear_blunder', 'drop_refutation')])
        self.assertEqual(self.classes(row(0, 0), row(1, 1000, best_captures='Ea')),
                         [('clear_blunder', 'refuted_major_capture')])
        self.assertEqual(self.classes(row(0, 0), row(1, 1000, best_captures='Pt')),
                         [('clear_blunder', 'refuted_minor_capture')])

    def test_repeated_position_and_terminal_and_unjudged(self):
        first = row(0, 0)
        again = dict(row(2, 0), diagram=first['diagram'])
        self.assertEqual(self.classes(first, row(1, 0), again, row(3, 1000)), [('clear_blunder', 'repetition')])
        # The move into a lost (concluded) position loses everything.
        lost = {'ply': 1, 'side': 'g', 'diagram': D, 'terminal': {'lost': False}}
        self.assertEqual(self.classes(row(0, 0), lost), [('clear_blunder', 'other')])
        r = a.analyze([game(row(0, None), row(1, 0))])
        self.assertEqual((r['moves_judged'], r['unjudged_moves']), (0, 1))

    def test_fix_rule_ignores_other_and_needs_thirty_percent(self):
        rows = [row(0, 0), row(1, 1000, best_drop=True)]
        r = a.analyze([game(*rows)])
        self.assertEqual(r['fix_rule'], {'class': 'drop_refutation', 'share': 1.0, 'triggers': True})
        r = a.analyze([game(row(0, 0), row(1, 1000))])
        self.assertEqual(r['fix_rule'], {'class': None, 'share': 0, 'triggers': False})

    def test_suite_rows_keep_the_split_by_game(self):
        r = a.analyze([game(row(0, 0), row(1, 1000))])
        s = a.suite(r, 5_000_000)
        self.assertEqual(len(s['positions']), 1)
        self.assertIn(s['positions'][0]['split'], ('development', 'held-out'))


class Bins(unittest.TestCase):
    def manifest(self, bins):
        from tools.strength.tests.test_tori_cloud import manifest
        m = manifest()
        m['experiment'] = 'msh207-test'
        m['bins'] = bins
        return m

    def test_assessment_campaign_is_valid(self):
        bins = tori_cloud.assessment_bins()
        tori_cloud.validate_manifest(self.manifest(bins))
        tasks = [t for b in bins for t in b]
        control = sorted(s for t in tasks if t['label'] == 'control' for s in t['seeds'])
        self.assertEqual(control, list(range(1, 51)))
        for base in tori_cloud.SCALING_BASES_MS:
            odds = [t for t in tasks if t['label'] == 'odds' and t['base_ms'] == base]
            self.assertEqual(sum(len(t['seeds']) for t in odds), 96)
            self.assertTrue(all(t['factor'] == 2 and t['increment_ms'] == base // 50 for t in odds))

    def test_judge_campaign_is_valid(self):
        bins = tori_cloud.judge_bins()
        tori_cloud.validate_manifest(self.manifest(bins))
        self.assertEqual(sorted(t['shard'] for b in bins for t in b), list(range(32)))


if __name__ == '__main__':
    unittest.main()
