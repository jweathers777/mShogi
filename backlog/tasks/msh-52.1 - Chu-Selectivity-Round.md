---
id: MSH-52.1
title: Chu Selectivity Round
status: In Progress
assignee:
  - codex
created_date: '2026-11-24T17:44:00Z'
updated_date: '2026-11-24T17:49:00Z'
labels:
  - chu
  - search
milestone: m-5
dependencies:
  - MSH-201
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/chu-strength-harness.md
parent_task_id: MSH-52
ordinal: 244000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Improve Chu search decisions with a bounded round of classical shared-core selectivity techniques. John started this step after MSH-201; preserve native Chu rules and use the accepted MSH-201 engine as the frozen predecessor. Evaluation tuning and the later strength measurement are separate steps.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Quiescence checks, full SEE, game-scaled margins, move ordering and shared df-pn mate search each have a recorded measured outcome or a concrete reason they do not apply.
- [ ] #2 Kept changes pass native correctness and tactical checks and have supporting clocked predecessor evidence; inconclusive or rejected experiments are reported honestly.
- [ ] #3 Required workspace checks pass, useful design documentation is current, and run identities, settings, costs and cleanup evidence are recorded.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze accepted MSH-201 source 79c2409 (product-identical to main 6c2414b) and reuse its hashed Linux binaries and frozen opening/tactical inputs. 2. Test one coherent candidate: eight quiet royal-threat moves at the first quiescence ply, complete threat generation with native root restrictions, and quiet ordering by the existing placement delta. Existing full compound exchange model and MARGIN_UNIT=100 remain; shared df-pn is inapplicable because Chu admits moves into threat and its royal-capture rule_score is not the solver mate terminal. 3. Run focused native comparisons, elementary/forcing probes at 20ms, fixed-depth cost diagnostics, and clocked predecessor SPRT H0=0/H1=15 alpha=beta=.05. Use c3-standard-4,10s+0.1s nominal, at most600 paired openings, checkpoints in complete batches, within US$10 and under two hours of game campaigns; record cutoffs unscored and every launched pair. Choose batch sizing after the tactical/cost pilot before game results. Keep supported changes; report and revert null results. 4. Run workspace checks, document technique outcomes, integrate the result and delete task resources.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started after John requested Chu selectivity under MSH-52. Only this step is assigned; later evaluation/measurement remain separate. Branch codex/msh-52.1-chu-selectivity. Frozen engine source79c2409 from runs/msh-201/build-final/out/mshogi, SHA25637f238fb151d90fea8af39067f4f7a26037d0fb9831eae0df3d602f84ed90472. No native rules, compatibility shims or history-work models will be added.

Candidate372b689 implemented and pushed. The new native comparison checks the complete set of royal-threat successors across sourced fixtures and restricted roots. Focused Chu checks30 passed/1 ignored, workspace Clippy and fmt pass; full workspace tests running. Cloud build build-msh521-candidate is active at runs/msh-52.1/build-candidate. Initial submission configuration failed locally before any allocation; corrected to the shared controller parallelism setting, actual build has one task/VM. Next: same-VM tactical probes and fixed-depth cost comparison, then freeze game batch sizing before results.
<!-- SECTION:NOTES:END -->
