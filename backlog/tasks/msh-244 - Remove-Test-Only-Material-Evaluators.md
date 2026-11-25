---
id: MSH-244
title: Remove Test-Only Material Evaluators
status: Ready
assignee: []
created_date: '2026-11-25T12:20:00Z'
updated_date: '2026-11-25T12:41:00Z'
labels:
  - cleanup
  - search
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 3100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John decided on 2026-11-25 (MSH-209 decision 3): the material evaluate functions and Evaluation types in crates/mshogi-search/src (chu.rs, shogi.rs, tenjiku.rs, tori.rs, xiangqi.rs; about 40 lines each) have no product caller; only tests use them (src/*/tests.rs, src/*/position_tests.rs, tests/shogi.rs, tests/xiangqi.rs). Remove them and the tests that exist only to check them. The position_tests.rs compare functions are also the search-position differential tests that AGENTS.md requires: remove only their evaluator assertions.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The evaluate functions and Evaluation types of the five games are removed from mshogi-search, with the tests and test helpers that exist only to check them (including the Tori earlier_evaluation test code).
- [ ] #2 The differential tests still compare board, hands, hash, check and legal moves against the native rules; perft and differential tests pass.
- [ ] #3 piece_value and the search evaluation (SearchPosition::evaluate) are unchanged; workspace checks pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.
<!-- SECTION:NOTES:END -->
