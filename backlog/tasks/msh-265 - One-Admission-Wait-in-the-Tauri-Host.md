---
id: MSH-265
title: One Admission Wait in the Tauri Host
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-27T00:47:00Z'
labels:
  - desktop
  - tauri
  - refactor
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
references:
  - https://github.com/jweathers777/mShogi/actions/runs/45918215436
priority: medium
ordinal: 7750
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
desktop/src-tauri/src/main.rs (1,499 lines) repeats the reserve, receipt and 15 ms poll loop in game_command and game_new; game_file has a third poll loop, and current_contract_snapshot repeats game_snapshot. Verification fixtures are sorted into five modes (desktop_verification_mode), but desktop/src/verification/bootstrap.tsx renders the same ProductApp for three of them, and the verification flag of ProductApp adds nothing beyond verificationControls. The host tests repeat the stale-token checks, the receipt wait loop and near copies of the Tori and Chu file tests.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 One admit-and-wait helper serves the host commands; the duplicate snapshot code and the verification modes that render the same application are merged.
- [ ] #2 Host tests use shared helpers; the Tauri tests pass, and the host CI passes on the affected operating systems.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Keep controller admission and file picker reservations at their current boundaries. Share the asynchronous waiter and operation completion polling.
2. Route equivalent fixtures to the same product mode and use the controls prop to render Tenjiku verification controls.
3. Reuse host polling, stale-token and file test helpers, then run the shared checks and all affected host CI jobs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-265-tauri-admission. Next: combine the host admission waits and verification entry points, refactor the host tests, and run affected host CI.

Shared the host waiter and snapshot path, consolidated product verification modes, and made the host tests use common admission, stale-token, and file helpers. Next: run the integration checks and desktop host CI for Linux, macOS, and Windows.

Verified code commit 0deb722 locally with python3 tools/checks.py run --base origin/main. TypeScript, 578 frontend tests, formatting, Clippy, and Tauri tests (8 normal, 16 verification) passed. Both frontend builds passed, including the production verification-marker check. Desktop host CI run 45918215436 on 0deb722 passed on Linux. Next: wait for the macOS and Windows jobs to pass, then complete criterion 2, integrate, and clean up codex/msh-265-tauri-admission.
<!-- SECTION:NOTES:END -->
