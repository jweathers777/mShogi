---
id: MSH-208
title: Tori Repetition Rule
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-20T07:34:00Z'
updated_date: '2026-11-20T08:21:00Z'
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
- [ ] #1 T06 states John's rule and its sources: the same position is board, both hands and side to move, anywhere in the known history; a move that would create the third occurrence is refused without changing state; if every otherwise legal action would create a third occurrence, the side to move loses
- [ ] #2 Native tests with independently established expectations cover: refusal of the third-occurrence move, occurrences separated by unrelated play, cycles through capture and drop, check and non-check cycles, the forced loss, imported roots (no inferred earlier history), undo/redo and exact prefixes
- [ ] #3 The native check uses a per-position 64-bit hash with full-key confirmation on a match; a differential test against the full-key scan agrees on random and constructed games
- [ ] #4 The separate Tori replay-work count is removed; entry limit 1,024 with the 512 KiB record limit kept (the shared 1 MiB native envelope and engine request bound all games; raising them belongs to MSH-209), with exact boundary tests; records that load before the change and contain no third-occurrence event still load; any record that no longer replays is identified and the reason recorded
- [ ] #5 Conformance fixtures, desktop document/runtime evidence and engine validation are updated to the new rule; affected desktop checks pass at the level the change requires
- [ ] #6 The engine search follows the rule; any search-scan change is accepted on the program's checks (correctness, game test against the predecessor)
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
<!-- SECTION:NOTES:END -->
