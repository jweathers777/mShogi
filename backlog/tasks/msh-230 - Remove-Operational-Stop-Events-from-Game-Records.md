---
id: MSH-230
title: Remove Operational Stop Events from Game Records
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-23T11:04:00Z'
updated_date: '2026-11-23T12:12:00Z'
labels:
  - simplification
  - core
milestone: m-6
dependencies: []
priority: high
ordinal: 237000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Split from MSH-229: Tori, Chu and Tenjiku store cancellation, engine failure and resource/work limits as native Stop commands and incomplete fields. These are session facts, not player moves or game outcomes; their storage also creates special ending reservations. Remove this operational history while preserving real commands, rules conclusions, explicit failure/retry feedback and the recorded compatibility policy.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Operational stops no longer add native game commands or alter the saved game; console and desktop report failures and cancellation as session state
- [ ] #2 Native rules, repetition, player endings and complete action histories are unchanged; existing rules expectations are not loosened
- [ ] #3 Migrate affected consumers and fixtures in bounded reviewed steps; record line counts and pass required Rust/frontend/persistence checks after each step
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started from main 8480977. Branch codex/msh-230-session-stops. Trace the native Stop commands, record compatibility readers, console/session feedback and verification fixtures before migration; preserve all native rules and player history.
<!-- SECTION:NOTES:END -->
