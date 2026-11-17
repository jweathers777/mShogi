"""Build the MSH-190 (Shogi Selectivity Round) tactical suite from mined rows.

Input rows come from `shogi_tsume mine` (native-replayed game reports and the
native reference prover in crates/mshogi-runner/examples/support). Each row is
a diagram without earlier history whose answer set is every checking first
move that mates within the shortest mate length. The split into development
and held-out parts is by source game (SHA-256 of report digest and game index),
so positions of one game never cross the split.

Usage: python3 -m tools.strength.make_shogi_tsume MINED.jsonl OUT.json
"""
import hashlib
import json
import sys


def split_of(row):
    game = f"{row['source_report_sha256']}/{row['game_index']}"
    return 'held-out' if hashlib.sha256(game.encode()).digest()[0] % 4 == 0 else 'development'


def build(mined_path):
    rows, seen = [], set()
    for line in open(mined_path):
        r = json.loads(line)
        key = ' '.join(r['sfen'].split()[:3])
        if key in seen:
            continue
        seen.add(key)
        game = f"{r['source_report_sha256'][:12]}/{r['game_index']}"
        rows.append({
            'id': f"tsume-{r['source_report_sha256'][:8]}-g{r['game_index']}-e{r['event']}",
            'sfen': r['sfen'], 'moves': [], 'expected_moves': r['expected_moves'],
            'mate_plies': r['mate_plies'], 'phase': 'endgame', 'split': split_of(r),
            'geometry': game, 'opening_id': r['opening_id'],
            'source': (f"Position after event {r['event']} of game {r['game_index']} in native-replayed "
                       f"MSH-187 report sha256 {r['source_report_sha256']}; imported as a diagram without history"),
            'license': 'Project-generated native self-play data, BSD-LICENSE',
        })
    rows.sort(key=lambda r: (r['split'], r['id']))
    return rows


def main():
    rows = build(sys.argv[1])
    out = {'name': 'shogi-tsume-v1', 'profile': 'mshogi.shogi.csa36.v1',
           'license': 'Project-generated native self-play data, BSD-LICENSE',
           'metric': ('all checking first moves that force mate within mate_plies, the shortest mate '
                      'length; answer sets from the native reference prover, not from search output'),
           'split_rule': 'held-out when sha256(report digest/game index)[0] % 4 == 0; never cross a game',
           'positions': rows}
    with open(sys.argv[2], 'x') as f:
        json.dump(out, f, indent=1)
    held = sum(r['split'] == 'held-out' for r in rows)
    print(len(rows), 'held-out', held)


if __name__ == '__main__':
    main()
