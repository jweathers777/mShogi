---
id: MSH-38
title: Verify installed M3 applications and bundled engines on three platforms
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T21:03:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-37
documentation:
  - docs/desktop/m3-design.md
ordinal: 38000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Package compilation is insufficient evidence that the installed application and engine work together for the complete desktop milestone.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Actual packaged application installation/launch and bundled-engine play are verified on Windows, macOS and Linux with exact source, package and environment identities.
- [ ] #2 Each platform exercises analysis/cancellation, outcomes/recovery, save/reopen/history and synthetic fixture interactions; failures are resolved or remain explicit blockers.
- [ ] #3 Normal package contents exclude instrumentation; durable screenshots/logs/native CI and reproducible launch/install instructions support all three platform claims.
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
Preserve exact normal package identities and reuse unchanged macOS CUA evidence from MSH-37. Extend the external package verifier with a bounded W3C WebDriver harness against actual installed Linux/Windows hosts, using tauri-driver 2.0.6 and OS-specific native dialog/close helpers outside distributables. Preflight actual desktops and matching WebView2/WebKit drivers; install real packages, drive visible play, analysis cancellation/replacement, genuine engine failure/retry, Save/Open/history/Return to live/Resume, and native quit/window close. Build separately labeled QA packages for promotion/claims/incomplete/capacity/synthetic states, preserve normal artifacts, and verify instrumentation exclusion. Capture exact source/package/host/engine/runtime/driver/environment, ordered checks, native screenshots and failure traces; resolve actual findings or record concrete blockers without substituting package construction. Independently review harness and resulting evidence, integrate/push normally, require applicable final-source native CI; leave M3 open until MSH-39.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @codex, codex/m3-desktop in .claude/worktrees/next-task; MSH-37 is Done and integrated/pushed with complete GUI/evidence review and all-platform native/package CI. Isolated delegation: platform_gui_research Linux native adapter, capacity_native Windows native adapter/runtime driver setup; root owns common harness/workflow/integration, capacity_review independent review. No product instrumentation or new rules work. M2 stays closed, M4-M6 remain future To Do.

External GUI verifier implemented with W3C pointer/keyboard actions and read-only DOM observations, real native file pickers/close/Quit, exact executable/process identity and pre-teardown exit proof. Normal Linux GUI stays actually apt-installed; genuine missing-sidecar failure/retry runs in the separately labeled writable QA standard package, preserving installed /usr/bin permissions. QA promotion/claim/capacity/synthetic phases are distinct. Native evidence directories are per phase. Independent static review caught and resolved Linux sidecar permissions, Windows DLL lookup, post-Open inspector selection, promotion text selector and synthetic startup readiness. Workflow YAML parsing, Python compilation and diff checks pass; actual Windows/Linux GUI execution is not yet verified.

