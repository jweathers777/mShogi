---
id: MSH-4
title: Implement explicit chess position state and the FEN boundary
status: Ready
assignee: []
created_date: '2026-10-22T13:26:00Z'
labels:
  - rules
  - state
  - parsing
milestone: m-1
dependencies:
  - MSH-2
  - MSH-3
documentation:
  - docs/design.md
  - AGENTS.md
priority: high
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Represent all M1-relevant chess state explicitly and make positions reproducible through a well-defined input/output boundary.

## Design references

A-02; A-03; A-04; M1 rules profile

## Scope

Implement board/side state, castling rights, en passant state, relevant counters, explicit validation, and FEN parsing/serialization under the adopted profile. Keep history requirements and match state distinct. Prefer understandable state over speculative compression.

## Verification expectations

Use valid and malformed independent fixtures, semantic round trips, position invariants, and explicit error cases. Check that failed external input leaves existing caller state unchanged.

## Out of scope

A universal six-game FEN format, claims that a FEN establishes repetition history, final public ABI, and move generation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All M1-relevant position fields have explicit meanings and invariants documented in the code or profile.
- [ ] #2 Valid profile fixtures parse and serialize with the defined semantic equivalence.
- [ ] #3 Malformed or unsupported input produces clear deterministic errors without partially mutating an existing position.
- [ ] #4 The code does not imply that board/FEN equality establishes match-history or repetition equivalence.
- [ ] #5 Tests cover boundary fields and validation policy, with independent fixture expectations rather than only self-round-trips.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
