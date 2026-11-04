---
id: MSH-44
title: Deliver complete tenjiku shogi rules and functional product integration
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-04T16:27:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-39
  - MSH-72
  - MSH-73
  - MSH-79
  - MSH-83
  - MSH-87
documentation:
  - docs/design.md
  - docs/m4/delivery-plan.md
  - docs/m4/implementation-map.md
  - docs/testing/conformance-policy.md
  - docs/m4/tenjiku/README.md
  - docs/m4/tenjiku/handoff.md
  - docs/m4/tenjiku/native-v1.md
  - docs/m4/tenjiku/native-verification.json
ordinal: 44000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-01, P-03, P-04 and A-01 through A-07 require a complete tenjiku shogi implementation, not a piece-movement demonstration. This scope owns the game-specific rules, adapters and conformance evidence across the product. Shared mechanisms and cross-game integration are owned once by MSH-46; coordinate their delivery without duplicating ownership here. Functional mShogi engine support is required in M4; measured strength targets and tuning belong to MSH-53 in M5.

Open design questions (D-07/D-08): Resolve the authoritative historical conventions for compound actions, capture and other rule-derived effects, promotions, royal pieces, repetition and game endings; record disputes rather than choosing silently. Select the necessary state/action, notation and adapter contracts against the actual consumers, preserving complete actions and history; no speculative universal representation or protocol is prescribed. This To Do scope is not authorization to begin implementation or a change to the current M3 batch.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A sourced, identified and versioned tenjiku shogi rules profile covers the complete game: initial state, every piece/action category, captures and derived effects, promotions and any drops/special choices, turn progression, legality, automatic/claimable outcomes and history-dependent adjudication. Consequential interpretations are recorded; unresolved required rules block conformance rather than disappearing from scope.
- [ ] #2 The reusable rules library owns authoritative position, complete actions and atomic rejection, exact apply/undo, rules-relevant history and match/adjudication state. Saved-state, repetition and search identities are specified separately; clients and training tools do not duplicate legality or outcomes.
- [ ] #3 Independent sourced tenjiku shogi fixtures cover legal and illegal choices, transitions, special cases, history and outcomes. Exact reversal, sequence and record round trips, applicable perft/divide and differential/reference comparisons have reproducible evidence and provenance; self-consistency or legacy agreement alone is insufficient.
- [ ] #4 The independently usable console can select and inspect tenjiku shogi, list and apply complete actions, undo, report history/outcomes and run appropriate diagnostics with honest coverage and separate human/machine output. Game-specific command/notation adapters preserve the authoritative semantics.
- [ ] #5 The native mShogi engine can search and play tenjiku shogi using the same authoritative rules/history, with legal complete results, correct terminal/claim handling, request identity, limits, cancellation and stale-result safety. A functional baseline and reproducible self-play/match smoke evidence are recorded without claiming competitive strength.
- [ ] #6 Canonical saved tenjiku shogi records preserve profile/version, initial state, necessary history, complete actions and applicable match policies/settings/events. Replay restores semantics exactly, and unsupported profiles or lossy notation/adapters fail explicitly. This ticket owns game-specific encoding/replay coverage; common schema migration belongs to MSH-46.
- [ ] #7 The desktop supports actual tenjiku shogi human/engine play and bounded inspection/analysis, complete action entry, readable accessible presentation, outcomes/recovery, save/reopen and history inspection through the authoritative native session. Game-specific end-to-end evidence exists; common infrastructure, cross-game regression and packaged three-platform acceptance are owned by MSH-46 and verified at MSH-47.
- [ ] #8 The game coverage ledger and independent review show all required tenjiku shogi scope satisfied, with no required behavior relabeled as optional or deferred to M5. Shared integration findings remain visible in MSH-46 and block M4 closure in MSH-47.
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

Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

MSH83 ratifies two owner-selected whole Tenjiku profiles rather than one universal convention. Complete delivery retains all original8AC and DoD: actual native/shared handoff87/79, functional search/supervision and complete console/notation, native-sourced traditional-kanji16x16 board, two named New Game profile choices, actual playable/save/history/analysis/recovery/platform evidence. Profile is immutable within game; Load preserves identity and fresh New may select other profile. No normal unsupported selector activation, game implementation, M5strength claim or scope closure occurs in83. See currentprofile/corpus/decisions/handoff; no new planning hierarchy/dependency.

