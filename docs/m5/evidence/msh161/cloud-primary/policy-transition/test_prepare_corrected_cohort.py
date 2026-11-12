"""Offline admission tests; no engines, Git mutations, or network calls."""
import copy
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import unittest

SCRIPT = Path(__file__).with_name('prepare_corrected_cohort.py')
spec = importlib.util.spec_from_file_location('corrected_cohort', SCRIPT)
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


class CohortTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / 'repo'
        primary = m.REPO / m.PRIMARY
        cohort = m.read(primary / 'cohort-freeze.json')
        names = {m.PRIMARY / 'cohort-freeze.json', m.PRIMARY / 'cloud-config.json'}
        for entry in cohort['batches']:
            names.add(Path(entry['manifest']))
            manifest = m.read(m.REPO / entry['manifest'])
            names.update(Path(n) for n in manifest['jobs'][0]['source_files'])
        names.update(p.relative_to(m.REPO) for p in (m.REPO / 'tools/experiments').glob('*.py'))
        for name in names:
            destination = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(m.REPO / name, destination)
        old = m.read(self.root / m.PRIMARY / 'batch-03-manifest.json')
        self.old = old
        self.build = copy.deepcopy(old['jobs'][0]['build'])
        self.build['source'] = '1' * 40
        for name in ('candidate', 'runner'):
            self.build['engines'][name]['source'] = '1' * 40
            self.build['engines'][name]['sha256'] = '2' * 64
        self.build_path = self.root / 'new-build.json'
        m.save(self.build_path, self.build)
        self.amendment = self.root / 'amendment.json'
        m.save(self.amendment, {'schema': 'msh161-automatic-thresholds-v1',
               'claim_policy': 'explicit_only', 'uci_optional_claims_available': False,
               'native_source': '1' * 40,
               'original_study_freeze_sha256': old['jobs'][0]['study_freeze_sha256']})
        self.image = 'registry/image@sha256:' + '3' * 64
        self.output = self.root / 'output'

    def prepare(self, **kwargs):
        return m.prepare(self.root, self.build_path, self.amendment, kwargs.pop('image', self.image),
                         '4' * 40, self.output, **kwargs)

    def rewrite(self, path, value):
        path.write_text(json.dumps(value))

    def test_exact_coverage_dependency_resealing_and_original_preservation(self):
        receipt = self.prepare()
        self.assertEqual((receipt['pairs'], receipt['games']), (80, 160))
        all_ids = set()
        for index in range(3, 11):
            name = f'batch-{index:02d}-manifest.json'
            original_path = self.root / m.PRIMARY / name
            self.assertEqual(original_path.read_bytes(), (self.output / 'originals' / name).read_bytes())
            old, new = m.read(original_path), m.read(self.output / name)
            m.validate_manifest(new)
            self.assertEqual(new['experiment'], f'msh161-primary-v2-batch-{index:02d}')
            for before, after in zip(old['jobs'], new['jobs']):
                self.assertEqual(before['spec'], after['spec'])
                self.assertEqual(before['environment']['cloud']['config'], after['environment']['cloud']['config'])
                self.assertEqual(before['expected_outputs'], after['expected_outputs'])
                self.assertNotEqual(before['id'], after['id'])
                self.assertEqual(after['claim_policy'], 'explicit_only')
                self.assertEqual(after['study_amendment']['sha256'], m.sha(self.amendment))
                if after['kind'] == 'game_pair':
                    all_ids.add(after['spec']['id'])
        self.assertEqual(len(all_ids), 80)
        with self.assertRaisesRegex(ValueError, 'already exist'):
            self.prepare()

    def test_wrong_original_hash_rejected_before_write(self):
        path = self.root / m.PRIMARY / 'batch-06-manifest.json'
        path.write_text(path.read_text() + ' ')
        with self.assertRaisesRegex(ValueError, 'Original manifest changed'):
            self.prepare()
        self.assertFalse(self.output.exists())

    def test_frozen_helper_change_rejected(self):
        path = self.root / m.STUDY / 'run_batch.py'
        path.write_text(path.read_text() + '\n# modified\n')
        with self.assertRaisesRegex(ValueError, 'Frozen study source changed'):
            self.prepare()
        self.assertFalse(self.output.exists())

    def test_reference_change_rejected(self):
        self.build['engines']['stockfish19']['sha256'] = '5' * 64
        self.rewrite(self.build_path, self.build)
        with self.assertRaisesRegex(ValueError, 'Unchanged opponent/reference'):
            self.prepare()

    def test_wrong_amendment_source_rejected(self):
        amendment = m.read(self.amendment)
        amendment['native_source'] = '6' * 40
        self.rewrite(self.amendment, amendment)
        with self.assertRaisesRegex(ValueError, 'Build differs'):
            self.prepare()

    def test_old_image_requires_explicit_package_mode(self):
        image = self.old['jobs'][0]['environment']['cloud']['image']
        with self.assertRaisesRegex(ValueError, 'new image'):
            self.prepare(image=image)
        self.prepare(image=image, package_only=True)
        manifest = m.read(self.output / 'batch-03-manifest.json')
        self.assertTrue(manifest['policy_transition']['package_only'])
        self.assertTrue(all(j['package_only'] for j in manifest['jobs']))

    def test_worker_sources_refresh_but_screening_policy_stays_identical(self):
        path = self.root / 'tools/experiments/chess.py'
        path.write_text(path.read_text() + '\n# candidate adapter\n')
        self.prepare()
        new = m.read(self.output / 'batch-03-manifest.json')
        for before, after in zip(self.old['jobs'], new['jobs']):
            self.assertEqual(after['source_files']['tools/experiments/chess.py'], m.sha(path))
            for key in ('nodes', 'automatic_confirmations', 'study_freeze_sha256'):
                self.assertEqual(before.get(key), after.get(key))


if __name__ == '__main__':
    unittest.main()
