"""Native probe process ownership and Shogi result-label partition contracts."""
import contextlib
import io
import json
import os
from pathlib import Path
import tempfile
import sys
import time
import unittest
from tools.strength.shogi_tools import native
from tools.strength.texel_data import key, split


class ShogiToolsTests(unittest.TestCase):
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

    @unittest.skipUnless(os.name=='posix','process-group test requires POSIX')
    def test_outer_probe_timeout_terminates_its_child(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp);helper=root/'helper';pidfile=root/'pid'
            helper.write_text(f'#!{sys.executable}\nimport subprocess,time\n'
                              'p=subprocess.Popen(["sleep","60"])\n'
                              f'open({str(pidfile)!r},"w").write(str(p.pid))\n'
                              'time.sleep(60)\n')
            helper.chmod(0o755)
            with self.assertRaisesRegex(RuntimeError,'retained'):
                native('validate',root/'unused',helper,timeout=1)
            pid=int(pidfile.read_text())
            for _ in range(100):
                try:os.kill(pid,0)
                except ProcessLookupError:break
                time.sleep(.01)
            else:self.fail('probe child survived timeout')
