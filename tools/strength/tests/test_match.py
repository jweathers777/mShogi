"""Independent game evidence, pair scoring, and campaign bounds."""
import argparse
import contextlib
import io
import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest import mock

from tools.strength import cli, internal_match
from tools.strength.internal_match import score_pair, summarize_game


def report(swapped, score, stop='game_concluded'):
    return {'games': [{'pairing': {'swapped': swapped}, 'score_first_twice': score,
                      'steps': [], 'stop': stop}]}


class PairScoring(unittest.TestCase):
    def test_tenjiku_requires_an_explicit_immutable_profile(self):
        common = ['match', '--candidate', 'unused', '--baseline', 'unused',
                  '--candidate-nodes', '1000', '--out', 'unused']
        for extra in [['--game', 'tenjiku'], ['--game', 'chu', '--profile', 'adams']]:
            with self.subTest(extra=extra), contextlib.redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit) as raised:
                    cli.main(common + extra)
                self.assertEqual(raised.exception.code, 2)

    def test_invalid_catalog_reports_a_cli_error(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            catalog = root / 'catalog.json'
            command = ['match', '--game', 'chess', '--candidate', 'new', '--baseline', 'old',
                       '--candidate-nodes', '512', '--openings', str(catalog)]
            for index, (contents, error) in enumerate([
                ({'profile': 'mshogi.chu.jcsa2019.v1'}, 'another profile'),
                ({'openings': []}, 'nonempty unique records'),
                ({'openings': [{'id': 'same'}, {'id': 'same'}]}, 'nonempty unique records'),
            ]):
                stderr = io.StringIO()
                catalog.write_text(json.dumps(contents))
                with contextlib.redirect_stderr(stderr), self.assertRaises(SystemExit) as raised:
                    cli.main(command + ['--out', str(root / f'out-{index}')])
                self.assertEqual(raised.exception.code, 2)
                self.assertIn(error, stderr.getvalue())

    def test_colors_are_reversed_before_pair_statistics(self):
        for first, second, scores, pair in [
            (2, 0, [2, 2], 1), (0, 2, [0, 0], 0),
            (2, 2, [2, 0], .5), (1, 0, [1, 2], .75),
        ]:
            result = score_pair(1, [summarize_game(report(False, first), False),
                                    summarize_game(report(True, second), True)])
            self.assertEqual(result['scores_twice'], scores)
            self.assertEqual(result['pair_score'], pair)

    def test_cutoffs_do_not_become_draws_or_scored_pairs(self):
        for stop in ['pending_adjudication', 'wall_budget', 'stop:ResourceLimit']:
            result = score_pair(1, [summarize_game(report(False, 2), False),
                                    summarize_game(report(True, None, stop), True)])
            self.assertEqual(result['scores_twice'], [2, None])
            self.assertIsNone(result['pair_score'])
            self.assertEqual(result['stops'][1], stop)

    def test_wrong_color_or_cleanup_failure_cannot_enter_statistics(self):
        first = report(False, 2)
        with self.assertRaises(ValueError):
            summarize_game(first, True)
        first['games'][0]['cleanup'] = ['unacknowledged']
        with self.assertRaises(ValueError):
            summarize_game(first, False)


class ClockDispatch(unittest.TestCase):
    def campaign(self, command, play):
        with mock.patch.object(internal_match, 'environment', return_value={}), \
             mock.patch.object(internal_match, 'engine_identity', return_value={}), \
             mock.patch.object(internal_match, 'run_game', side_effect=play), \
             contextlib.redirect_stdout(io.StringIO()):
            cli.main(command)

    def test_selfplay_depth_sets_both_work_limits_and_rejects_clocks(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory) / 'selfplay'
            command = ['selfplay', '--game', 'tenjiku', '--profile', 'adams',
                       '--engine', 'engine', '--depth', '1', '--out', str(out), '--max-pairs', '1']
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                cli.main(command + ['--base', '60'])

            def play(args, seed, swapped, out):
                self.assertEqual((args.depth, args.baseline_depth), (1, 1))
                self.assertEqual((args.candidate_nodes, args.baseline_nodes), (500000, 500000))
                return dict(summarize_game(report(swapped, 2), swapped), wall_seconds=1)

            self.campaign(command + ['--candidate-nodes', '500000'], play)

    def test_failed_game_does_not_stop_its_opposite_color_or_later_pairs(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory) / 'campaign'
            command = ['match', '--game', 'shogi', '--candidate', 'new', '--baseline', 'old',
                       '--candidate-nodes', '512', '--out', str(out), '--game-wall-ms', '3600001',
                       '--wall-seconds', '7700', '--max-steps', '100000', '--max-pairs', '2',
                       '--workers', '2']

            def play(args, seed, swapped, out):
                self.assertEqual((args.game_wall_ms, args.max_steps), (3600001, 100000))
                failed = seed == 1 and not swapped
                return dict(summarize_game(report(swapped, None if failed else 2,
                                                  'runner_error' if failed else 'game_concluded'), swapped),
                            wall_seconds=1)

            self.campaign(command, play)
            result = json.loads((out / 'summary.json').read_text())
            self.assertEqual((result['pairs'], result['scored_pairs'], result['unscored_games']), (2, 1, 1))
            self.assertEqual(result['wins_draws_losses'], [1, 0, 2])
            self.assertEqual(result['rows'][0]['stops'], ['runner_error', 'game_concluded'])

    def test_all_runner_failures_are_retained_and_return_a_campaign_error(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory) / 'campaign'
            command = ['match', '--game', 'chess', '--candidate', 'new', '--baseline', 'old',
                       '--candidate-nodes', '20', '--max-pairs', '2', '--out', str(out)]
            with mock.patch.object(internal_match, 'environment', return_value={}), \
                 mock.patch.object(internal_match, 'engine_identity', return_value={}), \
                 mock.patch.object(internal_match.subprocess, 'run',
                                   side_effect=subprocess.CalledProcessError(1, 'config')), \
                 contextlib.redirect_stdout(io.StringIO()), self.assertRaises(SystemExit) as raised:
                cli.main(command)
            self.assertIn('no scheduled game produced a verified report', str(raised.exception))
            result = json.loads((out / 'summary.json').read_text())
            self.assertEqual((result['pairs'], result['unscored_games']), (2, 4))
            self.assertEqual(len(list(out.glob('game-*.config.log'))), 4)

    def settings(self, options):
        parser = argparse.ArgumentParser()
        internal_match.options(parser)
        args = parser.parse_args(['--game', 'tenjiku', '--profile', 'adams',
                                  '--candidate', 'new', '--baseline', 'old', '--out', 'out', *options])
        internal_match.configure(args, parser)
        return args

    def test_long_clock_defaults_include_both_balances_and_time_odds(self):
        args = self.settings(['--base', '60', '--increment', '1.5', '--max-steps', '768'])
        self.assertEqual(args.game_wall_ms, 1_399_200)
        odds = self.settings(['--base', '60', '--increment', '1.5', '--max-steps', '769',
                              '--candidate-time-factor', '2'])
        self.assertEqual(odds.game_wall_ms, 2_102_100)
        calibration = self.settings(['--base', '60', '--increment', '1.5', '--calibrate'])
        self.assertEqual(calibration.game_wall_ms, 151_500)

    def game_args(self, **overrides):
        options = dict(helper='writer', profile=None, candidate='new', baseline='old',
                       candidate_nodes=5_000_000, baseline_depth=64, baseline_nodes=5_000_000,
                       calibrate=False, max_steps=100_000, game_wall_ms=900_000, game='shogi',
                       watchdog_ms=None, runner='runner', base=10, increment=.1,
                       openings='frozen.json', opening_count=4, random_plies=0, candidate_time_factor=1)
        return SimpleNamespace(**(options | overrides))

    def test_game_replays_before_scoring_and_selects_fixed_opening(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            args = self.game_args(candidate_time_factor=2)
            (out / 'game-0006-second.report.json').write_text(json.dumps(report(True, 0)))
            child = mock.Mock()
            child.wait.return_value = 0
            with mock.patch.object(internal_match.subprocess, 'Popen', return_value=child), \
                 mock.patch.object(internal_match.subprocess, 'run') as run:
                result = internal_match.run_game(args, 6, True, out)
            command = run.call_args_list[0].args[0]
            self.assertEqual(command[8:10], ['100000', '900000'])
            for flag, value in [('--swapped', 'true'), ('--clock-base-ms', '10000'),
                                ('--clock-increment-ms', '100'), ('--opening-index', '1'),
                                ('--candidate-time-factor', '2')]:
                self.assertEqual(command[command.index(flag) + 1], value)
            self.assertEqual(run.call_args_list[1].args[0][1], 'experiment-replay')
            self.assertEqual(result['score_twice'], 2)
            self.assertTrue((out / result['report']).exists())

    def test_replay_failure_keeps_raw_report_and_prevents_score(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            raw = out / 'game-0001-first.report.json'
            raw.write_text('{}')
            child = mock.Mock()
            child.wait.return_value = 0
            with mock.patch.object(internal_match.subprocess, 'Popen', return_value=child), \
                 mock.patch.object(internal_match.subprocess, 'run',
                                   side_effect=[None, subprocess.CalledProcessError(1, 'replay')]):
                result = internal_match.run_game(self.game_args(), 1, False, out)
            self.assertTrue(raw.exists())
            self.assertEqual(result['report'], raw.name)
            self.assertEqual(result['stop'], 'runner_error')
            self.assertIsNone(result['score_twice'])

    def test_watchdog_only_stops_its_game(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            child = mock.Mock()
            with mock.patch.object(internal_match.subprocess, 'Popen', return_value=child), \
                 mock.patch.object(internal_match.subprocess, 'run'), \
                 mock.patch.object(internal_match, 'wait_for_game',
                                   side_effect=subprocess.TimeoutExpired('runner', 1)), \
                 mock.patch.object(internal_match, 'stop_game') as stop:
                result = internal_match.run_game(self.game_args(), 1, False, out)
            stop.assert_called_once_with(child)
            self.assertEqual(result['stop'], 'runner_error')
            self.assertIsNone(result['score_twice'])


if __name__ == '__main__':
    unittest.main()
