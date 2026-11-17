"""Offline cloud policy/evidence tests; no credentials or jobs required."""
import copy
import json
import tempfile
import time
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock, patch

from tools.strength import shogi_cloud as cloud, anchor_cloud
from tools.experiments.core import encode, digest


def manifest():
    engine = {'path': '/opt/engines/engine', 'sha256': 'a'*64, 'source': 'b'*40}
    build = {'source': 'b'*40, 'engines': {name: dict(engine, path='/opt/engines/'+name)
              for name in ('candidate','baseline','runner','config_writer')}}
    build['engines']['baseline']['source'] = cloud.PREDECESSOR
    openings = {'openings': [{'id': str(i), 'sfen': None, 'moves':['7g7f']} for i in range(6)]}
    start = time.time()
    return {'schema':1,'experiment':'msh187-test','campaign_started_epoch':start,
            'deadline_epoch':start+3600,'estimated_compute_before_usd':0,
            'task_seconds':900,'build':build,'openings':openings,'openings_sha256':digest(encode(openings)),
            'bins':[[1],[2]], 'protocol':{'base_ms':5000,'increment_ms':200,'elo0':0,'elo1':15,
                'alpha':.05,'beta':.05,'min_pairs':30,'max_pairs':600}}


class ManifestTests(unittest.TestCase):
    def test_fixed_protocol_and_global_deadline(self):
        m=manifest(); cloud.validate_manifest(m)
        for key,value in [('base_ms',1000),('min_pairs',1),('max_pairs',601)]:
            bad=copy.deepcopy(m);bad['protocol'][key]=value
            with self.assertRaises(ValueError):cloud.validate_manifest(bad)
        bad=copy.deepcopy(m);bad['deadline_epoch']+=1
        with self.assertRaises(ValueError):cloud.validate_manifest(bad)

    def test_duplicate_gap_boolean_or_excessive_bin_rejected(self):
        for bins in ([[1],[1]],[[1],[3]],[[True]],[[1,2,3,4]],[[601]]):
            m=manifest();m['bins']=bins
            with self.assertRaises(ValueError):cloud.validate_manifest(m)

    def test_hash_and_predecessor_bound(self):
        m=manifest();m['openings']['openings'][0]['moves'].append('3c3d')
        with self.assertRaises(ValueError):cloud.validate_manifest(m)
        m=manifest();m['build']['engines']['baseline']['source']='c'*40
        with self.assertRaises(ValueError):cloud.validate_manifest(m)

    def test_native_pair_dispatch_has_symmetric_limits(self):
        args=cloud.pair_args(manifest(),Path('/openings.json'))
        self.assertEqual((args.base,args.increment,args.max_steps,args.pair_wall_ms),(5,.2,320,180000))
        self.assertEqual(args.candidate_nodes,args.baseline_nodes)
        self.assertEqual(args.baseline_depth,64)
        self.assertEqual(args.watchdog_ms,60000)

    def test_failed_execution_retains_receipt_and_diagnostics(self):
        with tempfile.TemporaryDirectory() as temp:
            dest=Path(temp)/'pair';m=manifest()
            def fail(args,seed,out):
                (out/'pair.log').write_text('retained diagnostic')
                raise RuntimeError('native replay failed')
            with patch.object(cloud,'file_hash',return_value='a'*64), \
                 patch.object(cloud.native_match,'run_pair',side_effect=fail):
                result=cloud.execute_pair(m,1,dest)
            self.assertFalse(result['complete']);self.assertIsNone(result['score'])
            self.assertIn('native replay failed',result['error'])
            self.assertTrue((dest/'receipt.json').exists())
            self.assertIn('pair.log',result['artifacts'])


