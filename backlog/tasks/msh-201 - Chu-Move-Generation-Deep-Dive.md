---
id: MSH-201
title: Chu Search Speed
status: To Do
assignee: []
created_date: '2026-11-19T07:12:00Z'
updated_date: '2026-11-20T22:48:00Z'
labels:
  - chu
  - search
milestone: m-5
dependencies: []
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
