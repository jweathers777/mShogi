---
id: MSH-143
title: Chess Search Efficiency
status: In Review
assignee:
  - '@codex'
created_date: '2026-11-09T16:23:00Z'
updated_date: '2026-11-09T18:25:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Root Codex owns primary codex/rewrite after reviewed142checkpoint50b459a; no competing builder. Preserve141frozenmacOSretry and142localevidence. Current frozen22bf634qualitative5sCPU sample identifies redundant Position::apply structural validation mostly in q projections (263/334 validation samples), SEE43; exact TTcomparison only0.3percent, so no unsupported cache redesign. Expose existing checked native board projection, prove public-apply/error equivalence and use for q/SEE without changing actual native Game/history admission. Reproduce/resolve recorded left14defensive-resource noncontactmate horizon case with bounded exact reply inspection, not broader recursive quiet expansion; preserve varied sacrifices/checkevasions. Measure deterministic identical-work projection ablation then full corrected realistic-time representative searches, including actual primitive counts/CPU attribution. Independently review native boundary and tactical behavior, run focused native/search/process checks and one combined142+143 Rust3OS integration campaign; retain all failures and source identities. Only after acceptance, conduct proportionate fresh adaptive reassessment within owner authorization; no learned evaluation or cross-game transfer.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-142 varied frontier verification found a controlled remaining boundary limitation: r6k/8/5K2/8/6Q1/8/8/8 b - - 0 1. Independent exhaustive immediate-mate replies establish only Ra7,Ra6+,Rf8+ prevent all mate1. Current full initial q frontier chooses safe defence, but q left14 chooses Rg8, missing noncontact Qh5/Qh4/Qh3 mate after the quiet-check cutoff. Preserve and correct this general threatened-defence horizon case within search-frontier efficiency work, without reopening recursive broad quiet expansion. Failed receipt /private/tmp/msh142/frontier-defence-failure.log will accompany142evidence;142passing test explicitly covers initial frontier only. KRKP e4/e5 remains a demonstrated endgame weakness outside142narrow exact-KPK/fortress/conversion subset; no claim of correction from static knowledge.

Implemented measured native checked-projection reuse for q/SEE, preserving all complete effects/errors and actual Game history admission. Exhaustive coordinate/promotion comparison with publicapply passes. New bounded final-defence immediate-reply proof corrects Rg8/noncontactmate boundary failure without recursivequiet expansion; cancellation/restoration and guardedAutoClaimprecedence pass.92release searchlib tests,25nativeChess tests,4real release desktopplay tests pass;workspaceclippy/fmt clean; generatedfixturesunchanged.3sequential16positionprojection-onlyfixedruns have identical complete outputs/PVs/workstats; combinedfixedtimeabout4.9percent lower aggregate,1stimeddepthsunchangedorhigher. Qualitative CPU sample and safeTTaudit preserved; no speculativecache redesign or strengthclaim. Independentreview finds no material defect. Canonical docs/m5/evidence/msh143/verification.json; required combined142+143Rust3OSCI pending at finalsource. No opponentgames yet.
<!-- SECTION:NOTES:END -->
