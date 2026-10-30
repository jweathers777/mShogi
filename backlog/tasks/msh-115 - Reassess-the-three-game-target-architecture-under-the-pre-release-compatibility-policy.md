---
id: MSH-115
title: >-
  Reassess the three-game target architecture under the pre-release
  compatibility policy
status: Done
assignee:
  - '@codex'
created_date: '2026-10-30T07:54:00Z'
updated_date: '2026-10-30T08:16:00Z'
labels:
  - architecture-review
  - planning
milestone: m-4
dependencies:
  - MSH-106
documentation:
  - docs/design.md
  - docs/architecture-health-roadmap.md
  - docs/review/architecture-health-106.md
  - docs/review/architecture-target-115.md
type: spike
ordinal: 115000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner corrects the MSH-106 compatibility premise: this unreleased greenfield rewrite has no external internal-API consumers or development-save compatibility obligation. Reassess actual target architecture without preserving obsolete adapters/readers merely because earlier stages used them. Research/documentation/independent critique only; no production refactor, implementation task creation, variant, P9 or M5 execution. Preserve native semantics/histories/atomic admission, execution safety, current-format lossless persistence, product consistency and external protocol interoperability. Keep completed106/evidence historical and unchanged.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Record the owner-approved pre-release compatibility policy durably, separating disposable internal interfaces/formats from required native semantics, execution/persistence guarantees and external standards.
- [x] #2 Compare actual current Chess/Shogi/Xiangqi frontend session, board interaction, native lifecycle/admission, archive and IPC responsibilities at an exact source; identify unnecessary compatibility and tests preserving obsolete interfaces.
- [x] #3 Publish a source-grounded target architecture comparing meaningful incremental versus coherent-replacement alternatives, with retain/migrate/replace/delete inventory, shared/native ownership and full semantic preservation.
- [x] #4 Reconcile relevant MSH-106 conclusions as retained, modified or withdrawn under the corrected premise; preserve historical evidence without making it a production compatibility requirement.
- [x] #5 Provide proposed bounded implementation scopes/dependencies/timing, benefits/costs, correctness/performance risks and differential/regression/deletion verification, without creating or activating implementation tasks.
- [x] #6 Obtain independent critique for unnecessary compatibility, overgeneralization, incomplete deletion/migration, hidden native/history changes, missed sharing and excessive cost; resolve report findings and record alternatives/material owner choices.
- [x] #7 Record actual source/static/dependency/document checks and reused evidence limits honestly; preserve separate P9 triage/completed delivery evidence, with no production implementation or changed milestone gates.
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
Primary integrator Codex in main codex/rewrite at82bec546991acd552824a1c2e4a554541dc03b15. Inspect source/ownership/dependencies and106 evidence; bounded readonly research for UI/archive/IPC deletion and test mapping; record accepted policy in design. Compare coherent replacement and incremental options; publish target/deletion/test matrix and proposed sequence only. Obtain fresh independent critique, validate source/task/evidence preservation/docs/graph, finalize only new review and normally integrate/push. No refactor/implementation-task creation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup: clean main codex/rewrite82bec54, fetchedorigin aligned0/0; no competingInProgress task. Pinned Backlog1.55.1 used after global1.55.0 mismatch; overview/creation/execution guides read. Initial non-escalatedcreate hit required.git Backlogcreationlock permissions; authorizedCLIretry created only115. Dependency106Done; completed106/45/history preserved. Newaccepted ownerpolicy recorded in designA09/D17, separate from unapproved target. Actualreadonly source research reassess115_ui/archive inspected current callers/hosthandler/snapshot/commands/archive/enginewire. Draft target recommends coherent staged currentcontract replacement, no perpetuallegacyfacade/readers, while preserving native semantics. Current sharedChessDTO is incomplete: embeddedoldSnapshot, empty commonobservations and no completeTSclaim/Fulloperations; migration inventory explicitly protects these behaviors before deletion. Report compares currentpergameenvelopes/incremental/coherent/universal/bigbang alternatives and proposes R1-R6 scopes/deps/deletion/tests; no implementationtask created. Fresh independent critique underway; no productionchanges or test/build/GUI/P9/M5campaign.

