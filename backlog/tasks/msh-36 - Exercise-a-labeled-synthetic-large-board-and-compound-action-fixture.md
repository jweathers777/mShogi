---
id: MSH-36
title: Exercise a labeled synthetic large-board and compound-action fixture
status: Ready
assignee: []
created_date: '2026-10-25T12:48:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-31
documentation:
  - docs/desktop/m3-design.md
ordinal: 36000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The desktop presentation must encounter large geometry and staged selection before variant implementation without claiming an unsupported real game.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A clearly labeled synthetic development fixture displays a readable 16-by-16 board and multi-stage route selection without claiming another real game.
- [ ] #2 Partial selection and cancellation never commit an action; completed native fixture choices remain distinct from chess sessions and records.
- [ ] #3 Pointer, keyboard, orientation and route ambiguity interactions are verified in the running app; normal distributables exclude test instrumentation.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
