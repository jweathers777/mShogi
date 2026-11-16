---
id: MSH-179
title: Clock and Session for All Games
status: Done
assignee:
  - '@codex'
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-16T19:48:00Z'
labels:
  - handle:all-games-time-state
milestone: m-5
dependencies: []
priority: medium
ordinal: 186000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Clock and Session for All Games.

**Problem:** Only Chess has a time budget per move (MSH-167 (Engine Time Management)) and engine state that lasts between the moves of a game (MSH-169 (Engine Session Persistence)). The other games search with fixed depth and node limits, the internal execution protocol rejects `time_ms` for them, and their searches start empty at each move. Tori already uses the shared core (MSH-165 (Tori Search Core Adoption)), and the other games will follow.

**Objective:** Each game on the shared core can search with a time budget and keep its transposition table and statistics between the moves of one game, through the same mechanism as Chess.

**Approach:** Generalize the Chess mechanisms instead of copying them for each game: time allocation in the core, the session of the engine process (`crates/mshogi-engine/src/server.rs`) and the `time_ms` admission in the protocol. Keep fixed depth and node limits reproducible, as Chess does. Do not change desktop defaults; MSH-140 (Player Thinking Time) owns player choices. A game that moves to the core later uses this mechanism through its adapter. This changes protocol and process behavior, so it is Level 2: select the platforms from the actual risk.

**Completion:** Tori, and each other game already on the core, plays under a time budget and keeps engine state between moves, with Chess behavior unchanged.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Every game on the shared core accepts a time budget through the internal protocol, and the engine never exceeds the hard limit in a recorded set of games under a clock
- [x] #2 Engine state lasts between the moves of one game for every game on the core, and a new, loaded or taken-back game starts empty, as for Chess
- [x] #3 Requests with fixed depth or node limits stay reproducible, and saved documents with earlier observations still open
- [x] #4 The mechanism is shared: no per-game copy of time allocation or session handling, and Chess behavior and tests are unchanged
- [x] #5 Level 2 checks on the selected platforms, and independent review, are recorded
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Baseline: main 36de154a; Chess alone retains timed search state. Inspect all adapters and protocol consumers. Use one shared state/timing mechanism, native root admission and history identity; fixed-limit calls remain fresh. Owner: Codex, primary checkout, codex/all-games-time-state.
2. Add shared timed/persistent search support for all six games and protocol admission. Preserve desktop defaults and earlier saved observations. Reset state on non-continuation histories and game/profile changes.
3. Verify fixed-depth nodes and best moves against the frozen baseline; run native/search and protocol/session regressions, bounded timed native games and reset tests. Initial timing experiment bound: 10 minutes actual local runtime; compare elapsed requests against budgets and correct overruns before expansion. No strength claim; identical fixed searches need no predecessor game test under the roadmap exception.
4. Level 2: protocol and process-session changes. Run focused native engine/session/console checks locally on macOS and select Linux/Windows process checks based on final diff. Full packaged GUI compatibility is deferred to the next integration/release checkpoint. Obtain independent review, resolve findings, integrate into main and push.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Early independent review found that explicit cancel after a completed request left the idle child alive. This could retain tables after loading an equal history. Added a separate retirement flag consumed by the supervisor before reuse; ordinary next-move submission does not set it. This makes process lifetime an actual cross-platform risk: required Level 2 scope is the targeted process suite on macOS, Linux and Windows, plus local search/session/persistence checks. Full packaged GUI remains deferred.

Initial local checks: all seven frozen standard-root depth-2 results match predecessor nodes, best moves and scores; generic timed state/reset tests pass. Existing Chess fixed-move timing deliberately counts private search time, not complete request wall time (its regression requires elapsed >= submitted time). Preserve that contract. Clock evidence must record the actual game-clock allowance, submitted search budget, explicit transport reserve and full request elapsed separately; no hard-real-time guarantee is claimed for atomic native replay/validation.

Checkpoint: seven frozen depth-2 standard-root node/best/score records are exactly unchanged. All 28 native clock-play turns passed with shared clock allocation and measured caller reserves (300 ms cold, 50 ms warm); initial fixed-search-time experiment is retained and shows Chess setup overhead, not a game-clock compliance result. Continue to final Level 2 checks and review; no predecessor strength match is required under the identical fixed-search exception.

Product source 88d052034f59d641413ac5a48f9585fd883b590a is committed and pushed on codex/all-games-time-state. Independent source/evidence review accepts the change; idle-cancel finding is resolved and actual subprocess regression passes. Evidence is in docs/m5/evidence/msh179/. Final local serial process regression and targeted process CI on macOS/Linux/Windows (run 43940509964) are running; these are not yet recorded as passed. Workspace clippy and formatting pass. No integration or Done claim until required results complete.

Final local source 88d05203: process suite 329 passed/4 ignored; desktop-session suite 321 passed/5 ignored; standalone Tauri host check passed (existing tao dependency warnings). Source diff under crates/desktop/tools remains empty since tested commit. Routine push CI 43940472152 passed format, clippy, search and frontend checks. Targeted Linux process job passed; hosted macOS and Windows remain pending. Acceptance 1–4 supported; criterion 5 and integration remain open.

All required checks passed on product source 88d05203. Targeted process run 43940509964 completed successfully on macos-15, ubuntu-24.04 and windows-2022. Final verification.json records job identities/times and local checks. Independent review accepts source and evidence; final result addendum records the completed gates. No production changes since tested source. Ready for normal integration into main.

Integrated reviewed commits88d05203 and3c86ec04 into main. No separate task worktree was created; the primary checkout was used. The completion record is documentation-only. Final push and cleanup follow.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
All six games and both Tenjiku profiles now accept time_ms and retain tables and move statistics through one shared session rule. Exact timed history continuations keep state. Different histories or profiles clear it. Fixed-limit requests remain fresh. Explicit cancellation also retires idle children, so an equal-history reload starts empty. Desktop defaults and native rules are unchanged.

Seven frozen depth-2 node, best-move and score comparisons are identical. All 28 native-admitted clock turns met their recorded game-clock allowances with explicit startup and delivery reserves. The initial failed Chess wall-time assumption is preserved; its fixed search-time semantics remain unchanged. The roadmap identical-search exception applies; no strength claim or predecessor match was made. Search: 166 passed, 4 ignored. Final local process: 329 passed, 4 ignored. Desktop session: 321 passed, 5 ignored. Formatting, workspace clippy, standalone host and routine native/frontend CI passed. Targeted process run 43940509964 passed on macOS, Linux and Windows. Independent review accepted the final source and evidence after the idle-cancel correction.

Integrated into main at 3c86ec04; tested product source 88d052034f59d641413ac5a48f9585fd883b590a is unchanged. See docs/m5/evidence/msh179/README.md and verification.json. Cooperative timing does not guarantee wall time under arbitrary scheduling or histories. Full packaged GUI verification remains deferred to the next integration or release checkpoint. MSH-140 (Player Thinking Time) owns player controls. Next ready task: MSH-184 (Native Report Replay Cost).
<!-- SECTION:FINAL_SUMMARY:END -->
