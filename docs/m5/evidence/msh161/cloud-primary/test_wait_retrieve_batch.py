"""Offline glue tests; no Google calls, subprocesses or engines."""
import copy,json,sys,tempfile,unittest
from pathlib import Path
from unittest.mock import patch
from types import SimpleNamespace
HERE=Path(__file__).resolve().parent;sys.path.insert(0,str(HERE))
import wait_retrieve_batch as w
class Store:
 def __init__(self,manifest):self.data={manifest['experiment']+'/manifest.json':w.core.encode(manifest)}
 def get(self,k):return self.data[k]
 def put(self,k,v):
  if k in self.data:return False
  self.data[k]=v;return True
class Tests(unittest.TestCase):
 def fixture(self,t):
  h=Path(t)
  for name in ['batch-01-manifest.json','batch-01-submission.json','cloud-config.json','retrieve_batch.py']:(h/name).write_bytes((HERE/name).read_bytes())
  manifest=w.read(h/'batch-01-manifest.json');submitted=w.read(h/'batch-01-submission.json');store=Store(manifest)
  result={'job':dict(submitted,status={'state':'SUCCEEDED'})}
  def retrieve(command,**kwargs):
   self.assertEqual(command[1:], [str(h/'retrieve_batch.py'),'1'])
   d=h/'batch-01-review';d.mkdir();(d/'retrieval-state.json').write_bytes(w.core.encode({'status':'retrieved_verified_needs_independent_review','experiment':manifest['experiment'],'verified_jobs':2410}))
   return SimpleNamespace(returncode=0)
  return h,store,result,retrieve
 def test_success_one_wait_one_retrieve_and_durable_receipts(self):
  with tempfile.TemporaryDirectory() as t:
   h,store,result,retrieve=self.fixture(t)
   with patch.object(w,'HERE',h),patch.object(w.gcp,'GCSStore',return_value=store),patch.object(w.gcp,'wait_for_completion',return_value=result) as wait,patch.object(w.subprocess,'run',side_effect=retrieve) as sub:
    state=w.run(1,'fake-subscription')
   wait.assert_called_once();sub.assert_called_once();self.assertEqual(state['verified_jobs'],2410);self.assertEqual(len([k for k in store.data if '/completions/' in k]),2)
 def test_wrong_uid_never_retrieves(self):
  with tempfile.TemporaryDirectory() as t:
   h,store,result,_=self.fixture(t);result['job']['uid']='wrong'
   with patch.object(w,'HERE',h),patch.object(w.gcp,'GCSStore',return_value=store),patch.object(w.gcp,'wait_for_completion',return_value=result),patch.object(w.subprocess,'run',side_effect=AssertionError('No retrieval')):
    with self.assertRaises(ValueError):w.run(1,'fake')
   self.assertEqual(w.read(h/'batch-01-wait-retrieve.json')['status'],'failed_preserved_for_review')
 def test_failed_batch_retains_terminal_and_no_retrieval(self):
  with tempfile.TemporaryDirectory() as t:
   h,store,result,_=self.fixture(t);result['job']['status']['state']='FAILED'
   with patch.object(w,'HERE',h),patch.object(w.gcp,'GCSStore',return_value=store),patch.object(w.gcp,'wait_for_completion',side_effect=w.gcp.BatchJobFailed(result)),patch.object(w.subprocess,'run',side_effect=AssertionError('No retrieval')):
    with self.assertRaises(w.gcp.BatchJobFailed):w.run(1,'fake')
   self.assertEqual(w.read(h/'batch-01-terminal.json'),result);self.assertTrue(any('/completions/' in k for k in store.data))
 def test_wrong_manifest_never_waits(self):
  with tempfile.TemporaryDirectory() as t:
   h,store,result,_=self.fixture(t);store.data[next(iter(store.data))]=b'{}'
   with patch.object(w,'HERE',h),patch.object(w.gcp,'GCSStore',return_value=store),patch.object(w.gcp,'wait_for_completion',side_effect=AssertionError('No wait')):
    with self.assertRaises(ValueError):w.run(1,'fake')
if __name__=='__main__':unittest.main()
