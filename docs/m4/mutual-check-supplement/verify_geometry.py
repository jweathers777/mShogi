"""Cross-check two independent conditional geometry enumerations."""
import itertools
import json

import directed_geometry as directed
import independent_geometry as independent


def verify():
    all_directed = directed.enumerate_pairs()
    ours = {pair for pair in all_directed if pair[0][0] >= 0}
    theirs = set()
    for row in independent.enumerate_candidates():
        for (ra, ea_list), (rb, eb_list) in itertools.product(
            row['own_edges'], row['enemy_edges']
        ):
            for ea, eb in itertools.product(ea_list, eb_list):
                if independent.fits_board([(0, 0), row['L'], *ea, *eb]):
                    theirs.add((row['L'], (ra, tuple(ea)), (rb, tuple(eb))))
    if ours != theirs:
        raise AssertionError({'directed_only': sorted(ours - theirs),
                              'independent_only': sorted(theirs - ours)})
    expected = {
        ((1, -3), 1, 1), ((1, -2), 1, 1), ((1, -2), 1, 2),
        ((1, -2), 2, 1), ((1, -2), 2, 2), ((2, -1), 1, 1),
        ((3, -2), 2, 2), ((1, 2), 1, 1), ((2, 1), 1, 1),
    }
    for t in range(2, 9):
        expected.add(((t, t), t, t))
        expected.add(((t, -t), t, t))
    assert {(l, a[0], b[0]) for l, a, b in ours} == expected
    assert len(ours) == 23
    assert len({pair[0] for pair in ours}) == 20
    assert len(all_directed) == 46
    assert all(len(a[1]) == len(b[1]) == 2 for _, a, b in ours)

    # Exchange sides and rotate 180 degrees around the midpoint of the kings.
    transform = lambda p: (1 - p[0], -2 - p[1])
    s = ((0, -1), (1, 0))
    t = ((-1, -2), (1, 0))
    assert set(map(transform, t)) == {(2, 0), (0, -2)}
    assert set(map(transform, s)) == {(0, -2), (1, -1)}

    return {
        'agrees': True,
        'normalized_king_offsets': 20,
        'normalized_pairings': 23,
        'full_reflection_pairings': 46,
        'cycle_sizes': [[2, 2]],
        'mixed_case_symmetry': 'side exchange plus 180-degree rotation verified',
        'scope': 'Necessary geometry after prior proof reductions; not a full game-state search.',
        'pairings': sorted(ours),
    }


if __name__ == '__main__':
    result = verify()
    print(json.dumps({key: value for key, value in result.items()
                      if key != 'pairings'}, indent=2))
