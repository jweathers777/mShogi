---
id: MSH-166
title: Search Selectivity Completion
status: To Do
assignee: []
created_date: '2026-11-13T21:29:00Z'
labels:
  - handle:search-selectivity-completion
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: high
ordinal: 170000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Search Selectivity Completion.

**Problem:** The Chess search has null-move pruning, late-move reductions, futility, exchange pruning and check extension. A source survey on 2026-11-13 did not find reverse futility pruning, razoring, late-move pruning, internal iterative reduction, singular extensions or an improving condition, and late-move reductions are off in low-material positions.

**Objective:** Complete the standard classical selectivity in the shared core and keep only what game tests support.

**Approach:** Add one technique at a time. Accept each one only on the game test against the predecessor. Put the techniques in the shared core with switches per game where a game needs them. Do not tune on single positions.

**Completion:** Each listed technique has a recorded accept or reject decision from a game test, and the accepted set is the default.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each technique in the description has a recorded game-test result and an accept or reject decision
- [ ] #2 The accepted set together is stronger than the engine at the start of the task in a game test
- [ ] #3 The blunder suite result is not worse than at the start of the task
- [ ] #4 Accepted techniques are in the shared core, not in Chess-only code, unless a recorded reason shows they are Chess-specific
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
