---
id: MSH-207
title: Tori Strength Assessment
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T19:43:00Z'
updated_date: '2026-11-21T19:11:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Approved by John 2026-11-21: caps US$8 and 6 hours (cloud only). Rules below are fixed before any result.
Frozen engine: main c7be8c6 (MSH-206 fit2). The cloud build uses the task branch, whose engine code equals main (only tools differ).
1. Scaling curve (campaign msh207-assess): self-play time odds, the candidate has twice the clock, at base 1.25, 2.5, 5, 10 and 20 s with increment base/50; 96 pairs per clock (distinct seeds, so distinct openings). Report Elo of doubling with 95% interval per clock. Same campaign: 50 control pairs at 5 s + 0.1 s (the games to judge).
2. Judged move quality (campaign msh207-judge): tori_strength judge searches every position of the 100 control games after the opening with mShogi at 5,000,000 nodes (the search maximum, about 25x a game move), fresh table, full game history. Same-family judge: weaker evidence than an independent engine; errors both share stay unseen. Report clear blunders and mistakes per 100 moves and mean win-probability loss.
3. Rules (tools/strength/blunder_rules.py, shared with MSH-193 and MSH-198): p = 1/(1+exp(-cp/600)), mate 1 or 0, concluded position won or lost; clear blunder p_before >= 0.30 and p_after <= p_before - 0.30; mistake p_before >= 0.20 and loss >= 0.15.
4. Classes, first match (tools/strength/tori_assessment.py): time_trouble (clock < 1,000 ms before the move), repetition (position before the move occurred earlier in the game), allowed_short_mate (judge mates the mover within 9 plies), missed_capture (judge's move captures, the played move does not; the MSH-206 table concern), drop_refutation, refuted_major_capture (Falcon, Eagle, Crane, Quail), refuted_minor_capture (Pheasant, Goose, Swallow), other.
5. Fix rule: one round only if one class other than other has >= 30% of the clear blunders and maps to a general change; game test against the predecessor (H1 15 search, 10 evaluation). Otherwise report and stop.
6. Suite tools/strength/data/tori-blunders-v1.json: every clear blunder and mistake position, held-out by game (sha256 % 4 == 0).
7. Independent review; John's stop-or-continue decision (MSH-51).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
From MSH-206 (Tori Evaluation Round) review: many fitted table weights sit at the +-150 bound (for example Eagle columns 0 and 2 at -150, Phoenix ranks 0 and 6 at -150). At depth 1, a quiet Falcon promotion to an Eagle on a good column scores above taking a free Swallow with promotion. Check in the blunder screen whether such table preferences cause blunders (rarely visited squares fitted on little data).
<!-- SECTION:NOTES:END -->
