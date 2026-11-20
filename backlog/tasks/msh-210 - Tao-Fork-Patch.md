---
id: MSH-210
title: Tao Fork Patch
status: Done
assignee:
  - '@claude'
created_date: '2026-11-20T22:17:00Z'
updated_date: '2026-11-20T22:23:00Z'
labels:
  - desktop
milestone: m-6
dependencies: []
ordinal: 217000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: replace the 37k-line vendored copy of tao in desktop/src-tauri/vendor/tao with a git dependency on John's GitHub fork (jweathers777/tao), which carries only the 34-line macOS applicationShouldTerminate patch, and offer the patch upstream to tauri-apps/tao.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 desktop/src-tauri uses tao from the fork branch mshogi/v0.37.2-should-terminate at a pinned revision; vendor/tao is deleted
- [x] #2 The desktop host builds and its tests pass on macOS
- [x] #3 An upstream pull request to tauri-apps/tao is open, or the reason it is not is recorded
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Point [patch.crates-io] tao at the fork rev 97dd480. 2. Delete vendor/tao, update Cargo.lock. 3. cargo build + test of desktop/src-tauri on macOS. 4. Merge to main. 5. Port patch to tao dev and open the upstream PR.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Fork branch jweathers777/tao mshogi/v0.37.2-should-terminate, rev 97dd480: tag tao-v0.37.2 plus the app_delegate.rs patch, byte-identical to the former vendored file (only file that differed from the registry package). macOS aarch64, Rust 1.99.1: desktop/src-tauri cargo test --locked 8 passed, clippy -D warnings clean, cargo tree shows tao from the fork. GUI not re-run: patched source is byte-identical to the version verified in MSH-35. Upstream PR https://github.com/tauri-apps/tao/pull/1442 (port to dev using define_class!; cargo check/test/fmt pass on macOS). Commits are unsigned; tao asks for signed commits.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced the 37k-line vendored tao copy with a pinned git patch to John's fork, which carries only the macOS applicationShouldTerminate change (Dock Quit, logout and shutdown reach the Save/Discard/Cancel prompt). Offered the change upstream as tauri-apps/tao#1442. Desktop host tests and clippy pass on macOS. Follow-up: remove the patch entry when a tao release with the change reaches Tauri; sign the upstream commit if maintainers require it.
<!-- SECTION:FINAL_SUMMARY:END -->
