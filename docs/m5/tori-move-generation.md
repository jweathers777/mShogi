# MSH-200 (Tori Move Generation Deep Dive): discussion paper

A discussion task with John (2026-11-19). It must be decided before MSH-205 (Tori Selectivity Round), which depends on it (John, 2026-11-20). This paper lists the Tori pieces and drops with their native rules, measures what the search spends on them, and proposes options. Native rules stay the authority. Any faster search-side code is accepted only with perft and native differential tests. Evidence: evidence/msh200.

## How the search generates moves today

`crates/mshogi-search/src/tori/position.rs`, a private copy of the native position:

- The 7×7 board is stored in 49-bit bitboards per side and piece kind, plus a square array. Hands, hash and material are updated incrementally.
- Step and jump targets come from precomputed tables. Rays (Eagle, Quails) are walked one square at a time.
- One move is 16 bits (from, to). A drop uses `from = 49 + hand kind`. Promotion is compulsory, so the move carries no face.
- **Legality:** every move is made, then the own Phoenix is tested with `attacked()`. A second `attacked()` call sets the check flag. A checking Swallow drop also runs a full legal-move search for the drop-mate rule.
- `attacked()` tests every piece kind's step/jump table from the target square, then walks every enemy ray piece's full rays.

## Pieces and their measured cost

Measured on c3-standard-4 (cloud), 1,577 real game positions (every 4th ply from ply 20 of 32 self-play games at 5 s + 0.1 s), 200 repeats. Times are per piece of the side to move, for computing all its targets.

| Piece | Native rule (T02, mover's view) | Current method | Time per piece |
| --- | --- | --- | --- |
| Phoenix | 8 adjacent steps (royal) | table | 4.2 ns |
| Falcon | 7 steps (not straight back); promotes to Eagle by zone | table | 3.4 ns |
| Eagle | steps forward/sideways; rays diagonal-forward and straight back; **limited** rays diagonal-back of length 1–2 | table + ray walk | 17.2 ns |
| Crane | 6 steps (not sideways) | table | 3.0 ns |
| Pheasant | 2 back-diagonal steps; jump 2 forward | table | 3.0 ns |
| Left Quail | back-left step; rays forward and back-right (handed) | table + ray walk | 10.0 ns |
| Right Quail | back-right step; rays forward and back-left (handed) | table + ray walk | 9.7 ns |
| Swallow | 1 step forward; promotes to Goose by zone | table | 2.7 ns |
| Goose | jumps (±2, 2) and (0, −2) | table | 3.0 ns |
| **Drops** | 6 hand kinds on empty squares; Swallow: not on last rank, at most 2 own per file, no direct drop-mate | empty-square mask per kind | 19.8 ns per position for 36 drops |

**Per position** (real games): 57 candidate moves, of which **36 (64%) are drops**: 22.5 ordinary drops and 13.8 Swallow drops. Generating all of them costs **234 ns** (4 ns per move). The ray pieces are the most expensive per piece, but there are few of them, so their share is small.

## Where the time goes

| Operation | Time | Note |
| --- | --- | --- |
| Generate all moves of a position | 234 ns | about 4 ns per move |
| `attacked(Phoenix)` | 20 ns | called twice in every make |
| make + unmake, ordinary move | 60 ns | 12% of tried moves illegal |
| make + unmake, capture | 71 ns | 19% illegal |
| make + unmake, drop | 59 ns | 15% illegal: the side is in check and the drop does not block |
| make + unmake, Swallow drop | 66 ns | includes the drop-mate test when it gives check |
| perft, per leaf node | 94 ns | generation + make/unmake |
| search, depth 6 | 1.22 M nodes/s | 0.82 µs per node, all search work included |

**Finding.** Generation is cheap; trying moves is not. About two thirds of each make/unmake is the two `attacked()` calls (2 × 20 ns of about 60 ns). Drops are most of the moves, and for them both calls are mostly unnecessary:

- Outside check, a drop never exposes the own Phoenix (it adds a piece; it moves none).
- A drop gives check only by the dropped piece's own attack. A drop cannot give a discovered check.

The search does not try every child at every node (cutoffs). So the gain in search speed will be smaller than the gain in make, and only a measurement can show it. MSH-204 measured that doubling the time is worth about +330 Elo at the game-test control, so a 20–30% speed-up would be worth a lot.

## Options

| | Option | Change | Expected benefit | Risk / cost |
| --- | --- | --- | --- | --- |
| A | **Drop legality and check without `attacked()`** | Outside check, a drop is legal with no test. Check after a drop = the dropped piece's targets contain the enemy Phoenix (one table lookup or one ray). In check: generate only drops on squares between the checker and the Phoenix; a contact or jump check cannot be blocked. | make of 64% of moves from about 60 to about 25 ns; much smaller evasion lists | Low. Differential test against native legality and check on random and game positions |
| B | **Danger-set filter for moves** (as Xiangqi MSH-203) | Run the full legality test only for Phoenix moves, moves while in check, and moves of a piece on a line from the own Phoenix to an enemy ray piece (pins). Run the check test only for moves that can give check (direct target or uncovering a ray). | make of ordinary moves from about 60 to about 35 ns | Medium: pin and discovered-check lines of Eagle and Quail rays (handed, limited). Same differential test |
| C | **Cheap Swallow drop-mate test** | An adjacent Swallow check cannot be blocked: the only defences are Phoenix moves and capturing the Swallow (legal captures). Test only those, not a full legal-move search. | removes the rare but costly full search after checking Swallow drops | Low. Rule T04(3); native differential on all Swallow-drop checks |
| D | **Ray tables for Eagle and Quails** | Per-direction ray masks with a first-blocker lookup (bit scan), including Eagle's limited back-diagonal (length 2). Also reverse rays from the target in `attacked()`. | ray pieces 10–18 to about 4 ns; `attacked()` 20 to about 10 ns | Low. Table test against the ray walk |
| E | **Drop ordering and staging** (search side) | Generate drops in a separate late stage; order checks and drops near the enemy Phoenix first; reduce late drops more | fewer drops made at cut nodes | Belongs to MSH-205 (selectivity); listed here because drops dominate |

Options A–D change only how the search position computes the same legal moves and check flags. They change no rule and no search decision rule, so the search output can stay identical at fixed depth (a strong check). Option E changes which moves the search tries first. It is a selectivity change, measured by game test in MSH-205.

## Recommendation

1. Implement **A, B, C and D** together as one speed set in this task.
   - Validate with: Tori perft and differential tests against native legality and check flags (random play, game positions, suite and fixtures); identical fixed-depth search output; nodes per second and time to depth on the MSH-204 positions; and the game test against the predecessor at 5 s + 0.1 s in the cloud.
   - Expected result: a 20–40% faster search. This is an estimate to be measured; I commit to no number before measuring.
2. Leave **E** to MSH-205.
3. Keep the move encoding (16 bits) and the board representation (49-bit bitboards). Neither shows up as a cost.

## Questions for John

1. Do you agree with A–D as the move-generation set, with E moved to MSH-205?
2. Is there a Tori piece you want treated differently from the table above (for example the handed Quails or the Eagle's limited back rays)?
3. Should the set be accepted on speed with identical fixed-depth output alone, or also on the game test against the predecessor? I propose both.
