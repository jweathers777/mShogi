---
id: MSH-131
title: Integrate Tori into the single native desktop runtime and lifecycle
status: In Review
assignee:
  - '@Codex'
created_date: '2026-11-01T21:28:00Z'
updated_date: '2026-11-02T09:03:00Z'
labels:
  - m4
  - owner-msh-77
milestone: m-4
dependencies:
  - MSH-130
references:
  - >-
    backlog/tasks/msh-77 -
    Extend-shared-contracts-for-real-tori-shogi-consumers.md
documentation:
  - docs/m4/tori/shared-plan.md
  - docs/m4/tori/shared-consumer-ownership.json
  - docs/m4/tori/native-acceptance.md
  - docs/testing/conformance-policy.md
  - docs/m4/tori/desktop-document.md
  - docs/m4/tori/desktop-document-evidence.md
  - docs/m4/tori/desktop-runtime.md
  - docs/m4/tori/desktop-runtime-evidence.md
type: task
ordinal: 131000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Consume the completed Tori codec/admission slice before accepting live Tori work. Actual surviving owners are lib.rs DesktopController/ActiveDocument/RuntimeMetadata, shared.rs SharedDesktopCommand/SessionOperation/SessionToken/SharedSnapshot, shared_inspection.rs, persistence.rs sole install_document, lifecycle.rs, takeback.rs and shared_archive.rs worker/receipts. Extend their tagged native adapters only. Use native::Tori identities, tori::Game/GameRequest/GameCommand/GameContext/GameUndo and native prefix/inspection APIs; Shogi Play is concretely shogi::Action and cannot admit Tori. MSH-42 later supplies the real engine; this slice must be verifiable against actual native games before it exists.

Component of MSH-77 shared integration under MSH-46, not complete Tori delivery MSH-42. Accepted source994521bec0637d6f8f17103f1106c0025d950753/profile mshogi.tori.modern.v1, native acceptance a3677e1e862fdf3877c784f12f56a99c4bfbf167. Original103 independently expected fixtures and713 native assertions remain fixed; external engines are optional nonauthoritative investigations, never gates. See shared-plan.md and shared-consumer-ownership.json for exact consumer obligations. No rules, search/evaluation, console/notation, product Tori artwork/page, selector entry, other variant or M5 implementation. Future worker records its current implementation plan only after activation.

