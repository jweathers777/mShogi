---
id: MSH-207
title: Tori Strength Assessment
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T19:43:00Z'
updated_date: '2026-11-21T18:52:00Z'
labels:
  - handle:tori-strength-assessment
  - tori
  - strength
milestone: m-5
dependencies:
  - MSH-206
references:
  - docs/m5/strength-roadmap.md
ordinal: 214000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Strength Assessment

**Problem.** Tori has no rated or strong external opponents; John decided (2026-11-19) not to search for them.

**Objective.** The measurement and blunder-screen steps of the program for Tori without external engines, then John's stop-or-continue decision.

**Approach.** A scaling curve (strength against time or depth) from self-play time odds; a blunder screen judged by a much deeper mShogi search (same-family, weaker evidence; stated as such), with rules fixed before results.

**Completion.** Report with scaling curve, judged move quality, blunder classes and a blunder suite; John's decision recorded.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Scaling curve and judged move quality reported with their limits (same-family judge).
- [ ] #2 Blunder rule, classes and fix rule fixed before results; blunder suite with a held-out split; independent review recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
From MSH-206 (Tori Evaluation Round) review: many fitted table weights sit at the +-150 bound (for example Eagle columns 0 and 2 at -150, Phoenix ranks 0 and 6 at -150). At depth 1, a quiet Falcon promotion to an Eagle on a good column scores above taking a free Swallow with promotion. Check in the blunder screen whether such table preferences cause blunders (rarely visited squares fitted on little data).
<!-- SECTION:NOTES:END -->
