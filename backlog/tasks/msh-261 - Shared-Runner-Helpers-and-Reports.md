---
id: MSH-261
title: Shared Runner Reports and Engine Transport
status: Ready
assignee: []
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T11:55:00Z'
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
- [ ] #1 Shared types and helpers live in one runner module; Shogi and Xiangqi use generic report, replay and run code with hooks only for real differences (Xiangqi pending adjudication and score).
- [ ] #2 One failure-kind enum serves all runners; every report output refuses to overwrite an existing file; each replay command replays once.
- [ ] #3 Runner tests pass; production line counts before and after are recorded.
- [ ] #4 One engine transport serves the three runners; per-runner code holds only its game and clock logic (Chess clocks and claims stay with ChessMatch).
- [ ] #5 Each drift is resolved to one behavior and recorded in the notes; one clock ledger type is used where the clocks are the same.
- [ ] #6 Runner tests pass, including the process tests in CI on the affected operating systems.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Test duplication found by MSH-209, in scope here: tests/usi_fixture.rs and xiangqi_fixture.rs share about 50 lines of go and quit modes; the engine-builder helper exists four times with only the binary name different; the wrapper test exists three times (runner.rs, shogi_runner.rs, xiangqi_runner.rs). Move the shared parts into one test support module.

John approved consolidation on 2026-11-26: this task retains the full scope of MSH-261 (Shared Runner Helpers and Reports), MSH-262 (One Engine Transport for the Runners). Reports, failure types, process handling and test support must agree across the runner refactor. Agent labels are model:large, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.
<!-- SECTION:NOTES:END -->
