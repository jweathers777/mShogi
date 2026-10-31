---
id: MSH-121
title: >-
  R6: Independently verify the integrated current architecture and packaged
  product
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-31T17:00:00Z'
labels:
  - architecture-migration
  - planning
milestone: m-4
dependencies:
  - MSH-116
  - MSH-117
  - MSH-118
  - MSH-119
  - MSH-120
  - MSH-123
  - MSH-124
  - MSH-125
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-target-115.md
  - docs/desktop/post-r4-ux.md
priority: medium
type: task
ordinal: 121000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved MSH-115 migration stage; shared integration ownership supports MSH-46 without changing its existing prerequisites or closure gates. Development-era internal compatibility is not required. Preserve native selected profiles, complete history/actions/adjudication, atomicity, exact work identity, safe cancellation, current-format losslessness, admission/headroom and external interoperability. Planning creates future intent only; inspect current source and record a bounded plan on activation. One integrator owns overlapping cutovers; stop at this ticket after normal verification, independent review, integration and push.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Independent final source/consumer review verifies one controller/runtime, one active native document, current execution/IPC/document contract, frontend owner, shared application UI and interaction controller against the approved target, with game-native rules/search/history and capability content retained.
- [ ] #2 A negative-reference/caller/package audit covers startup, host handler registration, crate exports, frontend imports, examples, tests, synthetic/verification harnesses and builders. No obsolete production facades/transports/fallbacks/readers/controller copies or R2/R3 bridges remain. R1-R5 own deletion at their stages; R6 audits completion rather than postponing it.
- [ ] #3 Current native semantic transcripts and independent fixtures establish all-three-game complete actions/history/claims/declarations/outcomes, exact prefixes, Take back retained lineage, Full/exact proof/Unknown distinctions, actual observations and mode/limit/identity preservation. Do not substitute self-generated wrapper equality or reopen accepted rules just for architecture symmetry.
- [ ] #4 Integrated cancellation/replacement/history/undo/reopen tests reject stale results across games and purpose/limit/request/session/process changes. Lossless admission covers actual bytes/events/controls/ordinary-plus-exact observations/replay/stop headroom, including 256-to-257, interrupted/stale Save/Open and unsaved replacement/quit.
- [ ] #5 Actual installed or packaged application interaction on Windows, macOS and Linux exercises all three games: play and native special choices, claims/outcomes/recovery/retry, ordinary/advanced supported analysis cancellation/replacement, Save/Open, history/Return to live/Resume, Take back and native quit/window-close. Package build or bundled-engine smoke cannot replace GUI evidence; unavailable required checks leave this task unfinished.
- [ ] #6 Same-size normal/minimum cross-game captures and independent interaction/accessibility assessment establish shared theme/layout/player/status/analysis/history/actions/technical sections and shared conventions while boards/pieces/hands/native controls remain appropriate. Labeled synthetic compound/large-board evidence stays distinct from real games; normal distributables exclude instrumentation.
- [ ] #7 Current final source, independent review findings/resolutions, normal integration/push, appropriate native/desktop CI and exact package/environment/platform evidence are recorded. P9 baseline and MSH-111/112 histories remain qualified; a passing rerun is not proof of a fix. Do not close M4/46/47 or start variants/M5 automatically.
- [ ] #8 No intermediate adapter or split application is accepted as final architecture. Historical game/task/verification evidence remains preserved without retaining obsolete executable contracts; stop after this bounded integration verdict.
- [ ] #9 Final current-product acceptance additionally covers completed123/124/125 on all three required packaged platforms: sourced two-column Moves with exact non-move event context and origin/destination/path cues, supported-only dropdown with preserved side buttons, Analysis/Game Actions/Moves order and nonintrusive diagnostics/licenses, Save/Load with intentional atomic replacement, concise dirty Quit, direct native Resign and one-step browse Return versus intentional stopped Resume. Earlier AC terms Save/Open, history and technical sections refer to these owner-approved visible Save/Load, Moves and support placement; preserve all underlying semantic/safety/platform obligations and distinguish actual GUI from tests/builds.
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
1. Root Codex owns R6 integration in the primary clean codex/rewrite checkout at13510da, aligned0/0 with origin. Use pinned Backlog1.55.1; preserve unrelated worktrees and historical evidence. Independent read-only native and frontend reviewers challenge actual current source/consumers, negative deletion inventory and native capability retention; one integrator owns any narrow corrections.
2. Trace startup/host registrations/exports/current execution/current document/runtime/install/frontend subscription/operation and board interaction paths. Audit obsolete families/readers/facades/bridges/controllers across production, examples, tests, QA and builders; classify native specializations separately from redundant ownership. Reconcile all9AC against actual sources, independent fixture expectations and current semantic transcripts, not prior summaries alone.
3. Execute focused current native/session/host/frontend/process and admission/identity tests; reuse unchanged rule/reference expectations. Add only meaningful cross-boundary probes or narrowly scoped corrections needed by demonstrated gaps, preserving native profiles/semantics, losslessness, exact prefixes, work retirement and complete saveability. Record knownMSH112 failures as such; no timeout/assertion weakening.
4. Audit and run current packaged normal/separate-QA all3game campaigns on Windows/macOS/Linux, expanding only required unsupported verification coverage. Identify source/package/engine/host/runtime/driver/environment; actual local Mac GUI plus normal1180x820/min760x650 all3 captures/native close, special actions/Full/exact supported capabilities, and separately labeled synthetic routes. Normal instrumentation exclusion remains required; no build-only substitute.
5. Produce docs/review/architecture-acceptance-121.md mapping all9criteria to source/semantics/executed checks/platforms/limitations and independent review, with negative caller/package audit and123/124/125 UX reconciliation. Resolve material findings or record concrete blockers. Check only supported AC/DoD, normal integration/push, clean primary checkout and stop121; do not closeM4/46/47 or start variants/MSH112/M5.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner-approved post-R4 UX planning2026-10-31: complete direct dependency set retains116-120 and adds123/124/125 so final combined product acceptance includes the approved interface. Original8AC/4DoD remain unchecked and preserved; added criterion clarifies superseded visible terms without weakening atomic Save/Load, native diagnostics, exact history or required three-platform GUI.119 staysDone;120 and121 remainReady/unassigned/unstarted. No unrelated M4/variant/M5 gate or implementation added.

