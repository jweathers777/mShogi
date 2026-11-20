---
id: MSH-204
title: Tori Strength Harness
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T19:43:00Z'
updated_date: '2026-11-20T16:29:00Z'
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
- [x] #1 At least 100 balanced openings with source and balance rule; a tactical suite of at least 200 positions with native-prover answers and a held-out part, with the baseline score.
- [x] #2 A game-test control with median late-game depth of at least 8; Texel export with a split; a time-odds measurement (2x time) recorded.
- [x] #3 Tori games use the existing clocked native experiment route (mshogi-match experiment-run with a Fischer clock, structured internal protocol, MSH-179); any gap for this harness (for example per-player clocks for time odds, or opening catalogs) is closed with tests; no text engine protocol is added, since no external Tori engines are used.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
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

Revised 2026-11-20 after MSH-208, before any game-test result: runner max_steps cap 512 -> 1024 per game (native event limit); Tori default max_steps 1000. Length-limit rule (tools/strength/native_match.py, fixed now): a Tori pair where a game reaches the step limit without a native result is excluded from SPRT and counted (length_limited_pairs); any other unscored stop still stops the campaign; stop when more than 10% of at least 20 pairs are excluded. Remaining: Tori mate prover + tactical suite (>= 200, held-out), control with median late-game depth >= 8, Texel export with split, 2x time-odds measurement (needs per-player clock in the native runner), report, review, merge.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-19 harness infrastructure (Level 1 local, macOS arm64): (1) Tori adopts the incremental session protocol (core GameDelta/IncrementalRecord, engine ToriAdapter; engine turns only). (2) Native Tori Game::fork (exact copy, fresh runtime identity) for search validation, ToriResult::validate and runner transaction copies. (3) Tori admission computes encoded bytes from cached per-event lengths (exact; differential test against full encoding after play/undo/redo/prefix/stop/resign). (4) Clocked runner gives the nominal (soft) allocation, not the 3x hard ceiling. (5) Experiment reports omit each step's native record (request and observation context); decode restores it from native replay without re-decoding; reports and byte charges are now linear in game length (Tori pair 4.6 MB -> 40 KB gz; replay of a 357+111-ply pair 75 s -> 1.75 s). Smoke: per-request overhead flat (median -13 to -5 ms over the game). Tests: core/engine/search/runner/console 943 passed, desktop-session 322 passed, Clippy clean. OPEN DECISION: 2 of 20 smoke games reached the recorded MSH-129 native Tori capacity (65,536 replay units, about 360 board plies) and ended unscored; asked John how to treat this.

2026-11-19 openings: tools/strength/data/tori-openings-v1.json, 218 admitted of 300 distinct candidates (82 unbalanced). Rule fixed before screening: 8 plies from the standard root, each ply chooses uniformly (seed 204) among actions within 50 of the best depth-6 score; screen at fixed depth 12 by the search at 6c7be47a, admit |score| <= 100 Swallow hundredths. Generator crates/mshogi-search/examples/tori_openings.rs; candidates and screen in docs/m5/evidence/msh204/openings/. 8 pairs at 5 s + 0.1 s through the clocked route from the catalog: 15 games concluded natively, 1 reached native capacity (ply 361 incl. opening). Capacity total so far: 3 of 36 games.

2026-11-20: Blocked on MSH-208 (Tori Repetition Rule). John decided the Tori repetition rule (Shogi-style position repetition, third occurrence forbidden, different move required) and the removal of the replay-work count that ended about 8% of self-play games unscored at about 360 plies. Game tests resume after MSH-208. MSH-209 (Native Consistency Review) covers the cross-game safeguard inconsistency.

2026-11-20 after MSH-208 (Tori Repetition Rule, commit 9d06871d): 8 pairs (16 games) at 5 s + 0.1 s from tori-openings-v1: 15 concluded natively, 0 reached a native limit (old 360-ply limit gone), median 205.5 plies, 1 game reached the runner step limit (max_steps 500; runner Config::validate caps max_steps at 512). Next in MSH-204: raise the runner step cap toward the native 1,024-event limit, and fix before results the rule for a game that reaches a limit (pair excluded from SPRT, counted, stop above a set share), as proposed to John on 2026-11-20.

2026-11-20 independent review of 6c7be47a and 6f035708 (fresh reviewer): B1 blocking — the clock-allocation change broke replay of committed clocked Shogi reports (MSH-186) and changed future Shogi runs silently. Resolved at c7e013a2: FischerClock.allocation (hard|nominal), absent = hard and not serialized, so configuration identities are unchanged; validate uses the recorded policy; config writer flag --clock-allocation; native_match.py passes nominal for Tori only. Both committed MSH-186 clocked reports (clock-attempt-01/02 pair-0001) replay again; a test shows a report under the other policy is refused. Notes resolved: explicit null record in an elided observation refused; GameDelta doc comment corrected; catalog writer tools/strength/make_tori_openings.py committed and reproduces tori-openings-v1.json byte for byte.

2026-11-20 results (report docs/m5/tori-strength-harness.md, evidence docs/m5/evidence/msh204): openings 218 (tori-openings-v1); tactical suite 502 positions (378 dev, 124 held-out; 150/152/200 of 3/5/7 plies) from 112 local 1 s + 0.02 s self-play pairs; answers = checking answers from the native prover, other played actions proved by the full-width prover at scoring time (proof store, tests against plain recursion). Baseline (cloud c3-standard-4): 20 ms 346/502, 200 ms 484/502, 1000 ms 501/502; no unknown proofs. Control 5 s + 0.1 s nominal: 56 pairs, 111 native conclusions, 1 length-limited pair (1.8%); real-game median depth from ply 100 = 10 (p10 8); fresh 100 ms search median 8 (p10 7). Time odds 2x: 96 pairs [1,0,23,0,72], +330 Elo (266-418), H1. Texel export/split: 256 games, 12,880 positions, train 9,995 / validation 383 / held-out 2,097. Cloud: build-38d880a0-1, msh204-smoke-1, msh204-campaign-1; about 3.5 VM-hours, US$0.70 upper bound; cleanup verified. Process note: two local prover runs (about 10 CPU-hours) were stopped as wasteful; John 2026-11-20: heavy work runs in the cloud, including compiles (tools/strength/cloud_build.py). Independent review requested.

2026-11-20 independent review complete (docs/m5/evidence/msh204/review.md): harness-completion review found no blocking defect; should-fix items resolved in 7ba3291a (per-game depth unchanged at median 10; collect checks; proof keys with diagram digest and prover version; five-ply and cross-depth prover tests; time factor needs a clock); re-review clean; notes applied. Verification Level 1 (native tooling, runner, Python tools; no product rules or desktop change in this branch since MSH-208): macOS arm64 focused checks — tools/strength tests 152 passed; tori_strength example tests passed; runner lib tests passed earlier at 9fdb6ed5; Clippy/fmt clean. Heavy measurement only in the cloud (c3-standard-4).
<!-- SECTION:NOTES:END -->
