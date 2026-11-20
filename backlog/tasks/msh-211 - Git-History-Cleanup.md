---
id: MSH-211
title: Git History Cleanup
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-20T22:27:00Z'
updated_date: '2026-11-20T22:27:00Z'
labels:
  - repository
milestone: m-6
dependencies: []
ordinal: 218000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: remove from all Git history the files that should never have been committed: evidence directories under docs/, docs/review/, screenshots, archives and logs under docs/, and the former vendored tao copy. Move the few live inputs that code still reads out of evidence first. Keep legacy-cpp and master unchanged. Back up the full repository to GCS before the rewrite.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Live inputs read by code or tests are moved out of evidence directories and the workspace tests pass
- [ ] #2 A verified bundle of the pre-rewrite repository is in gs://mshogi-experiments-216675633801/backups/
- [ ] #3 No commit in history contains the removed paths; legacy-cpp and master keep their hashes
- [ ] #4 origin main and main_v2 are force-pushed and the local checkout uses the new history
<!-- AC:END -->
