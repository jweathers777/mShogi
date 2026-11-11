"""GCP transport for experiment jobs; no Chess rules or scientific decisions.

References (verified 2026-11-11):
https://docs.cloud.google.com/batch/docs/reference/rest/v1/projects.locations.jobs
https://docs.cloud.google.com/storage/docs/request-preconditions
https://docs.cloud.google.com/python/docs/reference/storage/latest/google.cloud.storage.blob.Blob

Cloud resources are created only by explicit submit_job calls. SDK credentials use
Application Default Credentials; this module never reads or prints credential files.
"""
from __future__ import annotations

import concurrent.futures
import json
import math
import re
import subprocess
import tempfile
import threading
from urllib.parse import urlsplit


def _api_errors():
    from google.api_core.exceptions import NotFound, PreconditionFailed
    return NotFound, PreconditionFailed


def _key(key, *, prefix=False):
    if not isinstance(key, str) or (not key and not prefix):
        raise ValueError('Object key must be nonempty text')
    stripped = key.rstrip('/') if prefix else key
    if key.startswith('/') or (stripped and ('\\' in stripped or
                     any(part in ('', '.', '..') for part in stripped.split('/')) or
                     any(ord(c) < 32 for c in stripped))):
        raise ValueError('Unsafe object key')
    if not prefix and key.endswith('/'):
        raise ValueError('Object key must name a file')
    return key


def parse_uri(uri):
    """Return bucket and optional prefix, rejecting ambiguous URL spellings."""
    parsed = urlsplit(uri)
    if (parsed.scheme != 'gs' or not re.fullmatch(r'[a-z0-9][a-z0-9._-]{1,220}[a-z0-9]', parsed.netloc)
            or parsed.query or parsed.fragment or '%' in parsed.path):
        raise ValueError('Expected gs://bucket/optional-prefix without query or fragment')
    prefix = parsed.path.lstrip('/').rstrip('/')
    if parsed.path.startswith('//'):
        raise ValueError('Ambiguous storage path')
    _key(prefix, prefix=True)
    return parsed.netloc, prefix


class GCSStore:
    """Immutable object store. 412 means already present; other errors propagate.

    Result objects are never deleted by the worker. The separate conditional
    delete method is for owned execution claims only. Scope delete IAM to the
    claims prefix; bucket retention/versioning is not changed here.
    Download generation pinning prevents mixed-version reads even if an external
    administrator changes an object despite the create-only writer contract.
    """
    def __init__(self, uri, client=None, max_object_bytes=128 * 1024 * 1024):
        bucket, self.prefix = parse_uri(uri)
        if type(max_object_bytes) is not int or max_object_bytes <= 0:
            raise ValueError('Invalid object byte limit')
        self.max_object_bytes = max_object_bytes
        if client is None:
            from google.cloud import storage
            client = storage.Client()
        self.client = client
        self.bucket = client.bucket(bucket)

    def _name(self, key, *, prefix=False):
        _key(key, prefix=prefix)
        return '/'.join(x for x in (self.prefix, key) if x)

    def put(self, key, data):
        if not isinstance(data, bytes) or len(data) > self.max_object_bytes:
            raise ValueError('Expected bounded bytes')
        _, precondition = _api_errors()
        try:
            self.bucket.blob(self._name(key)).upload_from_string(
                data, content_type='application/octet-stream',
                if_generation_match=0, checksum='crc32c', timeout=60)
        except precondition:
            return False
        return True

    def get(self, key):
        missing, _ = _api_errors()
        blob = self.bucket.blob(self._name(key))
        try:
            blob.reload(timeout=60)
            if blob.size is None or blob.size > self.max_object_bytes:
                raise ValueError('Cloud object exceeds byte limit or has unknown size')
            result = blob.download_as_bytes(if_generation_match=blob.generation,
                                            checksum='crc32c', timeout=60)
        except missing as exc:
            raise FileNotFoundError(key) from exc
        if len(result) > self.max_object_bytes:
            raise ValueError('Downloaded cloud object exceeds byte limit')
        return result

    def delete(self, key, expected):
        """Release a claim only if its bytes and generation still match.

        Caller must restrict this to claims it owns, never successful results.
        A generation race propagates as PreconditionFailed for reconciliation.
        """
        if not isinstance(expected, bytes):
            raise ValueError('Expected claim bytes')
        missing, _ = _api_errors()
        blob = self.bucket.blob(self._name(key))
        try:
            blob.reload(timeout=60)
            if blob.size is None or blob.size > self.max_object_bytes:
                raise ValueError('Cloud claim exceeds byte limit')
            generation = blob.generation
            actual = blob.download_as_bytes(if_generation_match=generation,
                                            checksum='crc32c', timeout=60)
            if actual != expected:
                raise ValueError('Claim owner mismatch; refusing deletion')
            blob.delete(if_generation_match=generation, timeout=60)
        except missing:
            return False
        return True

    def exists(self, key):
        # The SDK returns False only for NotFound. Permission/network errors must
        # propagate; they are not evidence that a successful job is absent.
        return self.bucket.blob(self._name(key)).exists(timeout=60)

    def list(self, prefix=''):
        name = self._name(prefix, prefix=True)
        # An empty key prefix refers to this store's directory, never adjacent
        # roots such as "study2" when the store root is "study".
        if not prefix and name:
            name += '/'
        root = self.prefix + '/' if self.prefix else ''
        return sorted(blob.name[len(root):] for blob in
                      self.client.list_blobs(self.bucket, prefix=name, timeout=60)
                      if blob.name.startswith(root))


