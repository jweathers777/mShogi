#!/usr/bin/env python3
"""Read-only summary of MSH-161 query receipts; screens are not chess findings.

Usage: python summarize_analysis.py ANALYSIS_DIRECTORY [OUTPUT_JSON]
No engines, rules adjudication, theme classification or technical attribution.
"""
import argparse
import json
from pathlib import Path

from analysis import compare, decode, digest, save

THRESHOLDS = {'significant': .05, 'serious': .10, 'major': .20}


def screen(loss):
    if loss is None or loss < .05:
        return None
    return 'major' if loss >= .20 else 'serious' if loss >= .10 else 'significant'


def summarize(directory, candidate_index=0):
    directory = Path(directory).resolve()
    if (directory / 'inflight.json').exists():
        raise ValueError('Analysis is inflight or has an unreconciled interruption')
    manifest_path = directory / 'manifest.json'
    manifest = decode(manifest_path.read_text())
    latest, attempts = {}, []
    for path in sorted(directory.glob('g*-attempt*.json')):
        value = decode(path.read_text())
        if value['engine_sha256'] != manifest['engine_sha256'] or value['nodes_requested'] != manifest['nodes']:
            raise ValueError('Query identity differs from manifest: ' + str(path))
        attempts.append((path, value))
        latest[value['query']] = (path, value)
    grouped = {}
    for name, (path, value) in latest.items():
        decision = value['decision']
        key = (decision['game'], decision['ply'])
        bucket = grouped.setdefault(key, {'decision': decision, 'queries': {}})
        if bucket['decision'] != decision:
            raise ValueError('Query roots disagree: ' + name)
        kind = name.rsplit('-', 1)[1]
        bucket['queries'][kind] = (path, value)
    rows = []
    for key, bucket in sorted(grouped.items()):
        decision, queries = bucket['decision'], bucket['queries']
        refs = {kind: {'path': str(path), 'sha256': digest(path), 'status': value['status'],
                       'assessment': value.get('assessment'), 'error': value.get('error')}
                for kind, (path, value) in queries.items()}
        row = {k: decision[k] for k in ('game', 'opening_id', 'pairing', 'ply', 'mover', 'engine_index', 'origin', 'played')}
        row.update(query_refs=refs, history_ref={'path': str(next(iter(queries.values()))[0]),
                                              'json_pointer': '/decision',
                                              'contains': ['initial_fen', 'history', 'history_origin']},
                   provisional=True, confirmation='not_confirmed_by_this_summary')
        if 'unrestricted' in queries and 'played' in queries:
            result = compare(queries['unrestricted'][1], queries['played'][1])
        else:
            result = {'loss': None, 'uncertain': True, 'reason': 'missing_unrestricted_or_played_query'}
        row['comparison'] = result
        row['screen'] = screen(result['loss']) if not result['uncertain'] else None
        row['expected_score_mover'] = {
            kind: value.get('assessment', {}).get('expected_score_mover')
            for kind, (_, value) in queries.items()}
        rows.append(row)
    games = []
    for gi in sorted({x['game'] for x in rows}):
        played = [x for x in rows if x['game'] == gi and x['origin'] == 'played']
        groups = {}
        for player in sorted({x['engine_index'] for x in played}):
            turns = [x for x in played if x['engine_index'] == player]
            flagged = [x for x in turns if x['screen']]
            def event(row):
                return {'ply': row['ply'], 'move': row['played'], 'loss': row['comparison']['loss'],
                        'screen': row['screen'], 'query_refs': row['query_refs']}
            groups[str(player)] = {'played_decisions': len(turns),
                                  'usable_comparisons': sum(not x['comparison']['uncertain'] for x in turns),
                                  'screen_counts': {name: sum(x['comparison']['loss'] is not None
                                      and not x['comparison']['uncertain'] and x['comparison']['loss'] >= threshold
                                      for x in turns) for name, threshold in THRESHOLDS.items()},
                                  'first_provisional_deterioration': event(flagged[0]) if flagged else None,
                                  'largest_provisional_deterioration': event(max(flagged, key=lambda x: x['comparison']['loss'])) if flagged else None}
        links = []
        for previous, current in zip(played, played[1:]):
            if (previous['engine_index'] != candidate_index and current['engine_index'] == candidate_index
                    and current['ply'] == previous['ply'] + 1 and previous['screen']):
                links.append({'kind': 'opponent_error_response_candidate', 'opponent_ply': previous['ply'],
                              'response_ply': current['ply'], 'opponent_loss': previous['comparison']['loss'],
                              'response_loss': current['comparison']['loss'],
                              'response_comparison_uncertain': current['comparison']['uncertain'],
                              'note': 'Sequential screening link only. Opportunity, recovery, missed chance and causal importance require controlled confirmation.'})
        games.append({'game': gi, 'players': groups, 'opponent_response_candidates': links})
    expected_queries = manifest['planned_decisions'] * (1 if manifest.get('roots_input') or manifest.get('move') else 2)
    complete = len(latest) == expected_queries and all(v['status'] == 'complete' for _, v in latest.values())
    return {'schema': 1, 'purpose': 'Provisional all-move screening for independent gameplay review',
            'limitations': ['Finite Stockfish self-play expected-score proxy, not actual opponent winning probability.',
                           'Threshold crossings are candidate events, not confirmed mistakes or causal explanations.',
                           'Largest screening loss can occur after a game was already lost; causal importance is not assigned.',
                           'Prepared moves are analyzed separately and never attributed to either competitor.',
                           'No automatic advantage, recovery, conversion, game-phase or thematic classification.'],
            'manifest_ref': {'path': str(manifest_path), 'sha256': digest(manifest_path)},
            'analysis_directory': str(directory), 'candidate_engine_index': candidate_index,
            'thresholds_expected_score_loss': THRESHOLDS, 'complete_query_coverage': complete,
            'coverage': {'planned_queries': expected_queries, 'latest_query_receipts': len(latest),
                         'completed_latest_queries': sum(v['status'] == 'complete' for _, v in latest.values()),
                         'decisions_with_receipts': len(rows),
                         'prepared_decisions': sum(x['origin'] == 'prepared' for x in rows),
                         'played_decisions': sum(x['origin'] == 'played' for x in rows),
                         'usable_played_comparisons': sum(x['origin'] == 'played' and not x['comparison']['uncertain'] for x in rows)},
            'resources': {'attempts_including_failures': len(attempts),
                          'wall_seconds': sum(v['wall_seconds'] for _, v in attempts),
                          'child_cpu_seconds': sum(v['child_cpu_seconds'] for _, v in attempts),
                          'failed_attempts': sum(v['status'] != 'complete' for _, v in attempts),
                          'query_artifact_bytes': sum(p.stat().st_size for p, _ in attempts)},
            'native_game_status': manifest['games'], 'games': games, 'decisions': rows}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory')
    parser.add_argument('output', nargs='?')
    parser.add_argument('--candidate-index', type=int, default=0)
    args = parser.parse_args()
    result = summarize(args.directory, args.candidate_index)
    if args.output:
        path = Path(args.output)
        if path.exists():
            raise FileExistsError('Preserve original summary; choose a new output path')
        save(path, result)
    else:
        print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
