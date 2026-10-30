---
id: MSH-118
title: >-
  R3: Establish native application runtime document ownership and the current
  host contract
status: Done
assignee:
  - '@codex'
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-30T18:44:00Z'
labels:
  - architecture-migration
  - planning
milestone: m-4
dependencies:
  - MSH-116
  - MSH-117
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-target-115.md
  - docs/desktop/native-application.md
  - docs/review/native-application-118.md
priority: medium
type: task
ordinal: 118000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved MSH-115 migration stage; shared integration ownership supports MSH-46 without changing its existing prerequisites or closure gates. Development-era internal compatibility is not required. Preserve native selected profiles, complete history/actions/adjudication, atomicity, exact work identity, safe cancellation, current-format losslessness, admission/headroom and external interoperability. Planning creates future intent only; inspect current source and record a bounded plan on activation. One integrator owns overlapping cutovers; stop at this ticket after normal verification, independent review, integration and push.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 One application controller owns explicit common runtime/document metadata and exactly one active native-tagged document. Replace the ever-present global Chess DesktopSession plus optional overlay ownership; game-native state, histories, settings, journals, observations and semantic command admission remain independent.
- [x] #2 One bounded admission/execution/file lifecycle handles typed native candidates, stable receipts, identity reservations and actual per-native replay/encoding costs off the serialized host lock. Commit only after complete current-format admission; failed/cancelled/stale work preserves accepted saveable state.
- [x] #3 One installation transaction checks authority/receipt, retires execution, allocates fresh document/session/request/process/selection identities and installs validated data stopped with saved observations historical. History selection, semantic undo, Take back and replacement never revive old work.
- [x] #4 The current host command/snapshot/file/application-close contract provides complete native Chess claim/preferences/ordinary and Full analysis behavior, Shogi capabilities and Xiangqi event/claim/proof context. Remove empty compatibility projections as a target; no frontend rules or common outcome reinterpretation.
- [x] #5 Delete the R2 new-codec-to-old-runtime adapter and replaced native metadata/install owners here. A temporary current-controller-to-old-frontend-shape/command forwarder may serve only the named in-repository caller until R4, with no second session or old-file support; R4 deletes its endpoints and DTO together.
- [x] #6 Native menu Quit/window-close and common request/cancel/finish close behavior migrate to neutral application ownership rather than being deleted by desktop_* name prefix. Preserve dirty guards, file/command reservations, Tao termination regression and normal instrumentation exclusion.
- [x] #7 Focused all-three-game admission/install/current-IPC/identity/Take back/cross-game replacement and failure tests plus actual packaged lifecycle/file/close interaction on an identified platform support the changed owner; distinguish final R6 three-platform evidence.
- [x] #8 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
- [x] #9 Native admission owns explicit bounded typed resource facts delivered through the current host/snapshot/error contract, with resource category and applicable actual count/limit while unknown/multiple causes remain expressible. Preserve bounded Technical Details and truthful generic fallback; capacity is distinct from illegal actions, game outcomes and engine failures. Focused native payload tests cover control-only zero observations, genuine and combined ordinary/exact observation accounting, 256-to257, bytes/events/replay and stop/close headroom, with Save/Save As preserved. R4 consumes these facts without diagnostic-string inference.
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
1. Split common native runtime/document metadata from game-owned state and introduce exactly one ActiveDocument Chess/Shogi/Xiangqi variant, removing dormant Chess and optional overlays. 2. Consolidate typed candidate admission, file completion and guarded installation using existing bounded workers; move native reconstruction/apply/encoding costs off the serialized host lock, preserve stable receipts, reservations and fresh stopped identity. 3. Complete current native command/snapshot/file/close contracts, especially Chess claims/preferences/Full/observations; replace string-derived resource classification with bounded native typed facts at actual limit checks. Delete R2 capture/install ownership; document the sole temporary current-controller-to-existing-frontend forwarder and its MSH-119 deletion endpoint. 4. Port semantic regressions and independently expected fixtures, add ownership/install/stale/capacity/host-close tests, exercise packaged file/lifecycle/close journeys on an identified platform, obtain separate independent review, run applicable CI and normally integrate/push only MSH-118.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup 2026-10-30: root Codex owns MSH-118 in primary codex/rewrite checkout at7414b795d47e9a6408f1b21f07cb1baacfddf8dc. Clean, fetched origin aligned0/0; no In Progress owner. Existing unrelated worktrees preserved. PATH Backlog1.55.0 incompatible; using pinned1.55.1 after overview/execution guides. Read owner request, design A-09/A-10, task/dependencies116/117Done and approved migration. Actual source: DesktopController retains DesktopSession plus optional Shogi/Xiangqi overlays; common metadata lives in Chess; three install paths; shared Chess observation placeholders. Read-only boundary audit delegated; root is primary builder/integrator. No R4/R5 work or new compatibility requirement.

Implementation checkpoint: common metadata extracted to RuntimeMetadata and optional native overlays replaced by a single ActiveDocument. Core worker owns native files; root owns host/lifecycle and integration; isolated resource types and external GUI-verifier work do not overlap production ownership. Native lifecycle now owns command/file RAII reservations and exact-token close approval; host retains technical cleanup only. Root host8 and frontend17 focused checks plus TypeScript pass at working source. Read-only reviewer independently reran17 frontend checks and found post-picker Open/close race; root added under-lock recheck and focused regression. Initial core run149pass/7fail exposed dormant-Chess assumptions and two actual replacement/poll cutover issues; fixes and rerun pending. All AC/DoD unchecked; no Done/CI/package claim yet. No R4/R5 started.

