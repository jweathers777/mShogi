---
id: MSH-159
title: Classical Evaluation Decisions
status: In Progress
assignee:
  - '@codex-coordinator'
created_date: '2026-11-10T18:27:00Z'
updated_date: '2026-11-10T18:54:00Z'
labels:
  - handle:classical-evaluation-decisions
dependencies:
  - MSH-158
ordinal: 164000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Classical Evaluation Decisions.

## Problem
Two quiet endgame decision failures remain after MSH-158; score disagreement alone does not establish an evaluation defect.

## Objective
Improve real Chess decisions through independently justified general classical evaluation corrections, or establish the specific remaining search obstacle.

## Approach
Inspect the complete evaluator and matched intermediate states from 0UOyM, 0m5mI and queen control 28pJK. Establish consequences and rankings with proof/tablebases where available and reviewed finite analysis otherwise. Test a small set of evaluation-only hypotheses on existing development controls; freeze any supported candidate before the existing held-out benchmark. No search changes, new corpus, matches or unrelated work.

## Completion
Retain only supported corrections, preserve native semantics, review independently and integrate normally. Record failed hypotheses and unresolved limits; MSH-153 stays held unless evidence separately justifies it.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Matched full-history intermediate positions and term decompositions establish evaluation ranking weaknesses or precise search-coverage obstacles with independently reviewed consequences.
- [ ] #2 Focused general evaluation-only experiments measure five-second decisions and relevant controls; retained changes have meaningful supported benefit and rejected trials are preserved.
- [ ] #3 Native authority, outcomes, history, state restoration and cancellation remain intact under relevant Level 1 local correctness and independent review.
- [ ] #4 Any supported frozen candidate receives existing160/80 five-second and40 one-second validation without held-out tuning; all new engine computation stays within2400seconds and outcome uncertainty is reported.
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
1. Freeze e93a750 production predecessor from completed MSH-158; reuse its160/80/40 measurements. Record ownership in current assessment checkout, base d1f2c67, primary integration codex/rewrite.
2. Review full evaluator and retained actual interior traces; extract matched states and independent consequence/ranking evidence. Keep held-out sealed during diagnosis.
3. Test a few coherent general evaluation hypotheses with small relevant development subsets and selected15s probes; no search-policy changes or broad weight sweep.
4. Reserve1240s full candidate benchmark,180s references,120s correctness allowance and860s focused diagnostics/experiments within2400global cap. Reuse baseline receipts; no matches.
5. Freeze supported source/procedure before held-out, independently review diagnosis/source/outcomes, then Level1 local regression and normal integration. Stop with precise unresolved mechanism if distinct trials fail; no unrelated continuation.

First isolated trial: in opposite-colored bishop-only endings, discount only advanced passed-pawn promotion premium when a safe bishop currently controls the immediate advance square. Preserve pawn material and escort; require current geometry, not a future Bc1-to-Bd2 tempo. Test negative cases and mirrored native positions, then existing focused development decisions. Second distinct hypothesis is target-aware king activity, accounting for existing passer escort rather than duplicating it.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Complete evaluator review and independent diagnosis found plausible feature gaps, not a proven arithmetic error. Bc1 does not control e1 after ...e2; only Bd2 establishes promotion control. Current static already prefers Bd2 over Kd2, so a higher score alone cannot establish benefit. Full-history finite references include stronger g4 and Ba6 defenses, not only published continuations. Independent reviews /tmp/msh159-evaluator-review.md and /tmp/msh159-eval-diagnosis.md; source owner for trial1 @native_round_plan, coordinator integrates. Baseline source e93a750; no held-out access and no engine work yet.

Exact tablebase lookups and native legal-set replay establish a selected pawn-race static inversion: cached endpoint White loss evaluates+55 while alternate actual leaf White win evaluates+43. A guarded mandatory own-side clearance tempo corrects these to−245/+343 without score calibration; isolated trial selects exf3+ Kh6 at5s and15s, preserving other29mechanism/control choices. Bishop stable-front trial initially selectedKc3 through an invalid geometric king-capture guard; corrected trial returnsBd6 and is not retained on that result. A distinct loose-passer pressure trial corrects finite-supported Bc1>Kc2 static ranking and changes completed15s actual-root continuation from Kc2 retreat to Bc1/Bd2/Kb4, but5s root staysBd6 and separate5s intermediate roots already chooseBc1/Bd2 inbothversions. Further guard review excludes protected intermediate landing squares; corrected combined candidate undergoing checks and actual-root tracing before freeze. Held-out remains sealed; no search policy changes.
<!-- SECTION:NOTES:END -->
