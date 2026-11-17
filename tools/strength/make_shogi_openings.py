"""Write the candidate MSH-190 (Shogi Selectivity Round) opening catalog.

Each trunk is an original USI transcription of a conventional opening
structure (the style names follow common usage, for example the JT Shogi
guide). Each candidate extends a trunk by one two-move development plan for
each side, interleaved sente, gote, sente, gote. Native validation decides
legality; an engine balance screen with a fixed rule decides admission. No
professional game record, book line or commentary is copied. CC0-1.0.

Usage: python3 -m tools.strength.make_shogi_openings OUT.json
"""
import itertools
import json
import sys

# name: (trunk, sente plans, gote plans). Trunks end with sente to move.
TRUNKS = {
    'yagura': ('7g7f 8c8d 6g6f 3c3d 7i6h 7a6b 5g5f 5c5d 3i4h 4a3b',
               ['6i7h 4i5h', '6h7g 6i7h', '2g2f 2f2e', '8h7g 4i5h'],
               ['6a5b 5a4b', '6b5c 6a5b', '6c6d 6b6c', '2b3c 6a5b']),
    'bishop-exchange': ('7g7f 8c8d 2g2f 3c3d 8h2b+ 3a2b 7i8h 2b3c 8h7g 7a6b',
                        ['3i3h 2f2e', '6i7h 4i5h', '3i4h 4g4f', '9g9f 1g1f'],
                        ['4a3b 6a5b', '8d8e 4a3b', '5a4b 6a5b', '6c6d 6b6c']),
    'double-wing': ('2g2f 8c8d 2f2e 8d8e 6i7h 4a3b 9g9f 9c9d 3i3h 7a7b',
                    ['1g1f 4g4f', '3g3f 3h3g', '7g7f 4i4h', '5i5h 4i4h'],
                    ['1c1d 6a6b', '7c7d 7b7c', '3c3d 5a5b', '8b8d 5a5b']),
    'side-pawn-capture': ('7g7f 3c3d 2g2f 8c8d 2f2e 8d8e 6i7h 4a3b 2e2d 2c2d '
                          '2h2d 8e8f 8g8f 8b8f 2d3d 7c7d',
                          ['3d3f 1g1f', '5i5h 3d3f', '3d3f 3i3h', '3d3f 9g9f'],
                          ['8f8d 5a5b', '8f8d 2b3c', '8f8d 9c9d',
                           '8f8d 1c1d']),
    'fourth-file-rook': ('7g7f 3c3d 6g6f 8c8d 2h6h 8d8e 8h7g 7a6b 5i4h 5a4b',
                         ['4h3h 3h2h', '7i7h 4h3h', '9g9f 4h3h', '6i5h 4h3h'],
                         ['4b3b 9c9d', '6a5b 4b3b', '1c1d 4b3b', '5c5d 4b3b']),
    'third-file-rook': ('7g7f 3c3d 6g6f 8c8d 2h7h 8d8e 8h7g 7a6b 5i4h 5a4b',
                        ['4h3h 3h2h', '7i6h 4h3h', '9g9f 4h3h', '6i5h 4h3h'],
                        ['4b3b 9c9d', '6a5b 4b3b', '1c1d 4b3b', '5c5d 4b3b']),
    'central-rook': ('7g7f 3c3d 5g5f 8c8d 2h5h 8d8e 8h7g 7a6b 5i4h 5a4b',
                     ['4h3h 3h2h', '7i6h 4h3h', '9g9f 4h3h', '6g6f 4h3h'],
                     ['4b3b 9c9d', '6a5b 4b3b', '1c1d 4b3b', '5c5d 4b3b']),
    'gote-fourth-file': ('7g7f 3c3d 2g2f 4c4d 2f2e 2b3c 5i6h 8b4b 6h7h 5a6b',
                         ['3i4h 5g5f', '9g9f 3i4h', '3i3h 1g1f', '4i5h 3i4h'],
                         ['6b7b 7b8b', '6b7b 9c9d', '1c1d 6b7b', '7a7b 9c9d']),
    'gote-third-file': ('7g7f 3c3d 2g2f 4c4d 2f2e 2b3c 5i6h 8b3b 6h7h 5a6b',
                        ['3i4h 5g5f', '9g9f 3i4h', '3i3h 1g1f', '4i5h 3i4h'],
                        ['6b7b 7b8b', '6b7b 9c9d', '1c1d 6b7b', '4a5b 6b7b']),
    'gote-central-rook': ('7g7f 3c3d 2g2f 5c5d 2f2e 8b5b 5i6h 5a6b 6h7h 6b7b',
                          ['3i4h 9g9f', '3i3h 1g1f', '4i5h 3i4h', '5g5f 4i5h'],
                          ['7b8b 7a7b', '9c9d 7b8b', '5d5e 7b8b', '1c1d 7b8b']),
    'ishida': ('7g7f 3c3d 7f7e 8c8d 2h7h 8d8e 5i4h 5a4b 4h3h 4b3b',
               ['3h2h 3i3h', '6g6f 3h2h', '9g9f 3h2h', '7i6h 3h2h'],
               ['6a5b 7a6b', '9c9d 7a6b', '1c1d 6a5b', '7a6b 6c6d']),
    'static-rook-burrow': ('7g7f 3c3d 2g2f 4c4d 5i6h 8b4b 6h7h 5a6b 9i9h 6b7b',
                           ['8h7g 7h8h', '5g5f 3i4h', '2f2e 3i4h', '1g1f 3i4h'],
                           ['7b8b 7a7b', '9c9d 7b8b', '3a3b 7b8b', '6a5b 7b8b']),
    'double-ranging-rook': ('7g7f 3c3d 2h7h 8b3b 5i4h 5a6b 4h3h 6b7b 3h2h 7b8b',
                            ['3i3h 6g6f', '9g9f 3i3h', '1g1f 3i3h', '6i5h 3i3h'],
                            ['7a7b 4a5b', '9c9d 7a7b', '1c1d 7a7b', '4c4d 7a7b']),
    'bishop-exchange-climbing': ('7g7f 8c8d 2g2f 3c3d 8h2b+ 3a2b 7i8h 2b3c 2f2e 7a7b 3i3h 4a3b',
                                 ['4g4f 3h4g', '6i7h 8h7g', '9g9f 1g1f', '4i5h 5i6h'],
                                 ['6c6d 7b6c', '5a4b 6a5b', '9c9d 1c1d', '8d8e 7b8c']),
    'gote-quick-central': ('7g7f 5c5d 2g2f 8b5b 2f2e 5d5e 5i6h 5a6b 6h7h 6b7b',
                           ['3i4h 9g9f', '4i5h 3i4h', '5g5f 4i5h', '1g1f 3i4h'],
                           ['7b8b 7a7b', '3c3d 7b8b', '9c9d 7b8b', '7a6b 7b8b']),
    'opposing-rook': ('7g7f 3c3d 6g6f 8c8d 8h7g 8d8e 2h8h 4a3b 5i4h 5a4b',
                      ['4h3h 3h2h', '7i6h 4h3h', '9g9f 4h3h', '4h3h 1g1f'],
                      ['6a5b 7a6b', '9c9d 7a6b', '1c1d 7a6b', '5c5d 7a6b']),
    'bishop-diagonal-open': ('2g2f 8c8d 7g7f 3c3d 2f2e 8d8e 6i7h 4a3b 3i3h 7a7b',
                             ['9g9f 1g1f', '4g4f 3h4g', '5i5h 4i4h', '3g3f 3h3g'],
                             ['5a5b 6a6b', '9c9d 1c1d', '7c7d 7b7c', '5a4b 6a5b']),
    'bishop-out-static': ('7g7f 8c8d 2g2f 8d8e 8h7g 3c3d 7i8h 4a3b 6i7h 7a7b',
                          ['2f2e 5i6h', '5i6h 4i5h', '9g9f 1g1f', '3i4h 4g4f'],
                          ['9c9d 1c1d', '7c7d 7b7c', '6a5b 5a4b', '6a6b 5a5b']),
    'gote-bishop-exchange': ('7g7f 8c8d 2g2f 8d8e 8h7g 3c3d 7i8h 2b7g+ 8h7g 3a2b 3i3h 7a7b',
                             ['4g4f 3h4g', '6i7h 9g9f', '2f2e 4i5h', '1g1f 9g9f'],
                             ['2b3c 4a3b', '5a4b 6a5b', '9c9d 1c1d', '7c7d 7b7c']),
    'fourth-file-castle': ('7g7f 3c3d 6g6f 8c8d 2h6h 7a6b 5i4h 5a4b 4h3h 4b3b 3h2h 6a5b',
                           ['3i3h 1g1f', '9g9f 3i3h', '7i7h 3i3h', '6i5h 3i3h'],
                           ['8d8e 9c9d', '5c5d 1c1d', '4a4b 9c9d', '8d8e 1c1d']),
    'gote-ishida': ('7g7f 3c3d 2g2f 3d3e 2f2e 8b3b 5i6h 5a6b 6h7h 6b7b',
                    ['3i4h 4i5h', '9g9f 3i4h', '5g5f 3i4h', '1g1f 3i4h'],
                    ['7b8b 7a7b', '9c9d 7b8b', '6a5b 7b8b', '1c1d 7b8b']),
    'double-fourth-file': ('7g7f 3c3d 2h6h 8b4b 5i4h 5a6b 4h3h 6b7b 3h2h 7b8b',
                           ['3i3h 9g9f', '9g9f 1g1f', '1g1f 3i3h', '6i5h 3i3h'],
                           ['7a7b 4a5b', '9c9d 7a7b', '1c1d 7a7b', '6a5b 7a7b']),
}

