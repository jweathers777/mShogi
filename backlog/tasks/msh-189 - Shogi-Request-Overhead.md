---
id: MSH-189
title: Shogi Request Overhead
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-17T16:41:00Z'
updated_date: '2026-11-17T18:17:00Z'
labels:
  - handle:shogi-request-overhead
milestone: m-5
dependencies:
  - MSH-187
documentation:
  - docs/m5/evidence/msh187/prefix-diagnosis/README.md
  - docs/m5/shogi-strength-round.md
  - docs/m5/shared-search-core.md
priority: high
ordinal: 196000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Request Overhead

## Problem
Each timed Shogi search request still receives and validates the complete native game ledger and reconstructs the game (strict decode plus `to_game`). MSH-187 (Shogi Strength Round) measured about 25 + 25 ms for this at 311 history events on the M1 Pro, about 10 + 10 ms at 159 events (docs/m5/evidence/msh187/prefix-diagnosis/README.md). The cost grows with game length and is spent inside the move budget. At fast controls late-game budgets were 115-200 ms, so this overhead takes a large part of each move. It will be larger for long Shogi games and for Chu and Tenjiku. Also, a timed search can return no move: MSH-187's failed campaigns stopped on requests that completed no iteration ("incomplete:Stopped", depth 0). A conventional engine always finishes at least depth 1 (or has a legal move ready) before it obeys a time stop. Desktop play with low clock time has the same risk.

## Objective
Per-move request cost that does not grow with game length, and a timed search that always returns a legal move.

## Approach
- The engine session keeps the authoritative native game for an active timed session and receives only the new actions since its last request. It applies them through the native rules (each new action is still validated by the native authority). A request that does not extend the kept game (takeback, new game, imported root, mismatch) falls back to full validation and resets the session. Native rules stay the single authority for moves, histories, declarations and results.
- Timed search: complete depth 1 (or, at worst, choose a native legal move by the root ordering) before a time stop can end the search. Fixed-work requests keep their present semantics.
- Use one shared mechanism that other games can adopt. Shogi is the first consumer; do not build a Shogi-only path.
- Measure before and after: request overhead at about 50, 150 and 300 history events, and a timed request near the minimum budget.

## Completion
Verification Level 2 (process and session lifetime, protocol). Independent review. No evaluation or search-policy change in this task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Measured per-request overhead (excluding search) at about 300 history events is at most 10% of the overhead at task start, and grows at most slightly from 50 to 300 events; measurements and environment are recorded.
- [ ] #2 Every new action in a kept session is validated by the native rules; takeback, new game, imported root and mismatched history reset the session, with tests for each case.
- [ ] #3 A timed request always returns a native legal move, including at the minimum budget and in check; a test reproduces the MSH-187 depth-0 failure history and shows a returned move.
- [ ] #4 Fixed-work requests and saved-game formats are unchanged; affected desktop, USI and runner consumer tests pass.
- [ ] #5 The mechanism is shared and documented in docs/m5/shared-search-core.md for adoption by the other games; independent review accepts it.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze task-start main52e078e6 and measure full-request admission at about50/150/300 events plus minimum-time failure roots. Inspect native record, transport, response and retained-search paths. 2. Add one shared incremental native-session and transport mechanism: validated full root/reset, exact prior-session identity, native admission of new actions, atomic mismatch handling and cold reset; Shogi is first consumer. Remove redundant full-history work from the steady timed path, including response transport. 3. Add a shared timed legal-choice guarantee with honest observations before the first complete iteration; keep fixed-work and cancellation semantics, saved formats and native terminal results. 4. Verify reset/mismatch/illegal suffix/newgame/import/takeback and process restart, low-time/in-check/failure-history cases, native perft/differential/tactics and affected desktop/USI/runner. Level2: actual macOS and Linux process/protocol checks; select Windows targeted checks after source risk review. 5. Compare admission timing with baseline, require300-event cost<=10% and near-flat50-to300; independently review source/evidence, update shared-core documentation, integrate main and push.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Shared mechanism (John, 2026-11-17): MSH-187 connected only Shogi to the generic engine session slot with a Shogi-specific prefix check (crates/mshogi-engine/src/server.rs, shogi.rs: record.has_prefix). Build incremental session state at that shared session layer, game-independent, so Chess, Tori, Xiangqi, Chu and Tenjiku can use it without new code paths. Shogi is the first user.

Owner Codex chat01a1e634-7fc9-72c2-b4b4-07022a7df3ec, branchcodex/shogi-request-overhead in primary checkout. Clean main52e078e6, no active Backlog owner, app inventory only this chat active on repo; other worktrees untouched. Backlog1.55.3 matches baseline. Baseline predecessor52e078e6 (native equivalent to completed MSH-187 source). Intended benefit is>=90% less300-event admission overhead with near-constant per-new-action cost and a legal timed result at minimum budget. Before/after use identical native accepted histories and idle release builds on this Mac, focused local measurement <=10min per checkpoint; no cloud games or substantial spending launched by this plan. This task explicitly authorizes timed legal fallback where prior MSH-187 preserved no-fallback behavior; no evaluation/selectivity changes. A failed resource/latency objective requires direction review, not weaker criteria.

Implementation checkpoint: generic native Session<R> with Shogi strict event suffix and compact atomic rollback; generic transport SessionServer<A>, exact acknowledged endpoint, child-owned native game, and bridge compact response reattachment. First request stays ordinary protocol4; only exact advertised capability enables suffixes, so frozen older engines remain full-request peers. Full/current saved observations and fixed-work encodings remain unchanged. Shared timed fallback is unscored depth0 with native root-order move and real work/status; explicit cancellation suppresses it. Native concluded roots/declarations remain authoritative. Independent reviews found and resolved initial old-peer capability gap; initial external-tagged reset fix preserves strict canonical RawValue decoding. Core386tests pass/2ignored; preliminary affectedconsumer678pass/9ignored. After negotiation change, all engine94pass/2ignored and affected actual console22pass; strict6-packageClippy and standaloneTauri check pass. Search suite, idle before/after timings, actuallegacy peer probe and platform evidence remain pending. Preliminary baseline complete non-search cost50/150/300events5.819/28.331/75.719ms; strengthened exact-success marker will rerun baseline before acceptance. No strength/rating claim or cloudgame campaign.

Verification selection after source review: Level2 protocol/session lifecycle requires targeted process suites on hosted ubuntu-24.04 and windows-2022, plus actual local macOS checks. Existing .github/workflows/rust.yml process suite is sufficient; no full packaging or GUI campaign is needed. Final local search171pass5ignored; all final engine94pass2ignored; affected console22pass. Hosted runs will use frozen feature source and one completion monitor. Source is stable for idle corrected baseline/candidate timing; no more local CPU work until profiling finishes.
<!-- SECTION:NOTES:END -->