Reviewed source checkpoint: one ActiveDocument and RuntimeMetadata, worker-native admission/install, controller-owned lifecycle and typed resource facts implemented. Independent read-only r3_boundary_audit approved corrected source; findings and original local logs indexed in docs/review/native-application-118.md. Native default165 before final corrections plus final runtime6/budget2/control1/byte1/replay1, host9, console actual subprocess12, frontend243, GUI verifier50, TypeScript, workspace/native/host Clippy and fmt pass. Post-deadline committed Save stays successful; live revisions monotonic across Open; post-picker close race rejected. Preparing reviewed source commit for applicable CI and actual packaged lifecycle verification; all criteria remain unchecked pending that evidence. R4 frontend forwarder explicitly expires in MSH-119; no later task started.

Committed source b568ea7103e283155a851df16d3b382b3842eebc pushed. Independent reviewer confirmed exact source and16 local log hashes. Normal local macOS27.0.1 arm64 .app built; actual Chess/Xiangqi play, selected-history analysis, native Save/Open stopped, Return to live/Resume, native window-close Cancel and Cmd-Q Save & continue passed; host/engine absence confirmed.17 retained artifacts/package hashes independently audited in docs/review/evidence/msh118/macos. Final local pinned all-feature run177pass/1fail: Full-byte test expected legacy diagnostic spacing16 MiB versus current16MiB; actual typed byte facts now replace that brittle string assertion, retaining real overflow and complete Save/Open checks. Corrected stress rerun/review pending. CI41103862958/41103863940 running; intermittent jobs-list502 and empty step lists accurately remain observation limitations, not successes.

Initial CI terminal: Rust41103862958 default166passes onall3OS, all-feature177/1fails the same obsolete16 MiB substring assertion. Corrected semantic resource-fact/full-byte test passed162.70s includingSave/Open; independent reviewer approved strongerassertions. Desktop41103863940 normalmacpackagepasses; Linux/Windows13/15actualphasespass. ActualretainedDOM proves ChessOpen succeededwithnewreceipt butverifierexpectedoldtext; Xqdiagnosticserror emptyfieldcollidedwithW3Cerror detector. Externalverifier-onlyfixes requireactualacceptedpath/unmodified/settledOpenreceipt andwrapreadonlyXqdiagnostics withoutchangingtransporterrorhandling.52rootGUItests/39independentlyrerun smokechecks pass; originalfailurereports/screenshots/logexcerpts retained. No production changes sinceb568ea7. Independentreview approves verificationcorrection forfreshCI; taskremainsInReview.

Final acceptance: independent read-only r3_boundary_audit supports AC1–9 with no material finding at tested integration6e9b2e20a0315bbe902189376a6c0241dfd810f1; production unchanged fromapprovedb568ea7103e283155a851df16d3b382b3842eebc. Rust41106568111/Desktop41106568074 pass all3OS. Native166default/178allfeatures eachOS, realFull-byteSave/Open andgoldenfixtures pass; frontend243eachOS andhost6default/9localallfeatures pass. Linux33retained/294originalGUIfiles andWindows8retained/255originalGUIfiles independentlyhashaudited;15/15actualphases each with nativeexitproofs, normalinstrumentationexclusion, realXqcancellation/stoppedfreshOpen/event1analysis/corruptOpen/close. Local normalmacOS actualfile/lifecycle/Save-and-Quit evidence separately audited. Final currentcontract, fullACmatrix and exactpackage/CI/review provenance in docs/review/native-application-118.md. Original failures preserved. Sourceintegratedandpushedcodex/rewrite; finaldocumentation/evidencecompletion follows normally. No taskworktreecreated; unrelatedcheckouts unchanged. MSH119 nextready butunstarted; R4/R5/R6/variants/M5 notactivated.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
R3 establishes exactly one ActiveDocument with uniform RuntimeMetadata, bounded worker admission, guarded current-document installation and application-owned command/file/close lifecycle. Dormant Chess overlays, replaced metadata owners and the R2 three-install bridge are deleted.

The current native contract preserves Chess claims, Full analysis, history, preferences and Take back; Shogi actions and declaration semantics; and Xiangqi WXF events, claims and proof context. Bounded typed resource facts preserve existing limits, reservations and complete saveability. The sole finite frontend DTO/IPC forwarder owns no session or obsolete-file reader; MSH-119 deletes it with its callers.

Production source: b568ea7103e283155a851df16d3b382b3842eebc. Tested integration on codex/rewrite: 6e9b2e20a0315bbe902189376a6c0241dfd810f1; subsequent changes are verification or completion evidence only. Independent source, verification and artifact audits support all nine criteria with no material finding. Rust 41106568111 and Desktop 41106568074 pass all three systems: native 166 default / 178 all-feature tests per OS, frontend 243 per OS, plus normal macOS lifecycle interaction and 15-phase actual Linux/Windows packaged GUI campaigns. Original failed test/verifier probes remain explicit.

Exact package hashes, tests, review method, corrections and platform distinctions: docs/review/native-application-118.md. MSH-119/R4 frontend consolidation, R5 board interaction and R6 final integrated platform acceptance remain separate. M4 stays open. Stop after MSH-118; MSH-119 is the sole dependency-ready task and remains unstarted.
<!-- SECTION:FINAL_SUMMARY:END -->
