"""Source and supply boundaries; no claim of a Linux GUI run."""
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('prepare_driver', Path(__file__).with_name('prepare-linux-gui-driver.py'))
prepare = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prepare)


class PreparationTests(unittest.TestCase):
    def test_only_pool_policy_changes_and_replay_stays_disabled(self):
        original = 'let client = Client::builder(executor)\n' + prepare.ANCHOR + '    .build_http();\n'
        corrected = prepare.patch_source(original)
        self.assertEqual(corrected.replace('    .pool_max_idle_per_host(0)\n', ''), original)
        self.assertIn('.retry_canceled_requests(false)', corrected)

    def test_unexpected_or_already_patched_source_is_rejected(self):
        for source in ('', prepare.ANCHOR * 2, prepare.REPLACEMENT):
            with self.subTest(source=source), self.assertRaises(ValueError):
                prepare.patch_source(source)

    def test_untrusted_archive_rejected_before_extraction(self):
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            prepare.extract(b'not the pinned crate', Path('unused'))


if __name__ == '__main__':
    unittest.main(verbosity=2)
