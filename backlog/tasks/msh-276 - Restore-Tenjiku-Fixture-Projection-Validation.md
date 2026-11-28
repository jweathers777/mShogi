---
id: MSH-276
title: Restore Tenjiku Fixture Projection Validation
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-27T07:17:00Z'
updated_date: '2026-11-28T20:58:00Z'
labels:
  - documentation
  - model:small
  - effort:medium
dependencies: []
documentation:
  - docs/m4/tenjiku/fixtures.md
priority: high
type: bug
ordinal: 250100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The standalone Tenjiku data validator fails with "profile expectation digest changed" on main at 5b5c35f. The terminology change cadca34 updated fixture scope values, the source file hash and the validator, but the locked per-profile projections still include the old scope bytes. Restore validation of the reviewed fixture data.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The standalone Tenjiku validator and its negative data-admission tests pass.
- [ ] #2 The new projection digests are traceable to the scope terminology change. Independently derived rules expectations and validation checks are preserved.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Compare predecessor and current fixture projections and verify the terminology-only change. 2. Update the reviewed projection locks. 3. Run the validator, negative tests, and shared checks; integrate and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Compared cadca34^ fixture bytes and both projections with their predecessor locks. After the terminology renames in cadca34, both complete predecessor projections equal the current projections; independently derived values are unchanged. Updated the two projection locks. Next: run checks and integrate. Branch: codex/msh-276-fixture-projections.
<!-- SECTION:NOTES:END -->
