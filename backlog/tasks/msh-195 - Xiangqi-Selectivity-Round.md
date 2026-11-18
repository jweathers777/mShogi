---
id: MSH-195
title: Xiangqi Selectivity Round
status: Ready
assignee: []
created_date: '2026-11-18T19:25:00Z'
labels:
  - handle:xiangqi-selectivity-round
  - xiangqi
  - strength
milestone: m-5
dependencies:
  - MSH-194
references:
  - docs/m5/strength-roadmap.md
ordinal: 202000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Selectivity Round

**Problem.** Xiangqi uses only the default shared-core search hooks: no quiescence checks, a one-step exchange estimate, default margins and no mate search. Checks and forcing sequences are frequent in Xiangqi.

**Objective.** The selectivity step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Xiangqi.

**Approach.** Adopt the shared-core techniques of MSH-190 (Shogi Selectivity Round) that fit Xiangqi: bounded quiescence check stage, full SEE (Cannon screens and Horse legs make it differ from Chess), Xiangqi-scaled margins, move ordering, and the shared df-pn mate search if it pays. Test as one coherent set against the frozen predecessor.

**Completion.** Each technique has a recorded outcome; kept changes pass correctness checks, the tactical suite and the sequential predecessor test.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each adopted or rejected technique has a recorded outcome with its reason.
- [ ] #2 Perft, native differential and history tests pass; the tactical suite score is recorded against the baseline.
- [ ] #3 The kept set passes the sequential pentanomial predecessor test (H1 about 15 Elo) at the harness control, with compute recorded; a null result is reported and reverted.
- [ ] #4 Independent review is recorded and findings are resolved.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
