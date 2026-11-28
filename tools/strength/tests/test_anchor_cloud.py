"""Offline lifecycle checks. Never submit a real cloud job."""
import json
import tempfile
import time
import unittest
from contextlib import contextmanager
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock, patch

from tools.strength import anchor_cloud as controller, batch

POLICY = controller.CampaignPolicy('test', 21600, 15, 'tools.strength.test_worker', ('worker',))


def accept(manifest):
    """Manifest content is each campaign's own check; the controller owns bounds."""


class CloudControllerTests(unittest.TestCase):
    def prepare(self, root):
        manifest = {'campaign_started_epoch': time.time(), 'estimated_compute_before_usd': 0,
                    'bins': [['work']]}
        manifest['deadline_epoch'] = manifest['campaign_started_epoch'] + 21600
        config = {'project': 'mshogi', 'region': 'us-central1', 'machine_type': 'c3-standard-4',
                  'parallelism': 8, 'max_retries': 0, 'vm_hourly_usd': .2016,
                  'service_account': 'worker@mshogi.iam.gserviceaccount.com',
                  'store_uri': 'gs://mshogi-test-bucket/test',
                  'notification_topic': 'projects/mshogi/topics/completion'}
        for name, data in (('manifest', manifest), ('config', config)):
            (root/name).write_text(json.dumps(data))
        return SimpleNamespace(manifest=str(root/'manifest'), config=str(root/'config'),
                               job='test-run-job', out=str(root/'out'),
                               image='registry/image@sha256:' + 'a'*64,
                               subscription='projects/mshogi/subscriptions/completion')

    def run_fake(self, args, store, submit_failure=False, cleanup_failure=False, **options):
        events = []
        def cloud(command):
            events.append(command[:3])
            if command[:3] == ['batch', 'jobs', 'submit']:
                if submit_failure:
                    raise TimeoutError('uncertain submit')
                return {'name': 'jobs/test-run-job'}
            return []
        def cleanup(*arguments, **kwargs):
            events.append(['cleanup'])
            if cleanup_failure:
                raise RuntimeError('cannot confirm cleanup')
        with patch.object(controller, 'cloud', side_effect=cloud), \
             patch.object(controller, 'cleanup', side_effect=cleanup) as cleanup_mock, \
             patch.object(controller.gcp, 'GCSStore', return_value=store), \
             patch.object(controller.gcp, 'wait_for_completion', return_value={'job': {'status': 'SUCCEEDED'}}):
            try:
                controller.run(args, **(dict(policy=POLICY, manifest_validator=accept) | options))
            except RuntimeError:
                pass
        return events, cleanup_mock

    def test_policy_selects_worker_command_and_owner_label(self):
        with tempfile.TemporaryDirectory() as directory:
            args = self.prepare(Path(directory))
            store = Mock(); store.put.return_value = True; store.list.return_value = []
            _, cleanup = self.run_fake(args, store)
            request = json.loads((Path(args.out)/'batch-request.json').read_text())
            commands = request['taskGroups'][0]['taskSpec']['runnables'][0]['container']['commands']
            self.assertEqual(commands[:3], ['-m', 'tools.strength.test_worker', 'worker'])
            self.assertIn('test-owner', request['labels'])
            self.assertEqual(cleanup.call_args.kwargs['owner_label'], 'test-owner')

    def test_match_projects_batch_runtime_and_deadline_beyond_thirty_minutes(self):
        with tempfile.TemporaryDirectory() as directory:
            args = self.prepare(Path(directory))
            manifest = json.loads(Path(args.manifest).read_text())
            manifest.pop('deadline_epoch')
            command = ['match', '--game', 'tenjiku', '--profile', 'adams', '--candidate', './new',
                       '--baseline', './old', '--base', '60', '--increment', '1.5',
                       '--max-steps', '768', '--max-pairs', '2', '--workers', '2', '--out', 'adams']
            manifest.update(schema=1, files={}, bins=[[command], [command, command]])
            Path(args.manifest).write_text(json.dumps(manifest))
            store = Mock()
            store.put.return_value = True
            store.list.return_value = []
            args.job = 'strength-long-clock'
            self.run_fake(args, store, policy=batch.POLICY, manifest_validator=batch.validate,
                          project_runtime=batch.projected_runtime)
            request = json.loads((Path(args.out)/'batch-request.json').read_text())
            runtime = request['taskGroups'][0]['taskSpec']['maxRunDuration']
            self.assertEqual(runtime, '6650s')
            resolved = json.loads((Path(args.out)/'manifest.json').read_text())
            intent = json.loads((Path(args.out)/'intent.json').read_text())
            self.assertEqual(resolved['deadline_epoch'] - intent['started_epoch'], 7070)
            self.assertEqual(resolved['task_runtime_seconds'], 6650)

    def test_explicit_batch_runtime_can_extend_match_projection(self):
        with tempfile.TemporaryDirectory() as directory:
            args = self.prepare(Path(directory))
            manifest = json.loads(Path(args.manifest).read_text())
            manifest.pop('deadline_epoch')
            command = ['selfplay', '--game', 'chess', '--engine', './engine', '--candidate-nodes', '10',
                       '--max-pairs', '1', '--out', 'selfplay']
            manifest.update(schema=1, files={}, bins=[[command]])
            Path(args.manifest).write_text(json.dumps(manifest))
            config = json.loads(Path(args.config).read_text())
            config['max_run_seconds'] = 3600
            Path(args.config).write_text(json.dumps(config))
            store = Mock()
            store.put.return_value = True
            store.list.return_value = []
            args.job = 'strength-long-config'
            self.run_fake(args, store, policy=batch.POLICY, manifest_validator=batch.validate,
                          project_runtime=batch.projected_runtime)
            request = json.loads((Path(args.out)/'batch-request.json').read_text())
            self.assertEqual(request['taskGroups'][0]['taskSpec']['maxRunDuration'], '3600s')

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
                controller.run(args, policy=POLICY, manifest_validator=accept)
            cloud.assert_not_called()

    def test_cleanup_refuses_another_owners_job(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            with patch.object(controller, 'jobs_named', return_value=[{'labels': {'test-owner': 'other'}}]), \
                 patch.object(controller, 'cloud') as cloud, self.assertRaises(RuntimeError):
                controller.cleanup('test-run', 'us-central1', 'mine', out, True,
                                   owner_label=POLICY.owner_label)
            cloud.assert_not_called()
            self.assertFalse(json.loads((out/'cleanup.json').read_text())['verified_absent'])

    def test_job_selection_uses_name_and_exact_basename(self):
        with patch.object(controller, 'cloud', return_value=[
                {'name': 'projects/123/locations/us-central1/jobs/test-run'},
                {'name': 'projects/123/locations/us-central1/jobs/test-run-extra'}]) as cloud:
            jobs = controller.jobs_named('test-run', 'us-central1')
        self.assertEqual(len(jobs), 1)
        self.assertIn('--filter=name:test-run', cloud.call_args.args[0])

    def test_cleanup_reserve_prevents_late_submission(self):
        with tempfile.TemporaryDirectory() as directory:
            args = self.prepare(Path(directory))
            manifest = json.loads(Path(args.manifest).read_text())
            manifest['campaign_started_epoch'] = time.time() - 21500
            manifest['deadline_epoch'] = manifest['campaign_started_epoch'] + 21600
            Path(args.manifest).write_text(json.dumps(manifest))
            with patch.object(controller, 'cloud') as cloud, self.assertRaises(ValueError):
                controller.run(args, policy=POLICY, manifest_validator=accept)
            cloud.assert_not_called()

    def test_cleanup_checks_disks_and_retains_owned_job(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            owned = {'name': 'jobs/test-run', 'uid': 'unique', 'labels': {'test-owner': 'mine'}}
            with patch.object(controller, 'jobs_named', side_effect=[[owned], []]), \
                 patch.object(controller, 'cloud', return_value=[]) as cloud:
                controller.cleanup('test-run', 'us-central1', 'mine', out, True,
                                   owner_label=POLICY.owner_label)
            result = json.loads((out/'cleanup.json').read_text())
            self.assertTrue(result['verified_absent'])
            self.assertEqual(result['owned_job']['uid'], 'unique')
            self.assertTrue(any(call.args[0][:3] == ['compute', 'disks', 'list'] for call in cloud.call_args_list))

    def test_unknown_submission_absence_is_not_claimed_verified(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory)
            with patch.object(controller, 'jobs_named', return_value=[]), \
                 patch.object(controller, 'cloud', return_value=[]), self.assertRaises(RuntimeError):
                controller.cleanup('test-run', 'us-central1', 'mine', out, False,
                                   owner_label=POLICY.owner_label)
            self.assertFalse(json.loads((out/'cleanup.json').read_text())['verified_absent'])

    def test_cleanup_waits_beyond_seven_polls_for_job_metadata(self):
        with tempfile.TemporaryDirectory() as directory:
            now = [0.0]
            owned = {'labels': {'test-owner': 'mine'}}
            def sleep(seconds):
                now[0] += seconds
            with patch.object(controller, 'jobs_named', side_effect=[[owned]] + [[owned]]*9 + [[]]), \
                 patch.object(controller, 'cloud', return_value=[]), \
                 patch.object(controller.time, 'monotonic', side_effect=lambda: now[0]), \
                 patch.object(controller.time, 'sleep', side_effect=sleep):
                controller.cleanup('test-run', 'us-central1', 'mine', Path(directory), True,
                                   owner_label=POLICY.owner_label)
            result = json.loads((Path(directory)/'cleanup.json').read_text())
            self.assertTrue(result['verified_absent'])
            self.assertEqual(result['polls'], 10)
            self.assertEqual(now[0], 45)

    def test_cleanup_poll_window_stops_at_campaign_deadline(self):
        with tempfile.TemporaryDirectory() as directory:
            now = [0.0]
            owned = {'labels': {'test-owner': 'mine'}}
            def sleep(seconds):
                now[0] += seconds
            with patch.object(controller, 'jobs_named', return_value=[owned]), \
                 patch.object(controller, 'cloud', return_value=[]) as cloud, \
                 patch.object(controller.time, 'monotonic', side_effect=lambda: now[0]), \
                 patch.object(controller.time, 'time', side_effect=lambda: 1000+now[0]), \
                 patch.object(controller.time, 'sleep', side_effect=sleep), self.assertRaises(TimeoutError):
                controller.cleanup('test-run', 'us-central1', 'mine', Path(directory), True,
                                   owner_label=POLICY.owner_label, campaign_deadline=1012)
            result = json.loads((Path(directory)/'cleanup.json').read_text())
            self.assertFalse(result['verified_absent'])
            self.assertEqual(now[0], 12)
            self.assertEqual(result['poll_allowance_seconds'], 12)


class WorkerHelperTests(unittest.TestCase):
    def test_worker_starts_upload_before_the_shorter_batch_task_deadline(self):
        receipt = {}
        command = ['bench', '--source', 'input', '--out', 'out']
        manifest = {'task_runtime_seconds': 600, 'deadline_epoch': 8_200,
                    'files': {}, 'bins': [[command]]}

        @contextmanager
        def worker_task(*args):
            yield manifest, Mock(), receipt

        child = Mock()
        child.wait.return_value = 0
        args = SimpleNamespace(manifest='manifest', store='store')
        with patch.object(controller, 'worker_task', side_effect=worker_task), \
             patch.object(controller, 'put_new'), \
             patch.object(batch.time, 'time', return_value=1_000), \
             patch.object(batch.time, 'monotonic', side_effect=[100, 110]), \
             patch.object(batch.subprocess, 'Popen', return_value=child):
            receipt['task_index'] = 0
            batch.worker(args)
        child.wait.assert_called_once_with(timeout=500)
        self.assertEqual(receipt['commands'], [{'argv': command, 'exit': 0}])

    def test_cloud_region_checked_from_observed_zone(self):
        class Reply:
            def __init__(self, value):
                self.value = value
            def __enter__(self):
                return self
            def __exit__(self, *args):
                pass
            def read(self, bound):
                return self.value.encode()
        for zone, accepted in (('projects/123/zones/us-central1-a', True),
                               ('projects/123/zones/us-east1-a', False)):
            values = ['projects/123/machineTypes/c3-standard-4', zone, '123', 'Intel Sapphire Rapids']
            with patch.object(controller, 'environment',
                              return_value={'architecture': 'x86_64', 'platform': 'Linux'}), \
                 patch.object(controller.urllib.request, 'urlopen', side_effect=[Reply(v) for v in values]):
                if accepted:
                    self.assertEqual(controller.cloud_environment()['gcp']['zone'], zone)
                else:
                    with self.assertRaisesRegex(ValueError, 'region'):
                        controller.cloud_environment()

    def test_immutable_upload_conflict_is_not_a_resume(self):
        store = Mock()
        store.put.return_value = False
        with self.assertRaisesRegex(ValueError, 'existing immutable'):
            controller.put_new(store, 'tasks/0/started.json', b'{}')


if __name__ == '__main__':
    unittest.main()
