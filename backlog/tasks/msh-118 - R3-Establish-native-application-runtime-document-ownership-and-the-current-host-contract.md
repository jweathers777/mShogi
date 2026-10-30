---
id: MSH-118
title: >-
  R3: Establish native application runtime document ownership and the current
  host contract
status: Ready
assignee: []
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-30T12:12:00Z'
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
- [ ] #1 One application controller owns explicit common runtime/document metadata and exactly one active native-tagged document. Replace the ever-present global Chess DesktopSession plus optional overlay ownership; game-native state, histories, settings, journals, observations and semantic command admission remain independent.
- [ ] #2 One bounded admission/execution/file lifecycle handles typed native candidates, stable receipts, identity reservations and actual per-native replay/encoding costs off the serialized host lock. Commit only after complete current-format admission; failed/cancelled/stale work preserves accepted saveable state.
- [ ] #3 One installation transaction checks authority/receipt, retires execution, allocates fresh document/session/request/process/selection identities and installs validated data stopped with saved observations historical. History selection, semantic undo, Take back and replacement never revive old work.
- [ ] #4 The current host command/snapshot/file/application-close contract provides complete native Chess claim/preferences/ordinary and Full analysis behavior, Shogi capabilities and Xiangqi event/claim/proof context. Remove empty compatibility projections as a target; no frontend rules or common outcome reinterpretation.
- [ ] #5 Delete the R2 new-codec-to-old-runtime adapter and replaced native metadata/install owners here. A temporary current-controller-to-old-frontend-shape/command forwarder may serve only the named in-repository caller until R4, with no second session or old-file support; R4 deletes its endpoints and DTO together.
- [ ] #6 Native menu Quit/window-close and common request/cancel/finish close behavior migrate to neutral application ownership rather than being deleted by desktop_* name prefix. Preserve dirty guards, file/command reservations, Tao termination regression and normal instrumentation exclusion.
- [ ] #7 Focused all-three-game admission/install/current-IPC/identity/Take back/cross-game replacement and failure tests plus actual packaged lifecycle/file/close interaction on an identified platform support the changed owner; distinguish final R6 three-platform evidence.
- [ ] #8 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
- [ ] #9 Native admission owns explicit bounded typed resource facts delivered through the current host/snapshot/error contract, with resource category and applicable actual count/limit while unknown/multiple causes remain expressible. Preserve bounded Technical Details and truthful generic fallback; capacity is distinct from illegal actions, game outcomes and engine failures. Focused native payload tests cover control-only zero observations, genuine and combined ordinary/exact observation accounting, 256-to257, bytes/events/replay and stop/close headroom, with Save/Save As preserved. R4 consumes these facts without diagnostic-string inference.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