Startup13510da9e24b69a340444f272668ecb59478ddbb clean codex/rewrite, fetched origin0/0. Dependencies116/117/118/119/120/123/124/125 allDone; no active task owner displaced. Unrelated3worktrees preserved. GlobalBacklog1.55.0 mismatch resolved using existing pinned1.55.1; overview/execution read. Owner attachment explicitly requires fresh integrated source challenge and actual all3game/all3OS GUI, current normal/minimum views, native capability and lifecycle evidence. Existing125 production58a5d7a and verification8de evidence is baseline context, not automatic R6acceptance.

Fresh independent current135source audits PASS: native294checks(45rulesfixtures+22execution+15actualsubprocess+212session), frontend158+strictTS. No materialproductionfinding. Identified requiredGUIcoveragegaps in old15phases, so externalQA verifier now31phases plus100unit/gatechecks, native QA-only5exact-report fixture(1PASS) with independent Save/Open5reports PASS. InitialQAauthoring Clone compilefailure and wrongselected-prefix attachment rejection corrected without weakening nativeadmission. Independentverifier findings event8selector/newnativeack/Windowsjobownership/Winexecution-suffix addressed. Newgate5secbound stayswithin unchangedwatchdog, executesrealengine andrecordsidentities. Productdefaultsource unchanged; QAfixture/hostbranch/externaltools only. Currentstatus remainsInProgress with everyAC/DoD unchecked; actualfreshall3platformGUI/CI nowrequired.

Fresh independent native/public-API and frontend review confirms warning-prefix verifier correction: selected event8 is immutable intra-command history with live canonical9, event9 is committed warning prefix. Corrected probe checks exact selected events, canonical scope, selection identity and warning visibility;102 independent external checks PASS. No product correction required. Superseded176 Desktop campaign retains its actual outcome; corrected verifier campaign required before closure.
<!-- SECTION:NOTES:END -->
