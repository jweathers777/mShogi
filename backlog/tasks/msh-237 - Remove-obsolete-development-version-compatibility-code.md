---
id: MSH-237
title: Remove obsolete development-version compatibility code
status: Done
assignee:
  - '@claude'
created_date: '2026-11-24T20:31:00Z'
updated_date: '2026-11-25T09:57:00Z'
labels:
  - cleanup
  - model:medium
  - effort:high
dependencies: []
references:
  - crates/mshogi-engine/src/chu.rs
  - crates/mshogi-search/src/chu.rs
  - crates/mshogi-engine/tests/all_games_timing.rs
documentation:
  - AGENTS.md
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The unreleased application still accepts historical evaluator/search identities, and an ignored timing test converts old protocol-v4 replies. These paths conflict with the zero-backward-compatibility rule in AGENTS.md. Remove obsolete development-version support while preserving current games, rules profiles and useful current regression coverage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Product readers, identity tables and callers use current development interfaces without obsolete version adapters or discarded fields.
- [x] #2 Tests and fixtures describe current behavior; obsolete protocol capture/conversion paths are removed.
- [x] #3 All six games and their selected current rules profiles remain supported, and required checks pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Removed (development-version support only):
- Full-width evaluator/search identities of five games, engine observation_identity classifier, their exact-score checks, XIANGQI_LEGACY_PLAY_LIMITS, per-game transport bounds for old unscored failures.
- Ignored v4 capture test; fixed-baseline.json now holds the current values (same values as the earlier in-test overrides).
- Optional Chess selective_depth (now required); defaulted play_mode in saved settings; defaulted experiment clock allocation.
- ExecutionContext.process_generation (report-compatibility metadata). Internal protocol is now 6.
- Console `record`, `record load` and `mshogi shogi` aliases.
- Historical "unsupported" Tori/Tenjiku scenario and fixture names (now "incomplete").

Kept, with reason:
- Chess record and match schema 1/2: current M2 and Full adjudication modes, selected by mode.
- Observation schema 1, desktop document v2 and native record schema numbers: the only current versions; others are rejected. No Tori schema-1 reader exists any more.
- ClockAllocation::Hard: still the move-time policy of Chess, Shogi, Xiangqi and Tenjiku matches.
- Material evaluate functions of the search adapters: no product caller now, but current position tests use them as oracles.
- docs/m4 rules corpus (TORI-C-IDENTITY input, CHU consumer wrong_process_generation) and docs/m4/chu/profile.md still mention process generation; the rules corpus and profile were not edited.

Strength runs: a frozen predecessor must be built from this merge or later (protocol 6). Reports and configs from earlier builds no longer replay.
Fixtures: regenerated with the existing generators. Committed Xiangqi, Chess capacity, Chu analysis and Chu complete-resignation fixtures were stale and now match generator output.
Review (one fresh subagent): found stale runner tests, Shogi audit tools that required the old clock shape, the macOS scenario selector, the Chu GUI engine label, old transport bounds and stale docs. All fixed.

Checks after merging main (2eca95f), macOS: fmt pass; clippy pass (also --all-features); cargo test --workspace 1186 passed, 0 failed, 11 ignored; desktop-session --all-features 200 passed; tsc pass; vitest 595 passed (Node 23.11, package asks for 24); Python tools/strength 175 (1 skipped), tools/experiments 30, desktop/scripts 177, ci_checks self-test 8. Routine GitHub Rust workspace CI passed on branch commit 4f8c458. Not run: three-platform full CI, packaged GUI journeys, engine-strength match (no search or evaluation change).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed obsolete development-version support: old search identities, protocol-v4 capture, process_generation (protocol 6), optional or defaulted old fields, console aliases and historical fixture names. All six games and rules profiles are unchanged. Excluding fixtures and Backlog, 1320 lines removed and 569 added. Checks pass (1186 Rust tests, 595 frontend). Follow-up: MSH-240 (Retire finished-campaign strength tools).
<!-- SECTION:FINAL_SUMMARY:END -->