Verification: focused real native/candidate/controller tests appropriate to this boundary, exact-source commands/outcomes, independent completed-diff review, relevant existing-game regressions and applicable native/desktop CI for actual changed production. Distinguish source/controller/frontend/package/GUI evidence; no fake Tori engine or product-capability claim.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Exactly one ActiveDocument::Tori variant holds authoritative tori::Game state in the existing DesktopController and RuntimeMetadata. Explicit native Tori operation/side/settings dispatch constructs context-checked GameRequest at the serialized admission boundary; complete moves/drops, out-of-turn resignation, Stop and unsupported agreement/impasse preserve native semantics. All mutations/observations/preferences/undo/takeback/close operations use the prerequisite complete document admission from their first accepted state.
- [ ] #2 Actual controller tests execute the82 MSH-77 consumer requirements and13 MSH-77 pending fields from the supplemental mapping, with exact independently expected fixtures and owner rotations. Rejected board/root/Open/stale/unsupported/resource requests preserve the complete accepted native document/settings/observations and cannot install a new document or grant stale execution authority. Diagnostic/progress revisions and explicit cancellation retirement may change as specified. Resource refusal is not illegality or an invented game result; the saved complete prior state remains available.
- [ ] #3 Native read-only board/hand inspection uses selected prefix LocalPosition::inspect; complete live choices use Game::legal_actions and Game::submit. Exact event-index selection, Return to live, Undo and atomic Take back restore correct root/history/obligation/conclusion and native-derived geometry while retiring execution permanently. No future moves or repetition information leak into a prefix; same-board/different-history and return-to-same-diagram never revive old work.
- [ ] #4 Bind the surviving ExecutionContext/Target/Observation family to full native record/purpose/request/session/process-generation/limits and selected event. Actual existing internal subprocess UnsupportedMode and bounded termination/cancellation/replacement tests prove unscored handling, distinct administrative reasons and cross-game stale rejection. All six identity dimensions are negatively tested; no Tori success/result family, search, telemetry, draw or fallback move is invented. engine_turn/move_analysis/Chess Full/Xiangqi exact capabilities remain false; unsupported Analyze/Resume/Retry execution is explicit.
- [ ] #5 Install validated Tori candidates through sole persistence::install_document and the existing bounded worker, receipts and lifecycle guards. Failed/corrupt/cancelled/resource-limited Open/Replace leaves live state and execution intact; successful reopen swaps atomically with fresh runtime identities, historical observations only and work stopped. Unsaved replacement/quit and native close reservations remain protected; Resume cannot advertise or launch nonexistent Tori work.
- [ ] #6 A minimal correct SharedSnapshot accompanies the new runtime for real boundary tests; no fake Shogi placeholder or frontend rules. Verify actual current file Save/Load, interrupted-save dirty state, historical prefixes, takeback/save/reopen,256-to257 refusal, reserved close and stale installation races. Focused current Chess/Shogi/Xiangqi controller/document/lifecycle/actual subprocess regressions pass; independent runtime/admission review and applicable CI are recorded. No Tori supported-game selector entry or final product board. Native controller Save/Open is exercised through the real contract/verification path; normal host New Game/Open must explicitly refuse Tori before a real product renderer is available under42, rather than send an unsupported snapshot to a Chess/Shogi fallback.
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
1. Root Codex is sole builder/integrator in clean aligned primary codex/rewrite086579f; independent read-only tori131_review. Preserve unrelated worktrees and immutable native profile/corpus713 assertions. Use pinned Backlog1.55.1.
2. Add one ActiveDocument::Tori state arm holding actual native Game plus accepted immutable MSH130 document, selection and execution metadata; native typed Tori commands/settings, LocalPosition inspection and minimal truthful SharedSnapshot. Preserve normal host product gates and no engine capabilities.
3. Extend existing NativeWork admission and sole persistence::install_document. Perform complete codec candidate validation/native reconstruction off-path before swapping; all commands/settings/observations/rewinds/close admitted before acceptance, exact tokens and fresh monotonic retired identities. Reuse existing FileWorker Save/Open receipts and atomic files, with explicit native verification admission distinct from normal product activation.
4. Execute actual controller fixtures for all82 runtime consumer requirements/13fields, exact history/inspection/Undo/Take back and same-board retirement, actual unsupported internal child/cancellation/failure; strict rejected replacement and capacity including256to257/reserved endings, Save/reopen/stale receipts/dirty close. Preserve42 notation and132 full presentation ownership.
5. Independent completed diff/runtime/security/admission/stale-result review, resolve findings, focused old-game controller/worker/process regressions, format/lint/standalone Tauri and applicable exact-source native/desktopCI; record honest revisions/results and132handoff. Finalize131only when all6AC/4DoD supported, integrate/push normally then stop.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH130 implementation handoff (not131activation): immutable ValidatedToriDocument/currentenvelope1, ToriControl/Settings, exactnative initial/current/retiredbranches/observations/floor getters, pureCandidateWork inexistingNativeWork/FileWorker and exactSavedreceipt. ExistingpublicOpen/ActiveDocument/install_documentTori remainunsupported.131must useonecontroller/soleinstall withcompletecandidateadmission beforeacceptance, expensiveprep offUI, positivefreshstoppedruntimeIDs, nativeactorcontext/inspection, actualprefix/Takeback andSavedreceiptidentity+revisionchecks. WholeTori budget262144 differsfromnative65536 andothergames16000; actualusage/reservedStopResignClose and256to257fixtures documented. Historicalobservationsnorequest, Tori executionunsupported/unadvertised; no normalchooser/productrenderer. All13pendingruntimeassertions/82requirements remain131pending. This notechanges no131status/criteria/dependencies/implementationplan.

MSH-130 is Done, reviewed implementation integrated/pushed at8fd12f953beeacac46fdd417b2054bedeafe2d6e. Native CI41355796434 PASS all3; exact17codec checks and actual UnsupportedMode child→codec roundtrip passed perplatform. Relevant normal package/host/frontend checks pass all3; pending Windows existing-game GUI snapshot remains accurately recorded separately. Use documented immutable codec/candidate/worker API and measured reservations; no live Tori install or new engine yet.131now dependency-ready Ready/unassigned, not activated; all13/82runtime assertions remain pending.

Started131only at clean primary086579f; remote ls-remote matches.130Done, no other InProgress/InReview tasks. Global CLI mismatch avoided via pinned1.55.1. Source inspection confirms one ActiveDocument/RuntimeMetadata, shared NativeWork workers and sole install_document; explicit Tori branches required to avoid existing Chess fallback. No Tori GUI/engine/chooser implementation authorized.