def _integer(config, name, default, minimum, maximum):
    value = config.get(name, default)
    if type(value) is not int or not minimum <= value <= maximum:
        raise ValueError(f'{name} must be an integer in [{minimum}, {maximum}]')
    return value


def _identifier(value, pattern, name):
    if not isinstance(value, str) or not re.fullmatch(pattern, value):
        raise ValueError(f'Invalid {name}')
    return value


def validate_config(config):
    """Normalize the small, explicitly provisioned execution configuration."""
    allowed = {'project', 'region', 'store_uri', 'machine_type', 'parallelism',
               'max_run_seconds', 'max_retries', 'service_account', 'cpu_milli',
               'memory_mib', 'boot_disk_gb', 'boot_image', 'vm_hourly_usd', 'notification_topic',
               'network', 'subnetwork'}
    if not isinstance(config, dict) or set(config) - allowed:
        raise ValueError('Unknown cloud configuration fields')
    result = dict(config)
    result['project'] = _identifier(config.get('project'), r'[a-z][a-z0-9-]{4,61}[a-z0-9]', 'project')
    result['region'] = _identifier(config.get('region'), r'[a-z]+(?:-[a-z]+)+[0-9]', 'region')
    parse_uri(config.get('store_uri', ''))
    if ('network' in config) != ('subnetwork' in config):
        raise ValueError('network and subnetwork must be specified together')
    if 'network' in config:
        resource_name = r'[a-z](?:[a-z0-9-]{0,61}[a-z0-9])?'
        base = 'projects/' + re.escape(result['project'])
        _identifier(config['network'], base + '/global/networks/' + resource_name, 'same-project network')
        _identifier(config['subnetwork'], base + '/regions/' + re.escape(result['region']) + '/subnetworks/' + resource_name, 'same-project and region subnetwork')
    result['service_account'] = _identifier(config.get('service_account'),
        r'[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.iam\.gserviceaccount\.com', 'service account')
    # Fixed C3 x86 hardware: changing to another family requires an explicit code
    # review and recorded comparability check, not an implicit auto selection.
    machine = config.get('machine_type', 'c3-standard-4')
    _identifier(machine, r'c3-standard-(4|8|22|44|88|176)', 'machine type')
    cpus = int(machine.rsplit('-', 1)[1])
    result['machine_type'] = machine
    result['cpu_milli'] = _integer(config, 'cpu_milli', cpus * 1000, cpus * 1000, cpus * 1000)
    result['memory_mib'] = _integer(config, 'memory_mib', cpus * 3072, 1024, cpus * 3500)
    result['parallelism'] = _integer(config, 'parallelism', 8, 1, 16)
    result['max_run_seconds'] = _integer(config, 'max_run_seconds', 21600, 60, 172800)
    result['max_retries'] = _integer(config, 'max_retries', 1, 0, 3)
    result['boot_disk_gb'] = _integer(config, 'boot_disk_gb', 30, 20, 200)
    if config.get('boot_image') is not None:
        _identifier(config['boot_image'], r'projects/[a-z0-9-]+/global/images/[a-z0-9-]+', 'pinned boot image')
    if 'notification_topic' in config:
        _pubsub_name(config['notification_topic'], result['project'], 'topics')
    if 'vm_hourly_usd' in config:
        rate = config['vm_hourly_usd']
        if isinstance(rate, bool) or not isinstance(rate, (int, float)) or not math.isfinite(rate) or rate <= 0:
            raise ValueError('vm_hourly_usd must be a positive finite owner-reviewed rate')
    return result


