"""Cost-bounded single-batch controller and worker helpers on the GCP transport.

No match selection or game rules live here. Run one reviewed manifest, wait on
its existing Pub/Sub completion subscription, preserve all evidence, then delete
the job and verify its VMs are absent. No retries or automatic next batch.
"""
import concurrent.futures
import json
import math
import os
import queue
import re
import subprocess
import threading
import time
import urllib.request
import uuid
from dataclasses import dataclass
from pathlib import Path

from tools.experiments import gcp
from tools.experiments.core import encode, digest, environment


@dataclass(frozen=True)
class CampaignPolicy:
    """Reviewed fixed policy; callers cannot set it from a cloud manifest."""
    prefix: str
    seconds: int
    compute_usd: float
    worker_module: str
    worker_arguments: tuple = ()

    @property
    def owner_label(self):
        return self.prefix + '-owner'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def put_new(store, key, data):
    require(store.put(key, data), 'existing immutable artifact: ' + key)


def cloud_environment():
    """The worker's host record; refuses a VM that is not the approved type and region."""
    result = environment()
    require(result['architecture'] == 'x86_64' and result['platform'].startswith('Linux'), 'expected Linux x86_64')
    result['gcp'] = {}
    for field in ('machine-type', 'zone', 'id', 'cpu-platform'):
        request = urllib.request.Request('http://metadata.google.internal/computeMetadata/v1/instance/' + field,
                                         headers={'Metadata-Flavor': 'Google'})
        with urllib.request.urlopen(request, timeout=5) as response:
            result['gcp'][field] = response.read(4096).decode()
    require(result['gcp']['machine-type'].rsplit('/', 1)[-1] == 'c3-standard-4', 'wrong VM type')
    require(result['gcp']['zone'].rsplit('/', 1)[-1].startswith('us-central1-'), 'wrong VM region')
    result['batch'] = {key: os.environ.get(key) for key in
                       ('BATCH_TASK_INDEX', 'BATCH_TASK_COUNT', 'BATCH_JOB_UID', 'BATCH_TASK_RETRY_ATTEMPT')}
    return result


def save(path, value):
    path = Path(path)
    with path.open('x') as stream:
        json.dump(value, stream, indent=2, allow_nan=False)
        stream.write('\n')


def cloud(args, timeout=60):
    result = subprocess.run(['gcloud', *args, '--project=mshogi', '--format=json', '--quiet'],
                            check=True, capture_output=True, text=True, timeout=timeout)
    return json.loads(result.stdout) if result.stdout.strip() else {}


def jobs_named(job, region, timeout=None):
    args = ['batch', 'jobs', 'list', '--location='+region, '--filter=name:'+job]
    jobs = cloud(args) if timeout is None else cloud(args, timeout=timeout)
    if not isinstance(jobs, list):
        raise ValueError('Expected a job list')
    return [row for row in jobs if row.get('name', '').rsplit('/', 1)[-1] == job]


