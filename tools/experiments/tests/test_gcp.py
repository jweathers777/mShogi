"""Transport-contract tests; these are not live GCP execution evidence."""
import concurrent.futures
import json
import subprocess
import unittest
from types import SimpleNamespace
from unittest.mock import Mock, patch

from tools.experiments import gcp


class Missing(Exception): pass
class Conflict(Exception): pass
class Denied(Exception): pass


class Blob:
    def __init__(self, name, objects):
        self.name, self.objects = name, objects
        self.size = None
        self.generation = None
        self.uploads, self.downloads, self.deletes = [], [], []

    def upload_from_string(self, data, **kwargs):
        self.uploads.append(kwargs)
        if self.name in self.objects:
            raise Conflict()
        self.objects[self.name] = (data, 7)

    def reload(self, **kwargs):
        if self.name not in self.objects: raise Missing()
        data, self.generation = self.objects[self.name]
        self.size = len(data)

    def download_as_bytes(self, **kwargs):
        self.downloads.append(kwargs)
        data, generation = self.objects[self.name]
        if kwargs['if_generation_match'] != generation: raise Conflict()
        return data

    def delete(self, **kwargs):
        self.deletes.append(kwargs)
        if kwargs['if_generation_match'] != self.objects[self.name][1]: raise Conflict()
        del self.objects[self.name]

    def exists(self, **kwargs): return self.name in self.objects


class Bucket:
    def __init__(self): self.objects, self.blobs = {}, {}
    def blob(self, name):
        return self.blobs.setdefault(name, Blob(name, self.objects))


class Client:
    def __init__(self): self.b = Bucket()
    def bucket(self, name): return self.b
    def list_blobs(self, bucket, prefix, **kwargs):
        return [SimpleNamespace(name=k) for k in bucket.objects if k.startswith(prefix)]


class StorageTests(unittest.TestCase):
    def setUp(self):
        self.errors = patch.object(gcp, '_api_errors', return_value=(Missing, Conflict))
        self.errors.start()
        self.addCleanup(self.errors.stop)
        self.client = Client()
        self.store = gcp.GCSStore('gs://test-bucket/study', client=self.client, max_object_bytes=32)

    def test_create_only_winner_is_unchanged(self):
        self.assertTrue(self.store.put('job/success', b'first'))
        self.assertFalse(self.store.put('job/success', b'conflicting retry'))
        self.assertEqual(self.store.get('job/success'), b'first')
        blob = self.client.b.blob('study/job/success')
        self.assertTrue(all(x['if_generation_match'] == 0 for x in blob.uploads))
        self.assertEqual(blob.downloads[-1]['if_generation_match'], 7)
        self.assertEqual(blob.downloads[-1]['checksum'], 'crc32c')

    def test_missing_and_denied_are_distinct(self):
        with self.assertRaises(FileNotFoundError): self.store.get('missing')
        blob = self.client.b.blob('study/denied')
        blob.reload = Mock(side_effect=Denied())
        with self.assertRaises(Denied): self.store.get('denied')
        blob.exists = Mock(side_effect=Denied())
        with self.assertRaises(Denied): self.store.exists('denied')
        blob.upload_from_string = Mock(side_effect=Denied())
        with self.assertRaises(Denied): self.store.put('denied', b'x')

    def test_claim_delete_checks_content_and_generation(self):
        self.store.put('claims/job', b'owner-one')
        with self.assertRaisesRegex(ValueError, 'owner mismatch'):
            self.store.delete('claims/job', b'owner-two')
        self.assertEqual(self.store.get('claims/job'), b'owner-one')
        self.assertTrue(self.store.delete('claims/job', b'owner-one'))
        self.assertEqual(self.client.b.blob('study/claims/job').deletes[-1]['if_generation_match'], 7)
        self.assertFalse(self.store.delete('claims/job', b'owner-one'))

    def test_generation_race_cannot_remove_replacement_claim(self):
        self.store.put('claims/job', b'owner-one')
        blob = self.client.b.blob('study/claims/job')
        original = blob.download_as_bytes
        def race(**kw):
            data = original(**kw)
            self.client.b.objects[blob.name] = (b'owner-two', 8)
            return data
        blob.download_as_bytes = race
        with self.assertRaises(Conflict): self.store.delete('claims/job', b'owner-one')
        self.assertEqual(self.client.b.objects[blob.name], (b'owner-two', 8))

    def test_list_is_scoped_to_root(self):
        self.store.put('a/result', b'a')
        self.store.put('b/result', b'b')
        self.client.b.objects['study2/secret'] = (b'x', 1)
        self.assertEqual(self.store.list(), ['a/result', 'b/result'])
        self.assertEqual(self.store.list('a/'), ['a/result'])

    def test_unsafe_keys_and_large_objects_rejected(self):
        for key in ['../x', '/x', 'x//y', 'x/../y', 'x\\y', 'x\ny', '']:
            with self.subTest(key=key), self.assertRaises(ValueError): self.store.put(key, b'x')
        with self.assertRaises(ValueError): self.store.list('/')
        with self.assertRaises(ValueError): self.store.put('large', b'x' * 33)
        self.client.b.objects['study/large'] = (b'x' * 33, 1)
        with self.assertRaises(ValueError): self.store.get('large')
        for uri in ['https://bucket/a', 'gs://test-bucket/a?x=1', 'gs://test-bucket/../a', 'gs://test-bucket//a', 'gs://test-bucket/%2e']:
            with self.subTest(uri=uri), self.assertRaises(ValueError): gcp.parse_uri(uri)


