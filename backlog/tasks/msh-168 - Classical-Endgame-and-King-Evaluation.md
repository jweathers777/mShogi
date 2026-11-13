---
id: MSH-168
title: Classical Endgame and King Evaluation
status: To Do
assignee: []
created_date: '2026-11-13T21:29:00Z'
updated_date: '2026-11-13T22:01:00Z'
labels:
  - handle:classical-endgame-king-evaluation
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: high
ordinal: 172000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Classical Endgame and King Evaluation.

**Problem:** In MSH-161 (Competitive Gameplay Diagnosis) the engine gave back endgame advantages about twice as often as its opponent, and it declined drawing simplifications while its own score was strongly negative in level positions. In level positions with little material, 44 percent of its own scores were 100 centipawns or more from zero. Pawn pushes next to the own king were four times more frequent in confirmed collapses than in all moves.

**Objective:** Make the Chess evaluation agree better with the true result in endgames and in king-safety decisions, after the search is deep.

**Approach:** Recognizers and scale factors for drawn or hard-to-win material (rook and bishop against rook, opposite-colored bishops, wrong rook pawn, no mating material). Passed-pawn, king-distance and pawn-race terms. King-shelter and pawn-storm terms. Accept each group on the game test. Tune the term weights by regression on labeled positions (Texel tuning, authorized by John on 2026-11-13), with a recorded data source and a held-out check, and confirm each tuned set by a game test. No neural-network learned evaluation. No tablebase in this task.

**Completion:** The accepted terms are the default, and the endgame measures from the review improve.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each term group has a recorded game-test result and an accept or reject decision
- [ ] #2 On the recorded MSH-161 level endgame positions, the share of own scores 100 centipawns or more from zero is clearly lower than the baseline
- [ ] #3 The blunder suite result for endgame positions is better than at the start of the task
- [ ] #4 The accepted evaluation is stronger than the engine at the start of the task in a game test
- [ ] #5 Weight tuning uses a recorded position source and method, and the tuned weights are confirmed by a game test
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
