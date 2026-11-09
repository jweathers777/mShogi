---
id: MSH-139
title: Chess Strength Qualification
status: To Do
assignee: []
created_date: '2026-11-09T11:26:00Z'
updated_date: '2026-11-09T12:41:00Z'
labels:
  - handle:chess-strength-qualification
milestone: m-5
dependencies:
  - MSH-138
documentation:
  - docs/m5/chess-strength-qualification.md
priority: high
ordinal: 144000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Strength Qualification. Owner consolidation2026-11-09 requires demonstrated strong conventional Chess before architecture transfer, beyond MSH-137 (Chess Search Refinement) engineering readiness. Research target/protocol/overall budget and present a concise proposal, then obtain explicit owner approval before any formal adaptive campaign or substantial compute. Use unweakened independently rated opponents from one dated pool where practical, equal thinking time and honest hardware/memory/condition differences, paired-opening adaptive estimation followed by credible nearby-family evidence, independent tactical/endgame checks and significant-mistake analysis. Reuse existing runner/experiment infrastructure. Learned evaluation remains later. Existing completed engineering and MSH-49.2 (Stockfish Baseline) evidence stay historical.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A research-backed meaningful classical strength target, adaptive opponent/protocol and aggregate resource budget are reviewable and explicitly approved by John before formal qualification work.
- [ ] #2 Actual full-strength fair-condition games from frozen sources, paired openings and appropriate uncertainty establish the agreed standard against credible independent nearby engine families; adaptive selection, failures, incomplete/censored games and condition/model limitations remain explicit. No official CCRL or human rating is claimed.
- [ ] #3 Independent tactical/endgame expectations and bounded strong-reference mistake analysis complement full-game strength evidence. If the standard is not met or evidence is inconclusive, record directed conventional-engine improvement work and leave qualification/transfer unfinished.
- [ ] #4 Independent final review reconciles the approved standard, native legality/history, exact source/opponent/build/conditions, all outcomes and uncertainty before acceptance and normal integration. Shogi/Xiangqi cannot proceed merely because a report or algorithm checklist exists.
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
Research-backed approval proposal is docs/m5/chess-strength-qualification.md: conditional localreferenceindexpoint≥3100/one-sided95%lower≥3000,2independentnearbyfamilies,≤24adaptive scoutgames+64freshconfirmationpairs/128games at1s,6haggregate active cap includingfailed/preflight/qualityanalysisruns, nopaidcompute. Frozen2024CCRL40/15 singleCPU anchors, initialArasan18.3, nearTexel1.07/Arasan21.2. Independentmethod/source review approvedproposal; this is NOT ownerapproval or qualification. No references built/run, no formalgames. RemainsToDo untilJohn approves target/protocol/budget; actualstandardmustlaterbe demonstrated beforeDone/transfer.

MSH-138 (Desktop Search Limits) is Done after actual3OSCI and independent final acceptance atfac34c9. Engineering dependency satisfied, but this task remainsToDo/unassigned because John has not approved the concrete target/protocol/six-hourtotalcap. The written proposal is reviewable; all formal qualification/reference runs remain unperformed. Sourcechanges do notreset budget orauthorize reusingconfirmationroots. MSH-50 (Conventional Shogi) and MSH-54 (Conventional Xiangqi) remain blocked until the agreed strength standard is demonstrated.
<!-- SECTION:NOTES:END -->
