---
id: MSH-194
title: Xiangqi Strength Harness
status: Done
assignee:
  - '@claude'
created_date: '2026-11-18T19:25:00Z'
updated_date: '2026-11-18T21:15:00Z'
labels:
  - handle:xiangqi-strength-harness
  - xiangqi
  - strength
milestone: m-5
dependencies: []
references:
  - docs/m5/strength-roadmap.md
ordinal: 201000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Strength Harness

**Problem.** Xiangqi runs on the shared search core (MSH-181) with a basic evaluation (material, placement, tempo). It has no measurement harness of the Shogi standard: the native Xiangqi runner is untimed, starts a fresh engine process per turn and accepts at most four openings. Without a harness, later rounds cannot be measured.

**Objective.** Give Xiangqi the harness step of the per-game classical strength program (docs/m5/strength-roadmap.md).

**Approach.** Reuse the Shogi harness pattern (MSH-186, MSH-190, the clocked shogi-match route of MSH-191) and the shared tools. Adjust for Xiangqi rules (WXF repetition, chase and count claims stay native).

**Completion.** The criteria below are met with recorded evidence; the opponent and judge proposal is ready for John.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 At least 100 balanced Xiangqi openings, with their source and a balance check, are in tools/strength/data.
- [x] #2 A tactical suite of at least 200 positions with independently established answers (for example native-verified forced mates) and a held-out part exists, with the baseline score recorded.
- [x] #3 A clocked, persistent-process Xiangqi match route plays mShogi against its frozen predecessor and an external UCI engine, admits every reply natively, and replays recorded games natively; tests cover it.
- [x] #4 A predecessor game-test control is chosen at which the median late-game depth is at least 8, with the measurement recorded.
- [x] #5 Self-play export of quiet positions with game results for Texel tuning works, with a split by game or opening.
- [x] #6 A proposed external opponent pool and a stronger judge engine (builds, licences, limits, any rating reference) are recorded for John's approval; no external study runs in this task.
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
1. Clocked persistent Xiangqi match route in mshogi-runner (xiangqi-match / xiangqi-match-replay), modelled on shogi_match.rs, UCI Pikafish 0.9 dialect, controller-owned Fischer clocks, native admission and replay; tests.
2. Opening catalog: original transcriptions of conventional Xiangqi opening trunks with development extensions, native validation and a fixed engine balance screen; at least 100.
3. Tactical suite: forced mates mined from self-play reports and proved by a native reference prover (checking-move AND/OR search), answer set = all shortest-mate first moves, split by source game; baseline score.
4. Self-play/export example for depth measurement and Texel rows; choose the predecessor control with median late-game depth >= 8.
5. Opponent and judge proposal (Pikafish, Fairy-Stockfish classical, ElephantEye) with builds, licences, limits and rating-reference limits for John.
6. Docs, evidence, review, integration.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Progress 2026-11-18 (branch codex/msh-194-xiangqi-harness, commits a9bcb915, c97fde2b, b3a0442b). Verification level 1 (local macOS arm64) plus protocol/process change in runner and console (targeted). Report: docs/m5/xiangqi-strength-harness.md; evidence docs/m5/evidence/msh194.
- Found and fixed: Xiangqi UCI had no clocks; per-request overhead grew ~2.5 ms/ply (native submit copies history, 3-4 replays per request). Added native Game::fork, session event-prefix reuse, incremental position, timed fallback move. Overhead now flat ~3 ms.
- Clocked route xiangqi-match/xiangqi-match-replay with shared runner::clocked host (Shogi route refactored onto it; its tests pass).
- Openings: 112 admitted (depth 11, +-150), 12 families.
- Tactical suite: 373 positions (91 held-out), complete native answer sets; baseline 373/373 at 200 ms: saturated, regression guard only.
- Control 5 s + 0.1 s: late-game fresh depth median 10 at 100 ms; 1,120 games all native conclusions.
- Export + texel split for xiangqi verified.
- Finding: 47/1,120 games lost by perpetual chase (search scores chase cycles as draw).
- Opponent/judge proposal in report; no external games run.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Xiangqi now has the harness step of the per-game classical strength program. Report: docs/m5/xiangqi-strength-harness.md; evidence docs/m5/evidence/msh194 (verification.json, review.md).

What changed and why:
- Clocked play: Xiangqi UCI accepts movetime and Fischer clocks (shared clock policy), keeps its table across a game (XiangqiSearchSession), and returns a native-ordered fallback move when a timed search ends before depth 1.
- Overhead fault found and fixed: request cost grew ~2.5 ms per ply (native submit copies history; 3-4 replays per request), so 5 s games lost on time near ply 60. Native Game::fork (copy with fresh runtime identity, no replay), event-prefix session reuse and incremental position parsing make it flat at ~3 ms.
- Clocked match route xiangqi-match / xiangqi-match-replay on a shared runner::clocked host (Shogi route refactored onto it); tools/strength/xiangqi_games.py.
- 112 openings (depth 11, +-150, 12 families); 373-position tactical suite (91 held-out) with complete native answer sets; late-game control 5 s + 0.1 s (fresh median depth 10 at 100 ms); Texel export and xiangqi split; opponent/judge proposal (Pikafish judge; classical Fairy-Stockfish and ElephantEye; relative ladder only, no public rating list) with proposed MSH-197 caps for John.

Verification (Level 1 + targeted runner/console): release suites 1,246 passed, 16 ignored, 0 failed; clippy/fmt clean; Python 139 passed. Debug workspace run stopped on slow Chu real-engine tests (unchanged code; pass in release). Linux/Windows CI and packaged GUI deferred to the next integration checkpoint. Independent review: no blocking findings; fixes applied.

Limits and follow-ups: the tactical suite is saturated (regression guard only); 47 of 1,120 self-play games were lost by the perpetual-chase rule (search scores chase cycles as draws) - candidate for MSH-195 (Xiangqi Selectivity Round). Integrated into main by merge eb9d0de3.
<!-- SECTION:FINAL_SUMMARY:END -->
