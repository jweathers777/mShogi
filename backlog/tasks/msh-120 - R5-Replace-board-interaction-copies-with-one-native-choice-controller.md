---
id: MSH-120
title: 'R5: Replace board interaction copies with one native-choice controller'
status: Ready
assignee: []
created_date: '2026-10-30T12:06:00Z'
labels:
  - architecture-migration
  - planning
milestone: m-4
dependencies:
  - MSH-119
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-target-115.md
priority: medium
type: task
ordinal: 120000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved MSH-115 migration stage; shared integration ownership supports MSH-46 without changing its existing prerequisites or closure gates. Development-era internal compatibility is not required. Preserve native selected profiles, complete history/actions/adjudication, atomicity, exact work identity, safe cancellation, current-format losslessness, admission/headroom and external interoperability. Planning creates future intent only; inspect current source and record a bounded plan on activation. One integrator owns overlapping cutovers; stop at this ticket after normal verification, independent review, integration and push.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One interaction owner replaces selection/inspection state in Chess/synthetic Board, ShogiBoard and XiangqiBoard. It consumes native opaque origins/action identities, ordered route steps and completion alternatives; it owns selection/prefix/candidates, terminal-versus-continuation, completion, cancellation, non-actionable inspection and board focus lifetime.
- [ ] #2 Specialized renderers supply board topology/geometry/coordinates/decorations, piece art/orientation, hands/board-adjacent native content, focus geometry and promotion/compound-route completion views. They compose within R4 shared product sections, do not recreate app layouts/panels/guards, and never compute legality/effects.
- [ ] #3 Complete native promotion/decline/forced choices, drops, equal-endpoint distinct routes, terminal prefixes with continuation, capture-before-enemy-inspection precedence and cancellation remain exact; partial selection never commits and Xiangqi steps[0] is not generalized into a universal action model.
- [ ] #4 Inspection uses authoritative native selected-history context and shared freshness, remains visibly non-actionable and retires on document/history/orientation/capability/unmount changes. Preserve Inspecting {side} {piece} · Esc clears, distinct markers, accessible announcements, Escape and keyboard/pointer focus recovery.
- [ ] #5 Delete replaced interaction implementations, obsolete controller adapters/imports and development >9 scrolling assumptions from production paths. Preserve game-specific sizing and full-board visibility without required scrolling/zooming at supported sizes; the labeled 16x16 compound fixture uses the same controller and stays verification-only.
- [ ] #6 Existing independently meaningful BoardRoutes/native generated choices/hands/promotion/inspection/stale tests run through the current controller. Actual packaged all-three-game pointer/keyboard/orientation/inspection/completion/focus plus separately labeled synthetic route journeys and independent interaction review support the change; normal packages exclude instrumentation.
- [ ] #7 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
