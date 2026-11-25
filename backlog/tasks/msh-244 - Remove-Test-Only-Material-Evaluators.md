---
id: MSH-244
title: Remove Test-Only Material Evaluators
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:20:00Z'
updated_date: '2026-11-25T13:02:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Record baseline test counts.
2. Remove the five evaluate functions and Evaluation types, and imports that become unused.
3. Delete tests that only check them; keep piece_value and other assertions in mixed tests; remove only evaluator assertions from the differential tests.
4. Remove the Tori earlier_evaluation test code and update documents that name it.
5. Run workspace checks, integrate, finalize.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.

Removed evaluate/Evaluation from the five adapters; none had a product caller and no open task or Rust example needs them (examples call SearchPosition::evaluate only). Mixed tests keep their piece_value and terminal-band assertions. Tests: 1186 passed, 11 ignored before; 1185 passed, 11 ignored after (one Shogi material test removed). Product lines +6 -189, test lines +8 -199. Checks dropped with the assertions: Shogi and Chu position material totals against native pieces (Tenjiku and Xiangqi still compare); Tori INITIAL weights against the earlier formula; native Shogi, Xiangqi, Tori material arithmetic. Search evaluation consistency tests are unchanged. Follow-up: MSH-270 (Remove Uncalled Search Material Code).
<!-- SECTION:NOTES:END -->
