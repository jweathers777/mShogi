---
id: MSH-52.2
title: Chu Evaluation Round
status: In Progress
assignee:
  - codex
created_date: '2026-11-24T19:15:00Z'
updated_date: '2026-11-24T19:23:00Z'
labels:
  - chu
  - evaluation
milestone: m-5
dependencies:
  - MSH-52.1
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/chu-strength-harness.md
parent_task_id: MSH-52
ordinal: 245000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace the untuned Chu material and placement weights with a measured classical evaluation, using the shared Texel fitter and native self-play outcomes. This is the evaluation step John requested after the selectivity round; no neural evaluation or external opponents.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 About 500,000 or more quiet self-play positions have native outcome labels, a split by game or opening, duplicate control and recorded held-out loss before and after fitting.
- [ ] #2 Evaluation features agree with the search score through moves, captures, promotion and undo; native correctness and tactical checks pass.
- [ ] #3 Kept weights have supporting predecessor SPRT evidence at H1 about 10; unsupported candidates are reported and reverted.
- [ ] #4 Required checks and review are complete, durable documentation is current, and run identities, costs and resource cleanup are recorded.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a Chu model for the shared Texel fitter, preserving the current score as INITIAL and sharing fitted material with static exchange. Test feature/search agreement and undo.
2. Add native self-play corpus generation; measure a small cloud pilot before scaling to about 500,000 quiet labelled positions. Freeze game/opening splits and remove duplicates.
3. Fit static material and placement first; measure held-out loss, tactical results and search cost. Add per-node terms only if evidence supports the extra work.
4. Run the candidate against frozen MSH-201 source 79c2409 at the established fast time control, SPRT 0/10. Keep only supported changes.
5. Complete required checks and fresh review, record costs and run identities, integrate and remove task resources.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Checkpoint: branch codex/msh-52.2-chu-evaluation. Added an 88-weight static material/advance/central-file/tempo model; DEFAULT still INITIAL. Shared material feeds SEE and capture ordering. Added shared-fitter adapter and native self-play tool with no labels for unfinished games. Initial feature/move/undo tests passed. Next: cloud corpus pilot (500/2000 nodes per turn, native outcomes), then freeze production protocol after measuring yield. No cloud spend yet; no candidate weights fitted.
<!-- SECTION:NOTES:END -->
