"""Write the candidate MSH-194 (Xiangqi Strength Harness) opening catalog.

Each trunk is an original UCI transcription (files a-i from Red's left, ranks
0-9 from Red's back rank) of a conventional Xiangqi opening structure; the
style names follow common English usage (central cannon, screen horses, same-
and opposite-direction cannons, elephant, pawn, horse and palace-corner
cannon openings). Each candidate extends a trunk by one two-move development
plan for each side, interleaved Red, Black, Red, Black. Native validation
decides legality (plans whose moves were blocked in the first screen were
corrected for legality only, before the final screen); an engine balance screen with a fixed rule decides
admission. No game record, book line or commentary is copied. CC0-1.0.

Usage:
  python3 -m tools.strength.make_xiangqi_openings candidates OUT.json
  python3 -m tools.strength.make_xiangqi_openings finalize CANDIDATES.json SCREEN.json ENGINE_SHA256 OUT.json
"""
import itertools
import json
import random
import sys

# name: (trunk, red plans, black plans). Trunks end with Red to move.
TRUNKS = {
    'central-cannon-screen-horses': (
        'h2e2 h9g7 h0g2 i9h9 i0h0 b9c7',
        ['h0h4 c3c4', 'g3g4 b0c2', 'c3c4 b0c2', 'h0h6 b0c2'],
        ['g6g5 c9e7', 'c6c5 h7i7', 'b7a7 a9b9', 'g9e7 f9e8']),
    'same-direction-cannons': (
        'h2e2 h7e7 h0g2 h9g7 i0h0 i9h9',
        ['h0h6 b0c2', 'b0c2 c3c4', 'g3g4 h0h4', 'b2c2 b0a2'],
        ['b9c7 h9h5', 'b9c7 c6c5', 'g6g5 b9c7', 'b9a7 a9b9']),
    'opposite-direction-cannons': (
        'h2e2 b7e7 h0g2 b9c7 i0h0 a9b9',
        ['h0h6 b0c2', 'b0c2 b2b6', 'c3c4 b0c2', 'g3g4 h0h4'],
        ['h9g7 i9h9', 'b9b5 h9g7', 'g6g5 h9g7', 'c6c5 h9g7']),
    'central-cannon-palace-corner-defence': (
        'h2e2 h9g7 h0g2 b7d7 i0h0 b9c7',
        ['h0h6 b0c2', 'c3c4 b0c2', 'g3g4 b0a2', 'b2c2 b0a2'],
        ['a9b9 c9e7', 'i9i8 a9b9', 'g6g5 c9e7', 'f9e8 a9b9']),
    'elephant-opening': (
        'g0e2 c6c5 h0g2 b9c7 i0h0 a9b9',
        ['b0c2 b2a2', 'c3c4 b0c2', 'h0h1 b0a2', 'g3g4 b0c2'],
        ['h9g7 i9h9', 'b7b3 h9g7', 'h7h3 h9g7', 'g6g5 h9g7']),
    'pawn-opening-cannon-reply': (
        'c3c4 b7c7 h2e2 h9g7 h0g2 i9h9',
        ['i0h0 b0c2', 'b0a2 a0b0', 'g3g4 i0h0', 'b2b6 i0h0'],
        ['b9a7 a9b9', 'g6g5 b9a7', 'c9e7 b9a7', 'd9e8 b9a7']),
    'pawn-opening-pawn-reply': (
        'c3c4 g6g5 h0g2 h9g7 i0h0 i9h9',
        ['b2e2 b0c2', 'b0c2 b2b6', 'g0e2 b0c2', 'h2h6 b0c2'],
        ['b9c7 b7a7', 'h7h3 b9c7', 'c9e7 b9c7', 'b7e7 b9c7']),
    'horse-opening': (
        'h0g2 h9g7 i0h0 i9h9 c3c4 c6c5',
        ['b0c2 b2a2', 'b2e2 b0c2', 'g0e2 b0c2', 'h2h6 b0c2'],
        ['b7e7 b9c7', 'b7d7 b9c7', 'c9e7 b9c7', 'g6g5 b9c7']),
    'cross-palace-cannon': (
        'h2d2 h9g7 h0g2 i9h9 i0h0 b9c7',
        ['b0c2 a0b0', 'g3g4 b0c2', 'c3c4 b0c2', 'h0h4 b0c2'],
        ['c9e7 a9b9', 'c6c5 a9b9', 'g6g5 c9e7', 'b7b3 a9b9']),
    'palace-corner-cannon': (
        'b2d2 h9g7 b0c2 i9h9 a0b0 h7i7',
        ['h0g2 i0h0', 'h2e2 h0g2', 'g3g4 h0g2', 'c3c4 h0g2'],
        ['b9c7 a9b9', 'b7e7 b9c7', 'g6g5 b9c7', 'c9e7 b9c7']),
    'central-cannon-river-chariot': (
        'h2e2 h9g7 h0g2 i9h9 i0h0 b9c7 h0h6 c6c5',
        ['b0c2 c3c4', 'g3g4 b0c2', 'b0a2 a0b0', 'b2c2 b0a2'],
        ['b7a7 a9b9', 'c9e7 b7b3', 'f9e8 a9b9', 'h7i7 h9h8']),
    'central-cannon-flank-cannon': (
        'h2e2 h9g7 h0g2 i9h9 i0h0 b9c7 b2c2 c9e7',
        ['b0a2 a0b0', 'h0h4 b0a2', 'g3g4 b0a2', 'c3c4 b0a2'],
        ['a9b9 b7b3', 'f9e8 a9b9', 'g6g5 a9b9', 'h7i7 a9b9']),
    'central-cannon-early-chariot-defence': (
        'h2e2 b9c7 h0g2 b7a7 i0h0 a9b9',
        ['h0h4 b0c2', 'c3c4 b0c2', 'g3g4 b0c2', 'h0h6 b0c2'],
        ['h9g7 i9h9', 'b9b5 h9g7', 'h9i7 i9h9', 'g6g5 h9g7']),
    'central-cannon-advanced-cannon-defence': (
        'h2e2 h9g7 h0g2 h7h3 i0h0 i9h9',
        ['b0c2 c3c4', 'g3g4 b0c2', 'h0h1 b0c2', 'b2b6 b0c2'],
        ['b9c7 c6c5', 'b9c7 a9a8', 'g6g5 b9c7', 'c9e7 b9c7']),
}

