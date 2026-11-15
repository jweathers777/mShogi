---
id: MSH-53
title: Establish and meet measured tenjiku shogi engine strength criteria
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-15T16:29:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-44
  - MSH-109
  - MSH-50
  - MSH-54
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
ordinal: 178000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Functional tenjiku shogi search/play does not satisfy P-02. This ticket owns the tenjiku shogi-specific opponent pool, transparent baseline, release-strength criteria, search/evaluation improvement and measured verdict, using MSH-48 shared infrastructure. An initial learned evaluator improvement is separately owned by MSH-55; no requirement for an identical neural architecture in every game is inferred.

Open D-09 questions: establish meaningful tenjiku shogi opponents and starting positions, resource/fairness conditions, release-strength targets and uncertainty criteria from baseline/pilot evidence; record owner decisions on material targets before claiming success. Search tuning, evaluator choices and compute budgets remain empirical, not resolved by this planning ticket.

Project direction 2026-11-13 (AGENTS.md, "Current Chess strength direction"): Tenjiku Shogi moves to the one shared search core and supplies its own search position, evaluation and special rules; do not build a separate search for this game. docs/m5/shared-search-core.md records what Tenjiku Shogi needs from the core. Before Tenjiku uses the core, the continuation-history table of the core must change (see docs/m5/shared-search-core.md), and make/unmake must handle the multiple captures of the Fire Demon. The order of the remaining games is John's decision; the earlier 2026-11-08 order (Shogi and Xiangqi before the other variants) is history. Transfer useful techniques, not Chess constants, pruning assumptions, royal semantics or repetition identities; preserve the accepted native profile(s). Routine correctness/tactical tests and bounded profiling accompany coherent increments. Game tests follow "Game tests in this direction" in docs/m5/strength-roadmap.md; the earlier 15-minute match allowance no longer applies (revised 2026-11-15). Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A reproducible tenjiku shogi baseline and suitable independent opponent pool are established, with profile compatibility, limits, openings/colors, failures and executable identities recorded. Unavailable external opponents remain explicit measurement limitations; self-play alone is not independent strength evidence.
- [ ] #2 Game-specific tenjiku shogi strength targets and comparison/uncertainty criteria are explicitly decided and documented from baseline evidence before a release-strength verdict. Numeric targets are not fabricated from other games or nodes-per-second results.
- [ ] #3 The mShogi tenjiku shogi engine has measured search/evaluation improvements and meets the agreed criteria in reproducible matches against the defined pool. Search correctness, legal complete actions, history-aware terminal behavior and regression evidence remain intact; speedups alone do not establish stronger play.
- [ ] #4 The final tenjiku shogi report records the tested revision, all experimental conditions and uncertainty, actual successes/failures and independent review. Open required targets block completion and M5 closure rather than being absorbed into optional M6 research.
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
<!-- SECTION:NOTES:END -->
