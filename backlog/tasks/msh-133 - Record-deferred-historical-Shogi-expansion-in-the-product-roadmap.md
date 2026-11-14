---
id: MSH-133
title: Record deferred historical Shogi expansion in the product roadmap
status: Done
assignee:
  - '@codex'
created_date: '2026-11-05T08:24:00Z'
updated_date: '2026-11-14T18:42:00Z'
labels:
  - handle:historical-shogi-roadmap
dependencies: []
documentation:
  - docs/design.md
  - docs/m4/delivery-plan.md
  - docs/architecture-health-roadmap.md
  - docs/m5/opening-knowledge.md
type: docs
ordinal: 133000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
**Problem**
The roadmap needed a record of John's interest in larger historical Shogi games after the current six-game programme.

**Objective**
Record Dai, Dai Dai, Maka Dai Dai, Tai and possibly Taikyoku Shogi as future options. Complete M4, then give priority to M5 strength, speed and quality for the six existing games. Work on the extra games requires a future decision from John.

**Approach**
Add this direction to docs/design.md and add links from the related plans. Record possible large-board display needs and shared engine responsibilities. Keep existing tasks, milestones and dependencies unchanged.

**Completion**
The documentation was independently reviewed, integrated and pushed. The temporary checkout is now absent. This task is complete; it does not authorize research or implementation of the extra games.

Handle: Historical Shogi Roadmap
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The authoritative roadmap records the five provisional variants and ordering, explicit deferral and unchanged M4/M5 priorities without authorizing implementation or rules research.
- [x] #2 Design records prospective large-board UX, native/shared ownership, engine investigation topics and engine-independent future delivery expectations while preserving current six-game contracts.
- [x] #3 Documentation and Backlog checks plus proportionate independent review confirm every pre-existing task and milestone is unchanged and no new implementation task or dependency is introduced.
- [x] #4 The reviewed documentation update is integrated into codex/rewrite and pushed; the isolated worktree is cleaned up and no further task starts.
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
1. Inspect current roadmap, milestones, M5 plans and accepted shared/native architecture. 2. Add one deferred roadmap section and narrow cross-references, preserving current contracts and all existing Backlog records. 3. Validate exact record preservation, graph and links; obtain bounded independent review. 4. Integrate only this documentation/task diff into codex/rewrite, push and remove the isolated checkout.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex for this documentation task only. Branch codex/historical-shogi-roadmap; isolated app-managed checkout /Users/johnweathers/.codex/worktrees/historical-shogi-roadmap/mShogi, based on a4bb78c6dfca7bf486f9aa4ebb88b278ca0a5838. Primary codex/rewrite has active MSH-44 Tenjiku work and an unrelated design appendix; preserve all of it. Use cached pinned Backlog.md 1.55.1, not PATH 1.55.0. No required CI jobs: documentation/Backlog checks and independent review only; no build/native/GUI campaign.

Verification record, 2026-11-05: Darwin 27.0.0 arm64, Python 3.11.0, pinned Backlog.md 1.55.1. Base a4bb78c6dfca7bf486f9aa4ebb88b278ca0a5838. PASS: Python/subprocess git ls-tree + git show byte comparison preserves all 140 pre-existing Backlog files, including all 132 tasks and all milestone definitions/configuration. PASS: Python additive-line and new relative-link/heading-anchor checks across four docs; git diff --check; pinned backlog doctor (no duplicate IDs/self-dependencies/cycles). Initial doctor under restricted sandbox failed EPERM creating an empty local drafts directory; authorized retry passed. Independent read-only reviewer /root/roadmap_review PASS, no findings: separately verified 140-file preservation, 133 unique/resolved task IDs and acyclic graph/no duplicate edges, all new links, roadmap consistency and main Tenjiku appendix separation. Reviewed SHA256 design b54cb6d19c714cc8cded52ae3b42134a4d0cdb28f6c32b253bf2a41b509c499d; architecture 468a571f79cc679678b4119a8e1aea96a5c84491fc67d7f92ed20e106560d883; M4 1fdd33c2c546114a516c7c1dcdd3870567b09769581b91b1d49708d2b072c999; M5 50df6b33e467f0a4e281033e8fac1091e81c18a521d716e170b0f751f6ef4b59. No required CI jobs; production builds/native tests/GUI verification and historical fact research NOT RUN, appropriate for planning-only scope. No future initiative task/milestone or implementation tasks created; MSH-133 tracks only this documentation update. Integration will stage only the reviewed patch and this task, preserving concurrent primary dirty files and its design appendix.