def cleanup(job, region, owner, out, submission_confirmed, *, owner_label, campaign_deadline=None):
    """Delete only this intent's labelled job, then verify jobs, VMs and disks.

    Label definitions: https://docs.cloud.google.com/batch/docs/organize-resources-using-labels
    An uncertain submission with no observed job cannot prove the server will
    not create it later. Preserve that uncertainty for explicit reconciliation.
    """
    receipt = {'verified_absent': False, 'submission_confirmed': submission_confirmed}
    try:
        jobs = jobs_named(job, region)
        if jobs:
            if len(jobs) != 1 or jobs[0].get('labels', {}).get(owner_label) != owner:
                raise RuntimeError('Job ownership differs; refusing deletion')
            receipt['owned_job'] = jobs[0]
            receipt['deletion'] = cloud(['batch', 'jobs', 'delete', job, '--location='+region])
        allowance = 180 if campaign_deadline is None else min(180, max(0, campaign_deadline-time.time()))
        poll_deadline = time.monotonic() + allowance
        receipt['poll_allowance_seconds'] = allowance
        receipt['polls'] = 0
        def remaining_timeout():
            remaining = poll_deadline - time.monotonic()
            if remaining <= 0:
                raise TimeoutError('Cleanup verification window expired')
            return min(60, remaining)
        while True:
            jobs = jobs_named(job, region, timeout=remaining_timeout())
            vms = cloud(['compute', 'instances', 'list', '--filter=labels.batch-job-id='+job],
                        timeout=remaining_timeout())
            disks = cloud(['compute', 'disks', 'list', '--filter=labels.batch-job-id='+job],
                          timeout=remaining_timeout())
            receipt['polls'] += 1
            if not isinstance(vms, list) or not isinstance(disks, list):
                raise ValueError('Expected VM and disk lists')
            receipt.update(remaining_jobs=jobs, remaining_vms=vms, remaining_disks=disks)
            if not jobs and not vms and not disks:
                receipt['verified_absent'] = bool(submission_confirmed or receipt.get('owned_job'))
                break
            time.sleep(min(5, remaining_timeout()))
        if not receipt['verified_absent']:
            raise RuntimeError('Cloud cleanup absent or submission status is not established')
    except BaseException as exc:
        receipt['error'] = repr(exc)
        raise
    finally:
        receipt['at_epoch'] = time.time()
        save(out/'cleanup.json', receipt)