First actual GUI run 40228382037 at 9b7a2a6 failed and remains recorded as failed. Linux really installed/launched and loaded the frontend, but WebKit SVG-group element-click rejected the first move; native desktop was blank with DRI3 warnings. Windows really installed/launched and played e2e4/a7a5, then verifier clear() appended a controlled number field and validation correctly refused analysis. Native Windows images also exposed an external WebView2 console overlay. Evidence retained in docs/desktop/evidence/m3-platforms/*-attempt-1. Independently reviewed corrections use real viewport W3C pointer actions with read-only geometry/hit checks, large-board keyboard routes, recorded Xvfb compositing override, real Select-all/Backspace/typing with observed-value assertion, external-driver console suppression, and exact native app/modal focus before raw desktop screenshots. No product source or criteria changed; all corrected GUI journeys require a new actual run.

Corrected run 40229968153 at41866a3: all-platform Rust CI40229968189 passed and macOS normal package passed. Linux actual desktop now renders and real e2e4/a7a5, active analysis cancellation and bounded replacement pass. Native Save created complete normal-complete.mshogi.mshogi because GTK retained the default extension; exact requested-path assertion failed. Failed partial report/screenshots/archive preserved in linux-attempt-2. Independently reviewed external adapter correction selects the whole location field with native Ctrl+A, captures entered path and requires the exact file. No product source or acceptance weakening; full Linux rerun remains required. Windows corrected GUI execution remains pending.

Windows second actual attempt40229968153 also passed real play, active analysis cancellation and bounded replacement, then native Save selected default game.mshogi rather than the requested path. Failed report/images preserved in windows-attempt-2. The helper incorrectly raised the main window above the exact-PID ownerless native Save modal; correction binds that modal by PID/executable/native class/exact title and uses native keyboard filename replacement with pre/post-blur value observations. The actual native trace additionally exposed a genuine bundled-engine ConsoleWindowClass stealing foreground focus. MSH-38 resolves that platform defect with Windows-gated CREATE_NO_WINDOW on supervised internal-search child launch, preserving piped protocol and standalone CLI/UCI. External native captures now reject bound-engine consoles and post-capture focus drift. Local format, engine Clippy, ten engine checks and nine actual-process desktop play/analysis/persistence tests passed on macOS; Windows behavior still requires corrected packaged GUI/native CI. Independent review underway; no ticket completion claimed.

Fresh local macOS normal package rebuilt from f8a132b: host0bb29929, engine45d264 unchanged. CUA real play/cancellation, complete Save, clean Quit with separate process absence, fresh process Open/history/Return live/Resume and native close Cancel plus freshPID32024 Discard exit passed; durable exact package/archive/screenshots/process-observations in macos-current. Explicitly exclude ambiguous app-bound AX reactivation as exit proof. The journey found completed I/O messages falsely saying current historical viewing/paused play after Return/Resume. Corrected only completion wording to past-operation evidence; current state remains native banner/controls. Updated generated snapshots and external assertions; 40 native session tests, tsc,13 focused History/Document tests and format/diff checks passed. Independent review and final package GUI checks still required.

Third Linux GUI attempt40231293207 atf0c8dd6 passed exact native Save:5822bytes/2observations/5controls/2gameevents; first Return left the correct full path in the real GTK Open dialog with Open enabled. Failedpartial native screenshots/archive/report preserved in linux-attempt-3. Independently reviewed external correction uses bounded1second Return wait then actual Alt+O acceptance and retains pre-fallback screenshot/key. No content-state checks weakened; Open/history/Quit/fixtures remain pending. Completed I/O message fixb5b17d0 independently reviewed; rebuilt macOS normal actual Open/history/Returnlive/Resume/nativeQuitGuardDiscard passed, separate escalatedpgrep empty/exit1 before any app-bound observation. Focused current-source report macos-status; native snapshot generator matches committed fixtures exactly. Windows third GUI result pending.

Third Windows run atf0c8dd6 exact native Save passed5822bytes/2observations/5controls/2gameevents; nine native captures had no exact-engine console and actual ownerless pickers remained foreground. Open failed external UIA name-only ambiguity: two Open dropdowns plus correct ID1 button. Failedpartial evidence windows-attempt-3 preserved. Independent review replayed actual UIA tree through correction and selected unique title/ID1. Combined fourth run will include reviewed Linux Alt+O fallback, Windows button identity correction and completed-message fixb5b17d0. External verifier now independently records/attempts all6isolatedphases only after successful cleanup, while ANYrequiredphase failure makes overallnonzero; cleanupfailure aborts and retains failedJSON. Three isolated reporting-control-flow checks pass, explicitly not GUI passes. Final actual platform evidence remains pending.

Fourth Linux40232694201 ate313ef0 attemptedall6phases. Standard recovery/retry, promotion and capacity passed with native pre-teardown host/engine absence; exact Save/Open/historical analysis/Returnlive/Resume/nativecloseCancel passed in normal. Normal+claim finalguardPOST terminatedWebKit beforeHTTPresponse, so stoppedproofwasnotrun and those phases remainfailed. Synthetic startup/status RemoteDisconnected exposed driver-readiness race beforeapplaunch. Independently reviewed commoncorrection scopes knownterminaltransport errors ONLYafterobservednativeQuitguard/resolvedfinalbuttonPOST, stillrequiresnativehost+engineabsenceBEFOREteardown; startup-only boundedtransportretry recordsattempts and abortsondriverdeath. Screenshotorder now allows150msnativepresentationafterwebviewcapture, retainingrawdesktoponfailure. Actual capacity nativePNG exposed offscreenpauseexplanationabove disabledStart; independentlyreviewed productfix focusesnewnativecapacityid alert once (tabIndex-1), preservinguserfocus onidenticalpolls. FourCapacitytests and25focusedCapacity/Analysis/Historyfrontendchecks+tscpassed; actualpackagedvisibilityproofpending.
<!-- SECTION:NOTES:END -->
