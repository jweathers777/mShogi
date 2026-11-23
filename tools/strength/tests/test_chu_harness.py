"""Chu clock policy and outcome-only data projection checks."""
import contextlib
import io
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

from tools.strength import native_match, texel_data


class ChuHarnessTests(unittest.TestCase):
    def test_clocked_catalog_pair_uses_nominal_and_retains_time_odds(self):
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            report = {'games': [dict(pairing={'swapped': swap}, score_first_twice=2,
                                     steps=[], stop='native_concluded') for swap in (False, True)]}
            (out / 'pair-0001.report.json').write_text(json.dumps(report))
            args = SimpleNamespace(config_writer='writer', candidate='candidate', baseline='baseline',
                candidate_nodes=5_000_000, baseline_depth=64, baseline_nodes=5_000_000,
                calibrate=False, max_steps=500, pair_wall_ms=180000, base=5, increment=.1,
                game='chu', candidate_time_factor=2, openings='openings.json', opening_count=100,
                watchdog_ms=None, runner='runner')
            with patch.object(native_match.subprocess, 'run') as run, patch.object(native_match.subprocess, 'Popen') as proc:
                proc.return_value.wait.return_value = 0
                result = native_match.run_pair(args, 1, out)
            command = run.call_args_list[0].args[0]
            self.assertEqual(command[command.index('--clock-allocation') + 1], 'nominal')
            self.assertEqual(command[command.index('--candidate-time-factor') + 1], '2')
            self.assertEqual(command[command.index('--opening-index') + 1], '0')
            self.assertEqual(result['scores_twice'], [2, 0])
            self.assertEqual(result['pair_score'], .5)

    def test_chu_unfinished_pairs_stop_instead_of_becoming_draws(self):
        args = SimpleNamespace(game='chu')
        self.assertIsNotNone(native_match.unfinished(args, [
            {'pair_score': None, 'stops': ['native_concluded', 'step_limit']}]))

    def test_chu_split_keeps_pair_together_and_has_no_unfinished_labels(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            games = [dict(game_id=str(i), opening_id='same-opening', sente_outcome=outcome,
                outcome=None if outcome is None else {'winner': 'sente'}, truncated=outcome is None,
                stop='native_concluded' if outcome is not None else 'step_limit',
                samples=[] if outcome is None else [dict(diagram={'pieces': [i], 'side': 'sente', 'protection': []},
                                                           sente_outcome=outcome)])
                for i, outcome in enumerate([1, 0, None])]
            (root / 'games-0.jsonl').write_text('\n'.join(map(json.dumps, games)))
            with contextlib.redirect_stdout(io.StringIO()):
                texel_data.split(root, root / 'manifest.json', game='chu', by_opening=True, allow_empty=True)
            rows = [json.loads(line) for part in ('train','validation','heldout')
                    for line in (root / (part + '.jsonl')).read_text().splitlines()]
            self.assertEqual(len(rows), 2)
            self.assertEqual(len({r['split'] for r in rows}), 1)
            self.assertEqual({r['result'] for r in rows}, {0, 1})


if __name__ == '__main__':
    unittest.main()
