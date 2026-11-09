---
id: MSH-143
title: Chess Search Efficiency
status: Ready
assignee: []
created_date: '2026-11-09T16:23:00Z'
updated_date: '2026-11-09T18:11:00Z'
labels:
  - handle:chess-search-efficiency
milestone: m-5
dependencies:
  - MSH-141
documentation:
  - docs/m5/chess-strength-assessment-report.md
priority: high
ordinal: 148000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Search Efficiency. Continue the owner-authorized conventional Chess roadmap after MSH-141 (Chess Frontier Reliability). Profile remaining significant native query, exchange-estimation, ordering/research and exact-context transposition costs on representative timed positions. Select established practical improvements from measured bottlenecks, preserving native complete history and outcome authority. This is coherent engine engineering, not an algorithm inventory or node-count target. No learned evaluation, arbitrary rating target or Shogi/Xiangqi transfer; no substantial study per technique, routine local matches15minutes cumulative/task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Current cost measurements distinguish actual native/SEE/search work and useful safe transposition reuse from reported visit counts, with bounded reproducible representative inputs.
- [ ] #2 Significant practical inefficiencies are corrected where supported by evidence, with independent tactical, exact-context/history, cancellation and restoration regressions; unsupported speculative changes are not presented as improvements.
- [ ] #3 Before/after realistic-time evidence, independent review and required platform verification support a coherent integrated increment; remaining substantial opportunities and readiness for a fresh adaptive reassessment are reported without declaring maturity or transfer.
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
MSH-142 varied frontier verification found a controlled remaining boundary limitation: r6k/8/5K2/8/6Q1/8/8/8 b - - 0 1. Independent exhaustive immediate-mate replies establish only Ra7,Ra6+,Rf8+ prevent all mate1. Current full initial q frontier chooses safe defence, but q left14 chooses Rg8, missing noncontact Qh5/Qh4/Qh3 mate after the quiet-check cutoff. Preserve and correct this general threatened-defence horizon case within search-frontier efficiency work, without reopening recursive broad quiet expansion. Failed receipt /private/tmp/msh142/frontier-defence-failure.log will accompany142evidence;142passing test explicitly covers initial frontier only. KRKP e4/e5 remains a demonstrated endgame weakness outside142narrow exact-KPK/fortress/conversion subset; no claim of correction from static knowledge.
<!-- SECTION:NOTES:END -->
