---
id: MSH-49.2
title: >-
  Evaluate conventional Chess against unrestricted Stockfish at equal thinking
  time
status: To Do
assignee: []
created_date: '2026-11-08T13:07:00Z'
updated_date: '2026-11-08T18:08:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-49.1
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
  - docs/m5/chess-conventional.md
  - docs/m5/evidence/msh49-1/verification.json
  - docs/m5/chess-unrestricted-comparison.md
  - docs/m5/evidence/msh49-2/verification.json
parent_task_id: MSH-49
priority: high
ordinal: 136000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
A meaningful external strength measurement follows the substantially complete conventional engine, not each optimization. The owner stopped the earlier64/32 depth2Stockfish study before any formal root or game. This milestone uses full-strength Stockfish under comparable computational resources, principally equal thinking time. It must not assume parity, a human Elo, an invented score threshold or approval of the superseded4hour request. Routine correctness/tactical tests and bounded profiling accompany coherent increments. Aggregate routine local engine-match work is limited to15minutes per task unless John explicitly authorizes more; no per-technique substantial statistical studies or multi-hour benchmarks. Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The conventional-engine implementation checkpoint is complete and independently reviewed before substantive formal evaluation. Exact source/build/executable, Stockfish binary/network/options and hardware/resources are pinned; full-strength Stockfish has no depth/node/skill/Elo handicap and receives the same thinking-time conditions with recorded comparable threads/memory/pondering policy.
- [ ] #2 Before execution, John approves the material total compute/time budget and a concrete fair comparison protocol/interpretation or acceptance target. Openings with color pairing, sample/stopping/uncertainty rules, time control and failures are prespecified; routine technical setup does not require separate approvals and the old64/32 weak-baseline thresholds are inactive.
- [ ] #3 Actual results and all failed/incomplete attempts are preserved with native histories, process/clock diagnostics and reproducible receipts. Failed/capped/incomplete work is never silently scored or discarded. A clear Stockfish defeat remains an honest result, not a reason to handicap it or invent rating/parity claims.
- [ ] #4 Independent review reconciles final source, complete experimental conditions, uncertainty and actual outcomes. Any unmet agreed release target remains an explicit MSH-49 completion blocker; the completed report distinguishes measured strength from correctness and development regressions.
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
Primary builder Codex in the clean, origin-aligned primary codex/rewrite checkout at 0b3b458. MSH-49.1 is Done; no active competing worker. Preserve the historical-access worktree and stopped old study. (1) Inspect current runner, UCI time/resource behavior, accepted engine evidence and pinned Stockfish artifact. Research official opponent controls. (2) Prepare one concrete unrestricted equal-time comparison protocol, exact artifacts/options/resources, opening sampling/color pairing, fixed sample/stopping/failure/uncertainty interpretation and a bounded local budget. Verify setup with focused offline tests and at most 900 aggregate seconds of explicitly labelled routine smoke matches; no formal study before owner approval. Avoid production changes unless the current harness cannot express the accepted comparison. (3) Independently review protocol/setup, then request only the material total study budget and comparison/acceptance interpretation, with all ordinary preparation complete. Continue unaffected preparation while waiting; elapsed time is never approval. (4) If approved, freeze source/artifacts/protocol before formal roots, execute bounded sequential paired games with one budget ledger, native replay and preserved failures. Independently reconcile actual results, uncertainty and all four original criteria; integrate/push normally and complete only if supported. Otherwise retain unfinished task with exact approval blocker and reviewable handoff. No Shogi/other task or engine optimization in this assignment.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-49.1 handoff: conventionalengine implementation eb4601d0fd33e790ac9905309fb011400aeafb5c and fixture-onlycorrection/testedsource b8cc85167f85eb031830d5f5268c1f94520f8a25 integrated codex/rewrite. RequirednativeCI42492362033 attempt1 passedactualUbuntu24/macOS15/Windows2022; independent source/fixture/provenance reviews approved. Canonicalengineeringevidence docs/m5/evidence/msh49-1/verification.json distinguishes1134localtests/affectedconsumers/profiles from49.6924s selfplay (one native mate, oneunscoredplycap), no strengthestimate. This formalstudyhasnotbegun, noopponenthandicap/threshold/budgetapprovalinferred. PreserveToDo/unassigned untilconcretecomparisonconditions/materialbudgetareselected withJohn perAC2; routinepreparationmaymakeproposalreviewable butmustnotlaunchformal/substantialmatches. Old64/32/depth2SFstudy remainsstopped; do not revivethatprotocol or4hourrequest.

Ownership: @codex, primary codex/rewrite at 0b3b4585c48619f1c9781eb3e623fb6d873342bc, clean and exact origin alignment. Backlog 1.55.1 matches baseline. No Ready dependency-satisfied task exists; next planned MSH-49.2 has completed dependency 49.1 and is activated for concrete authorized preparation. Owner approval of formal study budget/comparison remains mandatory AC2, not inferred from this task assignment. No formal or routine matches have run in this task; routine charge starts 0/900 seconds.

Preparation delivered by Codex at clean origin-aligned0b3b458, primary codex/rewrite; no production changes. Concrete reviewed proposal docs/m5/chess-unrestricted-comparison.md +canonical protocol/build/verification at docs/m5/evidence/msh49-2. Proposed16iid opening pairs/32games, equal go movetime500 only (no externaldepth/node handicap), full SF19 defaultnetwork,1searchthread each/no pondering, same M1Pro32GiB. Memory explicitly unequal/modest: native4096entryTT vsSF16MBHash+98.5MBnetwork. Proposed3600s total active local matches inclusive smoke/failures/cleanup; no paidcompute/vendor fees; stopnewpair<600s remaining, shared5s outercleanup reserve, incomplete/failure remains unscored/no replacement. Proposal accepts complete independentlyreviewed measurement regardlessscore, not numerical releasebar/parity/Elo. OwnerAC2 decision remains mandatory; no formal seed/roots/freeze/results or approval exists. One routine smoke consumed5.840419957879931/900s with8 actual go movetime500 requests,2intentional4plycapsunscored, cleanprocess/no stderr/failures andnative replayexit0. Fresh independent msh492_protocol_review approved setup/protocol/provenance; all5artifacthashes/sourceequivalence and7offlinecontroltests independentlyverified. Foundnestedcleanupwindow issue corrected toone shared5s deadline, controlled timeout test verifies no renewedwindow andhonestunsettledcharge. Actual negative formal invocation refusesbeforeanyengine/rootdraw. Existing acceptednative3OS evidence reused atb8cc851; no Rust/UI/productchanges ornewCIcampaign. ReturnToDo/unassigned solely pending material ownerbudget/interpretation decision, all4AC/DoD unchecked. Upon approval recordactualreply+exactprotocolSHA, reactivateownership, commit decision then freeze/draw/run once; do not revive old64/32/depth2study. Prep integration/push follows, hashes inGit history/completion report.
<!-- SECTION:NOTES:END -->
