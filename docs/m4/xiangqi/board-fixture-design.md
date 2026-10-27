# Independent Xiangqi board fixture design — MSH-91

This document and [`board.json`](../../../tests/fixtures/xiangqi/board.json)
freeze independently expected board facts before native/reference comparison.
The author used the selected `mshogi.xiangqi.wxf2018.v1` profile, the official
bilingual WXF2018 setup and movement clauses, and the previously reviewed MSH-80
F001/F002 seeds. No reference-engine result, native move generator, perft output
or historical implementation was used to derive the expected lists.

## Scope and encoding

The corpus contains **105 fixtures**: 51 original Red-facing cases, their 51
Black-facing counterparts, the two preserved F002 history seeds, and one
hand-enumerated board-only perft seed. **101** are eligible for the restricted
live-root reference domain. Four General-capture diagnostics are explicitly
`structural_analysis` and `reference_eligible: false`: they deliberately have
the moving side already threatening the opposing General. A live predecessor
could not legally have reached those diagrams with that side to move.

Coordinates are Red-bottom files `a`–`i`, ranks `0`–`9`, intersections rather
than squares. FEN uses `r/n/b/a/k/c/p` for Rook/Horse/Elephant/Advisor/General/
Cannon/Soldier, upper case Red, lower case Black, `w` Red to move and `b` Black.
The two unused fields are `- -`. The final `0 1` is a declared diagnostic
counter pair, not repetition history or a reachability certificate.

The source-backed Black counterparts rotate every coordinate 180 degrees and
exchange ownership, including both Generals and all blockers. This is an explicit
transcription check: Red advances toward increasing ranks, Black toward decreasing
ranks; each palace remains its owner's home palace; each Elephant stays on its
own river side. It does not assume a generic White/Black abstraction. A fixture's
reasoning should be read in its concrete owner/coordinates after this mapping.

Each fixture records provenance and reasoning. `legal_moves` is an exhaustive
whole-position expectation; `moves_from` is exhaustive for the named origin;
`must_include` and `must_exclude` assert only the listed facts. The latter must
never be presented as complete expected action sets. The differential harness
may compare complete native/reference sets separately, without replacing these
independent facts. `in_check` describes the moving General's current threat
state and does not itself assert a game outcome.

## Clause-to-case map

| Cases (and their Black counterparts) | Independently expected distinction | Governing WXF2018 clauses |
| --- | --- | --- |
| F001 | Exact 44 opening actions, all members and no others |1.4,2.1–2.7 |
| B01–03 | Open rook rays, friendly blocking, first enemy capture, board edge |2.4,2.8 |
| B04–09 | Cannon quiet rays; zero/one/two screens; either-owner mount; nearest enemy |2.6,2.8 |
| B10–13 | Eight horse moves; friendly/enemy leg blockers; boundary |2.5 |
| B14–18 | Elephant eyes of either owner, interior diagonals, river exclusion |2.3 |
| B19–21 | Advisor corners and center; palace-only diagonals |2.2 |
| B22–23 | General orthogonal palace movement and opposing-file exclusion |2.1,2.9 |
| B24–28 | Soldier before/after river, edge/far rank, no retreat; no General capture |2.7,2.8 |
| B29–31 | Sole facing-General blocker, direct General exposure, capture does not excuse exposure |2.9,2.10 |
| B32–34 | Rook pin, horse-leg discovered check, removing a second cannon screen creates check |2.4–2.6,2.10 |
| B35–38 | Complete check evasions: General escape, capture, rook interposition, horse-leg blocking |2.10A–C |
| B39–41 | Cannon check cured by removing mount, adding second screen, or capturing checker; replacing mount fails |2.6,2.10C |
| B42–43 | No legal actions without check versus with check; no result assigned here |2.1,2.4,2.5,2.9,2.10 |
| B44–49 | Captures of all six capturable native families |2.4,2.8 |
| B50 | Explicit prohibition of General capture, structural-only diagnostic |2.8 |
| F002-C/Q | Same target board and side, different preserved four-ply history seeds; same local action set |2; future history19/20 belongs to94 |
| P001 | Small independently enumerated perft/divide, no outcome pruning |2.1,2.9 |

Article2.8 expressly excludes the General from capturable material. Neither a
reference's permissive structural position parser nor pseudo-legal attack
geometry can turn a General capture into a complete legal action. These four
structural diagnostics remain native rejection obligations even though excluded
from comparator admission.

## Hand derivation of P001

Only Red General `d0`, Black General `f9`; Red to move. No action is pruned for
impossibility-to-win or other history/game conclusions. This is **board-only**
local enumeration, not WXF adjudication.

1. Red has exactly `d0d1` and `d0e0` (depth1 = **2**).
2. After `d0d1`, Black can play `f9f8` or `f9e9`.
   After `d0e0`, Black can play only `f9f8`: `f9e9` would face `e0`.
   Thus depth2 = **3**, divide `{d0d1:2,d0e0:1}`.
3. After `d0d1 f9f8`, Red has `d1d0,d1d2,d1e1` (3).
   After `d0d1 f9e9`, Red has `d1d0,d1d2` (2); `d1e1` faces `e9`.
   After `d0e0 f9f8`, Red has `e0d0,e0e1` (2); `e0f0` faces `f8`.
   Thus depth3 = **7**, divide `{d0d1:5,d0e0:2}`.

This derivation was proposed separately by the integrator and independently
checked against the two movement clauses by the fixture author. It has no
reference-derived total. Deeper opening totals, if later observed differentially,
are separately labeled observations rather than attributed to this derivation.

## Independence and review boundary

This authoring step serializes manually specified diagrams and expected actions.
The helper used during authoring only encodes supplied piece placements as FEN,
sorts already specified actions, and transcribes the explicit coordinate/owner
rotation. It contains no movement, attack, legality or perft implementation.
Its temporary development path is not a product or reference dependency.

Independent review must inspect these expected facts before differential runs.
A disagreement must preserve the original expectation and report fixture ID,
source clause, FEN, missing/extra actions and relevant check/line reasoning. Any
correction requires a rule-backed review explanation, not matching engine output.
Actual reviewer findings, execution and comparator provenance are recorded in
the separate MSH-91 evidence rather than implied by this design document.

The corpus intentionally assigns no warning, repetition, chase, claim,
impossibility or concluded-game outcome. F002's `history_seed` is one complete
four-ply cycle returning to its displayed root; it is neither an imported
pre-root history nor a pending action list for reference admission. MSH-94/105
retain their gates. No console, engine, protocol, archive or desktop work is
implemented by these fixtures.

## Reviewed expectation correction after first differential run

The first frozen corpus was SHA256
`bf945517e4ea8a3627d2914d7206ab80f1394c571a30c29f65b3db9b84f63ad9`.
The comparator reported two extra actions against its complete expected sets:
B38 `e0f0` and its Black counterpart `e9d9`. The original expected data and
mismatch report are retained separately in admission evidence.

The fixture author and independent reviewer then re-examined the diagrams under
WXF2.5/2.9/2.10. Black Horse `f2` itself interposes between a Red General on `f0`
and Black General `f9`; it does not attack `f0`. Therefore `e0f0` is a legal
fourth evasion. In the counterpart, Red Horse `d7` likewise interposes between
Black General `d9` and Red General `d0`, so `e9d9` is legal. Both original reviews
had overlooked this explicit blocker. The renewed expected lists add exactly
those actions and record the complete line reasoning; no rule or implementation
was changed to match the comparator. This is a source-backed correction of an
independent expected fact, with the original mismatch preserved.
