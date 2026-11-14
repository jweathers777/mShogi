---
id: MSH-171
title: Rewrite Main Promotion
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-14T12:28:00Z'
updated_date: '2026-11-14T19:09:00Z'
labels:
  - handle:rewrite-main-promotion
dependencies:
  - MSH-169
  - MSH-170
references:
  - docs/m4/final-delivery.md
  - docs/m4/shared-integration/verification.json
  - docs/m5/evidence/msh164/verification.json
  - backlog/tasks/msh-172 - Two-Human-Desktop-Play.md
priority: medium
ordinal: 175000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Rewrite Main Promotion.

**Problem**
John requested a comparison and a task for promoting the rewrite on 2026-11-14. Remote refs confirm default master at 941cd4dc37e6e6210d4f993c96a553753e228b19, no main branch, and codex/rewrite at 1abc30917ae0bf41781270f3262a08485f06e052. master is an ancestor of the rewrite; all 621 rewrite-only commits preserve the old history. The rewrite is broader but is not a strict feature superset.

**Objective**
Owner decision, 2026-11-14: preserve the existing Git history, tag the old C++ version at 941cd4dc37e6e6210d4f993c96a553753e228b19, and create main from the accepted codex/rewrite tip once MSH-169 (Engine Session Persistence) and MSH-170 (Unused Code Removal) are integrated. Make main the default development branch. This promotion is authorized subject to the recorded integration and verification criteria; do not request the same branch-policy approval again. Two-human desktop play is separate follow-up MSH-172 (Two-Human Desktop Play) and does not block promotion. Mini Shogi, old saved-file import, old selectable-search/quiescence/script controls and Force move are not required; John accepts their absence.

**Approach**
Use the selected normal history-preserving promotion: create main at the accepted rewrite tip after both dependency tasks are integrated, publish an annotated tag for the original C++ tip, and set main as the remote default. Keep the original master and main_v2 history accessible. Do not create an orphan history or force-push. Reconcile workflow instructions that currently require codex/rewrite in the primary checkout. Audit unique branch commits with their owners; integrate applicable reviewed work and record the disposition of research-only, duplicate or superseded work. Do not merge every old task branch blindly.