def batch_config(manifest_uri, image, config, task_count):
    """One pipeline task per VM; its game and analysis stages never overlap."""
    config = validate_config(config)
    parse_uri(manifest_uri)
    _identifier(image, r'[a-zA-Z0-9./:_-]+@sha256:[a-f0-9]{64}', 'digest-pinned image')
    if type(task_count) is not int or not 1 <= task_count <= 10000:
        raise ValueError('task_count must be in [1, 10000]')
    disk = {'sizeGb': str(config['boot_disk_gb']), 'type': 'pd-balanced'}
    if config.get('boot_image'):
        disk['image'] = config['boot_image']
    result = {
        'taskGroups': [{
            'taskCount': str(task_count),
            'parallelism': str(min(task_count, config['parallelism'])),
            'taskCountPerNode': '1',
            'taskSpec': {
                'computeResource': {'cpuMilli': str(config['cpu_milli']),
                                    'memoryMib': str(config['memory_mib'])},
                'maxRetryCount': config['max_retries'],
                'maxRunDuration': f"{config['max_run_seconds']}s",
                'runnables': [{'container': {
                    'imageUri': image, 'entrypoint': 'python3',
                    'commands': ['-m', 'tools.experiments.cli', 'cloud-worker',
                                 '--manifest', manifest_uri, '--store', config['store_uri']],
                }}],
            },
        }],
        'allocationPolicy': {
            'instances': [{'policy': {'machineType': config['machine_type'],
                                     'provisioningModel': 'STANDARD', 'bootDisk': disk}}],
            'location': {'allowedLocations': ['regions/' + config['region']]},
            'serviceAccount': {'email': config['service_account'],
                               'scopes': ['https://www.googleapis.com/auth/cloud-platform']},
        },
        'logsPolicy': {'destination': 'CLOUD_LOGGING'},
        'labels': {'application': 'mshogi-experiments', 'schema': 'v1'},
    }
    if 'network' in config:
        result['allocationPolicy']['network'] = {'networkInterfaces': [{
            'network': config['network'], 'subnetwork': config['subnetwork']}]}
    if config.get('notification_topic'):
        result['notifications'] = [{'pubsubTopic': config['notification_topic'],
                                    'message': {'type': 'JOB_STATE_CHANGED'}}]
    return result


def _scope(project, region, job_id):
    _identifier(project, r'[a-z][a-z0-9-]{4,61}[a-z0-9]', 'project')
    _identifier(region, r'[a-z]+(?:-[a-z]+)+[0-9]', 'region')
    _identifier(job_id, r'[a-z][a-z0-9-]{0,61}[a-z0-9]', 'Batch job id')
    return ['--project', project, '--location', region, '--format=json', '--quiet']


def _run(args):
    completed = subprocess.run(['gcloud', 'batch', 'jobs', *args],
                               check=True, capture_output=True, text=True, timeout=120)
    return json.loads(completed.stdout) if completed.stdout.strip() else {}


def submit_job(job_id, manifest_uri, image, config, task_count):
    """Explicit billable action. Caller must first obtain owner authorization."""
    spec = batch_config(manifest_uri, image, config, task_count)
    scope = _scope(config['project'], config['region'], job_id)
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json') as handle:
        json.dump(spec, handle, allow_nan=False)
        handle.flush()
        return _run(['submit', job_id, *scope, '--config', handle.name])


def status_job(job_id, project, region):
    return _run(['describe', job_id, *_scope(project, region, job_id)])


def cancel_job(job_id, project, region):
    """Request Batch deletion/cancellation; GCS evidence is untouched.

    Caller should retain status before this call. The asynchronous operation is a
    request receipt, not proof that every task or VM has already stopped.
    """
    return _run(['delete', job_id, *_scope(project, region, job_id), '--async'])


def cost_estimate(vm_seconds, hourly_usd):
    """Compute-only estimate from supplied allocation time and reviewed rate."""
    for value in (vm_seconds, hourly_usd):
        if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value) or value < 0:
            raise ValueError('Cost inputs must be finite nonnegative numbers')
    return {'vm_seconds': vm_seconds, 'vm_hourly_usd': hourly_usd,
            'estimated_compute_usd': vm_seconds / 3600 * hourly_usd,
            'limitations': 'Estimate, not a billing receipt. Include VM startup/idle/retry time; excludes disk, storage, logging, network, taxes and discounts.'}


def _pubsub_name(value, project, collection):
    pattern = r'projects/' + re.escape(project) + '/' + collection + r'/[A-Za-z][A-Za-z0-9_.~+%-]{2,254}'
    _identifier(value, pattern, 'owner-project Pub/Sub ' + collection)
    if value.rsplit('/', 1)[1].lower().startswith('goog'):
        raise ValueError('Reserved Pub/Sub resource name')
    return value


class BatchJobFailed(RuntimeError):
    """Unsuccessful Batch outcome with the authoritative observation retained."""
    def __init__(self, result):
        self.result = result
        super().__init__('Batch job ended unsuccessfully (' + result['job']['status']['state'] + '): ' + result['job']['name'])


