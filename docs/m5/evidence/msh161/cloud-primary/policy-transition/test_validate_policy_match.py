"""Offline validator regressions; synthetic runner envelope, no engine execution."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import unittest

HELPER = Path(__file__).with_name('validate_policy_match.py')
SPEC = importlib.util.spec_from_file_location('policy_match_smoke', HELPER)
smoke = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(smoke)


def fixture():
    build = {'source': 'synthetic-revision', 'engines': {
        'candidate': {'path': '/synthetic/mshogi', 'sha256': 'candidate-sha'},
        'oxidation060': {'path': '/synthetic/oxidation', 'sha256': 'opponent-sha'},
    }}
    cfg = smoke.run_batch.config(build, {'id': 'synthetic-smoke', 'opponent': 'oxidation060',
        'opening': {'id': 'kr-k-optional-fifty', 'fen': '7k/8/8/8/8/8/8/KR6 b - - 100 1', 'moves': []}})
    cfg.update(claim_policy='explicit_only', max_plies=4)
    games = []
    for pairing in range(2):
        colors = [0, 1] if pairing == 0 else [1, 0]
        events = [{'request': {'at_ns': 1, 'version': 0, 'actor': 'black', 'command': 'start'},
                   'reply': {'version': 1, 'active': 'black', 'remaining_ns': None, 'conclusion': None, 'rejection': None}}]
        transcript = []
        for ply, move in enumerate(['h8g8', 'b1b2', 'g8h8', 'b2b1']):
            side = (ply + 1) % 2
            actor = ['white', 'black'][side]
            engine = colors[side]
            events.append({'request': {'at_ns': 100 + ply * 10, 'version': ply + 1, 'actor': actor, 'command': {'move': move}},
                           'reply': {'version': ply + 2, 'active': ['black', 'white'][side], 'remaining_ns': None, 'conclusion': None, 'rejection': None}})
            transcript.extend([
                {'engine': engine, 'direction': 'command', 'at_ns': ply * 10, 'text': 'go movetime 5000'},
                {'engine': engine, 'direction': 'response', 'at_ns': ply * 10 + 1, 'text': 'info depth 2 nodes 50 score cp -500 pv ' + move},
                {'engine': engine, 'direction': 'response', 'at_ns': ply * 10 + 2, 'text': 'bestmove ' + move},
            ])
        initial = {'rules_profile': 'mshogi.chess.match.v1', 'initial_fen': cfg['openings'][0]['fen'],
                   'history_origin': 'imported_fen', 'events': []}
        record = {'rules_profile': 'mshogi.chess.match.v1', 'initial_record_json': json.dumps(initial), 'events': events}
        games.append({'pairing': pairing, 'opening_index': 0, 'opening_id': cfg['openings'][0]['id'],
                      'engines_by_color': colors, 'stop': 'ply_limit', 'conclusion': None, 'failure': None,
                      'diagnostic_failures': [], 'match_record_json': json.dumps(record),
                      'transcript': {'truncated': False, 'entries': transcript},
                      'engines': [{'engine': e, 'identity_truncated': False, 'cleanup_failures': [],
                                   'stderr': {'text': '', 'truncated': False}, 'exit': {'success': True}}
                                  for e in [0, 1]]})
    report = {'schema_version': 1, 'config': cfg, 'games': games,
              'engine_identities': [{'executable': {'sha256': build['engines'][k]['sha256']}}
                                    for k in ['candidate', 'oxidation060']],
              'summaries': [{'wins': [0, 0], 'draws': 0, 'unscored': 2}]}
    return report, cfg, build


def envelope(report, build):
    text = json.dumps(report)
    return {'protocol': 'uci', 'report_json': text, 'report_sha256': hashlib.sha256(text.encode()).hexdigest(),
            'environment': {'harness_revision': build['source'], 'harness_dirty_sha256': hashlib.sha256(b'').hexdigest()}}


class Validation(unittest.TestCase):
    def test_start_plus_four_moves_per_color_passes(self):
        report, cfg, build = fixture()
        result = smoke.validate_report(envelope(report, build), cfg, build)
        self.assertEqual(result['native_accepted_moves'], 8)
        self.assertEqual(len(result['candidate_PV_backed_moves']), 4)
        self.assertTrue(result['audit']['healthy'])
        self.assertFalse(result['audit']['complete'])
        self.assertEqual(result['claims'], 0)

    def test_malformed_start_claim_rejection_or_pv_fails(self):
        for corruption in ['missing_start', 'wrong_start_actor', 'claim', 'rejection', 'pv_mismatch', 'truncated', 'scored']:
            with self.subTest(corruption=corruption):
                report, cfg, build = fixture()
                game = report['games'][0]
                record = json.loads(game['match_record_json'])
                if corruption == 'missing_start':
                    record['events'].pop(0)
                elif corruption == 'wrong_start_actor':
                    record['events'][0]['request']['actor'] = 'white'
                elif corruption == 'claim':
                    record['events'][1]['request']['command'] = {'claim': {'reason': 'fifty_move', 'intended': 'h8g8'}}
                elif corruption == 'rejection':
                    record['events'][1]['reply']['rejection'] = 'invalid'
                elif corruption == 'pv_mismatch':
                    next(e for e in game['transcript']['entries'] if e['engine'] == 0 and e['text'].startswith('info'))['text'] = 'info depth 2 nodes 50 pv a1a2'
                elif corruption == 'truncated':
                    game['transcript']['truncated'] = True
                elif corruption == 'scored':
                    report['summaries'][0].update(unscored=0, draws=2)
                game['match_record_json'] = json.dumps(record)
                with self.assertRaises((AssertionError, ValueError)):
                    smoke.validate_report(envelope(report, build), cfg, build)


if __name__ == '__main__':
    unittest.main()
