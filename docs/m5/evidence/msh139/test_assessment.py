"""Offline measurement-contract checks; no chess rules implementation."""
import copy
import gzip
import json
import unittest
import subprocess
import sys
import time
import tempfile
import zipfile
from pathlib import Path
import assessment as a

class AssessmentTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.outer=json.loads(gzip.decompress((a.HERE.parent/'msh49-2/pair-000/report.json.gz').read_bytes()))
        cls.report=json.loads(cls.outer['report_json'])
        old=json.loads((a.HERE.parent/'msh49-2/build.json').read_text())
        cls.build={'candidate':old['artifacts']['mshogi'],'runner':{'source':old['source']},'references':{'reference':old['artifacts']['stockfish']}}
        cls.config=cls.report['config']

    def outer_for(self,report):
        out=copy.deepcopy(self.outer);out['report_json']=json.dumps(report);out['report_sha256']=__import__('hashlib').sha256(out['report_json'].encode()).hexdigest();return out

    def audit(self,r):return a.audit_report(self.outer_for(r),self.config,self.build,'reference')

    def test_original_completed_pair_score(self):
        actual=self.audit(self.report)
        self.assertTrue(actual['healthy']);self.assertTrue(actual['complete']);self.assertEqual(actual['pair_score'],0)

    def test_cap_or_failure_is_not_scored(self):
        for stop in ['ply_limit','engine_failure']:
            r=copy.deepcopy(self.report);r['games'][0]['stop']=stop
            actual=self.audit(r);self.assertFalse(actual['complete']);self.assertIsNone(actual['pair_score'])
        r=copy.deepcopy(self.report);r['games'][0]['diagnostic_failures']=['cleanup failed']
        actual=self.audit(r);self.assertFalse(actual['healthy']);self.assertIsNone(actual['pair_score'])

    def test_normal_stderr_is_kept_but_not_false_failure(self):
        r=copy.deepcopy(self.report);r['games'][0]['engines'][0]['stderr']['observed_bytes']=7
        self.assertTrue(self.audit(r)['complete'])
        r['games'][0]['engines'][0]['stderr']['truncated']=True
        self.assertFalse(self.audit(r)['complete'])

    def test_hash_config_limits_colors_identity_rejected(self):
        mutations=[lambda r:r['config'].update(max_plies=1),
                   lambda r:r['engine_identities'][0]['executable'].update(sha256='wrong'),
                   lambda r:r['games'][0].update(engines_by_color=[1,0]),
                   lambda r:next(e for e in r['games'][0]['transcript']['entries'] if e['direction']=='command' and e['text'].startswith('go')).update(text='go depth 2')]
        for mutate in mutations:
            r=copy.deepcopy(self.report);mutate(r)
            with self.assertRaises(ValueError): self.audit(r)

    def test_streams_fresh_reproducible_and_order_independent(self):
        roots=[{'id':str(i),'phase':p,'moves':[]} for p in ['discovery','confirmation'] for i in (range(10,20) if p=='discovery' else range(20,30))]
        x=a.draw_root(roots,'discovery','a',3)
        self.assertEqual(x,a.draw_root(roots,'discovery','a',3))
        self.assertIn(int(x['id']),range(10,20))
        self.assertIn(int(a.draw_root(roots,'confirmation','a',3)['id']),range(20,30))

    def test_custom_start_rejected(self):
        with tempfile.TemporaryDirectory() as folder:
            archive=Path(folder)/'book.zip'
            with zipfile.ZipFile(archive,'w') as z:
                z.writestr('8moves_v3.pgn','[SetUp "1"]\n[FEN "8/8/8/8/8/8/K7/7k w - - 0 1"]\n\n*\n')
            result=a.prepare_book(archive,Path(folder)/'roots.gz')
            self.assertEqual(result['roots'],0)
            self.assertEqual(result['excluded'],{'nonstandard_start':1})

    def test_exited_leader_with_descendant_pipe_is_cleaned(self):
        child=subprocess.Popen([sys.executable,'-c',
            "import subprocess,sys; subprocess.Popen([sys.executable,'-c','import time; time.sleep(60)'])"],
            stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
        try:
            child.wait(timeout=3)
            self.assertTrue(a.group_alive(child.pid))
            _,settled=a.terminate(child,time.monotonic()+3)
            self.assertTrue(settled)
            self.assertFalse(a.group_alive(child.pid))
        finally:
            if a.group_alive(child.pid):a.terminate(child,time.monotonic()+3)

    def test_excluded_reference_cannot_run(self):
        with self.assertRaisesRegex(ValueError,'preflight'):
            a.verify_build({'references':{'bad':{'eligible':False}}},'bad')

    def test_unsettled_and_incomplete_runs_prevent_launch(self):
        with self.assertRaisesRegex(ValueError,'Unreconciled'):
            a.invoke({'inflight':{'process_group':123}},'unused','discovery','test')
        with self.assertRaisesRegex(ValueError,'incomplete'):
            a.invoke({'inflight':None,'runs':[{'phase':'discovery','complete':False}]},'unused','discovery','test')

    def test_actual_analysis_calls_upstream_bounds_with_array(self):
        ledger={'catalog':{'x':{'rating':2000}},'runs':[{'opponent':'x','phase':'discovery','complete':True,'pair_score':.25}]}
        result=a.analysis(ledger)['x/discovery']
        self.assertEqual(result['pairs'],1)
        self.assertEqual(result['supplemental_anytime_score_bounds'],[0.,1.])

    def test_posterior_uncertainty_and_symmetry(self):
        mixed=a.posterior([0,.25,.5,.75,1]*4,2000)
        self.assertAlmostEqual(mixed['score_mean'],.5,delta=.005)
        lo,hi=mixed['index_credible95'];self.assertLess(lo,2000);self.assertGreater(hi,2000)
        zero=a.posterior([0]*8,2000);one=a.posterior([1]*8,2000)
        self.assertGreater(zero['score_credible95'][0],0);self.assertLess(one['score_credible95'][1],1)
        self.assertAlmostEqual(zero['index_median']+one['index_median'],4000,delta=5)
        self.assertLess(a.posterior([.5]*40,2000)['index_credible95'][1]-a.posterior([.5]*40,2000)['index_credible95'][0],hi-lo)

if __name__=='__main__':unittest.main()
