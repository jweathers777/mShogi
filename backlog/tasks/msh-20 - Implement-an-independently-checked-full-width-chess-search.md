---
id: MSH-20
title: Implement an independently checked full-width chess search
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T20:44:00Z'
labels:
  - search
milestone: m-2
dependencies:
  - MSH-17
  - MSH-19
documentation:
  - docs/design.md
priority: high
ordinal: 20000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Design section 6 requires search correctness to be established on small independently solvable positions or synthetic trees before relying on pruning or transposition reuse. Perft and rules tests do not establish search correctness, and a speedup is not evidence of strength. Search must run on private working state, never on the caller's game (A-05).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A full-width search returns only legal actions and correct scores on independent synthetic trees and on small solvable chess positions (for example forced mates with known depth), covering score perspective and terminal scoring
- [ ] #2 Search runs on private working state and leaves the caller's position and history unchanged
- [ ] #3 Any pruning or transposition reuse is added only with targeted regressions and does not reuse history-dependent results solely by board key
- [ ] #4 A measured search performance baseline is recorded with hardware, build and parameters, with no invented target
- [ ] #5 Search consumes the rules-owned game context and adjudication, including relevant history and the selected claim policy. Tests use the same current position with differing relevant histories and distinguish available optional claims from automatic outcomes; the caller game and history remain unchanged.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
