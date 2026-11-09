---
id: MSH-139
title: Chess Strength Assessment
status: Done
assignee:
  - '@codex'
created_date: '2026-11-09T11:26:00Z'
updated_date: '2026-11-09T15:07:00Z'
labels:
  - handle:chess-strength-assessment
milestone: m-5
dependencies:
  - MSH-138
documentation:
  - docs/m5/chess-strength-assessment.md
  - docs/m5/chess-strength-assessment-report.md
priority: high
ordinal: 144000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Strength Assessment. Owner revision of 2026-11-09 authorizes adaptive discovery of the current conventional Chess engine's strength. The unapproved qualification target, numerical pass gates, fixed confirmation tournament and six-hour budget are superseded.

Use a broad consistent independently rated full-strength pool, equal thinking time, paired openings, established uncertainty estimates and fresh confirmation against at least two independent engine families. Stop when evidence is useful for the next development decision; retain operational safeguards and actual resource accounting, escalating genuinely substantial additional needs. Report actual local CCRL-referenced range, limitations and representative tactical, positional, endgame and search findings, without an official CCRL or human rating claim.

Then stop for John to choose the next objective. Do not select a final target, declare Chess sufficiently strong, improve production search/evaluation or transfer to other games in this task. Preserve completed correctness and historical evidence.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Current engine/opponent/source/condition identities, broad independently rated pool and adaptive selection/stopping method are recorded before measurement; no predetermined strength target, fixed tournament size or arbitrary campaign-hour allocation governs acceptance.
- [x] #2 Actual fair-condition full-strength paired games locate useful current-strength range with honest sequential uncertainty and fresh confirmation against at least two independent engine families; small samples, mismatches, failures, censoring and published-pool differences are not hidden. No official CCRL or human rating is claimed.
- [x] #3 Independent tactical/endgame positions and bounded strong-reference analysis explain representative tactical, positional, conversion and search weaknesses; the report is useful regardless of measured strength, without a pass/fail performance bar.
- [x] #4 Independent evidence review supports reproducibility, inference and conclusions; findings are integrated and reported, then work stops for owner choice of the next objective. Do not declare Chess sufficiently strong, choose a final target, improve production search/evaluation or begin cross-game transfer in this assignment.
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
Codex owns codex/rewrite in /Users/johnweathers/Development/Personal/mShogi. Startup source and origin were aligned at 9a509f0; the unrelated historical Tenjiku worktree is preserved. Use pinned Backlog 1.55.1.

Freeze unchanged candidate/runner, binary identities and conditions. Prepare a broad dated CCRL pool, building only informative full-strength references and checking real one-second behavior. Reuse the native external runner and replay, one complete opening pair per report, honest unscored technical caps/failures and bounded cleanup.

Freeze a generic opening book and disjoint discovery/confirmation populations. Infer separate exact-release/phase pentanomial Dirichlet posteriors with prior sensitivity; use upstream anytime score bounds as conservative supplemental evidence. Route by observed scores, uncertainty, information per cost and independent-family coverage. Continue fresh nearby-family confirmation until the range and quality findings are useful, without a fixed tournament size or campaign-hour allocation.

Run small independent tactical/endgame checks and bounded representative reference analysis between match batches. Independently review method, instrumentation and final evidence. Reuse unchanged product CI/GUI evidence. Integrate findings, update the handoff and stop for owner review; no production engine changes or cross-game work.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Research-backed approval proposal is docs/m5/chess-strength-qualification.md: conditional localreferenceindexpoint≥3100/one-sided95%lower≥3000,2independentnearbyfamilies,≤24adaptive scoutgames+64freshconfirmationpairs/128games at1s,6haggregate active cap includingfailed/preflight/qualityanalysisruns, nopaidcompute. Frozen2024CCRL40/15 singleCPU anchors, initialArasan18.3, nearTexel1.07/Arasan21.2. Independentmethod/source review approvedproposal; this is NOT ownerapproval or qualification. No references built/run, no formalgames. RemainsToDo untilJohn approves target/protocol/budget; actualstandardmustlaterbe demonstrated beforeDone/transfer.

MSH-138 (Desktop Search Limits) is Done after actual3OSCI and independent final acceptance atfac34c9. Engineering dependency satisfied, but this task remainsToDo/unassigned because John has not approved the concrete target/protocol/six-hourtotalcap. The written proposal is reviewable; all formal qualification/reference runs remain unperformed. Sourcechanges do notreset budget orauthorize reusingconfirmationroots. MSH-50 (Conventional Shogi) and MSH-54 (Conventional Xiangqi) remain blocked until the agreed strength standard is demonstrated.

Owner explicitly REVISES prior unapproved proposal and AUTHORIZES actual adaptiveassessment now. Old target/numericgates/fixedconfirmation/six-hourrequest are superseded, not current blockers. Current handle renamed by owner to Chess Strength Assessment. Existing task notes/evidence remain historical. Routine match batches/opponent choice are autonomous; genuinely substantial additional resources still require escalation.

