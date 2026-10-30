---
id: MSH-119
title: 'R4: Deliver one frontend owner and shared application UI across all games'
status: Done
assignee:
  - '@codex'
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-30T22:03:00Z'
labels:
  - architecture-migration
  - planning
milestone: m-4
dependencies:
  - MSH-118
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-target-115.md
priority: medium
type: task
ordinal: 119000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved MSH-115 migration stage; shared integration ownership supports MSH-46 without changing its existing prerequisites or closure gates. Development-era internal compatibility is not required. Preserve native selected profiles, complete history/actions/adjudication, atomicity, exact work identity, safe cancellation, current-format losslessness, admission/headroom and external interoperability. Planning creates future intent only; inspect current source and record a bounded plan on activation. One integrator owns overlapping cutovers; stop at this ticket after normal verification, independent review, integration and push.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Exactly one production frontend session/subscription/operation/document-close owner serves all games. Migrate all product, host, harness, fixture and example callers; delete useNativeSession, obsolete App orchestration/embedded Chess Snapshot/nested old command DTO, duplicate guards and Chess desktop_snapshot/command/save/save_as/open endpoints and R3 forwarder.
- [x] #2 One application-level product composition and section implementation owns layout/hierarchy, dark responsive theme, document/lifecycle controls, player and live status, analysis, history, Game Actions, Technical Details, dialogs/notices/errors/recovery, visible language and shared focus/keyboard/accessibility conventions. Three parallel game page implementations under common chrome are not an acceptable endpoint.
- [x] #3 Native capability content composes into those same shared sections: Chess claims and Full controls, Shogi declaration advice/formal controls and hands, Xiangqi WXF warnings/count claims/pending state, and native-specific completion content have correct meanings with application-owned placement/presentation. Board topology/artwork/orientation and genuinely native capability renderers remain specialized; no universal renderer or frontend rules interpreter.
- [x] #4 All existing Chess functionality is represented in the current tagged contract and shared UI: intended/current claims without playing an intended move, bounded read-only moves and explicit Full proof/Unknown/cancel/failure, native history/observation provenance, unresolved resignation, real telemetry/perspective, settings, retry/Resume and Take back. Empty shared Chess analysis projections cannot count as migration.
- [x] #5 Exact rendered snapshot authority, stale/dirty Save receipts, replaced documents, native quit/close guards, historical selection/Return to live and stopped reopen remain protected. Common controls and section order stay stable across games; promotion/drop/compound completion stays native supplied.
- [x] #6 R4 owns the complete application UI and common non-board focus/accessibility lifecycle; R5 owns the board selection/inspection/route controller and board focus transitions, consuming R4 slots/contracts. Any temporarily retained board-state copy is explicitly retired in R5; there is no unassigned UI ownership gap or separate UI-refactor ticket.
- [x] #7 Behavioral tests port from old hooks/Document/App/Analysis/History/Recovery instead of retaining old interfaces. Same-size normal and supported-minimum captures of all three actual packaged games, pointer/keyboard, native quit/save/reopen, analysis/claims/recovery and independent visual/source review demonstrate one application; record exact package/revision/platform and limitations.
- [x] #8 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
- [x] #9 Shared errors/notices/recovery consume R3 authoritative typed capacity/resource facts, not diagnostic substrings or numeric-limit guesses. Preserve truthful generic unknown/multiple-cause feedback, Technical Details, actual ordinary/exact counts, independent file-failure messaging and Save/Save As availability; native control/observation/byte/event/replay payload regressions cover all applicable games.
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
1. Replace actual production routing with one useGameSession owner and ProductApp composition; use the current tagged native snapshot/operations, preserving exact rendered authority, file receipts, dirty and native-close guards and non-board focus. 2. Migrate native Chess-specific presentation and operations away from embedded legacy Snapshot/DesktopCommand; delete desktop_* forwarding handlers/events, legacy frontend owner/pages/DTOs and migrate real fixture/example/host/consumer tests. 3. Compose one document/header/heading/player/status/analysis/history/actions/technical/dialog/recovery implementation with small native capability content and existing specialized boards/hands/artwork; board selection/controller copies stay explicitly with MSH-120. Consume typed capacity facts without diagnostic inference and preserve all native Chess claims/Full, Shogi declaration and Xiangqi event/proof behavior. 4. Port meaningful semantic/race/focus tests, verify actual normal/minimum packaged all-three-game journeys and visual consistency, obtain separate independent review, run focused and applicable CI, then integrate/push only MSH-119 with exact evidence.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup 2026-10-30: root Codex owns MSH-119 in primary codex/rewrite at82a8ae8b0b7c023306f960b0d48e36b33dc036b0. Clean primary checkout, remote fetched aligned; unrelated existing worktrees retained; no In Progress worker. PATH Backlog1.55.0 mismatch resolved using pinned1.55.1 after overview/execution guides. Read request/design A09/A10 D17/D18, migration plan and task/dependency118Done. Source confirms ProductApp mounts useGameSession then Chess App mounts useNativeSession; Shogi/Xq each assemble separate shell sections. Root owns single frontend/composition; bounded native current-contract/host/fixture builder and shared-panel builder have disjoint files; fresh read-only independent reviewer owns no implementation. Existing Board/ShogiBoard/XiangqiBoard interaction state remains R5; no universal renderer or rules change.

