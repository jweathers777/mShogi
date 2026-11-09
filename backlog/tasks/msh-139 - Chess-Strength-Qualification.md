---
id: MSH-139
title: Chess Strength Assessment
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-09T11:26:00Z'
updated_date: '2026-11-09T13:37:00Z'
labels:
  - handle:chess-strength-assessment
milestone: m-5
dependencies:
  - MSH-138
documentation:
  - docs/m5/chess-strength-assessment.md
priority: high
ordinal: 144000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Strength Assessment. Owner revision2026-11-09 authorizes adaptive discovery of current conventional Chess strength, superseding the unapproved qualification proposal. There is no3100/3000target, fixed128-gameconfirmation orsix-hourcampaigncap. Use a broad consistent independently rated full-strength pool, equal thinking time, small paired batches directed by scores/uncertainty, and fresh independent-family confirmation. Continue until evidence is useful for the next development decision; use established statistics and proportionate local compute, preserve operational safeguards/accounting, escalate genuinely substantial additional resources. Report actual local reference-range/uncertainty, tactical/endgame/positional/search limitations and realistic conventional improvement opportunities. Then STOP for owner review before selecting a target, changing the engine or transferring to other games. Preserve completed correctness/evidence and the rejected proposal as historical only.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Current engine/opponent/source/condition identities, broad independently rated pool and adaptive selection/stopping method are recorded before measurement; no predetermined strength target, fixed tournament size or arbitrary campaign-hour allocation governs acceptance.
- [ ] #2 Actual fair-condition full-strength paired games locate useful current-strength range with honest sequential uncertainty and fresh confirmation against at least two independent engine families; small samples, mismatches, failures, censoring and published-pool differences are not hidden. No official CCRL or human rating is claimed.
- [ ] #3 Independent tactical/endgame positions and bounded strong-reference analysis explain representative tactical, positional, conversion and search weaknesses; the report is useful regardless of measured strength, without a pass/fail performance bar.
- [ ] #4 Independent evidence review supports reproducibility, inference and conclusions; findings are integrated and reported, then work stops for owner choice of the next objective. Do not declare Chess sufficiently strong, choose a final target, improve production search/evaluation or begin cross-game transfer in this assignment.
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
Codex owns clean codex/rewrite at9a509f0, existing historical worktree preserved; pinned Backlog1.55.1. Freeze unchanged current candidate/runner and one datedCCRL singleCPU catalog spanning~1450–3250. Build/preflight initialSimbelmyne1.3.1 and only adaptively needed nativeUCI references. Reuse existingexternalrunner oneopeningpair/report and fullnative replay, equal1s/nohandicaps, honest512-plycap/noevaluation adjudication. Pin establishedgenericbook and independent discovery/confirmation streams. Use upstreamconfseq anytime-valid boundedpair-score intervals, standardElo score mapping and information-per-cost routing with separate anchor/model caveats; independentreview before scoredgames. Startsmall, followobservedstrength/uncertainty, confirmfreshnearbyfamilies until range/quality evidence is useful; monitor accumulatedcost and marginalinformation ratherthan preset games/hours. Add independenttactical/endgame and representativeerror analysis. No productionengine changes or newbroadGUI/CIcampaign; reuse currentacceptedproductsource. Finalindependentreview/report/integrateandstop forowner nextobjective.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Research-backed approval proposal is docs/m5/chess-strength-qualification.md: conditional localreferenceindexpoint≥3100/one-sided95%lower≥3000,2independentnearbyfamilies,≤24adaptive scoutgames+64freshconfirmationpairs/128games at1s,6haggregate active cap includingfailed/preflight/qualityanalysisruns, nopaidcompute. Frozen2024CCRL40/15 singleCPU anchors, initialArasan18.3, nearTexel1.07/Arasan21.2. Independentmethod/source review approvedproposal; this is NOT ownerapproval or qualification. No references built/run, no formalgames. RemainsToDo untilJohn approves target/protocol/budget; actualstandardmustlaterbe demonstrated beforeDone/transfer.

MSH-138 (Desktop Search Limits) is Done after actual3OSCI and independent final acceptance atfac34c9. Engineering dependency satisfied, but this task remainsToDo/unassigned because John has not approved the concrete target/protocol/six-hourtotalcap. The written proposal is reviewable; all formal qualification/reference runs remain unperformed. Sourcechanges do notreset budget orauthorize reusingconfirmationroots. MSH-50 (Conventional Shogi) and MSH-54 (Conventional Xiangqi) remain blocked until the agreed strength standard is demonstrated.

Owner explicitly REVISES prior unapproved proposal and AUTHORIZES actual adaptiveassessment now. Old target/numericgates/fixedconfirmation/six-hourrequest are superseded, not current blockers. Current handle renamed by owner to Chess Strength Assessment. Existing task notes/evidence remain historical. Routine match batches/opponent choice are autonomous; genuinely substantial additional resources still require escalation.

Before scored games: owner-authorized discovery method now docs/m5/chess-strength-assessment.md. Primary inference is a standard conjugate pentanomial Dirichlet posterior (.5/bin, .2/1 sensitivity); upstream empirical-Bernstein anytime bounds remain conservative supplemental evidence. Independent method review supports adaptive discovery and soft precision stopping, with exact-release/phase separation and model/censoring caveats. Broad 2024 CCRL pool, initial Simbelmyne 1.3.1 built ARM64 (only modern spelling of removed compiler flag). Current unchanged candidate/runner built release/locked from 9a509f0. CC0 8moves book pinned, 34699 nonterminal unique roots split before outcomes. Bounded protocol smoke healthy, native replay passed, two deliberate ply caps unscored. Six offline instrumentation tests passed. No scored assessment games yet.
<!-- SECTION:NOTES:END -->