Comparison at the recorded tips:
- master registers Mini Shogi, Chu and Tenjiku in src/Engine/Engine.cpp. The rewrite supplies Chess, standard Shogi, Xiangqi, Tori, Chu and Tenjiku, including two Tenjiku profiles in desktop/src/product/ProductApp.tsx. Mini Shogi is absent. Other old piece/resource directories do not establish playable games.
- Old Save/Load uses custom text despite its .pgn extension (src/Engine/GameRecord.cpp and src/GUI/mShogiFrame.cpp). No old-format import adapter was found. Current .mshogi documents and native JSON records are different; do not imply generic PGN support.
- Old desktop supports two human players. Rewrite setup assigns one human side against mShogi. Old console has six selectable searches, quiescence controls and script commands; rewrite tools do not preserve those command interfaces. Old Force stops search and plays its result; rewrite cancellation pauses without a move. John accepted these old control differences on 2026-11-14; two-human desktop play is tracked separately in MSH-172 (Two-Human Desktop Play).
- Rewrite rules, history, inspection, supervised execution, protocols, tests and strength infrastructure are much broader. Six-game M4 acceptance is recorded in docs/m4/final-delivery.md. This does not prove every Rust engine is stronger than the C++ engine. MSH-164 (Shared Search Core) compares against the preceding Rust engine only.
- README.md, desktop/README.md and some module comments still describe incomplete earlier milestones. Rust CI push filters use codex/** and PR filters use codex/rewrite. Update branch references and public coverage before promotion. During documentation cleanup, remove personal references to the project owner, including third-person references by name. State product requirements, decisions and workflow rules directly without personal attribution. Preserve required copyright/license notices and historical evidence records.
- MSH-169 (Engine Session Persistence) has an unmerged implementation commit on claude/msh-169-engine-session. MSH-170 (Unused Code Removal) has active dirty work in the primary checkout. codex/parallel-experiment-runner and codex/tenjiku-historical-access each have one unique documentation/research commit. Most other task branches have no unique commits. Recheck tips and ownership at execution.

**Completion**
Record decisions, branch dispositions, tested/promoted revisions, independent review, legacy references, actual remote default and rollback instructions. Use Level 3 integration verification with actual Linux/macOS/Windows native and desktop evidence; reuse unchanged source/scope evidence honestly. MSH-58 (Release Distribution) retains release policy, signing and distribution scope. Promotion alone is not a release or a strength claim. No tests, builds, GUI run or new strength campaign ran for this source comparison.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 John has selected the branch/default policy and authorized the remote default change; Mini Shogi, old saved files, two-human desktop play and old search/Force controls each have a recorded acceptance or linked follow-up decision.
- [ ] #2 Current ownership and outstanding branches with unique commits have recorded dispositions; required reviewed work is integrated and unrelated dirty work is preserved.
- [ ] #3 User documentation, coverage comments, CI branch filters and workflow instructions match the actual six-game product and approved integration/default branch.
- [ ] #4 A structured Level 3 record identifies the exact candidate source, actual three-platform native/desktop evidence, justified reused evidence and limitations; material findings are resolved and independent final review approves promotion.
- [ ] #5 main is created and published at the accepted rewrite tip after MSH-169 (Engine Session Persistence) and MSH-170 (Unused Code Removal) are integrated; an annotated tag preserves the old C++ tip, original/prototype histories remain accessible, and no orphan replacement or force-push is used.
- [ ] #6 Remote tips/default and applicable CI routing are verified; rollback instructions and handoff are complete without an unsupported release or playing-strength claim.
- [ ] #7 Current project documentation and workflow guidance state requirements and decisions directly, without personal references to the project owner or third-person references by name; required copyright/license notices and historical evidence remain intact.
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
1. Record branch/worktree dispositions and preserve unrelated files and historical refs. 2. Update current six-game documentation, remove personal attribution from current guidance, and route workflow/integration instructions to main. 3. Commit a coherent candidate on codex/rewrite; verify with Level 3 full Rust and packaged desktop CI on Linux/macOS/Windows, reusing unchanged historical rules/profile evidence with exact source qualifications. 4. Obtain independent review of final changes and platform evidence; resolve material findings. 5. Tag the C++ tip, create and publish main at the accepted rewrite tip, set the remote default and primary checkout to main, verify routing/history/rollback, and remove safely completed branches/worktrees. Record actual evidence and retained cleanup limits.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner decision recorded on 2026-11-14: preserve history; tag the old C++ tip; create main from the accepted rewrite and make it the default after MSH-169 (Engine Session Persistence) and MSH-170 (Unused Code Removal) are integrated. Other missing old features are not required. MSH-172 (Two-Human Desktop Play) is a separate Ready feature, not a promotion dependency. This edit records the decision only; no branch/default/tag change or feature implementation was performed.

Documentation requirement added on 2026-11-14: eliminate personal references to the project owner during the promotion documentation cleanup, including third-person references. No personal attribution is needed in current explanatory documentation. This is a requirement for the future cleanup; the cleanup has not run.

Owner: @codex, primary integration checkout /Users/johnweathers/Development/Personal/mShogi, branch codex/rewrite, start 2a3f132a. Backlog 1.55.3 matches baseline. MSH-169 and MSH-170 are Done and integrated. Level 3: branch promotion is an integration checkpoint following shared search/session changes; full Rust and desktop package jobs required on ubuntu-24.04, macos-15 and windows-2022. Prior M4 rules/packaged scope evidence is historical and will be reused only where unchanged; current execution must pass at the candidate revision. Independent reviewer /root/promotion_review is assigned read-only. Three untracked notification receipts are unrelated and will be preserved.

Candidate cfcf9d9e22eb46b7603e558b1951549f5e421008 published. Level 3 campaigns dispatched: Rust 43629719990 (suite full/platform all), Desktop 43629723352 (operation package/platform all). No passed platform claim yet. Local CI-policy self-tests passed 8/8; formatting and whitespace checks passed. Removed 14 completed local branches after ancestry checks. Managed Tenjiku checkout remains clean: attach_worktree refused ownership by another task; no app artifact identity is exposed to this chat, so no manual deletion or protection bypass. Branch dispositions and preserved unrelated notification hashes are in docs/review/evidence/msh171/branch-audit.json.

Actual local macOS27 arm64 normal package built at candidate compiled inputs and launched through CUA at the fixed org.mshogi.desktop identity. Chess completed two replies (4plies); Tori Gote engine-first play, Save/Open stopped at1ply, explicit Resume, human capture/engine reply to3plies, read-only root inspection/Return, Take back to1ply with2observations retained, Save and native Quit passed. Actual archives and identity/package receipts are retained under docs/review/evidence/msh171. Existing ad-hoc normal-bundle signature limitations are recorded; no signing/notarization or future approval reuse claim. Initial actual package smoke failed obsolete exact21node assertion (actual27); independent review approved contract-based verifier correction f8d8f2d5. Desktop43629723352 canceled, replacement43630147834 package/all pending. Native43629719990 continues with identical compiled/test inputs and command evidence. Four explicit remote branches removed after Done/criteria/ancestry checks; initial broad deletion rejection and the safer verified retry are recorded.
<!-- SECTION:NOTES:END -->