Before scored games: owner-authorized discovery method now docs/m5/chess-strength-assessment.md. Primary inference is a standard conjugate pentanomial Dirichlet posterior (.5/bin, .2/1 sensitivity); upstream empirical-Bernstein anytime bounds remain conservative supplemental evidence. Independent method review supports adaptive discovery and soft precision stopping, with exact-release/phase separation and model/censoring caveats. Broad 2024 CCRL pool, initial Simbelmyne 1.3.1 built ARM64 (only modern spelling of removed compiler flag). Current unchanged candidate/runner built release/locked from 9a509f0. CC0 8moves book pinned, 34699 nonterminal unique roots split before outcomes. Bounded protocol smoke healthy, native replay passed, two deliberate ply caps unscored. Six offline instrumentation tests passed. No scored assessment games yet.

Scouting checkpoint: 8 actual full-strength games, all native replayed and cleanly reaped. Oxidation 0.6.0 (CCRL1943): 2 wins/1 draw/1 loss; Goldfish1.13.0 (1982): 0 wins/1 draw/3 losses. Only two pairs per release: wide model/prior-sensitive intervals, no final estimate or qualification claim. Fresh confirmation now begins from the prepartitioned unused opening half. Twenty independent quality positions: 11/12 published first moves plus a native-verified equivalent mate; 7/8 WDL-preserving endgame choices, with four nondiscriminating cases. Unique rook win ...Re1 missed for drawn ...Rc1. Bounded Stockfish19 analysis confirms poor rook-to-pawn transition judgment in representative loss; evaluations are opinions, outcomes remain native. No production changes or additional Stockfish match study. Evidence checkpoint pushed at3c231b2; task remains In Progress.

Measurement complete; no further games planned pending final independent evidence acceptance. 24 eligible games / 12 pairs, every pair native-replayed with successful cleanup, zero failed/capped scored games. Fresh confirmation (8 games each): Oxidation 1W-4D-3L, Goldfish 2W-3D-3L. Conditional pentanomial posterior medians/bands 1883 [1750,2047] and 1952 [1756,2160]; prior sensitivity supports a rough local index around1900 with1700-2200 working span, explicitly not a pooled/joint95% or official/human rating. Independent stopping review approved: another batch would mainly narrow uncertainty rather than change the present decision. Approximately50 minutes measured match/diagnostic execution; build/preparation logs separate, no paid compute. Findings are docs/m5/chess-strength-assessment-report.md and the single verification.json; actual12-test receipt is tests-telemetry.log.gz (older10/11-test receipts retained). All production files remain unchanged from9a509f0 / runtime-equivalent acceptedfac34c9. No target, maturity verdict, production improvement or transfer chosen; MSH-50 (Conventional Shogi) and MSH-54 (Conventional Xiangqi) stay To Do for owner review.

Independent final evidence and wording reviews approve acceptance with no material finding. All15 pair receipts,2978 accepted moves,24 native conclusions, fixed draws/opening lineage, current binary/source identities, derived telemetry and actual12-test receipt reconcile. Stopping is supported by consistent fresh-family estimates and modest central prior sensitivity; no more games justified solely for tighter intervals. Original diagnostic source/probe failures and caps remain preserved; possible early tactical timing interference and author-observed IPC cleanup are explicitly qualified. Production and all consumers are unchanged from acceptedfac34c9; no new product/GUI campaign. AC1-4 and DoD1-3 supported; DoD4 awaits normal commit/push.

Normal integration completed: accepted report and evidence committed/pushed at4ee6c4de8e35fb2e3f8bb91b59b040abd65a28bd on codex/rewrite. All four current AC and four DoD items are supported and checked. This completion is the assessment handoff, with owner review required before choosing any next development objective. Final bookkeeping commit follows without changing measured source, results, requirements or limitations.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Established a conditional current-strength working index around1900, with a rough1700-2200 sensitivity span, through24 complete full-strength equal-one-second games. Fresh8-game results: Oxidation1W-4D-3L and Goldfish2W-3D-3L. Separate95% Bayesian credible bands and prior/pool/condition limitations are reported; no official or human rating is claimed.

All12 scored pairs passed native replay and cleanup. Twelve focused instrumentation tests passed. Independent method, stopping, source/receipt and final-report reviews approved completion. Twenty independent quality positions and bounded reference analysis identify tactical horizon, endgame judgment and search-frontier limitations. No additional match study or production refinement is needed to finish this assessment.

Exact candidate/runner build source9a509f077460e07341d2bccf830e4b02fcb0e95e; production remains equivalent to acceptedfac34c9. Accepted findings/evidence integrated and pushed normally to codex/rewrite at4ee6c4de8e35fb2e3f8bb91b59b040abd65a28bd. Canonical evidence is docs/m5/evidence/msh139/verification.json; findings are docs/m5/chess-strength-assessment-report.md. Historical setup failures, smoke caps and earlier helper versions remain preserved.

Stop for John to review the findings and choose the next objective. No final target, Chess-maturity verdict, further production change or cross-game transfer was selected. MSH-50 (Conventional Shogi) and MSH-54 (Conventional Xiangqi) remain To Do.
<!-- SECTION:FINAL_SUMMARY:END -->
