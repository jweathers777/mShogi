"""Offline adapter checks; no engine, cloud job or chess result is created."""
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from tools.experiments.core import digest
from tools.strength import anchor_worker as worker


def inputs(swapped=False):
    build = {'source': 'a' * 40, 'engines': {name: {
        'path': '/opt/engines/' + name, 'sha256': str(index) * 64, 'source': name + '-source',
        'args': ['uci'] if name == 'candidate' else [],
        'options': [{'name': 'Hash', 'value': '64'}]}
        for index, name in enumerate(('runner', 'candidate', 'opponent'))}}
    spec = {'id': 'g1', 'pair_id': 'p1', 'opponent': 'opponent', 'swapped': swapped,
            'opening': {'id': 'opening1', 'fen': None, 'moves': ['a2a3'] * 16}}
    return build, spec


def envelope(build, spec, wins=None, draws=0, unscored=0):
    cfg = worker.config(build, spec)
    game = {'pairing': 0, 'engines_by_color': [0, 1], 'stop': 'chess_conclusion',
            'failure': None, 'diagnostic_failures': [], 'conclusion': {'fixture': True},
            'transcript': {'truncated': False, 'entries': [
                {'direction': 'command', 'text': 'go wtime 60000 btime 60000 winc 600 binc 600'}]},
            'engines': [{'identity_truncated': False, 'cleanup_failures': [],
                         'stderr': {'truncated': False}, 'exit': {'success': True}} for _ in range(2)]}
    report = {'config': cfg, 'games': [game],
              'engine_identities': [{'executable': {'sha256': build['engines'][Path(e['path']).name]['sha256']}}
                                    for e in cfg['engines']],
              'summaries': [{'games': 1, 'wins': wins or [0, 0], 'draws': draws, 'unscored': unscored}]}
    raw = json.dumps(report)
    return {'schema': 1, 'protocol': 'uci', 'report_json': raw, 'report_sha256': digest(raw.encode()),
            'environment': {'harness_revision': build['source'], 'harness_dirty_sha256': digest(b'')}}


class ConfigTests(unittest.TestCase):
    def test_clock_and_external_arguments(self):
        build, spec = inputs()
        cfg = worker.config(build, spec)
        self.assertFalse(cfg['paired'])
        self.assertEqual(cfg['engines'][0]['args'], ['uci'])
        self.assertEqual(cfg['engines'][1]['args'], [])
        self.assertEqual(cfg['time_control']['increment'],
                         {'initial_ns': [60_000_000_000] * 2, 'increment_ns': 600_000_000})
        self.assertTrue(all(v is None for v in cfg['work'].values()))
        self.assertEqual(cfg['claim_policy'], 'auto_claim')
        # Independent native bound: validate_processes rejects command_bytes >4096.
        self.assertLessEqual(cfg['io']['command_bytes'], 4096)

    def test_color_reversal_keeps_engine_options(self):
        build, spec = inputs(True)
        cfg = worker.config(build, spec)
        self.assertEqual(cfg['engines'][0]['path'], '/opt/engines/opponent')
        self.assertEqual(cfg['engines'][1]['args'], ['uci'])

    def test_manifest_rejects_duplicate_game_and_unbounded_workers(self):
        build, spec = inputs()
        manifest = {'schema': 1, 'experiment': 'msh185-b01', 'deadline_epoch': 9999999999,
                    'build': build, 'bins': [[spec]]}
        worker.validate_manifest(manifest)
        manifest['bins'] = [[spec], [spec]]
        with self.assertRaisesRegex(ValueError, 'duplicate game'):
            worker.validate_manifest(manifest)
        manifest['bins'] = [[spec]] * 9
        with self.assertRaisesRegex(ValueError, 'eight'):
            worker.validate_manifest(manifest)


    def test_pair_members_must_share_opening(self):
        build, spec = inputs()
        second = dict(spec, id='g2', swapped=True, opening=dict(spec['opening'], id='different'))
        manifest = {'schema': 1, 'experiment': 'msh185-b01', 'deadline_epoch': 9999999999,
                    'build': build, 'bins': [[spec], [second]]}
        with self.assertRaisesRegex(ValueError, 'openings differ'):
            worker.validate_manifest(manifest)

