---
id: MSH-51
title: Establish and meet measured tori shogi engine strength criteria
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-18T19:25:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-42
  - MSH-109
  - MSH-50
  - MSH-54
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
ordinal: 176000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Functional tori shogi search/play does not satisfy P-02. This ticket owns the tori shogi-specific opponent pool, transparent baseline, release-strength criteria, search/evaluation improvement and measured verdict, using MSH-48 shared infrastructure. An initial learned evaluator improvement is separately owned by MSH-55; no requirement for an identical neural architecture in every game is inferred.

Open D-09 questions: establish meaningful tori shogi opponents and starting positions, resource/fairness conditions, release-strength targets and uncertainty criteria from baseline/pilot evidence; record owner decisions on material targets before claiming success. Search tuning, evaluator choices and compute budgets remain empirical, not resolved by this planning ticket.

Project direction 2026-11-13 (AGENTS.md, "Current Chess strength direction"): Tori Shogi moves to the one shared search core and supplies its own search position, evaluation and special rules; do not build a separate search for this game. docs/m5/shared-search-core.md records what Tori Shogi needs from the core. Tori already searches through the shared core (MSH-165 (Tori Search Core Adoption), done 2026-11-14); remaining work is its strength criteria, evaluation and time use. Project decision 2026-11-15: Shogi and Xiangqi move to the shared core first, then Chu Shogi and Tenjiku Shogi. Transfer useful techniques, not Chess constants, pruning assumptions, royal semantics or repetition identities; preserve the accepted native profile(s). Routine correctness/tactical tests and bounded profiling accompany coherent increments. Game tests follow "Game tests in this direction" in docs/m5/strength-roadmap.md; the earlier 15-minute match allowance no longer applies (revised 2026-11-15). Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A reproducible tori shogi baseline and suitable independent opponent pool are established, with profile compatibility, limits, openings/colors, failures and executable identities recorded. Unavailable external opponents remain explicit measurement limitations; self-play alone is not independent strength evidence.
- [ ] #2 Game-specific tori shogi strength targets and comparison/uncertainty criteria are explicitly decided and documented from baseline evidence before a release-strength verdict. Numeric targets are not fabricated from other games or nodes-per-second results.
- [ ] #3 The mShogi tori shogi engine has measured search/evaluation improvements and meets the agreed criteria in reproducible matches against the defined pool. Search correctness, legal complete actions, history-aware terminal behavior and regression evidence remain intact; speedups alone do not establish stronger play.
- [ ] #4 The final tori shogi report records the tested revision, all experimental conditions and uncertainty, actual successes/failures and independent review. Open required targets block completion and M5 closure rather than being absorbed into optional M6 research.
- [ ] #5 A substantially complete conventional engine for this game, on the shared search core with its own search position, evaluation and special rules, is independently verified before the major strength study; established techniques are adapted to native tactics, history and outcomes, and development game tests follow docs/m5/strength-roadmap.md without per-optimization formal studies.
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

Direction, 2026-11-18 (John): this game follows the per-game classical strength program in docs/m5/strength-roadmap.md (harness, selectivity round, evaluation round, approved anchor, blunder screen, stop decision), with the same round rules as Chess, Shogi and Xiangqi. Adjust only for the game: few or no rated opponents exist, so the anchor step becomes a recorded limitation and the evidence comes from frozen-predecessor tests, tactical suites and, if no stronger engine exists, a deeper search of mShogi as a same-family judge. Create the per-step tasks when this game starts; John selects when.
<!-- SECTION:NOTES:END -->
