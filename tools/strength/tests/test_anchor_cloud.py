"""Offline lifecycle checks. Never submit a real cloud job."""
import json
import tempfile
import time
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock, patch

from tools.strength import anchor_cloud as controller


class CloudControllerTests(unittest.TestCase):
    def prepare(self, root):
        engine = {'path': '/opt/engine', 'sha256': 'a' * 64, 'source': 'frozen', 'args': [], 'options': []}
        manifest = {'schema': 1, 'experiment': 'msh185-test',
                    'campaign_started_epoch': time.time(), 'estimated_compute_before_usd': 0,
                    'build': {'engines': {'candidate': engine, 'runner': engine, 'other': engine}},
                    'bins': [[{'id': 'game-1', 'pair_id': 'pair-1', 'opponent': 'other',
                               'swapped': False, 'opening': {'id': 'o', 'moves': ['e2e4'] * 16}}]]}
        manifest['deadline_epoch'] = manifest['campaign_started_epoch'] + 21600
        config = {'project': 'mshogi', 'region': 'us-central1', 'machine_type': 'c3-standard-4',
                  'parallelism': 8, 'max_retries': 0, 'vm_hourly_usd': .2016,
                  'service_account': 'worker@mshogi.iam.gserviceaccount.com',
                  'store_uri': 'gs://mshogi-test-bucket/test',
                  'notification_topic': 'projects/mshogi/topics/completion'}
        for name, data in (('manifest', manifest), ('config', config)):
            (root/name).write_text(json.dumps(data))
        return SimpleNamespace(manifest=str(root/'manifest'), config=str(root/'config'),
                               job='msh185-test-job', out=str(root/'out'),
                               image='registry/image@sha256:' + 'a'*64,
                               subscription='projects/mshogi/subscriptions/completion')

    def run_fake(self, args, store, submit_failure=False, cleanup_failure=False):
        events = []
        def cloud(command):
            events.append(command[:3])
            if command[:3] == ['batch', 'jobs', 'submit']:
                if submit_failure:
                    raise TimeoutError('uncertain submit')
                return {'name': 'jobs/msh185-test-job'}
            return []
        def cleanup(*arguments):
            events.append(['cleanup'])
            if cleanup_failure:
                raise RuntimeError('cannot confirm cleanup')
        with patch.object(controller, 'cloud', side_effect=cloud), \
             patch.object(controller, 'cleanup', side_effect=cleanup) as cleanup_mock, \
             patch.object(controller.gcp, 'GCSStore', return_value=store), \
             patch.object(controller.gcp, 'wait_for_completion', return_value={'job': {'status': 'SUCCEEDED'}}):
            try:
                controller.run(args)
            except RuntimeError:
                pass
        return events, cleanup_mock

    def test_storage_list_failure_cannot_bypass_cleanup_or_resources(self):
        with tempfile.TemporaryDirectory() as directory:
            args = self.prepare(Path(directory))
            store = Mock()
            store.put.return_value = True
            store.list.side_effect = RuntimeError('storage unavailable')
            events, cleanup = self.run_fake(args, store)
            cleanup.assert_called_once()
            self.assertTrue((Path(args.out)/'retrieval-error.json').exists())
            resources = json.loads((Path(args.out)/'resources.json').read_text())
            self.assertIsNotNone(resources['conservative_compute_usd_upper_bound'])
            self.assertIn(['cleanup'], events)

    def test_unknown_submission_still_attempts_narrow_cleanup(self):
        with tempfile.TemporaryDirectory() as directory:
            args = self.prepare(Path(directory))
            store = Mock(); store.put.return_value = True; store.list.return_value = []
            _, cleanup = self.run_fake(args, store, submit_failure=True)
            cleanup.assert_called_once()
            self.assertFalse(cleanup.call_args.args[-1])
            self.assertTrue((Path(args.out)/'intent.json').exists())
            self.assertTrue((Path(args.out)/'failure.json').exists())

    def test_cleanup_failure_has_no_false_compute_upper_bound(self):
        with tempfile.TemporaryDirectory() as directory:
            args = self.prepare(Path(directory))
            store = Mock(); store.put.return_value = True; store.list.return_value = []
            self.run_fake(args, store, cleanup_failure=True)
            resources = json.loads((Path(args.out)/'resources.json').read_text())
            self.assertIsNone(resources['conservative_compute_usd_upper_bound'])
            self.assertTrue(resources['cleanup_error'])

    def test_nan_budget_rejected_before_cloud_access(self):
        with tempfile.TemporaryDirectory() as directory:
            args = self.prepare(Path(directory))
            manifest = json.loads(Path(args.manifest).read_text())
            manifest['estimated_compute_before_usd'] = float('nan')
            Path(args.manifest).write_text(json.dumps(manifest))
            with patch.object(controller, 'cloud') as cloud, self.assertRaises(ValueError):
                controller.run(args)
            cloud.assert_not_called()

    def test_cleanup_refuses_another_owners_job(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            with patch.object(controller, 'jobs_named', return_value=[{'labels': {'msh185-owner': 'other'}}]), \
                 patch.object(controller, 'cloud') as cloud, self.assertRaises(RuntimeError):
                controller.cleanup('msh185-test', 'us-central1', 'mine', out, True)
            cloud.assert_not_called()
            self.assertFalse(json.loads((out/'cleanup.json').read_text())['verified_absent'])

    def test_job_selection_uses_name_and_exact_basename(self):
        with patch.object(controller, 'cloud', return_value=[
                {'name': 'projects/123/locations/us-central1/jobs/msh185-test'},
                {'name': 'projects/123/locations/us-central1/jobs/msh185-test-extra'}]) as cloud:
            jobs = controller.jobs_named('msh185-test', 'us-central1')
        self.assertEqual(len(jobs), 1)
        self.assertIn('--filter=name:msh185-test', cloud.call_args.args[0])

    def test_cleanup_reserve_prevents_late_submission(self):
        with tempfile.TemporaryDirectory() as directory:
            args = self.prepare(Path(directory))
            manifest = json.loads(Path(args.manifest).read_text())
            manifest['campaign_started_epoch'] = time.time() - 21500
            manifest['deadline_epoch'] = manifest['campaign_started_epoch'] + 21600
            Path(args.manifest).write_text(json.dumps(manifest))
            with patch.object(controller, 'cloud') as cloud, self.assertRaises(ValueError):
                controller.run(args)
            cloud.assert_not_called()

    def test_cleanup_checks_disks_and_retains_owned_job(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            owned = {'name': 'jobs/msh185-test', 'uid': 'unique', 'labels': {'msh185-owner': 'mine'}}
            with patch.object(controller, 'jobs_named', side_effect=[[owned], []]), \
                 patch.object(controller, 'cloud', return_value=[]) as cloud:
                controller.cleanup('msh185-test', 'us-central1', 'mine', out, True)
            result = json.loads((out/'cleanup.json').read_text())
            self.assertTrue(result['verified_absent'])
            self.assertEqual(result['owned_job']['uid'], 'unique')
            self.assertTrue(any(call.args[0][:3] == ['compute', 'disks', 'list'] for call in cloud.call_args_list))

    def test_unknown_submission_absence_is_not_claimed_verified(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            with patch.object(controller, 'jobs_named', return_value=[]), \
                 patch.object(controller, 'cloud', return_value=[]), self.assertRaises(RuntimeError):
                controller.cleanup('msh185-test', 'us-central1', 'mine', out, False)
            self.assertFalse(json.loads((out/'cleanup.json').read_text())['verified_absent'])


if __name__ == '__main__':
    unittest.main()
