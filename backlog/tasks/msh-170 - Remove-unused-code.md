---
id: MSH-170
title: Remove unused code
status: Ready
assignee: []
created_date: '2026-11-14T11:26:00Z'
labels:
  - handle:unused-code-removal
milestone: m-5
dependencies: []
priority: medium
ordinal: 174000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Unused Code Removal.

**Problem:** The rewrite left code that no product path, test or tool uses. One known example is `crates/mshogi-search/src/chess_candidate.rs`, an older Chess search with no consumer after MSH-164 (Shared Search Core). The compiler does not report such code when it is public or is reached only by its own tests. Unused code costs reading time and can mislead later work.

**Objective:** The repository contains no unused Rust, frontend, tool or script code on the rewrite branch.

**Approach:** Owner request, 2026-11-14. Identify candidates in all workspaces (Rust crates and their examples and benches, the standalone Tauri host, the frontend, `tools/`, scripts): modules, public items, features, dependencies and files with no consumer other than their own tests. Record the list with the evidence for each item. Remove the items that are clearly unused. Bring to John each item that a design requirement, a planned variant or a recorded task still needs, or whose status is not clear; do not remove those. Historical evidence under `docs/` and the preserved original history are out of scope. Behavior must not change. Level 1.

**Completion:** A recorded candidate list with a disposition for each item, the clear items removed, and all workspace checks passing.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A list of unused code candidates across the Rust workspace, the Tauri host, the frontend and the tools is recorded with the evidence for each
- [ ] #2 Each candidate is removed, or kept with a recorded reason or owner decision
- [ ] #3 Workspace builds, tests and linters pass after the removal, and product behavior does not change
- [ ] #4 Unused dependencies and features found in the same search are removed or recorded
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
