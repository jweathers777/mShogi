---
id: MSH-17
title: >-
  Implement chess draw claims, automatic results and the decided dead-position
  scope
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
labels:
  - rules
  - adjudication
milestone: m-2
dependencies:
  - MSH-16
documentation:
  - docs/rules/chess-coverage.md
priority: high
ordinal: 17000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The M1 classifier deliberately reports quiet for positions that FIDE ends or allows a claim on, for example bare kings, the 75-move rule or fivefold repetition. With the profile decided (MSH-15) and rules-owned history in place (MSH-16), the library must produce those outcomes. It must not present a heuristic material check as complete dead-position adjudication.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Threefold and fifty-move claims, including intended-move claims, are available exactly as the decided profile specifies and match independent fixtures
- [ ] #2 Automatic fivefold and 75-move results apply with the specified precedence, including checkmate on the last move
- [ ] #3 Dead-position recognition implements exactly the owner-decided scope, and any incompleteness is stated in the ledger and API docs
- [ ] #4 Game results are distinct from the position classifier, and the coverage ledger rows move to Implemented only with evidence
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