CONFIG = {'project': 'test-project', 'region': 'us-central1',
          'store_uri': 'gs://test-results/experiment',
          'service_account': 'worker@test-project.iam.gserviceaccount.com'}
IMAGE = 'us-central1-docker.pkg.dev/test-project/runner/worker@sha256:' + 'a' * 64
MANIFEST = 'gs://test-results/experiment/manifest.json'


class BatchTests(unittest.TestCase):
    def test_eight_workers_one_task_per_whole_machine(self):
        cfg = gcp.batch_config(MANIFEST, IMAGE, CONFIG, 100)
        group = cfg['taskGroups'][0]
        self.assertEqual((group['taskCount'], group['parallelism'], group['taskCountPerNode']), ('100', '8', '1'))
        self.assertEqual(group['taskSpec']['computeResource']['cpuMilli'], '4000')
        self.assertEqual(group['taskSpec']['maxRunDuration'], '21600s')
        self.assertEqual(group['taskSpec']['maxRetryCount'], 1)
        policy = cfg['allocationPolicy']
        self.assertEqual(policy['instances'][0]['policy']['provisioningModel'], 'STANDARD')
        self.assertEqual(policy['instances'][0]['policy']['machineType'], 'c3-standard-4')
        self.assertEqual(policy['location']['allowedLocations'], ['regions/us-central1'])
        container = group['taskSpec']['runnables'][0]['container']
        self.assertEqual(container['entrypoint'], 'python3')
        self.assertEqual(container['commands'], ['-m', 'tools.experiments.cli', 'cloud-worker', '--manifest', MANIFEST, '--store', CONFIG['store_uri']])
        self.assertNotIn('script', group['taskSpec']['runnables'][0])

    def test_explicit_network_is_emitted_without_default_network(self):
        config = {**CONFIG, 'network':'projects/test-project/global/networks/experiment-workers',
                  'subnetwork':'projects/test-project/regions/us-central1/subnetworks/experiment-workers'}
        result = gcp.batch_config(MANIFEST, IMAGE, config, 1)
        self.assertEqual(result['allocationPolicy']['network']['networkInterfaces'],
                         [{'network':config['network'], 'subnetwork':config['subnetwork']}])
        self.assertNotIn('network', gcp.batch_config(MANIFEST, IMAGE, CONFIG, 1)['allocationPolicy'])

    def test_network_names_project_region_and_pair_are_validated(self):
        network = 'projects/test-project/global/networks/experiment-workers'
        subnet = 'projects/test-project/regions/us-central1/subnetworks/experiment-workers'
        invalid = [
            {'network': network}, {'subnetwork': subnet},
            {'network': network.replace('test-project','other-project'), 'subnetwork': subnet},
            {'network': network, 'subnetwork': subnet.replace('test-project','other-project')},
            {'network': network, 'subnetwork': subnet.replace('us-central1','us-east1')},
            {'network': network, 'subnetwork': subnet + '/'},
            {'network': network + '/../../other', 'subnetwork': subnet},
            {'network': 'global/networks/experiment-workers', 'subnetwork': subnet},
            {'network': network.rsplit('/',1)[0] + '/Bad_name', 'subnetwork': subnet},
            {'network': None, 'subnetwork': None},
        ]
        for values in invalid:
            with self.subTest(values=values), self.assertRaises(ValueError):
                gcp.validate_config({**CONFIG, **values})

    def test_small_pilot_and_sixteen_ceiling(self):
        self.assertEqual(gcp.batch_config(MANIFEST, IMAGE, CONFIG, 1)['taskGroups'][0]['parallelism'], '1')
        self.assertEqual(gcp.batch_config(MANIFEST, IMAGE, {**CONFIG, 'parallelism':16}, 100)['taskGroups'][0]['parallelism'], '16')
        for key, value in [('parallelism',17), ('parallelism',True), ('cpu_milli',1000), ('machine_type','e2-standard-4'), ('max_retries',10), ('max_run_seconds',0), ('unexpected',1), ('vm_hourly_usd',float('nan'))]:
            with self.subTest(key=key), self.assertRaises(ValueError):
                gcp.batch_config(MANIFEST, IMAGE, {**CONFIG, key:value}, 1)
        with self.assertRaises(ValueError): gcp.batch_config(MANIFEST, IMAGE.split('@')[0]+':latest', CONFIG, 1)
        with self.assertRaises(ValueError): gcp.batch_config(MANIFEST, IMAGE, CONFIG, 0)

    def test_submit_argument_vector_and_observable_failure(self):
        def run(argv, **kw):
            self.assertEqual(argv[:5], ['gcloud','batch','jobs','submit','pilot-01'])
            self.assertTrue(kw['check'])
            self.assertNotIn('shell', kw)
            with open(argv[argv.index('--config')+1]) as f: document=json.load(f)
            self.assertEqual(document['taskGroups'][0]['taskCount'], '1')
            return SimpleNamespace(stdout='{"name":"pilot-01"}')
        with patch.object(gcp.subprocess, 'run', side_effect=run):
            self.assertEqual(gcp.submit_job('pilot-01', MANIFEST, IMAGE, CONFIG, 1), {'name':'pilot-01'})
        with patch.object(gcp.subprocess, 'run', side_effect=subprocess.CalledProcessError(1, 'gcloud')):
            with self.assertRaises(subprocess.CalledProcessError): gcp.submit_job('pilot-01', MANIFEST, IMAGE, CONFIG, 1)

    def test_cancel_requests_async_deletion_without_touching_store(self):
        with patch.object(gcp, '_run', return_value={'name':'operation'}) as run:
            self.assertEqual(gcp.cancel_job('pilot-01','test-project','us-central1'), {'name':'operation'})
            self.assertEqual(run.call_args.args[0][0], 'delete')
            self.assertIn('--async',run.call_args.args[0])

    def test_cost_is_estimated_and_excludes_unobserved_charges(self):
        result = gcp.cost_estimate(7200, .20)
        self.assertAlmostEqual(result['estimated_compute_usd'], .40)
        self.assertIn('not a billing receipt', result['limitations'])
        for value in [-1,float('inf'),True]:
            with self.assertRaises(ValueError): gcp.cost_estimate(value,.2)


