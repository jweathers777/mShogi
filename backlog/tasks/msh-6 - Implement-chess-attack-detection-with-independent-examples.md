---
id: MSH-6
title: Implement chess attack detection with independent examples
status: Ready
assignee: []
created_date: '2026-10-22T13:26:00Z'
labels:
  - rules
  - attacks
milestone: m-1
dependencies:
  - MSH-4
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 6000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Provide a precise attack query that legal move validation can rely on.

## Design references

A-01; A-04; M1 king safety

## Scope

Implement pawn, knight, king, and sliding attacks with occupancy and board-edge handling. Document attack semantics separately from complete legal move generation.

## Verification expectations

Use independent fixtures for boundaries, blockers, pins, king adjacency, and relevant occupancy changes. Test each piece category and both sides.

## Out of scope

Search evaluation, generalized royal-piece logic for all variants, and using the same generator as both implementation and oracle.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Attack queries cover all chess piece categories and both colors under documented semantics.
- [ ] #2 Sliding blockers and board boundaries are correct in curated independent cases.
- [ ] #3 Attack semantics do not accidentally exclude an attack merely because moving the attacking piece would expose its own king.
- [ ] #4 Tests include king adjacency and occupancy-sensitive examples needed by subsequent special-move legality.
- [ ] #5 The implementation remains independent of search and GUI state.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
