"""Build the MSH-204 (Tori Strength Harness) tactical suite from mined rows.

Input rows come from `tori_strength mine` (native-replayed experiment reports
and the native reference prover in
crates/mshogi-runner/examples/support/tori_mate_proof.rs). Each row is a
diagram without earlier history whose checking answer set is every checking
first action (drops included) that wins within the shortest checking win
length (3, 5 or 7 plies). Equal diagrams are kept once. One mating sequence
gives several consecutive rows, so each source game keeps at most PER_GAME
rows, longest wins first. The split into development and held-out parts is by
source report (SHA-256 of the report digest), so positions of one game, or of
the two games of one opening pair, never cross the split. `tori_strength
finalize` then drops positions with a one-ply win.

Usage: python3 -m tools.strength.make_tori_tactics OUT.json MINED.jsonl [MINED.jsonl ...]
"""
import collections
import hashlib
import json
import sys

PER_GAME = 3


def split_of(digest):
    return 'held-out' if hashlib.sha256(digest.encode()).digest()[0] % 4 == 0 else 'development'


def build(paths):
    rows, seen = [], set()
    by_game = collections.defaultdict(list)
    for path in paths:
        for line in open(path):
            r = json.loads(line)
            by_game[(r['source_report_sha256'], r['game'])].append(r)
    for digest, game in sorted(by_game):
        kept = 0
        for r in sorted(by_game[(digest, game)], key=lambda r: (-r['mate_plies'], r['event'])):
            board = json.dumps(r['diagram'], sort_keys=True)
            if board in seen or kept >= PER_GAME:
                continue
            seen.add(board)
            kept += 1
            rows.append({
                'id': f"mate-{digest[:10]}-g{game}-e{r['event']}",
                'diagram': r['diagram'], 'checking_answers': r['checking_answers'],
                'mate_plies': r['mate_plies'], 'split': split_of(digest), 'opening_id': r['opening_id'],
                'played': r['played'], 'played_in_answers': r['played_in_answers'],
                'source': (f"Position before event {r['event']} of game {game} of native-replayed experiment "
                           f"report sha256 {digest}; imported as a diagram without history"),
                'license': 'Project-generated native self-play data, BSD-LICENSE',
            })
    rows.sort(key=lambda r: (r['split'], r['id']))
    return rows


def main():
    rows = build(sys.argv[2:])
    out = {'name': 'tori-tactics-v1', 'profile': 'mshogi.tori.modern.v1',
           'license': 'Project-generated native self-play data, BSD-LICENSE',
           'notation': 'native Tori notation (mshogi_core::tori::notation)',
           'metric': ('pass when the chosen action wins (checkmate or quiet immobility, both losses for the side '
                      'without a legal action) within mate_plies, the shortest checking win (3-7 plies). '
                      'checking_answers lists every checking first action that wins with a checking attacker; an '
                      'action outside it passes only when the native full-width prover (every attacker action, '
                      'every defender reply) proves it wins within mate_plies, at scoring time '
                      '(tori_strength probe). Positions with a one-ply win are excluded (tori_strength finalize). '
                      'Answers come from the native prover, not from search output.'),
           'split_rule': 'held-out when sha256(report digest)[0] % 4 == 0; never cross a game or an opening pair',
           'answer_rule': ('native prover (crates/mshogi-runner/examples/support/tori_mate_proof.rs); a checking '
                           'attacker for the stored answers, a full-width attacker for any other played action'),
           'per_game': PER_GAME,
           'positions': rows}
    with open(sys.argv[1], 'x') as f:
        json.dump(out, f, indent=1)
    held = sum(r['split'] == 'held-out' for r in rows)
    print(len(rows), 'held-out', held, collections.Counter(r['mate_plies'] for r in rows))


if __name__ == '__main__':
    main()
