---
id: MSH-12
title: Add sequence properties and independent differential verification
status: Ready
assignee: []
created_date: '2026-10-22T13:27:00Z'
labels:
  - verification
  - properties
  - differential
milestone: m-1
dependencies:
  - MSH-2
  - MSH-9
  - MSH-10
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Extend focused tests with sequence-level evidence that detects state drift and move-generation disagreements.

## Design references

P-05; A-02; A-03; verification contract

## Scope

Generate bounded legal action sequences; test exact reversal and state serialization; compare recomputed derived state where present; add a bounded differential comparison with an independently implemented chess reference. Persist minimized or simplified failures as replayable regressions.

## Verification expectations

Record fixed seeds, limits, reference implementation/version, and reproducible commands. Separate a fast default suite from deeper optional or scheduled checks. Review licensing before adding an external test dependency.

## Out of scope

Deferring all tests to this ticket, treating agreement between two implementations as proof of full rules, and unbounded randomized CI runs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Fixed-seed legal sequences verify apply/undo restoration of every relevant state field.
- [ ] #2 Serialization and any incremental-versus-recomputed state checks run over meaningful generated positions.
- [ ] #3 A bounded independent differential comparison covers the documented position corpus and reports actionable disagreements.
- [ ] #4 Failures can be saved and replayed deterministically without requiring the original random run.
- [ ] #5 Fast and deeper verification commands have explicit purposes and limits, with no unsupported platform or completeness claim.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
