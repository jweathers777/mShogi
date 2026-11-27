---
id: MSH-264
title: Frontend Consistency and Desktop Cleanup
status: Done
assignee:
  - '@codex'
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-27T00:37:00Z'
labels:
  - desktop
  - frontend
  - refactor
  - cleanup
  - model:medium
  - effort:high
milestone: m-6
dependencies:
  - MSH-239
priority: medium
ordinal: 7700
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
One Frontend Game Table and Wire Types:
In desktop/src, the list of game kinds is typed again in product/native.ts, NativeBoard.tsx, ApplicationShell.tsx, NativeSessionPanels.tsx and resources.ts; game names differ (ProductApp 'Chu Shogi', ApplicationShell 'CHU', SupportDialog has its own list); the thinking and analyzing checks are written four times, with two definitions of analyzing (ApplicationView.tsx, NativeSessionPanels.tsx, GameFrame.tsx); ExecutionResult in src/native.ts has no tenjiku case although the Rust type has one; TenjikuEffectPiece (boardPresentation.ts) repeats TenjikuPieceIdentity (product/native.ts); the snapshot wire type is in NativeBoard.tsx; side and piece labels are rebuilt in several components (colorName turns any non-white side into Black). Small helpers are copied (Tab focus loop three times, scroll-into-view twice). Frontend tests: eleven files build their own fake session, six define deferred(), about ten define board() or square(). MSH-239 (Uniform Desktop Operations and Board Components) covers the operation shape and the piece and board components and runs first; do not repeat it.

Remove Remaining Retired Desktop References:
MSH-248 (Remove Unused Console and Frontend Code) found leftovers outside its list (checked 2026-11-25; check them again before you change them). The verification surfaces that showed a 'DEVELOPMENT CONTRACT' label and a shared-contract-application class are gone from the source, but desktop/scripts/gui-smoke.py and desktop/src/product/ChuContract.test.tsx still assert that they are absent, and docs/m4/desktop-session.md and docs/m4/desktop-archive.md still describe the label. Two rules use custom properties that no style sheet defines: .document-dirty in desktop/src/style.css (var(--muted)) and .moves-events in desktop/src/product/shell.css (var(--shell-line)).

One Kept Result Count:
The desktop session keeps the newest 256 engine results with KEPT_RESULTS (native_session.rs). A second constant, MAX_ARCHIVE_OBSERVATIONS (persistence.rs), has the same value but no production use; only the verification capacity fixtures use it. Use one constant for both, and remove the unused one. Found during MSH-268 (Remove Game Size and Engine Request Limits).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 One game table (kind, name, sides, labels) and one session-status helper serve the frontend; wire types are in one module and match the Rust types, including the Tenjiku result.
- [x] #2 Copied helpers and test helpers each live in one shared module; frontend tests and the TypeScript check pass; line counts before and after are recorded.
- [x] #3 No test or script asserts only that the retired contract class or label is absent; checks of kept behavior stay.
- [x] #4 The desktop session and archive documents do not say that a current surface shows a 'DEVELOPMENT CONTRACT' label.
- [x] #5 No style sheet uses a custom property that is not defined; the document state label stays readable.
- [x] #6 One constant defines the number of kept engine results; MAX_ARCHIVE_OBSERVATIONS is removed and the verification fixtures use the remaining constant.
- [x] #7 Workspace, desktop-session all-features and GUI script tests pass.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Put game metadata, native wire types, and session status in shared modules. Verify the wire shape against Rust and existing snapshots.
2. Consolidate focus, scrolling, and test setup. Remove retired assertions and labels, fix undefined CSS variables, and share the Rust result cap.
3. Record TypeScript line counts, run the required suites, get fresh review, and integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John approved consolidation on 2026-11-26: this task retains the full scope of MSH-264 (One Frontend Game Table and Wire Types), MSH-271 (Remove Remaining Retired Desktop References), MSH-272 (One Kept Result Count). The small retired-reference and result-count corrections share desktop verification with the frontend consolidation. Agent labels are model:medium, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on branch codex/msh-264-frontend-consistency. Next: inspect the desktop frontend and Rust wire types, then implement the approved consolidation.

Baseline: 6,762 lines across tracked TypeScript and TSX files under desktop/src (includes tests and excludes fixtures). The claim is pushed. Next: implement the recorded plan.

Checkpoint: shared game metadata, wire types, request status, focus and test helpers are in place. The retained result limit has one definition. Desktop-session all-features passed 196 tests. Fresh review caught short live headers required by the GUI journey; the game table now preserves CHU and TENJIKU, with coverage for all six headers. Next: resolve any remaining review findings, complete the workspace/frontend/script gate, and integrate.

Checkpoint: fresh review is complete. Preserved approved header labels, clarified in-memory retention, and made the Xiangqi witness type match Rust. Final strict TypeScript and the production frontend build passed. The full shared check gate is running. Next: confirm workspace, frontend and GUI-script results and integrate.

Workspace tests and Rust lint passed. The frontend suite found Chess analysis reports work through its dedicated status and dialogs need document-order sorting when a disclosure opens. Restored both in the shared helpers. The focused tests passed. Next run the full frontend and GUI-script checks and integrate.

Frontend size was 6,762 lines at d256279 and is 6,783 lines at 59d4cfe. Counts include every tracked TypeScript and TSX file under desktop/src, including tests, and exclude JSON fixtures.

The workspace check gate passed formatting, Clippy, and all 1,175 enabled Rust tests. Ten existing tests were ignored by the workspace command. Desktop-session all-features passed 196 tests. The final frontend run passed strict TypeScript and 578 tests in 41 files. Both normal and verification bundles built. The GUI script suite passed 177 tests. Fresh review covered all six fixture families and the final status and focus changes. Routine GitHub checks passed on the final feature commit (run 45917598363). Packaged GUI journeys were not run.

All seven CSS custom properties used across 11 stylesheets have definitions. Existing document label colors have contrast ratios of 8.97:1 and 9.08:1 against the header. The revised documentation links resolve.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
The six games share frontend game metadata, active request status, and native wire types, including Tenjiku results. Dialog, scrolling, and frontend test helpers are shared. Retired verification assertions and labels were removed, CSS references were corrected, and the production session and verification fixtures use one 256-result limit.

The required workspace, desktop-session, frontend, and GUI script checks passed. Integrated at 59d4cfe. MSH-243 (Game-Independent Strength Tool) is the direct successor.
<!-- SECTION:FINAL_SUMMARY:END -->
