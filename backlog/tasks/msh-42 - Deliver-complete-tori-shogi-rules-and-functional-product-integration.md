---
id: MSH-42
title: Deliver complete tori shogi rules and functional product integration
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-10-26T07:46:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-39
  - MSH-72
  - MSH-73
  - MSH-77
  - MSH-81
  - MSH-85
documentation:
  - docs/design.md
  - docs/m4/delivery-plan.md
  - docs/m4/implementation-map.md
ordinal: 42000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-01, P-03, P-04 and A-01 through A-07 require a complete tori shogi implementation, not a piece-movement demonstration. This scope owns the game-specific rules, adapters and conformance evidence across the product. Shared mechanisms and cross-game integration are owned once by MSH-46; coordinate their delivery without duplicating ownership here. Functional mShogi engine support is required in M4; measured strength targets and tuning belong to MSH-51 in M5.

Open design questions (D-07/D-08): Resolve the authoritative historical rule sources and conventions for movement, promotions, drops, repetition and game endings; record disputed interpretations before implementing affected behavior. Select the necessary state/action, notation and adapter contracts against the actual consumers, preserving complete actions and history; no speculative universal representation or protocol is prescribed. This To Do scope is not authorization to begin implementation or a change to the current M3 batch.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A sourced, identified and versioned tori shogi rules profile covers the complete game: initial state, every piece/action category, captures and derived effects, promotions and any drops/special choices, turn progression, legality, automatic/claimable outcomes and history-dependent adjudication. Consequential interpretations are recorded; unresolved required rules block conformance rather than disappearing from scope.
- [ ] #2 The reusable rules library owns authoritative position, complete actions and atomic rejection, exact apply/undo, rules-relevant history and match/adjudication state. Saved-state, repetition and search identities are specified separately; clients and training tools do not duplicate legality or outcomes.
- [ ] #3 Independent sourced tori shogi fixtures cover legal and illegal choices, transitions, special cases, history and outcomes. Exact reversal, sequence and record round trips, applicable perft/divide and differential/reference comparisons have reproducible evidence and provenance; self-consistency or legacy agreement alone is insufficient.
- [ ] #4 The independently usable console can select and inspect tori shogi, list and apply complete actions, undo, report history/outcomes and run appropriate diagnostics with honest coverage and separate human/machine output. Game-specific command/notation adapters preserve the authoritative semantics.
- [ ] #5 The native mShogi engine can search and play tori shogi using the same authoritative rules/history, with legal complete results, correct terminal/claim handling, request identity, limits, cancellation and stale-result safety. A functional baseline and reproducible self-play/match smoke evidence are recorded without claiming competitive strength.
- [ ] #6 Canonical saved tori shogi records preserve profile/version, initial state, necessary history, complete actions and applicable match policies/settings/events. Replay restores semantics exactly, and unsupported profiles or lossy notation/adapters fail explicitly. This ticket owns game-specific encoding/replay coverage; common schema migration belongs to MSH-46.
- [ ] #7 The desktop supports actual tori shogi human/engine play and bounded inspection/analysis, complete action entry, readable accessible presentation, outcomes/recovery, save/reopen and history inspection through the authoritative native session. Game-specific end-to-end evidence exists; common infrastructure, cross-game regression and packaged three-platform acceptance are owned by MSH-46 and verified at MSH-47.
- [ ] #8 The game coverage ledger and independent review show all required tori shogi scope satisfied, with no required behavior relabeled as optional or deferred to M5. Shared integration findings remain visible in MSH-46 and block M4 closure in MSH-47.
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
MSH62 planning refinement2026-10-26: original identity,status,AC and DoD remain intact. This complete-delivery record now depends explicitly on its required bounded component/shared work; docs/m4/delivery-plan.md traces each original criterion and single ownership. No game implementation or delivery completion occurred. Earlier M3-context wording remains historical; current authorization is planning-only.
<!-- SECTION:NOTES:END -->