TOPIC = 'projects/test-project/topics/experiment-completion'
SUBSCRIPTION = 'projects/test-project/subscriptions/experiment-consumer'
NOTICE_CONFIG = {**CONFIG, 'notification_topic': TOPIC}
JOB_NAME = 'projects/test-project/locations/us-central1/jobs/pilot-01'


def observed(state):
    return {'name': JOB_NAME, 'uid': 'unique-job-generation', 'status': {'state': state},
            'notifications': [{'pubsubTopic': TOPIC, 'message': {'type': 'JOB_STATE_CHANGED'}}]}


class Message:
    def __init__(self, **overrides):
        self.attributes = {'Type': 'JOB_STATE_CHANGED', 'JobUID': 'unique-job-generation',
                           'JobName': JOB_NAME, 'NewJobState': 'SUCCEEDED', **overrides}
        self.acks, self.nacks = 0, 0
    def ack(self): self.acks += 1
    def nack(self): self.nacks += 1


class Subscriber:
    def __init__(self, messages=(), error=None, topic=TOPIC):
        self.messages, self.error, self.topic = messages, error, topic
        self.stream = concurrent.futures.Future()
        self.subscription = None
    def get_subscription(self, request, timeout):
        self.subscription = request['subscription']
        return SimpleNamespace(topic=self.topic, detached=False)
    def subscribe(self, name, callback):
        if self.error:
            self.stream.set_exception(self.error)
        for message in self.messages:
            callback(message)
        return self.stream