SOURCE = ('Original MSH-190 USI transcription of a conventional opening structure '
          '(style names as in JT Shogi Guide08, https://www.jti.co.jp/culture/shogi/guide/08/index.html); '
          'trunk plus one independent development plan per side; no copied professional game, book line or commentary.')


def candidates():
    rows = []
    for name, (trunk, sente, gote) in TRUNKS.items():
        for i, j in itertools.product(range(len(sente)), range(len(gote))):
            s, g = sente[i].split(), gote[j].split()
            moves = trunk.split() + [s[0], g[0], s[1], g[1]]
            rows.append({'id': f'{name}-s{i + 1}g{j + 1}', 'family': name, 'sfen': None,
                         'moves': moves, 'source': SOURCE, 'license': 'CC0-1.0'})
    return rows


def main():
    rows = candidates()
    assert len({r['id'] for r in rows}) == len(rows)
    assert len({tuple(r['moves']) for r in rows}) == len(rows)
    out = {'name': 'shogi-openings-v2-candidates', 'license': 'CC0-1.0', 'openings': rows}
    with open(sys.argv[1], 'x') as f:
        json.dump(out, f, indent=1)
    print(len(rows))


if __name__ == '__main__':
    main()


def finalize(candidates_path, screen_paths, engine_sha256, out_path, limit=150, seed=190):
    """Admit candidates whose depth-9 predecessor score is within `limit`."""
    import random
    rows = json.load(open(candidates_path))['openings']
    scores = {}
    for path in screen_paths:
        for r in json.load(open(path))['positions']:
            scores[r['id']] = (r['score'], r['depth'])
    admitted = []
    for r in rows:
        score, depth = scores[r['id']]
        assert depth == 9
        if abs(score) <= limit:
            admitted.append({**r, 'screen_score': score})
    random.Random(seed).shuffle(admitted)
    out = {'name': 'shogi-openings-v2', 'license': 'CC0-1.0',
           'balance': (f'admitted when the frozen predecessor engine (sha256 {engine_sha256}) scored the final '
                       f'position within +-{limit} at fixed depth 9; rule fixed before screening; '
                       'paired colors; not a certified equal-evaluation claim'),
           'order': f'shuffled with random.Random({seed})',
           'openings': admitted}
    with open(out_path, 'x') as f:
        json.dump(out, f, indent=1)
    return len(admitted)
