---
id: MSH-31
title: Build the minimal accessible chess board and live game presentation
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T13:15:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Connect the completed native controller through serialized Tauri commands; render native snapshots and complete choices in an SVG board; add source/destination selection, promotion dialog, orientation and roving keyboard focus; show live history/status and actual final engine observations; verify focused frontend interactions and actual native application behavior, then independently review and integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @codex with isolated builder /root/desktop_foundation; branch codex/m3-desktop, checkout .claude/worktrees/next-task. Foundation28 and native controller29 complete with all applicable CI; no test instrumentation will be added to normal app.
<!-- SECTION:NOTES:END -->
