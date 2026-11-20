---
id: MSH-200
title: Tori Search Speed
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T07:12:00Z'
updated_date: '2026-11-20T23:39:00Z'
labels:
  - tori
  - search
milestone: m-5
dependencies: []
references:
  - docs/m5/strength-roadmap.md
priority: high
ordinal: 207000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: make the Tori search faster with conventional techniques. The cost profile (docs/m5/tori-move-generation.md, cloud c3-standard-4, build e6df0d79) shows that move generation is cheap (about 4 ns per move) and that trying moves is not: two attacked(Phoenix) tests take about two thirds of each make/unmake, and drops are 64% of moves. Implement the speed set A-D from that note: drop legality and check without attack tests, a danger-set filter for ordinary moves, a cheap Swallow drop-mate test, and ray tables for the Eagle and Quails. Drop ordering (option E) belongs to MSH-205 (Tori Selectivity Round).

Native rules stay the authority. The search-side position must agree with them in perft and differential tests. No rule changes.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Baseline and final speed are recorded on the same machine and positions: nodes per second and time to a fixed depth on the MSH-204 positions
- [ ] #2 Tori perft and differential tests against native legality and check flags pass (random play, game positions, suite and fixtures)
- [ ] #3 Each kept change passed the blunder suite and a game test against the frozen predecessor; rejected changes are recorded with their result
- [ ] #4 docs/m5/tori-move-generation.md is rewritten as a short design note of the final method
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Predecessor: main at the task start (bdf0c75; code as d50c082). 2. Exact speed set in tori/position.rs: ray tables with first-blocker scan, reverse-ray attack test, legality test only in check / Phoenix move / departure from a ray line to the Phoenix, check from the moved piece plus discovered-ray test, Swallow drop-mate test over Phoenix moves and Swallow captures only. 3. Correctness: native differential tests incl. a new child check-flag test (48 games), perft, Swallow drop rules; mutation checks of the new paths. 4. Cloud (tori_cloud --match, one c3-standard-4 type): same-VM depth-6 profile and 20 ms suite probe of both builds (node totals must match exactly), then candidate-predecessor pairs at 5 s + 0.1 s, sequential SPRT H0 0 / H1 15 in seed order. 5. Rewrite docs/m5/tori-move-generation.md as a design note.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-20 measurements (cloud c3-standard-4, build e6df0d79, job msh200-movegen-1, 106 s, about US$0.01, cleanup verified; evidence docs/m5/evidence/msh200): real game positions have 57 candidate moves, 64% drops; generate-all 234 ns (4 ns/move); per-piece 3-4 ns (step pieces), 10-18 ns (Eagle, Quails); attacked(Phoenix) 20 ns, called twice per make; make+unmake 59-71 ns; perft 94 ns/leaf; search depth 6 1.22 M nodes/s. Finding: trying moves (two attack tests per make) dominates, not generation. Options A-E and recommendation in docs/m5/tori-move-generation.md; awaiting John's decision.

2026-11-20, John: rescoped from a discussion task to an ordinary speed task. The options questions in the note need no answer; agents choose the techniques and measurements decide. Branch codex/msh-200-tori-movegen holds the cost profile tooling and the note; it is merged with main and pushed. Measurement data: gs://mshogi-experiments-216675633801/runs/msh-200/.

2026-11-21: the cost-profile tooling (tori::movegen_profile, tori_strength movegen, tori_cloud movegen task) and docs/m5/tori-move-generation.md are merged to main (d50c082); branch codex/msh-200-tori-movegen is deleted. Start a new branch from main.
<!-- SECTION:NOTES:END -->