R4 cutover implemented in primary working tree: one ProductApp/useGameSession/ApplicationView and common player/footer/panels/capability slots. Current Chess native details and direct operations replace embedded Snapshot and nested DesktopCommand; obsolete endpoints, old close event, QA aliases and actual consumers/fixtures migrated. Native168 + current runtime8 + actual subprocess12 + host9 PASS; native/console/host Clippy and formatting PASS (20 unchanged Tao warnings). Frontend strict tsc and223 tests PASS at current working source; initial migration failures corrected (piece coordinates/side art, direct preference shape, current typed-resource tests and native completion labels). Independent source review and actual normal/min packaged GUI evidence pending; R5 controllers unchanged.

Implementation ready for bounded independent source/visual review; current frontend223/223 and strict tsc PASS. GUI verifier55/55 PASS. Current normal macOS package built and package-smoke passes (host19a33348… engine57d09b1a…); actual native AX play/reply/Take back/Resume/Save As accepted. Full current-source captures, lifecycle checks and CI remain pending; no criteria checked yet.

Independent fresh read-only completed diff + six-image comparison review found two material Chess omissions: native in-check heading and Finish analysis on a concluded paused game. Both corrected using existing facts/common resume; added semantic regressions. Reviewer independently40 affected frontend PASS, earlier107 frontend+8 native PASS; current full225 and strict tsc PASS. No material visual finding at1180x820/760x650, all3 completeboards. Living contract/quit docs updated; historical evidence preserved. Source2918217 already pushed; final correction CI/package verification pending.

Initial291 Desktop CI41128859968: macOSbuild/smoke PASS; Windows15/15 actualGUI PASS; Linux11QA phasesPASS but normalChess/Shogi/Xq failed newTakeback input (elementnotinteractable; clippedsidebarcontrol), then dependentShogiOpen failed absent prior save. Independent reviewer identified actualtrace/geometry; bounded externaldriver keyboard-visibility correction in progress. Failure preserved under docs/review/evidence/msh119/initial-linux; no claim LinuxnormaljourneysPASS. Productsource0487945 fixesreviewfindings;225frontendPASS, finalnative CI/package evidence pending. Actual048 macconcludedFinishanalysis preservesCheckmate and nativeQuit exits0; exactreport under macos/reviewed-correction.json.

Independent reviewer approved bounded external-driver Takeback visibility correction: readonlynativeDOM facts + realboundedTabfocus, oneclick, noDOMmutation/IPC/actionreplay;62 driverchecksPASS (independent49PASS). Production unchanged from0487945. Corrected actualGUIcampaign and finalCI stillrequired. Exact291Windows15PASS audit and initialLinux11PASS/4FAIL retained; final048macFinishanalysis/regression capture verified independently. No unresolved material product/source/visual finding; task remainsInReview.

Native CI41128859801 all3OS passed behavior suites but failed generated snapshotdiff: newly exposedchess_match includes real request.at_ns timestamps in four fixtures. Independent reviewer identified generator determinism issue; nativebuilder owns fixture-only deterministic-time correction with repeated writer comparison. Production matchclock/history must remain unchanged. Corrected GUI verifier commitb3c2b57 pushed; finalCI remains pending, no readinesscriterion weakened.

