---
id: MSH-166
title: Search Selectivity Completion
status: Ready
assignee: []
created_date: '2026-11-13T21:29:00Z'
updated_date: '2026-11-15T16:29:00Z'
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

**Problem:** The shared core lacks reverse futility pruning, razoring, late-move pruning, internal iterative reduction, singular extensions and an improving condition. A first round (`docs/m5/evidence/msh166/`, `fb6ac806`) tested each technique alone with tests of at most 60 pairs. The 95% intervals were about ±45 Elo, so no test could decide; this is not a rejection. Five of the six additions had positive estimates, and four reduced the time to a fixed depth.

**Objective:** Complete the standard classical selectivity in the shared core, and make the engine stronger.

**Approach:** Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md`. Add the techniques together, with standard parameters from established engines. "Improving" is a modifier of the other techniques, not a technique alone. Singular extensions may wait for a later task if the chosen time control rarely reaches their depth. Test the set in one decisive game test against frozen `main`. Remove a part only for a recorded reason: a weak set result, a cost, or clear harm in Tori. The first-round patches are a usable start. Their `has_legal_move()` calls in the pruning paths add per-node cost to protect an approximation; remove them. Low-material late-move reductions stay as they are. Do not tune on single positions.

**Completion:** The accepted set is the default, it wins the game test, and the blunder suite is not worse.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The combined set has a game-test result with a sequential decision against the engine at the start of the task, and each technique left out has a recorded reason
- [ ] #2 The accepted set is stronger than the engine at the start of the task in that game test
- [ ] #3 The blunder suite result of the accepted set is not worse than at the start of the task
- [ ] #4 Accepted techniques are in the shared core, not in Chess-only code, unless a recorded reason shows they are Chess-specific, and the Tori tests pass
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Replaced 2026-11-15: the approach changed to the combined set first. The next worker writes the current plan here.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex /root/selectivity_builder, sole code builder; primary integration main checkout at09a32b8a. Verification Level1: platform-independent shared search selectivity; native game rules and product protocol unchanged. Freeze baseline before editing. Intended benefit is stronger play via useful search depth, not lower node counts alone. Initial resource cap: seven60-pair screens at50ms/move with4workers and native runner failure/ply bounds; final accepted-combination screen up to120pairs, actual CPU/wall recorded. Do not escalate or repeat weak screens without a new decision checkpoint. Full platform/package campaign deferred to Level3. Current inspection shows low-material LMR is already enabled in shared core; the task description predates MSH-164.

Screening clarification: run each optional technique independently against the frozen task-start predecessor, then test any supported combination separately. This keeps each screen attributable to one change and does not assume additive gains. The temporary orchestration initially checked an incorrect acceptance string (accept_h1 vs the actual H1); no result has been accepted from that check. Corrected the reproducible driver and will derive decisions only from each authoritative result.json, preserving the independent screened binaries/patches. No valid game evidence will be rerun solely for this bookkeeping correction.

Bounded round complete on main; no active builder remains. Seven independent 60-pair screens (840 scored games, no failed pairs) all reached max_pairs without a sequential decision. Reject all six additions as unproven; existing low-material LMR ablation is also inconclusive. Production search files are restored exactly to 09a32b8aeff3c30fff8f5ca47e16920b24feb1c4, so the baseline blunder result remains unchanged (86/146 avoided, 40 reference matches at 1000 ms). Match compute: 1270.4 s wall and 4862.2 s CPU including 66.7 s cancelled work; builds and checks are additional. Evidence: docs/m5/evidence/msh166/README.md and verification.json, exact candidate patches and compressed games. AC2 remains unmet: no supported strength improvement. Next action is a decision on a larger predeclared trial or a different coherent approach; the README proposes but does not run a held-out IIR trial. Do not treat its highest point estimate as proven strength. Prototype local tests passed 70 with one ignored; final production has no code changes. Independent final evidence review pending. Integration target is main; the old rewrite-target wording in the checklist predates MSH-171 (Rewrite Main Promotion).

Independent final review approved by /root/two_human_final_review: all seven patch/binary bindings, paired game scores and colours, sequential decisions, final ablation direction, and compute totals checked; restored production source confirmed identical to task start. No open review findings. This resolves the preceding pending-review note. The committed result is evidence and a resumable unfinished task, not a strength improvement.

Ticket revision, 2026-11-15 (requested by John): the approach changed from one technique at a time to the combined set first, with removals only for a recorded reason. The first round is kept as evidence. Its 60-pair tests could not decide, so its "reject" entries are not rejections, and the earlier checks of the first and third criteria no longer apply. The earlier final summary describes the first round only. General game-test rules are now in docs/m5/strength-roadmap.md.
<!-- SECTION:NOTES:END -->
