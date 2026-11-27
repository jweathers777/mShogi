"""The match driver must preserve native scoring and incomplete pairs."""
import unittest
import contextlib
import io
import json
import tempfile
import subprocess
from pathlib import Path
from types import SimpleNamespace
from unittest import mock

from tools.strength import native_match
from tools.strength.native_match import summarize


class PairScoring(unittest.TestCase):
    def test_tenjiku_requires_an_explicit_immutable_profile(self):
        common = ['native_match', '--candidate', 'unused', '--baseline', 'unused',
                  '--candidate-nodes', '1000', '--out', 'unused']
        for extra in [['--game', 'tenjiku'], ['--game', 'chu', '--profile', 'adams']]:
            with self.subTest(extra=extra), mock.patch('sys.argv', common + extra):
                with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as raised:
                    native_match.main()
                self.assertEqual(raised.exception.code, 2)

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

    def test_pending_xiangqi_adjudication_stays_unscored(self):
        report = self.report(None, None)
        report['games'][0]['stop'] = 'native_pending_adjudication'
        result = summarize(report)
        self.assertIsNone(result['pair_score'])
        self.assertEqual(result['stops'][0], 'native_pending_adjudication')

    def test_chu_capacity_stop_does_not_become_a_draw(self):
        report = self.report(2, None)
        report['games'][1]['stop'] = 'native_stop:ResourceLimit'
        result = summarize(report)
        self.assertIsNone(result['pair_score'])
        self.assertEqual(result['scores_twice'], [2, None])
        self.assertEqual(result['stops'][1], 'native_stop:ResourceLimit')


class NativeClockDispatch(unittest.TestCase):
    def test_operator_can_schedule_an_hour_long_pair(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory) / 'campaign'
            command = ['native_match', '--candidate', 'new', '--baseline', 'old',
                       '--candidate-nodes', '512', '--out', str(out),
                       '--pair-wall-ms', '3600001', '--wall-seconds', '3800',
                       '--max-steps', '100000', '--max-pairs', '1']
            row = summarize(PairScoring().report(2, 0))
            row['wall_seconds'] = 1
            with mock.patch('sys.argv', command), mock.patch.object(native_match, 'environment', return_value={}), \
                 mock.patch.object(native_match, 'engine_identity', return_value={}), \
                 mock.patch.object(native_match, 'run_pair', return_value=row) as play, \
                 contextlib.redirect_stdout(io.StringIO()):
                native_match.main()
            args = play.call_args.args[0]
            self.assertEqual((args.pair_wall_ms, args.max_steps), (3600001, 100000))
            self.assertEqual(json.loads((out / 'summary.json').read_text())['scored_pairs'], 1)

    def test_pair_replays_before_scoring_and_selects_fixed_opening(self):
        report = PairScoring().report(2, 0)
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            args = SimpleNamespace(config_writer='writer', candidate='new', baseline='old',
                candidate_nodes=5_000_000, baseline_depth=64, baseline_nodes=5_000_000,
                calibrate=False, max_steps=100_000, pair_wall_ms=900_000, game='shogi',
                watchdog_ms=None, runner='runner', base=10, increment=.1,
                openings='frozen.json', opening_count=4)
            (out / 'pair-0006.report.json').write_text(json.dumps(report))
            child = mock.Mock()
            child.wait.return_value = 0
            with mock.patch.object(native_match.subprocess, 'Popen', return_value=child), \
                 mock.patch.object(native_match.subprocess, 'run') as run:
                result = native_match.run_pair(args, 6, out)
            command = run.call_args_list[0].args[0]
            self.assertEqual(command[7:9], ['100000', '900000'])
            self.assertEqual(command[-8:], ['--clock-base-ms', '10000', '--clock-increment-ms', '100',
                                           '--openings', 'frozen.json', '--opening-index', '1'])
            self.assertEqual(run.call_args_list[1].args[0][1], 'experiment-replay')
            self.assertEqual(result['pair_score'], 1)
            self.assertTrue((out / result['report']).exists())

    def test_replay_failure_keeps_raw_report_and_prevents_score(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            raw = out / 'pair-0001.report.json'
            raw.write_text('{}')
            args = SimpleNamespace(config_writer='writer', candidate='new', baseline='old',
                candidate_nodes=100, baseline_depth=1, baseline_nodes=100, calibrate=False,
                max_steps=320, pair_wall_ms=1000, game='shogi', watchdog_ms=None, runner='runner')
            child = mock.Mock()
            child.wait.return_value = 0
            with mock.patch.object(native_match.subprocess, 'Popen', return_value=child), \
                 mock.patch.object(native_match.subprocess, 'run', side_effect=[None, subprocess.CalledProcessError(1, 'replay')]), \
                 self.assertRaises(subprocess.CalledProcessError):
                native_match.run_pair(args, 1, out)
            self.assertTrue(raw.exists())
            self.assertFalse((out / 'pair-0001.summary.json').exists())

    def test_cleanup_failure_cannot_enter_statistics(self):
        report = PairScoring().report(2, 0)
        report['games'][0]['cleanup'] = ['unacknowledged']
        with self.assertRaises(ValueError):
            summarize(report)


if __name__ == '__main__':
    unittest.main()
