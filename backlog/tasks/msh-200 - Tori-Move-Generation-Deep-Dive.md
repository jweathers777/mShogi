---
id: MSH-200
title: Tori Move Generation Deep Dive
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T07:12:00Z'
updated_date: '2026-11-20T17:05:00Z'
labels:
  - handle:tori-move-generation-deep-dive
  - tori
  - search
milestone: m-5
dependencies: []
references:
  - docs/m5/strength-roadmap.md
ordinal: 207000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Move Generation Deep Dive

**Problem.** Tori Shogi has pieces whose moves are asymmetric or unusual (Quail, Pheasant, Crane, Falcon, Phoenix) and drops. Fast, correct generation of these moves limits search speed and correctness, and the shared core's techniques assume ordinary moves.

**Objective.** With John, examine each piece of Tori that needs non-standard generation and choose the best way to generate, order and update its moves for search.

**Approach.** A discussion task first (John, 2026-11-19): list the pieces and their move rules from the native profile, measure the current generation cost, and compare candidate representations (for example precomputed tables, attack sets, special encodings for multi-step moves). Implementation follows in this task or in separate tasks as agreed. Native rules stay the authority; a faster search-side generator is validated by perft and differential tests against them.

**Completion.** The pieces, measured costs, options and the agreed decision are recorded; any agreed implementation passes perft and differential tests.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each Tori piece with non-standard generation is listed with its native move rule and its measured generation cost.
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Gather native rules (T02-T04) and the search-side generation code. 2. Measure generation, make/unmake, attack test, perft and search speed per piece and drop on real game positions, suite and openings, in the cloud (tori_strength movegen; one c3-standard-4 VM). 3. Write the discussion paper docs/m5/tori-move-generation.md with options and a recommendation. 4. John decides. 5. Implement the agreed set with perft/differential tests, identical fixed-depth output, speed and (proposed) a predecessor game test in the cloud.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-20 measurements (cloud c3-standard-4, build e6df0d79, job msh200-movegen-1, 106 s, about US$0.01, cleanup verified; evidence docs/m5/evidence/msh200): real game positions have 57 candidate moves, 64% drops; generate-all 234 ns (4 ns/move); per-piece 3-4 ns (step pieces), 10-18 ns (Eagle, Quails); attacked(Phoenix) 20 ns, called twice per make; make+unmake 59-71 ns; perft 94 ns/leaf; search depth 6 1.22 M nodes/s. Finding: trying moves (two attack tests per make) dominates, not generation. Options A-E and recommendation in docs/m5/tori-move-generation.md; awaiting John's decision.
<!-- SECTION:NOTES:END -->
