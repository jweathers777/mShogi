---
id: MSH-132
title: Complete Tori shared presentation and real-consumer contract verification
status: Done
assignee:
  - '@Codex'
created_date: '2026-11-01T21:28:00Z'
updated_date: '2026-11-02T10:45:00Z'
labels:
  - m4
  - owner-msh-77
milestone: m-4
dependencies:
  - MSH-131
references:
  - >-
    backlog/tasks/msh-77 -
    Extend-shared-contracts-for-real-tori-shogi-consumers.md
documentation:
  - docs/m4/tori/shared-plan.md
  - docs/m4/tori/shared-consumer-ownership.json
  - docs/m4/tori/native-acceptance.md
  - docs/testing/conformance-policy.md
  - docs/m4/tori/desktop-runtime.md
  - docs/m4/tori/desktop-runtime-evidence.md
  - docs/m4/tori/shared-presentation.md
  - docs/m4/tori/shared-presentation-evidence.md
type: task
ordinal: 132000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Complete semantic presentation and exercise the integrated Tori session/document through the existing frontend owner and common board interaction contract. Actual consumers: shared.rs presentation structs, shared_inspection.rs, moves.rs projection; desktop/src/product/native.ts/useGameSession.ts/ProductApp.tsx/ApplicationView.tsx/GameFrame.tsx, NativeBoard.tsx, boardPresentation.ts and interaction/useNativeChoiceController.ts. Runtime predecessor supplies a minimally valid snapshot; this task completes its semantic coverage and shared frontend verification. Final Tori-specific notation, board artwork and human-engine product journeys stay MSH-42; any fixture surface must be clearly verification-only, excluded from normal packages.

Component of MSH-77 shared integration under MSH-46, not complete Tori delivery MSH-42. Accepted source994521bec0637d6f8f17103f1106c0025d950753/profile mshogi.tori.modern.v1, native acceptance a3677e1e862fdf3877c784f12f56a99c4bfbf167. Original103 independently expected fixtures and713 native assertions remain fixed; external engines are optional nonauthoritative investigations, never gates. See shared-plan.md and shared-consumer-ownership.json for exact consumer obligations. No rules, search/evaluation, console/notation, product Tori artwork/page, selector entry, other variant or M5 implementation. Future worker records its current implementation plan only after activation.