Fixture-only correction reviewed independently: normalize copied UntimedMatchRecord output via authoritative native replay at event-index timestamps, assert every reply and nativehistory/conclusion/active/clock equality; timedrecordsreject.2exampletestsPASS independentlyrepeated; Clippy/fmtPASS; two completegenerationsall32JSONbyte-identical (evidenceindexretained). No productclock/history/archive change. Ready finalnormalCI;119remainsInReview until actualevidencegreen.

Finaleb nativeLinuxCI41131548408 hit existingMSH-112 early-exit classification race: same runner.rs172 MissingResponse/Brokenpipe32/exitNone vsUnexpectedExit; runner/engine sources unchanged. Retained failure, linkedrecurrence into112withoutstartingit; focusedunchangedmacOS test1PASS0.59s doesnotproveLinuxfix. NormalCIretry requested butGitHubrequiresoverallworkflowcomplete first. OtherfinalCI/GUIcontinues; noassertionweakening, timeoutincrease orproductrunnerchange.

Final independent PASS at eb027be: all-three native/Desktop CI green; final native audit records168default/180allfeatures eachOS and actual job/logSHA. Linux attempt2 passes unchanged,112recurrence preserved. FinalLinux/Windows15phaseGUI each audited; localmac exactpackages/captures and Full provenance separated. Finallinks/JSON/doctor/diff checks pass. All9AC/4DoD supported by review matrix; no unresolved material119finding. Completion bookkeeping only; next120unstarted.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed R4 on codex/rewrite. Production0487945b628314eed6d1be49d396dce41a362332; verification integrationeb027be7cec0f3206094c477a31f0cd9b9f73f6f already pushed. One ProductApp/useGameSession/ApplicationView owner and shared GameFrame/analysis/history/recovery/capability slots serve Chess/Shogi/Xiangqi. Deleted parallel product pages, legacy Chess public frontend/native DTOs and actual host forwarding endpoints/consumers; current direct native operations preserve full native semantics and lifecycle safety. Typed native resource facts replace all diagnostic guessing without changing limits or archive format. Complete surviving/deletion/functionality matrix and AC1-9 evidence in docs/review/frontend-application-119.md.

Independent fresh read-only source/test/visual/artifact review PASS; native Chess in-check and concluded Finish analysis findings resolved; verification/provenance corrections reviewed. Final225 frontend/strict TypeScript,168 default native,180 all-feature native EACH CI OS,8 local runtime,12 actual subprocess,9 host,2 fixture writer and62 GUI-driver checks PASS; lint/fmt PASS (20 unchanged Tao warnings). Rust41131548408 and Desktop41131548392 all3OS PASS at exacteb. Linux native attempt2 passes unchanged after existingMSH-112 attempt1 race; failure retained and112notfixed/unstarted. Initial fixture-diff and Linux clipped-control GUI failures plus superseded canceled runs preserved with bounded verification-only corrections.

Actual Linux and Windows installed normal/separate QA15/15 phases each PASS including native exits, checksum and normal instrumentation exclusion. Six same-size normal/min Linux captures plus six actual mac QA captures independently inspected;1180x820/760x650 complete boards/common shell. Actual local mac normal play/reply/switch/analysis/history/Takeback/Resume/SaveOpen/dirty/nativeclose and separate finalQA Checkmate Finishanalysis/Quit recorded with exact291/048 package identities; FullGUIUnknown is localmac, not claimed Linux/WindowsFullGUI. CI mac is build/smoke only. Evidence/audits under docs/review/evidence/msh119. Final docs/task/evidence commit carries no production changes.

R5/MSH-120 owns retained board selection/inspection/route/focus copies and is next dependency-ready; unstarted. R6/MSH-121 remains separate/unstarted. Native profiles/rules/search/execution/current archive/resource limits, R1-R3/M2/M3 evidence and M4 scope preserved. No R4 temporary worktree; unrelated worktrees retained. Stop after119.
<!-- SECTION:FINAL_SUMMARY:END -->
