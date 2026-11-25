---
id: MSH-238
title: Remove the fixed ten-minute experiment cap
status: Ready
assignee: []
created_date: '2026-11-24T22:29:00Z'
updated_date: '2026-11-25T11:58:00Z'
labels:
  - model:medium
  - effort:medium
dependencies: []
documentation:
  - docs/m5/chu-strength-harness.md
priority: medium
ordinal: 7910
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-52.3 truncated seven of eight 40s+0.4s versus 80s+0.8s color pairs at the fixed 600000ms experiment ceiling. Both the native runner and native_match reject a larger caller-specified budget, preventing longer measurements even within the approved cloud budget.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The native experiment runner and match driver accept positive representable caller-chosen durations above ten minutes without another arbitrary duration ceiling.
- [ ] #2 The configured deadline still stops work and records unfinished games as unscored; game rules and product search behavior are unchanged.
- [ ] #3 Current callers, tests and measurement documentation use the same duration contract; required checks pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: runs after the MSH-209 follow-ups and before MSH-243, which uses the runner.
<!-- SECTION:NOTES:END -->
