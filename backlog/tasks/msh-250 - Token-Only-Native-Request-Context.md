---
id: MSH-250
title: Token-Only Native Request Context
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T08:50:00Z'
labels:
  - core
  - simplification
  - performance
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 6700
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each native command request carries a full GameContext: the root and the complete event or command history, plus a runtime token in Xiangqi, Tori, Chu and Tenjiku. Callers build it with game.context(), which copies the whole history (about 150 call sites outside tests), and submit compares it field by field. Record replay does this for each event, so replay is quadratic in game length. Shogi submit also clones the whole Game three times per command (its GameUndo keeps full before and after copies). The runtime token alone already changes at each accepted command and at each retirement. This task includes Xiangqi and runs before MSH-227 (Xiangqi Record Replay Speed), as John decided on 2026-11-25; record in MSH-227 what changes for Xiangqi.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Shogi, Xiangqi, Tori, Chu and Tenjiku requests are bound by one small token type without a copy of the history, with the same stale-request refusals as before; Chess keeps its version admission or uses the same token.
- [ ] #2 Shogi admission and undo no longer copy the whole game per command.
- [ ] #3 Rules, perft and record tests pass unchanged; replay time per move at full length is measured before and after for each changed game.
- [ ] #4 One review by a fresh agent session.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. One shared request token in mshogi-core native (GameContext: instance + revision, Copy). Each game re-exports it as its GameContext, so callers keep expected: game.context(). Remove per-game RuntimeToken, GameContext structs, same_semantics, runtime(), Tenjiku search_identity.
2. Shogi: Game gets the token and an undo node; submit uses a length checkpoint and rollback (no Game clones); GameUndo keeps the checkpoint and is bound by instance and node; a clone is a new instance; Game equality compares history only.
3. Xiangqi: token-only request check; GameUndo bound by instance and node (no command/lineage copies). transaction_copy stays for MSH-227.
4. Tori, Chu, Tenjiku: token-only request check; undo shapes stay for MSH-252.
5. Update callers, tests and docs; measure with resource_admission (extended to Shogi and Xiangqi) before and after; fixed-search baseline; checks; review; integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Measurement tools that exist (MSH-209): mshogi-core examples record_replay and resource_admission (replay and admission time on generated walks), tenjiku_rules_work (Tenjiku modes play, controls, replay, undo), mshogi-engine example execution_cost.

