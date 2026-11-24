---
id: MSH-201
title: Chu Search Speed
status: In Progress
assignee:
  - codex
created_date: '2026-11-19T07:12:00Z'
updated_date: '2026-11-24T15:57:00Z'
labels:
  - chu
  - search
milestone: m-5
dependencies:
  - MSH-213
references:
  - docs/m5/strength-roadmap.md
ordinal: 208000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: make the Chu search faster with conventional techniques. Measure where search time goes (move generation, make and unmake, legality and check tests, attack tests), then remove the largest costs. This includes the unusual pieces (Lion double moves and igui, Lion-like moves of the Horned Falcon and Soaring Eagle, and the Lion-capture rules). Keep a change only on evidence: correctness checks, a speed measurement, and a game test against the frozen predecessor.

Native rules stay the authority. A faster search-side position must agree with them in perft and differential tests. No rule changes.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Baseline and final speed are recorded on the same machine and positions: nodes per second and time to a fixed depth, with the cost profile that guided the changes
- [ ] #2 Chu perft and differential tests against the native rules pass
- [ ] #3 Each kept change passed the blunder suite and a game test against the frozen predecessor; rejected changes are recorded with their result
- [ ] #4 A short design note in docs/ explains how the unusual pieces are generated and checked in search
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-20, John: rescoped from a discussion task to an ordinary speed task; agents choose the techniques. Blocker: Chu has no strength harness yet (opening set, blunder suite, game test against the predecessor; see MSH-204 (Tori Strength Harness) and docs/m5/strength-roadmap.md). Speed profiling and perft/differential work can start; AC 3 needs the harness.

Harness blocker is now the dependency MSH-213 (Chu Strength Harness).

Started from main182f017 after MSH-213 completion. Frozen predecessor engine source4f5e2ed and harness inputs/results are recorded in MSH-213. No compatibility obligation or game-rule change. Budget US$10; measurements and heavy builds use c3-standard-4 cloud workers.
<!-- SECTION:NOTES:END -->
