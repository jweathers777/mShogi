"""Write the game-test opening set from the 90 MSH-161 balanced master roots (one-time).

Each opening is the standard start plus the master-game prefix up to the screened
root (ply from the root FEN). These roots are exposed development material.
Run from the repository root:
    python3 tools/strength/make_openings.py tools/strength/data/openings-msh161.json
"""
import hashlib
import json
import sys
from pathlib import Path

SOURCE = Path('docs/m5/evidence/msh161/sources/main-selection.json')


def root_ply(fen):
    fields = fen.split()
    return 2 * (int(fields[5]) - 1) + (fields[1] == 'b')


def main(output):
    raw = SOURCE.read_bytes()
    roots = json.loads(raw)['selected_roots']
    openings = []
    for r in sorted(roots, key=lambda r: (r['family'], r['id'])):
        ply = root_ply(r['root_fen'])
        moves = r['source_game']['source_moves_uci'][:ply]
        if len(moves) != ply:
            raise SystemExit('short source game ' + r['id'])
        openings.append({'id': r['id'], 'family': r['family'], 'name': r.get('opening_name'),
                         'fen': None, 'moves': moves, 'root_fen': r['root_fen']})
    Path(output).write_text(json.dumps({
        'schema': 1, 'name': 'openings-msh161',
        'status': 'development only: MSH-161 balanced master roots (exposed)',
        'source': str(SOURCE), 'source_sha256': hashlib.sha256(raw).hexdigest(),
        'count': len(openings), 'openings': openings}, indent=1) + '\n')
    print(len(openings), 'openings')


if __name__ == '__main__':
    main(sys.argv[1])
