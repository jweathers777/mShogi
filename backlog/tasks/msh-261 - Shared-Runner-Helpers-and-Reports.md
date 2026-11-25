---
id: MSH-261
title: Shared Runner Helpers and Reports
status: Ready
assignee: []
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-25T12:24:00Z'
labels:
  - runner
  - refactor
  - model:medium
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7550
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After MSH-218 (Shared Protocol Runner), crates/mshogi-runner still repeats code. shogi.rs and xiangqi.rs share about 370 normalized lines (Config settings and from_json, report types and JSON, replay, run); shogi_match.rs and xiangqi_match.rs about 200 (from_json, report JSON, replay, run, admit); engine_for exists three times. Engine identities, SHA-256 digests, rerun eligibility, the platform string and a nanosecond clamp are copied between run.rs, protocol.rs, report.rs and experiments. Failure kinds exist in report.rs, protocol.rs and xiangqi.rs (the Xiangqi enum copies the protocol enum and adds four variants). Xiangqi modules import shared types from shogi.rs and shogi_match.rs. main.rs writes reports in three ways: shogi-match and xiangqi-match use fs::write, which overwrites earlier evidence, and shogi-replay and xiangqi-replay replay twice.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Shared types and helpers live in one runner module; Shogi and Xiangqi use generic report, replay and run code with hooks only for real differences (Xiangqi pending adjudication and score).
- [ ] #2 One failure-kind enum serves all runners; every report output refuses to overwrite an existing file; each replay command replays once.
- [ ] #3 Runner tests pass; production line counts before and after are recorded.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Test duplication found by MSH-209, in scope here: tests/usi_fixture.rs and xiangqi_fixture.rs share about 50 lines of go and quit modes; the engine-builder helper exists four times with only the binary name different; the wrapper test exists three times (runner.rs, shogi_runner.rs, xiangqi_runner.rs). Move the shared parts into one test support module.
<!-- SECTION:NOTES:END -->
