"""Read-only MSH-185 VM lifecycle accounting after controller cleanup.

Reuse MSH-161's insert-first to successful delete-last allocation interval.
This includes startup/idle/cleanup; it is not engine CPU time or billed usage.
No submission, deletion, retries or monitoring. One explicit logging read per run.
Example: python3 -m tools.strength.anchor_resources --run target/msh185/runs/b01
         --out target/msh185/resources/b01
Use --audit-logs SAVED_JSON for a wholly offline recheck.
"""
import argparse
from collections import Counter, defaultdict
from datetime import datetime, timezone
import hashlib
import json
import math
from pathlib import Path
import re
import subprocess


def epoch(value):
    return datetime.fromisoformat(value.replace('Z', '+00:00')).timestamp()


def utc(value):
    return datetime.fromtimestamp(value, timezone.utc).isoformat().replace('+00:00', 'Z')


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def write(path, value):
    with Path(path).open('x') as stream:
        json.dump(value, stream, indent=2, allow_nan=False)
        stream.write('\n')


def analyze(logs, job_uid, workers, hourly_rate, cleanup_verified):
    """Derive allocation intervals; incomplete evidence never becomes zero cost."""
    if not re.fullmatch(r'[a-z0-9-]+', job_uid):
        raise ValueError('Invalid job UID')
    if not math.isfinite(hourly_rate) or hourly_rate <= 0:
        raise ValueError('Positive finite hourly rate required')
    operations = defaultdict(list)
    seen, issues, notices = set(), [], []
    for row in logs:
        payload = row.get('protoPayload', {})
        method = payload.get('methodName', '')
        if method not in ('v1.compute.instances.insert', 'v1.compute.instances.delete'):
            raise ValueError('Unexpected audit method')
        resource = payload.get('resourceName', '')
        if not resource.rsplit('/', 1)[-1].startswith(job_uid + '-group'):
            raise ValueError('Audit resource is outside the exact job UID')
        operation = row.get('operation', {})
        operation_id = operation.get('id')
        if not operation_id:
            status = payload.get('status') or {}
            if method.endswith('.delete') and status.get('code') == 5:
                notices.append({'resource': resource, 'timestamp': row['timestamp'],
                                'kind': 'delete_already_absent', 'status': status})
            else:
                issues.append('Missing operation ID: ' + resource)
            continue
        key = (resource, method, operation_id)
        identity = (key, row.get('insertId'), row['timestamp'], bool(operation.get('first')),
                    bool(operation.get('last')))
        if identity in seen:
            continue
        seen.add(identity)
        operations[key].append(row)
    inserts, deletes, failures = [], defaultdict(list), []
    for (resource, method, operation_id), rows in sorted(operations.items()):
        first = [r for r in rows if r['operation'].get('first')]
        last = [r for r in rows if r['operation'].get('last')]
        if len(last) != 1:
            issues.append('Missing or ambiguous completion: ' + operation_id)
            continue
        terminal = last[0]
        status = terminal['protoPayload'].get('status') or {}
        failed = status.get('code', 0) != 0
        if method.endswith('.insert') and failed:
            failures.append({'operation': operation_id, 'resource': resource,
                             'at': terminal['timestamp'], 'status': status,
                             'zone': terminal.get('resource', {}).get('labels', {}).get('zone')})
            continue
        if failed:
            issues.append('Delete failed: ' + operation_id)
            continue
        if method.endswith('.delete'):
            deletes[resource].append(terminal)
            continue
        if len(first) != 1:
            issues.append('Missing or ambiguous insert start: ' + operation_id)
            continue
        inserts.append((resource, first[0], terminal, operation_id))
    vms = []
    for resource in set(deletes) - {item[0] for item in inserts}:
        issues.append('Deletion lacks successful insertion: ' + resource)
    for resource, start, inserted, operation_id in inserts:
        candidates = [r for r in deletes[resource] if epoch(r['timestamp']) >= epoch(inserted['timestamp'])]
        instance_id = inserted.get('resource', {}).get('labels', {}).get('instance_id')
        if len(candidates) != 1 or not instance_id:
            issues.append('Missing or ambiguous deletion/instance ID: ' + resource)
            continue
        end = candidates[0]
        delete_id = end.get('resource', {}).get('labels', {}).get('instance_id')
        if delete_id != instance_id:
            issues.append('Insert/delete instance IDs differ: ' + resource)
            continue
        seconds = epoch(end['timestamp']) - epoch(start['timestamp'])
        if seconds < 0:
            raise ValueError('Negative VM duration')
        vms.append({'id': instance_id, 'name': resource, 'insert_operation': operation_id,
                    'start': start['timestamp'], 'deleted': end['timestamp'], 'seconds': seconds,
                    'zone': inserted.get('resource', {}).get('labels', {}).get('zone')})
    vm_ids = {vm['id'] for vm in vms}
    if len(vm_ids) != len(vms):
        issues.append('Duplicate successful VM identity')
    for worker in workers:
        if worker.get('id') not in vm_ids:
            issues.append('Worker lacks a complete VM lifecycle: ' + str(worker.get('id')))
        if worker.get('machine-type', '').rsplit('/', 1)[-1] != 'c3-standard-4':
            issues.append('Unexpected worker machine type: ' + str(worker.get('id')))
        if not worker.get('cpu-platform') or not worker.get('zone', '').rsplit('/', 1)[-1].startswith('us-central1-'):
            issues.append('Missing CPU or wrong worker region: ' + str(worker.get('id')))
    if not logs:
        issues.append('No audit evidence; zero allocation is not established')
    if not cleanup_verified:
        issues.append('Controller cleanup was not verified')
    known_hours = sum(vm['seconds'] for vm in vms) / 3600
    return {'complete': not issues, 'issues': issues, 'vms': vms, 'notices': notices,
            'observed_worker_count': len({w.get('id') for w in workers}), 'workers': workers,
            'vm_ids_without_worker_environment': sorted(vm_ids - {w.get('id') for w in workers}),
            'successful_vm_count': len(vms), 'failed_insert_completion_count': len(failures),
            'failed_insert_zones': dict(Counter(f['zone'] for f in failures)), 'failed_insertions': failures,
            'known_complete_lifecycle_hours': known_hours,
            'vm_lifecycle_hours': known_hours if not issues else None,
            'estimated_compute_usd': known_hours * hourly_rate if not issues else None,
            'rate_usd_vm_hour': hourly_rate, 'actual_billed_cost': None,
            'note': 'Measured insert-start to delete-completion allocation envelope, not billed VM time. '
                    'Compute estimate excludes storage, registry, network and other charges. '
                    'Do not add task/engine CPU time to these overlapping VM intervals.'}


