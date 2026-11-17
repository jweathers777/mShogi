"""Independent algebraic and input-integrity checks for the anchor estimator."""
import math
import unittest

from tools.strength import anchor_stats as stats


def rows(pair_scores, anchor=2500, opponent='a', family='a'):
    return [{'game_id': f'{opponent}-{pair}-{side}', 'pair_id': f'{opponent}-{pair}',
             'opponent_id': opponent, 'family': family, 'anchor': anchor, 'score': score}
            for pair, scores in enumerate(pair_scores) for side, score in enumerate(scores)]


class AnchorStatsTests(unittest.TestCase):
    def test_logistic_known_odds(self):
        self.assertEqual(stats.expected_score(2500, 2500), .5)
        self.assertAlmostEqual(stats.expected_score(2900, 2500), 10 / 11)
        self.assertAlmostEqual(stats.expected_score(2100, 2500), 1 / 11)

    def test_symmetric_case_and_independent_sandwich_calculation(self):
        result = stats.fit(rows([(1, 1)] * 20 + [(0, 0)] * 20))
        self.assertAlmostEqual(result['estimate'], 2500)
        a = math.log(10) / 400
        # Forty independent cluster scores are +a or -a; 80 game scores
        # contribute 80*.25*a^2 to the derivative at the symmetric root.
        expected_se = math.sqrt((40 / 39) * 40 * a*a) / (20*a*a + 1/500**2)
        self.assertAlmostEqual(result['sandwich_se'], expected_se)
        self.assertEqual(result['t_critical'], 2.042273)  # df 39 rounds down to 30.
        self.assertFalse(result['routing_floor_used'])
        self.assertTrue(result['precision_stop_eligible'])

    def test_known_unregularized_three_to_one_odds(self):
        result = stats.fit(rows([(1, 1)] * 30 + [(0, 0)] * 10), prior_sd=1e6)
        self.assertAlmostEqual(result['estimate'], 2500 + 400 * math.log10(3), places=4)

    def test_symmetric_unequal_anchors(self):
        games = rows([(1, 0)] * 10, anchor=2300, opponent='a')
        games += rows([(1, 0)] * 10, anchor=2700, opponent='b')
        self.assertAlmostEqual(stats.fit(games)['estimate'], 2500)

    def test_prior_only_and_extreme_results_are_finite(self):
        prior = stats.fit([])
        self.assertAlmostEqual(prior['estimate'], 2500)
        self.assertAlmostEqual(prior['interval95'][0], 2500 - 1.95996398454 * 500, places=5)
        self.assertFalse(prior['precision_stop_eligible'])
        for scores in ([(1, 1)] * 4, [(0, 0)] * 4):
            result = stats.fit(rows(scores))
            self.assertTrue(all(math.isfinite(x) for x in result['interval95']))
            self.assertLess(result['interval95'][0], result['estimate'])
            self.assertGreater(result['interval95'][1], result['estimate'])
            self.assertFalse(result['precision_stop_eligible'])

    def test_no_false_precision_for_identical_draw_pairs(self):
        result = stats.fit(rows([(.5, .5)] * 40))
        self.assertGreater(result['half_width'], 0)
        self.assertFalse(result['precision_stop_eligible'])
        self.assertTrue(result['routing_floor_used'])

    def test_singleton_retained_and_changes_fit(self):
        games = rows([(1, 0)] * 30)
        singleton = rows([(1,)], opponent='b')
        result = stats.fit(games + singleton)
        self.assertEqual(result['scored_games'], 61)
        self.assertEqual(result['singleton_clusters'], 1)
        self.assertEqual(result['complete_pairs'], 30)
        self.assertGreater(result['estimate'], stats.fit(games)['estimate'])

    def test_constant_scores_at_unequal_anchors_cannot_trigger_precision(self):
        for score in (0, .5, 1):
            games = rows([(score, score)] * 15, anchor=2400)
            games += rows([(score, score)] * 15, anchor=2600, opponent='b')
            result = stats.fit(games)
            self.assertFalse(result['precision_stop_eligible'])
            self.assertTrue(result['routing_floor_used'])
            self.assertLessEqual(result['interval95'][0], result['routing_interval95'][0])
            self.assertGreaterEqual(result['interval95'][1], result['routing_interval95'][1])

    def test_order_color_and_rating_translation_invariance(self):
        games = rows([(1, 1), (.5, 0), (0, 1)] * 12)
        original = stats.fit(games)
        self.assertEqual(original, stats.fit(list(reversed(games))))
        translated = [dict(g, anchor=g['anchor'] + 100) for g in games]
        result = stats.fit(translated, prior_mean=2600)
        self.assertAlmostEqual(result['estimate'], original['estimate'] + 100)
        for new, old in zip(result['interval95'], original['interval95']):
            self.assertAlmostEqual(new, old + 100)

    def test_duplicate_or_inconsistent_identities_rejected(self):
        games = rows([(1, 0), (.5, .5)])
        for bad in (games + [games[0]],
                    [games[0], dict(games[1], opponent_id='b')],
                    [games[0], dict(games[1], anchor=2600)],
                    [games[0], dict(games[1], family='other')],
                    games + [dict(games[0], game_id='third')]):
            with self.assertRaises(ValueError):
                stats.fit(bad)

    def test_bad_scores_and_priors_rejected(self):
        game = rows([(1,)])[0]
        for score in (None, float('nan'), True, .25, -1, 2):
            with self.assertRaises(ValueError):
                stats.fit([dict(game, score=score)])
        for sd in (0, -1, float('inf'), float('nan')):
            with self.assertRaises(ValueError):
                stats.fit([game], prior_sd=sd)

    def test_family_disagreement_and_prior_sensitivity(self):
        games = rows([(1, 1)] * 30 + [(0, 0)] * 10)
        games += rows([(1, 1)] * 10 + [(0, 0)] * 30, opponent='b', family='b')
        result = stats.summarize(games)
        self.assertAlmostEqual(result['joint']['estimate'], 2500)
        self.assertTrue(result['family_disagreement_over_150'])
        self.assertEqual(set(result['prior_sensitivity']), {'250.0', '500.0', '1000.0'})
        one_sided = stats.summarize(rows([(1, 1)] * 4))['prior_sensitivity']
        self.assertLess(one_sided['250.0']['estimate'], one_sided['1000.0']['estimate'])


if __name__ == '__main__':
    unittest.main()
