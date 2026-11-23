---
id: MSH-219
title: Shogi Match Test Timing Failure
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-21T04:35:00Z'
updated_date: '2026-11-23T19:36:00Z'
labels:
  - runner
  - test
dependencies: []
priority: medium
ordinal: 226000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The runner test a_full_mshogi_game_replays_and_tampering_is_rejected (crates/mshogi-runner/tests/shogi_match.rs) plays a timed Shogi game between two release mshogi USI engines and requires a native conclusion. It failed once in a release workspace run on John's Mac during MSH-205 (assertion at line 124, report.stop) and passed in 3 later runs; an unidentified search/runner failure in MSH-200 may be the same. Find the stop reason and make the test deterministic, or fix the defect it shows.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The cause of the failure is identified and recorded
- [x] #2 The test gives the same result in repeated runs (for example 20 runs), or the defect is fixed with a test
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Capture full reports from the existing 2 s + 100 ms actual-engine test and identify its stop path; inspect retained prior evidence. 2. Make replay/tampering coverage independent of timed search variation while retaining an actual engine/protocol check, or fix a demonstrated runner defect. 3. Repeat the resulting test at least 20 times, run required Rust and applicable platform checks, record evidence, merge and clean.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started from main 7d82b11. Investigate actual timed-game stop evidence and replace timing-dependent test behavior or fix a demonstrated defect. Branch codex/msh-219-shogi-match-timing.

Checkpoint: original release test configuration is running serially through mshogi-match, saving complete reports under runs/msh-219/original-*.json. First four runs concluded with 117-169 replies, demonstrating time-dependent search variation; no failure reproduced yet. No original local failure log was present in retained MSH-205/files. Candidate test-only fix prepared: always-run scripted 11-reply native game retains multi-turn ledger/replay/tampering checks; separate actual release-engine mate-in-one uses the existing independently established USI/search fixture. No product/rules/search changes. Next: finish original reproduction, test candidate repeatedly and run required checks. Branch codex/msh-219-shogi-match-timing.

Concrete test defect reproduced on original run 13 (zero-based): native Length draw after 318 replies, engine_score [1,1]. The test reverses that score as its alleged tampering; [1,1] reversed is unchanged, so replay correctly succeeds and the test would fail. Copied original-13.json to draw-score-swapped.json, reversed score, and mshogi-match shogi-match-replay accepted all 318 replies (exit 0). This proves a timing-dependent false failure of the existing test; the separately reported historical report.stop failure cannot be attributed from missing logs. Adjusted candidate to a fixed twelve-ply King-cycle Fourfold draw and guaranteed contradictory score [0,0], plus actual mate-in-one protocol check.

Original configuration campaign complete on Darwin arm64, one game at a time: 20/20 native conclusions, 79-318 accepted replies, one Length draw. That draw proves the old score-reversal assertion can fail without invalid runner behavior. Historical non-conclusion stop did not repeat and its original evidence is unavailable. Candidate Shogi match suite passed all five tests, including scripted Fourfold draw and actual release-engine G*5b mate. Now checking that reintroducing the old score mutation fails the new draw regression, then repeating the fixed suite 20 times.

Historical source resolves the task-description mismatch: at task creation commit 0371aef (and preceding 05841f2), shogi_match.rs line 124 is assert!(tampered.replay().is_err()) after reversing engine_score at line 123; report.stop is at lines 110-115. Thus the recorded failing line matches the reproduced draw-score defect, not a clock stop. Root cause: wall-clock self-play can draw; reversing [1,1] leaves a valid report unchanged, and the test incorrectly requires rejection. Fixed suite passed 20/20 repetitions, all 5 tests each (100 test executions), with the release engine present. Reintroducing the original score reversal made the new scripted draw test fail at the corresponding assertion, confirming the regression. Product commit 175b52d; main merge already current. Formatting passed; workspace Clippy/debug tests running; targeted process CI https://github.com/jweathers777/mShogi/actions/runs/45250398037 .
<!-- SECTION:NOTES:END -->
