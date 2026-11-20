---
id: MSH-211
title: Git History Cleanup
status: Done
assignee:
  - '@claude'
created_date: '2026-11-20T22:27:00Z'
updated_date: '2026-11-20T22:36:00Z'
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
- [x] #1 Live inputs read by code or tests are moved out of evidence directories and the workspace tests pass
- [x] #2 A verified bundle of the pre-rewrite repository is in gs://mshogi-experiments-216675633801/backups/
- [x] #3 No commit in history contains the removed paths; legacy-cpp and master keep their hashes
- [x] #4 origin main and main_v2 are force-pushed and the local checkout uses the new history
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Backup: gs://mshogi-experiments-216675633801/backups/mshogi-pre-rewrite-2026-11-20.bundle (git bundle --all, verified, 437 MiB; includes refs/codex/* checkpoint refs). Old-to-new commit map: backups/mshogi-rewrite-2026-11-20-commit-map.txt. git filter-repo 2.47.0 --invert-paths: ^docs/(.*/)?evidence/, docs/review/, desktop/src-tauri/vendor/, ^docs/.*\.(png|jpe?g|gif|bmp|gz|tgz|xz|zip|tar|log)$. Result: 0 matching paths in any commit; 888 -> 870 commits on main (evidence-only commits dropped); pack 475 MiB -> 9.1 MiB. master 941cd4d, legacy-cpp 408f298 and main_v2 unchanged. crates, tools, desktop, tests, backlog, .github, Cargo files byte-identical in the new main tree; cargo test --workspace --locked --no-run passes on it (macOS). origin main force-pushed 33577c1 -> 6473a7f with lease; remote main_v2 holds none of the paths. Local repo adopted the new refs, codex checkpoint refs deleted, reflog expired, gc. MSH-200 evidence copied to runs/msh-200/ in GCS. Backlog notes that cite old commit hashes now point to old commits; use the commit map.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed evidence directories, docs/review, screenshots, archives, logs and the former vendored tao copy from all history; repository pack went from 475 MiB to 9 MiB. Live inputs moved out of evidence first. Full pre-rewrite backup and the commit map are in GCS under backups/. legacy-cpp, master and main_v2 unchanged. Follow-up: MSH-212 (Finished Study Tools Removal); local untracked evidence leftovers (about 53 GB) await John's decision.
<!-- SECTION:FINAL_SUMMARY:END -->
