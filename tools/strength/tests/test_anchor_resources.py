"""Offline audit operation fixtures with independently known durations."""
import copy
import json
from pathlib import Path
import tempfile
import unittest

from tools.strength.anchor_resources import analyze, collect, sha


UID = 'msh185-test-unique'


def event(method, operation, second, *, first=False, last=False, code=0, vm='1'):
    return {'insertId': f'{operation}-{second}', 'timestamp': f'2026-11-16T12:{second//60:02d}:{second%60:02d}Z',
            'operation': {'id': operation, 'first': first, 'last': last},
            'resource': {'labels': {'instance_id': vm, 'zone': 'us-central1-a'}},
            'protoPayload': {'methodName': 'v1.compute.instances.' + method,
                             'resourceName': f'projects/123/zones/us-central1-a/instances/{UID}-group0-{vm}',
                             'status': {'code': code} if code else {}}}


def lifecycle():
    return [event('insert', 'i1', 0, first=True), event('insert', 'i1', 10, last=True),
            event('delete', 'd1', 110, first=True), event('delete', 'd1', 120, last=True)]


def worker():
    return {'id': '1', 'machine-type': 'projects/123/machineTypes/c3-standard-4',
            'cpu-platform': 'Intel Sapphire Rapids', 'zone': 'projects/123/zones/us-central1-a'}


class ResourceTests(unittest.TestCase):
    def test_known_two_minute_vm_duration_and_cost(self):
        result = analyze(lifecycle(), UID, [worker()], .2, True)
        self.assertTrue(result['complete'])
        self.assertEqual(result['vms'][0]['seconds'], 120)
        self.assertAlmostEqual(result['vm_lifecycle_hours'], 1/30)
        self.assertAlmostEqual(result['estimated_compute_usd'], 1/150)
        self.assertIsNone(result['actual_billed_cost'])

    def test_failed_allocation_is_not_vm_runtime(self):
        logs = [event('insert', 'failed', 0, first=True, vm='2'),
                event('insert', 'failed', 10, last=True, code=8, vm='2')] + lifecycle()
        result = analyze(logs, UID, [worker()], .2, True)
        self.assertTrue(result['complete'])
        self.assertEqual(result['failed_insert_completion_count'], 1)
        self.assertEqual(result['successful_vm_count'], 1)
        self.assertEqual(result['failed_insert_zones'], {'us-central1-a': 1})

    def test_missing_delete_never_becomes_zero_cost(self):
        result = analyze(lifecycle()[:2], UID, [worker()], .2, True)
        self.assertFalse(result['complete'])
        self.assertIsNone(result['estimated_compute_usd'])

    def test_missing_insert_detected_without_worker_receipt(self):
        result = analyze(lifecycle()[2:], UID, [], .2, True)
        self.assertFalse(result['complete'])
        self.assertIsNone(result['vm_lifecycle_hours'])

    def test_failed_start_does_not_extend_later_success(self):
        logs = [event('insert', 'failed', 0, first=True),
                event('insert', 'failed', 5, last=True, code=8),
                event('insert', 'good', 20, first=True),
                event('insert', 'good', 30, last=True),
                event('delete', 'gone', 120, last=True)]
        result = analyze(logs, UID, [worker()], .2, True)
        self.assertTrue(result['complete'])
        self.assertEqual(result['vms'][0]['seconds'], 100)

    def test_foreign_job_and_mismatched_vm_rejected(self):
        logs = lifecycle()
        logs[0]['protoPayload']['resourceName'] = 'projects/123/instances/some-other-job'
        with self.assertRaises(ValueError):
            analyze(logs, UID, [], .2, True)
        logs = lifecycle()
        logs[-1]['resource']['labels']['instance_id'] = '9'
        self.assertFalse(analyze(logs, UID, [worker()], .2, True)['complete'])

    def test_duplicates_do_not_double_account(self):
        logs = lifecycle()
        result = analyze(logs + copy.deepcopy(logs), UID, [worker()], .2, True)
        self.assertTrue(result['complete'])
        self.assertEqual(result['successful_vm_count'], 1)

    def test_cleanup_unknown_or_empty_audit_is_incomplete(self):
        self.assertFalse(analyze(lifecycle(), UID, [worker()], .2, False)['complete'])
        self.assertFalse(analyze([], UID, [], .2, True)['complete'])

    def test_failed_allocation_delete_not_found_is_retained_as_notice(self):
        missing = event('delete', 'missing', 20, code=5, vm='2')
        del missing['operation']
        result = analyze(lifecycle() + [missing], UID, [worker()], .2, True)
        self.assertTrue(result['complete'])
        self.assertEqual(result['notices'][0]['kind'], 'delete_already_absent')

    def test_later_reconciliation_preserves_original_failure_and_hash(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            name = 'projects/mshogi/locations/us-central1/jobs/msh185-test'
            original = json.dumps({'verified_absent': False, 'at_epoch': 1794830520}).encode()
            (root/'cleanup.json').write_bytes(original)
            (root/'submission.json').write_text(json.dumps({'name': name, 'uid': UID,
                                                          'createTime': '2026-11-16T12:00:00Z'}))
            (root/'resources.json').write_text(json.dumps({'rate_usd_vm_hour': .2,
                                                         'conservative_compute_usd_upper_bound': None}))
            reconciliation = {'job_name': name, 'job_uid': UID,
                              'original_cleanup_sha256': sha(original), 'at_epoch': 1794830580,
                              'verified_absent': True, 'remaining_jobs': [], 'remaining_vms': [],
                              'remaining_disks': []}
            (root/'reconciliation.json').write_text(json.dumps(reconciliation))
            (root/'audit.json').write_text(json.dumps(lifecycle()))
            result = collect(root, root/'result', root/'audit.json', root/'reconciliation.json')
            self.assertTrue(result['complete'])
            self.assertFalse(result['original_cleanup_verified_absent'])
            self.assertEqual(result['original_cleanup_sha256'], sha(original))
            self.assertEqual((root/'cleanup.json').read_bytes(), original)
            self.assertIsNone(result['controller_conservative_compute_usd_upper_bound'])
            reconciliation['job_uid'] = 'another-job'
            (root/'bad-reconciliation.json').write_text(json.dumps(reconciliation))
            with self.assertRaises(ValueError):
                collect(root, root/'bad-result', root/'audit.json', root/'bad-reconciliation.json')


if __name__ == '__main__':
    unittest.main()
