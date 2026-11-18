"""MSH-193 (Shogi Blunder Screen): apply the frozen blunder rule and classify.

Inputs: games.json from `shogi_judge prepare` and judged.json from
`shogi_judge collect`. A clear blunder is an mShogi move with p_before >= 0.30
and p_after <= p_before - 0.30, where p is the judge's win probability for
mShogi (p = 1/(1+exp(-cp/600)); mate scores map to 1 or 0). Each blunder gets
one mechanism by priority: time trouble (< 2 s on mShogi's clock), allowed
short mate (judge mate in <= 7 for the opponent), refutation captures a piece,
refutation is a drop, refutation lands next to mShogi's King, otherwise
'other'. The judge is a measurement tool only.
"""
import argparse
import collections
import json
from pathlib import Path

from tools.strength.shogi_judge import key, win_probability

FILES = '987654321'


def board(sfen):
    """Map USI square ('5e') to piece letter for an SFEN board."""
    squares = {}
    for r, row in enumerate(sfen.split()[0].split('/')):
        file_index, i = 0, 0
        while i < len(row):
            c = row[i]
            if c.isdigit():
                file_index += int(c)
                i += 1
                continue
            piece = c
            if c == '+':
                piece = row[i:i + 2]
                i += 2
            else:
                i += 1
            squares[FILES[file_index] + 'abcdefghi'[r]] = piece
            file_index += 1
    return squares


def king_square(sfen, sente):
    for square, piece in board(sfen).items():
        if piece == ('K' if sente else 'k'):
            return square
    return None


def adjacent(a, b):
    return a and b and abs(int(a[0]) - int(b[0])) <= 1 and abs(ord(a[1]) - ord(b[1])) <= 1


def classify(turn, judged_before, judged_after, after_sfen, mshogi_sente):
    side = 0 if mshogi_sente else 1
    if turn['clock_before_ms'][side] < 2000:
        return 'time_trouble'
    score = judged_after.get('score')
    if score and score[0] == 'mate' and 0 < score[1] <= 7:
        return 'allowed_short_mate'
    reply = judged_after.get('bestmove', '')
    if '*' in reply:
        return 'drop_refutation'
    if len(reply) >= 4:
        target = reply[2:4]
        if target in board(after_sfen):
            return 'capture_refutation'
        if adjacent(target, king_square(after_sfen, mshogi_sente)):
            return 'king_zone_refutation'
    return 'other'


def analyze(games, judged, sfens):
    blunders, moves = [], 0
    by_group = collections.Counter()
    for game in games:
        mshogi_sente = game['candidate_sente']
        for turn in game['turns']:
            if turn['engine'] != 0 or turn['after'] is None:
                continue
            before, after = judged.get(turn['before']), judged.get(turn['after'])
            if before is None or after is None:
                continue
            p_before = win_probability(before['score'])
            p_after_opponent = win_probability(after['score'])
            if p_before is None or p_after_opponent is None:
                continue
            p_after = 1 - p_after_opponent
            moves += 1
            if p_before >= 0.30 and p_after <= p_before - 0.30:
                mechanism = classify(turn, before, after, sfens[turn['after']], mshogi_sente)
                group = ('elmo' if game['opponent'] == 'elmo-1000k' else 'gikou',
                         'win' if game['score'] == 1 else 'loss')
                by_group[group] += 1
                blunders.append({'game': game['id'], 'opponent': game['opponent'], 'version': turn['version'],
                                 'sfen': sfens[turn['before']], 'played': turn['reply'].split()[1],
                                 'judge_move': before['bestmove'], 'refutation': after['bestmove'],
                                 'p_before': round(p_before, 3), 'p_after': round(p_after, 3),
                                 'clock_ms': turn['clock_before_ms'][0 if mshogi_sente else 1],
                                 'mechanism': mechanism, 'group': list(group),
                                 'split': 'heldout' if hash_game(game['id']) else 'development'})
    shares = collections.Counter(b['mechanism'] for b in blunders)
    return {'mshogi_moves_judged': moves, 'clear_blunders': len(blunders),
            'per_100_moves': round(100 * len(blunders) / max(moves, 1), 2),
            'games_with_blunder': len({b['game'] for b in blunders}),
            'by_group': {f'{a}-{b}': n for (a, b), n in sorted(by_group.items())},
            'mechanisms': {k: {'count': v, 'share': round(v / max(len(blunders), 1), 3)}
                           for k, v in shares.most_common()},
            'blunders': blunders}


def hash_game(game_id):
    import hashlib
    return hashlib.sha256(game_id.encode()).digest()[0] % 4 == 0


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--games', required=True)
    p.add_argument('--judged', required=True)
    p.add_argument('--positions', required=True, help='manifest.json from shogi_judge prepare')
    p.add_argument('--output', required=True)
    a = p.parse_args()
    games = json.loads(Path(a.games).read_text())
    judged = json.loads(Path(a.judged).read_text())
    manifest = json.loads(Path(a.positions).read_text())
    sfens = {key(s): s for b in manifest['bins'] for s in b}
    result = analyze(games, judged, sfens)
    Path(a.output).write_text(json.dumps(result, indent=1))
    print(json.dumps({k: v for k, v in result.items() if k != 'blunders'}, indent=1))


if __name__ == '__main__':
    main()
