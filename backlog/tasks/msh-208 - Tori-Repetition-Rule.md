---
id: MSH-208
title: Tori Repetition Rule
status: Done
assignee:
  - '@claude'
created_date: '2026-11-20T07:34:00Z'
updated_date: '2026-11-20T08:59:00Z'
labels:
  - handle:tori-repetition-rule
  - tori
  - rules
milestone: m-5
dependencies: []
documentation:
  - docs/m4/tori/profile.md
ordinal: 215000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Repetition Rule

**Problem.** The recorded Tori repetition rule (T06 in docs/m4/tori/profile.md) was derived by an earlier agent session, not decided by John. It lets the move that creates a third occurrence stand and obliges the *other* player to depart differently. John's decision (2026-11-20), from a 2022 Japanese rule book, PyChess and Okano 禽将棋 (2008): the rule works like the Shogi repetition rule (same board, both hands and side to move, anywhere in the game), with two differences: three occurrences instead of four, and the move that would create the third occurrence is not allowed; a different move must be chosen (no draw). Separately, the native check scans full keys against all earlier positions and charges that as replay work (MAX_REPLAY_WORK 65,536, MSH-129). That count, not any rule, ends games at about 360 plies, which leaves about 8% of MSH-204 self-play games unscored.

**Objective.** The native Tori rule, records, desktop and engine follow John's rule, and game length is bounded only by the entry and byte limits.

**Approach.** Native check by a 64-bit hash per position, scanned two plies at a time, with a full-key comparison on each hash match (exact, no collision can decide a game). Remove the separate replay-work count; raise the entry limit to 4,096 and the record limit to 2 MiB. The engine search applies the same rule; a full-history search scan is kept or rejected by the game test against the predecessor.

**Completion.** The criteria below are met with recorded evidence; MSH-204 (Tori Strength Harness) can resume.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 T06 states John's rule and its sources: the same position is board, both hands and side to move, anywhere in the known history; a move that would create the third occurrence is refused without changing state; if every otherwise legal action would create a third occurrence, the side to move loses
- [x] #2 Native tests with independently established expectations cover: refusal of the third-occurrence move, occurrences separated by unrelated play, cycles through capture and drop, check and non-check cycles, the forced loss, imported roots (no inferred earlier history), undo/redo and exact prefixes
- [x] #3 The native check uses a per-position 64-bit hash with full-key confirmation on a match; a differential test against the full-key scan agrees on random and constructed games
- [x] #4 The separate Tori replay-work count is removed; entry limit 1,024 with the 512 KiB record limit kept (the shared 1 MiB native envelope and engine request bound all games; raising them belongs to MSH-209), with exact boundary tests; records that load before the change and contain no third-occurrence event still load; any record that no longer replays is identified and the reason recorded
- [x] #5 Conformance fixtures, desktop document/runtime evidence and engine validation are updated to the new rule; affected desktop checks pass at the level the change requires
- [x] #6 The engine search follows the rule; any search-scan change is accepted on the program's checks (correctness, game test against the predecessor)
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
Branch codex/msh-208-tori-repetition, stacked on codex/msh-204-tori-harness (shares native Tori fork/encoded-length changes). Verification Level 2 (native rule, saved-record format, desktop session and frontend consumers); no packaged GUI campaign unless a GUI-visible contract beyond the notice text changes.
1. Profile: rewrite T06 (docs/m4/tori/profile.md) to John's rule with sources (2022 Japanese rule book, PyChess, Okano 2008 via the Japanese Abstract Games Museum comparison; John's decision 2026-11-20). New profile edition string; schema_version 2.
2. Native core (crates/mshogi-core/src/tori): per-position 64-bit hash beside the full RepetitionKey; occurrences by scanning back two plies comparing hashes, confirming full keys. Replace Obligation (departure obligation) by the set of local actions that would create a third occurrence for the side to move (derived after each commit, stored in events/snapshots as a replay-checked derived fact); submit refuses such an action atomically; if every local action is in the set, RepetitionForfeit for the side to move; T05 mate/stalemate precedence when no local action. Remove MAX_REPLAY_WORK and replay_work; MAX_GAME_EVENTS/COMMANDS 4,096; MAX_GAME_RECORD_BYTES 2 MiB.
3. Records: schema 2 written; schema 1 read by replaying its commands under the new rule and comparing rule-independent derived facts (effects, occurrences, conclusion, positions), ignoring v1 obligation/replay_work; refused with a reason when a v1 command is now illegal or a conclusion differs.
4. Fixtures: re-derive TORI-H-* repetition expectations by hand for the new rule (count, initiator/creator, check cycle, forced quiet loss, overlap, replay/undo/prefix, forced full, unequal cycles); update native-fixture-ownership and verify-modern.py where they encode T06.
5. Consumers: desktop-session presentation and documents, frontend type/notice and generated TS fixtures, console tests, runner experiment tests, native envelope tests.
6. Engine search: root forbidden set from the native set; in search a move reaching a key with two earlier occurrences is illegal (scored as a loss for the mover); repeated (second) occurrence scored level as now. Separate experiment: full-history scan instead of the last capture/drop cutoff, kept only on the game test against the predecessor.
7. Tests: hash-scan vs full-key differential, boundary tests at 4,096 events/2 MiB, v1 compatibility tests; core/engine/search/runner/console/desktop-session suites, frontend vitest, Clippy; independent review.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-20: limits changed from the proposed 4,096 entries / 2 MiB to 1,024 entries / 512 KiB: native MAX_RECORD_ENVELOPE_BYTES and engine MAX_REQUEST_BYTES are 1 MiB for all games, so a 2 MiB Tori record could not cross the engine protocol or desktop documents. 512 KiB holds about 1,000 plies at about 470 bytes per event. Cross-game limits go to MSH-209.

