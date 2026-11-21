---
id: MSH-220
title: Tori Exchange Values from the Fitted Evaluation
status: To Do
assignee: []
created_date: '2026-11-21T17:47:00Z'
labels: []
dependencies: []
priority: low
ordinal: 227000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-206 (Tori Evaluation Round) fitted the Tori board and hand values, but static exchange (SEE) and move ordering still use the earlier fixed piece values (position.rs VALUE). Test whether exchange and ordering values taken from the fitted board and hand weights play stronger.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 SEE and ordering use values derived from eval::DEFAULT, or the null result is recorded and the change is reverted
- [ ] #2 The change passes the sequential predecessor test (H1 about 10) against the MSH-206 result, or the null result is reported
<!-- AC:END -->
