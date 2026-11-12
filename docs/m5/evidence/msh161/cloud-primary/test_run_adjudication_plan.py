"""Offline orchestration ordering; no cloud calls or engines."""
import sys,unittest
from unittest.mock import patch
import run_adjudication_plan as run

class Tests(unittest.TestCase):
 def invoke(self,fail=False,changed=False):
  events=[]
  def submit(args):
   events.append('submit')
   if fail:raise RuntimeError('preserved ambiguous submission')
  args=['run','--directory','unused','--job-name','job','--subscription','sub','--cache','cache','--spending-approved']
  with patch.object(sys,'argv',args),patch.object(run.control,'read',return_value={'controller_sha256':'frozen'}),patch.object(run.control.core,'file_hash',return_value='changed' if changed else 'frozen'),patch.object(run.control,'submit',side_effect=submit),patch.object(run.control,'wait_verify',side_effect=lambda a:events.append('wait-verify')):
   if fail or changed:
    with self.assertRaises((ValueError,RuntimeError)):run.main()
   else:run.main()
  return events
 def test_submit_then_wait_once(self):self.assertEqual(self.invoke(),['submit','wait-verify'])
 def test_submit_failure_stops_without_wait_or_retry(self):self.assertEqual(self.invoke(fail=True),['submit'])
 def test_changed_source_never_submits(self):self.assertEqual(self.invoke(changed=True),[])
if __name__=='__main__':unittest.main()
