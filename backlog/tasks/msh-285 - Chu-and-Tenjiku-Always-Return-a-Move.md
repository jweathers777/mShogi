---
id: MSH-285
title: Chu and Tenjiku Always Return a Move
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-28T10:01:00Z'
updated_date: '2026-11-28T10:28:00Z'
labels:
  - chu
  - tenjiku
  - search
  - model:medium
  - effort:medium
milestone: m-5
dependencies: []
documentation:
  - docs/m5/strength-roadmap.md
priority: high
ordinal: 262000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Chu and Tenjiku search use restricted_board! in crates/mshogi-search (chu.rs, tenjiku.rs), so TIMED_FALLBACK is false. When the clock or node limit stops the search before the first iteration completes, the engine returns no move. Shogi and Xiangqi use timed_moves! and always return a legal move. This caused 180 of 192 unscored games in MSH-53.3, the native incomplete:Stopped games in MSH-53.2 and the first-iteration losses in MSH-52.3. In the desktop application the same case can leave the engine without a move under time pressure. Prefer the best root move of the partial first iteration; fall back to the first ordered legal move.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A timed or node-limited Chu or Tenjiku search that stops before depth 1 completes returns a legal move (best partial-iteration root move when one is scored, else the first ordered legal move); explicit cancellation still returns no move
- [ ] #2 Focused tests cover a stop before the first completed iteration for both games; rules perft and existing search tests pass
- [ ] #3 Tenjiku and Chu each pass an SPRT against the frozen predecessor at their normal fast control (or are neutral with simpler code); results, cost and GCS path recorded
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Handoff: Claimed on branch codex/msh-285-always-return-move. Next: inspect root search and verify the fallback in both games.
<!-- SECTION:NOTES:END -->
