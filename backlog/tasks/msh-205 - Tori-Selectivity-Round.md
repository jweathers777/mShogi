---
id: MSH-205
title: Tori Selectivity Round
status: Done
assignee:
  - '@claude'
created_date: '2026-11-19T19:43:00Z'
updated_date: '2026-11-21T04:40:00Z'
labels:
  - handle:tori-selectivity-round
  - tori
  - strength
milestone: m-5
dependencies:
  - MSH-204
  - MSH-200
references:
  - docs/m5/strength-roadmap.md
priority: high
ordinal: 212000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Selectivity Round

**Problem.** Tori uses the shared core with default hooks; speed, selectivity and drop handling have not been tuned as for Shogi (MSH-190) and Xiangqi (MSH-195, MSH-203).

**Objective.** The selectivity step of the program for Tori, including search speed where the harness shows it pays.

**Approach.** Coherent set of shared-core techniques adapted to Tori (quiescence checks, SEE with drops, scaled margins, mate search, ordering) plus clear speed fixes from profiling; informed by MSH-200 (Tori Move Generation Deep Dive) where John has decided.

**Completion.** Each technique has a recorded outcome; kept changes pass correctness checks and the sequential predecessor test (H1 about 15).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Each adopted or rejected technique and speed change has a recorded outcome.
- [x] #2 Perft, native differential and history tests pass; tactical suite recorded.
- [x] #3 The set passes the sequential predecessor test or the null result is reported and reverted; independent review recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Baseline: main f8ee432 (MSH-200 engine), frozen as predecessor. Its binaries and worker image msh200-40edac60 are in the cloud.
Set (ported from the Shogi round MSH-190; Tori has the same drop-game structure):
1. MARGIN_UNIT 200: the smallest capture moves a Swallow (100) from board to hand.
2. Full SEE with board and hand values and compulsory promotion (Falcon->Eagle, Swallow->Goose), replaces the one-step exchange estimate.
3. Complete checking-move generator (direct checks of each face, discovered checks by ray pieces, drops), tested against the native rules.
4. Quiescence check stage (8) and root df-pn mate search.
5. Quiet ordering: direct checks first, drops near the enemy Phoenix.
6. In check, only drops that block a single ray check are searched.
Benefit: fewer tactical misses (Swallow and drop mates), better pruning scale.
Comparison: sequential pentanomial SPRT H0 0, H1 15, alpha=beta=0.05, max 600 pairs, at 5 s + 0.1 s on c3-standard-4 (tori_cloud.py --match); tactical suite at 20 ms.
Caps: US$10. Checkpoint: if the set is weak, test parts alone only with a specific reason.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-20 (John): MSH-205 depends on MSH-200 (Tori Move Generation Deep Dive): move generation, especially drops, limits search speed, and selectivity results measured on the current generator could need repeating after an MSH-200 change.

