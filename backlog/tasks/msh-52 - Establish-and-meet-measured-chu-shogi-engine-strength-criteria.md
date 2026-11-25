---
id: MSH-52
title: Establish and meet measured chu shogi engine strength criteria
status: Done
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-25T07:39:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-43
  - MSH-109
  - MSH-50
  - MSH-54
  - MSH-182
  - MSH-201
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
ordinal: 177000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Bring the Chu Shogi engine to its classical baseline through the per-game strength program in docs/m5/strength-roadmap.md (D-09). John decides when the baseline is reached. The baseline is the start point for learned evaluation (MSH-55, MSH-135).
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

Status check, 2026-11-15: the move of this game to the shared search core is now the separate task MSH-182. This task keeps the strength criteria, opponent pool and measurement, and depends on MSH-182.

Direction, 2026-11-18 (John): this game follows the per-game classical strength program in docs/m5/strength-roadmap.md (harness, selectivity round, evaluation round, approved anchor, blunder screen, stop decision), with the same round rules as Chess, Shogi and Xiangqi. Adjust only for the game: few or no rated opponents exist, so the anchor step becomes a recorded limitation and the evidence comes from frozen-predecessor tests, tactical suites and, if no stronger engine exists, a deeper search of mShogi as a same-family judge. Create the per-step tasks when this game starts; John selects when.

Direction, 2026-11-19 (John): do not look for external opponents or judges for this game. Optimize the search as far as classical techniques allow, adapted to the game. Evidence: frozen-predecessor tests, tactical suites, a scaling curve (strength against depth or time) and blunder rates judged by a much deeper mShogi search (same-family, weaker evidence; say so). See docs/m5/strength-roadmap.md, 'Games after Xiangqi: no opponent search'. This replaces the anchor step of the 2026-11-18 note.

Move generation discussion slot: MSH-201 (Chu Move Generation Deep Dive).

2026-11-20 (John): depends on MSH-201 (Chu Move Generation Deep Dive). Chu program step tasks (selectivity and later), when created, depend on MSH-201.

John started the Chu selectivity step on 2026-11-24 after MSH-201. Tracked as MSH-52.1 (Chu Selectivity Round); later evaluation and measurement are not started.

MSH-52.1 (Chu Selectivity Round) completed: combined quiescence/ordering candidate rejected by predecessor SPRT (41W/117L,79 scored pairs,10 unscored cutoffs); ordering-only pilot also rejected. No search changes kept; accepted MSH-201 engine remains the baseline. All five technique outcomes and evidence are in MSH-52.1. Next step is Chu evaluation; it has not been started.

MSH-52.2 (Chu Evaluation Round): fitted classical material/placement/tempo weights passed the frozen predecessor SPRT0/10 with164W/0D/74L across119 scored pairs; five pairs unscored. Heldout MSE improved1.63%, forcing suite70/71, elementary315/315 and igui80/80. Next Chu steps are the internal scaling curve and much-deeper same-family blunder screen, then stop decision. No external opponents or absolute rating claim.

John requested internal scaling and the deeper-search screen after evaluation. Active work is MSH-52.3 (Chu Strength Assessment). The accepted fitted engine is frozen; the parent stop/continue decision remains pending.

2026-11-25 (John): the October criteria (independent opponent pool, release targets met in matches) are replaced by the strength program criteria above, as done earlier for MSH-49 (Chess). This task is closed at the accepted baseline.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Baseline accepted by John on 2026-11-25: e2658c7 (MSH-52.2 fitted evaluation).
- Steps: harness MSH-213; speed MSH-201; selectivity MSH-52.1 (no change kept); evaluation MSH-52.2 (fitted weights passed the predecessor SPRT, 164W/0D/74L); assessment MSH-52.3. No external opponents, as John directed for games after Xiangqi.
- Scaling (2x time): +492 Elo at 5 s, +372 at 10 s, +597 at 20 s. The 2.5 s and 40 s points are not usable. These are relative estimates on fully scored pairs only.
- Same-family deeper judge (median depth 12 against 5): 0 clear blunders in 512 sampled moves (95% upper bound 0.58 per 100), 14 mistakes, no fix round.
- Further strength work goes to learned evaluation (MSH-55, MSH-135). Follow-up: MSH-238 (Remove the fixed ten-minute experiment cap).
- Review: no separate review applies to this decision record; the step tasks hold their own checks.
<!-- SECTION:FINAL_SUMMARY:END -->
