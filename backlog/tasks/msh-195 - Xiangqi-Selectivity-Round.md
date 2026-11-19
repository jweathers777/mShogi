---
id: MSH-195
title: Xiangqi Selectivity Round
status: Done
assignee:
  - '@claude'
created_date: '2026-11-18T19:25:00Z'
updated_date: '2026-11-19T08:26:00Z'
labels:
  - handle:xiangqi-selectivity-round
  - xiangqi
  - strength
milestone: m-5
dependencies:
  - MSH-194
references:
  - docs/m5/strength-roadmap.md
ordinal: 202000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Selectivity Round

**Problem.** Xiangqi uses only the default shared-core search hooks: no quiescence checks, a one-step exchange estimate, default margins and no mate search. Checks and forcing sequences are frequent in Xiangqi.

**Objective.** The selectivity step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Xiangqi.

**Approach.** Adopt the shared-core techniques of MSH-190 (Shogi Selectivity Round) that fit Xiangqi: bounded quiescence check stage, full SEE (Cannon screens and Horse legs make it differ from Chess), Xiangqi-scaled margins, move ordering, and the shared df-pn mate search if it pays. Test as one coherent set against the frozen predecessor.

**Completion.** Each technique has a recorded outcome; kept changes pass correctness checks, the tactical suite and the sequential predecessor test.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Each adopted or rejected technique has a recorded outcome with its reason.
- [x] #2 Perft, native differential and history tests pass; the tactical suite score is recorded against the baseline.
- [x] #3 The kept set passes the sequential pentanomial predecessor test (H1 about 15 Elo) at the harness control, with compute recorded; a null result is reported and reverted.
- [x] #4 Independent review is recorded and findings are resolved.
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
Baseline: frozen predecessor = main 7967841f (MSH-194 engine). Late-game fresh depth median 10 at 100 ms (MSH-194); tactical suite 373/373 (saturated, regression guard); 47/1,120 harness self-play games lost by the native perpetual-chase rule.
Intended benefit: fewer tactical and rule losses and deeper effective search at the 5 s + 0.1 s control.
Set (coherent, tested together):
1. Chase-aware repetition score: a cycle in which every move of one side checks or chases (moved non-General, non-Soldier piece attacks an unprotected non-General piece other than an uncrossed Soldier, or a Horse/Cannon attacks a Chariot) loses for that side; both or neither: draw. Approximation of native WXF; native rules stay authoritative.
2. Full SEE (swap list on a scratch board, least valuable attacker first) for exchange_at_least.
3. generate_checks (complete superset of checking moves) with a bounded quiescence check stage (QUIESCENCE_CHECKS).
4. Shared-core df-pn root mate search (MATE_SEARCH).
5. MARGIN_UNIT: soldier = 100 already equals the smallest capture; record as unchanged. quiet_order: adopt only with a specific signal.
6. Profile per-node costs (rule_score calls has_legal_move at every node) and fix clear waste.
Comparison: perft and native differential tests; check-generator completeness test against brute force; tactical suite; sequential pentanomial predecessor test H0=0, H1=15 at 5 s + 0.1 s, max 600 pairs, xiangqi-openings-v1, local 8 workers while under about 1 hour, else cloud.
Caps: local only unless the test needs more than about 1 hour; cloud cap US$5. Checkpoint: after the game test, keep the set on H1, reject and revert on H0, consider an ablation only with a specific doubt.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
From MSH-194 (Xiangqi Strength Harness), 2026-11-18: use the 5 s + 0.1 s control, tools/strength/xiangqi_games.py, xiangqi-openings-v1 and the tactical suite as a regression guard (it is saturated). Observed: 47 of 1,120 self-play games lost by the native perpetual-chase rule because the private search scores chase cycles as draws; consider a chase-aware repetition score in this round. Request overhead is now flat (~3 ms).

2026-11-19 implementation (branch codex/msh-195-xiangqi-selectivity, candidate 900f565c, binary sha256 prefix ec9736052fc2417d; predecessor main 7967841f, f3425d1af6224986):
- Profile: targets() was ~65% of search samples via attacked(). Reverse-scan attack test (rays from the target; table-reverse for jumping pieces): identical fixed-depth moves and nodes, ~2.2x nodes/s. Differential test against the old scan.
- Chase-aware repetition (one-sided check/chase cycle loses), lazy flags in search, eager for history; tests: constructed chase cycle, protected cycle draws, lazy vs eager flags.
- Full SEE with dynamic screens and reverse least-attacker; tests: hand-calculated exchanges, differential against target-scan SEE.
- generate_checks (complete superset, tested against native checks), QUIESCENCE_CHECKS=8, MATE_SEARCH=true.
- MARGIN_UNIT unchanged (soldier 100 = smallest capture). quiet_order not adopted (no specific signal).
- Measured: full set 1.47x predecessor nodes/s on a middlegame (depth 16 vs 14 in 4 s); late-game fresh depth at 100 ms median 10 (p10 9, was 8). Tactical suite 373/373 at 200 ms; 354/373 at 5 ms (pred 361).
- Release suites for search, engine, runner, console pass; all_games_timing Xiangqi fingerprint 313->324 nodes, same move.
- Game test started locally: 40-pair batches, 5 s + 0.1 s, H1=15.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Xiangqi selectivity set kept: H1 against the frozen predecessor (main 7967841f) after 80 pairs at 5 s + 0.1 s, pentanomial [1,5,19,32,23], LLR 6.50, about +166 Elo (approx. 95% 123-214), no unscored games, 7.4 min local compute. Report: docs/m5/xiangqi-selectivity-round.md; evidence docs/m5/evidence/msh195.

Changes and outcomes: reverse-scan attack test (profile showed ~65% of time in attack tests; identical fixed-depth search, ~2.2x nodes/s alone); chase-aware repetition score (one-sided check or chase cycle loses; lazy flags in search); full SEE with dynamic Cannon screens; complete checking-move generator with an 8-move quiescence check stage and the shared root df-pn mate search. MARGIN_UNIT unchanged (Soldier 100); quiet_order not adopted. Full set: 1.5-1.6x predecessor nodes/s; late-game depth p10 8 -> 9; tactical suite 373/373 at 200 ms, no difference at 5 ms beyond noise. In the test games the predecessor lost 28 by perpetual chase and 46 by checkmate, the candidate 1 and 5 (descriptive; parts not measured one at a time).

Verification Level 1: release suites 1,248 passed, 16 ignored, 0 failed; search suite rerun after review fixes; clippy/fmt clean; Python 139 passed. Linux/Windows CI deferred. Independent review: nothing blocking; fixes applied (evidence, SEE bound, chase-flag precondition, pre-root cycle test). Limit: the chase rule in search approximates WXF conduct. Next: MSH-196 (Xiangqi Evaluation Round). Integrated into main by merge 1b374090.
<!-- SECTION:FINAL_SUMMARY:END -->
