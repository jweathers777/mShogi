# Per-game classical strength program

John set this program on 2026-11-18. It applies to every game. Adjust a step
only for a real difference (available opponents, known techniques, game
rules), and record the adjustment and its reason in the game's task. Results
of finished steps are in their Backlog tasks; Git history keeps the earlier
reports.

## Target

Competent play with no obvious blunders, as far as classical (non-neural)
techniques reach with bounded effort. The result is the classical baseline for
later learned evaluation (MSH-55, MSH-135). Beating the strongest engines is
not a goal. Do not spend large time or money for small gains. Prefer
techniques that transfer between games; do not build heavy game-specific
machinery (for example Shogi piece-pair tables). Neural-network evaluation is
out of scope until John approves it.

## Steps for one game

Each step is one bounded task. Skip a step only with a recorded reason.

1. **Harness.** At least 100 balanced openings; a tactical suite of at least
   200 positions with independently established answers and a held-out part; a
   clocked, persistent-process match route with game admission of every
   reply; a game-test control at which the median late-game depth is at least
   8; a self-play export for Texel data.
2. **Speed.** Profile where search time goes (generation, make/unmake,
   legality and attack tests) and remove the largest costs, especially for the
   game's unusual pieces. Write a short design note of the method.
3. **Selectivity round.** Adopt the shared-core techniques that the game can
   use: quiescence check stage, full SEE, game-scaled margins (`MARGIN_UNIT`),
   move ordering, and the shared df-pn mate search where mates are deep or
   forcing. Each technique gets a recorded outcome.
4. **Evaluation round.** Texel-tune the handwritten terms (material,
   placement, mobility, King safety and game-specific terms) with
   `crates/mshogi-search/src/texel.rs` on about 500,000 or more quiet
   self-play positions, with a split by game or opening and a held-out loss.
5. **Strength measurement** (needs John's approval with time and dollar caps).
   Where a public rating list exists (CCRL, floodgate), one approximate rating
   against independent rated opponents, run in the cloud. Otherwise see
   "Measurement without a rating list". No public servers and no published
   games. Never invent a rating.
6. **Blunder screen.** Judge mShogi's moves with a much stronger engine (a
   measurement tool only). Fix the blunder rule, the classes and the fix rule
   (one mechanism of at least 30% that maps to a general change) before looking
   at the results. Keep the blunder positions as a reusable suite.
7. **Stop or continue.** John decides whether the game is at its baseline. A
   further round needs a specific weakness with evidence.

## Game tests

- Freeze the predecessor at the start of the task
  (`python3 -m tools.strength.cli build-engine`). Later tasks use the last
  accepted engine.
- Before the work, record the baseline, the intended benefit, the comparison
  method, the caps and the decision checkpoint.
- Use the sequential pentanomial test of the harness
  ([strength-harness.md](strength-harness.md)). Choose H1 before the test
  starts and do not change it: about 15 Elo for search changes, 10 for
  evaluation changes. The maximum (default 600 pairs) must be reachable by a
  gain of the expected size. A test that reaches its maximum without a
  decision is "no decision", not a rejection.
- Test coherent sets of standard techniques together. Test one part alone only
  when the set result is weak or there is a specific reason to doubt it.
- Choose a time control at which the change can act. A technique that starts
  at depth 7 needs a search that often reaches that depth.
- A change that keeps the search exactly identical (equal node counts and best
  moves at fixed depth) is not a strength change and needs no game test.
  Record the fixed-depth comparison instead.
- Keep a change only with evidence. Report null results and revert them.
- Pruning and evaluation inside the tree may be approximate (design A-03). Do
  not add work at each node to make a pruned score exact; the rules
  decide the real game.
- A shared-core change must keep the tests of every game on the core passing.
  Switch a technique off for a game where it is clearly worse.
- Fast predecessor game tests have standing authorization. A campaign of
  several hours or a rating study against external engines needs approval.
  Long runs go to the cloud (`tools/experiments/gcp.py`); see AGENTS.md.

## Measurement without a rating list

John, 2026-11-19. Report three measures separately:

- **Reference ladder (relative Elo).** Freeze independent opponents over a
  range of strengths (exact versions; fixed node limits where possible). Play
  a round robin and fit all ratings together (Ordo or BayesElo) with one rung
  fixed at 0. Report mShogi's place and its gap to the strongest engine.
- **Judged move quality.** With a much stronger engine as judge, report clear
  blunders per 100 moves and mean win-probability loss per move, with
  p = 1/(1+exp(-cp/600)); a clear blunder has p_before >= 0.30 and
  p_after <= p_before - 0.30.
- **Method check (optional, small).** Where a true anchor exists (Chess,
  Shogi), check that the ladder and quality measures place those engines
  consistently with their anchors.

**Games after Xiangqi** (Tori, Chu, Tenjiku): no external opponents or judges.
The evidence is frozen-predecessor tests, tactical suites, a scaling curve
(strength against depth or time) and blunder rates judged by a much deeper
mShogi search. A same-family judge is weaker evidence; say so in the task.
Steps 5 and 6 change accordingly.

## Status by game

| Game | State | Tasks |
| --- | --- | --- |
| Chess | Baseline. CCRL Blitz-referenced estimate about 2750 (MSH-185). | Final criteria: MSH-49 |
| Standard Shogi | Baseline accepted 2026-11-18. Floodgate-referenced index about 2765 (MSH-191); blunder screen MSH-193. | Final criteria: MSH-50 |
| Xiangqi | Baseline accepted 2026-11-19. Ladder and blunder screen: MSH-197, MSH-198. | Final criteria: MSH-54 |
| Tori Shogi | Baseline accepted 2026-11-21 (MSH-207): 2x time about +260 to +350 Elo; 0.49 clear blunders per 100 moves (same-family judge). | Final criteria: MSH-51 |
| Chu Shogi | Baseline accepted 2026-11-25 (MSH-52.3): 2x time about +370 to +600 Elo; 0 clear blunders in 512 sampled moves (same-family judge). | Final criteria: MSH-52 |
| Tenjiku Shogi | Evaluation complete for both profiles. The shared Texel fit passed the tactical screen and each profile’s clocked test against the accepted speed-step engine. Strength measurement and the blunder screen await John’s approval. | MSH-214 (harness), MSH-202 (speed), MSH-53.1 (selectivity), MSH-53.2 (evaluation), MSH-53 (final criteria) |

## Clock allocation of the experiment runner

In a clocked experiment, every game uses one policy. For each request, the
runner takes the normal share of the engine's remaining clock (the soft budget
of the shared allocation for 30 moves, at most 60 s). It subtracts a transport
reserve and gives the engine the result as a fixed move time. The watchdog of a
configuration must not be shorter than the largest share that its clocks permit.

John decided on this policy on 2026-11-25. Earlier, the runner also had a `hard`
policy (three times this share). Clocked runs of games other than Tori and Chu
used it, for example all clocked Shogi runs of this runner. Results from `hard`
runs are not comparable with results under the current policy. Tori and Chu runs
already used the normal share.
