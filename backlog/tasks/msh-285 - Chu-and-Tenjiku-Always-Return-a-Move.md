---
id: MSH-285
title: Chu and Tenjiku Always Return a Move
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-28T10:01:00Z'
updated_date: '2026-11-28T10:51:00Z'
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
- [x] #1 A timed or node-limited Chu or Tenjiku search that stops before depth 1 completes returns a legal move (best partial-iteration root move when one is scored, else the first ordered legal move); explicit cancellation still returns no move
- [x] #2 Focused tests cover a stop before the first completed iteration for both games; rules perft and existing search tests pass
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
Feasibility: Freeze main at 2ccc003 for both comparisons. Use c3-standard-4 with the prior normal clocks and default 5M-node and depth-64 safety caps. Chu at 10+0.1 can charge at most 240 s per 1,000-ply color pair before overhead, within the prior 600 s limit. Tenjiku at 60+1.5 scored 72/80 pairs in MSH-53.2 with 768 plies per game, a 1,200 s pair limit, two concurrent pairs per VM and a 1,260 s command within the 1,800 s job. Three of 160 games hit the wall, so retain every cutoff as unscored. At $0.201608 per VM-hour, a full eight-VM block reserves $0.81, earlier Tenjiku blocks cost $0.32–$0.61, and two Linux builds reserve $0.41. The builds, blunder screen and the first block for each game/profile fit under $4. Continue within the $10 task cap.

Handoff: Candidate b0acbf0 on codex/msh-285-always-return-move. Search and structured engine fallback implemented with specific tests. Next: finish workspace checks, run Linux blunder and predecessor checks, and integrate after acceptance.

Search and engine suites passed before the final consumer fixture update, including both Tenjiku profiles, Chu, perft and differential tests. Updated console integration tests and the three desktop snapshots that cover first-iteration node exhaustion. Full workspace and frontend checks are running. The Linux builds compare b0acbf0 with 2ccc003. Subsequent edits so far are test or verification-fixture only.

Local checks passed: workspace format, Clippy and 1,175 Rust tests (10 ignored), including existing search, independent Chu/Tenjiku perft, differential checks and real-console admission. Desktop type checking and all 578 frontend tests passed with Node 24.19.0. The native runner applied and replayed both one-node fallbacks for Chu, Adams and Muller in three local calibration pairs. A frozen-main regeneration proved the four completed Tenjiku snapshots already differed from their generator. Filed MSH-289 and kept these four snapshots outside the fix.

Handoff: Product candidate b0acbf0 with consumer test checkpoints on codex/msh-285-always-return-move. Linux candidate and frozen 2ccc003 built on c3-standard-4. Both build cleanups verified. Next: compare blunders and clocked SPRT, then integrate only after AC3 resolves. Cloud evidence: gs://mshogi-experiments-216675633801/runs/msh-285/.

b0acbf0 vs 2ccc003, c3-standard-4: tools.strength.cli blunders at depth 3, Chu 300K nodes and Adams/Muller 500K nodes, using MSH-52.3 and MSH-53.3 suites. All 43 positions match in choices, scores and nodes. Both Linux builds and the screen cost at most $0.0725 combined VM compute, with no jobs, VMs or disks left. GCS gs://mshogi-experiments-216675633801/runs/msh-285/blunders/.

Apply the roadmap search SPRT (0/15 Elo, alpha=beta=.05, minimum 30 scored pairs independently for Chu, Adams and Muller) before the first timed match. Use the established fast clocks. Next: Chu, then the two Tenjiku profiles on branch codex/msh-285-always-return-move.
<!-- SECTION:NOTES:END -->
