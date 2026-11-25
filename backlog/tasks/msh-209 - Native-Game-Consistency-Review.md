---
id: MSH-209
title: Codebase Consistency and Duplication Review
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-20T07:34:00Z'
updated_date: '2026-11-25T12:25:00Z'
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

INVENTORY (claude, 2026-11-25, code at main 67d4b7b). Format: location, size; reason the copies differ (rule, measurement, none); proposal -> task, or KEEP with reason. Lines are production lines unless stated; "normalized" = non-blank lines after game names are replaced. Strength tooling is out of scope (MSH-243).

A. John's decisions (facts checked against code)
A1. Chess AdjudicationMode M2/Full, schema 2, Full solver, ChessFullReachability, desktop Full panel. core chess/completeness.rs 208 + parts of adjudication.rs, matchplay.rs, record.rs, native.rs; engine shared.rs, server.rs, adapters; desktop-session chess_observation.rs 269, shared_chess.rs, native_session.rs; ChessAnalysisContent.tsx. Reason: optional mode. Facts correct: desktop chess_record.rs:80, engine server.rs:313, console session.rs:260, runner experiments/native.rs:58/208/226 reject non-M2 records; docs/rules/chess-completeness.md states no practical runtime bound (only optional position/transition budgets). -> MSH-246.
A2. ClockAllocation Hard/Nominal: runner experiments/mod.rs 143-272 + tests; examples/support/native_match_config.rs (flag, default Hard); tools/strength/native_match.py; docs/m5. Reason: comparability with old Shogi runs. -> MSH-245. Fact to note: strength-roadmap.md still says "hard (default when absent)", but the field is required since MSH-237.
A3. Test-only material evaluators: search chu/shogi/tenjiku/tori/xiangqi.rs evaluate + Evaluation (~40 lines each); callers only tests; piece_value stays (used by positions). -> MSH-244.

B. mshogi-core
B1. Request context: every request carries GameContext (root + full history + token); game.context() copies it (~150 call sites outside tests); submit compares it; record replay is quadratic; Shogi submit clones Game 3x and GameUndo keeps before/after copies. Reason: none (the runtime token already changes on each change). -> MSH-250.
B2. Repetition lookup: Chess hash-map count; Tori digest index (measured, MSH-208); Shogi, Xiangqi, Chu scan all earlier keys per command; Tenjiku scans all keys per candidate action (legal_actions_with_stop, adjudicate). Reason: none except Tori measurement. -> MSH-251.
B3. History/authority shell repeated in tori/chu/tenjiku game.rs (~150-200 lines each: RuntimeToken, GameContext, GameRequest, Resource, CapacityUsage, GameUndo, fork, event/select/record prefix, retire, undo, rollback, u64 overflow checks); drift (Tenjiku retire reserve, Chu checkpoint in undo). Xiangqi has its own form; Shogi/Chess use Clone, hence fork only in 4 games. Reason: none. -> MSH-252.
B4. Redo at Game and LocalPosition level (Tori, Chu, Tenjiku; GameRedo, LocalRedo, transition::Redo, branch counters): only core tests call it. -> MSH-247.
B5. Record codecs tori/chu/tenjiku record.rs (474/469/317; strict() identical; replay, re-encode compare, suffix, Serialize; RecordError mapped 3x in native.rs). Reason: none for the codec; root data = rule. -> MSH-253.
B6. Record byte limits: Tori/Chu 512 KiB, Shogi/Xiangqi/Tenjiku 1 MiB, Chess none; shared envelope 1 MiB; Xiangqi per-command cap minus 4 KiB. Reason: none recorded. User-reachable limit -> MSH-268 (needs-john).
B7. Perft: five APIs (chess/shogi/xiangqi/tori/tenjiku perft.rs, 766 lines, different depth types, budgets, stop); Chu none (console and chu/conformance.rs copies). Reason: none. -> MSH-254.
B8. Incremental sessions: generic core Session<R>; engine uses it only for Shogi, Tori, Chu (added per game, MSH-189/204/213); Xiangqi, Tenjiku, Chess replay the full record per request. -> MSH-260 (after MSH-222, MSH-253).
B9. Xiangqi exact Article 3.2A search (exact.rs 381, exact_request.rs 160, tests 611 + desktop evidence/UI): only explicit desktop analysis; parallel of Chess Full. -> MSH-269 (needs-john).
B10. Chess core design (Game, AdjudicatedGame, ChessMatch with version admission, clocks, claims) differs from the other five. Reason: match rules (clocks, claims). KEEP; MSH-250 aligns only the request token.
B11. Conformance modules (tori/chu/tenjiku *conformance*.rs, ~2,800 lines, cfg(test)) and mshogi-conformance (dev-dependency). Rules tests. KEEP.
B12. Words fused with numbers (~190 places, ~50 files; e.g. chu/game.rs "belong78/43", messages "exceeds16MiB"). Reason: earlier bulk edit. -> MSH-249 (code); docs/ in MSH-235 (note added).
B13. Measurement examples (core record_replay, resource_admission, tenjiku_rules_work; engine execution_cost): no caller now. KEEP: planned use for measurements in MSH-250, MSH-251 (notes added) and MSH-202. xiangqi_exact_probe goes with MSH-269.

