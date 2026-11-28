"""Chu clocked pair and outcome-only data projection checks."""
import contextlib
import io
import json
from pathlib import Path
import tempfile
import unittest

from tools.strength import texel_data


class ChuHarnessTests(unittest.TestCase):
    def test_cloud_run_uses_only_manifested_tools(self):
        from tools.strength.batch import validate
        manifest = dict(schema=1, campaign_started_epoch=0, deadline_epoch=1800,
                        estimated_compute_before_usd=0,
                        files={'bench': {'uri': 'gs://bucket/bench', 'sha256': 'a' * 64}},
                        bins=[[['bench', '--source', 'input.json', '--out', 'out.json']]])
        validate(manifest)
        for command in ([], ['cloud-submit'], ['unknown']):
            manifest['bins'] = [[command]]
            with self.assertRaises(ValueError):
                validate(manifest)

    def test_chu_split_keeps_pair_together_and_has_no_unfinished_labels(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            games = [dict(game_id=str(i), opening_id='same-opening', sente_outcome=outcome,
                outcome=None if outcome is None else {'winner': 'sente'}, truncated=outcome is None,
                stop='game_concluded' if outcome is not None else 'step_limit',
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
