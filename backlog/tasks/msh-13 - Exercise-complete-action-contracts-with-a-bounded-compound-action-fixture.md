---
id: MSH-13
title: Review real action boundaries with a bounded compound-action fixture
status: Ready
assignee: []
created_date: '2026-10-22T13:27:00Z'
updated_date: '2026-10-22T13:35:00Z'
labels:
  - architecture
  - variants
  - verification
milestone: m-1
dependencies:
  - MSH-5
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 13000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Catch public-interface assumptions that would prevent the promised larger games by reviewing the position and transition boundaries the chess implementation actually establishes.

## Design references

P-01; A-02; A-03; M1 compound-action exercise; MSH-4 and MSH-5 implementation boundaries

## Scope

Identify the concrete position, action, transition, and undo types/functions implemented by MSH-4 and MSH-5. Use a small explicitly synthetic fixture to explore a board index beyond chess size, distinct complete actions with equal endpoints but different routes, multiple rule-derived effects, atomic rejection, and complete undo. Exercise applicable production interfaces directly. Where a chess-specific type intentionally remains chess-specific, provide an explicit reviewed mapping to a future adapter and identify the changes that mapping requires. A disconnected toy model alone does not satisfy this task. Review how future interface serialization can preserve action identity without freezing its final schema.

## Verification expectations

Record the concrete production boundaries, which the fixture exercises, which it does not, and how each synthetic case maps to them. Use executable checks for the synthetic transition/effect cases, with independently inspected expected outcomes. Distinguish fixture evidence from claims about existing production behavior. Resolve any conflict with A-02/A-03 or create an explicit blocking follow-up and make MSH-14 depend on it before milestone closure. Keep this separate from supported-game claims.

## Out of scope

Implementing Chu or Tenjiku, inventing their rules, building a general rules DSL, requiring a universal action type, forcing chess to use a 256-square representation, or freezing the final public wire format.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The review names the actual position/action/transition/undo boundaries from MSH-4 and MSH-5, states which are intentionally chess-specific, and maps every synthetic case to those boundaries or a concrete future adapter.
- [ ] #2 A square beyond the chess range and two complete actions with equal endpoints but distinct routes remain distinguishable in the reviewed mapping; applicable shared production interfaces are exercised directly.
- [ ] #3 Executable synthetic checks demonstrate multiple rule-derived effects, complete reversal, and atomic rejection; the evidence explicitly distinguishes fixture-only behavior from production guarantees.
- [ ] #4 The mapping distinguishes partial UI selection from a committed complete action, preserves action identity for future serialization, and does not make client-provided effects authoritative.
- [ ] #5 Required changes and limits are recorded; any actual conflict with A-02/A-03 is resolved or tracked by a follow-up that blocks MSH-14, and no additional real game is claimed as implemented or validated.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
