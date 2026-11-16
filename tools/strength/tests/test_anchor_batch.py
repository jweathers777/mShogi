"""Offline allocation/collection checks; synthetic reports are not game evidence."""
import contextlib
import gzip
import io
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest

from tools.experiments.core import digest, encode
from tools.strength import anchor_batch as batch
from tools.strength.tests.test_anchor_worker import inputs, envelope


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(encode(value))


def make_run(root, *, complete=True, swapped=False, clock=False, partner_failure=False):
    build, spec = inputs(swapped)
    build['engines']['opponent'].update(family='test-family', rating=2500)
    specs = [spec]
    if partner_failure:
        specs.append(dict(spec, id='g2', swapped=not swapped))
    manifest = {'schema': 1, 'experiment': 'test', 'phase': 'bracket', 'build': build,
                'deadline_epoch': 9999999999, 'bins': [[s] for s in specs]}
    write(root/'manifest.json', manifest)
    folder = root/'raw/games'/spec['id']
    outer = envelope(build, spec, wins=[1, 0])
    if clock:
        report = json.loads(outer['report_json'])
        report['games'][0]['conclusion'] = {'win': {'winner': 'white', 'trigger': {'flag_fall': {'actor': 'black'}}}}
        outer['report_json'] = json.dumps(report)
        outer['report_sha256'] = digest(outer['report_json'].encode())
    raw = encode(outer)
    receipt = {'complete': complete, 'score': 0 if swapped else 1,
               'replay': {'returncode': 0}, 'report_sha256': digest(raw)}
    write(folder/'receipt.json', receipt)
    (folder/'report.json.gz').write_bytes(gzip.compress(raw))
    if partner_failure:
        write(root/'raw/games/g2/receipt.json', {'complete': False, 'score': None})
    return manifest, outer, receipt


class PrepareTests(unittest.TestCase):
    def test_bracket_allocates_eight_simultaneous_single_games(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            build, spec = inputs()
            write(root/'build.json', build)
            write(root/'openings.json', {'openings': [dict(spec['opening'], id=f'root-{i}') for i in range(200)]})
            write(root/'campaign.json', {'started_epoch': 1000})
            args = SimpleNamespace(build=root/'build.json', openings=root/'openings.json',
                                   campaign=root/'campaign.json', output=root/'manifest.json',
                                   first_pair=3, pairs=4, phase='bracket', opponents=['opponent'],
                                   name='test-bracket', compute_before=0, reason='nearest anchor')
            batch.prepare(args)
            manifest = json.loads((root/'manifest.json').read_text())
            self.assertEqual([len(tasks) for tasks in manifest['bins']], [1]*8)
            self.assertEqual(manifest['deadline_epoch'], 22600)
            for pair in range(3, 7):
                games = [tasks[0] for tasks in manifest['bins'] if tasks[0]['pair_id'] == f'p{pair:03d}']
                self.assertEqual({g['swapped'] for g in games}, {False, True})
                self.assertEqual(games[0]['opening'], games[1]['opening'])
                self.assertEqual(games[0]['opening']['id'], f'root-{pair}')

    def test_wrong_bracket_size_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            build, spec = inputs()
            write(root/'build.json', build)
            write(root/'openings.json', {'openings': [spec['opening']]*200})
            write(root/'campaign.json', {'started_epoch': 1000})
            args = SimpleNamespace(build=root/'build.json', openings=root/'openings.json',
                                   campaign=root/'campaign.json', output=root/'manifest.json',
                                   first_pair=0, pairs=5, phase='bracket', opponents=['opponent'])
            with self.assertRaisesRegex(ValueError, 'four pairs'):
                batch.prepare(args)
            self.assertFalse((root/'manifest.json').exists())


class CollectTests(unittest.TestCase):
    def test_scored_singleton_kept_when_partner_failed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            make_run(root, partner_failure=True)
            games, failures = batch.collect(root)
            self.assertEqual(len(games), 1)
            self.assertEqual(games[0]['game_id'], 'g1')
            self.assertEqual(games[0]['score'], 1)
            self.assertEqual(failures[0]['game_id'], 'g2')
            fit = batch.anchor_stats.fit(games)
            self.assertEqual(fit['singleton_clusters'], 1)

    def test_pending_clock_review_refuses_fit_then_explicit_decisions_apply(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            make_run(root, clock=True)
            with self.assertRaisesRegex(ValueError, 'timing review'):
                batch.collect(root)
            games, failures = batch.collect(root, {'g1': 'genuine-native-time-loss'})
            self.assertEqual(len(games), 1)
            self.assertEqual(failures, [])
            games, failures = batch.collect(root, {'g1': 'harness-caused-unscored'})
            self.assertEqual(games, [])
            self.assertEqual(failures[0]['reason'], 'harness-caused-unscored')

    def test_report_hash_mismatch_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            _, _, receipt = make_run(root)
            receipt['report_sha256'] = '0'*64
            write(root/'raw/games/g1/receipt.json', receipt)
            with self.assertRaisesRegex(ValueError, 'hash/replay'):
                batch.collect(root)

    def test_failed_native_replay_refused(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            _, _, receipt = make_run(root)
            receipt['replay']['returncode'] = 1
            write(root/'raw/games/g1/receipt.json', receipt)
            with self.assertRaisesRegex(ValueError, 'hash/replay'):
                batch.collect(root)

    def test_native_audit_rejects_wrong_runner_even_with_matching_outer_hash(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            _, outer, receipt = make_run(root)
            outer['environment']['harness_revision'] = 'b'*40
            raw = encode(outer)
            (root/'raw/games/g1/report.json.gz').write_bytes(gzip.compress(raw))
            receipt['report_sha256'] = digest(raw)
            write(root/'raw/games/g1/receipt.json', receipt)
            with self.assertRaisesRegex(ValueError, 'runner revision'):
                batch.collect(root)

    def test_receipt_score_cannot_override_native_score(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            _, _, receipt = make_run(root)
            receipt['score'] = .5
            write(root/'raw/games/g1/receipt.json', receipt)
            with self.assertRaisesRegex(ValueError, 'disagrees'):
                batch.collect(root)

    def test_duplicate_games_across_runs_refused_before_output(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            make_run(root/'run1')
            make_run(root/'run2')
            args = SimpleNamespace(runs=[root/'run1', root/'run2'], clock_reviews=None, output=root/'summary.json')
            with self.assertRaisesRegex(ValueError, 'duplicate game_id'):
                batch.summarize(args)
            self.assertFalse((root/'summary.json').exists())


if __name__ == '__main__':
    unittest.main()
