"""Offline tests. No engine process or network is used."""
import gzip
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

import analysis as a


def fixture():
    # Complete prefix includes a double pawn push; the reply is en passant.
    initial = {'schema_version': 1, 'rules_profile': 'mshogi.chess.match.v1',
               'initial_fen': a.START_FEN, 'history_origin': 'standard_start',
               'events': [{'ply': i, 'event': {'kind': 'move', 'action': move}}
                          for i, move in enumerate(['e2e4', 'a7a6', 'e4e5', 'd7d5'])]}
    def event(actor, command, rejection=None):
        return {'request': {'actor': actor, 'command': command}, 'reply': {'rejection': rejection}}
    record = {'schema_version': 1, 'rules_profile': initial['rules_profile'],
              'initial_record_json': json.dumps(initial), 'events': [
                  event('white', 'start'), event('white', {'move': 'e5d6'}),
                  event('black', {'move': 'e7e9'}, {'kind': 'invalid_action'}),
                  event('black', {'claim': {'reason': 'threefold', 'intended': 'g8f6'}})]}
    return {'schema_version': 1, 'config': {'openings': [{'id': 'ep', 'fen': None,
             'moves': ['e2e4', 'a7a6', 'e4e5', 'd7d5']}]},
            'games': [{'opening_index': 0, 'opening_id': 'ep', 'pairing': 1,
                       'engines_by_color': [1, 0], 'stop': 'chess_conclusion',
                       'match_record_json': json.dumps(record),
                       # Deliberately misleading transcript must be ignored.
                       'transcript': [{'line': 'bestmove g1f3'}]}]}


