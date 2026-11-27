# Tori move generation in the search

How the Tori search position (`crates/mshogi-search/src/tori/position.rs`)
generates and tries moves, and why. The Tori rules
(`mshogi-core`) stay the authority; the search position is a private, faster
copy that must agree with them (perft and differential tests in
`tori/position_tests.rs`).

## Representation

- The 7×7 board is a 49-bit bitboard per side and piece kind, plus a square
  array. Hands, hash and material update incrementally on make and unmake.
- A move is 16 bits (from, to). A drop uses `from = 49 + hand kind`.
  Promotion is compulsory, so the move carries no face.
- Step and jump targets of every piece come from precomputed tables.

## Where the time went

A cost profile of real game positions (MSH-200, cloud c3-standard-4) showed
that generating moves is cheap (about 4 ns per move, 57 moves per position, 64%
of them drops). Trying moves was the cost: every make ran two full attack
tests on the Phoenixes (legality and the check flag), about two thirds of a
make. So the work went into making moves, not into generation.

## Methods

**Ray pieces (Eagle, Quails).** Each ray of each piece, side and square is a
precomputed mask. Square indices change monotonically along a ray, so the
nearest occupied square is the lowest or highest set bit of `mask & occupied`;
the targets are the mask up to and including that square. The Eagle's
back-diagonal rays of length 2 and the handed Quail rays are ordinary masks
with their own length.

**Attack test.** Step and jump attackers come from "leapers to" tables. For
ray attackers the test looks from the target square along each reverse ray:
only the nearest piece on that ray can reach the target, so one bit scan per
ray decides it.

**Legality.** A move can expose the own Phoenix only if the side is in check,
the Phoenix moves, or the moving piece leaves a square on a ray line between
the Phoenix and an enemy ray piece (a pin). Only these moves get the attack
test. A drop outside check adds a piece and moves none, so it is always legal
for this rule.

**Check after a move.** The opponent was not in check before the move. After
it, check can come only from the moved piece itself (its attacks from the
destination, with its new face), or from a ray that its departure opened. A
drop opens no ray, and a capture leaves its square occupied. So the full
attack test runs only when the departure square lies on a ray line between
the enemy Phoenix and an own ray piece.

**Swallow drop mate.** A Swallow drop must not give checkmate (T04). A
Swallow checks from an adjacent square, so the check cannot be blocked: only a
Phoenix move or a capture of the Swallow can answer it. The test tries only
those moves, instead of a full legal-move search.

All of these give exactly the same legal moves and check flags as before, so
the search output does not change.

**Checking moves (MSH-205).** The quiescence check stage and the root mate
search need every legal checking move. A piece checks directly if its target
is on a square from which its face (after promotion) attacks the enemy
Phoenix; these squares come from the reverse rays from the Phoenix. An own
piece between the Phoenix and an own ray piece can give a discovered check, so
all its moves are candidates. One Tori case needs care: a Falcon that steps
forward into the zone becomes an Eagle, whose backward ray can pass the square
the Falcon left. That move is tested with the occupancy after the move. At the
root, the moves that the repetition rule forbids are removed, as in ordinary
generation. In check, only drops between the Phoenix and a single ray checker
are searched; no other drop can be legal.

## Result

Measured on one cloud machine type for both builds (MSH-200): depth-6 node
totals were identical. make went from about 68 to 38 ns, a drop make from 70 to
31 ns, the attack test from 20 to 8 ns, and a perft leaf from 95 to 42 ns.
Depth-6 search speed on game positions rose by 22%. In a game test at
5 s + 0.1 s against the predecessor, the faster search scored about +86 Elo
(95% interval 54–120) over 223 scored pairs.

## Checks that protect it

- `random_games_agree_with_the_rules`: legal sets, check flag, hash and
  encoding at every visited position of 48 random games.
- `check_after_every_legal_move_agrees_with_the_rules`: the check flag
  after every legal move of every visited position (more than 50,000 children).
- `perft_agrees_with_the_perft` and the Swallow drop rule tests for
  both sides.

Each new path (pins, discovered checks, Swallow answers by ray captures) was
broken on purpose once, and a test failed each time.
