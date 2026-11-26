---
id: MSH-272
title: One Kept Result Count
status: To Do
assignee: []
created_date: '2026-11-25T22:12:00Z'
updated_date: '2026-11-26T11:47:00Z'
labels:
  - desktop
  - model:small
  - effort:low
milestone: m-6
dependencies: []
priority: low
ordinal: 249000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The desktop session keeps the newest 256 engine results with KEPT_RESULTS (native_session.rs). A second constant, MAX_ARCHIVE_OBSERVATIONS (persistence.rs), has the same value but no production use; only the verification capacity fixtures use it. Use one constant for both, and remove the unused one. Found during MSH-268 (Remove Game Size and Engine Request Limits).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One constant defines the number of kept engine results; MAX_ARCHIVE_OBSERVATIONS is removed and the verification fixtures use the remaining constant.
- [ ] #2 Workspace, desktop-session all-features and GUI script tests pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John approved consolidation on 2026-11-26. Entire implementation scope, acceptance criteria (updated only for superseded requirements), references and relevant context are now owned by MSH-264. This record is archived as superseded, not Done. No implementation or acceptance check was completed by consolidation. Use the surviving task for future work.
<!-- SECTION:NOTES:END -->