Verification: focused real native/candidate/controller tests appropriate to this boundary, exact-source commands/outcomes, independent completed-diff review, relevant existing-game regressions and applicable native/desktop CI for actual changed production. Distinguish source/controller/frontend/package/GUI evidence; no fake Tori engine or product-capability claim.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Native metadata provides7x7 square coordinates/orientations, Sente/Gote profile-qualified identities, all9faces with forced promotion state, LeftQuail/RightQuail distinction before/after capture/drop and native hand counts/origins. Complete choices/effects derive from actual Game permission, inspection from immutable native local queries. No optional promotion/CSA declaration, frontend legality or imposed shared native board model.
- [x] #2 Extend strict frontend Tori snapshot/side/settings/operation and resource scope types only as required by actual native payloads. One useGameSession, ApplicationView/shared shell and useNativeChoiceController remain owners. Contract tests use real native-generated payloads, click/keyboard/drop/forced completion/cancellation, distinct non-actionable opponent board/hand inspection and delayed replies after selection/game changes. Native full token guards and duplicate submission protection remain intact; no second Tori application page.
- [x] #3 Exact selected prefix/context/provenance, board-ply versus same-ply native event, current/selected conclusion and incomplete administrative status are presented honestly. Native event/move projection supplies actual board entries and origin/destination/drop cues; Stop/resignation do not become move rows. Native action identifiers may be labeled technical in this bounded contract, not passed off as final game notation; C-NOTATION remains MSH-42. No engine score/analysis progress is shown without a real observation.
- [x] #4 Every MSH-77 supplemental consumer row has final integration evidence from the prior slices and appropriate semantic/frontend checks;713 original native assertions and all103 immutable expectations remain supported, not rewritten. Explicitly retain the one pending assertion and one consumer requirement of C-NOTATION under MSH-42. No pending native/consumer requirement may be silently passed; material within-scope findings block coordinating77closure.
- [x] #5 Focused frontend/native snapshot, resource presentation and current existing-game shared owner/board/inspection/history/Moves/Take back/Save/Load/lifecycle regressions pass, with source-tagged native and frontend evidence and independent review. Use separately labeled verification instrumentation only when needed, reuse stable QA identity; no broad three-platform GUI or product acceptance claim from contract tests. Normal distributables exclude fixtures.
- [x] #6 Tori execution capabilities remain unsupported/unadvertised and playable-game selector unchanged. Durable handoff reconciles all original77criteria, actual limitations and required42engine/console/notation/product/archive/complete-game evidence plus46/88/89all-game acceptance. Coordinating77 performs final integrated review/integration after all slices; this component alone cannot close77,42orM4. Normal product Open/New Game remains explicitly gated until42supplies the actual product renderer/playable consumer; contract/verification access uses this same native controller, never a second implementation.
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
1. Root Codex is sole builder/integrator in clean aligned primary codex/rewrite c89802e; preserve unrelated worktrees. Use pinned Backlog1.55.1 instead of global1.55.0. Separate read-only tori131_review performs native preflight and independent completed-boundary review. Preserve native profile/corpus/713assertions and prior13113fields/82requirements.
2. Complete native Tori presentation in existing shared_tori/shared presentation and moves projection/cache: nine readable native face identities, handed quails/hands, true forced result changes only, exact Play-only rows and native completed geometry. Add minimal tagged Tori selected/live/root/obligation/conclusion/incomplete metadata from actual Game getters; no rendering replay, new archive/limits or rules changes.
3. Extend strict frontend DTOs/operations/resource scopes and existing GameBoard/NativeBoard/ApplicationView/shared panels only where real Tori payloads require. Keep one useGameSession/choice controller/shell, native read-only inspection and exact token freshness. Clearly label technical move identifiers and unsupported engine/analysis; never expose Tori in normal playable chooser/Open/NewGame.
4. Generate deterministic real Tori DesktopController fixtures for standard/custom roots, both orientations/allfaces/quails/hands, capture/drop/forced promotion, prefixes/repetition/Stop/resign/Takeback/SaveOpen/observations/resource refusal. Test native mapping and shared frontend pointer/keyboard/drop/completion/Esc/focus/inspection delays/history/cues, capacity and unsupported observations. Audit every supplemental consumer row without double counting, preserve42notation1/1.
5. Use existing verification-only host/fixture bootstrap and stable QA identity if actual packaged Tori semantic interaction is needed; no Tori page/artwork/pretendengine. Normal distributions exclude fixture data/activation. Reuse131 native/persistence/process evidence as unaffected; review deadline UI as typed operational refusal, no new timing promise or architecture redesign.
6. Focused Rust/controller/frontend/type checks, both workspace and standalone Tauri format/lint, applicable exact-source native/desktopCI and existing-game regressions; independent semantic/interaction/resource/ownership review, resolve findings, record actual package/GUI scope and failures. Complete132only with sixAC/fourDoD supported, normal commit/integration/push, durable77handoff; stop before77/42/otherwork.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH131 implementation handoff only, not132activation: reviewed source253e4c949614d133b5295f6a362e3d5279adcef3 supplies actual ActiveDocumentTori in sole DesktopController/RuntimeMetadata/install_document, immutable complete admission, existing FileWorker Save/Open/receipts, native exactprefix/inspection/Undo/Takeback and retained lineage. Runtime actual82requirements/13fields execute separately from unchanged713native adapter; C-NOTATION1/1remains42. Minimal native SharedSnapshot is real but Moves/complete frontend metadata remain132. Use native owner and exact event/history/selected target, no second session. Normal productNewGame/Open gated, zero advertised engine capabilities; only verification supervised request returns real unscoredUnsupportedMode. Runtime-only browse reopenslive; saved observations preserveexactprefix. Exact-source131CI/finaldisposition in desktop-runtime-evidence.md.132remainsReady/unassigned/unstarted, all originalcriteria/dependencies unchanged.

Final131handoff: reviewed source6a2edd71b3136a9c2043f9137eb6d3fb8abf120d (productionce48e84) integrated/pushed; nativeCI41417589059 andDesktop41417589127 SUCCESS all3.247sessiontests inworkspace+allfeaturesperplatform; actualToriunsupported/failure/cancel/replacementprocesschecksPASS, existinggameLinux/WindowsinstalledGUIregressionsPASS, macpackage/hostonly(no newGUIcampaign). Actual13fields/82requirements established bycontroller; immutable713nativeassertions preserved, notation1/1remains42. docs/m4/tori/desktop-runtime.md anddesktop-runtime-evidence.md are current API/evidence handoff.132now nextdependency-readypending131finalization, stillReady/unassigned/unstarted; no plan/productactivation/criterion/dependencychange.

Started132only at clean c89802e; origin ls-remote matches,131Done dependency satisfied; no active competing task. Current source gap: emptyToriMoves, technicalFaceDebug labels andeverymoveResultcompletion, noToriTSdispatch. Root owns all edits/integration; reviewer read-only. No normalproductTori activation or nativeprofile/rules/corpus/limit changes.

Implementation boundary complete. Sole-builder root changes native Tori metadata/Play-only projection and existing shared frontend; generated18actualDesktopController DTOs, fixed normal admission gate retained. Independent native reviewer6presentationchecksPASS and exact13/82auditPASS; independent frontend27files342checks+Node24TS PASS. Resolved validation-budget Save promise, unsupported thinking UI and native board capability override. Working primary gained unrelated docs-only0f8b390 (forensic timing report); preserved unchanged. Packaged stableQA build succeeds; actualGUI verification underway, source CI and completion evidence pending. Initial helper compile errors, test type/import/copy/mock failures corrected; native fixture expectations unchanged.

