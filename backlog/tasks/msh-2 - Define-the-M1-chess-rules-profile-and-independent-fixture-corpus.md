---
id: MSH-2
title: Define the M1 chess rules profile and independent fixture corpus
status: Ready
assignee: []
created_date: '2026-10-22T13:26:00Z'
labels:
  - rules
  - reference
  - design
milestone: m-1
dependencies:
  - MSH-1
documentation:
  - docs/design.md
  - AGENTS.md
priority: high
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Establish what the first chess milestone promises and where independently expected answers come from before the implementation defines its own behavior.

## Design references

P-01; P-05; A-03; A-04; M1; verification contract

## Scope

Document a named orthodox-chess position profile, FEN validation policy, M1 rule coverage, outcome limitations, and a reusable fixture format with source/provenance. Curate expected move and perft examples covering ordinary moves, pins/checks, castling, en passant, promotions, mate, and stalemate. Document full-adjudication obligations deferred beyond M1.

## Verification expectations

Review primary rule sources and independently maintained reference cases. Record source version or retrieval details, precise expected values, and any discrepancies. At this stage fixtures may be plain data; executable consumption follows with the implementation.

## Out of scope

Writing the optimized move generator, treating legacy mShogi output as a rule oracle, or declaring full chess conformance.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The rules profile states M1 inclusion and exclusion explicitly, including the distinction between position-level terminal classification and full match adjudication.
- [ ] #2 FEN syntax, structural validation, analysis-position policy, and missing history are distinguished; serialization equivalence is defined semantically.
- [ ] #3 Fixture format includes rules profile, starting state, operation, independently established expectation, and provenance.
- [ ] #4 The corpus covers every M1 move category and meaningful king-safety cases, with expected legal moves or perft counts established independently of mShogi.
- [ ] #5 Unresolved rule questions and deferred draw/dead-position/claim obligations are visible in a coverage ledger and do not become silent implementation defaults.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
