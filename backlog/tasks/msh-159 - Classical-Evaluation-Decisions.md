---
id: MSH-159
title: Classical Evaluation Decisions
status: Done
assignee:
  - '@codex-coordinator'
created_date: '2026-11-10T18:27:00Z'
updated_date: '2026-11-10T19:32:00Z'
labels:
  - handle:classical-evaluation-decisions
dependencies:
  - MSH-158
documentation:
  - docs/m5/classical-evaluation-decisions.md
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
- [x] #1 Matched full-history intermediate positions and term decompositions establish evaluation ranking weaknesses or precise search-coverage obstacles with independently reviewed consequences.
- [x] #2 Focused general evaluation-only experiments measure five-second decisions and relevant controls; retained changes have meaningful supported benefit and rejected trials are preserved.
- [x] #3 Native authority, outcomes, history, state restoration and cancellation remain intact under relevant Level 1 local correctness and independent review.
- [x] #4 Any supported frozen candidate receives existing160/80 five-second and40 one-second validation without held-out tuning; all new engine computation stays within2400seconds and outcome uncertainty is reported.
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
1. Freeze e93a750 production predecessor from completed MSH-158; reuse its160/80/40 measurements. Record ownership in current assessment checkout, base d1f2c67, primary integration codex/rewrite.
2. Review full evaluator and retained actual interior traces; extract matched states and independent consequence/ranking evidence. Keep held-out sealed during diagnosis.
3. Test a few coherent general evaluation hypotheses with small relevant development subsets and selected15s probes; no search-policy changes or broad weight sweep.
4. Reserve1240s full candidate benchmark,180s references,120s correctness allowance and860s focused diagnostics/experiments within2400global cap. Reuse baseline receipts; no matches.
5. Freeze supported source/procedure before held-out, independently review diagnosis/source/outcomes, then Level1 local regression and normal integration. Stop with precise unresolved mechanism if distinct trials fail; no unrelated continuation.

First isolated trial: in opposite-colored bishop-only endings, discount only advanced passed-pawn promotion premium when a safe bishop currently controls the immediate advance square. Preserve pawn material and escort; require current geometry, not a future Bc1-to-Bd2 tempo. Test negative cases and mirrored native positions, then existing focused development decisions. Second distinct hypothesis is target-aware king activity, accounting for existing passer escort rather than duplicating it.

Final disposition: reject the whole frozen combined candidate after full comparison and independent outcome review; restore predecessor evaluation byte-for-byte. Do not select a subvariant or retune after held-out exposure. Complete this bounded investigation with preserved evidence, restored-source identity, audit and normal documentation integration.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Complete evaluator review and independent diagnosis found plausible feature gaps, not a proven arithmetic error. Bc1 does not control e1 after ...e2; only Bd2 establishes promotion control. Current static already prefers Bd2 over Kd2, so a higher score alone cannot establish benefit. Full-history finite references include stronger g4 and Ba6 defenses, not only published continuations. Independent reviews /tmp/msh159-evaluator-review.md and /tmp/msh159-eval-diagnosis.md; source owner for trial1 @native_round_plan, coordinator integrates. Baseline source e93a750; no held-out access and no engine work yet.

Exact tablebase lookups and native legal-set replay establish a selected pawn-race static inversion: cached endpoint White loss evaluates+55 while alternate actual leaf White win evaluates+43. A guarded mandatory own-side clearance tempo corrects these to−245/+343 without score calibration; isolated trial selects exf3+ Kh6 at5s and15s, preserving other29mechanism/control choices. Bishop stable-front trial initially selectedKc3 through an invalid geometric king-capture guard; corrected trial returnsBd6 and is not retained on that result. A distinct loose-passer pressure trial corrects finite-supported Bc1>Kc2 static ranking and changes completed15s actual-root continuation from Kc2 retreat to Bc1/Bd2/Kb4, but5s root staysBd6 and separate5s intermediate roots already chooseBc1/Bd2 inbothversions. Further guard review excludes protected intermediate landing squares; corrected combined candidate undergoing checks and actual-root tracing before freeze. Held-out remains sealed; no search policy changes.

Candidate96e4bcd is frozen; procedure recorded in9831230 before held-out reveal. Production changes only classical evaluation. Exact pawn-race evidence selected guarded clearance/race timing instead of speculative king-target distance; final opposite-bishop restriction includes current loose-pawn pressure and protected-landing/capturable-controller guards. Combined30-root development panel changes only the two target decisions to exf3+ and Kc3; Qc8 and28other controls retained. Level1:90 Chess tests pass,1manual profile ignored, release build succeeds on macOS arm64. Source and actual-trace reviews approve wider validation; pawn trace does not expose the corrected endpoint, so causal attribution stays bounded. Full160/80 five-second and40 one-second validation running under2400s global cap. Broader Linux/Windows and packaged GUI compatibility deferred to integration/release checkpoint; no platform-sensitive changes. No held-out tuning or unrelated work.

Full frozen results:5/160 development changes,0/80 held-out changes,1/40 fast change. Independent review supports target exf3+ and Kc3 benefits, suggests Rf3 benefit, and establishes material adverse Rg5-to-Rc2 and Bb1-to-Qb4+ choices. Reject combined96e4bcd under frozen retention gate; production crates/Cargo match d1f2c67 and evaluator matches e93a750. All48primary WDL and3mate goals preserved. Audit400native/12reference successes,3284hashes, legal full-history/PV replay, nooverlap; timed1788.75s, conservative registry2052.74/2400. Report docs/m5/classical-evaluation-decisions.md; raw archive and independent outcome review under docs/m5/evidence/msh159. No new engine work or unvalidated component selection; MSH-153 remains held. Separate deferred MSH-160 planning on integration branch is preserved and not activated.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the focused evaluation investigation; no production correction retained. Exact pawn-race endpoint ranking and finite-supported bishop defensive-role weaknesses were established. General experimental corrections changed five-second targets to exf3+ and Kc3, but the full frozen comparison found two material strategic regressions. Independent outcome/closure review rejected candidate96e4bcd; evaluator restored byte-for-byte to verified e93a750 predecessor, all production crates/Cargo unchanged from d1f2c67. Full160development/80heldout5s and40fast1s runs complete; heldout80unchanged;48WDL/3mate primary objectives preserved. Candidate90tests pass/1manualignored and release build pass; retained source reuses unchanged predecessor evidence.400native+12reference runs,3284original hashes audited;1788.75s timed,2052.74s conservative charge below2400. Level1 macOSarm64; broader platform/GUI checks deferred to integration/release, not claimed. Evidence report docs/m5/classical-evaluation-decisions.md and docs/m5/evidence/msh159 preserve failures, exact/finite limits, actual traces and rejected variants. MSH-153 held; MSH-160 deferred planning preserved. No further workstream started. Normal integration target codex/rewrite; integration reference will be appended after merge.

Integrated evidence/restoration commit c2075fc into codex/rewrite at c029333922effc29c86121ecee9662ee24d313f6, preserving unrelated ec1bcde opening-assessment planning. Production source remains identical to the verified predecessor. Independent final closure review verifies restored source and all3007archive entries. The application-primary assessment checkout is retained clean/synchronized for this chat because it is not an archivable disposable worktree; no ongoing engine process or experiment remains. Stop here: MSH-153 stays held and MSH-160 is deferred, with no next work item activated.
<!-- SECTION:FINAL_SUMMARY:END -->
