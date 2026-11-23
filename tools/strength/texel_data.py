"""Prepare self-generated Texel data; never use search scores as result labels.

Generate with the frozen chess_selfplay example, then run:
python3 -m tools.strength.texel_data split CORPUS_DIR MANIFEST.json
Raw games and derived datasets stay outside Git; the manifest records their hashes.
"""
import argparse
import collections
import hashlib
import json
from pathlib import Path


FIELDS = {'chess': 4, 'shogi': 3, 'xiangqi': 2, 'tori': None, 'chu': None}
POSITION = {'chess': 'fen', 'shogi': 'sfen', 'xiangqi': 'fen', 'tori': 'diagram', 'chu': 'diagram'}
OUTCOME = {'chess': 'white_outcome', 'shogi': 'sente_outcome', 'xiangqi': 'red_outcome', 'tori': 'sente_outcome', 'chu': 'sente_outcome'}


def key(fen, game='chess'):
    # Board identity: Chess FEN without clocks, SFEN board, side and hands
    # (its fourth field is only the move counter), Xiangqi FEN board and side,
    # Tori compact diagram (board, both hands, side; MSH-204) as canonical JSON.
    if isinstance(fen, dict):
        return json.dumps(fen, sort_keys=True, separators=(',', ':'))
    return ' '.join(fen.split()[:FIELDS[game]])


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def split(root, manifest_path, game='chess', allow_empty=False, by_opening=False):
    if game not in FIELDS:
        raise ValueError('unsupported game')
    if game in ('tori', 'chu') and not by_opening:
        raise ValueError('Tori/Chu exports carry no initial diagram; split --by-opening')
    position_field = POSITION[game]
    outcome_field = OUTCOME[game]
    files = sorted(root.glob('games-*.jsonl'))
    games = sorted((json.loads(line) for path in files for line in path.read_text().splitlines()),
                   key=lambda g: g['game_id'])
    assert games and len({g['game_id'] for g in games}) == len(games)
    partitions = {}
    boards = collections.defaultdict(set)
    rows = []
    for g in games:
        # Equal initial boards (even from transposed opening moves) are one family.
        source = g['opening_id'] if by_opening else key(g['initial_' + position_field], game)
        family = hashlib.sha256(source.encode()).hexdigest()
        bucket = int(family[:16], 16) % 10
        part = 'heldout' if bucket == 0 else 'validation' if bucket == 1 else 'train'
        partitions[g['game_id']] = part
        if g[outcome_field] is None:
            assert not g['samples']
        for sample in g['samples']:
            assert g['outcome'] is not None and not g['truncated']
            assert sample[outcome_field] == g[outcome_field]
            assert sample[outcome_field] in (0, .5, 1)
            row = {position_field: sample[position_field], 'result': sample[outcome_field],
                   'split': part, 'game_id': g['game_id'], 'family_id': family}
            boards[key(row[position_field], game)].add(part)
            rows.append(row)
    # Remove every occurrence of a board shared across partitions, including
    # clocks that differ. Within one partition retain its first occurrence only.
    seen = set()
    selected = collections.defaultdict(list)
    cross = 0
    for row in rows:
        identity = key(row[position_field], game)
        if len(boards[identity]) > 1:
            cross += 1
            continue
        if identity in seen:
            continue
        seen.add(identity)
        selected[row['split']].append(row)
    outputs = {}
    for part in ['train', 'validation', 'heldout']:
        assert selected[part] or allow_empty, f'empty {part}; add opening families before tuning'
    for part in ['train', 'validation', 'heldout']:
        path = root / (part + '.jsonl')
        with path.open('x') as f:
            for row in selected[part]:
                f.write(json.dumps(row, separators=(',', ':')) + '\n')
        outputs[part] = {'path': str(path), 'sha256': digest(path),
                         'positions': len(selected[part]),
                         'games': len({r['game_id'] for r in selected[part]})}
    manifest = {
        'schema': 1, 'game': game, 'source': 'Fresh mShogi self-play; native final outcomes',
        'training_ready': all(selected[p] for p in ['train', 'validation', 'heldout']),
        'allow_empty_demonstration': allow_empty,
        'use': 'User-authorized internal classical tuning; no imported corpus',
        'project_terms': 'BSD-LICENSE',
        'split_rule': ('SHA256(catalog opening id)' if by_opening else 'SHA256(initial board/side/rights-or-hands)')
                      + ' first64bits mod10:0heldout,1validation,2..9train',
        'dedup_rule': 'Remove all cross-partition board/side/rights duplicates; retain first within-partition board',
        'inputs': {str(p): digest(p) for p in files},
        'games': len(games), 'stops': dict(collections.Counter(g['stop'] for g in games)),
        'raw_positions': len(rows), 'cross_partition_rows_removed': cross,
        'remaining_within_partition_duplicates_removed': len(rows) - cross - len(seen),
        'outputs': outputs,
    }
    with manifest_path.open('x') as f:
        json.dump(manifest, f, indent=2)
        f.write('\n')
    print(json.dumps(outputs, indent=2))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=['split'])
    parser.add_argument('corpus', type=Path)
    parser.add_argument('manifest', type=Path)
    parser.add_argument('--game', choices=sorted(FIELDS), default='chess')
    parser.add_argument('--by-opening', action='store_true',
                        help='family = catalog opening id (use when random plies follow the catalog prefix)')
    parser.add_argument('--allow-empty-demonstration', action='store_true',
                        help='write empty partitions for a small generation demonstration; not training-ready')
    args = parser.parse_args()
    split(args.corpus, args.manifest, args.game, args.allow_empty_demonstration, args.by_opening)


if __name__ == '__main__':
    main()