def run(args, *, policy, manifest_validator):
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=False)
    manifest = json.loads(Path(args.manifest).read_text())
    manifest_validator(manifest)
    config = gcp.validate_config(json.loads(Path(args.config).read_text()))
    if config['project'] != 'mshogi' or config['machine_type'] != 'c3-standard-4' or config['parallelism'] != 8:
        raise ValueError('The approved project, machine and eight-worker limit are required')
    if config['max_retries'] != 0 or not 1 <= len(manifest['bins']) <= 8:
        raise ValueError('No retries; at most eight nonempty worker bins')
    if any(not rows for rows in manifest['bins']):
        raise ValueError('Empty worker bin')
    if not re.fullmatch(re.escape(policy.prefix) + r'-[a-z0-9-]{1,55}', args.job):
        raise ValueError('Use a new ' + policy.prefix + '- job ID')
    if config['region'] != 'us-central1' or not config.get('notification_topic'):
        raise ValueError('Approved region and existing notification topic required')
    gcp._pubsub_name(args.subscription, config['project'], 'subscriptions')
    start = time.time()
    deadline = manifest['deadline_epoch']
    if not start < deadline <= start + policy.seconds:
        raise ValueError('Invalid campaign deadline')
    if deadline - start <= 300:
        raise ValueError('Campaign deadline leaves no time beyond cleanup reserve')
    if deadline != manifest['campaign_started_epoch'] + policy.seconds:
        raise ValueError('Deadline must retain the original campaign start')
    before = manifest['estimated_compute_before_usd']
    if type(before) not in (int, float) or not math.isfinite(before) or not 0 <= before < policy.compute_usd:
        raise ValueError('Compute cap reached')
    if manifest['campaign_started_epoch'] > start:
        raise ValueError('Campaign start is in the future')
    if 'vm_hourly_usd' not in config:
        raise ValueError('Recorded VM hourly rate required')
    # At most eight VMs for the entire remaining campaign: a conservative reserve.
    reserve = 8 * (deadline - start) / 3600 * config['vm_hourly_usd']
    if manifest['estimated_compute_before_usd'] + reserve > policy.compute_usd:
        raise ValueError('Remaining maximum allocation would exceed compute cap')
    if jobs_named(args.job, config['region']):
        raise ValueError('Job already exists; reconcile instead of rerunning')
    store = gcp.GCSStore(config['store_uri'])
    key = 'manifest.json'
    data = encode(manifest)
    if not store.put(key, data):
        raise ValueError('Batch prefix already exists; reconcile instead of rerunning')
    uri = config['store_uri'].rstrip('/') + '/' + key
    spec = gcp.batch_config(uri, args.image, config, len(manifest['bins']), [
        '-m', policy.worker_module, *policy.worker_arguments, '--manifest', uri, '--store', config['store_uri']])
    owner = uuid.uuid4().hex
    spec['labels'][policy.owner_label] = owner
    spec['allocationPolicy']['labels'] = {policy.owner_label: owner}
    # VM runtime bound supplements the absolute worker deadline and local watchdog.
    spec['taskGroups'][0]['taskSpec']['maxRunDuration'] = str(min(config['max_run_seconds'], int(deadline-start))) + 's'
    save(out / 'batch-request.json', spec)
    save(out / 'manifest.json', manifest)
    save(out / 'intent.json', {'job': args.job, 'manifest_sha256': digest(data), 'started_epoch': start,
                              'max_remaining_compute_usd': reserve, 'image': args.image, 'owner': owner})
    attempted = False
    submission_confirmed = False
    terminal = None
    error = None
    try:
        attempted = True
        submitted = cloud(['batch', 'jobs', 'submit', args.job, '--location='+config['region'],
                           '--config='+str(out/'batch-request.json')])
        submission_confirmed = True
        save(out/'submission.json', submitted)
        result = queue.Queue()
        def wait():
            try:
                result.put((True, gcp.wait_for_completion(args.job, config, args.subscription)))
            except BaseException as exc:
                result.put((False, getattr(exc, 'result', {'error': repr(exc)})))
        threading.Thread(target=wait, daemon=True).start()
        ok, terminal = result.get(timeout=max(1, deadline-time.time()-300))
        save(out/'terminal.json', terminal)
        if not ok:
            raise RuntimeError('Batch failed; see terminal.json')
    except BaseException as exc:
        error = repr(exc)
        save(out/'failure.json', {'error': error, 'at_epoch': time.time()})
    finally:
        cleanup_end = None
        cleanup_error = None
        try:
            if attempted:
                try:
                    if deadline - time.time() >= 180:
                        save(out/'tasks.json', cloud(['batch','tasks','list','--job='+args.job,
                                                      '--location='+config['region']]))
                    else:
                        save(out/'tasks-skipped.json', {'reason': 'Preserve time for resource cleanup',
                                                       'at_epoch': time.time()})
                except Exception as exc:
                    save(out/'tasks-error.json', {'error': repr(exc)})
                finally:
                    # Delete before storage retrieval; storage latency must never
                    # extend billable match work or bypass resource cleanup.
                    try:
                        cleanup(args.job, config['region'], owner, out, submission_confirmed,
                                campaign_deadline=deadline, owner_label=policy.owner_label)
                    except BaseException as exc:
                        cleanup_error = repr(exc)
                    cleanup_end = time.time()
                try:
                    keys = store.list()
                    cache = out/'raw'; cache.mkdir()
                    def retrieve(key):
                        gcp._key(key)
                        raw = store.get(key); path = cache/key; path.parent.mkdir(parents=True, exist_ok=True)
                        path.write_bytes(raw)
                        return {'key': key, 'bytes': len(raw), 'sha256': digest(raw)}
                    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
                        save(out/'retrieval.json', list(pool.map(retrieve, keys)))
                except BaseException as exc:
                    save(out/'retrieval-error.json', {'error': repr(exc)})
                    error = error or repr(exc)
        finally:
            end = time.time()
            measured_end = cleanup_end or end
            save(out/'resources.json', {'started_epoch': start, 'ended_epoch': end,
                 'allocation_end_epoch': measured_end, 'wall_seconds': end-start,
                 'cleanup_error': cleanup_error,
                 'conservative_vm_hours_upper_bound': None if cleanup_error else 8*(measured_end-start)/3600,
                 'conservative_compute_usd_upper_bound': None if cleanup_error else 8*(measured_end-start)/3600*config['vm_hourly_usd'],
                 'rate_usd_vm_hour': config['vm_hourly_usd'],
                 'note': 'Upper bound through verified cleanup, not actual VM-hours. Unknown if cleanup failed. Obtain lifecycle logs.'})
        if cleanup_error:
            raise RuntimeError('Cleanup requires reconciliation: ' + cleanup_error)
    if error:
        raise RuntimeError(error)
