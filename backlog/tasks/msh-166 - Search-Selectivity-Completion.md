---
id: MSH-166
title: Search Selectivity Completion
status: To Do
assignee: []
created_date: '2026-11-13T21:29:00Z'
updated_date: '2026-11-15T15:27:00Z'
labels:
  - handle:search-selectivity-completion
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: high
ordinal: 170000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Search Selectivity Completion.

**Problem:** The Chess search has null-move pruning, late-move reductions, futility, exchange pruning and check extension. Reverse futility pruning, razoring, late-move pruning, internal iterative reduction, singular extensions and an improving condition are absent. Inspection for this task found that low-material late-move reductions are already enabled after MSH-164 (Shared Search Core); their policy needs an ablation result, not a duplicate implementation.

**Objective:** Complete the standard classical selectivity in the shared core and keep only what game tests support.

**Approach:** Add one technique at a time. Accept each one only on the game test against the predecessor. Put the techniques in the shared core with switches per game where a game needs them. Do not tune on single positions.

**Completion:** Each listed technique has a recorded accept or reject decision from a game test, and the accepted set is the default.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Each technique in the description has a recorded game-test result and an accept or reject decision
- [ ] #2 The accepted set together is stronger than the engine at the start of the task in a game test
- [x] #3 The blunder suite result is not worse than at the start of the task
- [ ] #4 Accepted techniques are in the shared core, not in Chess-only code, unless a recorded reason shows they are Chess-specific
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze main09a32b8a engine and record baseline benchmark/blunder suite; inspect actual shared core.
2. Test one classical technique at a time: reverse futility, razoring, late-move pruning, internal iterative reduction, singular extension and improving. Existing shared LMR already covers low material; test that policy by ablation.
3. For each candidate run native correctness and paired predecessor games at50ms/move,4workers,min30/max60pairs, SPRT0/15 Elo alpha=beta=.05. Keep only supported improvements; reject inconclusive screens without tuning repeated weak results.
4. Test accepted combination against frozen task start, require no worse development blunders, independent review and relevant native consumer checks. Integrate only evidence-supported defaults; if strength gate is unmet record unfinished criteria honestly.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex /root/selectivity_builder, sole code builder; primary integration main checkout at09a32b8a. Verification Level1: platform-independent shared search selectivity; native game rules and product protocol unchanged. Freeze baseline before editing. Intended benefit is stronger play via useful search depth, not lower node counts alone. Initial resource cap: seven60-pair screens at50ms/move with4workers and native runner failure/ply bounds; final accepted-combination screen up to120pairs, actual CPU/wall recorded. Do not escalate or repeat weak screens without a new decision checkpoint. Full platform/package campaign deferred to Level3. Current inspection shows low-material LMR is already enabled in shared core; the task description predates MSH-164.

Screening clarification: run each optional technique independently against the frozen task-start predecessor, then test any supported combination separately. This keeps each screen attributable to one change and does not assume additive gains. The temporary orchestration initially checked an incorrect acceptance string (accept_h1 vs the actual H1); no result has been accepted from that check. Corrected the reproducible driver and will derive decisions only from each authoritative result.json, preserving the independent screened binaries/patches. No valid game evidence will be rerun solely for this bookkeeping correction.

Bounded round complete on main; no active builder remains. Seven independent 60-pair screens (840 scored games, no failed pairs) all reached max_pairs without a sequential decision. Reject all six additions as unproven; existing low-material LMR ablation is also inconclusive. Production search files are restored exactly to 09a32b8aeff3c30fff8f5ca47e16920b24feb1c4, so the baseline blunder result remains unchanged (86/146 avoided, 40 reference matches at 1000 ms). Match compute: 1270.4 s wall and 4862.2 s CPU including 66.7 s cancelled work; builds and checks are additional. Evidence: docs/m5/evidence/msh166/README.md and verification.json, exact candidate patches and compressed games. AC2 remains unmet: no supported strength improvement. Next action is a decision on a larger predeclared trial or a different coherent approach; the README proposes but does not run a held-out IIR trial. Do not treat its highest point estimate as proven strength. Prototype local tests passed 70 with one ignored; final production has no code changes. Independent final evidence review pending. Integration target is main; the old rewrite-target wording in the checklist predates MSH-171 (Rewrite Main Promotion).

Independent final review approved by /root/two_human_final_review: all seven patch/binary bindings, paired game scores and colours, sequential decisions, final ablation direction, and compute totals checked; restored production source confirmed identical to task start. No open review findings. This resolves the preceding pending-review note. The committed result is evidence and a resumable unfinished task, not a strength improvement.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Bounded round on main: tested six additions separately and the existing low-material LMR policy by ablation. All seven results were inconclusive at 60 pairs; no addition was retained. Production source is unchanged from 09a32b8aeff3c30fff8f5ca47e16920b24feb1c4. Evidence and reproduction are in docs/m5/evidence/msh166/README.md and verification.json. Prototype tests passed 70 with one ignored; independent evidence review approved with no open findings. AC1 and unchanged-source AC3 are supported; AC2 remains unmet. Task is To Do and unassigned. Next action: decide on the proposed larger held-out trial or another coherent approach; no further campaign is running.
<!-- SECTION:FINAL_SUMMARY:END -->
