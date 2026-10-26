"""Necessary geometry, not a shogi state search or standalone impossibility proof.

Assumptions from the preceding proof lemmas:
* kings are fixed, and exactly one persistent nongazing bishop per side moves;
* neither bishop can supply a direct check or be captured;
* every discovery bishop landing square lies on another own piece's checking ray;
* hence those rays are orthogonal, because both bishops are already allocated;
* a persistent bishop move switches rays, so it connects adjacent axis corners
  of one fixed radius; horse orthogonal steps cannot switch these rays;
* the landing square must capture a checker or interpose a check against its king.

We deliberately overapproximate non-bishop attacking types. This may retain
impossible candidates but cannot remove a real one. Board placement is tested
by bounding-box span, allowing all translations into a 9 by 9 board.
"""

from itertools import combinations, product
import json

K = (0, 0)


def nonbishop_threat_or_interposition(point, king, forward):
    dx, dy = king[0] - point[0], king[1] - point[1]
    if dx == dy == 0:
        return False
    # Rook line, including any possible interposition. Also covers lance and
    # adjacent orthogonal attacks without needing their direction restrictions.
    if dx == 0 or dy == 0:
        return True
    # Adjacent diagonal: dragon or suitable minor piece; capture only.
    if abs(dx) == 1 and abs(dy) == 1:
        return True
    # Knight check: capture only; the direction is essential.
    return abs(dx) == 1 and dy == 2 * forward


def fits_board(points):
    return all(max(p[i] for p in points) - min(p[i] for p in points) <= 8
               for i in (0, 1))


def strictly_between_on_diagonal(point, a, b):
    if abs(b[0] - a[0]) != abs(b[1] - a[1]):
        return False
    return (min(a[0], b[0]) < point[0] < max(a[0], b[0])
            and min(a[1], b[1]) < point[1] < max(a[1], b[1])
            and (point[0] - a[0]) * (b[1] - a[1])
            == (point[1] - a[1]) * (b[0] - a[0]))


def bishop_edges(enemy_king, own_king, checking_enemy_forward):
    result = []
    kx, ky = enemy_king
    lx, ly = own_king
    for radius in range(1, 9):
        corners = [(kx + radius, ky), (kx, ky + radius),
                   (kx - radius, ky), (kx, ky - radius)]
        valid = [p for p in corners
                 if p != own_king
                 and (p[0] - kx) * (ly - ky) != (p[1] - ky) * (lx - kx)
                 and nonbishop_threat_or_interposition(p, own_king,
                                                       checking_enemy_forward)
                 and fits_board([enemy_king, own_king, p])]
        edges = []
        for a, b in combinations(valid, 2):
            if (abs(a[0] - b[0]) == abs(a[1] - b[1])
                    and fits_board([enemy_king, own_king, a, b])
                    and not strictly_between_on_diagonal(own_king, a, b)):
                edges.append(tuple(sorted((a, b))))
        if edges:
            result.append((radius, edges))
    return result


def enumerate_candidates():
    result = []
    # Horizontal reflection preserves forward directions, so x >= 0 suffices.
    # Every two points on a 9 by 9 board differ by at most 8 on either axis.
    for x, y in product(range(9), range(-8, 9)):
        if max(abs(x), abs(y)) <= 1:
            continue  # Coincident or adjacent kings cannot be live roots.
        L = (x, y)
        own = bishop_edges(K, L, -1)
        enemy = bishop_edges(L, K, 1)
        if own:
            result.append({"L": L, "own_edges": own, "enemy_edges": enemy})
    return result


if __name__ == "__main__":
    for candidate in enumerate_candidates():
        print(json.dumps(candidate))
