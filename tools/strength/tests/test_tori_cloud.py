"""MSH-204 (Tori Strength Harness): the cloud campaign manifest is validated."""
import copy
import time
import unittest

from tools.strength import tori_cloud

H = 'a' * 64
SRC = 'b' * 40


def manifest():
    start = time.time()
    return {'schema': 1, 'experiment': 'msh204-test', 'campaign_started_epoch': start,
            'deadline_epoch': start + tori_cloud.CAMPAIGN_SECONDS, 'estimated_compute_before_usd': 0,
            'task_seconds': 3600, 'bins': tori_cloud.campaign_bins(),
            'build': {'source': SRC, 'data_sha256': H,
                      'runtime_image': 'us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:' + H,
                      'engines': {k: {'path': '/opt/engines/' + k, 'sha256': H, 'source': SRC}
                                  for k in tori_cloud.ENGINES}}}


class Manifest(unittest.TestCase):
    def test_reviewed_task_list_is_valid(self):
        tori_cloud.validate_manifest(manifest())
        self.assertEqual(len(tori_cloud.campaign_bins()), 8)
        kinds = [t['kind'] for t in tori_cloud.campaign_tasks()]
        self.assertEqual((kinds.count('pairs'), kinds.count('probe'), kinds.count('depth')), (16, 12, 1))
        control = [s for t in tori_cloud.campaign_tasks() if t.get('label') == 'control' for s in t['seeds']]
        self.assertEqual(sorted(control), list(range(113, 169)))

    def test_bad_manifests_are_refused(self):
        cases = []
        m = manifest(); m['bins'][0][0]['factor'] = 3; cases.append(m)
        m = manifest(); m['bins'][-1][-1]['kind'] = 'shell'; cases.append(m)
        m = manifest(); m['bins'][1][0]['seeds'] = m['bins'][0][0]['seeds']; cases.append(m)
        m = manifest(); m['estimated_compute_before_usd'] = tori_cloud.COMPUTE_USD; cases.append(m)
        m = manifest(); m['build']['engines'].pop('helper'); cases.append(m)
        m = manifest(); m['build']['runtime_image'] = 'worker:latest'; cases.append(m)
        for bad in cases:
            with self.assertRaises(Exception):
                tori_cloud.validate_manifest(copy.deepcopy(bad))


class MoveGeneration(unittest.TestCase):
    def test_msh200_profile_campaign_is_valid_and_bounded(self):
        m = manifest()
        m['experiment'] = 'msh200-movegen'
        m['bins'] = [[{'kind': 'movegen', 'repeats': 200}]]
        tori_cloud.validate_manifest(m)
        for bad in ({'kind': 'movegen', 'repeats': 0}, {'kind': 'movegen', 'repeats': 200, 'shell': 'x'}):
            m['bins'] = [[bad]]
            with self.assertRaises(Exception):
                tori_cloud.validate_manifest(m)
        m['bins'] = [[{'kind': 'movegen', 'repeats': 200}]]
        m['experiment'] = 'msh999-movegen'
        with self.assertRaises(Exception):
            tori_cloud.validate_manifest(m)


class ProofKey(unittest.TestCase):
    def test_python_and_rust_proof_constants_agree(self):
        import re
        from pathlib import Path
        rust = Path('crates/mshogi-runner/examples/tori_strength.rs').read_text()
        bound = int(re.search(r'const FULL_VISITS: u64 = ([0-9_]+);', rust).group(1).replace('_', ''))
        version = re.search(r'const PROVER_VERSION: &str = "([^"]+)";', rust).group(1)
        self.assertEqual((bound, version), (tori_cloud.PROOF_BOUND, tori_cloud.PROVER_VERSION))


if __name__ == '__main__':
    unittest.main()
