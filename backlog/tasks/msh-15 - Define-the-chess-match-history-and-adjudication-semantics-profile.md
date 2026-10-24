---
id: MSH-15
title: Define the chess match, history and adjudication semantics profile
status: Ready
assignee: []
created_date: '2026-10-24T19:56:00Z'
labels:
  - rules
  - match
  - design
milestone: m-2
dependencies:
  - MSH-14
documentation:
  - docs/rules/chess-coverage.md
  - docs/design.md
  - docs/review/m1-action-boundary.md
priority: high
ordinal: 15000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M1 deliberately stopped at a position-level classifier. The coverage ledger records Deferred FIDE match-semantics obligations (dead position, resignation, agreed draws, threefold and fifty-move claims including intended-move claims, automatic fivefold/75-move results with checkmate precedence, time loss with the possible-mate exception) and Unresolved design questions: the history/claim API shape, automatic-result ordering, the extent of dead-position recognition, software match/event policy and cache identity. The design assigns completing the needed chess match semantics to M2, and the ledger makes M2 own every Deferred FIDE match-semantics row unless the owner explicitly moves one to M4. Implementation tasks cannot start until these semantics and decisions are written down, so this task produces the versioned profile they consume.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A versioned chess match/history profile specifies the history-dependent state, repetition identity (distinct from saved-state Eq and cache identity), claims (current and intended move), automatic results and their precedence, dead-position scope, match commands, and time-loss rules, each with FIDE references and recorded interpretations
- [ ] #2 Every Unresolved ledger row has a concrete proposal presented to John and a recorded owner decision before it is marked resolved; any obligation postponed beyond M2 is recorded with an owner decision and a named later owner
- [ ] #3 Independent expected fixtures (sourced or hand-derived with rationale) are defined for each specified claim and automatic result, ready for the implementation tasks
- [ ] #4 The coverage ledger and design references are updated to the decided profile without relabelling anything implemented
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
