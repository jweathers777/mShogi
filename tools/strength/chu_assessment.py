"""Chu sampled move quality; frozen MSH-52.3 rules, same-family evidence only.

Input is chu_judge JSONL. All sampled moves count, including unfinished games.
Missing judge scores stay unjudged. Suites preserve full native history and
split by opening (both colors together). Run output belongs under runs/ or GCS.
"""
import argparse
import collections
import json
import statistics
from pathlib import Path

from .blunder_rules import judgement, split_of, win_probability
from .native_match import telemetry

MATE_SCORE = 1_000_000
MATE_BOUND = MATE_SCORE - 256


def probability(row):
    if 'terminal_probability' in row:
        return row['terminal_probability']
    score = row.get('score')
    if score is None:
        return None
    return win_probability(('mate' if abs(score) >= MATE_BOUND else 'cp', score))


def classify(row):
    before, after = row['before'], row['after']
    best = before.get('best') or {}
    reply = after.get('best') or {}
    if row.get('clock_ms') is not None and row['clock_ms'] < 1000:
        return 'time_trouble'
    if row['repeated']:
        return 'repetition'
    if after.get('score') is not None and after['score'] >= MATE_SCORE - 9:
        return 'allowed_short_mate'
    if best.get('captures') and not row['played']['captures']:
        return 'missed_capture'
    if reply.get('captures_lion'):
        return 'lion_refutation'
    if len(reply.get('captures', [])) > 1:
        return 'compound_refutation'
    if reply.get('promotes'):
        return 'promotion_refutation'
    if reply.get('captures'):
        return 'other_capture'
    return 'other'


def analyze(rows):
    errors, depths, player_depths, node_ratios = [], [], [], []
    unjudged, loss, moves, deeper = 0, 0.0, 0, 0
    seen = set()
    for row in rows:
        identity = (row['game_id'], row['ply'])
        if identity in seen:
            raise ValueError('duplicate sampled move')
        seen.add(identity)
        before, after = row['before'], row['after']
        p, nxt = probability(before), probability(after)
        if p is None or nxt is None:
            unjudged += 1
            continue
        moves += 1
        drop = p - (1 - nxt)
        loss += max(0, drop)
        player = telemetry(row.get('player_observation'))
        if player and before.get('depth') is not None:
            depths.append(before['depth'])
            player_depths.append(player['completed_depth'])
            deeper += before['depth'] > player['completed_depth']
            if player.get('nodes'):
                node_ratios.append(before['nodes'] / player['nodes'])
        kind = judgement(p, 1 - nxt)
        if kind:
            errors.append(dict(row, kind=kind, error_class=classify(row), p_before=p, p_after=1-nxt,
                               split=split_of(row['opening_id'])))
    blunders = [r for r in errors if r['kind'] == 'clear_blunder']
    counts = collections.Counter(r['error_class'] for r in blunders)
    return {'sampled_moves': len(rows), 'moves_judged': moves, 'unjudged_moves': unjudged,
            'clear_blunders': len(blunders), 'mistakes': len(errors)-len(blunders),
            'clear_blunders_per_100_moves': 100*len(blunders)/moves if moves else None,
            'mean_win_probability_loss': loss/moves if moves else None,
            'blunder_classes': dict(counts),
            'classes_at_thirty_percent': [k for k,v in counts.items() if k != 'other' and v >= .3*len(blunders)],
            'median_player_depth': statistics.median(player_depths) if player_depths else None,
            'median_judge_depth': statistics.median(depths) if depths else None,
            'deeper_search_share': deeper/len(depths) if depths else None,
            'median_judge_player_node_ratio': statistics.median(node_ratios) if node_ratios else None,
            'limitation': 'Same-family judge; stratified sample of moves, not all moves. Shared errors can be missed. Class frequency alone does not establish a general mechanism.',
            'errors': errors}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('judged', nargs='+')
    p.add_argument('--out', required=True)
    a = p.parse_args()
    rows = [json.loads(line) for name in a.judged for line in Path(name).read_text().splitlines()]
    result = analyze(rows)
    Path(a.out).write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps({k:v for k,v in result.items() if k != 'errors'},indent=2))


if __name__ == '__main__':
    main()
