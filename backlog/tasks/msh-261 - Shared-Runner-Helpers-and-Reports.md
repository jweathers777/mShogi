---
id: MSH-261
title: Shared Runner Reports and Engine Transport
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T23:30:00Z'
labels:
  - runner
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7550
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Shared Runner Helpers and Reports:
After MSH-218 (Shared Protocol Runner), crates/mshogi-runner still repeats code. shogi.rs and xiangqi.rs share about 370 normalized lines (Config settings and from_json, report types and JSON, replay, run); shogi_match.rs and xiangqi_match.rs about 200 (from_json, report JSON, replay, run, admit); engine_for exists three times. Engine identities, SHA-256 digests, rerun eligibility, the platform string and a nanosecond clamp are copied between run.rs, protocol.rs, report.rs and experiments. Failure kinds exist in report.rs, protocol.rs and xiangqi.rs (the Xiangqi enum copies the protocol enum and adds four variants). Xiangqi modules import shared types from shogi.rs and shogi_match.rs. main.rs writes reports in three ways: shogi-match and xiangqi-match use fs::write, which overwrites earlier evidence, and shogi-replay and xiangqi-replay replay twice.

One Engine Transport for the Runners:
crates/mshogi-runner talks to engine processes in three places: run.rs GameRun (Chess), protocol.rs Controller (a fresh process per turn) and clocked.rs Host (persistent processes). Each repeats command writing and logging, the line reader, the ASCII and control-byte check, the handshake with options and the cleanup. They have drifted with no recorded reason: transcript limits (truncate and continue, fail, or stop recording), end-of-file handling, Chess skips the advertised-option check, handshake line caps of 256 and 512, cleanup with or without stop. Clock ledgers are written three times (shogi_match Clock, experiments FischerClock, protocol_match charge and increment) with different bounds.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Shared types and helpers live in one runner module; Shogi and Xiangqi use generic report, replay and run code with hooks only for real differences (Xiangqi pending adjudication and score).
- [x] #2 One failure-kind enum serves all runners; every report output refuses to overwrite an existing file; each replay command replays once.
- [x] #3 Runner tests pass; production line counts before and after are recorded.
- [x] #4 One engine transport serves the three runners; per-runner code holds only its game and clock logic (Chess clocks and claims stay with ChessMatch).
- [x] #5 Each drift is resolved to one behavior and recorded in the notes; one clock ledger type is used where the clocks are the same.
- [ ] #6 Runner tests pass, including the process tests in CI on the affected operating systems.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extract shared evidence, failure categories, clock accounting, report decoding and CLI output. 2. Consolidate recorded and clocked run/replay around native game adapters. 3. Unify bounded engine transport, handshakes and retirement while preserving ChessMatch timing precedence. 4. Consolidate fixture support, exercise drift cases, run affected process CI, obtain fresh review and integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Test duplication found by MSH-209, in scope here: tests/usi_fixture.rs and xiangqi_fixture.rs share about 50 lines of go and quit modes; the engine-builder helper exists four times with only the binary name different; the wrapper test exists three times (runner.rs, shogi_runner.rs, xiangqi_runner.rs). Move the shared parts into one test support module.

John approved consolidation on 2026-11-26: this task retains the full scope of MSH-261 (Shared Runner Helpers and Reports), MSH-262 (One Engine Transport for the Runners). Reports, failure types, process handling and test support must agree across the runner refactor. Agent labels are model:large, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-261-shared-runner from b003738. Next: consolidate runner code, verify process tests and request review.

Checkpoint: generic recorded and clocked game reports now replay through native adapters. Shared evidence helpers, failure enum, Fischer accounting and create-new CLI output are in place. cargo check --locked -j 2 -p mshogi-runner --all-targets passes. Next: transport, test fixtures, complete verification, review and integration on codex/msh-261-shared-runner.

