---
id: MSH-202
title: Tenjiku Search Speed
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-19T07:12:00Z'
updated_date: '2026-11-27T14:18:00Z'
labels:
  - tenjiku
  - search
  - model:large
  - effort:xhigh
milestone: m-5
dependencies:
  - MSH-214
references:
  - docs/m5/strength-roadmap.md
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: make the Tenjiku search faster with conventional techniques. Measure where search time goes (move generation, make and unmake, legality and check tests, attack tests), then remove the largest costs. This includes the unusual pieces (Fire Demon area moves and burning, range-jumping generals, Lion and Lion-like moves, Heavenly Tetrarch and Vice General moves). Keep a change only on evidence: correctness checks, a speed measurement, and a game test against the frozen predecessor.

Native rules stay the authority. A faster search-side position must agree with them in perft and differential tests. No rule changes.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Baseline and final speed are recorded on the same machine and positions: nodes per second and time to a fixed depth, with the cost profile that guided the changes
- [ ] #2 Tenjiku perft and differential tests against the native rules pass
- [ ] #3 Each kept change passed the blunder suite and a game test against the frozen predecessor; rejected changes are recorded with their result
- [ ] #4 A short design note in docs/ explains how the unusual pieces are generated and checked in search
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Freeze ba6cee4. Sample release search on standard positions for both profiles and measure costs before choosing one coherent optimization. Compare baseline and candidate at depth 3, cap 500000 nodes, on the first three catalog openings and the three MSH-214 ply-40 histories per profile, three repeats each on the same c3-standard-4 VM, with a fixed-depth signature. Run the 216 tactical positions per profile at depth 2. Run bounded, color-reversed clocked predecessor games separately on the first four catalog openings per profile at 60 s + 1.5 s, H0=0 H1=15, 768 ply and 1200 s per pair. Report all incomplete pairs and no inferred result. If full signatures stay identical this is a speed-only change: require faster measured searches, perfect suite, and completed game-test pairs per profile without a demonstrated regression. A changed search signature needs affirmative strength evidence before retention. Keep total cost below $10. Verify rules perft, full action equality and reversible effects on both profiles. Document the unusual routes and effects, review, run selected checks and integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-20, John: rescoped from a discussion task to an ordinary speed task; agents choose the techniques. Blocker: Tenjiku has no strength harness yet (opening set, blunder suite, game test against the predecessor; see MSH-204 (Tori Strength Harness) and docs/m5/strength-roadmap.md). Speed profiling and perft/differential work can start; AC 3 needs the harness.

Harness blocker is now the dependency MSH-214 (Tenjiku Strength Harness).

2026-11-25 (John): Tenjiku comes last, after the codebase cleanup; follows MSH-214.

2026-11-27 (John): Released MSH-202 for implementation after MSH-214. The architectural tasks MSH-277 through MSH-283 remain deferred and do not block this work.

Claimed on codex/msh-202-tenjiku-speed. Freeze ba6cee4 before any product edit. Next: baseline profile, fixed-depth census and matched predecessor tests for both profiles.
<!-- SECTION:NOTES:END -->
