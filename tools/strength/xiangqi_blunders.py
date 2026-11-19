"""MSH-198 (Xiangqi Blunder Screen): classify mShogi's errors in judged games.

Rules fixed before results (task MSH-198 plan). Inputs: retrieved MSH-197
ladder runs (native-replayed reports) and the collected Pikafish judgement
(`xiangqi_judge collect`). The judge is a measurement tool only.

- Clear blunder: p_before >= 0.30 and p_after <= p_before - 0.30.
- Mistake: p_before >= 0.20, loss >= 0.15, not a clear blunder.
- Class by priority: time_trouble (< 2,000 ms on mShogi's clock),
  repetition_rule (game lost by a native repetition fault within 20 plies),
  allowed_short_mate (opponent mates in <= 7 plies by the judge),
  refuted_major_capture (judge's reply captures a Chariot, Horse or Cannon),
  refuted_minor_capture (captures an Advisor, Elephant or Soldier), other.
"""
import argparse
import collections
import hashlib
import json
from pathlib import Path

from tools.strength.xiangqi_judge import extract, win_probability

START = 'rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR'
FILES = 'abcdefghi'


def board_after(moves):
    """Square -> piece letter after a native move list from the standard root."""
    board = {}
    for r, row in enumerate(START.split('/')):
        rank = 9 - r
        f = 0
        for c in row:
            if c.isdigit():
                f += int(c)
            else:
                board[f'{FILES[f]}{rank}'] = c
                f += 1
    for m in moves:
        board[m[2:]] = board.pop(m[:2])
    return board


def classify(game, turn, after, player_side):
    if turn['clock_ms'] < 2000:
        return 'time_trouble'
    c = game.get('conclusion') or {}
    reason = c.get('reason') or {}
    if (c.get('kind') == 'win' and isinstance(reason, dict) and reason.get('kind') == 'repetition'
            and c.get('winner') != player_side and len(game['moves']) - turn['index'] <= 20):
        return 'repetition_rule'
    score = after.get('score')
    if score and score[0] == 'mate' and 0 < score[1] <= 4:
        # A mate in n moves takes 2n-1 plies: n <= 4 is at most 7 plies.
        return 'allowed_short_mate'
    reply = after.get('bestmove', '')
    target = board_after(game['moves'][:turn['index'] + 1]).get(reply[2:4]) if len(reply) >= 4 else None
    if target and target.lower() in 'rnc':
        return 'refuted_major_capture'
    if target and target.lower() in 'abp':
        return 'refuted_minor_capture'
    return 'other'


def split_of(game_id):
    return 'held-out' if hashlib.sha256(game_id.encode()).digest()[0] % 4 == 0 else 'development'


def analyze(games, judged, player='mshogi'):
    errors, moves = [], 0
    for g in games:
        red_player = g['players'][g['red']]
        side = 'red' if red_player == player else 'black'
        for t in g['turns']:
            if t['player'] != player:
                continue
            before = judged.get(' '.join(g['moves'][:t['index']]))
            after = judged.get(' '.join(g['moves'][:t['index'] + 1]))
            if not before or not after:
                continue
            pb, pa_opp = win_probability(before['score']), win_probability(after['score'])
            if pb is None or pa_opp is None:
                continue
            pa = 1 - pa_opp
            moves += 1
            loss = pb - pa
            kind = ('clear_blunder' if pb >= 0.30 and pa <= pb - 0.30
                    else 'mistake' if pb >= 0.20 and loss >= 0.15 else None)
            if kind:
                errors.append({'game': g['id'], 'index': t['index'], 'kind': kind,
                               'moves': g['moves'][:t['index']], 'played': t['move'],
                               'judge_move': before['bestmove'], 'refutation': after['bestmove'],
                               'p_before': round(pb, 3), 'p_after': round(pa, 3), 'clock_ms': t['clock_ms'],
                               'class': classify(g, t, after, side), 'split': split_of(g['id'])})
    classes = collections.Counter(e['class'] for e in errors)
    return {'player': player, 'moves_judged': moves,
            'clear_blunders': sum(e['kind'] == 'clear_blunder' for e in errors),
            'mistakes': sum(e['kind'] == 'mistake' for e in errors),
            'classes': {k: {'count': v, 'share': round(v / max(len(errors), 1), 3)}
                        for k, v in classes.most_common()},
            'by_kind_and_class': {k: dict(collections.Counter(e['class'] for e in errors if e['kind'] == k))
                                  for k in ('clear_blunder', 'mistake')},
            'errors': errors}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('runs', nargs='+')
    p.add_argument('--judged', required=True)
    p.add_argument('--output', required=True)
    a = p.parse_args()
    games = extract(a.runs)
    judged = json.loads(Path(a.judged).read_text())
    result = analyze(games, judged)
    Path(a.output).write_text(json.dumps(result, indent=1))
    print(json.dumps({k: v for k, v in result.items() if k != 'errors'}, indent=1))


if __name__ == '__main__':
    main()