2026-11-20 implementation at 9d06871d (Level 2: native rule, record schema, desktop document/presentation, frontend). Checks (macOS arm64, release): cargo test --workspace --no-fail-fast 1,303 passed, 0 Tori failures; one Chess process test (mshogi-console desktop_play actual_missing_response_and_failed_process_pause_without_a_chess_score) failed once under full parallel load and passed 4 of 4 reruns alone, unrelated files; Clippy clean; fmt clean; desktop vitest 609 passed; tsc clean; desktop/scripts test_gui_smoke.py 133 passed; tools/strength tests 146 passed. New evidence: hand-derived fixtures TORI-H-COUNT/INITIATOR/CHECK/FORCED-QUIET/SEPARATED/REPLAY-UNDO/FORCED-FULL/CAPTURE-DROP/C-HISTORY (713 executed, 713 passed, 14 pending, 0 failed); digest-vs-full-key differential (64 seeded games, 1,310 restricted positions, 1 forced loss, all equal); schema-1 records from the earlier desktop QA documents (4 load, tori-obligation8 refused at event 7); exact 1,024-event boundary with Stop/Resign reservations. Desktop documents of the earlier edition are refused (identity mismatch); QA documents regenerated (forced-prefix6, repetition7). Packaged GUI journey (gui-smoke.py Tori repetition section) updated but NOT run; deferred to the next packaged GUI checkpoint. Search: root uses the native restriction; in-search third occurrence is a loss for the mover; the full-history scan experiment is deferred to MSH-205 (Tori Selectivity Round), whose predecessor is this engine, because a game test across a rule change has no valid predecessor.

2026-11-20 independent review of 9d06871d (fresh reviewer): nothing blocking; fixture expectations re-derived independently and confirmed. Should-fix resolved at c7e013a2: (1) envelope accepted mismatched Tori schema/edition labels -> payload labels must equal envelope labels, tests for both directions; (2) per-action history scan made replay superlinear and the desktop model understated it -> native position index (digest -> indices), desktop PLAY_WORK=128 calibrated (old: 65,341 units for a 360-ply record replaying in about 65 ms release; new: 1,022-ply record about 94 ms), MAX_TORI_DOCUMENT_WORK 262,144 -> 1,048,576 so full-length records fit. Note for John: TORI-H-INITIATOR restricts the player whose move would create the third occurrence (Gote in the 4-ply cycle Sente starts), per John's wording; the 2022 rule book and PyChess say the player who started the sequence must vary. Checks at c7e013a2: cargo test --workspace --release 1,305 passed 0 failed; Clippy clean; vitest 609; tsc clean; gui-smoke tests 133; tools/strength tests 146.

2026-11-20 second review round (both fresh reviewers re-reviewed c7e013a2): nothing blocking. Index equals full-key scan (reviewer's own 17,626 checks incl. undo/redo/record_prefix). Calibration corrected: long-test figure is two replays (about 140 ms per 1,022-ply replay, 137 us/ply); PLAY_WORK set to 192 (about 1.4x measured at full length; 256 made the work budget bind before the 4,096 aggregate event limit). Envelope test now matches EnvelopeError::Identity. All 10 committed clocked MSH-186 reports replay. Integrated: merge b40422e2 into main; post-merge core/runner/engine 588 passed 0 failed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Tori repetition now follows John's decision (2026-11-20): the same position (board, both hands, side to move) anywhere in the known history may not occur a third time; the move that would create it is refused, and a side whose every board-legal action would create it loses (T05 mate/stalemate precedence). The native check uses a 64-bit digest with full-key confirmation and a position index; the separate replay-work count (65,536, which ended games at about 360 plies) is removed; limits are 1,024 events and the existing 512 KiB record (the shared 1 MiB envelope binds all games; MSH-209). Records are schema 2 under edition MSH208-MODERN-2026-11-20; schema-1 records load by replay unless they contain a now-refused third occurrence; earlier-edition desktop documents are refused and the QA documents were regenerated. Repetition fixtures were re-derived by hand and independently re-checked; search, desktop presentation/document cost model (calibrated against measured release replay), frontend notice and GUI script follow the rule. Verification Level 2, macOS arm64: workspace 1,305 passed, vitest 609, gui-smoke tests 133, tools 146, Clippy clean; two independent review rounds, nothing blocking. Not run: packaged GUI journey (deferred to the next packaged GUI checkpoint). Deferred: full-history search scan experiment to MSH-205. Open for John: the restricted side is the one whose move would make the third occurrence (TORI-H-INITIATOR), while the 2022 rule book and PyChess word it as the player who started the sequence.
<!-- SECTION:FINAL_SUMMARY:END -->
