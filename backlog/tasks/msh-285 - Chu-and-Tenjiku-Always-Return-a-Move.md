---
id: MSH-285
title: Chu and Tenjiku Always Return a Move
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-28T10:01:00Z'
updated_date: '2026-11-28T10:37:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Enable root fallback for Chu and both Tenjiku profiles at time and node limits. Reuse the shared root search to keep the best scored partial move, or the first rules-ordered legal move when none was scored.
2. Preserve cancellation, legality and completed-depth telemetry. Check search, structured engine admission and rules perft, then update the applicable documentation.
3. Run the blunder screen and fast-control predecessor SPRTs on Cloud Batch after recording feasibility and cost. Integrate after checks pass.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Handoff: Claimed on branch codex/msh-285-always-return-move. Next: inspect root search and verify the fallback in both games.

Feasibility: Freeze main at 2ccc003 for both comparisons. Use c3-standard-4 and the prior normal-clock method with the default 5M-node and depth-64 safety caps. Chu at 10+0.1 can charge at most 240 s per 1,000-ply color pair before overhead, within its prior 600 s limit. At Tenjiku 60+1.5, MSH-53.2 scored 72/80 pairs within 768 plies per game and 1,200 s per pair, with only three wall stops across 160 games. Double-color total can exceed that for unusually long games. Use two concurrent pairs per VM and the proven 1,260 s command within the 1,800 s job. Keep all reports and treat any cutoffs as unscored. The rate is /bin/zsh.201608 per VM-hour. A full eight-VM match block costs at most /bin/zsh.81, earlier Tenjiku blocks cost /bin/zsh.32–/bin/zsh.61, and the two Linux builds reserve at most /bin/zsh.41 combined. Initial builds, blunder screen and the first block for each game/profile fit under  before further blocks. Stop at SPRT decision or before the  task cap.

Handoff: Implemented and locally checked search and structured engine fallback for Chu and both Tenjiku profiles. Branch codex/msh-285-always-return-move. Next: finish the shared workspace checks and run cloud blunder and predecessor checks.
<!-- SECTION:NOTES:END -->
