---
id: MSH-270
title: Remove Uncalled Search Material Code
status: Done
assignee:
  - '@claude'
created_date: '2026-11-25T13:01:00Z'
updated_date: '2026-11-25T22:40:00Z'
labels:
  - cleanup
  - search
  - model:medium
  - effort:low
milestone: m-6
dependencies: []
priority: medium
ordinal: 3900
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-244 (Remove Test-Only Material Evaluators) removed the last callers of ShogiPosition::material and ChuPosition::material in crates/mshogi-search. shogi::piece_value now has only test callers and repeats the VALUE table in shogi/position.rs, which exchange and ordering use; no test compares the two now.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ShogiPosition::material, its incremental material field and ChuPosition::material are removed; the Chu board material that evaluation uses stays
- [x] #2 Shogi piece values are defined in one place and keep the same numbers
- [x] #3 Search results and evaluation are unchanged; workspace checks pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Check the callers. 2. Remove ShogiPosition::material, its field, ChuPosition::material and shogi::piece_value. 3. Build the untuned Shogi INITIAL weights from the VALUE table. 4. Run the checks and integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-25: John approved moving this ahead of MSH-222 (Shared Engine Adapters) with the other deletion tasks.

Caller check: ShogiPosition::material, ChuPosition::material and ShogiPosition's material field had no readers. shogi::piece_value had only test callers (shogi/position_tests.rs). No open task plans to use them. The Chu Board material field stays; evaluation and tactical code use it.
Values: shogi::piece_value and VALUE agreed for all 14 faces (King 0; promoted Silver, Knight, Lance and Pawn 500). piece_value also answered for a promoted King or Gold, which are not faces. The INITIAL weights in shogi/eval.rs repeated the same numbers, so they are now built from VALUE; a temporary test (not committed) showed that all 133 weights equal the old literal array. DEFAULT weights and tuning bounds did not change.
Search unchanged: fixed_search_matches_recorded_baseline (mshogi-engine) passed with the recorded baseline file unchanged (action, node count and score at depth 2 for all profiles). The change removes only unread incremental updates from the Shogi make/unmake path.
Checks on commit of the code change, macOS: cargo fmt --check pass; cargo clippy --workspace --all-targets --locked -D warnings pass; cargo test --workspace --locked 1157 passed, 0 failed, 10 ignored. No strength match or cloud job.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed the unread Shogi incremental material (field and accessor), ChuPosition::material and shogi::piece_value. The VALUE table in shogi/position.rs is now the only definition of Shogi piece values; the untuned INITIAL weights are built from it with the same numbers. Evaluation and search results are unchanged (recorded fixed-search baseline passes; 1157 workspace tests pass, fmt and clippy clean). No follow-up tasks.
<!-- SECTION:FINAL_SUMMARY:END -->
