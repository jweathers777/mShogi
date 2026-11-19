---
id: MSH-205
title: Tori Selectivity Round
status: Ready
assignee: []
created_date: '2026-11-19T19:43:00Z'
labels:
  - handle:tori-selectivity-round
  - tori
  - strength
milestone: m-5
dependencies:
  - MSH-204
references:
  - docs/m5/strength-roadmap.md
ordinal: 212000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Selectivity Round

**Problem.** Tori uses the shared core with default hooks; speed, selectivity and drop handling have not been tuned as for Shogi (MSH-190) and Xiangqi (MSH-195, MSH-203).

**Objective.** The selectivity step of the program for Tori, including search speed where the harness shows it pays.

**Approach.** Coherent set of shared-core techniques adapted to Tori (quiescence checks, SEE with drops, scaled margins, mate search, ordering) plus clear speed fixes from profiling; informed by MSH-200 (Tori Move Generation Deep Dive) where John has decided.

**Completion.** Each technique has a recorded outcome; kept changes pass correctness checks and the sequential predecessor test (H1 about 15).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each adopted or rejected technique and speed change has a recorded outcome.
- [ ] #2 Perft, native differential and history tests pass; tactical suite recorded.
- [ ] #3 The set passes the sequential predecessor test or the null result is reported and reverted; independent review recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
