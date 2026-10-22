---
id: MSH-9
title: Complete legal move generation and position-level terminal classification
status: Ready
assignee: []
created_date: '2026-10-22T13:27:00Z'
labels:
  - rules
  - legality
  - terminal
milestone: m-1
dependencies:
  - MSH-8
documentation:
  - docs/design.md
  - AGENTS.md
priority: high
ordinal: 9000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Expose a trustworthy legal action set and the explicitly scoped chess position classifications.

## Design references

A-01; A-04; M1 legal scope

## Scope

Filter or directly generate legal moves with correct king safety, including pins, single/double check, king moves, and special moves. Classify check, checkmate, and stalemate separately from full match adjudication.

## Verification expectations

Compare complete legal sets and terminal classifications with the independent corpus. Exercise invalid requested actions and distinguish an empty legal set in check from one outside check.

## Out of scope

Calling limited material checks complete dead-position adjudication, full draw claims/clocks, or engine strength.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The public legal-action API returns exactly the expected legal set on the selected independent corpus.
- [ ] #2 Pins, discovered attacks, single/double check, king safety, and special-action edge cases are covered.
- [ ] #3 Check, checkmate, and stalemate are unambiguous and match independent fixtures.
- [ ] #4 The coverage ledger clearly identifies remaining draw/history/match obligations instead of reporting complete chess.
- [ ] #5 Applying a public legal action and rejecting an illegal one obey the transition and restoration contracts.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
