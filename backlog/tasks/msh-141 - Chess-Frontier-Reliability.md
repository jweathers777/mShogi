---
id: MSH-141
title: Chess Frontier Reliability
status: In Review
assignee:
  - '@codex'
created_date: '2026-11-09T16:16:00Z'
updated_date: '2026-11-09T17:49:00Z'
labels:
  - handle:chess-frontier-reliability
milestone: m-5
dependencies:
  - MSH-139
priority: high
ordinal: 146000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Frontier Reliability. Owner authorizes continued conventional Chess development after MSH-139 (Chess Strength Assessment), beginning with the Goldfish 41...Kh5 depth-1/295113-visit incident. Establish actual work and root cause before optimization; correct general search-frontier defects without position special cases or concealed limits. Preserve native rules/history authority, cancellation, completed-result semantics and product boundaries. No learned evaluation, rating target, substantial opponent study or cross-game transfer.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Preserved full-history incident and source telemetry reconcile depth/visit meanings; controlled profiles separate completed/incomplete iterations, main/quiescent work, extensions and repeated work, distinguishing proof from hypotheses.
- [x] #2 A general correction addresses any demonstrated significant search-frontier defect with independent tactical expectations, counterexamples and restoration/history/cancellation regression coverage; limits are not adjusted to conceal the problem.
- [x] #3 Bounded before/after realistic-time and deterministic work comparisons cover incident plus independent representative positions; limitations and any unresolved behavior are explicit, with no inferred playing-strength gain.
- [ ] #4 Independent review and relevant native platform verification support the exact integrated source; concise evidence and prioritized conventional follow-up work are recorded while Shogi/Xiangqi remain deferred.
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
Codex owns primary codex/rewrite at d86dfca, clean and origin-aligned; unrelated historical Tenjiku worktree preserved. Use pinned Backlog1.55.1. Reconstruct exact full-history incident from preserved reports. Add bounded per-iteration/frontier diagnostics and reproduce unchanged behavior before choosing a general correction. Independently research established qsearch/threat techniques, implement and verify focused behavior/history/cancellation cases. Compare deterministic and one-second representative searches, review independently, then run required Rust Linux/macOS15/Windows jobs at coherent source; no GUI campaign for search-only changes. Record subsequent endgame/evaluation and coherent development work separately, preserving owner hold on cross-game transfer.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Reproduced exact295113 visit incident from full81ply history: completeddepth1 at7734visits; only5main and295108q visits,287379 visits in interrupteddepth2;0checkextensions/0aspirationretries. General defects: pre-move support gate misses revealed queen support, and repeated quiet-threat/all-defence expansion turns qsearch into a second broad tree. Corrected after-move native support and finite first2q forcing/3q defence frontier; native immediate mates/checkevasions/history/cancellation/limits unchanged. Fixedwork depth8 now12279visits with losingmate score;3uncapped1s repeats reproduce originaldepth1/+139 and corrected stable losingmate. Prior independent quietmate regression and new mirrored/boundary/native cases pass. Independent code/research review approved; local release search/attacks and workspace clippy pass. Required Rust3OS CI and final nativeconsumer check pending. Single evidence docs/m5/evidence/msh141/verification.json. Follow-ups MSH-142 (Classical Endgame Judgment), MSH-143 (Chess Search Efficiency); no matchgames, learned evaluation, ratingtarget or cross-game transfer.

Final local integration checks passed at production source1d20ae6:680 core/search/console tests,0fail,4explicitignored;4 actual release desktop-play tests pass; workspace all-target clippy and format pass. Independent final evidence review matched original raw transcript/fullhistory, archived baseline diagnostics-only diff, all artifact/source/binary hashes,295113visit arithmetic and all3uncapped before/after rounds. Required native CI run42655073948 (Linux129291275785,macOS15 129291275256,Windows129291276143) pending; one60s completion-aware monitor. Auto-triggered Desktop run42655073772 cancelled as unnecessary package campaign; no GUI/interface change. AC4/DoD remain unchecked pending actual3OS completion.

CI run42655073948: all3OS native/release checks passed; Linux129291275785 then failed stale generated shared-chess-analysis-mate telemetry (65visits/seldepth7 now48/5). I missed this fixture refresh before CI. Regenerated with native producer; independent recursive diff confirms exactly8scalars and unchanged move/PV/score/depth/state/schema.34frontend consumer tests pass with pinnedNode24.19.0. Cancelled superseded macOS/Windows jobs, preserved full job receipts and failure diff. Production unchanged from1d20ae6; fixture-inclusive checkpoint and fresh required3OS CI follow. No criteria/limits/tests weakened.

Final platform handoff: production1d20ae6, fixture-inclusive22bf634, both integrated/pushed to codex/rewrite. Rust run42660806919 at22bf634: Linux129311544941 and Windows129311544328 passed all steps (attempt1). macOS129311544814 and supported macOS-only retry129327572801 both cancelled after15m1s without acquiring a hosted runner (runner_id0, no steps). GitHub reports hosted-runner acquisition failure and macOS arm64 capacity constraints. Attempt2 Linux129327575705/Windows129327577416 receipts carry forward attempt1 success; healthy platforms were not rerun. One completion-aware monitor completed; bounded retry campaign stopped after the second capacity failure. Later supported retry of completed macOS job129327572801 on identical22bf634 is the remaining action, subject to current API eligibility/capacity. No source/runner/check/acceptance changes or local macOS substitution. MSH-141 stays In Review; AC4/final acceptance unsupported. Canonical receipts and historical failures: docs/m5/evidence/msh141/verification.json; explanation: docs/m5/chess-frontier-reliability.md. MSH-142 (Classical Endgame Judgment) and MSH-143 (Chess Search Efficiency) remain scoped follow-ups pending this platform gate; no cross-game transfer or strength claim.

Independent final receipt reconciliation (chess_frontier_review) approves blocked handoff with no material mismatch: exact22bf634 source, every Linux/Windows required step passed, both macOS cancellations901seconds/runner_id0/no steps and matching capacity annotations, preserved historical/carried-forward receipts, all artifact hashes match. Production diff1d20ae6..22bf634 empty. In Review and incomplete AC4/final acceptance are required; no local-platform substitution. Evidence-only final commit follows; no active CI monitor or retry remains.
<!-- SECTION:NOTES:END -->
