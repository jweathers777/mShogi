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

    def test_sfen_identity_keeps_hands_and_side_but_not_counter(self):
        self.assertEqual(key('board b G 1','shogi'),key('board b G 97','shogi'))
        self.assertNotEqual(key('board b G 1','shogi'),key('board b S 1','shogi'))
        self.assertNotEqual(key('board b G 1','shogi'),key('board w G 1','shogi'))

    def test_small_demo_keeps_sente_labels_and_marks_empty_partitions(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)
            g={'game_id':'one','initial_sfen':'board b G 1','sente_outcome':0,
               'outcome':{'winner':'gote'},'truncated':False,'stop':'native_outcome',
               'samples':[{'sfen':'board w G 8','sente_outcome':0},
                          {'sfen':'board w G 12','sente_outcome':0},
                          {'sfen':'board w S 12','sente_outcome':0}]}
            (root/'games-0.jsonl').write_text(json.dumps(g)+'\n')
            with contextlib.redirect_stdout(io.StringIO()):
                with self.assertRaisesRegex(AssertionError,'empty'):
                    split(root,root/'strict.json','shogi')
                self.assertFalse((root/'train.jsonl').exists())
                split(root,root/'manifest.json','shogi',allow_empty=True)
            m=json.loads((root/'manifest.json').read_text())
            self.assertFalse(m['training_ready'])
            rows=[json.loads(line) for part in ('train','validation','heldout') for line in (root/(part+'.jsonl')).read_text().splitlines()]
            self.assertEqual(len(rows),2)
            self.assertEqual({r['result'] for r in rows},{0})
            self.assertTrue(all('sfen' in r and 'fen' not in r for r in rows))

    def test_chu_board_tuple_order_is_not_part_of_identity(self):
        pieces = [["1a", {"side": "g", "base": "K", "promoted": False}],
                  ["12l", {"side": "s", "base": "K", "promoted": False}]]
        diagram = {'pieces': pieces, 'side': 's', 'protection': {'owner': None, 'squares': []}}
        reversed_diagram = dict(diagram, pieces=list(reversed(pieces)))
        self.assertEqual(key(diagram, 'chu'), key(reversed_diagram, 'chu'))
        self.assertNotEqual(key(diagram, 'chu'), key(dict(diagram, side='g'), 'chu'))
