---
id: MSH-196
title: Xiangqi Evaluation Round
status: Done
assignee:
  - '@claude'
created_date: '2026-11-18T19:25:00Z'
updated_date: '2026-11-19T12:46:00Z'
labels:
  - handle:xiangqi-evaluation-round
  - xiangqi
  - strength
milestone: m-5
dependencies:
  - MSH-195
references:
  - docs/m5/strength-roadmap.md
ordinal: 203000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Evaluation Round

**Problem.** The Xiangqi evaluation has only material, a placement table and a tempo bonus. Mobility, General safety (Advisors, Elephants, open files) and phase-dependent piece values (Cannon, Horse) are absent, and no weight is fitted.

**Objective.** The evaluation step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Xiangqi.

**Approach.** As in MSH-188 (Shogi Evaluation Round): a linear term set with incremental static terms, fitted with the shared Texel tool on at least about 500,000 quiet self-play positions generated in the cloud, with a held-out split.

**Completion.** A fitted evaluation that passes the sequential predecessor test, or a reported null result.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A self-play corpus of at least about 500,000 quiet positions is generated with recorded compute and a split by game or opening.
- [x] #2 The fitted weights lower the held-out loss against the initial weights; both losses are recorded.
- [x] #3 Correctness tests and the tactical suite pass; the set passes the sequential predecessor test (H1 about 10 Elo), or the null result is reported and reverted.
- [x] #4 Independent review is recorded and findings are resolved.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Executed plan, see report docs/m5/xiangqi-evaluation-round.md: 80-weight linear evaluation (INITIAL = old evaluation); cloud self-play corpus (3,200 pairs, 631,802 unique quiet positions); Texel fits; held-out loss read after each fit (three times); sequential tests H1=10 at 5 s + 0.1 s. fit1 H0, bounded revision fit2 H0, controlled experiment fit3 (per-node terms pinned at 0) H1 with confirmation; review follow-up: exchange values from material weights, no decision at 600 pairs (+9.0), kept on recorded grounds.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-19: corpus msh196-corpus (3,200 pairs, 631,802 unique quiet positions, US$1.19). fit1 (all terms) H0 -33 Elo/200 pairs; fit2 (bounded revision: skip zero per-node terms, Advisor/Elephant lower 150) H0 -33/200; controlled experiment fit3 (per-node terms pinned 0, otherwise fit2) H1 at 40 pairs +262, confirmation pairs 41-80 +221; combined +241 (187-308). Final source keeps fit3; search skips per-node work. Report docs/m5/xiangqi-evaluation-round.md. Release suites pass; review running.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Xiangqi now uses a Texel-fitted classical evaluation: material plus rank and file tables for Soldier, Horse, Chariot and Cannon, General placement and central Advisor/Elephant terms, with exchange values taken from the fitted material. Report: docs/m5/xiangqi-evaluation-round.md; evidence docs/m5/evidence/msh196.

Corpus: cloud self-play of the frozen predecessor (main 61d42fb9), 3,200 pairs at 2 s + 0.05 s, 631,802 unique quiet positions split by opening (training 528,821 / validation 46,516 / held-out 56,465), US$1.19 (+0.08 smoke), cleanup verified. Held-out loss 0.06162 -> 0.05259 (kept fit3).

Game tests (5 s + 0.1 s, H1 = 10): fit1 with per-node terms H0 (-33 Elo, 200 pairs); bounded revision fit2 H0 (-33); controlled experiment fit3 with per-node terms pinned at 0 H1 at 40 pairs, independent confirmation batch +221 Elo (144-325). Review follow-up: exchange values from material weights, no decision at 600 pairs (+9.0, -4.7 to +22.6), kept on recorded grounds (consistency; tactical suite at 5 ms 347 -> 358). Per-node terms stay in code at weight 0, skipped by the search; their failure is descriptive, not causally explained (refit also changed static weights).

Verification Level 1: release suites (search, engine, runner, console, desktop session) pass, Clippy/fmt clean, Python 139 passed; new tests: INITIAL equals the old formula, colour mirror, evaluation equals feature sum, Texel score equals search evaluation. Late-game depth median 10 -> 11. Linux/Windows CI deferred. Independent review: nothing blocking; findings applied. Next: MSH-197 (Xiangqi Strength Anchor) needs John's approval of the ladder and caps. Integrated into main by merge e15bdd63.
<!-- SECTION:FINAL_SUMMARY:END -->
