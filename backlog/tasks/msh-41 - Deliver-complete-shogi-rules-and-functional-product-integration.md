---
id: MSH-41
title: Deliver complete shogi rules and functional product integration
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-10-27T14:09:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-39
  - MSH-63
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-75
documentation:
  - docs/design.md
  - docs/m4/delivery-plan.md
  - docs/m4/implementation-map.md
  - docs/m4/shogi-delivery-review.md
  - docs/m4/evidence/msh75/closure-review.md
ordinal: 41000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-01, P-03, P-04 and A-01 through A-07 require a complete shogi implementation, not a piece-movement demonstration. This scope owns the game-specific rules, adapters and conformance evidence across the product. Shared mechanisms and cross-game integration are owned once by MSH-46; coordinate their delivery without duplicating ownership here. Functional mShogi engine support is required in M4; measured strength targets and tuning belong to MSH-50 in M5.

Open design questions (D-07/D-08): Resolve the authoritative rule sources and conventions for drops, promotions, repetition/perpetual check, impasse and match conclusions; do not infer them from the chess baseline or legacy code. Select the necessary state/action, notation and adapter contracts against the actual consumers, preserving complete actions and history; no speculative universal representation or protocol is prescribed. This To Do scope is not authorization to begin implementation or a change to the current M3 batch.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A sourced, identified and versioned shogi rules profile covers the complete game: initial state, every piece/action category, captures and derived effects, promotions and any drops/special choices, turn progression, legality, automatic/claimable outcomes and history-dependent adjudication. Consequential interpretations are recorded; unresolved required rules block conformance rather than disappearing from scope.
- [x] #2 The reusable rules library owns authoritative position, complete actions and atomic rejection, exact apply/undo, rules-relevant history and match/adjudication state. Saved-state, repetition and search identities are specified separately; clients and training tools do not duplicate legality or outcomes.
- [x] #3 Independent sourced shogi fixtures cover legal and illegal choices, transitions, special cases, history and outcomes. Exact reversal, sequence and record round trips, applicable perft/divide and differential/reference comparisons have reproducible evidence and provenance; self-consistency or legacy agreement alone is insufficient.
- [x] #4 The independently usable console can select and inspect shogi, list and apply complete actions, undo, report history/outcomes and run appropriate diagnostics with honest coverage and separate human/machine output. Game-specific command/notation adapters preserve the authoritative semantics.
- [x] #5 The native mShogi engine can search and play shogi using the same authoritative rules/history, with legal complete results, correct terminal/claim handling, request identity, limits, cancellation and stale-result safety. A functional baseline and reproducible self-play/match smoke evidence are recorded without claiming competitive strength.
- [x] #6 Canonical saved shogi records preserve profile/version, initial state, necessary history, complete actions and applicable match policies/settings/events. Replay restores semantics exactly, and unsupported profiles or lossy notation/adapters fail explicitly. This ticket owns game-specific encoding/replay coverage; common schema migration belongs to MSH-46.
- [x] #7 The desktop supports actual shogi human/engine play and bounded inspection/analysis, complete action entry, readable accessible presentation, outcomes/recovery, save/reopen and history inspection through the authoritative native session. Game-specific end-to-end evidence exists; common infrastructure, cross-game regression and packaged three-platform acceptance are owned by MSH-46 and verified at MSH-47.
- [x] #8 The game coverage ledger and independent review show all required shogi scope satisfied, with no required behavior relabeled as optional or deferred to M5. Shared integration findings remain visible in MSH-46 and block M4 closure in MSH-47.
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
Reconcile the original eight complete-delivery obligations using the accepted MSH-75 independent integrated assessment, actual execution and source-specific CI evidence; preserve every original criterion and historical record. Close only after MSH-75 and required shared prerequisites are complete, with M4/other games/shared final acceptance left open. This is delivery reconciliation, not another implementation batch.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH62 planning refinement2026-10-26: original identity,status,AC and DoD remain intact. This complete-delivery record now depends explicitly on its required bounded component/shared work; docs/m4/delivery-plan.md traces each original criterion and single ownership. No game implementation or delivery completion occurred. Earlier M3-context wording remains historical; current authorization is planning-only.

Owner-authorized MSH75 reconciliation2026-10-27: MSH75 is now Done after fresh independent full-delivery assessment, completed requirements1–8, all required shared tickets69/72/73 and console90 integrated, and final actual verification. Sole integrator@codex, primary codex/rewrite; testedsource80d6b5cc71c8ceb5bf782d93295797aed1e18d66, evidence integration/push4c4145eaacf7fcaf89a338773aa110a3700e4c75. Original AC/DoD wording/dependencies and historical planning notes preserved; all now checked by supported evidence rather than childstatus inference. docs/m4/shogi-delivery-review.md maps AC1profile/completeclauses, AC2nativeauthority/actions/history/reversal, AC3independentfixture/reference/replay, AC4standalonecommonconsole, AC5functionalnativeengine/USI/actualrunner, AC6completecanonicalnative/v2archive and61capacity, AC7actualnormaldesktop andplatformjourneys, AC8independentcompleteledger/reconciliation. Four fresh independent AI reviews PASS; no material product defect; actual GUI/current-source evidence audited separately from tests/source/owner visual approval. Final nativeCI40512188338 and desktopCI40512188317 all3platformjobscomplete/success at4c; original Linux13/14transportfailure preserved, unchanged rerun14/14 and actual missing-engine/retry/nativeQuit independentlyverified. Explicit limitations remainuntimed/finitebaseline/USIsubset/no measuredstrength/no formalaccessibilitycertification/signeddistribution; they do not omit a selectedprofileclause. MSH50strengthfuture, MSH42–47 andsharedlaterextensions/fullM4closure remainopen; nextdependency-readyresearch80–83 unstarted.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Complete standard-shogi delivery accepted under the original eight criteria and immutable mshogi.shogi.csa36.v1 profile, based on MSH-75 independent integrated review rather than component status alone. Native full choices, atomic transitions/reversal, history/adjudication/canonical records, common standalone console, functional history-aware engine, structured supervision/USI/selfplay/independent opponent, lossless v2 persistence and actual normal desktop play/analysis/history/recovery are supported. Full AC1–8 mapping, fresh/reused evidence categories, commands/revisions/platform provenance, independent verdicts and limitations: docs/m4/shogi-delivery-review.md. Product tested80d6b5c, review evidence integrated/pushed4c4145eaacf7fcaf89a338773aa110a3700e4c75 on codex/rewrite; native40512188338 and desktop40512188317 all3platformCI PASS, Linux unchangedrerun14/14 with firsttransportfailure retained. Closed after75Done and allrequiredcurrent shared prerequisites. No production scope/fixture expectation/profile change was needed by this review. M2/M3 and completed history preserved; M4/other games/shared six-game closure remain open, strength remainsMSH50. Next eligibleM4queue80xiangqi then independent81–83research; none started.
<!-- SECTION:FINAL_SUMMARY:END -->
