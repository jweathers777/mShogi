---
id: MSH-42
title: Deliver complete tori shogi rules and functional product integration
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-01T12:24:00Z'
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
  - docs/m4/tori/handoff.md
  - docs/m4/tori/profile.md
  - docs/m4/tori/native-plan.md
  - docs/m4/tori/native-fixture-ownership.json
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

MSH81 currentresearch2026-10-31 supplies docs/m4/tori source/profile-gate ledger and currentR1-R6consumer handoff. Tohoku15-imageprimarywitness nowacquired; textdate1828 is notprovenphysicalcopydate. Candidateprofile remainsunratified; P02promotion/drop timing,P03formalrepetition/check andP04exceptionalendings blockconformance. Preserve nativequailidentity/7x7geometry/history,currentexecution/document/runtime/sharedinteractionowners andsaveability. Source-groundedscopeconstraints only: no detailedfutureplan,dependency/ACchange,Readyactivation or deliveryclosure; complete81 before affected85/77implementation.

Superseding owner-approved modern direction2026-11-01: MSH81 now supplies complete mshogi.tori.modern.v1 in docs/m4/tori/profile.md, concrete independent fixture specs and current R1-R6 handoff. Earlier historical acquisition/P01-P06 blocking notes remain dated evidence but do not gate this selected modern identity. T06 explicitly formalizes third-occurrence departure responsibility and forced cycles; T05/T07 specify rare endings/root/admission. No native conformance already claimed. MSH85 remains To Do and must perform original AC4 bounded native-slice refinement before Ready; MSH77 waits actual85handoff;42 retains complete console/functional-engine/archive/desktop/review obligations. All original dependencies and AC/DoD preserved; no activation or implementation by this note.

Bounded native refinement2026-11-01 (planning only): coordinating85 now owns flat component126fixture/reference admission→127structuralstate/transitions→128locallegality/inspection→129history/records/nativehandoff;126dependsDone81 and no componentdepends85/77/42. All103immutablefixtures explicitly mapped in native-fixture-ownership.json (126dataownerall;127/128/129primary14/69/20), with lateractualprocess/document/notation facets preserved under77/42. Original85AC4 is satisfied only by reviewedplanning, not kernelimplementation;85cannotclose before nativecomponents and coreconsumerprobe. Native GameKind addition may require minimal explicitunsupported exhaustivearms; real engine/session/archive/frontend remainlaterownership. Preserveprofile/oldresearch/alloriginalAC/DoD; nohistoricalacquisition or optionalexactproof gate. See native-plan.md for actual inspectedboundaries, verification and currentdependencysets.
<!-- SECTION:NOTES:END -->