def wait_for_completion(job_id, config, subscription, client=None):
    """Wait on a pre-existing Pub/Sub subscription, without a polling loop.

    Caller wraps this long-lived local process with the computation exit notifier.
    Keep one dedicated subscription per completion consumer; unrelated messages
    on that subscription are acknowledged. The subscription must already exist
    before Batch submission, retaining events during local process restarts.

    One initial Batch lookup handles a job already complete at attachment. A
    matching terminal notification causes one authoritative lookup. Event UID
    matching prevents an old job with a recycled name from completing this wait.
    The subscription and topic must belong to the configured owner project.
    No resources or IAM are created or changed here.

    Sources, verified 2026-11-11:
    https://docs.cloud.google.com/batch/docs/enable-notifications
    https://docs.cloud.google.com/batch/docs/monitor-jobs-using-notifications
    Attributes: JobUID, JobName, NewJobState, Type.
    """
    config = validate_config(config)
    topic = config.get('notification_topic')
    if not topic:
        raise ValueError('notification_topic must be configured before submission')
    _pubsub_name(subscription, config['project'], 'subscriptions')
    _scope(config['project'], config['region'], job_id)
    initial = status_job(job_id, config['project'], config['region'])
    name = initial.get('name')
    uid = initial.get('uid')
    expected_name = f"projects/{config['project']}/locations/{config['region']}/jobs/{job_id}"
    # Batch may canonicalize project ID to project number in the returned name.
    # Always match notifications against the authoritative returned name and UID.
    if (not isinstance(name, str) or not name.endswith('/locations/' + config['region'] + '/jobs/' + job_id)
            or not isinstance(uid, str) or not uid):
        raise ValueError('Batch response lacks job identity: ' + expected_name)
    # Deletion/cancellation in progress ends this completion consumer as failure;
    # it does not establish that cloud resource cleanup has finished.
    terminal = {'SUCCEEDED', 'FAILED', 'CANCELLED', 'DELETION_IN_PROGRESS', 'CANCELLATION_IN_PROGRESS'}

    def finish(observed, attributes, origin):
        result = {'job': observed, 'notification': attributes, 'completion_source': origin}
        if observed['status']['state'] in ('DELETION_IN_PROGRESS', 'CANCELLATION_IN_PROGRESS'):
            result['cleanup_complete'] = False
            result['requires_cleanup_verification'] = True
        if observed['status']['state'] != 'SUCCEEDED':
            raise BatchJobFailed(result)
        return result

    if initial.get('status', {}).get('state') in terminal:
        return finish(initial, None, 'initial-status')
    required_notification = any(
        entry.get('pubsubTopic') == topic and
        entry.get('message', {}).get('type') == 'JOB_STATE_CHANGED' and
        not entry.get('message', {}).get('newJobState')
        for entry in initial.get('notifications', []))
    if not required_notification:
        raise ValueError('Existing Batch job does not publish all job state changes to configured topic')
    owned_client = client is None
    if client is None:
        from google.cloud import pubsub_v1
        client = pubsub_v1.SubscriberClient()
    stream = None
    completion = concurrent.futures.Future()
    lock = threading.Lock()
    try:
        description = client.get_subscription(request={'subscription': subscription}, timeout=60)
        if description.topic != topic or getattr(description, 'detached', False):
            raise ValueError('Subscription is detached or points to a different topic')

        def callback(message):
            attributes = dict(message.attributes)
            with lock:
                if completion.done():
                    message.nack()
                    return
                if (attributes.get('Type') != 'JOB_STATE_CHANGED' or
                        attributes.get('JobUID') != uid or attributes.get('JobName') != name or
                        attributes.get('NewJobState') not in terminal):
                    message.ack()
                    return
                try:
                    observed = status_job(job_id, config['project'], config['region'])
                    if observed.get('uid') != uid or observed.get('name') != name:
                        raise ValueError('Batch identity changed while awaiting completion')
                    if observed.get('status', {}).get('state') not in terminal:
                        # Retain this event for redelivery if the read model lags.
                        # This is event-driven delivery, not a scheduled status poll.
                        message.nack()
                        return
                    message.ack()
                    try:
                        result = finish(observed, attributes, 'pubsub-terminal-event')
                    except BatchJobFailed as exc:
                        completion.set_exception(exc)
                    else:
                        completion.set_result(result)
                except Exception as exc:
                    message.nack()
                    completion.set_exception(exc)

        stream = client.subscribe(subscription, callback=callback)

        def stream_finished(future):
            with lock:
                if completion.done():
                    return
                try:
                    future.result()
                except Exception as exc:
                    completion.set_exception(exc)
                else:
                    completion.set_exception(RuntimeError('Pub/Sub stream ended before Batch completion'))

        stream.add_done_callback(stream_finished)
        return completion.result()
    finally:
        if stream is not None:
            stream.cancel()
            try:
                stream.result(timeout=30)
            except Exception:
                pass
        if owned_client:
            client.close()
