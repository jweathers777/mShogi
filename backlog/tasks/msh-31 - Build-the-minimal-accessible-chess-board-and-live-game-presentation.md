---
id: MSH-31
title: Build the minimal accessible chess board and live game presentation
status: Ready
assignee: []
created_date: '2026-10-25T12:48:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-28
  - MSH-29
documentation:
  - docs/desktop/m3-design.md
ordinal: 31000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Players need a readable board and complete native action selection without duplicating rules in JavaScript.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The board-centered SVG interface renders native pieces, turn, game status and move history; legal native choices drive click-to-move and all promotion choices.
- [ ] #2 Board orientation can change independently of human color; keyboard arrows, Enter/Space, Escape, visible focus and coordinate/piece labels support board and promotion operation.
- [ ] #3 Interaction tests and running-app evidence cover native choice selection, promotion, orientation and keyboard behavior; frontend contains no rules engine and independent review is recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
