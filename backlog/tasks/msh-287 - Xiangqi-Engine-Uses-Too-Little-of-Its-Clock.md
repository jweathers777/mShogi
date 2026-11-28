---
id: MSH-287
title: Xiangqi Engine Uses Too Little of Its Clock
status: Ready
assignee: []
created_date: '2026-11-28T10:14:00Z'
updated_date: '2026-11-28T10:19:00Z'
labels:
  - xiangqi
  - time-management
  - strength
  - model:medium
  - effort:medium
milestone: m-5
dependencies: []
documentation:
  - docs/m5/xiangqi-blunder-screen.md
  - docs/m5/strength-roadmap.md
priority: medium
ordinal: 265000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In the MSH-198 blunder screen (MSH-197 ladder, 60 s + 1 s), mShogi used a median of 1.5 s per move, and a 10 s rerun avoided 42% of all errors and 13 of 26 clear blunders. The old 30% single-mechanism fix rule blocked a fix round. Time allocation is shared across games (MSH-245 One Clock Policy for All Games), so a better allocation should be one shared change. The experiment runner's fixed-share policy is John's decision of 2026-11-25 (docs/m5/strength-roadmap.md); ask John before changing it.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Measure how the engine's own time manager spends a 60 s + 1 s clock in Xiangqi (time per move by phase, unused clock at game end)
- [ ] #2 A changed shared allocation (for example a larger share, or extra time when the best move or score is unstable) wins an SPRT against the predecessor in Xiangqi at 60 s + 1 s with the engine managing its own clock, or the task records why no candidate won
- [ ] #3 A kept change does not lose in a short SPRT in the other games that use the shared policy; results, cost and GCS path recorded
<!-- AC:END -->
