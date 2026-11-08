---
id: MSH-50
title: Establish and meet measured shogi engine strength criteria
status: Ready
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-08T19:02:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-41
  - MSH-109
  - MSH-49
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
ordinal: 50000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Functional shogi search/play does not satisfy P-02. This ticket owns the shogi-specific opponent pool, transparent baseline, release-strength criteria, search/evaluation improvement and measured verdict, using MSH-48 shared infrastructure. An initial learned evaluator improvement is separately owned by MSH-55; no requirement for an identical neural architecture in every game is inferred.

Open D-09 questions: establish meaningful shogi opponents and starting positions, resource/fairness conditions, release-strength targets and uncertainty criteria from baseline/pilot evidence; record owner decisions on material targets before claiming success. Search tuning, evaluator choices and compute budgets remain empirical, not resolved by this planning ticket.

Owner sequence2026-11-08: after the mature Chess milestone, adapt and develop established conventional search/evaluation as a coherent Shogi engine before substantial formal strength evaluation. Shogi/Xiangqi precede the less-studied variants. Transfer useful techniques, not Chess constants, pruning assumptions, royal semantics or repetition identities; preserve the accepted native profile(s). Routine correctness/tactical tests and bounded profiling accompany coherent increments. Aggregate routine local engine-match work is limited to15minutes per task unless John explicitly authorizes more; no per-technique substantial statistical studies or multi-hour benchmarks. Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A reproducible shogi baseline and suitable independent opponent pool are established, with profile compatibility, limits, openings/colors, failures and executable identities recorded. Unavailable external opponents remain explicit measurement limitations; self-play alone is not independent strength evidence.
- [ ] #2 Game-specific shogi strength targets and comparison/uncertainty criteria are explicitly decided and documented from baseline evidence before a release-strength verdict. Numeric targets are not fabricated from other games or nodes-per-second results.
- [ ] #3 The mShogi shogi engine has measured search/evaluation improvements and meets the agreed criteria in reproducible matches against the defined pool. Search correctness, legal complete actions, history-aware terminal behavior and regression evidence remain intact; speedups alone do not establish stronger play.
- [ ] #4 The final shogi report records the tested revision, all experimental conditions and uncertainty, actual successes/failures and independent review. Open required targets block completion and M5 closure rather than being absorbed into optional M6 research.
- [ ] #5 A substantially complete game-specific conventional search/evaluation/execution engine is independently verified before the major strength study; established techniques are adapted to native tactics/history/outcomes and routine development stays within the15minute cumulative match allowance without per-optimization formal studies.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner follow-on2026-11-08: after initial Chess diagnostic49.2, prioritize conventional Shogi/Xiangqi development rather than repeated small Chess benchmarking. Adapt49.1 techniques to native tactics/history/outcomes; do not copy Chess constants or pruning assumptions. Routine algorithm/verification choices are autonomous, with meaningful correctness/tactical tests, profiling and15minutes cumulative routine matches/task unless explicitlyextended. Material release targets or substantial formal studies return toowner whenappropriate; they do not block ordinary conventional engineering. Preserve selectednativeprofiles andallfiveAC. No implementation ofthisfuturetask starts in49.2.
<!-- SECTION:NOTES:END -->
