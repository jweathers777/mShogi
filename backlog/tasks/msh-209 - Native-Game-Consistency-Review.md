---
id: MSH-209
title: Codebase Consistency and Duplication Review
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-20T07:34:00Z'
updated_date: '2026-11-25T12:11:00Z'
labels:
  - handle:native-consistency-review
  - architecture
  - rules
  - model:large
  - effort:xhigh
milestone: m-5
dependencies:
  - MSH-208
documentation:
  - docs/design.md
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Find overengineered, inconsistent and duplicated code across the whole codebase, and turn each finding into a bounded task. John (2026-11-20 and 2026-11-25): one consistent design style, shared code wherever it is efficient (AGENTS.md, Do not repeat yourself); really clean up before Tenjiku work.

Scope: mshogi-core native mechanisms (resource limits, repetition detection, record encoding, fork, undo and prefix, incremental sessions); mshogi-search positions and repetition scans; mshogi-engine adapters; mshogi-desktop-session; the desktop frontend and Tauri host; console, runner and match tools; and examples that are not strength tools. Out of scope: the strength tooling (tools/strength and the Rust strength examples in crates/mshogi-runner/examples and crates/mshogi-search/examples), which MSH-243 (Game-Independent Strength Tool) reviews and consolidates.

Start from the MSH-229 inventory and the current code. Known findings already have tasks: MSH-222 (Shared Engine Adapters), MSH-227 (Xiangqi Record Replay Speed), MSH-239 (Uniform Desktop Operations and Board Components). Do not duplicate them.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 An inventory in the task notes lists each duplicated, inconsistent or overengineered mechanism with its location, size in lines and the reason it differs (game rule, measurement or none).
- [ ] #2 Each finding has a proposed shared design, or a recorded rule or measurement reason to keep it separate.
- [ ] #3 Proposals that change a game rule or a recorded native contract (limits, adjudication) are approved by John before implementation.
- [ ] #4 Each approved change is a bounded Ready task with a priority and ordinal that place it before MSH-238; no task duplicates an existing one.
- [ ] #5 Each of John's three decisions of 2026-11-25 in the notes (one Chess judging mode, one clock policy, no test-only evaluators) has a bounded Ready task or is done.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-229 now records a codebase-wide audit/exactness inventory in its task notes. Concrete simplification work is split into MSH-230 (operational Stop history), MSH-231 (redundant record metadata), MSH-232 (native resource admission/copies), MSH-233 (execution identity/validation), with Xiangqi replay costs in existing MSH-227. Use that inventory as current input and avoid duplicate implementation tasks; no rules change was made by MSH-229.

Status check, 2026-11-25 (ticket review approved by John): MSH-229 to MSH-233 (all Done) removed the separate replay-work counts and whole-history copies in native admission (MSH-232), redundant record metadata (MSH-231) and repeated execution validation (MSH-233). Still to inventory and decide here: entry and byte limits per game; repetition detection against all earlier positions; Game::fork (now Xiangqi, Tori, Chu, Tenjiku; not Chess or Shogi); incremental engine sessions (now Shogi, Tori, Chu); repetition scans in search for drop games. Start from the current code, not from the 2026-11-20 list in the description.

Order set by John, 2026-11-25: MSH-237, MSH-234, MSH-209, MSH-222, MSH-239, MSH-227, then this review's follow-ups (medium priority, ordinals 6100 to 7900), then MSH-238, MSH-221, MSH-235; Tenjiku (MSH-214, MSH-202) last.

John's decisions, 2026-11-25 (from the items MSH-237 kept). Create tasks for them; do not decide them again.

1. Chess has one way of judging games and one record schema. There is no mode: remove AdjudicationMode and the name M2, and keep only the behavior that M2 has today (material-subset dead positions). Remove the Full mode, schema 2, the optional desktop Full analysis panel ("Advanced: Full mating reachability"), ExecutionPurpose::ChessFullReachability and the Full solver. Evidence: desktop record replay, engine server, console and experiment runner already reject Full records; docs/design.md says expensive exact proofs are not delivery gates; the Full solver has no practical runtime bound. Detecting only material cases is the usual practice. This is a rules change John approved: update docs/rules and tests, and get one fresh review.
2. One clock policy for all games: the normal share (today ClockAllocation::Nominal). Remove ClockAllocation, the Hard path and the --clock-allocation flag; update docs/m5/strength-roadmap.md. Earlier Hard results are not comparable.
3. Remove the test-only material evaluators in mshogi-search (chu, shogi, tenjiku, tori, xiangqi evaluate and Evaluation) and the tests that exist to check them. Some callers are also the search-position differential tests (position_tests.rs compare functions), which AGENTS.md requires: remove only the evaluator assertions there and keep the board, hand, hash, check and legal-move comparisons. Tori earlier_evaluation test code is built on the evaluator and goes with it.

Order update by John, 2026-11-25: MSH-240 runs before this review.

John, 2026-11-25: renaming unclear uses of "native" is MSH-241 (Replace Unclear Uses of Native in Names); do not add it as a review follow-up.

John, 2026-11-25: the strength tooling is out of scope; MSH-243 (Game-Independent Strength Tool) reviews and consolidates it. Do not create tasks for it. The single clock policy decision (item 2) stays here and includes its changes to the match tool and runner. Follow-up tasks keep ordinals 6100 to 7900, so they run before MSH-238, MSH-243 and MSH-241.

Checkpoint (claude, branch claude/msh-209-consistency-review): claimed 2026-11-25. Reading code; sweeps of console, runner/tools and desktop frontend/Tauri done; core, search, engine and desktop-session in progress. No inventory or follow-up tasks written yet.
<!-- SECTION:NOTES:END -->
