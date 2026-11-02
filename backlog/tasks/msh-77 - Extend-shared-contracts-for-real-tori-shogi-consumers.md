---
id: MSH-77
title: Extend shared contracts for real tori shogi consumers
status: Done
assignee:
  - '@Codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-02T11:38:00Z'
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
  - docs/m4/tori/shared-acceptance.md
  - docs/m4/tori/playable-delivery.md
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
- [x] #1 The real game-owned native handoff is consumed through explicit typed profile/record/session/engine dispatch and supported capabilities; existing chess/shogi contexts remain compatible and stale/cancelled results cannot cross profiles.
- [x] #2 Shared mechanisms accommodate 7x7 square-board/hands,piece promotion/ownership and native game-specific history/action/outcome payloads through native metadata and complete choices,with no frontend legality or mandatory common board representation.
- [x] #3 Canonical envelope/history/admission supports the concrete game payload and documented resource policy without losing prior accepted data; unsupported profiles/conversions remain explicit.
- [x] #4 Real authoritative game-generated fixtures and interaction tests establish the extension; synthetic M3 fixtures alone do not prove real-game support. Required cross-game regression and ownership boundaries are recorded.
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
Planning-only refinement at a3677e1e862fdf3877c784f12f56a99c4bfbf167; root Codex is sole builder/integrator in primary codex/rewrite, no other active task owner and unrelated worktrees preserved.
1. Inspect selected modern profile, completed85/126-129,77/42/46 and actual surviving R1-R6 core/execution/controller/document/frontend/choice consumers.
2. Define minimal independently verifiable implementation slices with concrete types, fixtures, resource and stale-result obligations. Sequence codec/admission before installable runtime if needed to avoid transient unsaveable accepted work. Preserve original77AC/DoD and42/46ownership; no engine, console, product board or chooser implementation.
3. Create only source-justified flat component tasks with complete dependencies and executable acceptance/verification expectations, no speculative child implementation plans. Exhaustively map14pending fields and83consumer requirements to slices plus retained42duties.
4. Obtain separate read-only review of current source grounding, boundaries and graph; resolve findings. Validate unchanged713assertion inputs/native source, original criteria, all fixture owners/links/dependencies, backlog doctor and diff.
5. Integrate reviewed planning normally and push. Leave77 open/blocked on components, first dependency-satisfied sliceReady/unassigned; stop before implementation.

1. Root Codex is sole integrator in clean primary codex/rewrite febf4d2, origin aligned; preserve unrelated worktrees. Use compatible pinned Backlog1.55.1 (global1.55.0 is read-only). All seven direct dependencies Done; no other active task. This is final acceptance/documentation only, not MSH42 implementation.
2. Independently reconcile original77AC1–4/DoD against actual native/runtime/document/shared frontend source and accepted130–132 evidence. Launch one fresh read-only integrated reviewer early; audit97supplementalrows,13runtimefields/82requirements/713nativeassertions/103immutableexpectations and retained42notation1/1. Assess mandatory180action success versus bounded360action precise refusal and state integrity; no replay/limit changes.
3. Required verification gate: reuse exact52b0390 Native41427941953 andDesktop41427942000 all3platform PASS, source identity and package/GUI evidence at its real scope; newly execute only dependency/immutable/source/evidence/link/doctor/diff audits unless a material gap requires focused testing. No new CI jobs or GUI campaign for unchanged production; no redundant monitor/wait. One structured77acceptance record references prior per-source results instead of copying them.
4. Inspect actual MSH42 search/execution/console/native presentation/document/desktop exposure boundaries and record an executable small-boundary next assignment, preserving its eightAC, final owner visual approval and broader46/47/88/89ownership. Independent acceptance/handoff review, resolve findings, close77only if supported, normal documentation-only integration/push, stop before42implementation. Preserve historical planning notes using append rather than replacing them.
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

