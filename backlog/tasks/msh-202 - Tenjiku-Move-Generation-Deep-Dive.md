---
id: MSH-202
title: Tenjiku Move Generation Deep Dive
status: To Do
assignee: []
created_date: '2026-11-19T07:12:00Z'
labels:
  - handle:tenjiku-move-generation-deep-dive
  - tenjiku
  - search
milestone: m-5
dependencies: []
references:
  - docs/m5/strength-roadmap.md
ordinal: 209000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tenjiku Move Generation Deep Dive

**Problem.** Tenjiku Shogi adds the Fire Demon (area burning), range-jumping Generals (Great, Vice, Rook and Bishop Generals), Heavenly Tetrarchs and Lion-like pieces. Fast, correct generation of these moves limits search speed and correctness, and the shared core's techniques assume ordinary moves.

**Objective.** With John, examine each piece of Tenjiku that needs non-standard generation and choose the best way to generate, order and update its moves for search.

**Approach.** A discussion task first (John, 2026-11-19): list the pieces and their move rules from the native profile, measure the current generation cost, and compare candidate representations (for example precomputed tables, attack sets, special encodings for multi-step moves). Implementation follows in this task or in separate tasks as agreed. Native rules stay the authority; a faster search-side generator is validated by perft and differential tests against them.

**Completion.** The pieces, measured costs, options and the agreed decision are recorded; any agreed implementation passes perft and differential tests.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each Tenjiku piece with non-standard generation is listed with its native move rule and its measured generation cost.
- [ ] #2 Options and John's decision are recorded in the task.
- [ ] #3 Any agreed implementation passes perft and native differential tests and has a recorded speed measurement.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
