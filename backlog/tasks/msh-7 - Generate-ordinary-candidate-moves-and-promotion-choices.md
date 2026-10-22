---
id: MSH-7
title: Generate ordinary candidate moves and promotion choices
status: Ready
assignee: []
created_date: '2026-10-22T13:26:00Z'
labels:
  - rules
  - move-generation
milestone: m-1
dependencies:
  - MSH-5
  - MSH-6
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Generate the complete intended ordinary chess candidate set with an explicit contract before final king-safety filtering.

## Design references

A-01; A-02; M1 move categories

## Scope

Implement ordinary piece moves, pawn single/double advances and captures, and all promotion choices. Define whether generated candidates are pseudo-legal and what final filtering still must establish.

## Verification expectations

Compare candidate sets with hand-reviewed fixtures, including blocked movement, edge positions, both colors, and distinct underpromotions. Test absence of duplicates and prohibited own-piece/king captures.

## Out of scope

Castling, en passant, final legal-move claims, search ordering, and micro-optimization without evidence.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Candidate generation covers the intended ordinary movement and promotion categories for both colors.
- [ ] #2 Blocked and edge cases match independent expected sets and do not produce duplicates.
- [ ] #3 All promotion choices remain distinct actions throughout generation and transition.
- [ ] #4 The candidate-versus-legal contract is explicit and callers cannot mistake this intermediate API for complete legality.
- [ ] #5 Relevant transition and attack tests remain passing.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
