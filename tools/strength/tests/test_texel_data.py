"""Partition tests: game families and clock-independent board identity."""
import contextlib
import hashlib
import io
import json
from pathlib import Path
import tempfile
import unittest
from tools.strength.texel_data import split, key


class PartitionTests(unittest.TestCase):
    def test_families_cross_split_duplicates_and_unfinished(self):
        # Find one opening family per partition using the documented hash rule.
        roots = {}
        for n in range(200):
            fen = f'root{n} w - - 0 1'
            bucket = int(hashlib.sha256(key(fen).encode()).hexdigest()[:16], 16) % 10
            part = 'heldout' if bucket == 0 else 'validation' if bucket == 1 else 'train'
            roots.setdefault(part, fen)
        games = []
        for i, part in enumerate(['train', 'validation', 'heldout']):
            games.append({'game_id': i, 'initial_fen': roots[part],
                          'white_outcome': 1.0, 'outcome': {'checkmate': {'winner': 'white'}},
                          'truncated': False, 'stop': 'native_outcome',
                          'samples': [{'fen': f'unique{i} w - - 0 1', 'white_outcome': 1.0},
                                      {'fen': f'shared w - - {i} 5', 'white_outcome': 1.0}]})
        repeated = dict(games[0], game_id=3)
        games.append(repeated)
        games.append(dict(games[0], game_id=4, samples=[], white_outcome=None,
                          outcome=None, truncated=True, stop='ply_limit'))
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root/'games-0.jsonl').write_text('\n'.join(map(json.dumps, games)))
            with contextlib.redirect_stdout(io.StringIO()):
                split(root, root/'manifest.json')
            manifest = json.loads((root/'manifest.json').read_text())
            self.assertEqual(manifest['cross_partition_rows_removed'], 4)
            self.assertEqual(manifest['remaining_within_partition_duplicates_removed'], 1)
            for part in roots:
                rows = [json.loads(s) for s in (root/(part+'.jsonl')).read_text().splitlines()]
                self.assertEqual(len(rows), 1)
                self.assertEqual(rows[0]['split'], part)
                self.assertNotIn('shared', rows[0]['fen'])
