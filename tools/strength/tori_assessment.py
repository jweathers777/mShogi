"""MSH-207 (Tori Strength Assessment): judged move quality and blunder screen.

Rules fixed before results (task MSH-207 plan); the shared definitions are in
`blunder_rules`. Input: the outputs of `tori_strength judge`, a fresh mShogi
search of every position of self-play games at 5,000,000 nodes. The judge is
the same engine family as the player, so it is weaker evidence than an
independent stronger engine, and errors that both share stay unseen. Every
move of the judged games is an mShogi move.

- A score is from the side to move; |score| >= MATE_BOUND is a mate in
  MATE_SCORE - |score| plies. A concluded position is won or lost (Tori has
  no draw). The mover's p_after is 1 - p of the next position.
- Class of an error, first match:
  time_trouble (the mover's clock is below 1,000 ms before the move),
  repetition (the position before the move occurred earlier in the game),
  allowed_short_mate (the judge mates the mover within 9 plies after it),
  missed_capture (the judge's move captures, the played move does not),
  drop_refutation (the judge's move after the error is a drop),
  refuted_major_capture (it captures a Falcon, Eagle, Crane or Quail),
  refuted_minor_capture (it captures a Pheasant, Goose or Swallow),
  other.
- Fix rule: one round only if one class other than `other` has at least 30%
  of the clear blunders and maps to a general change.
"""
import argparse
import collections
import json
from pathlib import Path

from tools.strength.blunder_rules import judgement, split_of, win_probability

MATE_SCORE = 1_000_000
MATE_BOUND = MATE_SCORE - 256
SHORT_MATE_PLIES = 9
MAJOR = {'Fa', 'Ea', 'Cr', 'LQ', 'RQ'}
FIX_SHARE = 0.30


def score_of(row):
    """Judge score of a position row as ('cp', value) or ('mate', sign), or None."""
    if 'terminal' in row:
        return ('mate', -1 if row['terminal']['lost'] else 1)
    s = row.get('score')
    if s is None:
        return None
    if abs(s) >= MATE_BOUND:
        return ('mate', 1 if s > 0 else -1)
    return ('cp', s)


def classify(row, after, repeated):
    if row.get('clock_ms') is not None and row['clock_ms'] < 1000:
        return 'time_trouble'
    if repeated:
        return 'repetition'
    if after.get('score') is not None and after['score'] >= MATE_SCORE - SHORT_MATE_PLIES:
        return 'allowed_short_mate'
    if row.get('best_captures') and not row.get('played_captures'):
        return 'missed_capture'
    if after.get('best_drop'):
        return 'drop_refutation'
    victim = after.get('best_captures')
    if victim in MAJOR:
        return 'refuted_major_capture'
    if victim:
        return 'refuted_minor_capture'
    return 'other'


def analyze(judged):
    """Move quality and classified errors of judge outputs (parsed JSON)."""
    errors, moves, unjudged, loss = [], 0, 0, 0.0
    for output in judged:
        for game in output['games']:
            rows = sorted(game['positions'], key=lambda r: r['ply'])
            seen = set()
            for row, after in zip(rows, rows[1:]):
                key = json.dumps(row['diagram'], sort_keys=True)
                repeated = key in seen
                seen.add(key)
                if 'played' not in row:
                    continue
                p_before, p_next = win_probability(score_of(row)), win_probability(score_of(after))
                if p_before is None or p_next is None:
                    unjudged += 1
                    continue
                p_after = 1 - p_next
                moves += 1
                loss += max(0.0, p_before - p_after)
                kind = judgement(p_before, p_after)
                if kind:
                    errors.append({'game': game['game_id'], 'opening_id': game['opening_id'], 'ply': row['ply'],
                                   'side': row['side'], 'diagram': row['diagram'], 'kind': kind,
                                   'class': classify(row, after, repeated), 'played': row['played'],
                                   'judge_move': row.get('best'), 'refutation': after.get('best'),
                                   'p_before': round(p_before, 3), 'p_after': round(p_after, 3),
                                   'clock_ms': row.get('clock_ms'), 'split': split_of(game['game_id'])})
    blunders = [e for e in errors if e['kind'] == 'clear_blunder']
    classes = collections.Counter(e['class'] for e in blunders)
    shares = {k: {'count': v, 'share': round(v / len(blunders), 3)} for k, v in classes.most_common()}
    top = max((k for k in classes if k != 'other'), key=classes.get, default=None)
    return {'moves_judged': moves, 'unjudged_moves': unjudged,
            'games': sum(len(o['games']) for o in judged),
            'judge': {k: judged[0][k] for k in ('nodes', 'evaluator_id', 'search_id')} if judged else None,
            'clear_blunders': len(blunders),
            'clear_blunders_per_100_moves': round(100 * len(blunders) / max(moves, 1), 2),
            'mistakes': len(errors) - len(blunders),
            'mistakes_per_100_moves': round(100 * (len(errors) - len(blunders)) / max(moves, 1), 2),
            'mean_win_probability_loss': round(loss / max(moves, 1), 4),
            'blunder_classes': shares,
            'fix_rule': {'class': top, 'share': shares[top]['share'] if top else 0,
                         'triggers': bool(top) and shares[top]['share'] >= FIX_SHARE},
            'errors': errors}


def suite(result, nodes):
    """Blunder suite: one row per error position, split by game."""
    rows = [{'id': f"{e['game'][:12]}-{e['game'].split('/')[1]}-p{e['ply']}", 'diagram': e['diagram'],
             'kind': e['kind'], 'class': e['class'], 'played': e['played'], 'judge_move': e['judge_move'],
             'refutation': e['refutation'], 'p_before': e['p_before'], 'p_after': e['p_after'],
             'clock_ms': e['clock_ms'], 'opening_id': e['opening_id'], 'split': e['split'],
             'source': (f"MSH-207 self-play game {e['game']} (5 s + 0.1 s), board ply {e['ply']} after "
                        f"the opening, judged by mShogi at {nodes:,} nodes")}
            for e in result['errors']]
    rows.sort(key=lambda r: (r['split'], r['id']))
    return {'name': 'tori-blunders-v1', 'profile': 'mshogi.tori.modern.v1',
            'license': 'Project-generated data, BSD-LICENSE',
            'metric': ('avoid: the engine plays a move other than `played`; better: the judge no longer rates the '
                       'new move an error. `judge_move` is one good move, not the only one. The diagram has no '
                       'game history (class `repetition` depends on it).'),
            'definitions': ('p = 1/(1+exp(-cp/600)); clear blunder p_before >= 0.30 and p_after <= p_before - 0.30; '
                            'mistake p_before >= 0.20 and loss >= 0.15; same-family judge (mShogi)'),
            'split_rule': 'held-out when sha256(game id)[0] % 4 == 0; never across a game',
            'positions': rows}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('judged', nargs='+', help='tori_strength judge outputs')
    p.add_argument('--output', required=True)
    p.add_argument('--suite')
    a = p.parse_args()
    judged = [json.loads(Path(f).read_text()) for f in a.judged]
    result = analyze(judged)
    Path(a.output).write_text(json.dumps(result, indent=1))
    if a.suite:
        Path(a.suite).write_text(json.dumps(suite(result, judged[0]['nodes']), indent=1) + '\n')
    print(json.dumps({k: v for k, v in result.items() if k != 'errors'}, indent=1))


if __name__ == '__main__':
    main()
