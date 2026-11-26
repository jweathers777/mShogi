---
id: MSH-265
title: One Admission Wait in the Tauri Host
status: Ready
assignee: []
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-26T11:55:00Z'
labels:
  - desktop
  - tauri
  - refactor
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 7750
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
desktop/src-tauri/src/main.rs (1,499 lines) repeats the reserve, receipt and 15 ms poll loop in game_command and game_new; game_file has a third poll loop, and current_contract_snapshot repeats game_snapshot. Verification fixtures are sorted into five modes (desktop_verification_mode), but desktop/src/verification/bootstrap.tsx renders the same ProductApp for three of them, and the verification flag of ProductApp adds nothing beyond verificationControls. The host tests repeat the stale-token checks, the receipt wait loop and near copies of the Tori and Chu file tests.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One admit-and-wait helper serves the host commands; the duplicate snapshot code and the verification modes that render the same application are merged.
- [ ] #2 Host tests use shared helpers; the Tauri tests pass, and the host CI passes on the affected operating systems.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.
<!-- SECTION:NOTES:END -->
