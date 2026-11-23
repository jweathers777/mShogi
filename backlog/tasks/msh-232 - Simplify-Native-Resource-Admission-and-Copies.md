---
id: MSH-232
title: Simplify Native Resource Admission and Copies
status: Ready
assignee: []
created_date: '2026-11-23T11:05:00Z'
updated_date: '2026-11-23T11:09:00Z'
labels:
  - simplification
  - core
  - performance
milestone: m-6
dependencies: []
priority: high
ordinal: 239000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Split from MSH-229: Chu and Tenjiku encode complete histories and hypothetical ending/Stop continuations to prove capacity on ordinary commands; Tori retains encoded-length and reservation accounting. Replay-work counters and undo/transaction snapshots add further state and copies. Replace unnecessary accounting and history copying with the simplest measured bounds that keep commands atomic, supported games saveable and parsing/replay cancellable. Xiangqi encoding/copy costs are already MSH-227; native consistency coordination is MSH-209. Do not replace game-rule limits, such as Shogi move-count rules, with application limits.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Measure admission/replay time and allocations or copied/encoded bytes for representative short and long Tori, Chu and Tenjiku histories; identify unused or redundant accounting
- [ ] #2 Remove unnecessary work models, hypothetical-event sizing and whole-history copies in bounded reviewed steps while preserving no partial mutation, no loss of the accepted game and bounded cancellation
- [ ] #3 Rules fixtures and perft remain unchanged; record line counts and pass required Rust/frontend/affected-platform checks after each step
- [ ] #4 Remove unused multi-fact/omitted-resource accounting from native and frontend diagnostics, unless a concrete current producer requires it; retain bounded single-cause and unknown diagnostics
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-229 fresh review also found dead outgoing resource-diagnostic generality: ResourceFailure::multiple/MAX_RESOURCE_FACTS/omitted_facts and frontend multiple-cause fallback have no production multi-fact producer. Every current refusal is one typed resource fact; multi-fact/omitted cases only exist in tests. Simplify this DTO and its callers alongside resource accounting; preserve the one real cause, unknown fallback, output byte/UTF-8 bounds and useful limit/recovery feedback.
<!-- SECTION:NOTES:END -->
