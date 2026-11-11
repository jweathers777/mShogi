import copy, json, unittest
from pathlib import Path
import after_qualification as chain
import analysis
class RealSmoke(unittest.TestCase):
 def setUp(self):
  self.rows=[json.loads(p.read_text()) for p in sorted((Path(__file__).parent/'reference-smoke').glob('g*-attempt*.json'))]
 def test_uncertainty_is_operationally_valid_but_not_a_loss(self):
  chain.validate_smoke(self.rows)
  indexed={r['query'].rsplit('-',1)[-1]:r for r in self.rows}
  comparison=analysis.compare(indexed['unrestricted'],indexed['played'])
  self.assertIsNone(comparison['loss']);self.assertTrue(comparison['uncertain'])
 def test_invalid_evidence_is_refused(self):
  for mutation in ['missing_wdl','illegal_pv','bound_only','raw_truncated','other_uncertainty']:
   rows=copy.deepcopy(self.rows);r=rows[0]
   if mutation=='missing_wdl':r['assessment']['score']['wdl']=None
   if mutation=='illegal_pv':r['pv_independent_legal']=False
   if mutation=='bound_only':r['assessment']['score']['upperbound']=True
   if mutation=='raw_truncated':r['raw_truncated']=True
   if mutation=='other_uncertainty':r['assessment']['uncertainty']=['missing_wdl']
   with self.subTest(mutation=mutation),self.assertRaises(AssertionError):chain.validate_smoke(rows)
if __name__=='__main__':unittest.main()
