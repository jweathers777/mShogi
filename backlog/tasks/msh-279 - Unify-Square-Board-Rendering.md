---
id: MSH-279
title: Unify Square-Board Rendering
status: To Do
assignee: []
created_date: '2026-11-27T12:34:00Z'
labels:
  - technical-debt
  - model:large
  - effort:high
dependencies: []
documentation:
  - docs/design.md
  - desktop/README.md
priority: medium
ordinal: 253000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Follow up MSH-239 and MSH-264 by evaluating duplication among Board, SquareBoard, and ShogiBoard. Share rendering and interaction behavior where it is practical.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Common square-grid geometry, rendering, and interaction presentation use one implementation wherever practical.
- [ ] #2 Piece artwork, hand trays, and board geometry are independently composable.
- [ ] #3 Standard Shogi and Tori retain hand trays and drop interactions. Chess, Chu, and Tenjiku retain their appropriate presentation. Xiangqi retains intersection geometry.
- [ ] #4 Preserve approved appearance, keyboard navigation, movement inspection, promotion, and compound-action behavior across all games.
- [ ] #5 Verify frontend behavior and appearance at supported window sizes.
<!-- AC:END -->
