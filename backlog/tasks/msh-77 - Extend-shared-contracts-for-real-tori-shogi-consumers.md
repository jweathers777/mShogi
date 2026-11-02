---
id: MSH-77
title: Extend shared contracts for real tori shogi consumers
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-02T10:44:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-85
  - MSH-130
  - MSH-131
  - MSH-132
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
  - docs/m4/tori/shared-plan.md
  - docs/m4/tori/shared-consumer-ownership.json
  - docs/m4/tori/shared-plan-review.md
  - docs/m4/tori/shared-presentation.md
  - docs/m4/tori/shared-presentation-evidence.md
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Planning-only refinement at a3677e1e862fdf3877c784f12f56a99c4bfbf167; root Codex is sole builder/integrator in primary codex/rewrite, no other active task owner and unrelated worktrees preserved.
1. Inspect selected modern profile, completed85/126-129,77/42/46 and actual surviving R1-R6 core/execution/controller/document/frontend/choice consumers.
2. Define minimal independently verifiable implementation slices with concrete types, fixtures, resource and stale-result obligations. Sequence codec/admission before installable runtime if needed to avoid transient unsaveable accepted work. Preserve original77AC/DoD and42/46ownership; no engine, console, product board or chooser implementation.
3. Create only source-justified flat component tasks with complete dependencies and executable acceptance/verification expectations, no speculative child implementation plans. Exhaustively map14pending fields and83consumer requirements to slices plus retained42duties.
4. Obtain separate read-only review of current source grounding, boundaries and graph; resolve findings. Validate unchanged713assertion inputs/native source, original criteria, all fixture owners/links/dependencies, backlog doctor and diff.
5. Integrate reviewed planning normally and push. Leave77 open/blocked on components, first dependency-satisfied sliceReady/unassigned; stop before implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH81 currentresearch2026-10-31 supplies docs/m4/tori source/profile-gate ledger and currentR1-R6consumer handoff. Tohoku15-imageprimarywitness nowacquired; textdate1828 is notprovenphysicalcopydate. Candidateprofile remainsunratified; P02promotion/drop timing,P03formalrepetition/check andP04exceptionalendings blockconformance. Preserve nativequailidentity/7x7geometry/history,currentexecution/document/runtime/sharedinteractionowners andsaveability. Source-groundedscopeconstraints only: no detailedfutureplan,dependency/ACchange,Readyactivation or deliveryclosure; complete81 before affected85/77implementation.

Superseding owner-approved modern direction2026-11-01: MSH81 now supplies complete mshogi.tori.modern.v1 in docs/m4/tori/profile.md, concrete independent fixture specs and current R1-R6 handoff. Earlier historical acquisition/P01-P06 blocking notes remain dated evidence but do not gate this selected modern identity. T06 explicitly formalizes third-occurrence departure responsibility and forced cycles; T05/T07 specify rare endings/root/admission. No native conformance already claimed. MSH85 remains To Do and must perform original AC4 bounded native-slice refinement before Ready; MSH77 waits actual85handoff;42 retains complete console/functional-engine/archive/desktop/review obligations. All original dependencies and AC/DoD preserved; no activation or implementation by this note.

Bounded native refinement2026-11-01 (planning only): coordinating85 now owns flat component126fixture/reference admission→127structuralstate/transitions→128locallegality/inspection→129history/records/nativehandoff;126dependsDone81 and no componentdepends85/77/42. All103immutablefixtures explicitly mapped in native-fixture-ownership.json (126dataownerall;127/128/129primary14/69/20), with lateractualprocess/document/notation facets preserved under77/42. Original85AC4 is satisfied only by reviewedplanning, not kernelimplementation;85cannotclose before nativecomponents and coreconsumerprobe. Native GameKind addition may require minimal explicitunsupported exhaustivearms; real engine/session/archive/frontend remainlaterownership. Preserveprofile/oldresearch/alloriginalAC/DoD; nohistoricalacquisition or optionalexactproof gate. See native-plan.md for actual inspectedboundaries, verification and currentdependencysets.

Owner authorizes MSH77 refinement only. Startup clean primarycodex/rewrite a3677e1, origin refreshed/aligned; pinnedBacklog1.55.1 replaces global1.55.0 for all mutations. Accepted native source994521b/profilemshogi.tori.modern.v1/85Done preserved. No other active task records found. Separate tori77_boundary_review performs read-only independent plan/source/graph review; root alone owns durable planning changes. No taskworktree or application launch needed.