Current runtime implemented under one ActiveDocumentTori and sole install_document, immutable MSH130 admission off-path beforeacceptedchanges. Ten focused controller testsPASS at firstcompletecheckpoint; actual nativechild+forcedfailure/cancel/replacement/reopen2PASS; native38PASS/713assertions unchanged. Independenttori131_review reproduced10/10 including360history. Arc-backed document snapshots and precomputed closecandidate address synchronousencodingfinding; Save floorrefresh movedexistingFileWorker. Typed identityheadroom finding fixed/tested; repeated stop-only closeguard preserved. Longcontroller imported360record313906B/196404units; StopResignClose314540B/196409; installation5432ms,Stop6872ms,Resign6871ms,Save4152ms,close+snapshot6ms,Saveenqueue0ms (observations,notguarantees). Whole-document262144 unchanged; extra runtime nativepreparation boundedseparately underexisting15sworker. Historical broad241run239PASS/2FAIL solelyold129/130tests expecting nativeToriunsupported; updatingthoseauthorizedcontractgates while preservingnormalproductunsupported, not alteringrulesfixtures. One clippyboolstylefix, initialtestcompile/helperfieldnames andstrictresourceprobe corrected. NoGUI/externalengine ornexttask.

Implementation ready for final independent acceptance and exact-source CI. Current production-source reviewPASS; reviewerindependent10controller checks incllong andidentity/interruptedSavePASS. Fullsessioncheckpoint242PASS/0FAIL/0ignored plusfinal12fastcontrollerPASS and2actualTori processPASS; native38/713PASS, console6targets15PASS, normalTauri8hosttestsPASS, TauriCheck/clippyPASS(20oldTao warnings). Added actualforced8/unequal16 lastaction throughcontroller and exacthistoricalfailureprefix Save/Open; allnative/fixtureinputs preserved. Runtimeonlybrowse reopenslive; observationtargets preserveexactprefix. Additional maximumdecimalfloorwidth reservation protectsSaveafterclosedoc; targetedboundarycheckpending. MSH132stillunstarted. No ToriGUIclaim.

Final completed-source independent review PASS: separate tori131_review independently reproduced controller long/admission checks, typed identity/interrupted Save, outgoing failed-replacement preservation and exact floor-width boundary; exact82 requirements/13 fields independently reconciled. Final frozen fast controller13PASS plus isolated long1PASS; floor1PASS; six actual subprocess targets15PASS after atomic publication of test-only child PID marker resolved an intermediate instrumentation race. Full session242PASS is a checkpoint before the last3 controller/floor tests; final source expected246 tests, exact-source CI pending. Final lint/format/diff PASS. Outgoing Tori replacement admission now preserves current pending work until successful commit; failure leaves complete accepted state intact. No normal Tori product or engine activation. Task remains In Review pending exact-source native/desktop CI and final integration evidence.

Initial source253e4c949614d133b5295f6a362e3d5279adcef3 integrated/pushed. NativeCI41415490915 running; desktopCI41415490841 macOS failed standalone Tauri cargo fmt at newly added host product gate before package construction. Root workspace fmt did not cover separate Tauri workspace. Applied standalone cargo fmt and verified both manifests locally; purely formatting, no behavior change. Preserve original failure and rerun exact-source CI after correction; no package/GUI pass claimed for initial failed run.

InitialUbuntu nativeCI41415490915:245PASS/1FAIL at long360Resignunwrap: existing authoritative15sValidationBudget expired underconcurrentdebugload. Production correctly refused withoutacceptance; invalid wallclocktestassumption corrected afterindependentreview: mandatory180completecontrollerStop/Resign/Close/Save/freshstoppedOpen plus360near-limitprobe, onlyexactValidationBudget/Application/Milliseconds limit15000 count>=15000 alternate refusal, completeacceptedstate/pending/dirty preserved andstrictretained/closeencoding verified, oldfilepreservedonSavefailure. Othererrorsstillfail. No productionlimit/rule/fixturechanges. Mandatory180localPASS10.73s, everyoperation<=2.1s;360rerun andindependentfinaldiffreviewpending. Finalsessionexpected247checks; initialfailure retained as evidence.

Final bounded test correction independently reviewed PASS, including stable open/sessiondocument-version-selection authority, phase andpurpose preservation (progressrevision allowed). Local14fastPASS including mandatory180actualSave/freshstoppedOpen,360completeactualSave/OpenPASS37.24s, lint/bothfmt/diffPASS. Test-only correction preserves selectedprofile/native713/independentfixtureexpectations, production15sdeadline andallcapacitylimits. Applicable exact-source CI rerun required; finalsuite247tests. No newtaskactivation.
<!-- SECTION:NOTES:END -->
