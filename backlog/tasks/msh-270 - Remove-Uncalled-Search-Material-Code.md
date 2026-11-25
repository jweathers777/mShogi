---
id: MSH-270
title: Remove Uncalled Search Material Code
status: Ready
assignee: []
created_date: '2026-11-25T13:01:00Z'
labels:
  - cleanup
  - search
  - model:medium
  - effort:low
milestone: m-6
dependencies: []
priority: low
ordinal: 248000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-244 (Remove Test-Only Material Evaluators) removed the last callers of ShogiPosition::material and ChuPosition::material in crates/mshogi-search. shogi::piece_value now has only test callers and repeats the VALUE table in shogi/position.rs, which exchange and ordering use; no test compares the two now.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 ShogiPosition::material, its incremental material field and ChuPosition::material are removed; the Chu board material that evaluation uses stays
- [ ] #2 Shogi piece values are defined in one place and keep the same numbers
- [ ] #3 Search results and evaluation are unchanged; workspace checks pass
<!-- AC:END -->
