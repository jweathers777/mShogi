"""Offline orchestration regressions; subprocess execution is forbidden."""
import copy,importlib.util,os,sys,unittest,tempfile,argparse
from pathlib import Path
from unittest.mock import patch
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE))
import adjudication_control as c
import adjudication_dispatch as d
class Memory:
 def __init__(self):self.data={};self.reads=[]
 def get(self,k):self.reads.append(k);return self.data[k]
 def put(self,k,v):
  if k in self.data:return False
  self.data[k]=v;return True
class Tests(unittest.TestCase):
 @classmethod
 def setUpClass(cls):cls.plan=c.read(HERE/'batch-01-adjudication/plan.json')
 def test_actual_plan_identity_and_discovery(self):
  p=self.plan;c.validate_plan(p);self.assertEqual(len(p['decisions']),298)
  seen=set()
  for j in p['jobs']:
   key=(j['spec']['id'],j['game'],j['ply'],j['move']);self.assertNotIn(key,seen);seen.add(key)
   self.assertEqual(j['source_revision'],'51351c48ea1adf9ac96011aec1440e44ceb741a4')
   if j['move'] is None:self.assertFalse(any(k['spec']==j['spec'] and k['game']==j['game'] and k['ply']==j['ply'] and k['move']==j['adjudication']['decision']['played'] for k in p['jobs']))
  weights=[sum(2 if next(j for j in p['jobs'] if j['id']==i)['move'] is None else 1 for i in b) for b in p['bins']];self.assertLessEqual(max(weights)-min(weights),1)
 def test_duplicate_bins_refused(self):
  p=copy.deepcopy(self.plan);p['bins'][1].append(p['bins'][0][0])
  with self.assertRaises(ValueError):c.validate_plan(p)
 def test_small_panels_have_no_empty_workers(self):
  p=copy.deepcopy(self.plan);p['jobs']=p['jobs'][:2];p['bins']=c.balanced(p['jobs'])
  self.assertEqual(len(p['bins']),2);c.validate_plan(p)
  p['bins'].append([])
  with self.assertRaises(ValueError):c.validate_plan(p)
  with self.assertRaises(ValueError):c.balanced([])
 def test_wrong_budget_and_modified_job_refused(self):
  p=copy.deepcopy(self.plan);p['jobs'][0]['nodes']=5000000
  with self.assertRaises(ValueError):c.validate_plan(p)
 def test_request_dispatch_hash_binding(self):
  p=copy.deepcopy(self.plan);p['dispatcher_sha256']='0'*64
  with self.assertRaises(ValueError):c.batch_request(p,HERE/'batch-01-adjudication/plan.json',HERE/'batch-01-adjudication/dispatcher.py')
  actual=c.batch_request(self.plan,HERE/'batch-01-adjudication/plan.json',HERE/'batch-01-adjudication/dispatcher.py')
  self.assertEqual(actual,c.read(HERE/'batch-01-adjudication/batch-request.json'))
 def dispatcher_fixture(self):
  p=copy.deepcopy(self.plan);store=Memory();raw=c.core.encode(p);store.data['plan']=raw;store.data[p['original_manifest']['key']]=b'manifest'
  p['original_manifest']['sha256']=c.core.digest(b'manifest')
  for j in p['jobs']:
   for r in j['adjudication']['consumed_successes']:store.data[r['key']]=b'bound';r['sha256']=c.core.digest(b'bound')
   j['id']=c.core.job_id(j)
  p['bins']=c.balanced(p['jobs']);raw=c.core.encode(p);store.data['plan']=raw
  args=['dispatcher','--store',p['store_uri'],'--plan-key','plan','--plan-sha256',c.core.digest(raw)]
  return p,store,args
 def test_dispatch_assigned_only_and_immutable_reuse(self):
  p,store,args=self.dispatcher_fixture();executed=[]
  def fake_run(j,s,repo):
   key=c.core.prefix(j)+'/success.json'
   if key not in s.data:executed.append(j['id']);s.put(key,c.core.encode({'job':j,'status':'complete'}))
   return c.core.decode(s.get(key))
  with patch.object(d,'GCSStore',return_value=store),patch.object(d.core,'run_job',side_effect=fake_run),patch.object(d.cli,'install_signal_handlers'),patch.object(sys,'argv',args),patch.dict(os.environ,{'BATCH_TASK_INDEX':'0'}),patch.object(c.subprocess,'run',side_effect=AssertionError('No subprocess allowed')):
   d.main();d.main()
  self.assertEqual(executed,p['bins'][0]);self.assertTrue(all(k in executed for k in p['bins'][0]))
  assigned_refs={r['key'] for j in p['jobs'] if j['id'] in p['bins'][0] for r in j['adjudication']['consumed_successes']}
  allrefs={r['key'] for j in p['jobs'] for r in j['adjudication']['consumed_successes']}
  self.assertFalse((allrefs-assigned_refs)&set(store.reads))
 def test_wrong_plan_hash_no_execution(self):
  p,store,args=self.dispatcher_fixture();args[-1]='0'*64
  with patch.object(d,'GCSStore',return_value=store),patch.object(d.core,'run_job',side_effect=AssertionError('No execution allowed')),patch.object(sys,'argv',args):
   with self.assertRaises(ValueError):d.main()
 def test_verify_single_inventory_preserves_failed_attempts(self):
  import argparse
  p=copy.deepcopy(self.plan);p['jobs']=p['jobs'][:2];p['bins']=c.balanced(p['jobs'])
  p['consumed_successes']=[];p['original_manifest']={'key':'manifest','sha256':c.core.digest(b'fixture')}
  store=Memory();store.exists=lambda key:False
  store.data['manifest']=b'fixture';store.data[p['plan_prefix']+'/plan.json']=c.core.encode(p)
  receipts={};results={}
  for j in p['jobs']:
   prefix=c.core.prefix(j);artifacts={}
   for i,move in enumerate([j['move']] if j['move'] is not None else [None,j['adjudication']['decision']['played']]):
    key=prefix+'/q'+str(i);artifacts['analysis/g000-p000-attempt'+str(i)+'.json']={'key':key}
    store.data[key]=c.core.encode({'forced_move':move,'decision':j['adjudication']['decision'],'nodes_requested':20000000,'engine_sha256':j['build']['engines']['stockfish19']['sha256'],'status':'complete','exit_code':0,'process_group_settled':True,'raw_truncated':False,'assessment':{'score':{},'bestmove':move},'pv_independent_legal':True})
   results[j['id']]={'wall_seconds':3,'child_cpu_seconds':2,'artifacts':artifacts}
   store.data[prefix+'/success.json']=c.core.encode(results[j['id']])
   for name,status in [('a','failed'),('b','complete')]:
    key=prefix+'/attempts/'+name+'/receipt.json';receipts[key]=True
    store.data[key]=c.core.encode({'status':status,'wall_seconds':1,'child_cpu_seconds':.5})
  unrelated=p['experiment']+'/jobs/unrelated/attempts/a/receipt.json'
  store.data[unrelated]=c.core.encode({'status':'failed','wall_seconds':999})
  inventory=sorted([*receipts,unrelated,p['experiment']+'/jobs/unrelated/attempts/a/artifacts/receipt.json'])
  with patch.object(c,'read',return_value=p),patch.object(Path,'read_bytes',return_value=c.core.encode(p)),patch.object(c.core,'LocalStore',return_value=store),patch.object(c.core,'verify_result',side_effect=lambda s,j:results[j['id']]),patch.object(c.core,'file_hash',return_value='hash'),patch.object(c,'save_new') as save:
   with patch.object(store,'list',create=True,return_value=inventory) as listing:
    c.verify(argparse.Namespace(plan='plan',cache='cache',output='out'))
    listing.assert_called_once_with(p['experiment']+'/jobs/')
  result=save.call_args.args[1]
  self.assertEqual({r['key'] for r in result['attempts']},set(receipts))
  self.assertEqual(result['failed_attempts'],2);self.assertEqual(result['all_attempt_wall_seconds'],4)
 def wait_fixture(self,directory):
  planpath=Path(directory)/'plan.json';c.save_new(planpath,self.plan)
  (Path(directory)/'dispatcher.py').write_bytes((HERE/'batch-01-adjudication/dispatcher.py').read_bytes())
  req=c.batch_request(self.plan,planpath,Path(directory)/'dispatcher.py');c.save_new(Path(directory)/'batch-request.json',req)
  submitted=dict(req,name='projects/mshogi/locations/us-central1/jobs/review-test',uid='test-uid')
  c.save_new(Path(directory)/'submission-result.json',{'returncode':0,'stdout':c.core.encode(submitted).decode()})
  args=argparse.Namespace(plan=str(planpath),job_name='review-test',subscription='unused',cache=str(Path(directory)/'cache'),output_directory=str(Path(directory)/'out'))
  return args,submitted
 def test_wait_wrong_job_refused_before_cloud(self):
  with tempfile.TemporaryDirectory() as t:
   a,r=self.wait_fixture(t);a.job_name='wrong'
   with patch.object(c.gcp,'wait_for_completion',side_effect=AssertionError('No cloud allowed')):
    with self.assertRaises(ValueError):c.wait_verify(a)
 def test_wait_wrong_uid_no_fetch(self):
  with tempfile.TemporaryDirectory() as t:
   a,r=self.wait_fixture(t);r['uid']='wrong'
   with patch.object(c.gcp,'wait_for_completion',return_value={'job':r}),patch.object(c,'fetch_plan',side_effect=AssertionError('No fetch allowed')):
    with self.assertRaises(ValueError):c.wait_verify(a)
   self.assertTrue((Path(a.output_directory)/'failure.json').exists())
 def test_wait_failure_preserves_terminal_evidence(self):
  with tempfile.TemporaryDirectory() as t:
   a,r=self.wait_fixture(t);result={'job':dict(r,status={'state':'FAILED'})}
   with patch.object(c.gcp,'wait_for_completion',side_effect=c.gcp.BatchJobFailed(result)),patch.object(c,'fetch_plan',side_effect=AssertionError('No fetch allowed')):
    with self.assertRaises(c.gcp.BatchJobFailed):c.wait_verify(a)
   self.assertEqual(c.read(Path(a.output_directory)/'cloud-completion.json'),result)
if __name__=='__main__':unittest.main()
