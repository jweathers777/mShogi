import unittest
from tools.experiments import core, quick_status
from tools.experiments.tests import test_core


class Records:
    def __init__(self): self.data = {}; self.reads = []
    def list(self, prefix): return [k for k in self.data if k.startswith(prefix)]
    def get(self, key):
        self.reads.append(key)
        if '/artifacts/' in key: raise AssertionError('Artifact read')
        return core.encode(self.data[key])
    def put(self, key, value): raise AssertionError('Status must not write')


class QuickStatusTests(unittest.TestCase):
    def setUp(self):
        test_core.CoreTests.setUp(self)
        self.records = Records()
        self.screen = core.seal(dict(self.job, kind='reference_screening', nodes=500000,
            inputs={'report': {'job': self.job['id'], 'path': 'report.json.gz'}}))
        self.manifest = {'schema': 1, 'experiment': 'test', 'jobs': [self.job, self.screen],
                         'pipelines': [self.job['id']], 'automatic_confirmations': True}

    def complete(self, job, attempt='success'):
        base = core.prefix(job)
        r = {'job': job, 'attempt': attempt, 'status': 'complete'}
        self.records.data[base+'/success.json'] = r
        self.records.data[base+'/attempts/'+attempt+'/receipt.json'] = r
        self.records.data[base+'/attempts/'+attempt+'/artifacts/report.json'] = 'never read'

    def test_pending_and_unknown_future_confirmations(self):
        r = quick_status.snapshot(self.manifest, self.records)
        self.assertEqual((0, 2), (r['games_complete'], r['games_total']))
        self.assertEqual((0, 1), (r['confirmation_plans_published'], r['confirmation_plans_expected']))
        self.assertEqual([], self.records.reads)
        self.assertIn('0/0 known', quick_status.format_report(r))

    def test_derived_progress_failed_retry_and_no_artifact_reads(self):
        self.complete(self.job); self.complete(self.screen)
        confirm = core.seal(dict(self.screen, kind='confirmation', nodes=5000000,
            selection={'screen_job': self.screen['id']}, move='e2e4', game=0, ply=0))
        other = core.seal(dict(confirm, move='d2d4'))
        self.records.data[core.prefix(self.screen)+'/derived-jobs.json'] = [confirm, other]
        self.complete(confirm)
        self.records.data[core.prefix(confirm)+'/attempts/failed/receipt.json'] = {'job':confirm,'status':'failed'}
        self.records.data[core.prefix(other)+'/claim.json'] = {}
        # A success wins over a stale claim, without claiming process liveness.
        self.records.data[core.prefix(confirm)+'/claim.json'] = {}
        r = quick_status.snapshot(self.manifest, self.records)
        self.assertEqual(2, r['games_complete'])
        self.assertEqual(1, r['completed']['confirmation'])
        self.assertEqual(2, r['known_totals']['confirmation'])
        self.assertEqual(1, r['failed_attempts'])
        self.assertEqual(1, r['claims_without_success'])
        self.assertEqual(3, len(self.records.reads))

    def test_validation_probe_excluded_from_study_counts(self):
        probe = core.seal(dict(self.job, validation_probe='test'))
        self.complete(probe)
        r = quick_status.snapshot(self.manifest, self.records)
        self.assertEqual(0, r['games_complete'])
        self.assertEqual(1, r['unlisted_completion_records'])

    def test_completion_counts_markers_without_reading_success_contents(self):
        self.complete(self.job)
        self.records.data[core.prefix(self.job)+'/success.json']['status'] = 'failed'
        r = quick_status.snapshot(self.manifest,self.records)
        self.assertEqual(2, r['games_complete'])
        self.assertEqual([], self.records.reads)
        self.assertIn('success records and artifacts not checked', r['verification'])

    def test_read_error_is_not_reported_as_zero_progress(self):
        self.records.data[core.prefix(self.screen)+'/derived-jobs.json'] = []
        self.records.get = lambda key: (_ for _ in ()).throw(PermissionError('denied'))
        with self.assertRaises(PermissionError):quick_status.snapshot(self.manifest,self.records)

    def test_foreign_confirmation_plan_rejected(self):
        confirm = core.seal(dict(self.screen,kind='confirmation',nodes=5000000,
                               selection={'screen_job':'wrong'},game=0,ply=0,move='e2e4'))
        self.records.data[core.prefix(self.screen)+'/derived-jobs.json'] = [confirm]
        with self.assertRaisesRegex(ValueError,'Invalid saved'):quick_status.snapshot(self.manifest,self.records)
