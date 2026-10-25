---
id: MSH-29
title: Implement the authoritative native desktop session controller
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T13:07:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-27
documentation:
  - docs/desktop/m3-design.md
ordinal: 29000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Concurrent webview commands and delayed engine results must not corrupt or reinterpret the live chess match.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A serialized Rust controller owns an untimed M2 ChessMatch, exposes native pieces/legal choices/history/claims/conclusions, and rejects stale or invalid human commands atomically.
- [x] #2 Acceptance checks exact game context, request identity, session version and process generation; duplicates, superseded work and cancelled responses cannot apply actions.
- [x] #3 Focused native tests cover new-game version protection, same-board different-history context, generation mismatch, cancellation races, legal moves/claims and unsupported mode rejection; independent review is recorded.
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
Implement a tested framework-independent serialized ChessMatch controller with versioned native snapshots and human commands; bind engine envelopes to exact context and generation; add safe engine-turn lifecycle and adversarial context/cancel tests.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner codex; branch codex/m3-desktop; checkout .claude/worktrees/next-task. MSH-27 contract independently reviewed and integrated.

Implemented framework-free authoritative ChessMatch session and serialized DesktopController. 11 focused native tests pass: atomic commands, actual choices and claims, cancellation, duplicate/generation/ABA checks, identical FEN with different histories, incomplete and invalid results, native promotion/mate, and Full-mode restoration refusal. Independent review /root/m3_contract_review inspected boundaries and reran all11 tests; no actionable findings. Workspace clippy --all-targets --locked -D warnings and full cargo test --workspace --locked passed locally. Native CI/integration pending.

Integrated source 074fe515b9c9b9ab65dfc9a256dcbf0d07eb3132 on codex/rewrite. Native Rust CI https://github.com/jweathers777/mShogi/actions/runs/40196021766 succeeded on ubuntu-24.04 (121176901775), macos-15 (121176901942), windows-2022 (121176901969): fmt, workspace clippy, tests, build and console smoke. Existing M2 evidence reused; desktop package CI remains MSH-28 evidence.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented and independently reviewed the authoritative untimed native session controller with exact-context result acceptance, atomic human commands, native snapshots and explicit unsupported Full restoration. Eleven focused tests and all three native Rust CI jobs pass.
<!-- SECTION:FINAL_SUMMARY:END -->
