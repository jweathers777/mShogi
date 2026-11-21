---
id: MSH-219
title: Shogi Match Test Timing Failure
status: Ready
assignee: []
created_date: '2026-11-21T04:35:00Z'
updated_date: '2026-11-21T04:35:00Z'
labels:
  - runner
  - test
dependencies: []
priority: medium
ordinal: 226000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The runner test a_full_mshogi_game_replays_and_tampering_is_rejected (crates/mshogi-runner/tests/shogi_match.rs) plays a timed Shogi game between two release mshogi USI engines and requires a native conclusion. It failed once in a release workspace run on John's Mac during MSH-205 (assertion at line 124, report.stop) and passed in 3 later runs; an unidentified search/runner failure in MSH-200 may be the same. Find the stop reason and make the test deterministic, or fix the defect it shows.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The cause of the failure is identified and recorded
- [ ] #2 The test gives the same result in repeated runs (for example 20 runs), or the defect is fixed with a test
<!-- AC:END -->
