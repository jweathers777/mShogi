---
id: MSH-167
title: Engine Time Management
status: Done
assignee:
  - '@codex'
created_date: '2026-11-13T21:29:00Z'
updated_date: '2026-11-15T15:59:00Z'
labels:
  - handle:engine-time-management
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: medium
ordinal: 174000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Engine Time Management.

**Problem:** The engine uses a fixed time for each move. It spends the same time on a forced recapture and on a critical decision.

**Objective:** Use thinking time where it changes the result.

**Approach:** In the shared core, a soft and a hard limit per move from the clock or the configured time. Stop early when the best move is stable or forced. Extend when the best move changes or the score falls. Support a game clock with increment for game tests. Coordinate with MSH-140 (Player Thinking Time); do not change the desktop default in this task.

**Completion:** The engine plays correctly under a game clock with increment and under fixed time per move, and the time policy wins a game test at equal total time.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The engine never exceeds the hard limit in a recorded set of games under a clock with increment
- [x] #2 With equal total time, the time policy is stronger than fixed time per move in a game test
- [x] #3 Fixed time per move still works for the desktop and for diagnostics
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze fb6ac806 predecessor and inspect shared search, UCI timing and native match controller. Record baseline and comparison before changes.
2. Implement shared adaptive soft timing, forced-move exit and clock reserve; keep fixed movetime and desktop defaults supported. Add boundary and restoration tests.
3. Run native differential/perft regressions, development blunders and a bounded equal-clock paired comparison. Record actual elapsed time, deadline violations and compute, not only game scores.
4. Obtain independent review. Retain a policy only with supported benefit; stop or change direction at the declared checkpoint. Integrate verified work into main and preserve unmet criteria honestly.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Finding from MSH-163 (Engine Strength Test Harness), 2026-11-13, engine 65887fb0 on Apple M1 Pro: under a 5 s + 0.05 s Fischer clock in mshogi-match self-play, about 247 of 320 games ended in flag fall (unresolved native conclusion) near ply 60. Transcripts show the engine spends down to its last ~50 ms (go wtime 50 winc 50 -> searches 50 ms) because the UCI clock reserve is min(remaining/20, 50 ms) and hard = 3 x (remaining/30 + increment). At 1 s + 0.01 s flags fall within ~30 moves. The MSH-163 game test therefore defaults to fixed 200 ms per move; its clock mode (--base/--increment) is ready for this task's AC #1 and #2.

Owner: Codex /root, sole builder in primary main checkout at fb6ac806. No other active task owner; historical-access worktree is unrelated and preserved. Backlog 1.55.3 matches baseline. Verification Level 1 for shared scalar search policy, with focused UCI/runner integration tests for timing inputs; no wire schema or process model change planned. Full Linux/Windows and packaged GUI compatibility deferred to the next Level 3 checkpoint. Desktop defaults and MSH-140 (Player Thinking Time) remain unchanged. Integration target main supersedes the old rewrite-branch checklist wording.

Frozen predecessor: fb6ac8064a68, target/strength/engines/fb6ac8064a68/mshogi with build identity. Intended practical benefit: no recorded hard-deadline overrun in clock games and stronger play from adaptive allocation. Initial resource limit: one candidate round, baseline/candidate 146-position development suite at 1000ms (4 workers), clock safety smoke up to 8 pairs, and one strength comparison min30/max120 pairs at fast clock, 4 workers, SPRT0/15 alpha=beta=.05. Stop and record inconclusive if no H1 by cap; do not tune repeatedly. Exact strength control is pending user interpretation of equal total time (equal clock allowance or matched actual thinking time). No strength match starts before that decision. Hard response time includes delivery, not only core search; record raw overruns without excluding them. Baseline build completed before production compilation. Shared/core clock policy is independent of game rules; no rule change.

First candidate checkpoint: 8 clock self-play pairs at5s+50ms,1990 recorded responses, zero response-hard-budget overruns and no failed pairs;120.4s wall/217s CPU (exact totals in retained result). This is safety evidence, not a strength comparison. Fixed1000ms blunders fell86/146 to82/146 avoided (reference40 to36). Decision: change direction within timing implementation by preserving fixed-movetime setup/polling semantics and applying adaptive accounting only to explicitly marked clock requests; do not tune evaluation/search scores. Candidate1 patch, binary identity and results retained. Extra bounded verification for corrected mode routing: one146-position suite and up to8 clock pairs; strength test still waits for equal-time decision.

