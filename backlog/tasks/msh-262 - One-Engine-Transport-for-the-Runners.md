---
id: MSH-262
title: One Engine Transport for the Runners
status: Ready
assignee: []
created_date: '2026-11-25T12:22:00Z'
labels:
  - runner
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7600
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
crates/mshogi-runner talks to engine processes in three places: run.rs GameRun (Chess), protocol.rs Controller (a fresh process per turn) and clocked.rs Host (persistent processes). Each repeats command writing and logging, the line reader, the ASCII and control-byte check, the handshake with options and the cleanup. They have drifted with no recorded reason: transcript limits (truncate and continue, fail, or stop recording), end-of-file handling, Chess skips the advertised-option check, handshake line caps of 256 and 512, cleanup with or without stop. Clock ledgers are written three times (shogi_match Clock, experiments FischerClock, protocol_match charge and increment) with different bounds. Do this after MSH-261 (Shared Runner Helpers and Reports).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One engine transport serves the three runners; per-runner code holds only its game and clock logic (Chess clocks and claims stay with ChessMatch).
- [ ] #2 Each drift is resolved to one behavior and recorded in the notes; one clock ledger type is used where the clocks are the same.
- [ ] #3 Runner tests pass, including the process tests in CI on the affected operating systems.
<!-- AC:END -->
