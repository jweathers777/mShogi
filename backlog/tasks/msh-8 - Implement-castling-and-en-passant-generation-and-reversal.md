---
id: MSH-8
title: Implement castling and en passant generation and reversal
status: Ready
assignee: []
created_date: '2026-10-22T13:27:00Z'
labels:
  - rules
  - special-moves
  - undo
milestone: m-1
dependencies:
  - MSH-7
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 8000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Implement the chess special moves whose legality and effects cannot be reduced to an ordinary single-piece capture.

## Design references

A-02; A-03; M1 special move categories

## Scope

Implement castling movement/rights/attack constraints and en passant timing, victim-square effects, relevant occupancy changes, and complete undo. Integrate their candidate generation with the preceding work.

## Verification expectations

Use independent cases for both castling sides, lost rights, attacked transit positions, blocked paths, en passant expiry, and en passant discovered attacks. Compare exact post-state and restored pre-state.

## Out of scope

Chess960, universal multi-leg variant support, and weakening the adopted chess validation policy.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Castling handles the relevant pieces, rights, occupancy, and king-safety conditions under the adopted orthodox profile.
- [ ] #2 En passant removes the correct off-destination victim and handles its availability and occupancy effects.
- [ ] #3 Illegal special actions are rejected at the checked boundary without changing caller state.
- [ ] #4 Every special-action mutation and derived field is restored exactly by undo.
- [ ] #5 Independent legality and restoration cases are recorded as deterministic regressions.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
