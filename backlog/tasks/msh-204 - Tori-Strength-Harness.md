---
id: MSH-204
title: Tori Strength Harness
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T19:43:00Z'
updated_date: '2026-11-20T08:48:00Z'
labels:
  - handle:tori-strength-harness
  - tori
  - strength
milestone: m-5
dependencies:
  - MSH-208
references:
  - docs/m5/strength-roadmap.md
ordinal: 211000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Strength Harness

**Problem.** Tori Shogi searches through the shared core (MSH-165) with a basic evaluation. It has no openings catalog, tactical suite, chosen game-test control or Texel export of the program standard. Tori has no USI-style text protocol, but it does not need one: the native experiment runner already plays clocked Tori games through the structured internal protocol (MSH-179), and no external Tori engines are used (John, 2026-11-19).

**Objective.** The harness step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Tori.

**Approach.** Reuse the clocked native experiment route; openings; a tactical suite from a native mate prover (Tori has drops); a game-test control; Texel export; a time-odds measurement of what speed is worth.

**Completion.** The criteria below are met with recorded evidence.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 At least 100 balanced openings with source and balance rule; a tactical suite of at least 200 positions with native-prover answers and a held-out part, with the baseline score.
- [ ] #2 A game-test control with median late-game depth of at least 8; Texel export with a split; a time-odds measurement (2x time) recorded.
- [ ] #3 Tori games use the existing clocked native experiment route (mshogi-match experiment-run with a Fischer clock, structured internal protocol, MSH-179); any gap for this harness (for example per-player clocks for time odds, or opening catalogs) is closed with tests; no text engine protocol is added, since no external Tori engines are used.
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
Findings 2026-11-19 (smoke games, 5 s + 0.1 s): both games ended unscored with clock_limit. (a) Per-request overhead grows with game length (about 70 ms at move 195): Tori requests carry and replay the full record; the shared incremental session of MSH-189 (Session<R>, SessionServer<A>) is wired only for Shogi. (b) The native runner passes the clock allocation's hard limit (3x nominal) as a fixed move time, so the engine spends the base time in about 20 moves.
Plan:
1. Tori adopts the incremental session: native tori GameRecord has_prefix/suffix and a strict GameDelta, IncrementalRecord impl (append with full native admission and effect checks); engine Delta::Tori, ToriAdapter, server dispatch, Client::supports Tori. Tests as for Shogi (tampered deltas, prefix mismatch, reset).
2. Runner clock allocation: time_ms = nominal allocation (soft), not the hard limit; record the semantic change (affects future clocked native-route runs, incl. Shogi native route).
3. tori_match_config on shared Args with catalogs; native_match.py Tori clocks/openings (done).
4. Openings: seeded plausible openings (shallow-search candidate moves within a margin of best), balance screen at fixed depth with a rule fixed before screening; at least 100.
5. Tactical suite: native Tori mate prover (checks-only attacker, drops), mined from self-play, complete answer sets as in MSH-194; held-out split.
6. Control with late-game depth >= 8; Texel export; time-odds test (2x time).
7. Review, report, integration.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-19 harness infrastructure (Level 1 local, macOS arm64): (1) Tori adopts the incremental session protocol (core GameDelta/IncrementalRecord, engine ToriAdapter; engine turns only). (2) Native Tori Game::fork (exact copy, fresh runtime identity) for search validation, ToriResult::validate and runner transaction copies. (3) Tori admission computes encoded bytes from cached per-event lengths (exact; differential test against full encoding after play/undo/redo/prefix/stop/resign). (4) Clocked runner gives the nominal (soft) allocation, not the 3x hard ceiling. (5) Experiment reports omit each step's native record (request and observation context); decode restores it from native replay without re-decoding; reports and byte charges are now linear in game length (Tori pair 4.6 MB -> 40 KB gz; replay of a 357+111-ply pair 75 s -> 1.75 s). Smoke: per-request overhead flat (median -13 to -5 ms over the game). Tests: core/engine/search/runner/console 943 passed, desktop-session 322 passed, Clippy clean. OPEN DECISION: 2 of 20 smoke games reached the recorded MSH-129 native Tori capacity (65,536 replay units, about 360 board plies) and ended unscored; asked John how to treat this.

2026-11-19 openings: tools/strength/data/tori-openings-v1.json, 218 admitted of 300 distinct candidates (82 unbalanced). Rule fixed before screening: 8 plies from the standard root, each ply chooses uniformly (seed 204) among actions within 50 of the best depth-6 score; screen at fixed depth 12 by the search at 6c7be47a, admit |score| <= 100 Swallow hundredths. Generator crates/mshogi-search/examples/tori_openings.rs; candidates and screen in docs/m5/evidence/msh204/openings/. 8 pairs at 5 s + 0.1 s through the clocked route from the catalog: 15 games concluded natively, 1 reached native capacity (ply 361 incl. opening). Capacity total so far: 3 of 36 games.

2026-11-20: Blocked on MSH-208 (Tori Repetition Rule). John decided the Tori repetition rule (Shogi-style position repetition, third occurrence forbidden, different move required) and the removal of the replay-work count that ended about 8% of self-play games unscored at about 360 plies. Game tests resume after MSH-208. MSH-209 (Native Consistency Review) covers the cross-game safeguard inconsistency.

2026-11-20 after MSH-208 (Tori Repetition Rule, commit 9d06871d): 8 pairs (16 games) at 5 s + 0.1 s from tori-openings-v1: 15 concluded natively, 0 reached a native limit (old 360-ply limit gone), median 205.5 plies, 1 game reached the runner step limit (max_steps 500; runner Config::validate caps max_steps at 512). Next in MSH-204: raise the runner step cap toward the native 1,024-event limit, and fix before results the rule for a game that reaches a limit (pair excluded from SPRT, counted, stop above a set share), as proposed to John on 2026-11-20.

2026-11-20 independent review of 6c7be47a and 6f035708 (fresh reviewer): B1 blocking — the clock-allocation change broke replay of committed clocked Shogi reports (MSH-186) and changed future Shogi runs silently. Resolved at c7e013a2: FischerClock.allocation (hard|nominal), absent = hard and not serialized, so configuration identities are unchanged; validate uses the recorded policy; config writer flag --clock-allocation; native_match.py passes nominal for Tori only. Both committed MSH-186 clocked reports (clock-attempt-01/02 pair-0001) replay again; a test shows a report under the other policy is refused. Notes resolved: explicit null record in an elided observation refused; GameDelta doc comment corrected; catalog writer tools/strength/make_tori_openings.py committed and reproduces tori-openings-v1.json byte for byte.
<!-- SECTION:NOTES:END -->
