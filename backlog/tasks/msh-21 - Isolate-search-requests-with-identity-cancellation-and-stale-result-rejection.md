---
id: MSH-21
title: Isolate search requests with identity, cancellation and stale-result rejection
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
labels:
  - search
  - concurrency
milestone: m-2
dependencies:
  - MSH-20
documentation:
  - docs/design.md
priority: high
ordinal: 21000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
A-05 requires that every search request and result identify the request and its base position and version, that superseded results cannot change the current game, and that cancellation neither corrupts the game nor blocks behind unbounded progress output. Protocol adapters, the match runner and the M3 desktop all depend on this contract.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each search request and result carries the request identity and base position/version, and a result for a superseded request is rejected
- [ ] #2 Cancellation returns promptly within a documented bound, leaves game state intact, and is tested, including delayed results after a position change
- [ ] #3 Progress reporting is bounded and cannot block the game or corrupt protocol output
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
