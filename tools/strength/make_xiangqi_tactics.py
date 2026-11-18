"""Build the MSH-194 (Xiangqi Strength Harness) tactical suite from mined rows.

Input rows come from `xiangqi_strength mine` (native-replayed clocked game
reports and the native reference prover in
crates/mshogi-runner/examples/support/xiangqi_mate_proof.rs). Each row is a
diagram without earlier history whose answer set is every checking first
move that mates within the shortest mate length (3, 5 or 7 plies). Equal
boards are kept once. One mating sequence gives several consecutive rows, so
each source game keeps at most PER_GAME rows, longest mates first. The split
into development and held-out parts is by source game (SHA-256 of report
digest), so positions of one game never cross the split.

Usage: python3 -m tools.strength.make_xiangqi_tactics OUT.json MINED.jsonl [MINED.jsonl ...]
"""
import collections
import hashlib
import json
import sys

PER_GAME = 3


def split_of(row):
    return 'held-out' if hashlib.sha256(row['source_report_sha256'].encode()).digest()[0] % 4 == 0 \
        else 'development'


def build(paths):
    rows, seen = [], set()
    by_game = collections.defaultdict(list)
    for path in paths:
        for line in open(path):
            r = json.loads(line)
            by_game[r['source_report_sha256']].append(r)
    for digest in sorted(by_game):
        kept = 0
        for r in sorted(by_game[digest], key=lambda r: (-r['mate_plies'], r['ply'])):
            board = ' '.join(r['fen'].split()[:2])
            if board in seen or kept >= PER_GAME:
                continue
            seen.add(board)
            kept += 1
            rows.append({
                'id': f"mate-{digest[:10]}-p{r['ply']}",
                'fen': r['fen'], 'moves': [], 'expected_moves': r['expected_moves'],
                'mate_plies': r['mate_plies'], 'split': split_of(r), 'opening_id': r['opening_id'],
                'source': (f"Position before reply {r['ply']} of native-replayed clocked report sha256 "
                           f"{digest} ({r['game']}); imported as a diagram without history"),
                'license': 'Project-generated native self-play data, BSD-LICENSE',
            })
    rows.sort(key=lambda r: (r['split'], r['id']))
    return rows


def main():
    rows = build(sys.argv[2:])
    out = {'name': 'xiangqi-tactics-v1', 'profile': 'mshogi.xiangqi.wxf2018.v1',
           'license': 'Project-generated native self-play data, BSD-LICENSE',
           'metric': ('all checking first moves that force mate within mate_plies, the shortest mate '
                      'length (3-7 plies); answer sets from the native reference prover, not from search output'),
           'split_rule': 'held-out when sha256(report digest)[0] % 4 == 0; never cross a game',
           'per_game': PER_GAME,
           'positions': rows}
    with open(sys.argv[1], 'x') as f:
        json.dump(out, f, indent=1)
    held = sum(r['split'] == 'held-out' for r in rows)
    print(len(rows), 'held-out', held, collections.Counter(r['mate_plies'] for r in rows))


if __name__ == '__main__':
    main()
