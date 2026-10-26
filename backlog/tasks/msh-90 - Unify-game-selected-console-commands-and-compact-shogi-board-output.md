---
id: MSH-90
title: Unify game-selected console commands and compact shogi board output
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T16:47:00Z'
updated_date: '2026-10-26T16:55:00Z'
labels:
  - m4
  - owner-msh-46
  - console
milestone: m-4
dependencies:
  - MSH-68
  - MSH-69
  - MSH-70
documentation:
  - docs/m4/delivery-plan.md
  - docs/m4/shogi-console.md
ordinal: 90000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-requested bounded console follow-up after MSH-68/70; preserve native authority and machine contracts. Stop before MSH-71/72.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Canonical --game chess|shogi shell/profile and selected help work in actual subprocesses; default chess and shogi aliases survive; invalid/conflicting selections and incompatible machine modes fail clearly without protocol stdout contamination.
- [ ] #2 Common position/show/legal/move/undo/history/status/perft/divide operations use native adapters; hands, advisory eligibility, formal declaration/terminal commands and imported provenance retain native meanings.
- [ ] #3 Both adapters support typed-envelope record print/save/load-file/load-json and documented legacy aliases. Bounded strict wrong-game/malformed/unsupported/oversized loads preserve session and undo; new-file saves never overwrite. Chess observations/Full remain explicitly unsupported.
- [ ] #4 Dedicated compact formatter reproduces the owner-specified 7g7f 3c3d board byte for byte, with aligned promoted tokens/dots/files, unchanged legal labels and native state/encodings; hands/status remain below it.
- [ ] #5 Primary docs and executable examples use explicit game selection and describe native history/status, aliases, limits and record semantics. Focused subprocess tests, existing protocol/console regressions, independent review and applicable CI support completion.
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
1. Primary builder @codex owns clean main checkout on codex/rewrite, base c2c18b9. No competing implementation or temporary checkout needed. Inspect native adapters and strict RawValue envelope; retain native authority.
2. Add minimal game/argument dispatch and shared bounded record-file utilities with explicit canonical envelope operations/native restore admission. Keep legacy record encodings/aliases and machine launches intact.
3. Add isolated compact formatter, update help/guides and actual subprocess regressions for both games, atomic failure/undo and exact display.
4. Run focused console/engine regressions and native checks; obtain separate read-only review, resolve findings, commit/push normally, record native CI and completion. Stop before MSH-71/72.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup: only main worktree, clean codex/rewrite at c2c18b9; no active owners. Global Backlog 1.55.0 differs from baseline, so use verified /private/tmp/mshogi-bootstrap-npm-cache/_npx/749414256ca1c44f/node_modules/backlog.md-darwin-arm64/backlog 1.55.1. Sandboxed creates failed EPERM before task creation; authorized escalated CLI created MSH-90 once. No historical tasks modified.

Implementation: minimal selected human dispatch; explicit common typed-envelope record operations with game-specific legacy aliases, shared 1MiB bounded reads/atomic no-clobber publication, strict native RawValue parsing and native-only restoration admission; dedicated five-character shogi formatter. No rules/search/wire encoding changes. Initial existing console suite 60 tests passed. New common subprocess suite: 9 tests, initial 8 pass/1 documentation snapshot failure before guide update; after updating guide all9 pass. Workspace all-targets Clippy passed. Full native regression run and independent read-only review now in progress. MSH-46/75 closure dependencies added without removal; MSH-68/70 unchanged.

Pre-integration verification on macOS27.0.1/Darwin27 arm64, rustc1.89.0: cargo test --workspace --locked passed327 (50 test groups,0 failures); cargo fmt --all -- --check, workspace all-targets Clippy -D warnings, git diff --check passed. Existing console69 include9 new common-interface tests and actual chess UCI/v2/v3/shared/desktop subprocess regressions. Additional all-feature desktop-session46 passed. Release build passed. Executed release --game shogi shell with position startpos, move7g7f3c3d,show; captured /private/tmp/msh90-board-capture.txt exactly matches owner layout and docs/m4/shogi-console.md fixture; hands/status follow. Executed revised shogi-engine.md Python example (explicit selector + record print): depth2,nodes962,rootSente score0,PV1g1f1a1b. No native rules/search expectations changed.
Independent method: separate read-only AI reviewer /root/msh90_review reviewed complete diff against c2c18b9, native RawValue/restoration consumers and task/docs. Independent CARGO_TARGET_DIR=/private/tmp/mshogi-review90-target cargo test --locked -p mshogi-console passed69. Additional real subprocess probes passed trailing selector dispatch, both-game identity, symlink no-clobber/temp cleanup and conflicting-schema atomic rejection; revised engine example matched. No material findings. Optional README grammar nit corrected. Native/platform CI pending on integrated source; local execution does not establish other platforms. No temporary worktree was created; primary checkout remains codex/rewrite.
<!-- SECTION:NOTES:END -->
