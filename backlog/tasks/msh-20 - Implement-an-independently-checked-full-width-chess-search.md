---
id: MSH-20
title: Implement an independently checked full-width chess search
status: In Review
assignee:
  - '@search-builder'
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T23:21:00Z'
labels:
  - search
milestone: m-2
dependencies:
  - MSH-17
  - MSH-19
documentation:
  - docs/design.md
  - docs/search.md
priority: high
ordinal: 20000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Design section 6 requires search correctness to be established on small independently solvable positions or synthetic trees before relying on pruning or transposition reuse. Perft and rules tests do not establish search correctness, and a speedup is not evidence of strength. Search must run on private working state, never on the caller's game (A-05).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A full-width search returns only legal actions and correct scores on independent synthetic trees and on small solvable chess positions (for example forced mates with known depth), covering score perspective and terminal scoring
- [x] #2 Search runs on private working state and leaves the caller's position and history unchanged
- [x] #3 Any pruning or transposition reuse is added only with targeted regressions and does not reuse history-dependent results solely by board key
- [x] #4 A measured search performance baseline is recorded with hardware, build and parameters, with no invented target
- [x] #5 Search consumes the rules-owned game context and adjudication, including relevant history and the selected claim policy. Tests use the same current position with differing relevant histories and distinguish available optional claims from automatic outcomes; the caller game and history remain unchanged.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Implement a deterministic full-width native search over private AdjudicatedGame context, consulting outcomes and selected optional/auto claim policy before static leaves. 2. Verify common search recursion against independent synthetic trees and hand-solvable chess positions, including identical current positions with different relevant histories, legal returned actions and unchanged caller context. 3. Record measured release performance with build/hardware/limits; obtain independent review and integrate with native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Primary integrator Codex delegates isolated implementation to search-builder on shared branch codex/m2-remaining after MSH-17 completion. Owned files are mshogi-search search implementation/tests/examples/docs only. MSH-18 is independently eligible and uses separate core match files. No shared-file competing builder is permitted.

Isolated implementation by search_builder is complete and independently reviewed by review_evaluator with no material findings. Full-width iterative negamax over private AdjudicatedGame; explicit optional zero-choice versus required AutoClaim, mate distance, legal PV, deterministic depth/node limits and completed-iteration-only publication. Focused verification: 2 synthetic recursion tests, 9 search integration tests, 5 existing evaluator tests and 1 doctest; search Clippy -D warnings, format and diff checks pass. Measured release baseline in docs/search.md: Apple M1 Pro 10-core/32GB macOS27.0.1 Rust1.89, standard depth3 9765 nodes in 1435.762ms; no strength claim. Independent reviewer reran all search tests.
<!-- SECTION:NOTES:END -->