Checkpoint 2026-11-21 (handoff): set committed 643f645 on branch claude/msh-205-tori-selectivity; Tori tests pass (17), new tests: check generator and in-check search generator against native (48 games), Falcon->Eagle promotion check, SEE values, df-pn drop mate and Swallow rule; mutation of discovered and promotion check paths fails the tests. Predecessor = MSH-200 build 40edac60 (gs://mshogi-experiments-216675633801/build). Candidate build store gs://mshogi-experiments-216675633801/build-643f6451. Next: derived worker image, tori_cloud --match and --suite, collect.

Checkpoint 2026-11-21: candidate build 643f645 (store gs://mshogi-experiments-216675633801/build-643f6451). Worker image worker@sha256:b3e2d72d (tag msh205-643f6451; base = msh200 Tori worker, frozen data digest af2d54e8 verified; candidate in /opt/engines, MSH-200 40edac60 in /opt/engines/baseline). Campaign msh205-match-1 submitted (c3-standard-4, 8 VMs). tori_cloud now accepts any msh<number> campaign name (70ea533). Next: collect match, then suite campaign msh205-suite-1 with its own store prefix.

2026-11-21 campaign msh205-match-1 (643f645 vs 40edac60, c3-standard-4, <=2.42 VM-h, <=US$0.49, VMs verified deleted; build <=0.63 VM-h, <=US$0.13): no result. 113 of 448 games stopped with the candidate's own ContextMismatch failure. Cause: the Tori engine reply check (mshogi-engine/src/tori.rs matches) required a Complete reply to reach the requested depth; the new root df-pn mate search returns a proven mate as Complete at depth 7-9. Replaying 10 failed requests locally gave Complete with mate scores at depth 7-9 in each. Shogi and Xiangqi have no such rule. Fix: Complete below the requested depth is accepted only with a proven-win score (>= MATE_BOUND). Regression test tori_protocol a_root_mate_proof_is_a_complete_reply_below_the_requested_depth uses a failed game position with a node limit (deterministic); it fails without the fix. Next: rebuild, rerun as msh205-match-2.

Checkpoint: fix c6874b9 built (store build-c6874b9d, <=0.62 VM-h, <=US$0.12), image worker@sha256:2a72b075 (tag msh205-c6874b9d), campaign msh205-match-2 submitted.

Results (cloud c3-standard-4, same machine type for both builds):
- Game test msh205-match-2: c6874b9 vs 40edac60 (MSH-200), 5 s + 0.1 s, 224 pairs (1 length-limited, unscored), 447 native conclusions, no failures. Pentanomial [23,0,102,0,98], +122 Elo (89-157). Sequential SPRT H0 0 / H1 15: H1 at pair 35; fixed-sample LLR 7.06. Late-game depth from ply 100: median 10, p10 8. <=2.42 VM-h, <=US$0.49.
- Tactical suite 20 ms (msh205-suite-1): 359 -> 477 of 502; held-out 92 -> 121 of 124. <=1.48 VM-h, <=US$0.30.
- Fixed depth 6 (same VM): nodes games 2,031,634 -> 3,260,449, suite 1,886,835 -> 3,140,749, openings 753,052 -> 918,625 (mate search and quiescence checks); nodes/s about 14% lower on game positions.
- Task compute: builds <=US$0.25, failed campaign msh205-match-1 <=US$0.49, match-2 <=US$0.49, suite <=US$0.30: total <=US$1.53. All VMs verified deleted. Run data: gs://mshogi-experiments-216675633801/runs/msh-205/.
Technique outcomes (tested as one coherent set, per the roadmap; the set result is strong, so no part was tested alone): MARGIN_UNIT 200 kept; full SEE kept; complete check generator kept; quiescence check stage (8) kept; root df-pn mate search kept; quiet ordering (checks, drops near the Phoenix) kept; in-check drops only on interposition squares kept (exact, no change to legal moves).
Independent review (subagent, diff + task): 1 blocking finding fixed in 05841f2 (generate_checks did not remove root moves forbidden by the repetition rule, so the root mate search could return a forbidden move; new test fails without the fix). The fix changes only such roots; the game test had no engine failure, so it was not repeated. Should-fix: the mate exception in the reply check is limited to the shared-core identity (done); a native mate check of the reported line is not possible because the reported line is cut at 8 plies and mates are up to 64. Nits fixed (test doc comment, stale tool comment).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Tori selectivity round: the Shogi selectivity set ported to the Tori search position, tested as one set against the MSH-200 engine.
Changes: margins scaled to the Swallow (MARGIN_UNIT 200); full SEE with board and hand values and compulsory promotion; a complete checking-move generator (direct, discovered, a Falcon that promotes and checks through its start square, drops), tested against the native rules; quiescence check stage (8); root df-pn mate search; quiet ordering for checks and drops near the Phoenix; in check, drops only between the Phoenix and a single ray checker. The engine reply check now accepts a proven mate that completes below the requested depth (shared-core search only); without this the engine refused its own replies in 113 of 448 test games of the first campaign.
Results (cloud c3-standard-4): game test at 5 s + 0.1 s, +122 Elo (89-157), sequential SPRT H1 at pair 35 of 224, 447 native conclusions, no failures; tactical suite at 20 ms 359 -> 477 of 502 (held-out 92 -> 121 of 124). Compute <=US$1.53, VMs deleted; run data gs://mshogi-experiments-216675633801/runs/msh-205/.
Checks: fmt, clippy, full workspace tests (release) pass; CI passed on every branch commit; perft, native differential, repetition and new targeted tests pass; each new check path and the two fixes were broken on purpose once and a test failed. Independent review: 1 blocking finding (root mate search could return a move the repetition rule forbids) fixed with a test.
Follow-up: MSH-219 (Shogi Match Test Timing Failure). Next: MSH-206 (Tori Evaluation Round).
<!-- SECTION:FINAL_SUMMARY:END -->
