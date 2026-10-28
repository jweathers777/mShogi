---
id: MSH-96
title: Deliver standalone xiangqi console and native diagnostics
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-28T09:03:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-84
  - MSH-95
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 96000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Add explicit xiangqi selection/profile to the existing independently usable console, inspect native board/side/history/outcomes, list/apply complete actions, exact undo and relevant perft/divide diagnostics; preserve common human/machine output and existing games.
- [x] #2 Use documented game-owned notation/position adapters and canonical envelope save/load with strict failure atomicity; refuse lossy unsupported conversions or incomplete-history restoration explicitly.
- [x] #3 Actual subprocess tests cover bounded input/errors, terminal commands preserving board ply, history-sensitive results and replay; record focused checks and independent review. Protocol/match consumers remain a separate ticket.
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
1. Add explicit Xiangqi console selection/profile and a native session adapter for board/local choices, authoritative commands/history/warnings/claims/outcomes, local perft/divide and exact undo; no evaluator/search or external protocol.
2. Add the minimal native validated restore-with-undo helper and policy getter required by this real consumer; use strict canonical envelopes and transactional candidates with actual encoded-bound preflight, preserving failed loads/moves/saves.
3. Expose clearly separate fast and explicit bounded exact no-win diagnostics through one cancellable native worker/result slot; retain exact purpose/target/source context, retire old work on replacement/undo, clean up on quit and never apply an analysis result as a game action.
4. Add actual subprocess journeys for native state/history/terminal commands, record Save/Load/undo, strict failures/bounds and exact Unknown/cancel/stale behavior; preserve Chess/Shogi compatibility.
5. Independently review, run focused/native regressions and applicable CI, integrate/push and complete96 before rereading graph and activating97.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Root@codex sole builder/integrator on primary codex/rewrite atfd5fe6f34bcff8299f0990fb4ea71ce1b194c1eb. Clean sole checkout, refreshed origin matches, no active task owners. Dependencies84/95Done. BacklogPATH1.55.0 read-only; pinned1.55.1 matchesAGENTS and owns mutations. Task97 remains unstarted until96Done per authorizedqueue. Console needs native restore-with-undo (Game internal command ledger stays authoritative) and readable policy access; no speculative universal interface. Exact work is diagnostic/read-only and not a new canonical saved mode.

Implemented native console, strict canonical Save/Load and native command undo, explicit bounded read-only exact diagnostics and retirement; documented docs/m4/xiangqi/console.md. macOS arm64: cargo fmt --all; cargo clippy --workspace --all-targets --locked -- -D warnings; cargo test --workspace --locked PASS. Separate reviewer console96_review reviewed final source/docs, executed48 focused tests PASS, no material findings; docs/m4/xiangqi/console-reviews/msh-96.md. Initial fixture corrections preserved native rule contract. Await source integration/native CI before completion;97 remains unstarted.

Integrated/pushed source d7de50d1e0b931c4dd86e8fec0126fdedcc5c254 on codex/rewrite. Required Rust workspace CI40658621082 PASS on ubuntu-24.04, macos-15, windows-2022 (format, lint, workspace tests, QA constructors, snapshot regeneration, build and console smoke). https://github.com/jweathers777/mShogi/actions/runs/40658621082 . Supporting desktop package CI40658621076 still running at completion, not claimed passed. backlog doctor and git diff --check PASS. All3AC/4DoD supported; no remaining finding. No temporary worktree created.97 can now begin under authorized queue after dependency reread.

Supporting desktop package CI40658621076 subsequently PASS all3 platforms (normal package/bundled engine plus packaged GUI compatibility journey infrastructure). https://github.com/jweathers777/mShogi/actions/runs/40658621076 . This supplements, rather than substitutes for, recorded native console verification.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered explicit native Xiangqi console/profile, Red/Black board/status/history/warnings/claims/terminal commands, complete move batches, exact command undo, perft/divide and strict atomic canonical Save/Load. Bounded explicit exact diagnostics preserve actual full-source provenance, Unknown/failure and cancellation/retirement without committing an outcome. Native restoration helper retains undo only when requested; default parsers unchanged. Eleven actual subprocess fixtures plus all workspace regressions pass; independent final source/docs reviewer executed48 passing tests with no material findings. Integrated source d7de50d1e0b931c4dd86e8fec0126fdedcc5c254, three-platform native CI40658621082 all PASS. External protocols100, evaluator/search/child97 and desktop98+ remain separate;45/M4 stay open.
<!-- SECTION:FINAL_SUMMARY:END -->
