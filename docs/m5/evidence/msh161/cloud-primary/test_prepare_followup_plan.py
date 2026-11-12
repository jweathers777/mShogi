"""Offline followup coverage and refusal checks using completed immutable evidence."""
import argparse,copy,tempfile,unittest
from pathlib import Path
import prepare_followup_plan as f
H=Path(__file__).resolve().parent
class Tests(unittest.TestCase):
 def test_actual_plan_reuses_played_and_only_adds_two_alternatives(self):
  p=f.c.read(H/'batch-02-followup/plan.json');f.c.validate_plan(p)
  self.assertEqual({(j['spec']['id'],j['game'],j['ply'],j['move']) for j in p['jobs']},{('main-02-02',0,65,'g8g7'),('main-02-08',0,96,'c5e4')})
  self.assertEqual(len(p['reused_queries']),2);self.assertEqual(len(p['bins']),2)
  self.assertEqual({r['played'] for r in p['reused_queries']},{'f6f8','f4f5'})
 def refused(self,rows):
  with tempfile.TemporaryDirectory() as t:
   d=Path(t);s=d/'selection.json';s.write_bytes(f.c.core.encode(rows))
   a=argparse.Namespace(prior_plan=str(H/'batch-02-adjudication/plan.json'),cache=str(H/'local-cache'),selection=[str(s)],plan_id='test-offline',output=str(d/'out'))
   with self.assertRaises(ValueError):f.prepare(a)
   self.assertFalse((d/'out').exists())
 def test_duplicate_request_refused(self):
  row=f.c.read(H/'batch-02-adjudication/followup-selection-00-04.json')[0];self.refused([row,copy.deepcopy(row)])
 def test_already_searched_alternative_refused(self):
  row=f.c.read(H/'batch-02-adjudication/followup-selection-05-09.json')[0]
  plan=f.c.read(H/'batch-02-adjudication/plan.json')
  prior=next(j['move'] for j in plan['jobs'] if j['spec']['id']==row['pair'] and j['game']==row['game'] and j['ply']==row['ply'] and j['move'] not in [None,row['moves'][0]])
  row['moves'][1]=prior;self.refused([row])
 def test_illegal_alternative_refused(self):
  row=f.c.read(H/'batch-02-adjudication/followup-selection-00-04.json')[0];row['moves'][1]='a1a8';self.refused([row])
if __name__=='__main__':unittest.main()
