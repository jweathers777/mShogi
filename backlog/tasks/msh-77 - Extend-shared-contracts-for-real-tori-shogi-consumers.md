---
id: MSH-77
title: Extend shared contracts for real tori shogi consumers
status: To Do
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-01T12:24:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-85
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/tori/handoff.md
  - docs/m4/tori/profile.md
  - docs/m4/tori/native-plan.md
  - docs/m4/tori/native-fixture-ownership.json
type: task
ordinal: 77000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Before activation refine against the accepted game profile and concrete native handoff; verify codecs,context races,records and shared UI behavior with independently expected real-game fixtures. No dependency on the aggregate game delivery or MSH46 final completion.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The real game-owned native handoff is consumed through explicit typed profile/record/session/engine dispatch and supported capabilities; existing chess/shogi contexts remain compatible and stale/cancelled results cannot cross profiles.
- [ ] #2 Shared mechanisms accommodate 7x7 square-board/hands,piece promotion/ownership and native game-specific history/action/outcome payloads through native metadata and complete choices,with no frontend legality or mandatory common board representation.
- [ ] #3 Canonical envelope/history/admission supports the concrete game payload and documented resource policy without losing prior accepted data; unsupported profiles/conversions remain explicit.
- [ ] #4 Real authoritative game-generated fixtures and interaction tests establish the extension; synthetic M3 fixtures alone do not prove real-game support. Required cross-game regression and ownership boundaries are recorded.
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
MSH81 currentresearch2026-10-31 supplies docs/m4/tori source/profile-gate ledger and currentR1-R6consumer handoff. Tohoku15-imageprimarywitness nowacquired; textdate1828 is notprovenphysicalcopydate. Candidateprofile remainsunratified; P02promotion/drop timing,P03formalrepetition/check andP04exceptionalendings blockconformance. Preserve nativequailidentity/7x7geometry/history,currentexecution/document/runtime/sharedinteractionowners andsaveability. Source-groundedscopeconstraints only: no detailedfutureplan,dependency/ACchange,Readyactivation or deliveryclosure; complete81 before affected85/77implementation.

Superseding owner-approved modern direction2026-11-01: MSH81 now supplies complete mshogi.tori.modern.v1 in docs/m4/tori/profile.md, concrete independent fixture specs and current R1-R6 handoff. Earlier historical acquisition/P01-P06 blocking notes remain dated evidence but do not gate this selected modern identity. T06 explicitly formalizes third-occurrence departure responsibility and forced cycles; T05/T07 specify rare endings/root/admission. No native conformance already claimed. MSH85 remains To Do and must perform original AC4 bounded native-slice refinement before Ready; MSH77 waits actual85handoff;42 retains complete console/functional-engine/archive/desktop/review obligations. All original dependencies and AC/DoD preserved; no activation or implementation by this note.

Bounded native refinement2026-11-01 (planning only): coordinating85 now owns flat component126fixture/reference admission→127structuralstate/transitions→128locallegality/inspection→129history/records/nativehandoff;126dependsDone81 and no componentdepends85/77/42. All103immutablefixtures explicitly mapped in native-fixture-ownership.json (126dataownerall;127/128/129primary14/69/20), with lateractualprocess/document/notation facets preserved under77/42. Original85AC4 is satisfied only by reviewedplanning, not kernelimplementation;85cannotclose before nativecomponents and coreconsumerprobe. Native GameKind addition may require minimal explicitunsupported exhaustivearms; real engine/session/archive/frontend remainlaterownership. Preserveprofile/oldresearch/alloriginalAC/DoD; nohistoricalacquisition or optionalexactproof gate. See native-plan.md for actual inspectedboundaries, verification and currentdependencysets.
<!-- SECTION:NOTES:END -->
