---
id: MSH-35
title: Save reopen and inspect desktop game history
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T16:48:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-33
  - MSH-34
documentation:
  - docs/desktop/m3-design.md
ordinal: 35000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Games must survive application restarts and permit read-only inspection without accidentally altering the live game.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Saved sessions preserve canonical game/match records, policy, commands/events, modes/limits and engine observations; reopening either restores supported semantics or clearly rejects unsupported restoration.
- [ ] #2 Historical viewing is visibly read-only and distinct from live play; returning live is explicit, and any offered branch/change invalidates obsolete work with new session identity.
- [ ] #3 Round-trip, corrupt/unsupported record, history/claim and stale-response tests plus actual desktop save/reopen/inspection evidence pass without silently downgrading modes.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Implement a strict versioned desktop archive with complete canonical MatchRecord/GameRecord, settings, command/control journal, exact-context observations and identity high-water marks. Add controlled core record accessors/replay cancellation without changing M2 rules. One bounded native worker performs atomic file save, strict/preflighted untimed-M2 open and exact historical event-prefix reconstruction off the interaction path; commit only current preconditions, leave failures intact and reopen paused with fresh runtime identities/timestamp floor. Add native file dialogs, unsaved-change replacement/quit guards and distinct historical UI/analysis provenance. Verify round-trips, failed/interrupted saves, corrupt/unsupported/resource-limited opens, claims/conclusions, stale work and real packaged save/close/reopen/history/return-live journeys; independently review, integrate/push and collect applicable CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner clarification 2026-10-25: implement only after MSH-34 and MSH-33 complete. Versioned desktop persistence must include canonical MatchRecord and GameRecord semantics, claims/conclusions, commands/events, policies, modes/limits, relevant settings and exact-context observations; not merely FEN/actions or GameRecord helper restoration. Observations reopen as historical, never pending work; runtime identities are fresh, snapshots remain monotonic, untimed engine work stays stopped until explicit Resume. Atomic save failure/interruption preserves prior valid file and dirty state. Corrupt/unsupported/resource-limited open leaves current session intact; validate/replay off interaction path with limits, commit candidate atomically after current-session precondition. Protect unsaved changes on replace/open/ordinary quit. Historical view visibly read-only with explicit Return to live; analysis uses exact history prefix without future repetition/events and rejects late results after selection changes. No branching/variation editing required. Actual save/close/reopen/history/return-live journeys required; separate instrumentation only. Existing AC preserved. MSH-36–39 remain open. See M3 design second-batch clarifications.

Owner @codex on codex/m3-desktop reusable checkout .claude/worktrees/next-task. MSH-33 and MSH-34 are Done with exact-source three-platform native/package evidence. Isolated native archive/history/worker builder and frontend builder plus host-only native-dialog/quit integration builder coordinate public DTOs under primary integration. Scope stays untimed M2 live restoration; advanced Full analysis contexts/limits are preserved as historical observations, unsupported live modes/timed sessions rejected before solver replay. No branching, timedplay, syntheticfixture or M3 closure in this task.

Implementation refinement: native Save/SaveAs/Open dialogs use pinned Rust-only tauri-plugin-dialog 2.8.3 off the UI thread. To fulfill ordinary macOS Dock Quit unsaved-change protection, the standalone desktop host vendors exact Tao 0.37.2 (119 upstream files including licenses), with one source-file delta adding a typed applicationShouldTerminate callback that queues existing preventable CloseRequested events and excludes native dialog windows. No added unsafe block or weakened host lint; root M2 dependency lock/rules remain unchanged. Provenance/control flow and separate actual GUI verification requirement: docs/desktop/macos-quit.md. NewGame also checks document revision as well as game version so settings changes cannot bypass an older unsaved-change confirmation. Canonical archive contract and explicit caps: docs/desktop/archive-v1.md.

Implementation is complete and under independent review. Primary local checks passed: full Rust workspace format, all-target Clippy with warnings denied, and workspace tests; 3 new actual-file/engine desktop persistence journeys; desktop host Rust 1.90 format/Clippy/tests plus isolated verification-feature check; 52 frontend tests across 9 files and strict TypeScript/Vite production build. Native builder reports 29 all-feature session tests and deterministic regeneration of 16 native fixtures. Independent host/vendor review is clean. Broader review identified selection watermark, bounded Full witness validation and two keyboard focus restoration gaps; fixes and regression tests are in the source and final review is pending. Normal local package build and UCI/engine-turn/move-analysis/Full smoke passed, but final-source package rebuild and actual save/close/reopen/history/Dock Quit GUI evidence remain required. macOS desktop automation currently reports the Mac locked; owner unlock requested. No GUI success is inferred from native/frontend tests. AC/Done remain unchecked pending runtime evidence, final independent assessment, integration and platform CI.

Final independent code review is clean after all findings were fixed. Reviewer independently passed 29 all-feature session tests, 3 real-file/engine tests, 52 frontend tests, strict TypeScript and 2 host tests, plus exact upstream Tao comparison. Confirmed complete canonical replay, bounded proof witness verification, unsupported witnessed-match preflight, watermark/timestamp preservation, historical observations, fresh paused restoration and focus recovery. The desktop is accessible again; actual normal packaged GUI verification is underway. Initial actual Cmd-Q/Cancel preserved e2e4/a7a5 history and restored keyboard focus to the SVG e4 square. Source will be pushed on codex/m3-desktop for platform CI while the GUI completion gate remains open; no integration/Done claim yet.
<!-- SECTION:NOTES:END -->
