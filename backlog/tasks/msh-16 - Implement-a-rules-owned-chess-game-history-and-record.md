---
id: MSH-16
title: Implement a rules-owned chess game history and record
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
labels:
  - rules
  - history
  - state
milestone: m-2
dependencies:
  - MSH-15
documentation:
  - docs/review/m1-action-boundary.md
  - docs/design.md
priority: high
ordinal: 16000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Chess Position is deliberately local: it holds no history, and its Eq is saved-state identity only. The only undo stack today is the console session's adapter-level Vec<Undo>. Repetition, claims and automatic results need history owned by the rules library (A-01, A-03); the M1 action-boundary review records this as required change 6. A canonical game record must also carry the rules-profile ID, initial state and complete actions without effects or private undo layout (A-03, A-06).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A rules-library game type owns the current position plus the history the match profile requires, including repetition identity distinct from saved-state equality and from any cache key
- [ ] #2 Applying and undoing through the game restores position, history and repetition tables exactly; sequence-property checks in the style of MSH-12 cover it
- [ ] #3 A canonical game record (profile ID, initial state, ordered complete actions, no effects, no private undo layout) round-trips and is replayable deterministically
- [ ] #4 Console and future adapters use the rules-owned history rather than keeping their own rule-relevant state
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
