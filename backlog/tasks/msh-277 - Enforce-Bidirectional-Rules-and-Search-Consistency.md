---
id: MSH-277
title: Enforce Bidirectional Rules and Search Consistency
status: To Do
assignee: []
created_date: '2026-11-27T12:34:00Z'
labels:
  - technical-debt
  - model:large
  - effort:high
dependencies: []
documentation:
  - docs/design.md
  - docs/development.md
priority: high
ordinal: 251000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Make agreement between authoritative rules positions and optimized search positions a mechanical integration requirement. Build on MSH-254 and MSH-274, reusing the shared check selector, CI policy, and existing test coverage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Evaluate the current pre-integration and CI policy and record which suites and cases it runs or misses for changes on either side.
- [ ] #2 Changes to either position representation select and run the applicable differential and perft checks before integration and in CI, covering all six games and both Tenjiku profiles.
- [ ] #3 Differential coverage checks complete legal actions, resulting positions, game-specific effects, and make/unmake restoration where applicable.
- [ ] #4 Add relevant targeted regressions for any changed movement or transition semantics. Keep documented search-only approximations distinct from unintended rules divergence.
- [ ] #5 Check-selection tests prove that changes on either side cannot silently omit the applicable checks.
- [ ] #6 Reuse tools/checks.py, tools/ci_checks.py, and existing tests. Keep pre-commit hooks lightweight and avoid unnecessary repeated builds.
<!-- AC:END -->