class NotificationTests(unittest.TestCase):
    def test_optional_topic_sets_official_batch_notification(self):
        cfg = gcp.batch_config(MANIFEST, IMAGE, NOTICE_CONFIG, 1)
        self.assertEqual(cfg['notifications'], [{'pubsubTopic': TOPIC, 'message': {'type': 'JOB_STATE_CHANGED'}}])
        self.assertNotIn('notifications', gcp.batch_config(MANIFEST, IMAGE, CONFIG, 1))
        with self.assertRaises(ValueError):
            gcp.validate_config({**CONFIG, 'notification_topic': 'projects/other-project/topics/events'})

    def test_initial_completed_requires_no_stream(self):
        client = Mock()
        with patch.object(gcp, 'status_job', return_value=observed('SUCCEEDED')) as status:
            result = gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, client)
        self.assertEqual(result['completion_source'], 'initial-status')
        self.assertEqual(status.call_count, 1)
        client.subscribe.assert_not_called()

    def test_initial_failed_retains_authoritative_receipt(self):
        with patch.object(gcp, 'status_job', return_value=observed('FAILED')):
            with self.assertRaises(gcp.BatchJobFailed) as raised:
                gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, Mock())
        self.assertEqual(raised.exception.result['job']['status']['state'], 'FAILED')

    def test_initial_cancellation_or_deletion_never_waits_or_succeeds(self):
        for state in ('CANCELLED', 'DELETION_IN_PROGRESS', 'CANCELLATION_IN_PROGRESS'):
            with self.subTest(state=state), patch.object(gcp, 'status_job', return_value=observed(state)):
                client = Mock()
                with self.assertRaises(gcp.BatchJobFailed) as raised:
                    gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, client)
                self.assertEqual(raised.exception.result['job']['status']['state'], state)
                client.subscribe.assert_not_called()
                if state.endswith('_IN_PROGRESS'):
                    self.assertFalse(raised.exception.result['cleanup_complete'])
                    self.assertTrue(raised.exception.result['requires_cleanup_verification'])

    def test_cancellation_or_deletion_event_retains_unsuccessful_receipt(self):
        for state in ('CANCELLED', 'DELETION_IN_PROGRESS', 'CANCELLATION_IN_PROGRESS'):
            with self.subTest(state=state):
                client = Subscriber([Message(NewJobState=state)])
                with patch.object(gcp, 'status_job', side_effect=[observed('RUNNING'), observed(state)]):
                    with self.assertRaises(gcp.BatchJobFailed) as raised:
                        gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, client)
                self.assertEqual(raised.exception.result['notification']['NewJobState'], state)
                self.assertTrue(client.stream.cancelled())

    def test_filters_other_uid_name_and_nonterminal_then_verifies_success(self):
        messages = [Message(JobUID='old-generation'), Message(JobName='different-job'),
                    Message(NewJobState='RUNNING'), Message(Type='TASK_STATE_CHANGED'), Message()]
        client = Subscriber(messages)
        with patch.object(gcp, 'status_job', side_effect=[observed('RUNNING'), observed('SUCCEEDED')]) as status:
            result = gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, client)
        self.assertEqual(status.call_count, 2)
        self.assertEqual(result['completion_source'], 'pubsub-terminal-event')
        self.assertTrue(all(m.acks == 1 for m in messages))
        self.assertTrue(client.stream.cancelled())

    def test_failure_event_is_not_success_and_keeps_notification(self):
        client = Subscriber([Message(NewJobState='FAILED')])
        with patch.object(gcp, 'status_job', side_effect=[observed('RUNNING'), observed('FAILED')]):
            with self.assertRaises(gcp.BatchJobFailed) as raised:
                gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, client)
        self.assertEqual(raised.exception.result['notification']['NewJobState'], 'FAILED')
        self.assertTrue(client.stream.cancelled())

    def test_event_is_retained_until_authoritative_state_is_terminal(self):
        first, redelivery = Message(), Message()
        client = Subscriber([first, redelivery])
        with patch.object(gcp, 'status_job', side_effect=[observed('RUNNING'), observed('RUNNING'), observed('SUCCEEDED')]) as status:
            result = gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, client)
        self.assertEqual(first.nacks, 1)
        self.assertEqual(redelivery.acks, 1)
        self.assertEqual(status.call_count, 3)
        self.assertEqual(result['job']['status']['state'], 'SUCCEEDED')

    def test_stream_failure_exits_instead_of_hanging(self):
        client = Subscriber(error=Denied('subscriber denied'))
        with patch.object(gcp, 'status_job', return_value=observed('RUNNING')):
            with self.assertRaises(Denied):
                gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, client)

    def test_wrong_subscription_topic_or_project_rejected(self):
        client = Subscriber(topic='projects/test-project/topics/other-topic')
        with patch.object(gcp, 'status_job', return_value=observed('RUNNING')):
            with self.assertRaisesRegex(ValueError, 'different topic'):
                gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, client)
        with self.assertRaises(ValueError):
            gcp.wait_for_completion('pilot-01', NOTICE_CONFIG,
                'projects/other-project/subscriptions/consumer', client)

    def test_existing_job_without_notifications_rejected(self):
        value = observed('RUNNING'); value['notifications'] = []
        with patch.object(gcp, 'status_job', return_value=value):
            with self.assertRaisesRegex(ValueError, 'does not publish'):
                gcp.wait_for_completion('pilot-01', NOTICE_CONFIG, SUBSCRIPTION, Mock())


if __name__ == '__main__': unittest.main()
