---
id: MSH-5
title: Implement reversible ordinary actions and promotions
status: Ready
assignee: []
created_date: '2026-10-22T13:26:00Z'
labels:
  - rules
  - state
  - undo
milestone: m-1
dependencies:
  - MSH-4
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Establish an explicit transition and undo contract before legal move generation depends on it.

## Design references

A-02; A-03; M1 state restoration

## Scope

Implement quiet moves, ordinary captures, promotion choices, side/counter updates, relevant rights updates, and complete undo records. Distinguish a checked external-action boundary from any trusted internal transition primitive.

## Verification expectations

Use hand-reviewed expected before/after states and exact restoration checks. Cover captured rook/king-related rights where applicable, promotions, and all mutated counters/derived fields.

## Out of scope

Castling and en passant execution, a universal variant event engine, and accepting arbitrary client-supplied capture lists.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Ordinary moves, captures, and all chess promotion choices produce independently expected resulting state.
- [ ] #2 Undo restores every affected position field and any derived cache exactly.
- [ ] #3 Transition preconditions and checked-versus-trusted entry points are explicit; the public boundary cannot partially apply a rejected action.
- [ ] #4 The rules determine capture effects; an untrusted victim list is not authoritative.
- [ ] #5 Focused regression tests accompany the transitions and expose state differences clearly.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
