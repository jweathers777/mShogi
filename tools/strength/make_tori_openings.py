"""Write tools/strength/data/tori-openings-v1.json (MSH-204, Tori Strength Harness).

Inputs are the committed generator outputs in docs/m5/evidence/msh204/openings/
(candidates.json from `tori_openings candidates 300 8 6 50 204`, screen.json from
`tori_openings screen ... 12`, crates/mshogi-search/examples/tori_openings.rs).
Admission rule, fixed before screening: |depth-12 score| <= 100 Swallow hundredths.
Admitted rows are shuffled with random.Random(204).

Usage: python3 -m tools.strength.make_tori_openings CANDIDATES SCREEN OUT
"""
import json
import random
import sys

LIMIT = 100
SEED = 204


def write(candidates_path, screen_path, out_path):
    c = json.load(open(candidates_path))
    s = {r['id']: r for r in json.load(open(screen_path))['positions']}
    admitted, rejected = [], 0
    for r in c['openings']:
        score = s[r['id']]['score']
        if abs(score) <= LIMIT:
            admitted.append({**r, 'screen_score': score})
        else:
            rejected += 1
    random.Random(SEED).shuffle(admitted)
    out = {'name': 'tori-openings-v1', 'license': 'CC0-1.0', 'profile': 'mshogi.tori.modern.v1',
           'notation': 'native Tori notation (mshogi_core::tori::notation), from the standard root',
           'provenance': 'Project-generated (MSH-204, Tori Strength Harness); no copied game record or book line; CC0-1.0',
           'generation': c['rule'] + '; generator crates/mshogi-search/examples/tori_openings.rs',
           'balance': ('admitted when the mShogi Tori search at commit 6c7be47a (shared core, '
                       'mshogi.tori.material-placement.v1) scored the final position (side to move) within '
                       f'+-{LIMIT} Swallow hundredths at fixed depth 12; rule fixed before screening; paired colors; '
                       'not a certified equal-evaluation claim'),
           'order': f'admitted rows shuffled with random.Random({SEED})', 'rejected': {'unbalanced': rejected},
           'openings': admitted}
    with open(out_path, 'w') as f:
        json.dump(out, f, indent=1)
    return len(admitted), rejected


if __name__ == '__main__':
    print(*write(*sys.argv[1:4]))
