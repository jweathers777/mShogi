---
id: MSH-133
title: Record deferred historical Shogi expansion in the product roadmap
status: In Review
assignee:
  - '@codex'
created_date: '2026-11-05T08:24:00Z'
updated_date: '2026-11-05T08:46:00Z'
labels: []
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
John requests a planning-only roadmap update for prospective larger historical Shogi variants. Complete M4 and prioritize six-game M5 engine strength, efficiency and quality before any owner-authorized expansion. Preserve all existing tasks, milestones and dependencies; stop after reviewed integration and push.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The authoritative roadmap records the five provisional variants and ordering, explicit deferral and unchanged M4/M5 priorities without authorizing implementation or rules research.
- [x] #2 Design records prospective large-board UX, native/shared ownership, engine investigation topics and engine-independent future delivery expectations while preserving current six-game contracts.
- [x] #3 Documentation and Backlog checks plus proportionate independent review confirm every pre-existing task and milestone is unchanged and no new implementation task or dependency is introduced.
- [ ] #4 The reviewed documentation update is integrated into codex/rewrite and pushed; the isolated worktree is cleaned up and no further task starts.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect current roadmap, milestones, M5 plans and accepted shared/native architecture. 2. Add one deferred roadmap section and narrow cross-references, preserving current contracts and all existing Backlog records. 3. Validate exact record preservation, graph and links; obtain bounded independent review. 4. Integrate only this documentation/task diff into codex/rewrite, push and remove the isolated checkout.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex for this documentation task only. Branch codex/historical-shogi-roadmap; isolated app-managed checkout /Users/johnweathers/.codex/worktrees/historical-shogi-roadmap/mShogi, based on a4bb78c6dfca7bf486f9aa4ebb88b278ca0a5838. Primary codex/rewrite has active MSH-44 Tenjiku work and an unrelated design appendix; preserve all of it. Use cached pinned Backlog.md 1.55.1, not PATH 1.55.0. No required CI jobs: documentation/Backlog checks and independent review only; no build/native/GUI campaign.

Verification record, 2026-11-05: Darwin 27.0.0 arm64, Python 3.11.0, pinned Backlog.md 1.55.1. Base a4bb78c6dfca7bf486f9aa4ebb88b278ca0a5838. PASS: Python/subprocess git ls-tree + git show byte comparison preserves all 140 pre-existing Backlog files, including all 132 tasks and all milestone definitions/configuration. PASS: Python additive-line and new relative-link/heading-anchor checks across four docs; git diff --check; pinned backlog doctor (no duplicate IDs/self-dependencies/cycles). Initial doctor under restricted sandbox failed EPERM creating an empty local drafts directory; authorized retry passed. Independent read-only reviewer /root/roadmap_review PASS, no findings: separately verified 140-file preservation, 133 unique/resolved task IDs and acyclic graph/no duplicate edges, all new links, roadmap consistency and main Tenjiku appendix separation. Reviewed SHA256 design b54cb6d19c714cc8cded52ae3b42134a4d0cdb28f6c32b253bf2a41b509c499d; architecture 468a571f79cc679678b4119a8e1aea96a5c84491fc67d7f92ed20e106560d883; M4 1fdd33c2c546114a516c7c1dcdd3870567b09769581b91b1d49708d2b072c999; M5 50df6b33e467f0a4e281033e8fac1091e81c18a521d716e170b0f751f6ef4b59. No required CI jobs; production builds/native tests/GUI verification and historical fact research NOT RUN, appropriate for planning-only scope. No future initiative task/milestone or implementation tasks created; MSH-133 tracks only this documentation update. Integration will stage only the reviewed patch and this task, preserving concurrent primary dirty files and its design appendix.
<!-- SECTION:NOTES:END -->
