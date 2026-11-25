---
id: MSH-209
title: Codebase Consistency and Duplication Review
status: Ready
assignee: []
created_date: '2026-11-20T07:34:00Z'
updated_date: '2026-11-25T08:51:00Z'
labels:
  - handle:native-consistency-review
  - architecture
  - rules
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

Scope: mshogi-core native mechanisms (resource limits, repetition detection, record encoding, fork, undo and prefix, incremental sessions); mshogi-search positions and repetition scans; mshogi-engine adapters; mshogi-desktop-session; the desktop frontend and Tauri host; console, runner and match tools; examples; and tools/strength Python (per-game cloud, judge, self-play, openings and assessment scripts, about 7,500 lines).

Start from the MSH-229 inventory and the current code. Known findings already have tasks: MSH-222 (Shared Engine Adapters), MSH-227 (Xiangqi Record Replay Speed), MSH-239 (Uniform Desktop Operations and Board Components). Do not duplicate them.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 An inventory in the task notes lists each duplicated, inconsistent or overengineered mechanism with its location, size in lines and the reason it differs (game rule, measurement or none).
- [ ] #2 Each finding has a proposed shared design, or a recorded rule or measurement reason to keep it separate.
- [ ] #3 Proposals that change a game rule or a recorded native contract (limits, adjudication) are approved by John before implementation.
- [ ] #4 Each approved change is a bounded Ready task with a priority and ordinal that place it before MSH-238; no task duplicates an existing one.
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
<!-- SECTION:NOTES:END -->