Final integrated acceptance activated at febf4d2; all7direct dependenciesDone, current production identical to tested52b0390. Owner-approved within-assignment efficiency guidance applied: one acceptance record, early independent read-only review, reuse all required exact-source CI/platform evidence, no new broad tests/CI/GUI absent concrete gap. Root owns primary integration; unrelated worktrees preserved.

Integrated source/evidence acceptance review PASS AC1–4 by fresh independent read-only accept77_review: actual single native/runtime/worker/UI owners, lossless admission, exact history/identities, real fixtures and product gates verified. Newly executed static97row/hash/test-reference/source/dependency/CI-index auditsPASS. Existing52b0390 native41427941953/Desktop41427942000all3platform evidence reused; no new product tests/build/GUI/CI campaign. One structured acceptance index docs/m4/tori/evidence/shared-acceptance.json references existing per-source records. Mandatory180success/360typedelapsedrefusal reviewed; no new ordinary-workload blocker. Final docs/handoff/efficiency-policy review underway; no42implementation.

Fresh integrated acceptance and final documentation review by accept77_review PASS; finalsourceidentity/liveCIconfirmations and97rowauditPASS. All4AC/4DoD supported; no production/source/workflow/test change or newplatformcampaign. Completion record references structured evidence;42handoff/AGENTSownerpractices current. Ready for normal documentation-only integration/push; stop77only.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Accepted the integrated Tori shared application support without production changes. Reviewed integration febf4d2aa51c7e803a4f7d8d8345a72ac6f66086; production remains exact tested 52b0390cac09faf727a0c5ed74c1fd673f468a11. Original criteria:

AC1: PASS. Actual typed profile/record/native context and sole controller/install; full execution identity retirement, honest unsupported capabilities.
AC2: PASS. Native7x7/ninefaces/handedquails/hands/forced complete choices/history/outcomes; same frontend/session/choice owners, no frontend rules.
AC3: PASS. Current duplicate-preserving envelope, authoritative native+journal replay, complete precommit quotas/reservations, atomic SaveOpen/fresh stopped identities.
AC4: PASS. Real native/controller/frontend/packaged QA fixtures and exact-source cross-game CI;97rows reconciled with retained42notation1/1.

Verification reused: Native41427941953 andDesktop41427942000all6platformjobsPASS at the exactsource;253session/342frontend/14verificationhost checks,713native assertions and actualmacOSToriQA plus existing-gameWindows/LinuxGUI scopes retain their original provenance. New read-only source/dependency/hash/97row/expected-value/test-resolution/CIidentity/link/doctor/diff auditsPASS. No new test/build/GUI/CIcampaign or timed monitoring needed. Fresh independent accept77_review actual integrated-source/row/hash/test/deadline and finaldocumentation reviewPASS; no unresolvedmaterialfinding. One structured acceptance index references existing per-source records: docs/m4/tori/evidence/shared-acceptance.json; verdict docs/m4/tori/shared-acceptance.md.

15sdeadline accepted as documented operational limitation: mandatory180-action fulljourney succeeds;360near-limit canonlyrefuse via exacttyped elapsedfacts while preserving complete acceptedstate/dirty/priorfile. No new ordinary-workload blocker, no guaranteed near-limitSave timing, no limit/replay/rules/fixture change. NormalproductToriNewGame/Load still gated; engine/console/finalnotation/artwork/playablejourneys and owner visualapproval remainMSH42.13runtimefields/82requirements retained; notation1/1still42, not another passedsharedrequirement.

Updated MSH42handoff identifies real-result engine/archive/controller boundary plus console together, then desktop play and independent complete-game verification, preserving its eight originalcriteria; no new tasks/planningcampaign or42implementation. AGENTS now records the owner-approved within-assignment efficiency practices without weakening acceptance/platform gates. MSH42 remainsToDo/unassigned/unstarted, dependencies satisfied after77; M4/46/47/88/89stayopen. Root soleintegrator in primarycodex/rewrite, unrelatedworktrees/historypreserved; normal metadata-onlycommit/push, stopafter77.
<!-- SECTION:FINAL_SUMMARY:END -->
