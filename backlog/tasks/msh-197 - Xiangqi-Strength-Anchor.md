---
id: MSH-197
title: Xiangqi Strength Anchor
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-18T19:25:00Z'
updated_date: '2026-11-19T13:58:00Z'
labels:
  - handle:xiangqi-strength-anchor
  - xiangqi
  - strength
milestone: m-5
dependencies:
  - MSH-196
references:
  - docs/m5/strength-roadmap.md
ordinal: 204000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Strength Anchor

**Problem.** Predecessor tests show gains, not strength. Chess (MSH-185) and Shogi (MSH-191) have approximate ratings against public rating lists; Xiangqi has no current public rating list for engines (MSH-194, Xiangqi Strength Harness).

**Objective.** Measure Xiangqi strength by the program's method for games without a rating list (docs/m5/strength-roadmap.md, "Measurement without a rating list", John 2026-11-19): a fixed reference ladder (relative Elo) and move quality judged by a much stronger engine, reported separately.

**Approach.** Freeze the ladder before any game: classical ElephantEye, Fairy-Stockfish without its neural network, and Pikafish at three or four fixed node limits (exact versions and licences confirmed). Play a round robin in the cloud with the clocked route and fit all ratings together (Ordo or BayesElo) with one rung fixed at 0. Judge the ladder games with full-strength Pikafish using the MSH-193 definitions; this may be shared with MSH-198 (Xiangqi Blunder Screen). Optional small method check on Chess and Shogi, where true anchors exist. No public servers and no published games; no human-scale rating claimed. Report unscored stops (clock exhaustion) per engine.

**Completion.** Ladder placement, gap to full Pikafish, judged blunder rate and mean win-probability loss, with uncertainty, conditions and compute. Status stays To Do until John approves the ladder and caps (proposal: about 6 hours, US$15).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 John's approval of the opponent pool, time control and caps is recorded before any game.
- [ ] #2 Independent review is recorded and findings are resolved.
- [ ] #3 The frozen ladder (engines, versions, licences, limits), the fitting method and the reference rung are recorded before any game.
- [ ] #4 mShogi's ladder placement and gap to full-strength Pikafish are reported with uncertainty, conditions, failures, unscored stops per engine and compute; all scored games replay natively; cloud cleanup is verified.
- [ ] #5 Judged move quality (clear blunders per 100 moves, mean win-probability loss) is reported with the MSH-193 definitions; no human-scale rating is claimed.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Approval, 2026-11-19 (John, verbatim reply to 'Approve the MSH-197 reference ladder with caps of about 6 hours and US$15?'): "Yes, approve caps". Approved: reference ladder of classical ElephantEye, Fairy-Stockfish without its neural network and Pikafish at fixed node limits; full-strength Pikafish as judge; caps about 6 hours and US$15; no public servers, no published games.

Ladder frozen 2026-11-19 before any study game: docs/m5/evidence/msh197/ladder.json (players mshogi, eleeye 3.31 UCCI, fairy_sf_14 classical, Pikafish 2026-09-06 at 300/1000/3000 nodes; 60 s + 1 s; 290 pairs; Bradley-Terry fit with pika1000 = 0; Pikafish judge). Runner: per-engine wire formats (UCI, UCCI with resign/draw suffix rules, Fairy rank offset, fixed nodes).
<!-- SECTION:NOTES:END -->
