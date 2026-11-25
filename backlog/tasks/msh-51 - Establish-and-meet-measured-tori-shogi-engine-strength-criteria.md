---
id: MSH-51
title: Establish and meet measured tori shogi engine strength criteria
status: Done
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-25T07:39:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-207
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
ordinal: 176000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Bring the Tori Shogi engine to its classical baseline through the per-game strength program in docs/m5/strength-roadmap.md (D-09). John decides when the baseline is reached. The baseline is the start point for learned evaluation (MSH-55, MSH-135).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The program steps in docs/m5/strength-roadmap.md are done, or skipped with a recorded reason.
- [x] #2 John's stop decision is recorded, with the accepted baseline revision.
- [x] #3 The evidence and its limits are recorded in the step tasks; no target or rating is invented.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Status check, 2026-11-15: this task depends on MSH-50 (Shogi) and MSH-54 (Xiangqi) because of the 2026-11-08 order of games. That order is no longer the recorded direction (see the description). The dependencies stay until John decides the order.

Decision, 2026-11-15: John confirmed the order Shogi and Xiangqi first. The dependencies on MSH-50 (Shogi) and MSH-54 (Xiangqi) stay as recorded.

Direction, 2026-11-18 (John): this game follows the per-game classical strength program in docs/m5/strength-roadmap.md (harness, selectivity round, evaluation round, approved anchor, blunder screen, stop decision), with the same round rules as Chess, Shogi and Xiangqi. Adjust only for the game: few or no rated opponents exist, so the anchor step becomes a recorded limitation and the evidence comes from frozen-predecessor tests, tactical suites and, if no stronger engine exists, a deeper search of mShogi as a same-family judge. Create the per-step tasks when this game starts; John selects when.

Direction, 2026-11-19 (John): do not look for external opponents or judges for this game. Optimize the search as far as classical techniques allow, adapted to the game. Evidence: frozen-predecessor tests, tactical suites, a scaling curve (strength against depth or time) and blunder rates judged by a much deeper mShogi search (same-family, weaker evidence; say so). See docs/m5/strength-roadmap.md, 'Games after Xiangqi: no opponent search'. This replaces the anchor step of the 2026-11-18 note.

Move generation discussion slot: MSH-200 (Tori Move Generation Deep Dive).

2026-11-19 (John: "Let's move on to Tori Shogi now."): program step tasks MSH-204 (Tori Strength Harness), MSH-205 (Tori Selectivity Round), MSH-206 (Tori Evaluation Round), MSH-207 (Tori Strength Assessment, no external opponents); MSH-200 (Tori Move Generation Deep Dive) is the discussion slot with John.

Owner decision (John, 2026-11-21): the current classical Tori engine (main e012786, MSH-206 fit2 evaluation) is the accepted baseline for later learned-evaluation experiments; Tori strength work stops here and further strength work is deferred to learned evaluation (MSH-55). Evidence (MSH-207): no external opponents (Games after Xiangqi rule); scaling curve 2x time +346 Elo at 1.25 s to +261 at 20 s; same-family judge 0.49 clear blunders and 1.49 mistakes per 100 moves at 5 s + 0.1 s; the one fix round (drop_refutation) was rejected at -55 Elo. Suite tools/strength/data/tori-blunders-v1.json.

2026-11-25 (John): the October criteria (independent opponent pool, release targets met in matches) are replaced by the strength program criteria above, as done earlier for MSH-49 (Chess). This task is closed at the accepted baseline.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Baseline accepted by John on 2026-11-21: main e012786 (MSH-206 fitted evaluation).
- Steps: harness MSH-204; speed MSH-200; selectivity MSH-205; evaluation MSH-206; assessment MSH-207. No external opponents (no anchor step), as John directed for games after Xiangqi.
- Scaling: 2x time gives +346 Elo at 1.25 s, falling to +261 at 20 s.
- Same-family judge: 0.49 clear blunders and 1.49 mistakes per 100 moves at 5 s + 0.1 s. This is weaker evidence than an independent judge.
- The one fix round (drop_refutation) was rejected at -55 Elo. Suite tools/strength/data/tori-blunders-v1.json.
- Further strength work goes to learned evaluation (MSH-55, MSH-135).
- Review: no separate review applies to this decision record; the step tasks hold their own checks.
<!-- SECTION:FINAL_SUMMARY:END -->