class WorkerTests(unittest.TestCase):
    def test_duplicate_task_claim_prevents_execution(self):
        m=manifest();source=Mock();source.get.return_value=encode(m)
        store=Mock();store.put.return_value=False
        with patch.object(cloud.gcp,'GCSStore',side_effect=[source,store]), \
             patch.dict(cloud.os.environ,{'BATCH_TASK_INDEX':'0','BATCH_TASK_RETRY_ATTEMPT':'0'}), \
             patch.object(cloud,'execute_pair') as run, self.assertRaises(ValueError):
            cloud.worker('gs://test-bucket/manifest','gs://test-bucket/run')
        run.assert_not_called()

    def test_abort_marker_stops_before_new_pair_and_retains_task_receipt(self):
        m=manifest();source=Mock();source.get.return_value=encode(m)
        store=Mock();store.put.return_value=True;store.exists.return_value=True
        with patch.object(cloud.gcp,'GCSStore',side_effect=[source,store]), \
             patch.dict(cloud.os.environ,{'BATCH_TASK_INDEX':'0','BATCH_TASK_RETRY_ATTEMPT':'0'}), \
             patch.object(cloud,'cloud_environment',return_value={'gcp':{'cpu-platform':'Intel Sapphire Rapids'}}), \
             patch.object(cloud,'execute_pair') as run, self.assertRaises(ValueError):
            cloud.worker('gs://test-bucket/manifest','gs://test-bucket/run')
        run.assert_not_called()
        keys=[c.args[0] for c in store.put.call_args_list]
        self.assertIn('tasks/0/receipt.json',keys)
        self.assertNotIn('pairs/0001/launch.json',keys)

    def test_retry_attempt_refused(self):
        source=Mock();source.get.return_value=encode(manifest())
        with patch.object(cloud.gcp,'GCSStore',return_value=source), \
             patch.dict(cloud.os.environ,{'BATCH_TASK_INDEX':'0','BATCH_TASK_RETRY_ATTEMPT':'1'}), \
             patch.object(cloud,'execute_pair') as run, self.assertRaises(ValueError):
            cloud.worker('gs://test-bucket/manifest','gs://test-bucket/run')
        run.assert_not_called()


class ControllerTests(unittest.TestCase):
    def test_reused_controller_sets_shogi_module_policy_and_owned_label(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp);m=manifest()
            config={'project':'mshogi','region':'us-central1','machine_type':'c3-standard-4',
                    'parallelism':8,'max_retries':0,'vm_hourly_usd':cloud.RATE,
                    'service_account':'worker@mshogi.iam.gserviceaccount.com',
                    'store_uri':'gs://test-bucket/run','notification_topic':'projects/mshogi/topics/events'}
            for name,value in [('manifest',m),('config',config)]: (root/name).write_bytes(encode(value))
            args=SimpleNamespace(manifest=str(root/'manifest'),config=str(root/'config'),job='msh187-test',
                    out=str(root/'out'),image='registry/image@sha256:'+'a'*64,
                    subscription='projects/mshogi/subscriptions/completion')
            store=Mock();store.put.return_value=True;store.list.return_value=[]
            with patch.object(anchor_cloud,'cloud',return_value=[]), \
                 patch.object(anchor_cloud,'cleanup') as cleanup, \
                 patch.object(anchor_cloud.gcp,'GCSStore',return_value=store), \
                 patch.object(anchor_cloud.gcp,'wait_for_completion',return_value={}):
                anchor_cloud.run(args,policy=cloud.POLICY,manifest_validator=cloud.validate_manifest)
            request=json.loads((root/'out/batch-request.json').read_text())
            commands=request['taskGroups'][0]['taskSpec']['runnables'][0]['container']['commands']
            self.assertEqual(commands[:3],['-m','tools.strength.shogi_cloud','worker'])
            self.assertIn('msh187-owner',request['labels'])
            self.assertEqual(cleanup.call_args.kwargs['owner_label'],'msh187-owner')
            resource=json.loads((root/'out/resources.json').read_text())
            self.assertLess(resource['conservative_compute_usd_upper_bound'],2)

    def test_prior_failure_or_decision_prevents_cloud_submission(self):
        with tempfile.TemporaryDirectory() as temp:
            path=Path(temp)/'m';path.write_bytes(encode(manifest()))
            args=SimpleNamespace(manifest=str(path),previous=[],replay_runner="runner")
            for prior in ({'failures':['failure'],'sprt':{'decision':None}},
                          {'failures':[],'sprt':{'decision':'H1'}}):
                with patch.object(cloud,'summarize_runs',return_value=prior), \
                     patch.object(anchor_cloud,'run') as run,self.assertRaises(ValueError):
                    cloud.submit(args)
                run.assert_not_called()

    def test_empty_summary_has_no_decision(self):
        result=cloud.summarize_runs([])
        self.assertEqual(result['pairs_attempted'],0)
        self.assertIsNone(result['sprt']['decision'])
        self.assertEqual(result['conservative_compute_usd_upper_bound'],0)


