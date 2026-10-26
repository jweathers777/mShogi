---
id: MSH-90
title: Unify game-selected console commands and compact shogi board output
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T16:47:00Z'
updated_date: '2026-10-26T17:10:00Z'
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
- [x] #1 Canonical --game chess|shogi shell/profile and selected help work in actual subprocesses; default chess and shogi aliases survive; invalid/conflicting selections and incompatible machine modes fail clearly without protocol stdout contamination.
- [x] #2 Common position/show/legal/move/undo/history/status/perft/divide operations use native adapters; hands, advisory eligibility, formal declaration/terminal commands and imported provenance retain native meanings.
- [x] #3 Both adapters support typed-envelope record print/save/load-file/load-json and documented legacy aliases. Bounded strict wrong-game/malformed/unsupported/oversized loads preserve session and undo; new-file saves never overwrite. Chess observations/Full remain explicitly unsupported.
- [x] #4 Dedicated compact formatter reproduces the owner-specified 7g7f 3c3d board byte for byte, with aligned promoted tokens/dots/files, unchanged legal labels and native state/encodings; hands/status remain below it.
- [x] #5 Primary docs and executable examples use explicit game selection and describe native history/status, aliases, limits and record semantics. Focused subprocess tests, existing protocol/console regressions, independent review and applicable CI support completion.
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

Reviewed source integrated and normally pushed directly from the sole owned primary checkout on codex/rewrite: 1388722412ab9272fcea1967c983871a512a286c. Base c2c18b9; working tree clean after push. No temporary worktree/merge needed for this bounded single-builder follow-up. Reviewer final spot-check of parse-once cleanup and README correction passed9 focused tests, no findings; SHA256 main88ea75af42d87fa00c68ea9bd0697e472d8d41306215e5496758bec486e0ac04, session577c9960c22368cee6cc52b35ee3a1d5561fee0addeea5133514ee753dd960bb, shogi_sessione04e78955b1c3b65a98fb00bb8ebc6d006c2fc8eac5a3d31704eb656c80096ac, record_ioa8a12b8c3202d42aeadb255781f1c90ac6921b3599655b372151af67ef8fdcb4, common_console83e037f3ed5abe920b6ebb9c8e4deea17483233028ba8b62c4de773f71f53b14 match integrated files. Native CI https://github.com/jweathers777/mShogi/actions/runs/40363752343 and desktop CI https://github.com/jweathers777/mShogi/actions/runs/40363752410 launched on exact1388722; pending, not yet completion evidence. MSH-68/70 remain Done unchanged, MSH-71/72 Ready unstarted; M4/MSH-41–47 open.

Final integrated CI on 1388722412ab9272fcea1967c983871a512a286c: Rust workspace https://github.com/jweathers777/mShogi/actions/runs/40363752343 attempt1 SUCCESS on ubuntu-24.04/macos-15/windows-2022, including format/lint/workspace tests/all-feature native session tests/generated fixture equality/build/both canonical console smokes. Desktop packages https://github.com/jweathers777/mShogi/actions/runs/40363752410 attempt1 SUCCESS on all3: checks, normal package construction and bundled-engine smoke; Linux/Windows additionally execute installed/packaged GUI regression journeys using separately labeled verification builds. macOS package/engine evidence is not a new GUI run; unchanged accepted M3 macOS GUI/Tao/native-termination evidence is reused. No failed/cancelled/retried CI jobs. Non-failing runner-capacity and existing action-runtime deprecation annotations do not alter verdicts.
AC coverage:1 actual selected shell/help/profile/aliases/invalid-args/UCI tests;2 native adapter and unchanged MSH68 event/declaration/provenance tests;3 common file/JSON roundtrip, strict hostile/unsupported/oversized atomic loads with successful undo, no-clobber/symlink probes;4 exact owner board plus promoted stride/record equality and actual release capture;5 updated primary docs/executed engine example, independent69-test review and green native/desktop CI. All applicable DoD satisfied. Main checkout is the only worktree; no temporary checkout was created or needs cleanup.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the bounded common-console follow-up and integrated/pushed source 1388722412ab9272fcea1967c983871a512a286c on codex/rewrite. Canonical --game chess|shogi shell/profile routes to existing native adapters; default chess/shogi aliases and machine UCI/internal-search contracts remain intact. Both consoles now expose explicit typed-envelope record print/save/load-file/load-json with strict native validation, bounded reads, safe no-clobber publication and atomic native restoration/undo; legacy record forms retain documented game-specific meanings. Compact shogi show matches the requested five-character layout, with separate unchanged verbose action labels, hands and status.
Evidence:327 local workspace tests,46 all-feature desktop-session tests,fmt/Clippy/diff checks and release build/capture passed. Separate read-only AI source review, independent69-test console rebuild and adversarial subprocess probes found no material issue; minor documentation correction resolved. Native and desktop CI both passed attempt1 on all3 target systems at exact source1388722 (links above). Normal packaged Linux/Windows GUI regressions passed; macOS package/engine checks supplement unchanged prior GUI evidence.
Limits remain explicit: chess diagnostic status/history is position/move-level, observation-bearing/Full chess restoration rejects; shogi is untimed, USI and desktop adaptation remain future work. No rules/search/record schema changes or strength claim. MSH68/70 stay Done with unchanged evidence; required MSH90 closure dependencies were added to MSH46/75 without removal. MSH71/72 remain Ready/unstarted; M4 and MSH41–47 remain open. Stop at this task.
<!-- SECTION:FINAL_SUMMARY:END -->