def collect(run, output, audit_logs=None, cleanup_receipt=None):
    run, output = Path(run), Path(output)
    output.mkdir(parents=True, exist_ok=False)
    submission = json.loads((run/'submission.json').read_text())
    original_cleanup_raw = (run/'cleanup.json').read_bytes()
    original_cleanup = json.loads(original_cleanup_raw)
    cleanup_raw = Path(cleanup_receipt).read_bytes() if cleanup_receipt else original_cleanup_raw
    cleanup = json.loads(cleanup_raw)
    if cleanup_receipt:
        if cleanup.get('job_uid') != submission['uid'] or cleanup.get('job_name') != submission['name']:
            raise ValueError('Reconciliation receipt must bind exact submission job_name and job_uid')
        if cleanup.get('original_cleanup_sha256') != sha(original_cleanup_raw):
            raise ValueError('Reconciliation receipt must bind original_cleanup_sha256')
        if cleanup.get('at_epoch', 0) < original_cleanup.get('at_epoch', 0):
            raise ValueError('Reconciliation must be later than original cleanup')
        if any(cleanup.get(field) != [] for field in ('remaining_jobs', 'remaining_vms', 'remaining_disks')):
            raise ValueError('Reconciliation must explicitly show no jobs, VMs or disks')
    resources = json.loads((run/'resources.json').read_text())
    if not cleanup.get('verified_absent'):
        raise ValueError('Wait for verified controller cleanup before resource collection')
    uid = submission['uid']
    if not re.fullmatch(r'msh185-[a-z0-9-]+', uid):
        raise ValueError('Unexpected study job UID')
    start = epoch(submission['createTime']) - 300
    end = cleanup['at_epoch'] + 300
    query = ('resource.type="gce_instance" AND protoPayload.serviceName="compute.googleapis.com" '
             'AND (protoPayload.methodName="v1.compute.instances.insert" OR '
             'protoPayload.methodName="v1.compute.instances.delete") '
             f'AND protoPayload.resourceName:"/instances/{uid}-group" '
             f'AND timestamp>="{utc(start)}" AND timestamp<="{utc(end)}"')
    command = ['gcloud', 'logging', 'read', query, '--project=mshogi', '--format=json',
               '--order=asc', '--limit=10001', '--quiet']
    write(output/'query.json', {'command': command, 'job': submission['name'], 'uid': uid,
                              'offline_input': str(audit_logs) if audit_logs else None})
    if audit_logs:
        raw = Path(audit_logs).read_bytes()
    else:
        response = subprocess.run(command, capture_output=True, timeout=180, check=True)
        raw = response.stdout
    (output/'audit-logs.json').write_bytes(raw)
    logs = json.loads(raw)
    if not isinstance(logs, list) or len(logs) >= 10001:
        raise ValueError('Audit query reached limit or returned wrong shape; preserve and reconcile')
    workers = {}
    environments = {}
    receipt_hashes = {}
    for path in sorted((run/'raw'/'tasks').glob('*/receipt.json')):
        receipt_raw = path.read_bytes()
        receipt_hashes[str(path.relative_to(run))] = sha(receipt_raw)
        environment = json.loads(receipt_raw).get('environment', {})
        environments[path.parent.name] = environment
        data = environment.get('gcp')
        if data:
            previous = workers.setdefault(data['id'], data)
            if previous != data:
                raise ValueError('Conflicting environment for one VM')
    result = analyze(logs, uid, list(workers.values()), resources['rate_usd_vm_hour'], True)
    result.update(job_name=submission['name'], job_uid=uid, audit_sha256=sha(raw),
                  original_cleanup_sha256=sha(original_cleanup_raw),
                  original_cleanup_verified_absent=original_cleanup.get('verified_absent'),
                  effective_cleanup_sha256=sha(cleanup_raw),
                  cleanup_reconciliation_path=str(cleanup_receipt) if cleanup_receipt else None,
                  cleanup_note='Explicit later reconciliation; original failure preserved' if cleanup_receipt else 'Original controller receipt',
                  task_receipt_hashes=receipt_hashes,
                  task_environments=environments,
                  requested_allocation=submission.get('allocationPolicy'),
                  container_images=[r.get('container', {}).get('imageUri')
                                    for group in submission.get('taskGroups', [])
                                    for r in group.get('taskSpec', {}).get('runnables', [])],
                  controller_conservative_vm_hours_upper_bound=resources.get('conservative_vm_hours_upper_bound'),
                  controller_conservative_compute_usd_upper_bound=resources.get('conservative_compute_usd_upper_bound'),
                  controller_resources_sha256=sha((run/'resources.json').read_bytes()))
    write(output/'resources.json', result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--run', required=True)
    parser.add_argument('--out', required=True)
    parser.add_argument('--audit-logs')
    parser.add_argument('--cleanup-receipt', help='Later exact-job reconciliation; original receipts stay unchanged')
    args = parser.parse_args()
    result = collect(args.run, args.out, args.audit_logs, args.cleanup_receipt)
    print(json.dumps({k: result[k] for k in ('complete', 'vm_lifecycle_hours', 'estimated_compute_usd',
                                          'failed_insert_completion_count', 'issues')}))
    if not result['complete']:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
