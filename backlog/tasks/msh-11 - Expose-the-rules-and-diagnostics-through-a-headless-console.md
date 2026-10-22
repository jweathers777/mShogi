---
id: MSH-11
title: Expose the rules and diagnostics through a headless console
status: Ready
assignee: []
created_date: '2026-10-22T13:27:00Z'
labels:
  - console
  - diagnostics
  - integration
milestone: m-1
dependencies:
  - MSH-9
  - MSH-10
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Allow a developer or user to reproduce rule behavior from commands without starting a GUI.

## Design references

P-03; P-05; A-01; M1 console

## Scope

Support loading/inspecting a chess position, listing legal actions, applying/undoing a sequence, reporting check/mate/stalemate, and running perft/divide. Define actual help, output, error, and exit behavior; add an interactive shell only as needed to satisfy the workflow.

## Verification expectations

Exercise real process invocations and a representative load/list/apply/undo/perft sequence. Check stdout/stderr and exit behavior as specified, including malformed input and a rejected action.

## Out of scope

Claimed UCI/USI compatibility, search, clocks, GUI, and a speculative universal machine protocol.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Documented console commands expose every promised M1 diagnostic through the real binary.
- [ ] #2 A position can be loaded, moved, undone, and inspected with state matching the library contract.
- [ ] #3 Malformed inputs and illegal actions produce clear errors while preserving relevant session state.
- [ ] #4 The console runs without UI, database, or Python dependencies.
- [ ] #5 README examples and a bounded process-level smoke check reproduce the implemented behavior.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
