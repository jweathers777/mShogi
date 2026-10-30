---
id: MSH-116
title: 'R1: Establish the single internal execution and saved-observation contract'
status: Ready
assignee: []
created_date: '2026-10-30T12:06:00Z'
labels:
  - architecture-migration
  - planning
milestone: m-4
dependencies:
  - MSH-115
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-target-115.md
priority: medium
type: task
ordinal: 116000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved MSH-115 migration stage; shared integration ownership supports MSH-46 without changing its existing prerequisites or closure gates. Development-era internal compatibility is not required. Preserve native selected profiles, complete history/actions/adjudication, atomicity, exact work identity, safe cancellation, current-format losslessness, admission/headroom and external interoperability. Planning creates future intent only; inspect current source and record a bounded plan on activation. One integrator owns overlapping cutovers; stop at this ticket after normal verification, independent review, integration and push.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Exactly one current tagged internal request/response family and submit/poll path serves actual Chess/Shogi/Xiangqi consumers. Remove Legacy/Shared dual transport, V2 wrapper conversion, chess_adapter and unknown-version legacy-shape fallback after all callers migrate; standard UCI/USI/Xiangqi interoperability remains unchanged.
- [ ] #2 Complete submitted game/profile/schema/history, purpose, selected prefix, request/session/process identities and limits bind acceptance. Moves, claims/declarations, concluded games, incomplete/Unknown, failure and Chess Full remain native-distinct; no fabricated fallback/score/outcome or revived retired work.
- [ ] #3 Durable observation structures, validators, fixtures, desktop/archive consumers and actual child entry points migrate with the transport. Observations retain exact native target/result/provenance as historical data; current-stage complete Save/Open works without preserving an old execution parser.
- [ ] #4 Actual encoded request/response/diagnostic limits, bounded queues/watchdogs/cleanup, cancellation/replacement and stale-result protection remain verified through real subprocess tests, including same-position/different-history and wrong purpose/limits/identity/selection.
- [ ] #5 R1 and R2 may coordinate saved-observation/codec cutover as independently reviewed subunits under one bounded integration owner if separate landing would create incomplete persistence or throwaway formats. Keep an acyclic task dependency, record exact subunit evidence, and do not mark either Done with unsupported criteria or leave a permanent adapter.
- [ ] #6 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
