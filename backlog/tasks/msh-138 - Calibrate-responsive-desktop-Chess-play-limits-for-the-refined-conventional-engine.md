---
id: MSH-138
title: Desktop Search Limits
status: In Review
assignee:
  - '@codex'
created_date: '2026-11-08T23:48:00Z'
updated_date: '2026-11-09T11:46:00Z'
labels:
  - handle:desktop-search-limits
milestone: m-5
dependencies:
  - MSH-137
documentation:
  - docs/m5/chess-classical-refinement.md
priority: high
ordinal: 143000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Desktop Search Limits. Owner consolidation2026-11-09 supersedes the earlier retain-or-revise20k budget scope. Ordinary desktop Chess should spend its allocated thinking time, returning its last completed iteration. Remove practical node/depth ceilings; only effectively unreachable technical guards remain. Preserve exact request/limit identities, cancellation, bounded memory/process lifetime, old saved documents and observations. Keep1s default for now; player-configurable thinking time is separate future product work. No strength campaign is authorized by this task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Ordinary desktop Chess is time-primary: no practical node or depth cap preempts productive timed search, and actual native/controller/subprocess journeys demonstrate deadline stopping and the last completed legal iteration without fabricated fallback.
- [ ] #2 Cancellation, supervision, bounded memory and technical recursion/representation guards remain; old untimed and timed20k documents/observations open unchanged and stopped, with explicit Resume selecting new defaults and fresh identities. Analysis/other-game limits and schemas remain compatible.
- [ ] #3 Relevant native/process/controller, frontend, standalone host and focused actual packaged desktop verification plus independent review support the integrated behavior. Roadmap records configurable thinking time and owner-approved strength qualification before Shogi/Xiangqi; no rating claim or unapproved qualification campaign.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Codex owns clean origin-aligned codex/rewrite at78a5577; unrelated historical Tenjiku worktree preserved. Implement a JSON-exact effectively unreachable technical visit ceiling and retained128-ply stack guard for1s play; use fixed hard-time allocation (soft=hard) for timed engine turns, preserve explicit small-node requests. Admit/migrate both prior saved play configurations only on Resume, retaining old observations/files and complete identities. Present thinking time as primary in desktop. Verify focused search/engine/controller/archives, generated fixture consumers and standalone host; then actual focused packaged macOS journey and required cross-platform jobs at coherent source. In parallel record handles/roadmap qualification gate and produce research-backed approval proposal, with no qualification matches.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Next bounded Chess product checkpoint after137, before50/54 transfer. Review supports engine architecture readiness; calibrate ordinary play defaults without increasing admission ceilings or conducting new opponent studies.

MSH-137 is Done after independent final acceptance and required actual3OSCI42538719298 at2a2ff7c. This task is now dependency-ready, unassigned and next in the Chess-first sequence. Use current codex/rewrite and its source/evidence; no additional Chess opponent studies. Do not reinterpret engine-readiness evidence as GUI-default maximum strength.

User explicitly authorizes time-primary play and supersedes previous20k/default-calibration constraints. Global Backlog1.55.0 is incompatible; pinned1.55.1 CLI used. No other active worker owns this task. Required CI scope will be declared after consumer inspection before campaign.

Consolidated related owner directions: MSH-139 (Chess Strength Qualification) To Do/unassigned awaits approval and later demonstrated strength; it now gates MSH-50 (Conventional Shogi) and MSH-54 (Conventional Xiangqi). MSH-140 (Player Thinking Time) records future configuration without expanding immediate scope. No formal qualification games authorized or run.

Current MSH-138 (Desktop Search Limits) checks pass: both prior-config/historical-observation compatibility,6engine execution, nativecompleted-iteration cancellation,29Chesssession and11UCI cases,4actualreleaseDesktopController/subprocess journeys,570frontend tests andtypecheck/workspaceClippy/fmt. Nativefixed-time constructoralso gives UCI movetime its full budget; separate clockallocationunchanged. All16regeneratedChessfixtures differonly/play_limits/nodes. Required CI: Rust(ubuntu-24.04/macOS15/windows2022), nowincludingactualreleaseDesktopControllerplayjourney; localstandaloneTauri checks andonefocusednormalpackagedmacOSplay/cancel/save/reopen journey. Reuse unchanged all-six GUI/platform package evidence; no broadsix-gamecampaign orqualificationmatches. ActualWindows/LinuxGUIinteractionnotinferredfromlocalmacOS; cross-platformnativeDesktopControllerjourneysrequired.

MSH-138 (Desktop Search Limits) source and all16generatedfixturechanges independently approved with no findings. Allfocusednative/process/controller/570frontend/standaloneTaurichecks passed; formalrecords in docs/m5/evidence/msh138/verification.json. Code nowfrozen for normalfixed-identitypackagedmacOSjourney andactualrequired3OSRustCI. No Chessstrength experiments ran. MSH-139 (Chess Strength Qualification) target/protocol/budget proposal reviewed but notowner-approved.
<!-- SECTION:NOTES:END -->
