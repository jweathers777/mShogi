---
id: MSH-72
title: >-
  Extend shared native sessions sides and complete-choice presentation for chess
  and shogi
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-69
  - MSH-70
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 72000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Native command/context tests,native-generated frontend fixtures,keyboard/orientation/drop/promotion and cross-game cancellation/focus tests; actual consumer integration follows shogi desktop task.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Native session dispatch serializes chess/shogi commands,result acceptance and profile replacement with exact document/session identity; the webview receives native snapshots/legal choices and never computes legality.
- [ ] #2 Profile-qualified side IDs,first-mover labels,piece ownership/orientation and native board descriptors separate chess White/Black from shogi Sente/Gote. Game switching retires old results and selection even at matching coordinates.
- [ ] #3 Shared board/selection supports9x9 square geometry,board or hand origin,ordered routes and promotion/decline completion with click/keyboard/focus/cancel behavior; partial selection cannot commit.
- [ ] #4 Native analysis capability/score metadata preserves chess Full and exposes only supported bounded shogi operations; stale/unsupported results cannot repaint or become engine-turn actions.
- [ ] #5 Concrete native shogi-generated hand/choice fixtures and unchanged chess journeys verify the common boundary,while game-specific labels/glyphs/outcomes/application journeys remain owned by MSH41.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
