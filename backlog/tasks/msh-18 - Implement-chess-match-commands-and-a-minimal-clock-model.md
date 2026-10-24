---
id: MSH-18
title: Implement chess match commands and a minimal clock model
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
labels:
  - match
  - clocks
milestone: m-2
dependencies:
  - MSH-16
documentation:
  - docs/rules/chess-coverage.md
  - docs/design.md
priority: medium
ordinal: 18000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Resignation, agreed draws and time loss are match commands or match state, not board actions (A-02), and FIDE 5.1.2 and 6.9 make their outcomes depend on whether mate is still possible. Reproducible self-play and engine matches in M2 need at least a deterministic time-control model with recorded limits.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Resignation and agreed-draw commands are distinct from board actions and follow the decided profile, including the possible-mate exceptions
- [ ] #2 A clock model supports the time controls the match runner needs, deterministically under test, and time loss applies the possible-mate exception per the profile
- [ ] #3 Match commands and clocks never mutate the rules position directly and are covered by tests with independent expectations
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
