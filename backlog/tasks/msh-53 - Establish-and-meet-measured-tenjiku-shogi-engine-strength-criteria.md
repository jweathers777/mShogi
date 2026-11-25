---
id: MSH-53
title: Establish and meet measured tenjiku shogi engine strength criteria
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-25T07:39:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-44
  - MSH-109
  - MSH-50
  - MSH-54
  - MSH-183
  - MSH-202
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
ordinal: 178000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Bring the Tenjiku Shogi engine to its classical baseline through the per-game strength program in docs/m5/strength-roadmap.md (D-09). John decides when the baseline is reached. The baseline is the start point for learned evaluation (MSH-55, MSH-135). No external opponents or judges (John, 2026-11-19). Both accepted Tenjiku rules profiles stay in scope.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The program steps in docs/m5/strength-roadmap.md are done, or skipped with a recorded reason.
- [ ] #2 John's stop decision is recorded, with the accepted baseline revision.
- [ ] #3 The evidence and its limits are recorded in the step tasks; no target or rating is invented.
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
Status check, 2026-11-15: this task depends on MSH-50 (Shogi) and MSH-54 (Xiangqi) because of the 2026-11-08 order of games. That order is no longer the recorded direction (see the description). The dependencies stay until John decides the order.

Decision, 2026-11-15: John confirmed the order Shogi and Xiangqi first. The dependencies on MSH-50 (Shogi) and MSH-54 (Xiangqi) stay as recorded.

Status check, 2026-11-15: the move of this game to the shared search core is now the separate task MSH-183. This task keeps the strength criteria, opponent pool and measurement, and depends on MSH-183.

Direction, 2026-11-18 (John): this game follows the per-game classical strength program in docs/m5/strength-roadmap.md (harness, selectivity round, evaluation round, approved anchor, blunder screen, stop decision), with the same round rules as Chess, Shogi and Xiangqi. Adjust only for the game: few or no rated opponents exist, so the anchor step becomes a recorded limitation and the evidence comes from frozen-predecessor tests, tactical suites and, if no stronger engine exists, a deeper search of mShogi as a same-family judge. Create the per-step tasks when this game starts; John selects when.

Direction, 2026-11-19 (John): do not look for external opponents or judges for this game. Optimize the search as far as classical techniques allow, adapted to the game. Evidence: frozen-predecessor tests, tactical suites, a scaling curve (strength against depth or time) and blunder rates judged by a much deeper mShogi search (same-family, weaker evidence; say so). See docs/m5/strength-roadmap.md, 'Games after Xiangqi: no opponent search'. This replaces the anchor step of the 2026-11-18 note.

Move generation discussion slot: MSH-202 (Tenjiku Move Generation Deep Dive).

2026-11-20 (John): depends on MSH-202 (Tenjiku Move Generation Deep Dive). Tenjiku program step tasks (selectivity and later), when created, depend on MSH-202.

2026-11-25 (John): the October criteria (independent opponent pool, release targets met in matches) are replaced by the strength program criteria above. Next steps: MSH-214 (Tenjiku Strength Harness), then MSH-202 (Tenjiku Search Speed).
<!-- SECTION:NOTES:END -->