if __name__=='__main__':unittest.main()


def native_report(m, seed):
    e=m['build']['engines'];index=(seed-1)%6;opening=m['openings']['openings'][index]
    cfg={'seed':seed,'clock':{'base_ms':5000,'increment_ms':200},'max_steps':320,
         'wall_budget_ms':180000,'rounds':1,
         'players':[{'executable':e[name]['path'],'limits':[{'profile':'mshogi.shogi.csa36.v1',
                       'work':{'depth':64,'nodes':5000000}}]} for name in ('candidate','baseline')],
         'openings':[{'id':opening['id'],'provenance':f"Frozen catalog SHA256 {m['openings_sha256']}; index {index}; source",
             'root':{'chess_match':None,'record':{'rules_profile':'mshogi.shogi.csa36.v1','game_kind':'shogi',
               'payload':{'root':{'category':'standard','known_history':'initial','counter_source':'initial_fixed',
                   'sfen':'lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1'},
                   'events':[{'command':{'kind':'play','action':move}} for move in opening['moves']]}}}}]}
    return {'config':cfg,'environment':{'harness_revision':m['build']['source'],'harness_dirty_sha256':digest(b''),
                'os':'linux','arch':'x86_64'},
            'identities':[{'executable':{'sha256':e[name]['sha256']}} for name in ('candidate','baseline')],
            'games':[{'pairing':{'swapped':swapped},'cleanup':[],'score_first_twice':2,'steps':[],
                      'stop':'native_concluded','native_outcome':{'fixture':True}} for swapped in (False,True)]}