Source-grounded refinement creates flat components130document/admission→131runtime/lifecycle→132complete semantic/frontend consumer verification. Codec-first prevents temporary accepted unsaveable sessions;131alone owns ActiveDocument/soleinstall,130onlyimmutable candidate payload. Original77AC/DoD unchanged; all prior69/72/73/85deps retained and complete set now includes130/131/132. Children scopedReady/unassigned/noimplementationplans; only130dependencyready. All14pending+83requirements mapped individually in supplementaryshared-consumer-ownership.json, preserving immutable native manifest:131final13/82actualruntime verdicts,42notation1/1,130/132supporting evidence notdoublecounted. No42/46dependencychange or engine/productimplementation.

Planning/refinement complete, not implementation acceptance. Separate read-only tori77_boundary_review finalPASS/no unresolvedmaterialfindings after exactsource main.rs reference correction and explicit native-controller vs normalproductadmission gate. Actual independent source/type/ownership review plus graph132tasks353edges, all14expectedvalues and83originalowner/scope rows, original77ACDoD/unchanged42/46/nativeinputs checksPASS; doctor/diffPASS. Root optionalmetadataaudit confirms37links, immutablepins and productionunchanged. Current plan/shared-consumer map/currenthandoff+deliverymap integrated with concrete130codec/admission→131runtime/lifecycle/soleinstallation→132semantic/frontendverification. Nativecontrolleracceptance never exposes Tori to unsupported normalrenderer; productOpen/NewGame stayexplicitunsupported until42realconsumer. All original77AC/DoD stayunchecked and77open, Ready/unassignedblockedoncomponents;130alone dependency-readyReady, nochildplans/owners/checks. No native/CI/GUIcampaign warranted for planning-onlychanges;713native evidence reused, notrerun. Noapp/worktreecreated, unrelatedworktreesandallhistorypreserved. Ready for normalplanningintegration/push; stopbeforeanyimplementation.

Reviewed planning integrated and normally pushed to codex/rewrite at8977a4b9c749e46146255861d1ead10f744e5b97 (origina3677e1→8977a4b). No production/source/profile/fixture/expectation change; accepted source994521b and85Done evidence untouched. Separate independent planreviewPASS, full132task353edgegraph/39links/fixtureownership/unchangedoriginal77criteria/allotherexistingtaskrecords/doctor/diffPASS. No native/CI/GUIcampaign rerun or newpassclaim; [skip ci] documentation-only convention. MSH77Ready/unassignedblockedon130/131/132, all originalAC/DoDunchecked;130alone dependency-readyReady. Three scopedcomponents contain constraints/observableAC/verification but no implementationplans. NativecontrollerSave/Open uses solearchitecture while normalproductToriadmission staysgatedfor42realrenderer/engine. No taskworktreecreated, unrelated111/100/98worktreesuntouched; primarycodex/rewrite retained. Stopafterrefinement, no implementation or nexttaskactivated.

MSH132 component handoff (no77activation/closure): production52b0390cac09faf727a0c5ed74c1fd673f468a11 adds complete nativeTori7x7/faces/quails/hands/forcedchoices/selected-liveprefix/status/provenance andPlay-onlyMoves/nativecues to existing sharedFrontendowner. docs/m4/tori/shared-presentation.md and shared-presentation-evidence.md map all4original77AC for its later independentacceptance; evidence/shared-presentation-audit.json reconciles every97supplementalrow to131actual13fields/82requirements, original713/103unchanged;42notation1/1stillpending. Actual stableQA CUAcontractjourneys and normalpackageexclusion recorded, notToriengine/finalUI/threeplatformGUIapproval. NormalNewGame/Open staygated; native15sdeadline/whole-documentlimitsunchanged and truthfulSave/Loadtime-budgetrefusal noted. Independentnative+frontendreviewsPASS;132finalCI/closurepending atthisnote.77remainsReady/unassigned andallAC/DoDunchecked;42/46/88/89ownershipunchanged, no nextimplementation started.

Final132handoff: all6AC/4DoD supported; production52b0390 source and bothCI41427941953/41427942000 SUCCESS acrossall3systems. Native253/frontend342/TS/host14features and normalpackagechecksPASS; actualexisting-gameLinux/WindowsGUI compatibilityPASS, localmacOSToriQA contractjourneys only. Independentnative/frontend/docreviewsPASS,97rowaudit retains131actual13/82 andnative713/103,42notation1/1pending. docs/m4/tori/shared-presentation-evidence.md contains finalactualdispositions/limits.132Done, so77nowdependency-readyReady/unassigned; no activation, currentplan or criterion/DoD checks applied. Originalfourcriteria await77ownfinalintegratedindependentreview.42/46/M4remainopen.
<!-- SECTION:NOTES:END -->
