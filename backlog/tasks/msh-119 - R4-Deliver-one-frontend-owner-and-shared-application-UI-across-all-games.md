---
id: MSH-119
title: 'R4: Deliver one frontend owner and shared application UI across all games'
status: Ready
assignee: []
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-30T12:10:00Z'
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
- [ ] #1 Exactly one production frontend session/subscription/operation/document-close owner serves all games. Migrate all product, host, harness, fixture and example callers; delete useNativeSession, obsolete App orchestration/embedded Chess Snapshot/nested old command DTO, duplicate guards and Chess desktop_snapshot/command/save/save_as/open endpoints and R3 forwarder.
- [ ] #2 One application-level product composition and section implementation owns layout/hierarchy, dark responsive theme, document/lifecycle controls, player and live status, analysis, history, Game Actions, Technical Details, dialogs/notices/errors/recovery, visible language and shared focus/keyboard/accessibility conventions. Three parallel game page implementations under common chrome are not an acceptable endpoint.
- [ ] #3 Native capability content composes into those same shared sections: Chess claims and Full controls, Shogi declaration advice/formal controls and hands, Xiangqi WXF warnings/count claims/pending state, and native-specific completion content have correct meanings with application-owned placement/presentation. Board topology/artwork/orientation and genuinely native capability renderers remain specialized; no universal renderer or frontend rules interpreter.
- [ ] #4 All existing Chess functionality is represented in the current tagged contract and shared UI: intended/current claims without playing an intended move, bounded read-only moves and explicit Full proof/Unknown/cancel/failure, native history/observation provenance, unresolved resignation, real telemetry/perspective, settings, retry/Resume and Take back. Empty shared Chess analysis projections cannot count as migration.
- [ ] #5 Exact rendered snapshot authority, stale/dirty Save receipts, replaced documents, native quit/close guards, historical selection/Return to live and stopped reopen remain protected. Common controls and section order stay stable across games; promotion/drop/compound completion stays native supplied.
- [ ] #6 R4 owns the complete application UI and common non-board focus/accessibility lifecycle; R5 owns the board selection/inspection/route controller and board focus transitions, consuming R4 slots/contracts. Any temporarily retained board-state copy is explicitly retired in R5; there is no unassigned UI ownership gap or separate UI-refactor ticket.
- [ ] #7 Behavioral tests port from old hooks/Document/App/Analysis/History/Recovery instead of retaining old interfaces. Same-size normal and supported-minimum captures of all three actual packaged games, pointer/keyboard, native quit/save/reopen, analysis/claims/recovery and independent visual/source review demonstrate one application; record exact package/revision/platform and limitations.
- [ ] #8 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
- [ ] #9 Shared errors/notices/recovery consume R3 authoritative typed capacity/resource facts, not diagnostic substrings or numeric-limit guesses. Preserve truthful generic unknown/multiple-cause feedback, Technical Details, actual ordinary/exact counts, independent file-failure messaging and Save/Save As availability; native control/observation/byte/event/replay payload regressions cover all applicable games.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