Fulltargetproposal ready docs/review/architecture-target-115.md. Acceptedpolicy A09/D17 is separate from owner-unapproved architecture. Retained106nativeauthority/realcompleteactions/measurementfirst; modifiedP1/P2 toreplace duplicatecontrollers/deletelegacyChess after fullbehaviorport; withdrewblanketoldarchive/internalAPIpreservation. Recommendedonecurrentdesktopouterenvelope/nativevariants, oneactive nativedocument/runtime/install/admission owner, oneinternalenginewire/currentIPC and frontendowner. Alternativecurrentpergameenvelopes viable; genericrules/framework andbigbangrejected. Complete removal/testsemantic matrix protects claims/Full/Unknown/nativeMatchRecord/Takebacklineage/proofhistory/identity/admission/quit. R1-R6 are reportproposals only, not nativeimplementationtickets. Fresh readonly reassess115_critique readrequest/policy/report/source acrossUI/host/engine/archive/native/tests; APPROVED with two nonblocking stagingnotes resolved and rechecked: bridge direction/deletionendpoints; R1-R5 stageown deletions, R6audit. R1includes durableobservationvalidators andcurrentstageSaveOpen. No materialdispute. No production edits/tests/builds/GUI/benchmark/P9triage; all earlier evidence remains historical and no currenttargetexecution claim.

Acceptance mapping: AC1designA09/D17 acceptedpolicy; AC2actualsource sec3/5; AC3targetalternatives+ownership/deletionmatrix sec1/3/4; AC4each106finding reconciledsec2; AC5R1-R6scopes/deps/timing/costs/risks/differentialtests sec4-6; AC6fresh independent critique/recheckedstaging sec7; AC7actualstatic+preservationchecks/P9separation/evidencelimits sec7.38localrefs/designIDs/full115-taskgraphPASS; all114oldtaskfiles/106report+evidence/production byte-unchanged; roadmap historicalsuffixverbatim,107-109gatesunchanged; doctor/diffPASS. All7AC/fourDoDs supported. No productiontest/build/GUI/benchmark/CIrerun or targetimplementation claim. Onlynew115review/policy/report/roadmappointer; no implementationtasksactivated/created. Normalcodex/rewritecommit+push completes durable review; ownerwillreviewtargetbeforeimplementation.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed MSH-115 architectural reassessment at reviewedsource82bec546991acd552824a1c2e4a554541dc03b15. Accepted pre-release policy recorded in docs/design.md A09/D17: no required external internal-API/development-save compatibility; historicalevidence preserved, native/currentfunctional/interop guarantees remain. docs/review/architecture-target-115.md recommends coherent bounded replacement over permanent compatibility extraction: one frontend/host/runtime owner and native-tagged document, one internal execution family, one current desktop envelope with authoritative native payloads, shared native-choice interaction with specialized renderers. Fullretain/migrate/replace/delete+testsemantic matrix preserves Chessclaims/Full/history/quit, nativeMatchRecord, Shogideclarations, Xqwarning/proofs, Takebacklineages, exactcontext/cancellation/saveability. Modified106P1/P2/P3/P4; withdrewoldformat/facadepreservation obligation; retainednativeownership/explicitdispatch/completeactions/measurement-first/externalprotocols/P9separation. Alternativesinclude separatecurrentpergameenvelopes; universalgamemodel/bigbangrejected. R1-R6implementationunits/deps/risks/verification are proposals only, no taskscreated untilownerreview. Directionaltemporarybridges have deletionendpoints; stageownsdeletion, finalauditnotgarbagecollector. Fresh independent reassess115_critique actualsource/report critique APPROVED, no materialfindings; two stagingnotesresolved/rechecked. Newstatic38links/designIDs/115taskDAG/doctor/diff/preservationPASS; no productiontests/builds/GUI/benchmarks/CIreruns. All114oldrecords/106evidence/production unchanged. Normalcommit/integration/push codex/rewrite; stopafter115. MSH106Done, M4/openfuturegatesunchanged; no P1-P3refactor/variant/P9/M5work.
<!-- SECTION:FINAL_SUMMARY:END -->
