# MSH-195 (Xiangqi Selectivity Round)

The selectivity step of the [per-game classical strength program](strength-roadmap.md#per-game-classical-strength-program-john-2026-11-18) for Xiangqi. **Result: the set passed its sequential test against the frozen predecessor after 80 pairs (H1), estimated +166 Elo (approx. 95% 123–214) at 5 s + 0.1 s.** Evidence: [evidence/msh195](evidence/msh195/).

## Baseline and plan (recorded before the work)

Predecessor: main `7967841f` (the MSH-194 engine). Late-game fresh-search depth at 100 ms: median 10, 10th percentile 8. Tactical suite: 373/373 at 200 ms (saturated). In the 1,120 MSH-194 self-play games, 47 were lost by the native perpetual-chase rule. Test: sequential pentanomial test H0 = 0, H1 = 15 Elo, at most 600 pairs, xiangqi-openings-v1, locally while under about an hour.

## Changes and outcomes

| Change | Outcome |
| --- | --- |
| **Reverse attack test.** A profile showed about 65% of search time in recomputing every enemy piece's moves to test one square. The test now scans outward from the square (Chariot and Cannon lines) and uses the symmetric move tables for jumping and stepping pieces. | Kept. Identical moves and node counts at fixed depth; about 2.2× nodes per second alone. Differential test against the old scan on random games. |
| **Chase-aware repetition score.** In a repetition cycle, a side whose every move checks loses; otherwise a side whose every move checks or chases loses; both or neither: draw. Chase approximation: the moved piece (not General or Soldier) attacks an unprotected opposing piece other than the General or an uncrossed Soldier, or a Horse or Cannon attacks a Chariot. Flags are computed lazily when a repetition is found. | Kept. Tests: a constructed one-sided chase cycle (from both sides to move), a protected cycle that draws, and lazy flags equal to flags computed during history replay. Native WXF rules still decide the real game. |
| **Full SEE.** Swap-list exchange evaluation; attackers are found again after each capture, so Cannon screens and Horse legs change correctly. | Kept. Hand-calculated exchanges including a Cannon that loses its screen; differential test against a reference attacker scan. |
| **Checking-move generator, quiescence check stage (8 moves), root df-pn mate search.** | Kept as part of the set. The generator contains every native checking move on random games (tested against native `is_in_check`). |
| `MARGIN_UNIT` | Unchanged: a Soldier (100) is already the smallest capture. |
| `quiet_order` | Not adopted: no specific ordering signal beyond history. |

Measurements of the full set: 1.47× the predecessor's nodes per second on a middlegame (depth 16 against 14 in 4 s); late-game fresh depth at 100 ms median 10, 10th percentile 9 (was 8). The new stages use part of the speed gain. Tactical suite: 373/373 at 200 ms; 354/373 at 5 ms (predecessor 361), where the root mate search costs time at very small budgets.

## Game test

Candidate `900f565c` against predecessor `7967841f`, both through `mshogi-match xiangqi-match`, 5 s + 0.1 s, xiangqi-openings-v1 in order, eight concurrent games on macOS arm64 (10 cores).

| Pairs | Pentanomial | LLR | Elo estimate |
| ---: | --- | ---: | ---: |
| 40 | [0, 4, 10, 16, 10] | 2.91 | +147 |
| 80 | [1, 5, 19, 32, 23] | 6.50 | +166 (123–214) |

H1 accepted at 80 pairs (bound ±2.94). No unscored game. Compute: 7.4 minutes of local wall time.

Outcomes, descriptive (160 games): the predecessor lost 28 games by the perpetual-chase rule, 46 by checkmate and 7 by stalemate; the candidate lost 1, 5 and 4. 69 games were drawn (43 natural count, 26 repetition). The chase-aware repetition score probably explains a large part of the gain, but the parts were not measured one at a time (coherent set, as the program prescribes).

## Correctness

Search, engine, runner and console release suites pass. The Xiangqi entry of the all-games fixed fingerprint changed from 313 to 324 nodes with the same move and score (root mate search and check stage). Perft and native differential tests are unchanged and pass.

## Limits

The chase rule in the search is an approximation of WXF conduct; it can misjudge cycles that native rules decide otherwise (for example exchanges, mutual chases, protected pieces that become unprotected). The game test measures the gain against this predecessor only, at one time control; it is not a rating.
