"""Split native outcome-labelled self-play data by source family and board identity."""
import argparse
import collections
import hashlib
import json
from pathlib import Path
from tools.experiments.core import file_hash as digest


FIELDS = {'chess': 4, 'shogi': 3, 'xiangqi': 2, 'tori': None, 'chu': None, 'tenjiku': None}
POSITION = {'chess': 'fen', 'shogi': 'sfen', 'xiangqi': 'fen', 'tori': 'diagram', 'chu': 'diagram', 'tenjiku': 'diagram'}
OUTCOME = {'chess': 'white_outcome', 'shogi': 'sente_outcome', 'xiangqi': 'red_outcome', 'tori': 'sente_outcome', 'chu': 'sente_outcome', 'tenjiku': 'sente_outcome'}


def key(fen, game='chess'):
    # Board identity: Chess FEN without clocks, SFEN board, side and hands
    # (its fourth field is only the move counter), Xiangqi FEN board and side,
    # Tori compact diagram (board, both hands, side; MSH-204) as canonical JSON.
    if isinstance(fen, dict):
        if game in ('chu','tenjiku'):
            fen = dict(fen, pieces=sorted(fen['pieces'], key=lambda p: json.dumps(p, sort_keys=True, separators=(',', ':'))))
        return json.dumps(fen, sort_keys=True, separators=(',', ':'))
    return ' '.join(fen.split()[:FIELDS[game]])


def game_rows(files):
    """Read one game at a time; a Chu corpus can contain gigabytes of diagrams."""
    for path in files:
        with path.open() as stream:
            for line in stream:
                yield json.loads(line)


def samples(g, game, by_opening):
    position_field, outcome_field = POSITION[game], OUTCOME[game]
    source = g['opening_id'] if by_opening else key(g['initial_' + position_field], game)
    family = hashlib.sha256(source.encode()).hexdigest()
    bucket = int(family[:16], 16) % 10
    part = 'heldout' if bucket == 0 else 'validation' if bucket == 1 else 'train'
    if g[outcome_field] is None:
        assert not g['samples']
    for sample in g['samples']:
        assert g['outcome'] is not None and not g['truncated']
        assert sample[outcome_field] == g[outcome_field]
        assert sample[outcome_field] in (0, .5, 1)
        row = {position_field: sample[position_field], 'result': sample[outcome_field],
               'split': part, 'game_id': g['game_id'], 'family_id': family}
        identity = hashlib.sha256(key(row[position_field], game).encode()).digest()
        yield identity, row


def split(root, manifest_path, game='chess', allow_empty=False, by_opening=False, profile=None):
    if game not in FIELDS:
        raise ValueError('unsupported game')
    if game in ('tori', 'chu', 'tenjiku') and not by_opening:
        raise ValueError('Diagram exports carry no initial diagram; split --by-opening')
    files = sorted(root.glob('games-*.jsonl'))
    game_ids, stops = set(), collections.Counter()
    boards = {}
    raw = 0
    masks = {'train': 1, 'validation': 2, 'heldout': 4}
    # Only board hashes and partition bits stay in memory. Re-read the frozen
    # input to write rows after all cross-partition duplicates are known.
    for g in game_rows(files):
        assert g['game_id'] not in game_ids, 'duplicate game ID'
        game_ids.add(g['game_id'])
        stops[g['stop']] += 1
        for identity, row in samples(g, game, by_opening):
            boards[identity] = boards.get(identity, 0) | masks[row['split']]
            raw += 1
    assert game_ids, 'empty corpus'
    counts = collections.Counter(mask for mask in boards.values() if mask in masks.values())
    for part, mask in masks.items():
        assert counts[mask] or allow_empty, f'empty {part}; add opening families before tuning'
    selected_games = {part: set() for part in masks}
    cross = 0
    seen = set()
    from contextlib import ExitStack
    with ExitStack() as stack:
        streams = {part: stack.enter_context((root / (part + '.jsonl')).open('x')) for part in masks}
        for g in game_rows(files):
            for identity, row in samples(g, game, by_opening):
                part = row['split']
                if boards[identity] != masks[part]:
                    cross += 1
                    continue
                if identity in seen:
                    continue
                seen.add(identity)
                selected_games[part].add(row['game_id'])
                streams[part].write(json.dumps(row, separators=(',', ':')) + '\n')
    outputs = {}
    for part, mask in masks.items():
        path = root / (part + '.jsonl')
        outputs[part] = {'path': str(path), 'sha256': digest(path),
                         'positions': counts[mask], 'games': len(selected_games[part])}
    manifest = {
        'schema': 1, 'game': game, 'profile': profile, 'source': 'Fresh mShogi self-play; native final outcomes',
        'training_ready': all(counts[mask] for mask in masks.values()),
        'allow_empty_demonstration': allow_empty,
        'use': 'User-authorized internal classical tuning; no imported corpus',
        'project_terms': 'BSD-LICENSE',
        'split_rule': ('SHA256(catalog opening id)' if by_opening else 'SHA256(initial board/side/rights-or-hands)')
                      + ' first 64 bits mod 10: 0 heldout, 1 validation, 2..9 train',
        'dedup_rule': 'Remove all cross-partition board/side/rights duplicates; retain first within-partition board',
        'inputs': {str(p): digest(p) for p in files},
        'games': len(game_ids), 'stops': dict(stops),
        'raw_positions': raw, 'cross_partition_rows_removed': cross,
        'remaining_within_partition_duplicates_removed': raw - cross - len(seen),
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
