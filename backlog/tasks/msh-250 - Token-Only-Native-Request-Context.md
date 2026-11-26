---
id: MSH-250
title: Token-Only Native Request Context
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T08:30:00Z'
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
<!-- SECTION:NOTES:END -->
