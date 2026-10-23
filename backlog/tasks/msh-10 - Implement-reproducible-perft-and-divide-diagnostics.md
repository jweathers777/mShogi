---
id: MSH-10
title: Implement reproducible perft and divide diagnostics
status: Ready
assignee: []
created_date: '2026-10-22T13:27:00Z'
updated_date: '2026-10-23T18:05:00Z'
labels:
  - diagnostics
  - perft
  - verification
milestone: m-1
dependencies:
  - MSH-9
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Make move-generation disagreements reproducible and localizable to a branch of the move tree.

## Design references

P-05; M1 diagnostics; verification contract

## Scope

Add perft and stable per-root-action divide output with explicit depth semantics, validated inputs, and useful failure reporting. Consume independently established fixture counts.

## Verification expectations

Run the selected reference corpus at bounded documented depths and compare expected totals/divisions where available. Confirm that diagnostics restore the starting state and handle zero/invalid depth and malformed positions.

## Out of scope

Search strength measurement, comparing arbitrary variants through a chess-only representation, and using newly generated mShogi counts as independent expected answers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Perft totals match the independently established corpus at documented depths.
- [ ] #2 Divide output identifies complete root actions in deterministic order and supports locating the first disagreement.
- [ ] #3 Running diagnostics preserves the complete starting state.
- [ ] #4 Depth/input semantics and failure output are documented and tested.
- [ ] #5 A repeatable performance baseline is recorded with build and hardware context without turning it into a strength claim.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Input from MSH-9 review (2026-10-23): legal_actions() treats legality as independent of the u32 counter domain (pending owner approval in MSH-9). Perft from a position at the counter limit therefore cannot be computed by applying listed actions at depth 2 or more. MSH-10 must define this explicitly; normalizing counters on a private copy, as legal_actions does, is the consistent choice.
<!-- SECTION:NOTES:END -->