MSH87actual native handoff implements both immutable modernprofiles and source-derived complete actions/effects/Game/history/schema1/exactreversal/conformance; review/evidence docs/m4/tenjiku/native-v1.md and native-verification.json. Native source is not finished product:79stillowns shared/session/frontend/whole-document admission and selected-profile New Game,44retains functional history-aware evaluation/search, completeconsole/notation, actual engine journeys, traditional-kanji/promoted-face assets and final16x16playable delivery. No normal selector or Tenjiku engine advertised; no44implementation started. Original complete-game acceptance criteria remainunchecked and unchanged.

87nativehand offDone at9749eec, independentacceptance and exact-source Ubuntu/macOS/Windows RustCI41816309261success. Bothwholeprofiles complete native actions/effects/history/records/reversal/conformance delivered; 204rows/621fields perprofilepassed,0failed.79nowReady foractualsharedconsumer but unstarted. Original44completeproductcriteria remainunchanged/unchecked: functionalnativeevaluation/search, console/notation, actualenginejourneys, traditional-kanji/promotedassets andfinal16x16playablepresentation remain44work. Native1MiB/1024events/512Munits andmeasuredcosts do not establishwhole-desktop or15secondworkeracceptance. Noengineadvertisement ornormalgameselectoractivation in87.

MSH79 shared handoff implemented/reviewed and integrated as3a290024df82d23f215165707c81de4928ad3a4a; requiredRust41856238536/Desktop41856238041CI stillpending, so79notyetDone. See docs/m4/tenjiku/shared-v1.md andsingle shared-verification.json forexplicitwholeprofiles, immutable current-envelope payload, solecontroller/workers/install, exacthistory/branches/observations/freshstoppedOpen,16x16opaquechoices andnativeeffectphases. Measured160Maggregatework policy plusindependent15sdeadline andactualendingClose reserves; raw5.5sdecode/native metricsseparatedfromrealworkerOpen/Save/install proof. Twelveconsumerrows exercisedbothprofiles;5productrowsremain44. Execution staysgenericunscoredUnsupportedMode; normalNew/Load/selector off, labeledQAonly.44mustdeliverfunctionalnativeevaluation/full-historyboundedsearch, actualsupervisedengine/completegames, console/finalnotation, licensedkanji/finalboard andnormal/minimumthreeplatformgameplay beforeenablingbothnamedwholeprofilechoices. Preserveall8originalAC/DoD;44ToDo/unassigned/unstarted, noimplementationplan ornewhierarchy.

Final79handoff: source3a290024df82d23f215165707c81de4928ad3a4aimplementationindependentlyaccepted, but79remainsInReview becausemacOS15Rust/Desktoprunneracquisitionfailedtwice; same-sourceDesktopthirdretry126702335253queued, RustMacretryawaitsLinuxrunterminalreceipt. WindowsRust andLinux/WindowsDesktopactual5gamecampaignspassed; LinuxRustallrequiredstepspassedbutterminalmetadatareceiptpendingatlastrefresh. See79singleledger/shared-v1foractualcontracts,resource/lockmetrics,failuresandjobIDs. DoNOTtreat79asDone/44dependency-readyuntilrequiredterminalreceiptsreconciled.44remainsToDo/unassigned/unstarted, alloriginal8AC/DoDunchecked; nofunctionalengine,normalactivation,finalnotation/artorpackagedTenjigameplayclaim. Nextresume79only; no44implementationplanoradditionalhierarchy.

Latest79handoff supersedesLinux-pendingnote: four exact3a290jobs have terminalsuccess (RustLinux/Windows, DesktopLinux/Windowswithactual5gameGUI). OnlyrequiredmacOS15Rust/Desktopreceiptsmissingaftertwo hostedrunneracquisitioncancellations; thirdDesktopjob126702335253queued, RustMacrerunawaitsworkflowAPIterminalstate.79InReview/AC4+DoD1incomplete, sourceimplementationaccepted, no44activation. Resume79onlyforCIreconciliation/finalization;44ToDo/unassigned/unstarted andalloriginal8AC/DoDunchanged.
<!-- SECTION:NOTES:END -->