C. mshogi-search
C1. Root adapters chu/tenjiku/tori/shogi/xiangqi.rs (268-451, 1,669 total; Chu/Tenjiku differ in ~60 of 300 normalized). Reason: none. -> MSH-258.
C2. rule_score repetition scan: Shogi whole history, Tori to last capture/drop (recorded approximation), Xiangqi/Chu to last irreversible move, Tenjiku contains over all keys. Scoring = rule; bound = no measurement. -> MSH-259.
C3. Search positions: shogi vs tori position.rs (1,346/1,543; ~660 normalized shared lines in scattered blocks of at most 22 lines: SEE, null move, ordering); chu vs tenjiku ~270 of 1,100. Reason: hot path, different board representations. KEEP (AGENTS.md); revisit only with a measurement.

D. mshogi-engine
D1. Per-game adapters, limits constants, server and incremental per-game arms. -> MSH-222 (existing).
D2. Incremental sessions for three games only. -> MSH-260.

E. mshogi-desktop-session
E1. Per-game operation variants and wire enum. -> MSH-239 (existing).
E2. Presentation modules chu/tenjiku/tori (318/270/132): shared blocks at most 10 lines; content is game-specific. KEEP.
E3. Fixture generator examples (8 files + support, ~2,700 lines): scenarios differ (under 15% shared). KEEP; reproducibility is MSH-221.
E4. ResourceScope repeats GameKind plus Application (resource.rs 31-46). -> note in MSH-239.
E5. NativeGame Evidence type exists only for Xiangqi exact reports. -> MSH-269.

F. Desktop frontend, Tauri host, desktop scripts
F1. Piece components, Chu/Tenjiku boards, play command per game. -> MSH-239 (existing).
F2. Board controls repeated in Board.tsx, ShogiBoard.tsx, XiangqiBoard.tsx (~80 lines each, drifted); Chess on the old colour path; test-only branches. Falls under MSH-239 AC2 -> note added to MSH-239.
F3. Game-kind list typed 5x, names differ, status checks 4x (two analyzing definitions), ExecutionResult lacks tenjiku, duplicate types, labels rebuilt, copied helpers, test helpers. Reason: none. -> MSH-264.
F4. ToriBirdPiece.tsx + css (96, dormant), ~30 unused CSS classes, retired forbidden markers. -> MSH-248.
F5. Tauri main.rs: three poll loops, snapshot copy, five verification modes for three renderings, test copies. -> MSH-265.
F6. Font subset scripts 3x73 lines (constants only) + 2 older; gui-smoke.py per-game helpers; fixture list copy; driver event shapes; module loading 6x. -> MSH-266.
F7. verification/TenjikuLayout.tsx, TenjikuControls.tsx: used by verification/bootstrap.tsx. KEEP.

G. Console
G1. tori/chu/tenjiku_session.rs (410/450/441; 297-352 normalized lines shared); shogi/xiangqi drift (error style, draw names, export check); shared items in session.rs; main.rs 6 arms. Reason: rules only for notation, board, hands, draws, profile. -> MSH-255.
G2. usi.rs / xiangqi_uci.rs (~480 normalized lines shared), uci.rs line reader copy, byte check 3x, unreasoned drift. -> MSH-256.
G3. stop command (test-only), position record-file/json alias, unreachable line checks 6x, xiangqi_uci ended flag never true. -> MSH-248.
G4. Copied chess perft order_key, Chu perft recursion. -> MSH-254.
G5. Test helpers (run 6x, temp dir 6x, protocol harness 2x80, bridge 4x, desktop helpers, selfplay ~180 shared). -> MSH-257.

H. Runner (not strength examples)
H1. shogi.rs/xiangqi.rs (~370 shared), shogi_match/xiangqi_match (~200 shared), engine_for 3x, copied helpers, failure kinds 3x, fs::write overwrites evidence, double replay; test fixtures/helpers. -> MSH-261.
H2. Three transports (run.rs, protocol.rs, clocked.rs) with unreasoned drift; three clock ledgers. Chess clock/claims = rule. -> MSH-262.
H3. experiments/native.rs per-game arms, mod.rs tables, run/validate decision copy. -> MSH-263.
H4. Runner modes shogi-run, xiangqi-run, shogi-match, experiment-external have no tool caller now. KEEP: planned use for reference ladders and external judges of Chess, Shogi, Xiangqi (strength-roadmap.md, Measurement without a rating list); consolidated by MSH-261/262.

I. Python tools (not tools/strength)
I1. check_chess_*.py (7 scripts, 477 lines), differential_chess.py, tools/shogi, tools/xiangqi helpers; compare_perft.py undocumented; test_reference.py not in CI. -> MSH-267.
I2. tools/experiments helpers that overlap tools/strength (environment, file hash, write-new JSON; core.file_hash and GCSStore.delete/exists used only by tests). -> MSH-243 (listed in MSH-240 notes).

Existing tasks kept as they are: MSH-222, MSH-227 (note), MSH-235 (note), MSH-238, MSH-239 (note), MSH-241, MSH-243, MSH-221.
Not reviewed in depth: per-game legal-move generation in core and search, the alpha-beta core, desktop-session internals inside MSH-239 scope, docs/ content, CI workflow files (grep only), CSS beyond grep.
Estimated removal (code and tests) if all Ready follow-ups are done: about 9,000 lines, most from MSH-246, MSH-252/253, MSH-254-258, MSH-261/262, MSH-266; MSH-269, if approved, about 1,600 more.
<!-- SECTION:NOTES:END -->
