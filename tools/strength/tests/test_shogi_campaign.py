"""Offline MSH-190 campaign policy tests; no credentials or jobs required."""
import copy
import tempfile
import time
import unittest
from pathlib import Path
from unittest.mock import Mock, patch

from tools.experiments.core import digest, encode
from tools.strength import shogi_campaign as campaign


def manifest(openings=120):
    engine = {'path': '/opt/engines/engine', 'sha256': 'a' * 64, 'source': 'b' * 40}
    build = {'source': 'b' * 40, 'runtime_image':
             'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:' + 'c' * 64,
             'engines': {n: dict(engine, path='/opt/engines/' + n)
                         for n in ('candidate', 'baseline', 'runner', 'config_writer')}}
    catalog = {'openings': [{'id': str(i), 'sfen': None, 'moves': ['7g7f']} for i in range(openings)]}
    start = time.time()
    return {'schema': 1, 'experiment': 'msh190-test', 'campaign_started_epoch': start,
            'deadline_epoch': start + campaign.POLICY.seconds, 'estimated_compute_before_usd': 0,
            'build': build, 'openings': catalog, 'openings_sha256': digest(encode(catalog)),
            'bins': [[1, 2, 3, 4], [5, 6, 7, 8]],
            'protocol': {'base_ms': 10000, 'increment_ms': 500, 'elo0': 0, 'elo1': 15, 'alpha': .05,
                         'beta': .05, 'min_pairs': 30, 'max_pairs': 100, 'pair_ms': 900000,
                         'concurrency': 2, 'task_seconds': 2400}}


class ManifestTests(unittest.TestCase):
    def test_valid_manifest_and_fixed_deadline(self):
        m = manifest()
        campaign.validate_manifest(m)
        bad = copy.deepcopy(m)
        bad['deadline_epoch'] += 1
        with self.assertRaises(ValueError):
            campaign.validate_manifest(bad)

    def test_openings_never_repeat_and_catalog_is_large(self):
        m = manifest()
        m['protocol']['max_pairs'] = 121
        with self.assertRaises(ValueError):
            campaign.validate_manifest(m)
        with self.assertRaises(ValueError):
            campaign.validate_manifest(manifest(openings=99))
        m = manifest()
        m['openings']['openings'][0]['moves'].append('3c3d')
        with self.assertRaises(ValueError):
            campaign.validate_manifest(m)

    def test_bins_and_bounds(self):
        for bins in ([[1], [1]], [[1], [3]], [[]], [[101]], [[n] for n in range(1, 10)]):
            m = manifest()
            m['bins'] = bins
            with self.assertRaises(ValueError):
                campaign.validate_manifest(m)
        m = manifest()
        m['protocol']['task_seconds'] = 1000
        with self.assertRaises(ValueError):
            campaign.validate_manifest(m)
        for key, value in (('elo1', 30), ('alpha', .1), ('concurrency', 3)):
            m = manifest()
            m['protocol'][key] = value
            with self.assertRaises(ValueError):
                campaign.validate_manifest(m)

    def test_pair_dispatch_uses_opening_index_and_symmetric_caps(self):
        args = campaign.pair_args(manifest(), Path('/openings.json'))
        self.assertEqual((args.base, args.increment, args.max_steps, args.pair_wall_ms), (10, .5, 320, 900000))
        self.assertEqual((args.candidate_nodes, args.baseline_nodes, args.baseline_depth), (5000000, 5000000, 64))
        self.assertEqual(args.opening_count, 120)

    def test_failed_pair_retains_receipt(self):
        with tempfile.TemporaryDirectory() as temp:
            dest = Path(temp) / 'pair'

            def fail(args, seed, out):
                (out / 'pair.log').write_text('diagnostic')
                raise RuntimeError('native replay failed')
            with patch.object(campaign, 'file_hash', return_value='a' * 64), \
                 patch.object(campaign.native_match, 'run_pair', side_effect=fail):
                result = campaign.execute_pair(manifest(), 1, dest)
            self.assertFalse(result['complete'])
            self.assertIn('native replay failed', result['error'])
            self.assertIn('pair.log', result['artifacts'])


class WorkerTests(unittest.TestCase):
    def test_duplicate_task_claim_prevents_execution(self):
        m = manifest()
        source = Mock()
        source.get.return_value = encode(m)
        store = Mock()
        store.put.return_value = False
        with patch.object(campaign.gcp, 'GCSStore', side_effect=[source, store]), \
             patch.dict(campaign.os.environ, {'BATCH_TASK_INDEX': '0', 'BATCH_TASK_RETRY_ATTEMPT': '0'}), \
             patch.object(campaign, 'execute_pair') as run, self.assertRaises(ValueError):
            campaign.worker('gs://test-bucket/manifest', 'gs://test-bucket/run')
        run.assert_not_called()


if __name__ == '__main__':
    unittest.main()
