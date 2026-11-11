import concurrent.futures
import copy
import os
import platform
import tempfile
import threading
import unittest
from pathlib import Path
from unittest.mock import patch
from tools.experiments import core, cli

class CoreTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name);self.store=core.LocalStore(self.root/'store')
        self.binary=self.root/'binary';self.binary.write_bytes(b'fixture executable identity')
        self.source=self.root/'source';self.source.write_bytes(b'frozen source')
        engine={'path':str(self.binary),'sha256':core.file_hash(self.binary),'source':'b'*40}
        self.job=core.seal({'kind':'game_pair','experiment':'test','source_revision':'a'*40,
            'build':{'source':'b'*40,'compiler':'fixture','architecture':platform.machine(),'build_flags':'none','engines':{'runner':engine,'candidate':engine,'opponent':engine,'stockfish19':engine}},
            'source_files':{'source':core.file_hash(self.source)},'environment':{'architecture':platform.machine(),'system':platform.system()},
            'spec':{'opening':{'fen':'fixture'},'opponent':'opponent'},'inputs':{},'expected_outputs':['report.json.gz']})
    def execute(self,job,out,inputs,repo):
        (out/'report.json.gz').write_bytes(b'verified fixture report')
        return {'complete':True}
    def run_job(self,executor=None):return core.run_job(self.job,self.store,self.root,executor or self.execute)
    def test_success_restart_skips_execution_and_preserves_result(self):
        first=self.run_job()
        second=self.run_job(lambda *args:self.fail('successful job recomputed'))
        self.assertEqual(first,second)
        self.assertEqual(1,len([k for k in self.store.list('test/jobs/') if k.endswith('/receipt.json')]))
    def test_failure_never_success_and_retry_preserves_both(self):
        def broken(job,out,*args):
            (out/'partial').write_text('failure retained');raise RuntimeError('fixture failure')
        with self.assertRaisesRegex(RuntimeError,'fixture failure'):self.run_job(broken)
        self.assertFalse(self.store.exists(core.prefix(self.job)+'/success.json'))
        self.assertFalse(self.store.exists(core.prefix(self.job)+'/claim.json'))
        self.run_job()
        receipts=[core.decode(self.store.get(k)) for k in self.store.list('test/jobs/') if k.endswith('/receipt.json')]
        self.assertEqual({'failed','complete'},{r['status'] for r in receipts})
        self.assertEqual(b'failure retained',self.store.get(next(k for k in self.store.list('test/jobs/') if k.endswith('/partial'))))
    def test_incomplete_output_fails(self):
        with self.assertRaisesRegex(ValueError,'missing expected'):self.run_job(lambda *a:{'complete':True})
        self.assertFalse(self.store.exists(core.prefix(self.job)+'/success.json'))
    def test_success_corruption_cannot_trigger_rerun(self):
        result=self.run_job();key=result['artifacts']['report.json.gz']['key']
        self.store.path(key).write_bytes(b'corrupted')
        with self.assertRaisesRegex(ValueError,'corrupt'):self.run_job(lambda *a:self.fail('recomputed'))
    def test_concurrent_claim_does_not_duplicate(self):
        entered=threading.Event();release=threading.Event()
        def hold(*args):entered.set();release.wait(5);return self.execute(*args)
        with concurrent.futures.ThreadPoolExecutor(1) as pool:
            future=pool.submit(self.run_job,hold);self.assertTrue(entered.wait(5))
            try:
                with self.assertRaisesRegex(RuntimeError,'claimed'):self.run_job()
            finally:release.set()
            future.result()
    def test_interrupted_claim_requires_reconciliation(self):
        self.store.put(core.prefix(self.job)+'/claim.json',b'old owner')
        with self.assertRaisesRegex(RuntimeError,'claimed'):self.run_job()
        with self.assertRaisesRegex(ValueError,'ownership'):self.store.delete(core.prefix(self.job)+'/claim.json',b'new owner')
    def test_immutable_put_race(self):
        with concurrent.futures.ThreadPoolExecutor(8) as pool:
            outcomes=list(pool.map(lambda n:self.store.put('race/key',str(n).encode()),range(8)))
        self.assertEqual(1,sum(outcomes))
    def test_identity_source_binary_and_architecture(self):
        altered=copy.deepcopy(self.job);altered['spec']['opponent']='other'
        with self.assertRaisesRegex(ValueError,'altered'):core.validate_job(altered)
        self.source.write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError,'source file differs'):self.run_job()
        self.source.write_bytes(b'frozen source');self.binary.write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError,'engine executable differs'):self.run_job()
    def test_input_substitution_rejected(self):
        result=self.run_job();other=core.seal(dict(self.job,spec={'opening':{'fen':'other'},'opponent':'opponent'}))
        self.store.put(core.prefix(other)+'/success.json',core.encode(result))
        child=core.seal(dict(self.job,kind='confirmation',nodes=5000000,game=0,ply=0,
            inputs={'report':{'job':other['id'],'path':'report.json.gz'}}))
        with self.assertRaisesRegex(ValueError,'substituted'):core.run_job(child,self.store,self.root,self.execute)
    def test_cancel_stops_pending_but_allows_verified_success(self):
        self.run_job();self.store.put('test/cancel.json',b'{}');self.run_job()
        other=core.seal(dict(self.job,spec={'opening':{'fen':'other'},'opponent':'opponent'}))
        with self.assertRaisesRegex(RuntimeError,'cancelled'):core.run_job(other,self.store,self.root,self.execute)
    def test_unsafe_paths_and_duplicate_json(self):
        for key in ('../escape','/absolute','a/../../escape','a\\b','a//b'):
            with self.assertRaises(ValueError):self.store.put(key,b'bad')
        with self.assertRaises(ValueError):core.decode('{"id":1,"id":2}')
    def test_manifest_missing_dependency(self):
        child=core.seal(dict(self.job,kind='confirmation',nodes=1,inputs={'report':{'job':'c'*64,'path':'x'}}))
        with self.assertRaisesRegex(ValueError,'missing dependency'):core.validate_manifest({'schema':1,'experiment':'test','jobs':[child],'pipelines':[]})
    def test_missing_derived_plan_does_not_hide_confirmations(self):
        self.run_job()
        screen=core.seal(dict(self.job,kind='reference_screening',nodes=500000,
            inputs={'report':{'job':self.job['id'],'path':'report.json.gz'}},expected_outputs=['confirmation-plan.json']))
        def screen_executor(job,out,*args):
            (out/'confirmation-plan.json').write_bytes(core.encode([{'game':0,'ply':2,'played':'e2e4','alternative':'d2d4'}]));return {'complete':True}
        core.run_job(screen,self.store,self.root,screen_executor)
        manifest={'schema':1,'experiment':'test','jobs':[self.job,screen],'pipelines':[self.job['id']],'automatic_confirmations':True}
        alljobs=cli.all_jobs(manifest,self.store)
        self.assertEqual(4,len(alljobs))
        self.assertEqual(['pending','pending'],[r['state'] for r in core.status(dict(manifest,jobs=alljobs),self.store)][-2:])

if __name__=='__main__':unittest.main()
