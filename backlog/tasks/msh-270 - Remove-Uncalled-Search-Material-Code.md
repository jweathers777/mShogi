---
id: MSH-270
title: Remove Uncalled Search Material Code
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T13:01:00Z'
updated_date: '2026-11-25T22:34:00Z'
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
- [ ] #1 ShogiPosition::material, its incremental material field and ChuPosition::material are removed; the Chu board material that evaluation uses stays
- [ ] #2 Shogi piece values are defined in one place and keep the same numbers
- [ ] #3 Search results and evaluation are unchanged; workspace checks pass
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-25: John approved moving this ahead of MSH-222 (Shared Engine Adapters) with the other deletion tasks.
<!-- SECTION:NOTES:END -->
