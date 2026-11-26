---
id: MSH-256
title: One USI and Xiangqi UCI Front End
status: Done
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T18:52:00Z'
labels:
  - console
  - protocol
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7250
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In crates/mshogi-console, usi.rs (731 lines) and xiangqi_uci.rs (632) share about 480 normalized lines: the search worker and mailbox, retire, isready, quit, stop and go handling, number parsing, report and run. Real dialect differences: USI handshake and usinewgame, gameover, go mate, SFEN, byoyomi and bestmove win or resign; Xiangqi UCI side naming and the Hash option. Differences with no recorded reason: only USI checks time and generation before it accepts a result; a bare go searches 1 s in USI but uses fixed limits in Xiangqi; only Xiangqi reuses a position that the new move list extends; USI has no Hash option; Xiangqi has no movestogo. uci.rs (Chess) has its own line reader (a line reader copied from main.rs that also ends a line at CR), and the ASCII and control-byte check exists three times.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 One generic protocol worker and loop serves USI and Xiangqi UCI through a dialect trait that holds only real dialect differences; the Chess UCI front end uses the same line reader and byte check, without restoring the removed input line cap for records and position commands.
- [x] #2 Each difference without a reason is resolved to one behavior and recorded in the notes; console protocol tests pass.
- [x] #3 Production line counts before and after are recorded.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Measure the cost of checking a Xiangqi history prefix before replacing the two console search sessions. 2. Share input framing, byte validation, the search worker, result admission, position replay, options and time controls. Keep native roots, side names, score units, special commands and result encoding in the dialects. 3. Verify protocol regressions, production line counts, repository checks, independent review and process CI on all supported operating systems.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: MSH-268 removes the input line cap for game records and position commands; do not add it back in the shared front end.

MSH-222 (2026-11-26), finding in this scope: ShogiSearchSession and XiangqiSearchSession now live in mshogi-engine/src/server.rs. They could be one SearchSession<G: EngineGame> that uses EngineGame::extends; the Xiangqi session compares events directly, which is cheaper than a committed_prefix replay, so measure before merging.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-256-protocol at 655e63b. Next: compare the protocol paths and record the implementation plan.

Both protocols now use the same worker and exact immutable request identity, covering generation, position, hash, time and limits. Bare go uses one second unless Depth or Nodes selected fixed work. Both reuse an admitted move prefix, accept Hash (1–1024 MiB, defaults 16 for Shogi and 64 for Xiangqi) and positive movestogo (default 30). Hash applies to timed and fresh fixed-work tables. USI alone accepts byoyomi, gameover and go mate. Shared CR/LF protocol framing and byte checks preserve unlimited input; shell framing remains LF. On the Apple silicon Mac, release benchmark at 655e63b, 32 legal Xiangqi moves vs its 31-move prefix: canonical record construction + has_prefix 6864 ns, native root/policy/events 176 ns, native root + parsed protocol moves 98 ns per comparison (5000/50000/50000 repetitions with black_box). Each protocol fixes claim policy and can submit only board moves. This permits one generic search session using exact root and admitted actions. Existing 39 focused release tests passed before adding new coverage. Branch codex/msh-256-protocol. Next: independent review, final local/CI checks and counts.

Fresh independent review found an incompatible timed table could remain allocated after a new game or Hash resize followed by fixed work. Every request now drops that state before selecting its search mode; a fixed-work regression covers resize and new game. The retained table holds only a shared root/move list, so it releases the previous native game. Review also exercised 128 KiB input, CR framing, invalid bytes, retirement, suffixes and shutdown in both release processes. A pre-existing zero-main-time byoyomi defect is tracked separately in MSH-275 (Use the Current Byoyomi Period in USI Overtime). The USI guide describes the limitation. Console production Rust lines (excluding trailing cfg(test) modules) 3967 to 3579; all .rs lines under console/src 4144 to 3828. Protocol/reader files alone, production lines, 1760 to 1382. Next: finish local and three-platform process checks, integrate and clean.

Verified code 35967d3 against 655e63b on the arm64 Mac. CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2 python3 tools/checks.py run --base origin/main passed formatting, workspace Clippy and all 1165 Rust tests (10 pre-existing ignored). TypeScript passed. Embedded Node 24 could not load the signed native test dependency; the standalone Node 24.21.0 rerun of python3 tools/checks.py run --suite frontend passed TypeScript and all 575 tests. Backlog doctor and git diff --check passed. Targeted release process CI is running on all three systems at https://github.com/jweathers777/mShogi/actions/runs/45877067728 on 35967d3. Next: confirm all three, integrate, close MSH-256 and clean codex/msh-256-protocol.

Commit 35967d3 passed the GitHub push checks and all three release process jobs in run 45877067728: Ubuntu 24.04, macOS 15 and Windows 2022 ran cargo test --release --locked -p mshogi-engine -p mshogi-runner -p mshogi-console --tests. Merged into main at 2712943. Local verification and the original 655e63b baseline are recorded above. No Cloud Batch or Cloud Build jobs, GCP cost or GCS run artifacts. The independent benchmark built mshogi-core in release, then ran a rustc -O harness selecting legal[(17*i+7) % legal.len()] for plies i=0..31 from the standard Xiangqi root, and compared against the 31-move prefix.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
USI and Xiangqi UCI share search, clocks, position updates and request retirement. All three machine protocols use the same uncapped line reader and byte validation. Production console Rust fell from 3967 to 3579 lines. Independent review, local checks and release process CI passed on Ubuntu, macOS and Windows. MSH-275 tracks the existing USI overtime issue. Next eligible task: MSH-258 (One Search Root Adapter).
<!-- SECTION:FINAL_SUMMARY:END -->