Reviewed production52b0390cac09faf727a0c5ed74c1fd673f468a11 integrated/pushed. Actual macOS stableQA1180x820 CUA: forcedcapture/handtransfer/drop, nativeopponenthand+historicalinspection/Escfocus, Takeback/history/Returnlive/Goteflip, actualSaveLoad8287Bwithretainedcapturebranch, only3playablechooserentries, Chessrealengine642nodes, dirtyQuitCancel/Discard. No newidentity/prompt. Normalpackagebuild/smoke/exclusion and11stablehelperchecksPASS; no normalGUIlaunch or ToriWindows/Linuxcampaign claimed. Sixindependentnative+342independentfrontend+TSreviewPASS; whole-session252checkpointPASS plus6focusedcurrent/253discovered, native38PASS/713assertions. Applicable52b0390 nativeCI41427941953/Desktop41427942000 underway. docs/m4/tori/shared-presentation*.md andcomplete97-rowaudit handoff preserve42notation1/1 and replaydeadlinequalification. Await actualCI/finaldocsreview, then closure132only.

Final CI reconciliation: exact-source52b0390 native41427941953/Desktop41427942000 completed SUCCESS, all6platformjobs. Current source unchanged after tested revision. All6AC/all4DoD supported by independent reviews/tests/actualQA/package/CI and durable97rowhandoff. Final completion commit contains task/docs/evidence only; production integrated/pushed before closure. Stop132only.

Final independent CI/documentation reconciliation by tori77_boundary_review PASS: bothindices exact52b0390, all6jobsSUCCESS, actualLinux/Windows existing3gameGUI versus macOSpackage/host/localToriQA scopes accurately separated; bindingT06description/nodecommandpaths corrected. No productiondiff after testedsource, no material findings.132closure supported,77Ready/unstarted. Doctor/dependencyready/diff and currentrefs checksPASS.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed shared semantic presentation for real mshogi.tori.modern.v1 through the existing DesktopController and shared frontend. Native metadata preserves7x7/both orientations, all9faces/promoted states, handed quails/hands, complete native actions/forced promotion, selected/live exact event/history/provenance and typed conclusions/incomplete/resource facts. Native Play-only Moves and complete last-action geometry remain explicitly technical; final notation1field/1requirement stays MSH-42.

One ProductApp/useGameSession/ApplicationView/shared shell/choice controller remains. Real native-generated18DTOs cover pointer/keyboard/drop/completion/cancellation, read-only board/hand inspection/freshness, history/Take back/Return, resource and actual UnsupportedMode behavior. Zero Tori engine capabilities, no fake score/thinking/progress. Normal NewGame/Load remain gated and chooser onlyChess/Shogi/Xiangqi; separately labeled QA uses same runtime/shell and existing stable app identity.

Tested/integrated production52b0390cac09faf727a0c5ed74c1fd673f468a11, normally pushed to codex/rewrite. Native CI41427941953 and Desktop CI41427942000 SUCCESS/all3platforms each.253sessionchecks perplatform (workspace+isolatedfeatures), native38/713assertions, frontend342+TS, host14features, Clippy/format/fixturefreshness/normalpackages/bundledengine passed. Independent read-only native6checks and frontendfull342/TS reviews PASS; material findings resolved. Complete97-row audit reuses131actual13fields/82requirements, without duplicate counting or changing103native fixtures.

Actual macOS27.0.1arm64 stableQA CUA verified forcedcapture/handtransfer/drop, Esc/focus/opponent+historicalinspection, Takeback/Return/flip, nativeSaveLoad8287B withretainedcapturebranch, supported3gamechooser, Chessrealengine reply and guarded nativeQuit. Normal packageexcludes fixturedata/bootstrap/nativeactivation. Linux/Windows CI actualexisting-gameGUI passed; not a Tori three-platform/product acceptance claim.

Unchanged15s replay deadline can refuse bounded operations on slower/contended hosts. Corrected shared ValidationBudget recovery preserves accepted state and does not guarantee Save completion or imply reopening resets structural capacity. Actual131measurements reused; new elapsed-classifier DTOs are clock injection, not timing proof. MSH77 final independent acceptance remains Ready/unstarted, alloriginalAC/DoDunchecked; MSH42 owns notation/console/search/artwork/playable delivery;46/88/89 own broader all-game acceptance. No other task started. Contract/evidence/CI/package/audit handoff: docs/m4/tori/shared-presentation.md and shared-presentation-evidence.md.
<!-- SECTION:FINAL_SUMMARY:END -->