class AnalysisTests(unittest.TestCase):
    def test_complete_native_history_and_prepared_separation(self):
        rows, games = a.extract_decisions(fixture())
        self.assertEqual(len(rows), 5)
        self.assertEqual(rows[-1]['history'], ['e2e4', 'a7a6', 'e4e5', 'd7d5'])
        self.assertEqual(rows[-1]['played'], 'e5d6')
        self.assertEqual(rows[-1]['mover'], 'white')
        self.assertEqual(rows[-1]['engine_index'], 1)
        self.assertEqual([r['origin'] for r in rows], ['prepared'] * 4 + ['played'])
        self.assertTrue(all(r['engine_index'] is None for r in rows[:-1]))
        self.assertEqual(games[0]['rejected_events'], [2])
        self.assertNotIn('g8f6', games[0]['complete_history'])

    def test_config_history_mismatch_rejected(self):
        value = fixture()
        value['config']['openings'][0]['moves'][-1] = 'd7d6'
        with self.assertRaisesRegex(ValueError, 'differs'):
            a.extract_decisions(value)

    def test_wrong_accepted_actor_rejected(self):
        value = fixture()
        record = json.loads(value['games'][0]['match_record_json'])
        record['events'][1]['request']['actor'] = 'black'
        value['games'][0]['match_record_json'] = json.dumps(record)
        with self.assertRaisesRegex(ValueError, 'actor'):
            a.extract_decisions(value)

    def test_black_initial_position_perspective(self):
        value = fixture()
        fen = '8/8/8/8/8/8/4k3/7K b - - 81 99'
        value['config']['openings'][0].update(fen=fen, moves=[])
        record = json.loads(value['games'][0]['match_record_json'])
        initial = json.loads(record['initial_record_json'])
        initial.update(initial_fen=fen, history_origin='imported', events=[])
        record['initial_record_json'] = json.dumps(initial)
        record['events'] = [{'request': {'actor': 'black', 'command': {'move': 'e2e3'}},
                             'reply': {'rejection': None}}]
        value['games'][0]['match_record_json'] = json.dumps(record)
        rows, _ = a.extract_decisions(value)
        self.assertEqual(rows[0]['mover'], 'black')
        self.assertEqual(rows[0]['engine_index'], 0)
        self.assertEqual(rows[0]['initial_fen'], fen)
        self.assertAlmostEqual(a.expected_score([700, 200, 100], 'black', 'white'), .2)
        self.assertAlmostEqual(a.expected_score([700, 200, 100], 'black', 'mover'), .8)

    def test_wrapper_hash_and_gzip(self):
        nested = json.dumps(fixture())
        wrapper = {'report_json': nested, 'report_sha256': hashlib.sha256(nested.encode()).hexdigest()}
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'report.json.gz'
            with gzip.open(path, 'wt') as f:
                json.dump(wrapper, f)
            self.assertEqual(a.load_report(path), fixture())
            wrapper['report_sha256'] = 'wrong'
            with gzip.open(path, 'wt') as f:
                json.dump(wrapper, f)
            with self.assertRaisesRegex(ValueError, 'hash mismatch'):
                a.load_report(path)

    def test_duplicate_keys_fail(self):
        with self.assertRaisesRegex(ValueError, 'Duplicate'):
            a.decode('{"schema_version":1,"schema_version":2}')

    def test_aligned_score_is_not_combined_across_lines(self):
        exact = a.parse_info('info depth 12 score cp 91 wdl 350 600 50 nodes 500000 time 123 pv e2e4 e7e5')
        bound = a.parse_info('info depth 13 score cp 140 lowerbound wdl 600 350 50 nodes 500100 pv d2d4 d7d5')
        selected = a.select_score([exact, bound], 'e2e4')
        self.assertEqual(selected['score']['depth'], 12)
        self.assertEqual(selected['score']['wdl'], [350, 600, 50])
        self.assertIn('last_score_update_not_selected_exact_score', selected['uncertainty'])
        self.assertIsNone(a.select_score([exact, bound], 'd2d4')['score'])
        self.assertIsNone(a.parse_info('info depth 14 score cp 99'))
        self.assertIsNone(a.parse_info('info depth 14 wdl 500 500 0 pv e2e4'))

    def test_mate_stays_separate_and_missing_wdl_uncertain(self):
        info = a.parse_info('info depth 10 score mate 3 nodes 90 pv h5f7 e8e7')
        result = a.select_score([info], 'h5f7')
        self.assertEqual(result['score']['score'], {'kind': 'mate', 'value': 3})
        self.assertIn('missing_wdl', result['uncertainty'])
        self.assertNotIn('expected_score_mover', result)

    def test_negative_loss_preserved_not_clamped(self):
        identity = {'engine_sha256': 'test', 'nodes_requested': 500000, 'options': a.OPTIONS,
                    'decision': {'initial_fen': a.START_FEN, 'history': [], 'mover': 'white'}}
        unrestricted = {**identity, 'status': 'complete', 'assessment': {'expected_score_mover': .60, 'uncertainty': []}}
        played = {**identity, 'status': 'complete', 'assessment': {'expected_score_mover': .70, 'uncertainty': []}}
        result = a.compare(unrestricted, played)
        self.assertAlmostEqual(result['loss'], -.1)
        self.assertTrue(result['negative_loss_preserved'])
        self.assertTrue(result['uncertain'])
        played['nodes_requested'] = 1
        self.assertEqual(a.compare(unrestricted, played)['reason'], 'comparison_identity_mismatch_or_missing')
        played['nodes_requested'] = 500000
        played['assessment']['uncertainty'] = ['missing_wdl']
        self.assertIsNone(a.compare(unrestricted, played)['loss'])

    def test_orphan_marker_refused(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            a.save(directory / 'inflight.json', {'pid': 999999})
            with self.assertRaisesRegex(RuntimeError, 'Inflight/orphan'):
                with a.locked(directory):
                    self.fail('Must not enter')
            self.assertTrue((directory / 'inflight.json').exists())

    def test_summary_keeps_both_players_and_labels_only_candidates(self):
        import summarize_analysis as summary
        rows, _ = a.extract_decisions(fixture())
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            a.save(directory / 'manifest.json', {'engine_sha256': 'test', 'nodes': 500000,
                   'planned_decisions': 3, 'games': []})
            examples = [(rows[0], .8, .5),
                        ({**rows[-1], 'game': 0, 'ply': 4, 'engine_index': 1}, .7, .55),
                        ({**rows[-1], 'game': 0, 'ply': 5, 'engine_index': 0,
                          'mover': 'black', 'history': rows[-1]['history'] + ['e5d6'],
                          'played': 'e7e6'}, .6, .48)]
            for decision, best, selected in examples:
                for kind, value in [('unrestricted', best), ('played', selected)]:
                    name = f'g000-p{decision["ply"]:03d}-{kind}'
                    a.save(directory / (name + '-attempt0000.json'), {
                        'query': name, 'engine_sha256': 'test', 'nodes_requested': 500000,
                        'options': a.OPTIONS, 'decision': decision, 'status': 'complete',
                        'assessment': {'expected_score_mover': value, 'uncertainty': []},
                        'wall_seconds': 1, 'child_cpu_seconds': .8})
            result = summary.summarize(directory)
            self.assertTrue(result['complete_query_coverage'])
            self.assertEqual(result['coverage']['prepared_decisions'], 1)
            self.assertEqual(result['coverage']['played_decisions'], 2)
            self.assertEqual(result['resources']['attempts_including_failures'], 6)
            players = result['games'][0]['players']
            self.assertEqual(set(players), {'0', '1'})
            self.assertEqual(players['0']['first_provisional_deterioration']['ply'], 5)
            self.assertEqual(players['1']['largest_provisional_deterioration']['ply'], 4)
            links = result['games'][0]['opponent_response_candidates']
            self.assertEqual(len(links), 1)
            self.assertEqual(links[0]['kind'], 'opponent_error_response_candidate')
            self.assertTrue(all(x['provisional'] for x in result['decisions']))
            self.assertNotIn('technical_cause', result)
            (directory / 'g000-p005-played-attempt0000.json').unlink()
            self.assertFalse(summary.summarize(directory)['complete_query_coverage'])

    def test_root_screen_preserves_complete_prefix(self):
        rows, games = a.extract_roots([{'id': 'opening', 'fen': None,
                                       'moves': ['e2e4', 'c7c5', 'g1f3']}])
        self.assertEqual(games, [])
        self.assertEqual(rows[0]['history'], ['e2e4', 'c7c5', 'g1f3'])
        self.assertEqual(rows[0]['ply'], 3)
        self.assertEqual(rows[0]['mover'], 'black')
        self.assertEqual(rows[0]['origin'], 'supplied_root')
        self.assertIsNone(rows[0]['played'])
        with self.assertRaisesRegex(ValueError, 'Duplicate'):
            a.extract_roots([{'id': 'same', 'moves': []}, {'id': 'same', 'moves': []}])

    def test_output_lock_excludes_another_writer(self):
        with tempfile.TemporaryDirectory() as tmp:
            with a.locked(Path(tmp)):
                with self.assertRaises(BlockingIOError):
                    with a.locked(Path(tmp)):
                        self.fail('Second writer must not acquire lock')

    def test_historical_native_report_extracts_without_transcript(self):
        root = Path(__file__).resolve().parents[1]
        report = a.load_report(root / 'msh151/pair-001-discovery-goldfish113/report.json.gz')
        for game in report['games']:
            game.pop('transcript', None)
        rows, games = a.extract_decisions(report)
        self.assertEqual(len(games), 2)
        self.assertEqual(rows[16]['played'], 'd2d3')
        self.assertEqual(rows[16]['history'][-1], 'e8g8')
        self.assertTrue(all(g['stop'] == 'chess_conclusion' for g in games))
        self.assertEqual(sum(g['decisions'] for g in games), len(rows))


if __name__ == '__main__':
    unittest.main()
