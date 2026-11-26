---
id: MSH-275
title: Use the Current Byoyomi Period in USI Overtime
status: To Do
assignee: []
created_date: '2026-11-26T18:33:00Z'
labels:
  - console
  - protocol
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 249000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Account for the current USI byoyomi period when main time runs out. The MSH-256 review reproduced the existing behavior with go btime 0 wtime 0 byoyomi 500 from startpos: the Fischer allocator capped spending at zero remaining main time and returned a depth-zero, zero-node move after about 2 ms.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 With zero main time and a positive byoyomi period, the timer assigns usable thinking time within the current period and leaves a response margin.
- [ ] #2 Tests cover the transition to overtime on both sides, zero main time with and without a period, and ordinary Fischer and fixed-work controls.
- [ ] #3 The USI protocol guide describes the current-period behavior and a real process test verifies a native legal bestmove within the period.
<!-- AC:END -->
