---
id: MSH-137
title: >-
  Advance conventional Chess to practical classical maturity before cross-game
  transfer
status: Done
assignee:
  - '@codex'
created_date: '2026-11-08T21:47:00Z'
updated_date: '2026-11-09T00:50:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-136
documentation:
  - docs/m5/strength-roadmap.md
priority: high
ordinal: 139000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner correction after MSH-136: major algorithm presence and the initial Stockfish diagnostic do not establish conventional Chess maturity. Continue coherent researched engineering across native overhead, history-safe TT reuse, selective search and classical evaluation until remaining opportunities are mainly diminishing returns, speculative research or disproportionate effort. Preserve completed 49/49.1/49.2/136 and evidence. This supersedes immediate Shogi/Xiangqi transfer; no new formal strength studies. Routine match experiments retain the cumulative 15-minute task budget, with no artificial budget resets across subtasks.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Significant practical native and search-state costs are measured and corrected with authoritative rules, history, exact undo and cancellation preserved.
- [x] #2 Effective transposition reuse is improved where native future-state equivalence can be established; unsafe board-only reuse is excluded by controlled tests and independent review.
- [x] #3 Selective search and classical evaluation are coherently refined against independently expected tactical/endgame tests, bounded profiles and proportionate development comparisons, rather than algorithm checklist completion.
- [x] #4 An independent final engineering assessment explains remaining opportunities and supports practical conventional maturity before Shogi/Xiangqi transfer; no strength, Elo or parity claim is inferred from throughput or small tests.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Codex owns codex/rewrite at clean origin-aligned 24512d0; unrelated historical-access worktree preserved. Complete bounded native-query, history-safe TT and search/evaluation increments with their own tests, review and integration. Research primary classical engine sources independently, measure only short fixed-work profiles and focused correctness/tactical comparisons, no opponent campaign planned. Reassess remaining high-value work after each coherent checkpoint; do not declare maturity from algorithm inventory. Required native CI at integrated source checkpoints; reuse unaffected GUI evidence. Update roadmap and gate 50/54 on this task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Substantive native/search/evaluation increments implemented and independently reviewed, including lazy native summaries, checked transitions and prepared pruned children, derived occupancy/rays, exact future-context main/q TT, bounded replacement and classical tactical/pawn/king corrections. All acceptance remains open pending coherent final checks/platform receipts and maturity assessment. Fresh independent assessment withholds maturity: finish direct34-byte lossless identity storage, small4K/16K capacity+memory comparison and basic KQ/KR conversion coherence; direct native capture-to-target generation remains a measured SEE opportunity. No opponent matches or formal studies; all profiling producers so far280queries/~60.49 recorded query seconds including failed timing-tool receipt. Current broad native regression/lint campaign94834 uses frozen source manifest /private/tmp/msh137/final-source.json; no production edits while it runs. Continue concrete follow-ups after completion; no Shogi/Xiangqi release.

Final frozen source manifest accepted-source.json (patch603ca1d5,18files), independent native/search/maturity reviews approved. Final local checks:670native pass/6intentionalignored,11UCI,29desktopChess; workspaceClippy-Dwarnings/fmt pass. Isolated12query medians93.055/132.602/43.436/32.559ms; no strength inference. Historical forcedmate255positionproof and actualmate-search regression pass, original-root1s avoidsQxa1. 4K/16K no meaningful gain; default4K. All338profilingqueries/74.08recordedseconds and failures preserved in docs/m5/evidence/msh137/verification.json; no matches. Final actual Ubuntu24/macOS15/Windows2022 RustCI required; do not mark Done yet.

First nativeCI42536624859 atf1484b15 passed all3workspace test suites. macOS128902235827 failed fixture regeneration because existingChessmate snapshot expected29visits/seldepth2 insteadactual65/7; mate1/score/PV unchanged. SupersededLinux/Windowsjobs canceled afterworkspacepass. Regenerated allnativeChess/sharedfixtures: onlyonefile differed;34frontendconsumer tests pass Node24.22.1. No cratesourcechanges. Historicalreceipts preserved; newexact-source3OSCI required, task remainsInReview.

Final source/evidence reconciliation approved. Exact tested source2a2ff7c, actual required3OS native jobs all passed; production unchanged fromf1484b15. Source integrated/pushed normally to codex/rewrite. Completion records/evidence published with this finalization; no further test campaign needed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed coherent conventional Chess refinement: native query/state overhead, exact history-safe main/q TT reuse, staged selective search, proved quiet-mating threat coverage and classical pawn/king conversion corrections. Production integration f1484b15; fixture-corrected exact tested integration 2a2ff7c00897fdcd10527c8db8cc77f5e1220e65 on codex/rewrite. Independent native/search/maturity and final evidence reviews approve all criteria. Required Rust CI42538719298 attempt1 passed Ubuntu24.04 job128908684741, macOS15 job128908685074, Windows2022 job128908685190. Final workspace passes1169/1169/1166,11 intentional ignores each; local670 native+11 UCI+29 desktop and34 frontend checks passed. Representative fixed-visit median runtimes improved roughly2.8–4.1x, descriptively (policy changed), and historical forced mate is now proved/found. No Chess opponent experiments or strength claim. Historical failed/canceled source receipts retained in docs/m5/evidence/msh137/verification.json. Remaining selective threat/tuning/special-endgame limits explicit. Next MSH-138 calibrates ordinary desktop defaults within admitted ceilings before50/54; conventional program is not declared permanently finished from algorithm presence.
<!-- SECTION:FINAL_SUMMARY:END -->