class AuditTests(unittest.TestCase):
    def test_score_mapping_for_each_color(self):
        for swapped, expected in ((False, 1), (True, 0)):
            build, spec = inputs(swapped)
            outer = envelope(build, spec, wins=[1, 0])
            result = worker.audit(outer, worker.config(build, spec), build, spec)
            self.assertEqual(result['score'], expected)
            self.assertTrue(result['complete'])

    def test_draw_and_unscored_are_distinct(self):
        build, spec = inputs()
        self.assertEqual(worker.audit(envelope(build, spec, draws=1), worker.config(build, spec), build, spec)['score'], .5)
        result = worker.audit(envelope(build, spec, unscored=1), worker.config(build, spec), build, spec)
        self.assertIsNone(result['score'])
        self.assertFalse(result['complete'])

    def test_reject_hash_or_native_revision_mismatch(self):
        build, spec = inputs()
        outer = envelope(build, spec, draws=1)
        outer['report_sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'hash'):
            worker.audit(outer, worker.config(build, spec), build, spec)
        outer = envelope(build, spec, draws=1)
        outer['environment']['harness_revision'] = 'b' * 40
        with self.assertRaisesRegex(ValueError, 'revision'):
            worker.audit(outer, worker.config(build, spec), build, spec)

    def test_process_diagnostics_make_native_score_ineligible(self):
        build, spec = inputs()
        outer = envelope(build, spec, wins=[1, 0])
        raw = json.loads(outer['report_json'])
        raw['games'][0]['engines'][0]['cleanup_failures'] = ['child did not stop']
        outer['report_json'] = json.dumps(raw)
        outer['report_sha256'] = digest(outer['report_json'].encode())
        result = worker.audit(outer, worker.config(build, spec), build, spec)
        self.assertIsNone(result['score'])
        self.assertFalse(result['complete'])

    def test_flag_fall_requires_separate_timing_admission(self):
        build, spec = inputs()
        for conclusion, needs_review in (({'win': {'winner': 'white', 'trigger': 'flag_fall'}}, True),
                                         ({'draw': {'trigger': 'flag_fall'}}, True),
                                         ({'chess': {'checkmate': {'winner': 'white'}}}, False)):
            outer = envelope(build, spec, wins=[1, 0])
            raw = json.loads(outer['report_json'])
            raw['games'][0]['conclusion'] = conclusion
            outer['report_json'] = json.dumps(raw)
            outer['report_sha256'] = digest(outer['report_json'].encode())
            result = worker.audit(outer, worker.config(build, spec), build, spec)
            self.assertEqual(result['clock_review_required'], needs_review)
            # Complete means a replay-eligible result, not completed timing review.
            self.assertTrue(result['complete'])

    def test_cloud_region_checked_from_observed_zone(self):
        class Reply:
            def __init__(self, value):
                self.value = value
            def __enter__(self):
                return self
            def __exit__(self, *args):
                pass
            def read(self, bound):
                return self.value.encode()
        for zone, accepted in (('projects/123/zones/us-central1-a', True),
                               ('projects/123/zones/us-east1-a', False)):
            values = ['projects/123/machineTypes/c3-standard-4', zone, '123', 'Intel Sapphire Rapids']
            with patch.object(worker, 'environment', return_value={'architecture': 'x86_64', 'platform': 'Linux'}), \
                 patch.object(worker.urllib.request, 'urlopen', side_effect=[Reply(v) for v in values]):
                if accepted:
                    self.assertEqual(worker.cloud_environment()['gcp']['zone'], zone)
                else:
                    with self.assertRaisesRegex(ValueError, 'region'):
                        worker.cloud_environment()

    def test_clock_game_rejects_fixed_movetime(self):
        build, spec = inputs()
        outer = envelope(build, spec, wins=[1, 0])
        raw = json.loads(outer['report_json'])
        raw['games'][0]['transcript']['entries'][0]['text'] += ' movetime 1000'
        outer['report_json'] = json.dumps(raw)
        outer['report_sha256'] = digest(outer['report_json'].encode())
        with self.assertRaisesRegex(ValueError, 'work limit'):
            worker.audit(outer, worker.config(build, spec), build, spec)

    def test_deadline_prevents_process_start_and_keeps_receipt(self):
        build, spec = inputs()
        with tempfile.TemporaryDirectory() as temp, patch.object(worker, 'file_hash') as sha, patch.object(worker.subprocess, 'Popen') as launch:
            sha.side_effect = [build['engines'][n]['sha256'] for n in ('runner', 'candidate', 'opponent')]
            dest = Path(temp) / 'game'
            result = worker.run_game(build, spec, dest, 0)
            launch.assert_not_called()
            self.assertFalse(result['complete'])
            self.assertIn('deadline', result['error'])
            self.assertTrue((dest / 'receipt.json').exists())

    def test_immutable_upload_conflict_is_not_a_resume(self):
        class Store:
            def put(self, key, data):
                return False
        with self.assertRaisesRegex(ValueError, 'existing immutable'):
            worker.put_new(Store(), 'games/g1/launch.json', b'{}')


if __name__ == '__main__':
    unittest.main()