Transport checkpoint on codex/msh-261-shared-runner. All three runners use one transport. Record bounded command and response evidence, including info and deadline-racing output. The first retention overflow stops further admission with resource_limit. All dialects validate configured options against the first matching advertisement; up to 256 nonempty intermediate lines per handshake/readiness phase; USI readiness before newgame, UCI and UCCI also check readiness after newgame. Accept padded ASCII and tabs; reject invalid bytes. Clean EOF is missing_reply, nonzero natural exit is unexpected_exit, and a supervisor kill does not reclassify an earlier EOF. Preserve receipt timestamps and earliest deadline. All children retire with stop, quit, bounded drain, kill/reap and diagnostics. Fresh-process candidates commit only after retirement; already established native outcomes keep subsequent diagnostics. Shared Fischer settings allow base 1..3600000 ms, increment 0..60000 ms, charge before increment; ChessMatch owns Chess clocks and claims. Shared fixture support covers engines, protocol modes, wrapper replay and create-new report files. Native runner production source: 7939 physical lines at b003738 to 7315, 624 fewer, counting src/**/*.rs with comments and blanks but excluding experiments/tests.rs and cfg(test) items. Local runner tests pass; review fixes include contradictory ads, readiness info, receipt ordering, natural exit status, padded tokens and retirement evidence. Next: finish the required workspace check and GitHub process matrix; then integrate and clean up.

9b2d1a5 contains the reviewed implementation. Fresh review approved after separate CLI verification of delayed EOF, duplicate advertisements, readiness info, token padding and retirement diagnostics. python3 tools/checks.py run --base origin/main passed on macOS (format, workspace Clippy, 1172 passing tests/doc tests). No cloud experiments or new compute resources. Remaining: wait for all three process jobs on https://github.com/jweathers777/mShogi/actions/runs/45906698510 , integrate and clean up codex/msh-261-shared-runner.

Hosted process CI 45906698510 passed on Ubuntu and Windows. macOS exposed a reap-budget failure when a live child had closed stdin. Retirement now drains already queued stdout without waiting for EOF after an unsuccessful quit, preserving the remaining budget to terminate and reap. Nine controlled process tests passed locally using the original 100 ms watchdog. Final production count is 7939 to 7326 (613 fewer). Next: repeat the release process matrix and workspace check on the fixed branch, then integrate.

Review of the cleanup fix separated successful wire writes from transcript retention, and incomplete stdout capture from remaining journal capacity. A slow-quit child now exits naturally even with zero transcript bytes. A failed child with open stdout does not suppress the other child’s cleanup evidence. Two real subprocess regressions cover these paths. Superseded CI 45908103363 was cancelled. Corrected production line count is 7939 before and 7333 after (606 fewer). Next: verify final revision on all three hosted platforms, integrate and clean up.

edb4f7b passed the workspace checks and its hosted Ubuntu and Windows jobs (45908447797). The controlled closed-stdin case also passed on macOS, but its Xiangqi retirement test observed an unreaped child under a 300 ms budget. The updated transport gives stop/quit and stdout capture the first half of the absolute deadline, polls stdout in intervals up to 5 ms, and reserves the other half for forced termination, reaping and joining pipe workers. It samples exit status even after a delayed wakeup. The watchdog, native rules and test expectations remain the same. The local full runner suite and the final focused Xiangqi check passed. Production source now totals 7338 lines, 601 fewer than the baseline. Next: final workspace check, affected process CI and integration.

a41e89d passed local workspace, Ubuntu and Windows checks. macOS run 45909991524 found that a real deadline-racing reply can be dequeued during search after its retirement. The transport now records that response as retired and uses one monotonic receipt for its timestamp and deadline classification. The fixture produces its late reply on stop instead of depending on an 80 ms scheduling delay. Shogi and Xiangqi still require the exact retired bestmove and reaped child. Both focused process tests pass locally. Updated production count: 7939 to 7347 (592 fewer). Next: complete the release matrix, then integrate and clean up.
<!-- SECTION:NOTES:END -->
