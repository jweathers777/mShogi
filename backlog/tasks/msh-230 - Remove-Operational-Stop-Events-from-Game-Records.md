---
id: MSH-230
title: Remove Operational Stop Events from Game Records
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-23T11:04:00Z'
updated_date: '2026-11-23T12:59:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Remove native Stop state and reservations; retain the explicit Tori schema-1 replay promise with wire-only legacy validation. Bump current schemas; development-era Tori-2/Chu-1/Tenjiku-1 encodings can retire under design A-09.
2. Move console stop feedback to session memory and migrate desktop, runner and verification fixtures. Preserve real commands and rules expectations.
3. Review the bounded migration, record source line counts, run Rust/frontend/persistence checks, then integrate and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started from main 8480977. Branch codex/msh-230-session-stops. Trace the native Stop commands, record compatibility readers, console/session feedback and verification fixtures before migration; preserve all native rules and player history.

Checkpoint: native Stop commands/incomplete state and Stop-only capacity reservations removed; current schemas Tori 3, Chu 2, Tenjiku 2. Explicit Tori schema-1 reader validates old annotations then omits them while retaining player history. Console interruption state is transient; desktop uses existing Cancel/failure state. All workspace targets compile. Full release tests and fresh review are running; frontend fixture refresh and complete checks remain. Branch codex/msh-230-session-stops.

Fresh review completed with no concrete Rust defects. Added direct console coverage for every stop reason across Tori, Chu and both Tenjiku profiles: byte-identical records/save and cleared feedback after reopen. TypeScript and 176 Python checks pass. First frontend run has only two expected old-fixture failures; native fixture regeneration is queued behind the two-job release build. Checkpoint CI found a single-element reservation-test loop; fixed. Separate existing example-output collision recorded as MSH-234.

First release workspace pass reached consumer tests and stopped on experiments reading the old checked-in Tori document schema. The native document generators will refresh Tori/Chu/Tenjiku GUI documents and manifests before rerunning. No game-rule failure observed in this pass.

Fixture migration: refreshed 87 existing frontend DTO fixtures and 43 existing saved-document/manifest fixtures through native generators. Compared all 41 changed saved documents against the prior files after removing only schema labels and operational fields: roots, player commands, effects, outcomes, sides and settings are identical. Source count (.rs/.ts/.tsx/.py/.mjs, all tracked paths) 172040 -> 171697, net -343. fmt and workspace all-target clippy pass. Full release/debug and affected-OS persistence verification remain before integration.

Final frontend fixture assertions now pass: 603 frontend tests and 176 Python tests. Updated the GUI analysis verifier to the existing read-only result contract (game version unchanged, session revision advances), with negative tests for stale revision or changed game version; regenerated native DTOs exposed the old assertion. No product behavior changed in this verification adjustment.

Final code 1b75415: full release workspace 1182 passed/11 ignored; desktop all-features 200 passed; frontend 603 passed; Python 176 passed; TypeScript, fmt and workspace all-target clippy passed. Targeted persistence passed Linux/macOS/Windows in GitHub run 45176582506. Final fresh review finding (exact analysis version in smoke verifier) fixed and negative-tested. Debug repeats four known MSH-199 Chu failures; an extra no-fail-fast run was stopped at the known-cost twelve-ply Chu protocol test and the required standard command is being completed. Source stages: 172040 baseline ->171696 (a6e3b6d) ->171697 (bf83d0c) ->171700 final, net -340. No Cloud Batch jobs/VMs, GCP compute cost USD0. No strength or packaged-GUI campaign: no search-strength change or release.

Required debug command completed with the same four known MSH-199 chu_internal failures (30.47 s); no test assertion or limit was changed. The full release suite passed all 1182 tests, including those four. Final evidence is under gs://mshogi-experiments-216675633801/runs/msh-230/ with a 60-day lifecycle. Ready to integrate.
<!-- SECTION:NOTES:END -->
