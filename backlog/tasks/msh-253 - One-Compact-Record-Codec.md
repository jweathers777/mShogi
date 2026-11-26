---
id: MSH-253
title: Shared Record Codec and Incremental Engine Sessions
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T16:06:00Z'
labels:
  - core
  - persistence
  - refactor
  - engine
  - model:large
  - effort:high
milestone: m-6
dependencies:
  - MSH-252
priority: medium
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
One Compact Record Codec:
tori/record.rs, chu/record.rs and tenjiku/record.rs in mshogi-core repeat the same codec: strict() (identical in all three), replay of the commands with stop checks, the re-encode-and-compare check, from_json_with_stop, to_game, to_game_with_undo, GameDelta suffixes (Tori and Chu), Serialize and Deserialize, and a RecordError enum that native.rs maps three times (Stopped, Capacity). Xiangqi and Shogi records follow a similar pattern. Only the root data and the command type differ by game.

Incremental Engine Sessions for Every Game:
The engine keeps a retained native game between turns (incremental session: mshogi-core native/incremental.rs and mshogi-engine incremental.rs) only for Shogi (MSH-189), Tori (MSH-204) and Chu (MSH-213); each was added during the strength work of that game. Xiangqi, Tenjiku and Chess replay the full record on each request. The Tenjiku harness (MSH-214) will play long games through the same route.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic compact record codec in mshogi-core serves Tori, Chu and Tenjiku, including a suffix delta for each; per-game code holds only its root data and command type; Xiangqi and Shogi use it too, or the notes record why not.
- [ ] #2 native.rs maps one record error type; committed records, fixtures and envelope test outputs are byte-identical before and after.
- [ ] #3 Production line counts before and after are recorded; workspace checks pass.
- [ ] #4 Incremental sessions are a capability of the generic engine adapter; each game that has a record suffix uses them without per-game session code; a game left out has a recorded reason.
- [ ] #5 Engine protocol and incremental tests cover each game; the replay cost per request at full length is measured before and after for Xiangqi and Tenjiku.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Capture exact envelope/fixture output and full-length replay timings from a63fa47. 2. Use the shared history owner for a generic compact codec and atomic suffix. Adapt Xiangqi command boundaries and Shogi through the common session interface. 3. Dispatch retained engine requests through EngineGame, exercise protocol and suffix rejections, and compare wire bytes, production counts and costs. 4. Run selected workspace and platform checks, independent review, integrate and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Context carried from MSH-260 (Incremental Engine Sessions for Every Game); historical observations, not a replacement for the current criteria and dependency fields:
MSH-222 (2026-11-26), finding in this scope: EngineGame (mshogi-engine adapter.rs) has KIND, record, extends (record prefix) and valid_limits. The incremental Adapter impls (ShogiAdapter, turn_adapter for Tori and Chu) can become one impl over EngineGame where Record: IncrementalRecord<Game = Game>; server.rs dispatch_request still has three per-game arms and execute_retained::<G> is already generic.

John approved consolidation on 2026-11-26: this task retains the full scope of MSH-253 (One Compact Record Codec), MSH-260 (Incremental Engine Sessions for Every Game). The shared codec provides the record suffix used by incremental engine sessions; verify both ends together. Agent labels are model:large, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-253-record-codec at a63fa47. Next: baseline evidence, codec and retained adapter, then checks and review.

Checkpoint on codex/msh-253-record-codec. Implemented one codec for the three shared GameHistory games, Xiangqi multi-event suffixes, and generic retained engine sessions. Shogi keeps its schema-specific canonical SFEN, declaration/score and diagnostic conversion, while using the same engine session. Xiangqi retains its root codec, claim policy and many-events-per-command replay. Chess retains its distinct adjudicated match record with clocks and observations, has no validated suffix, and uses full requests. Frozen and new envelope suites passed with 23 distinct byte-identical envelopes. Baseline complete-game measurements (seed 253, release, nine final zero-node timed requests) on macOS arm64: Xiangqi 81 events 15,746 us; Adams 175 events 3,703 us; Muller 161 events 3,456 us. Next: finish added regression tests, compare 42 committed game documents, run workspace and platform checks and fresh review, then integrate.

Independent review of a63fa47..66a670a found a preflight performance regression, a coverage gap and stale protocol documentation. The branch now checks Xiangqi request tokens before checkpointing, tests retained move analysis against valid and forged same-ply targets, and documents timed retention for all five adapters. Byte comparison of reconstructed canonical records in all 42 committed .mshogi files, 23 envelope-test outputs and six generated full/base records succeeded. Clippy type alias and Xiangqi test expectation corrected: a rejected count claim emits two audit events; resignation emits two more. Next: finish workspace checks, compare native admission at 1,000 commands, collect targeted CI, integrate and clean up.

The retained-analysis regression exposed a decoder defect: when the outer delta content preceded its kind, Serde buffered and lost the raw inner bytes. The transport now captures that field as RawValue before selecting the native decoder. A reordered forged request returns InvalidRequest and the valid target still searches. The focused engine test passes. Long 1,000-command comparisons and end-to-end workspace/CI verification are in progress. Branch codex/msh-253-record-codec.

Evidence on Apple M1 Pro, Rust 1.99.1 release, a63fa47 versus 25805f0. Deterministic resource_admission walk through the 1,000th command (999-command base plus one board action), median of nine requests. Time for request decode, native replay and prefix admission excluding search/process I/O, in microseconds: Xiangqi 82,906 -> 50; Tenjiku Adams 15,825 -> 50; Muller 15,463 -> 48. Whole zero-node timed request over process pipes: Xiangqi 83,131 -> 87 us and 59,577 -> 511 bytes; Adams 16,461 -> 104 us and 135,695 -> 781 bytes; Muller 15,999 -> 105 us and 136,298 -> 751 bytes. Each advance admits the one new command. Complete seed-253 games ended at 81, 175 and 161 events respectively. Their final native admission was 15,747 -> 54 us, 3,383 -> 49 us and 3,201 -> 51 us. All 42 committed .mshogi native record outputs, 23 distinct envelope-test outputs and six generated complete/prefix envelopes compared byte for byte. Production lines in all changed Rust src files, with blank/comment lines and without test-only items/files: 8,075 -> 7,620 (-455). Inputs, workload source, commands and results: gs://mshogi-experiments-216675633801/runs/msh-253/record-admission.tar.gz (64,087 bytes). No cloud VM or build; cloud compute cost US$0. Current branch: codex/msh-253-record-codec. Remaining: final workspace, frontend fixture comparison, process CI 45854759990 and persistence CI 45854765652; then integrate, finalize, clean up.
<!-- SECTION:NOTES:END -->
