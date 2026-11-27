---
id: MSH-278
title: Evaluate Remaining Game History Specializations
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
priority: medium
ordinal: 252000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Follow up MSH-252 by comparing the generic GameHistory with the specialized Shogi and Xiangqi history owners. Consolidate common operations where the resulting architecture is simpler.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Identify which differences follow from game rules, event semantics, storage strategy, or measured performance.
- [ ] #2 Evaluate Shogi prefix replay and Xiangqi multiple-event commands.
- [ ] #3 Share operations where it simplifies the architecture, retaining independently justified storage and event models.
- [ ] #4 Verify unchanged rules, adjudication, transactional integrity, replay performance, and record semantics.
- [ ] #5 Record the reasons for each retained specialization.
<!-- AC:END -->
