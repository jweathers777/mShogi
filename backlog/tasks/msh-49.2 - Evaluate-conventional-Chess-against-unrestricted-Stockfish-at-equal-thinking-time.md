---
id: MSH-49.2
title: >-
  Evaluate conventional Chess against unrestricted Stockfish at equal thinking
  time
status: To Do
assignee: []
created_date: '2026-11-08T13:07:00Z'
updated_date: '2026-11-08T15:25:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-49.1
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
  - docs/m5/chess-conventional.md
  - docs/m5/evidence/msh49-1/verification.json
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

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-49.1 handoff: conventionalengine implementation eb4601d0fd33e790ac9905309fb011400aeafb5c and fixture-onlycorrection/testedsource b8cc85167f85eb031830d5f5268c1f94520f8a25 integrated codex/rewrite. RequirednativeCI42492362033 attempt1 passedactualUbuntu24/macOS15/Windows2022; independent source/fixture/provenance reviews approved. Canonicalengineeringevidence docs/m5/evidence/msh49-1/verification.json distinguishes1134localtests/affectedconsumers/profiles from49.6924s selfplay (one native mate, oneunscoredplycap), no strengthestimate. This formalstudyhasnotbegun, noopponenthandicap/threshold/budgetapprovalinferred. PreserveToDo/unassigned untilconcretecomparisonconditions/materialbudgetareselected withJohn perAC2; routinepreparationmaymakeproposalreviewable butmustnotlaunchformal/substantialmatches. Old64/32/depth2SFstudy remainsstopped; do not revivethatprotocol or4hourrequest.
<!-- SECTION:NOTES:END -->
