import tempfile
import unittest
from pathlib import Path
from tools.experiments import core


class CoreTests(unittest.TestCase):
    def test_encode_is_canonical_and_round_trips(self):
        value = {'b': [1, 2], 'a': {'y': None, 'x': 'text'}}
        data = core.encode(value)
        self.assertEqual(data, b'{"a":{"x":"text","y":null},"b":[1,2]}\n')
        self.assertEqual(core.decode(data), value)

    def test_encode_rejects_non_finite_numbers(self):
        with self.assertRaises(ValueError):
            core.encode({'x': float('nan')})

    def test_decode_rejects_duplicate_keys(self):
        with self.assertRaisesRegex(ValueError, 'duplicate JSON key: a'):
            core.decode(b'{"a":1,"a":2}')

    def test_file_hash_matches_digest(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'f'
            path.write_bytes(b'x' * (3 * 1024 * 1024 + 7))
            self.assertEqual(core.file_hash(path), core.digest(path.read_bytes()))

    def test_environment_describes_host(self):
        value = core.environment()
        for key in ('platform', 'architecture', 'python', 'cpu_count', 'packages'):
            self.assertIn(key, value)


if __name__ == '__main__':
    unittest.main()
