---
id: MSH-21
title: Isolate search requests with identity, cancellation and stale-result rejection
status: Done
assignee:
  - '@search-builder'
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T23:31:00Z'
labels:
  - search
  - concurrency
milestone: m-2
dependencies:
  - MSH-20
documentation:
  - docs/design.md
priority: high
ordinal: 21000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
A-05 requires that every search request and result identify the request and its base position and version, that superseded results cannot change the current game, and that cancellation neither corrupts the game nor blocks behind unbounded progress output. Protocol adapters, the match runner and the M3 desktop all depend on this contract.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Each search request and result carries the request identity and base position/version, and a result for a superseded request is rejected
- [x] #2 Cancellation returns promptly within a documented bound, leaves game state intact, and is tested, including delayed results after a position change
- [x] #3 Progress reporting is bounded and cannot block the game or corrupt protocol output
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
1. Add a bounded native request coordinator around the existing stop-aware search, with unique request IDs, exact base game context and version, private working state and stale-result rejection. 2. Keep pending/result/progress storage bounded and cancellation nonblocking; document cooperative node-boundary acknowledgement and validate delayed/superseded results. 3. Test cancellation, repeated/same-position different-history requests and caller preservation; obtain independent review, integrate and record native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Delegated isolated search-crate coordinator implementation to search_builder after MSH-20 completion. Parent Codex owns integration and MSH-18 core match work. No core/console or Backlog changes by delegate.

Implemented bounded native worker coordination with process-unique identity, exact game-record context and version, private snapshots, replacement cancellation, explicit completion envelopes and stale-result rejection. One pending and one completion slot; no progress stream. Cancellation signals without joining and acknowledges at a documented node/unwind boundary subject to OS scheduling. Focused cargo test -p mshogi-search --locked passed 27 tests including active-worker races and 8 coordinator integration tests; formatting and Clippy passed. Independent review by review_hardening over 523ba28 found no material issues and independently reran the 27 tests. Consumer serialization requirement and work-bound limitations documented in docs/search-coordination.md.

Integrated normally into origin/codex/rewrite at source e438f7c8bdcab56827a743da66729ebc7de44bd4. Native CI https://github.com/jweathers777/mShogi/actions/runs/40143002342 passed Rust (windows-2022) (job 121018470180), Rust (macos-15) (job 121018470322), Rust (ubuntu-24.04) (job 121018470381); formatting, Clippy, locked workspace tests/build and console smoke all passed. Closure bookkeeping reuses this exact source evidence. Existing deep/oracle evidence is retained for unchanged rules.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added bounded native search coordination with exact history/version identity, private snapshots, cooperative cancellation and stale-result rejection. Focused race/history tests and independent review passed; integrated into codex/rewrite with native three-platform CI. Protocol scheduling remains MSH-22.
<!-- SECTION:FINAL_SUMMARY:END -->
