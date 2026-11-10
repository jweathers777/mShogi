---
id: MSH-155
title: Chess Time Scaling
status: Done
assignee:
  - '@codex-coordinator'
created_date: '2026-11-10T11:25:00Z'
updated_date: '2026-11-10T12:57:00Z'
labels:
  - handle:chess-time-scaling
dependencies:
  - MSH-154
ordinal: 160000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Time Scaling.

## Problem
One-second assessments may obscure longer-time decision quality and do not establish a general strength ceiling or how opponents scale.

## Objective
Measure current decision quality at increasing time, compare reference-engine scaling, and prioritize engineering from the evidence.

## Approach
Freeze a small independent balanced benchmark including fresh positions and success controls; run bounded matched-time diagnostics with full histories and honest proof/reference distinctions. No substantial implementation or tournament.

## Completion
Reproducible scaling evidence, independently challenged conclusions, a stable routine benchmark and revised forward roadmap returned for owner review.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A frozen balanced decision-quality benchmark includes fresh independently justified tactical, defensive, positional and endgame expectations, full known histories and multiple acceptable moves where appropriate.
- [x] #2 Current mShogi and established references are compared at matched increasing allowances, recording actual time, choices, score/bounds, PVs and available search/iteration telemetry; regressions and inconclusive observations remain visible.
- [x] #3 Evidence separates absolute improvement from relative competitive claims and distinguishes execution, coverage and valuation hypotheses without unapproved matches or implementation.
- [x] #4 Independent review, Level1 local verification, reproducible evidence and a revised coherent roadmap support an explicit owner checkpoint; MSH-153 remains held.
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
1. Independently select and freeze8 balanced roots including4 fresh, plus outcome criteria and methodology review before results.
2. Reuse existing native frontier and reference UCI infrastructure; verify bounded driver and exact source/binary identities.
3. Sequential matched1/5/15s on8roots and3engines; at most2 predeclared informative roots at30s. Operational ceiling900s aggregate active searches including oracle probes/failures; no matches. Adjust down if outcomes stop discriminating.
4. Review time-quality trajectories and limited reference-relative evidence; use only narrowly discriminating followups within allowance.
5. Level1 diagnostic assertions, evidence validation, independent final review and updated roadmap; platform/GUI campaigns not required, broader compatibility deferred to future implementation/integration risk.
6. Integrate/push findings, stop for owner review before substantial implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner Codex coordinator; reusing clean app-managed chess-adversarial-assessment worktree and branch, integration codex/rewrite. Base dcc17e1; unchanged production source d53f344. Pinned Backlog1.55.1 via explicit Node23 runtime. Fresh SolHigh panel selector, AstraMedium methodology reviewer and AstraHigh diagnostic harness reviewer; no competing production builders. Owner request stored with evidence.

Level 1 diagnostic on macOS arm64, unchanged production d53f344. Consolidated evidence: docs/m5/evidence/msh155/verification.json; report: docs/m5/chess-time-scaling.md. Independent methodology approval in methodology.md; harness-verification is explicitly an author recheck. verify_evidence.py passed: 715 artifacts, 109 attempts, 683 ledger hashes, source equivalence, stable fixture and report links. 723.175 seconds active searches plus 0.132 seconds proof construction, below 900; zero matches. One pre-search initialization failure retained/charged; successful reviewed retry. No production or cross-platform/GUI checks claimed; broader compatibility deferred to future risk-selected implementation.

Assessment commit 0f6d24a integrated by fast-forward into codex/rewrite; package verifier passed again in the integration checkout. All diagnostic acceptance criteria and completion evidence are satisfied; status remains In Review for the explicitly requested owner checkpoint. Next action is owner review, not an implementation task. The app-managed assessment worktree is retained clean for that review: previous archive attempt reports that this task primary checkout cannot be removed; do not bypass the app lifecycle. No engine process requires it.

Owner approved the general revised roadmap on 2026-11-10 and authorized a broader independent public-data benchmark followed by one coherent conventional engineering round. The prior owner checkpoint is satisfied; new benchmark must precede substantial engine changes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the balanced eight-root matched-time diagnosis, stable benchmark and refined roadmap. Two selected historical errors improve by the earliest successful sampled allowance of five seconds; fresh exact controls already pass at one second. No supported regression in this small panel and no longer-time rating claim. Coverage/valuation and measured native execution remain complementary priorities. Independent final review approved diagnostic closure. MSH-153 (Classical Compensation Calibration) stays held. In Review for John before substantial implementation; normal integration target codex/rewrite.

Integrated assessment: 0f6d24a on codex/rewrite, with final metadata recorded separately. Owner-review checkpoint is the only remaining action; no further experiment or implementation started.
<!-- SECTION:FINAL_SUMMARY:END -->