Baseline (before, commit be774b8 = main 88e9572 + measurement tool), Apple M1 Pro, release: target/release/examples/resource_admission GAME N (11 admission/undo samples median, 5 replay samples median). Two passes, pass 2 shown (us): shogi 318: admission 91.8, undo 44.3, replay 19943 (62.7 per event); xiangqi 1000: admission 91.8, undo 2.2, replay 160577 (160.6 per event); tori 1000: admission 65.6, undo 155.7, replay 41972 (42.0 per event); chu 1000: admission 106.1, undo 0.1, replay 674203 (674 per event); tenjiku-adams 1000: admission 81.9, undo 2.0, replay 47370 (47.4 per event); tenjiku-muller 1000: admission 84.0, undo 2.6, replay 48778 (48.8 per event). Shogi record replay uses append_transaction, not submit. Production lines before (crates/*/src and desktop/src-tauri/src, no test files): 72686.

Checkpoint 2026-11-26 (branch claude/msh-250-request-token, commit 7cce8b2): implementation done; review next.
Design: crates/mshogi-core/src/native/context.rs has the one shared GameContext (instance, revision; Copy, Eq). Each game re-exports it, so callers keep expected: game.context() and no call site changed. GameContext::fresh() takes a new instance from one process-wide u64 counter (infallible; it cannot run out). advanced() keeps a checked revision so the existing Capacity(Counter) refusals and the Tenjiku ending reserve stay as they are (MSH-252 removes them). Removed: five per-game GameContext structs with root/history copies, RuntimeToken (Xiangqi, Tori, Chu, Tenjiku), GameContext::same_semantics/runtime/profile/root/commands, Tenjiku search_identity, four NEXT_INSTANCE statics, Xiangqi lineage vector and undo copies (after checkpoint, after_commands, after_lineage), Shogi Game clones in submit and undo. fork() of Xiangqi/Tori/Chu/Tenjiku and EngineGame::fork are now infallible (the only failure was instance exhaustion).
Shogi: Game holds the token and a history node (revision of the last command still in history). submit and the record append share append_transaction (length checkpoint + rollback, token advance on success). GameUndo = checkpoint + instance + before/after node. Game::clone is a new instance (fresh token); Game equality compares root and history only. MutualCheckDiagnostic now has root and candidate_commands fields.
Xiangqi: GameUndo bound by instance and node (node = revision of the command), O(1) check instead of comparing checkpoint, command ledger and lineage. transaction_copy stays (MSH-227).
Stale-request refusals (token gives each case): (1) accepted command: the revision advances on every success (submit in all five games, Shogi record append, Xiangqi rejected-claim audit) -> earlier token differs -> StaleContext. (2) undo: advances the revision in all five games (never restored). (3) retirement: Tori/Chu/Tenjiku retire_context and select_prefix advance it; Shogi and Xiangqi have no retirement call. (4) another instance: every Game::from_root/new/standard takes a fresh instance, so tokens of two instances never match, even with equal history. (5) fork (Xiangqi/Tori/Chu/Tenjiku) and Shogi clone: fresh instance. (6) replayed copy: record to_game, committed_prefix, prefix and record_prefix build from from_root -> fresh instance. Xiangqi/Tori/Chu/Tenjiku Game has no Clone. Atomic admission: the token advances only after a command succeeds; every refusal or stop returns before that and the existing rollback restores state, so a refused or cancelled command changes nothing (tests unchanged and passing).
Change for Shogi only: before, Shogi accepted a request on any game with equal root and history (a clone, another instance, a replay, or after undo back to the same history) and an undo token on any game in the equal state. Now these are refused, as in the other four games. This is stricter, never looser. Tests that compared Shogi contexts across instances to show equal history now compare the games (Game equality).
Tests: assertions that used same_semantics now compare GameRecord or (root, events) with the same expected values; fork().unwrap() calls lost unwrap. No rules or perft expectation changed.
After measurement (commit 7cce8b2, same machine and command, us; pass 2 / three Xiangqi passes): shogi 318: admission 61.7 (was 91.8), undo 0.08 (44.3), replay 13988 = 44.0 per event (62.7); xiangqi 1000: admission 84-91 (91.8), undo 0.04 (2.2), replay 154.2-155.2 ms = 155 per event (160.6), transaction_copy still dominates (MSH-227); tori 1000: admission 33.3 (65.6), undo 171.8 (155.7, unchanged code: recompute_repeated), replay 24874 = 24.9 per event (42.0); chu 1000: admission 12.5 (106.1), replay 615265 = 615 per event (674; repetition scans dominate, MSH-251); tenjiku-adams 1000: admission 6.3 (81.9), replay 8256 = 8.3 per event (47.4); tenjiku-muller 1000: admission 8.6 (84.0), replay 9215 = 9.2 per event (48.8). Record bytes and event counts are equal before and after.
Production lines (crates/*/src and desktop/src-tauri/src, no test files): 72686 before, 72569 after (-117). game.rs: shogi 759->820 (manual Clone/PartialEq, checkpoint), xiangqi 993->940, tori 708->664, chu 1115->1065, tenjiku 837->763; new native/context.rs 46.
Checks at 7cce8b2: fmt pass; clippy workspace all targets pass; workspace tests 1155 passed, 0 failed, 10 ignored (fixed_search_matches_recorded_baseline passed); desktop-session --all-features 196 passed and clippy pass; core perft including deep (release) 6 passed; desktop/src-tauri Cargo.lock current (cargo metadata --locked); src-tauri does not use the changed APIs and was not built. No frontend or Python code changed. Not platform-sensitive: no CI run needed.
<!-- SECTION:NOTES:END -->
