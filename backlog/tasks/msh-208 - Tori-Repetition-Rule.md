---
id: MSH-208
title: Tori Repetition Rule
status: To Do
assignee:
  - '@claude'
created_date: '2026-11-20T07:34:00Z'
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
- [ ] #4 The separate Tori replay-work count is removed; entry limit 4,096 and record limit 2 MiB, with exact boundary tests; records that load before the change and contain no third-occurrence event still load; any record that no longer replays is identified and the reason recorded
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