SOURCE = ('Original MSH-194 UCI transcription of a conventional Xiangqi opening structure '
          '(common English style names); trunk plus one independent development plan per side; '
          'no copied game record, book line or commentary.')


def candidates():
    rows = []
    for name, (trunk, red, black) in TRUNKS.items():
        for i, j in itertools.product(range(len(red)), range(len(black))):
            r, b = red[i].split(), black[j].split()
            moves = trunk.split() + [r[0], b[0], r[1], b[1]]
            rows.append({'id': f'{name}-r{i + 1}b{j + 1}', 'family': name, 'fen': None,
                         'moves': moves, 'source': SOURCE, 'license': 'CC0-1.0'})
    return rows


def finalize(candidates_path, screen_path, engine_sha256, out_path, limit=150, seed=194, depth=None):
    """Admit natively legal candidates whose fixed-depth score is within `limit`."""
    rows = json.load(open(candidates_path))['openings']
    screen = {r['id']: r for r in json.load(open(screen_path))['positions']}
    depths = {r['depth'] for r in screen.values() if r.get('legal')}
    assert len(depths) == 1, depths
    depth = depths.pop()
    admitted, rejected = [], {'illegal': 0, 'unbalanced': 0}
    for r in rows:
        s = screen[r['id']]
        if not s['legal']:
            rejected['illegal'] += 1
        elif abs(s['score']) > limit:
            rejected['unbalanced'] += 1
        else:
            admitted.append({**r, 'screen_score': s['score']})
    random.Random(seed).shuffle(admitted)
    out = {'name': 'xiangqi-openings-v1', 'license': 'CC0-1.0',
           'balance': (f'admitted when the frozen predecessor engine (sha256 {engine_sha256}) scored the final '
                       f'position (Red to move) within +-{limit} soldier hundredths at fixed depth {depth}; '
                       'rule fixed before screening; paired colors; not a certified equal-evaluation claim'),
           'order': f'shuffled with random.Random({seed})',
           'rejected': rejected,
           'openings': admitted}
    with open(out_path, 'x') as f:
        json.dump(out, f, indent=1)
    return len(admitted), rejected


def main():
    if sys.argv[1] == 'candidates':
        rows = candidates()
        assert len({r['id'] for r in rows}) == len(rows)
        assert len({tuple(r['moves']) for r in rows}) == len(rows)
        out = {'name': 'xiangqi-openings-v1-candidates', 'license': 'CC0-1.0', 'openings': rows}
        with open(sys.argv[2], 'x') as f:
            json.dump(out, f, indent=1)
        print(len(rows))
    elif sys.argv[1] == 'finalize':
        print(finalize(*sys.argv[2:6]))
    else:
        raise SystemExit(__doc__)


if __name__ == '__main__':
    main()
