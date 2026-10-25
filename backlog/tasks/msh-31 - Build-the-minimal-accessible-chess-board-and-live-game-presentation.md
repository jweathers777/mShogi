---
id: MSH-31
title: Build the minimal accessible chess board and live game presentation
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T14:15:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-28
  - MSH-29
documentation:
  - docs/desktop/m3-design.md
ordinal: 31000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Players need a readable board and complete native action selection without duplicating rules in JavaScript.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The board-centered SVG interface renders native pieces, turn, game status and move history; legal native choices drive click-to-move and all promotion choices.
- [x] #2 Board orientation can change independently of human color; keyboard arrows, Enter/Space, Escape, visible focus and coordinate/piece labels support board and promotion operation.
- [x] #3 Interaction tests and running-app evidence cover native choice selection, promotion, orientation and keyboard behavior; frontend contains no rules engine and independent review is recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Connect the completed native controller through serialized Tauri commands; render native snapshots and complete choices in an SVG board; add source/destination selection, promotion dialog, orientation and roving keyboard focus; show live history/status and actual final engine observations; verify focused frontend interactions and actual native application behavior, then independently review and integrate.

Verify and fix real native shutdown across macOS Quit and window close, retain failed-before-fix and precise corrected child-lifetime evidence, then collect final source native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @codex with isolated builder /root/desktop_foundation; branch codex/m3-desktop, checkout .claude/worktrees/next-task. Foundation28 and native controller29 complete with all applicable CI; no test instrumentation will be added to normal app.

Implemented native SVG board/controller wiring. Local13 frontend tests, strict TS/Vite, host fmt/clippy/build and11 native session tests pass. Independent /root/m3_contract_review resolved promotion focus, rendered-version binding and explicit Tauri shutdown findings; final shared-deadline/menu/cleanup review has no findings. Actual normal .app played White e2e4/a7a5 then keyboard g1f3/a5a4, and Black a2a3/e7e5/a3a4, with real depth2 telemetry. Native screenshots include default820 and compact650 full-board layouts. Actual controlled-child check exposed macOS predefined Quit bypass (failed child survived30s); corrected custom Quit and final teardown verified live child removal127.477ms afterQuit and42.663ms afterwindowclose, beforewatchdog/natural exit. Failure and successful exact-host proof retained; normalapp no instrumentation. Development command build/process/HTTP smoke passed, CUA unbundled GUI binding unavailable and not claimed. See docs/desktop/first-batch-evidence.md. Native CI and integration pending.

Final independent evidence/script review found no blocking claim issues; screenshots and exact shutdown timing/hash assertions checked. Root cargo fmt, workspace clippy --all-targets --locked -Dwarnings and full workspace tests passed after integration hardening. Final normal macOS host SHA5e2f9d39e266a054b71019c1073dca125d701a5b5e17e2845ae2187ca9fa224e; real engine SHA1461e608869d948b945e811075df7e5ebc07c49b3d8e8a91c72aa6117083a60e. Final normal package UCI and structured-interface search smokes pass and instrument/fixture markers are absent from production web assets. See retained local report and screenshots.

Integrated source d2883dfea338e0048d31e138b4c5ae49cc6cd4a8 on codex/rewrite. Final Rust CI https://github.com/jweathers777/mShogi/actions/runs/40200520368 and Desktop packages CI https://github.com/jweathers777/mShogi/actions/runs/40200520366 succeeded on all3 native systems. Each desktop job ran13 frontend tests, strict TS, host fmt/clippy/tests, built normalpackage and verifiedactualpackagedhostpresence/SHA plus UCI and structured engine searches. Installed UI verification remains MSH-38; actual local GUI evidence is macOS.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered native-choice SVG chess board with click/keyboard selection, explicit promotions, independent orientation and live native history/status/telemetry. Independent review and actual packaged play resolved focus, stale-version, viewport and macOS Quit lifecycle issues. Local native evidence and both final CI workflows pass on all3systems. M3 remains open; minimal-play integration MSH-32 follows.
<!-- SECTION:FINAL_SUMMARY:END -->