class EvidenceTests(unittest.TestCase):
    """Native subprocess is mocked; these test provenance, not game rules."""
    def create_batch(self, root):
        import gzip
        m=manifest()
        (root/'manifest.json').write_bytes(encode(m))
        start=m['campaign_started_epoch'];end=start+100
        (root/'intent.json').write_bytes(encode({'started_epoch':start,'manifest_sha256':digest(encode(m))}))
        (root/'cleanup.json').write_bytes(encode({'verified_absent':True,'at_epoch':end}))
        (root/'resources.json').write_bytes(encode({'started_epoch':start,'allocation_end_epoch':end,
            'ended_epoch':end,'cleanup_error':None,'rate_usd_vm_hour':cloud.RATE,
            'conservative_vm_hours_upper_bound':8*100/3600,
            'conservative_compute_usd_upper_bound':8*100/3600*cloud.RATE}))
        for index,assigned in enumerate(m['bins']):
            claim={'manifest_sha256':digest(encode(m)),'task_index':index,'started_epoch':m['campaign_started_epoch']}
            task=root/'raw/tasks'/str(index);task.mkdir(parents=True)
            (task/'started.json').write_bytes(encode(claim))
            (task/'receipt.json').write_bytes(encode(claim|{'environment':{'gcp':{'cpu-platform':'Intel Sapphire Rapids','machine-type':'projects/p/machineTypes/c3-standard-4','zone':'projects/p/zones/us-central1-a','id':str(index)}},'complete':True,'pairs':[{'seed':n,'complete':True} for n in assigned]}))
            for n in assigned:
                dest=root/'raw/pairs'/f'{n:04d}';dest.mkdir(parents=True)
                r=native_report(m,n);raw=encode(r)
                summary={'report':f'pair-{n:04d}.report.json.gz','report_sha256':digest(raw)}
                (dest/'launch.json').write_bytes(encode(claim|{'seed':n}))
                (dest/f'pair-{n:04d}.config.json').write_bytes(encode(r['config']))
                (dest/f'pair-{n:04d}.report.json.gz').write_bytes(gzip.compress(raw))
                (dest/f'pair-{n:04d}.replay.log').write_text('replayed 2 native experiment games\n')
                (dest/f'pair-{n:04d}.summary.json').write_bytes(encode(summary))
                (dest/'openings.json').write_bytes(encode(m['openings']))
                artifacts={p.name:{'sha256':digest(p.read_bytes()),'bytes':p.stat().st_size}
                           for p in dest.iterdir() if p.name!='launch.json'}
                (dest/'receipt.json').write_bytes(encode({'seed':n,'manifest_sha256':digest(encode(m)),
                    'task_index':index,'started_epoch':claim['started_epoch']+1,'ended_epoch':claim['started_epoch']+2,
                    'complete':True,'score':.5,'summary':summary,'artifacts':artifacts}))
        return m

    def test_healthy_two_task_batch_replays_and_scores(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp);self.create_batch(root)
            with patch.object(cloud.subprocess,'run') as replay:
                result=cloud.summarize_runs([root],'native-runner')
            self.assertEqual(replay.call_count,2)
            self.assertEqual(result['pentanomial'],[0,0,2,0,0])
            self.assertEqual(result['wins_draws_losses'],[2,0,2])
            self.assertEqual(result['failures'],[])

    def test_wrong_launch_or_task_claim_rejected(self):
        for relative,field,value in [('raw/pairs/0001/launch.json','manifest_sha256','f'*64),
                                     ('raw/pairs/0001/launch.json','task_index',1),
                                     ('raw/tasks/0/receipt.json','task_index',1)]:
            with tempfile.TemporaryDirectory() as temp:
                root=Path(temp);self.create_batch(root);p=root/relative
                changed=json.loads(p.read_text());changed[field]=value;p.write_bytes(encode(changed))
                with patch.object(cloud.subprocess,'run'),self.assertRaises(ValueError):
                    cloud.summarize_runs([root],'runner')

    def test_missing_task_seed_and_worker_replay_receipt_rejected(self):
        for kind in ('seed','replay'):
            with tempfile.TemporaryDirectory() as temp:
                root=Path(temp);self.create_batch(root)
                p=root/('raw/tasks/0/receipt.json' if kind=='seed' else 'raw/pairs/0001/receipt.json')
                changed=json.loads(p.read_text())
                if kind=='seed':changed['pairs']=[]
                else:del changed['artifacts']['pair-0001.replay.log']
                p.write_bytes(encode(changed))
                with patch.object(cloud.subprocess,'run'),self.assertRaises(ValueError):
                    cloud.summarize_runs([root],'runner')

    def test_same_id_alternate_prefix_rejected(self):
        m=manifest();r=native_report(m,1)
        r['config']['openings'][0]['root']['record']['payload']['events'][0]['command']['action']='2g2f'
        with self.assertRaisesRegex(ValueError,'prefix'):
            cloud.audit_report(r,r['config'],m,1)

    def test_native_replay_failure_blocks_aggregation(self):
        import subprocess
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp);self.create_batch(root)
            with patch.object(cloud.subprocess,'run',side_effect=subprocess.CalledProcessError(1,'runner')), \
                 self.assertRaises(subprocess.CalledProcessError):
                cloud.summarize_runs([root],'runner')

    def test_mixed_or_understated_resource_receipt_rejected(self):
        for field,value in [('started_epoch',0),('allocation_end_epoch',0),('rate_usd_vm_hour',.01),
                            ('conservative_compute_usd_upper_bound',0),('conservative_vm_hours_upper_bound',0),
                            ('cleanup_error','uncertain')]:
            with tempfile.TemporaryDirectory() as temp:
                root=Path(temp);self.create_batch(root);p=root/'resources.json'
                changed=json.loads(p.read_text());changed[field]=value;p.write_bytes(encode(changed))
                with patch.object(cloud.subprocess,'run'),self.assertRaises(ValueError):
                    cloud.summarize_runs([root],'runner')
