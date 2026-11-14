---
id: MSH-174
title: Legacy Complete Moves
status: Ready
assignee: []
created_date: '2026-11-14T14:14:00Z'
labels:
  - handle:legacy-complete-moves
milestone: m-5
dependencies:
  - MSH-173
references:
  - docs/m4/chu/profile.md
  - docs/m4/chu/notation.md
priority: medium
type: task
ordinal: 178000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Legacy Complete Moves.

**Problem**
The original console prints moves using Notator::Notate, which omits the intermediate square stored in Move::mVisited. Lion routes with the same endpoints can have different effects. Parsing that text cannot support reliable Chu matches.

**Objective**
Expose complete proposed and incoming Chu actions through a small headless interface around the frozen legacy engine.

**Approach**
Use the original Move data, including origin, ordered landings and promotion, rather than reconstructing a route from endpoints or printed notation. Keep interface-only changes separate from the frozen search/evaluation/rules. Define explicit readiness, search completion, accepted/rejected input and ending/error responses. Search limits, stopping behavior and unsupported operations must be observable. Reuse a suitable existing transport where practical; do not build a general variant framework.

**Completion**
The wrapped opponent can start Chu, consume complete action history and return complete choices with bounded machine-readable I/O. Focused tests and independent review cover the conversion. The unchanged original revision is 941cd4dc37e6e6210d4f993c96a553753e228b19.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 An explicit versioned interface identifies the wrapped source/build, starts a fresh Chu game, accepts complete actions or replayed history, and returns actual engine choices without scraping human display output.
- [ ] #2 Ordinary actions, optional promotion, ordered double captures, capture-return and quiet return preserve full move identity; ambiguous or unrepresentable actions produce typed refusal without guessing.
- [ ] #3 Incoming action acceptance is acknowledged and rejected inputs preserve the prior legacy state; tests compare expected board effects and histories for distinguishing routes.
- [ ] #4 Readiness, search limits/completion, no-move results, legacy conclusions, errors and stop/quit behavior are distinct; diagnostics cannot corrupt protocol output and blocking work remains externally bounded.
- [ ] #5 Independent expected fixtures and negative tests cover route/promotion conversion and failure behavior; interface changes do not silently fix or alter legacy search, evaluation or rules, and independent review is recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