Integration evidence: reviewed roadmap source integrated on codex/rewrite as 2ac32c012d386562cb80a42852cb7c44f3aee515, above concurrent disjoint CI commit 00d4fc2b49ca72e68beb2eccad5dd3edd5839d45. Initial integration guard stopped before mutation when the concurrent branch tip advanced; reinspection confirmed only two workflow paths changed. A narrow staged patch then preserved all 100 existing dirty/untracked files and the separate 17-line Tenjiku design appendix byte-for-byte. Exact integrated git-show SHA256 checks matched all four reviewed documents; all 140 pre-existing Backlog records remained identical to integration parent, and git diff parent revision --check passed. Normal git push origin codex/rewrite succeeded; local and origin tips matched 2ac32c0. Pinned backlog doctor passed again in the primary checkout. All existing M4/M5 status/scope/criteria/dependencies/milestone ownership remain unchanged by this task; active MSH-44 work remains with its existing worker. No next task started. Cleanup limitation: archive_worktree refused the app-created historical-shogi-roadmap checkout with The task primary checkout cannot be removed. attach_worktree for the original integration directory also refused because it is not a managed worktree. No protection was bypassed and no ordinary Git deletion substituted. Retained checkout has no build outputs or needed ignored files; lsof showed idle app tool runtimes only. Its sole ongoing purpose is supported app-managed archival after the app no longer treats it as this chat primary. Reviewed changes and MSH-133 are already integrated/pushed; the disposable branch has no unique commits. AC4 and DoD1/4 remain unchecked and task In Review solely for that cleanup requirement. Roadmap work is finished; expansion remains deferred and no further task is authorized.

Closure check, 2026-11-14: John requested completion and a plain-language description. Codex owns this record-only update on codex/rewrite in the primary checkout. Backlog.md 1.55.3 matches the current project baseline. Plan: verify the original integration and checkout absence, simplify the description, preserve all historical notes, and close the task. The original integration commit 2ac32c012d386562cb80a42852cb7c44f3aee515 is an ancestor of HEAD c83278116db2985dca31c137a0be1314db63b2df and origin/codex/rewrite. git worktree list no longer lists the historical-shogi-roadmap checkout, and its recorded directory does not exist. The earlier cleanup refusal remains above as historical evidence; the former cleanup blocker no longer applies. No claim is made about when or how that checkout was removed. AC4 and DoD1/4 can now be completed using these checks and the recorded original review, integration and push evidence. Verification level: Level 1, documentation record only; no product source changes, CI, native tests or GUI checks are required. Broader platform verification remains at its applicable integration/release checkpoint. No new task started and no follow-up is needed for this closure.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the roadmap documentation for possible Dai, Dai Dai, Maka Dai Dai, Tai and Taikyoku Shogi expansion. The direction remains deferred: finish M4, then prioritize M5 strength, speed and quality for the existing six games. Extra-game research and implementation require a future owner decision. The reviewed documentation was integrated and pushed on codex/rewrite in 2ac32c012d386562cb80a42852cb7c44f3aee515. Original documentation, link, Backlog integrity and independent review checks passed as recorded above. On 2026-11-14, the integration was confirmed in current history and the temporary checkout was confirmed absent. All acceptance criteria and completion items are satisfied. Rewrote the description in plain terms and kept the old checkout refusal in the historical notes. No product code changed and no follow-up task is needed.
<!-- SECTION:FINAL_SUMMARY:END -->