Correction to preceding rounded safety note: exact first-candidate wall120.192348666s, CPU216.718117s including .072316s cancelled work. Broad candidate1 search/console regression command passed. Corrected mode routing receives focused search/console/coordinator reruns; no rules or desktop-default changes.

User clarification2026-11-15: intended Stage4 is soft/hard timing, more time when best move changes or score falls, and less wasted time on easy moves. Interpret equal total time as equal game-clock allowance, not artificially identical elapsed thinking time. Comparison fixed before results: candidate adaptive vs frozen fb6ac806 fixed100ms per move; both5s+50ms clocks, paired exposed openings seed167,4workers,min30/max120pairs,SPRT0/15 alpha=beta=.05. Preserve actual time use, all failures and clock losses separately; native unscored outcomes stay unscored. User clarification resolves prior pending interpretation. This authorizes the planned bounded comparison, not a larger study.

Protocol checkpoint: fixed100ms predecessor comparison stopped too_many_failures: 29 scored pairs,3 failed,3 cancelled. Systematic predecessor flag falls invalidate an unconditional strength claim; do not combine these scores with another run. Independent protocol review permits fixed50ms under identical5s+50ms clocks, explicitly reporting unused initial pool/actual consumption. Candidate2 and predecessor remain frozen. New seed168, unchanged SPRT/min30/workers4; max85 attempted pairs from original120 allowance after counting completed,failed,cancelled attempts. Stop unresolved if failures recur or cap gives noH1. This change corrects protocol validity, not a response to the score.

Candidate2 measured safety:16 completed games/2058 responses,zero hard-response overruns,0 failed pairs,1 cancelled pair;121.143013584s wall,228.386561s CPU including .050499 cancelled. The failed fixed100ms comparison additionally records3854 candidate responses with zero overruns; all3 unanswered flag-fall searches belong to the predecessor. Final focused tests:70 search-library passes/1ignored,2 console unit passes,9 coordinator integration passes; clippy search+console all-targets passed; strength harness28 tests passed. Fixed-depth signature857606 nodes matches predecessor; under concurrent load one-repeat timing is not a speed claim. Candidate2 timed development suite83/146 avoided,38 reference vs86/146 and40 baseline; no improvement is claimed from it.

Final valid comparison: H1 at30pairs (LLR4.054507>2.944439),44wins5losses11draws,0failed3cancelled. Candidate3489responses0hard-budget overruns. Equal5s+50ms allowance vs frozen fixed50ms, not equal consumed compute: median unused clock1.05567s candidate vs4.97804s predecessor; total observed response time406.38341s vs175.58754s across60games. Independent final audit accepts integration within this comparison scope. Lower83vs86 development count remains disclosed. All4match runs including failed/cancelled work total614.23833s wall/1898.77314s CPU; builds,tests,bench and suites additional. Evidence docs/m5/evidence/msh167/verification.json and review.md. Per-game adapters outsideChess keep existing node limits; shared timing is reusable but this task does not change their controls.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented shared adaptive clock allocation: stable/forced choices stop early; changed choices and falling scores can use more time within the hard budget. Clock requests account for setup and delivery; fixed-time policy and desktop defaults remain unchanged. Integrated and pushed to main in 20ed7fbb56008d7f07cb8cb833a795326781fab4. Level 1 native correctness, perft/differential, console/coordinator tests, clippy and 28 harness tests passed; independent review accepted. Candidate 2 recorded zero hard-budget overruns in its 16-game safety set and both comparisons. The valid equal-clock test accepted H1 at 30 pairs: 44 wins, 5 losses, 11 draws against fixed 50 ms under 5 s + 50 ms clocks. This is equal allowance, not equal consumed compute or a general rating. The fixed 100 ms comparison failed on three predecessor flag falls and is not pooled. Development blunders were 83/146 avoided versus 86/146 baseline; this lower result is retained. Evidence: docs/m5/evidence/msh167/README.md, verification.json and review.md. Linux/Windows and packaged GUI checks were not run; broader compatibility remains at the next Level 3 checkpoint. MSH-140 (Player Thinking Time) remains separate; other-game adapter clock adoption is not delivered here. No blocker or disposable worktree remains. Next dependency-ready task: MSH-173 (Frozen Legacy Opponent).
<!-- SECTION:FINAL_SUMMARY:END -->
