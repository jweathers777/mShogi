---
id: MSH-121
title: >-
  R6: Independently verify the integrated current architecture and packaged
  product
status: Ready
assignee: []
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-31T09:05:00Z'
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

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner-approved post-R4 UX planning2026-10-31: complete direct dependency set retains116-120 and adds123/124/125 so final combined product acceptance includes the approved interface. Original8AC/4DoD remain unchecked and preserved; added criterion clarifies superseded visible terms without weakening atomic Save/Load, native diagnostics, exact history or required three-platform GUI.119 staysDone;120 and121 remainReady/unassigned/unstarted. No unrelated M4/variant/M5 gate or implementation added.
<!-- SECTION:NOTES:END -->
