"""Focused offline checks for the bounded MSH-151 adaptation; no engine launch."""
import copy
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest import mock

import assessment as a


class AssessmentTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        historical = a.HERE.parent/'msh139'
        cls.build = a.read(historical/'verification.json')['build']
        cls.outer = json.loads(gzip.decompress((historical/'pair-002-discovery-oxidation060/report.json.gz').read_bytes()))
        cls.report = json.loads(cls.outer['report_json'])
        cls.config = cls.report['config']

    def audit(self, report):
        outer = copy.deepcopy(self.outer)
        outer['report_json'] = json.dumps(report)
        outer['report_sha256'] = hashlib.sha256(outer['report_json'].encode()).hexdigest()
        return a.audit_report(outer, self.config, self.build, 'oxidation060')

    def test_native_eligible_pair_and_failure_semantics_preserved(self):
        self.assertTrue(self.audit(self.report)['complete'])
        for stop in ['ply_limit', 'engine_failure']:
            r = copy.deepcopy(self.report); r['games'][0]['stop'] = stop
            self.assertFalse(self.audit(r)['complete'])
            self.assertIsNone(self.audit(r)['pair_score'])
        r = copy.deepcopy(self.report); r['games'][0]['engines'][0]['cleanup_failures'] = ['failed']
        self.assertFalse(self.audit(r)['healthy'])

    def test_configuration_identity_color_and_time_are_still_checked(self):
        changes = [lambda r:r['config'].update(max_plies=1),
                   lambda r:r['engine_identities'][0]['executable'].update(sha256='changed'),
                   lambda r:r['games'][0].update(engines_by_color=[1, 0]),
                   lambda r:next(e for e in r['games'][0]['transcript']['entries']
                       if e['direction']=='command' and e['text'].startswith('go')).update(text='go depth 2')]
        for change in changes:
            r = copy.deepcopy(self.report); change(r)
            with self.assertRaises(ValueError):self.audit(r)

    def test_one_aggregate_budget_counts_failures_smokes_and_extra_work(self):
        ledger = {'runs':[{'seconds':300, 'complete':False}, {'seconds':25, 'phase':'smoke'}],
                  'resource_extra_seconds':15}
        self.assertEqual(a.resource_remaining(ledger), 560)
        self.assertEqual(a.admit_resource(ledger), 560)
        ledger['runs'].append({'seconds':341})
        with self.assertRaisesRegex(ValueError, 'aggregate budget'):a.admit_resource(ledger)
        ledger['resource_cap_seconds'] = 340
        self.assertEqual(a.resource_remaining(ledger), -341)
        with self.assertRaises(ValueError):a.admit_resource(ledger, smoke=True)

    def test_invalid_or_missing_accounting_is_not_free_work(self):
        for value in [-1, float('nan'), float('inf'), True]:
            with self.assertRaises(ValueError):a.resource_remaining({'runs':[], 'resource_extra_seconds':value})
        with self.assertRaises(KeyError):a.resource_remaining({'runs':[{}]})

    def test_preparation_inflight_and_unresolved_failures_block_launch(self):
        with self.assertRaisesRegex(ValueError, 'engineering acceptance'):
            a.invoke({'runs':[]}, 'unused', 'discovery', 'test')
        with self.assertRaisesRegex(ValueError, 'Unreconciled'):
            a.invoke({'inflight':{'process_group':123}, 'runs':[]}, 'unused', 'discovery', 'test')
        with self.assertRaisesRegex(ValueError, 'incomplete pair'):
            a.invoke({'runs':[{'phase':'discovery', 'complete':False}]}, 'unused', 'discovery', 'test')

    def test_fresh_population_excludes_all_attempts_before_sampling(self):
        with tempfile.TemporaryDirectory() as folder:
            p = Path(folder); source = p/'old.json.gz'; history = p/'old-ledger.json'; target = p/'fresh.json.gz'
            roots = [{'id':str(i), 'moves':[], 'phase':'discovery' if i < 3 else 'confirmation'} for i in range(6)]
            source.write_bytes(gzip.compress(json.dumps(roots).encode(), mtime=0))
            a.save(history, {'book':{'roots_sha256':a.digest(source)},
                             'runs':[{'opening':'0','phase':'smoke','complete':False},
                                     {'opening':'3','phase':'confirmation','complete':False}]})
            with self.assertRaisesRegex(ValueError, 'historical evidence'):
                a.fresh_book(source, history, source)
            result = a.fresh_book(source, history, target)
            fresh = json.loads(gzip.decompress(target.read_bytes()))
            self.assertEqual(result['excluded_prior_root_ids'], ['0','3'])
            self.assertEqual(result['phases'], {'discovery':2,'confirmation':2})
            self.assertEqual(fresh, [r for r in roots if r['id'] not in ['0','3']])
            for phase in ['discovery','confirmation']:
                for ordinal in range(20):
                    self.assertNotIn(a.draw_root(fresh, phase, 'opponent', ordinal)['id'], ['0','3'])
            source.write_bytes(b'changed')
            with self.assertRaisesRegex(ValueError, 'population changed'):
                a.fresh_book(source, history, target)

    def test_inherited_exclusions_and_all_new_attempts_are_preserved(self):
        with tempfile.TemporaryDirectory() as folder:
            p = Path(folder); source = p/'filtered.json.gz'; history = p/'ledger.json'; target = p/'fresh.json.gz'
            roots = [{'id':str(i), 'moves':[], 'phase':'discovery' if i < 4 else 'confirmation'} for i in range(2, 6)]
            source.write_bytes(gzip.compress(json.dumps(roots).encode(), mtime=0))
            a.save(history, {'book':{'roots_sha256':a.digest(source), 'excluded_prior_root_ids':['0','1']},
                             'runs':[{'opening':'2','phase':'smoke','complete':False},
                                     {'opening':'4','phase':'confirmation','complete':False}]})
            result = a.fresh_book(source, history, target)
            self.assertEqual(result['inherited_excluded_root_ids'], ['0','1'])
            self.assertEqual(result['newly_excluded_root_ids'], ['2','4'])
            self.assertEqual(result['excluded_prior_root_ids'], ['0','1','2','4'])
            self.assertEqual(result['parent_roots_sha256'], a.digest(source))
            self.assertEqual(result['historical_ledger_sha256'], a.digest(history))
            self.assertEqual(result['phases'], {'discovery':1,'confirmation':1})
            fresh = json.loads(gzip.decompress(target.read_bytes()))
            self.assertEqual(fresh, [root for root in roots if root['id'] in ['3','5']])
            for phase, expected in [('discovery','3'), ('confirmation','5')]:
                for ordinal in range(5):
                    self.assertEqual(a.draw_root(fresh, phase, 'reference', ordinal)['id'], expected)

    def test_reintroduced_inherited_root_is_rejected_before_publication(self):
        with tempfile.TemporaryDirectory() as folder:
            p = Path(folder); source = p/'invalid.json.gz'; history = p/'ledger.json'; target = p/'fresh.json.gz'
            roots = [{'id':str(i), 'moves':[], 'phase':'discovery' if i < 2 else 'confirmation'} for i in range(4)]
            source.write_bytes(gzip.compress(json.dumps(roots).encode(), mtime=0))
            # Even an internally consistent source digest cannot authorize
            # silently undoing a prior campaign's recorded exclusions.
            a.save(history, {'book':{'roots_sha256':a.digest(source), 'excluded_prior_root_ids':['0']}, 'runs':[]})
            with self.assertRaisesRegex(ValueError, 'Inherited excluded roots reintroduced'):
                a.fresh_book(source, history, target)
            self.assertFalse(target.exists())

    def test_old_new_posteriors_use_independent_streams(self):
        scores = [0,.25,.5,.75,1]
        result = a.posterior_change(scores, scores, draws=20000)
        lo, hi = result['conditional_index_change_credible95']
        self.assertLess(lo, -100); self.assertGreater(hi, 100)
        self.assertAlmostEqual(result['probability_positive_conditional_change'], .5, delta=.02)
        self.assertEqual(result, a.posterior_change(scores, scores, draws=20000))
        better = a.posterior_change([0]*8, [1]*8, draws=20000)
        self.assertGreater(better['conditional_index_change_credible95'][0], 0)
        with self.assertRaises(ValueError):a.posterior_change([], scores)
        with self.assertRaises(ValueError):a.posterior_change([.1], scores)

    def test_historical_delta_requires_same_binary_options_args_and_anchor(self):
        ref = {'source':'release', 'sha256':'executable', 'options':[], 'args':[]}
        baseline = {'build':{'references':{'x':ref}}, 'catalog':{'x':{'rating':1943}},
                    'runs':[{'opponent':'x','phase':'confirmation','complete':True,'pair_score':.5}]}
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder)/'baseline.json'; a.save(path, baseline)
            ledger = copy.deepcopy(baseline)
            ledger['baseline'] = {'path':str(path), 'sha256':a.digest(path)}
            with mock.patch.object(a, 'posterior', side_effect=lambda *args:{}), \
                 mock.patch.object(a, 'posterior_change', side_effect=lambda *args:{}) as delta:
                result = a.analysis(ledger)['x/confirmation']
                self.assertIn('historical_confirmation_comparison', result)
                self.assertEqual(delta.call_count, 3)  # main prior plus sensitivity
                cases = [
                    ('reference_source', lambda x:x['build']['references']['x'].update(source='other')),
                    ('reference_executable_sha256', lambda x:x['build']['references']['x'].update(sha256='other')),
                    ('reference_options', lambda x:x['build']['references']['x'].update(options=[['Hash','32']])),
                    ('reference_args', lambda x:x['build']['references']['x'].update(args=['--different'])),
                    ('rated_anchor', lambda x:x['catalog']['x'].update(rating=2000)),
                    ('reference_executable_sha256', lambda x:x['build']['references']['x'].pop('sha256')),
                ]
                for expected, mutate in cases:
                    with self.subTest(condition=expected):
                        changed = copy.deepcopy(ledger); mutate(changed); delta.reset_mock()
                        result = a.analysis(changed)['x/confirmation']
                        self.assertNotIn('historical_confirmation_comparison', result)
                        omission = result['historical_confirmation_comparison_omitted']
                        self.assertIn(expected, [d['condition'] for d in omission['differences']])
                        self.assertIn('no automatic historical delta', omission['reason'])
                        delta.assert_not_called()
            # Missing arguments mean the same empty command line, not a change.
            ledger['build']['references']['x'].pop('args')
            self.assertEqual(a.comparison_condition_differences(baseline, ledger, 'x'), [])

    def test_remaining_budget_bounds_process_and_records_timeout_unscored(self):
        # A fake process/clock exercises admission and cleanup without launching
        # any engine. Simulate consumption of the entire allowed search window.
        with tempfile.TemporaryDirectory() as folder:
            p = Path(folder); book = p/'roots.json.gz'
            book.write_bytes(gzip.compress(json.dumps([{'id':'fresh','moves':[], 'phase':'discovery'}]).encode()))
            ledger = {'engineering_accepted':True, 'runs':[], 'resource_extra_seconds':100,
                      'build':{'candidate':{'path':'candidate','model_identity':'current-eval'},
                               'runner':{'path':'runner'},
                               'references':{'ref':{'path':'ref','name':'Ref','source':'hash','options':[]}}},
                      'freeze':{'code_sha256':'digest','method_sha256':'digest'},
                      'book':{'path':str(book),'roots_sha256':'digest'}}
            clock = [100.0]; timeouts = []
            child = mock.Mock(pid=1234, returncode=-9)
            def communicate(timeout):
                timeouts.append(timeout); clock[0] += timeout
                raise subprocess.TimeoutExpired('fake', timeout)
            child.communicate.side_effect = communicate
            def cleanup(process, deadline):
                self.assertIs(process, child)
                self.assertLessEqual(deadline, 900)
                clock[0] += 3
                return (b'', b''), True
            with mock.patch.object(a, 'HERE', p), mock.patch.object(a, 'verify_build'), \
                 mock.patch.object(a, 'digest', return_value='digest'), \
                 mock.patch.object(a.time, 'monotonic', side_effect=lambda:clock[0]), \
                 mock.patch.object(a.subprocess, 'Popen', return_value=child) as launch, \
                 mock.patch.object(a, 'terminate', side_effect=cleanup):
                receipt = a.invoke(ledger, 'ref', 'discovery', 'offline test')
            launch.assert_called_once()
            self.assertEqual(timeouts, [760])  # 900-100extra-40reserved
            self.assertTrue(receipt['budget_limited_timeout'])
            self.assertFalse(receipt['complete']); self.assertIsNone(receipt['pair_score'])
            self.assertTrue(receipt['process_group_settled']); self.assertIsNone(ledger['inflight'])
            self.assertEqual(ledger['resource_total_seconds'], 863)
            self.assertLessEqual(ledger['resource_total_seconds'], 900)
            with self.assertRaisesRegex(ValueError, 'incomplete pair'):
                a.invoke(ledger, 'ref', 'discovery', 'must not replace silently')


if __name__ == '__main__':unittest.main()
