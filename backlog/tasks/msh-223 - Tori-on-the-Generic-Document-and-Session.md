---
id: MSH-223
title: Tori on the Generic Document and Session
status: Done
assignee:
  - '@claude'
created_date: '2026-11-21T23:34:00Z'
updated_date: '2026-11-22T10:48:00Z'
labels:
  - desktop
  - refactor
milestone: m-6
dependencies: []
priority: low
ordinal: 230000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: serve Tori with the generic document and session of MSH-217 (Shared Game Session), so that its copied document, file worker and session code go away. Blocker (John): the Tori saved payload has no selected_event field (deny_unknown_fields), so the generic document changes the saved bytes of every new Tori file, and old app versions cannot read new files. Measured in MSH-217: the Tori document differs from the generic one in 1,608 of 1,168 normalized lines and the session in 639 of 1,139 lines; the Tori core also needs the stop-aware calls and replay work.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 John decides the saved-format question (add selected_event to Tori files, or keep the Tori format with a per-game switch)
- [x] #2 Tori uses ValidatedGameDocument and NativeSession; its old document, worker and session code are removed; line counts recorded
- [x] #3 Committed Tori documents load and save as decided; desktop-session tests pass; one fresh-agent review
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Tori core: stop-aware replay and record parse (like Chu/Tenjiku).
2. NativeGame impl for Tori; desktop work model (ROOT_WORK, PLAY_WORK) moves to native_game.rs.
3. tori_document.rs becomes aliases of ValidatedGameDocument; SessionGame impl for Tori with hands/drops hooks; one session_wiring! macro for all generic games; generic save and verification entries.
4. Adapt tests; measure the Tori document work limit; regenerate Tori fixtures; checks; review.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Decision (John, 2026-11-22, in conversation): saved-format compatibility does not matter before release (design A-09). Tori moves to the generic document and gets selected_event; old Tori files need not load.

Checkpoint 2026-11-22 (branch claude/msh-223-tori-generic): Tori runs on ValidatedGameDocument and NativeSession. Lines (tori_document, shared_tori, native_game, native_session, shared_archive): 5,637 -> 3,898. Desktop-session lib 320 pass; frontend vitest 609 pass with regenerated Tori fixtures (saved files gain selected_event; analysis fixture budget raised from 1,000 to 100,000 nodes because MSH-206 evaluation no longer completes depth 1 in 1,000). Behavior notes: Tori browse selection now saved and restored; the Tori-only check that a command actor is the local human is gone (Chu/Tenjiku never had it; their tests enter engine-side commands); forged over-limit Tori records now refuse after a bounded native parse (category Unknown), not by the old desktop pre-check. Tori document work limit 1,048,576 -> 8,388,608 modelled units (measured: 360-event document 208,232 units in 33 ms release; the 4,096 retained-event limit binds first). Core: Chu and Tori record parse now stop-aware with typed capacity, like Tenjiku. Next: workspace tests, review, merge.

Review (fresh subagent, 2026-11-22): no serious defect, no rules change. Fixed: (1) schema-1 Tori replay is now stop-aware too (core test added); (2) the work-limit comment used a short-game rate; measured the worst case (ignored test measure_full_length_records_against_the_work_limit, release, macOS arm64): one 1,022-event record 196,352 units in 158 ms; largest valid document (4,088 retained events, two retired branches, selection) 2,551,620 units in 1.2 s; a forged 16 MB file of full-length records took 6.8 s to refuse at 8,388,608, so the limit is now 4,194,304 (refusal after 3.7 s); (3) over-limit record tests now use well-formed events and assert a typed Tori resource refusal; (5) one generic open for Chu, Tenjiku and Tori (native_session::open). Not changed: per-game record_failure glue stays (different core error types). Visible text changes (intended, fixtures regenerated): Tori reopen diagnostic uses the generic text; Tori failed observations show the generic unscored failure labels.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Tori now uses the generic document (ValidatedGameDocument) and session (NativeSession) of MSH-217; its own document, file worker and session code are removed. Production lines in the touched modules (tori_document, shared_tori, native_game, native_session, shared_archive): 5,637 -> about 3,890. The generic session gained hands/drops hooks, one wiring macro for every generic game, and generic save, open and verification entries. Core: Tori and Chu record parsing and Tori replay (both schemas) are stop-aware with typed capacity, like Tenjiku; no rules change. As John decided (A-09), Tori files gain selected_event and reopen at the viewed history position; Tori fixtures regenerated (analysis fixture node budget 1,000 -> 100,000 because the MSH-206 evaluation no longer completes depth 1 in 1,000). Tori document work limit 4,194,304 modelled units, measured: largest valid document 2,551,620 units in 1.2 s; forged worst case refused in 3.7 s. Checks: fmt, clippy (workspace, all targets; desktop-session all features), all workspace tests (release), 609 frontend tests; one fresh-agent review, findings fixed. Follow-ups: MSH-224 (Shogi and Xiangqi), MSH-225 (Chess).
<!-- SECTION:FINAL_SUMMARY:END -->
