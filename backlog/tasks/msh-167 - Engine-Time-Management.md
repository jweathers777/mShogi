---
id: MSH-167
title: Engine Time Management
status: To Do
assignee: []
created_date: '2026-11-13T21:29:00Z'
labels:
  - handle:engine-time-management
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: medium
ordinal: 171000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Engine Time Management.

**Problem:** The engine uses a fixed time for each move. It spends the same time on a forced recapture and on a critical decision.

**Objective:** Use thinking time where it changes the result.

**Approach:** In the shared core, a soft and a hard limit per move from the clock or the configured time. Stop early when the best move is stable or forced. Extend when the best move changes or the score falls. Support a game clock with increment for game tests. Coordinate with MSH-140 (Player Thinking Time); do not change the desktop default in this task.

**Completion:** The engine plays correctly under a game clock with increment and under fixed time per move, and the time policy wins a game test at equal total time.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The engine never exceeds the hard limit in a recorded set of games under a clock with increment
- [ ] #2 With equal total time, the time policy is stronger than fixed time per move in a game test
- [ ] #3 Fixed time per move still works for the desktop and for diagnostics
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
