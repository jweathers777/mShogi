# MSH-161 (Competitive Gameplay Diagnosis): independent review, 2026-11-13

Status: additive independent review. It replaces no other analysis. The existing synthesis, challenge, category tables and integration review stay unchanged and stay accepted as recorded. No second reviewer has checked this review.

Reviewer: Claude Fable 5.2 (`claude-fable-5-2`), started from `../independent-review-handoff.md`. Source revision: `f542d3c0` on `codex/rewrite`. Environment: macOS, local evidence mirror `../local-cache/`. No engine, search, game, retrieval or cloud job ran. No production file changed. The MSH-161 Backlog record was not changed.

Order of work: frozen method and owner request, then raw screening artifacts and native transcripts, then independent conclusions, then the existing synthesis and challenge.

## Main conclusion

The existing synthesis is correct in what it says, but it omits three measurements that change the priorities:

- **Both engines make the same types of error at similar rates.** The category tables describe play at this strength level, not a weakness specific to mShogi.
- **The measurable difference is retention of an advantage.** mShogi gets ahead more often than Oxidation but keeps the advantage less often, most clearly in endgames.
- **The native score includes 12 draws from reference-lost positions.** The controller claimed these draws for Oxidation while Oxidation was winning.

## Observations

### 1. Native results (authoritative)

- Total: 82 wins / 58 draws / 60 losses. Prepared: 67/55/58. Unaided: 15/3/2.
- All 142 decisive games ended in checkmate.
- Prepared pairs: 12 with two wins, 9 with two losses, 10 with two draws.

### 2. AutoClaim changes the result in one direction

- In 42 of 47 claim draws, mShogi made the last move and the controller then claimed for Oxidation. Oxidation received no query. (Checked in the transcript and match record of `main-06-03` game 1.)
- In 12 of these (all prepared), mShogi was reference-lost (E about 0). Oxidation's own score was +174 cp to mate in 9. mShogi's own score was 0, so it steered to the claim. Games: `main-02-03/g0`, `main-02-07/g0`, `main-03-04/g1`, `main-03-05/g0`, `main-03-06/g1`, `main-04-04/g1`, `main-06-03/g1`, `main-07-04/g1`, `main-08-07/g0`, `main-09-08/g0`, `main-10-01/g0`, `main-10-03/g0`.
- There is no opposite case. No claim ended a game while mShogi was reference-winning.
- mShogi receives `setoption name ClaimPolicy value auto`. Oxidation has no such option.
- If these 12 games were losses, the prepared score would be 49.2%, not 52.5%. This is a bound, not a counterfactual result. Native results stay authoritative.

### 3. Symmetric all-move comparison (500k screening, prepared games)

Each cell shows mShogi / Oxidation.

| Measure | Opening | Middlegame | Endgame |
|---|---|---|---|
| Mean E loss per move (root E .1–.9) | .043 / .051 | .057 / .075 | .023 / .025 |
| Level-to-lost collapses per 1000 level moves | 37 / 53 | 67 / 110 | 31 / 35 |
| Give-backs per 1000 moves at ≥ +300 cp | (too few moves) | 20 / 22 | 10.0 / 5.7 |

- A collapse is root E .30–.70 followed by played E < .15. A give-back is root ≥ +300 cp followed by played ≤ +80 cp.
- mShogi collapses from level positions less often than Oxidation in each phase.
- The endgame give-back rate is the only rate where mShogi is clearly worse: 20 events in 18 games against 8 events in 8 games.

### 4. Conversion of an advantage (prepared games)

"Ahead" means four successive roots at or above the threshold (reference cp, 500k screening).

| Threshold | mShogi ahead: games / won / drawn / lost | Oxidation ahead: games / won / drawn / lost |
|---|---|---|
| +150 cp | 110 / 67 / 28 / 15 | 79 / 58 / 16 / 5 |
| +300 cp | 88 / 67 / 12 / 9 | 68 / 58 / 10 / 0 |
| +500 cp | 74 / 67 / 6 / 1 | 62 / 58 / 4 / 0 |

- mShogi lost 9 games after it was at +300 cp. Oxidation lost none.
- Oxidation's unconverted draws include the controller claims in item 2.
- By 30 plies after the prefix, mShogi is at ≥ +100 cp in 46% of prepared games and at ≤ −100 cp in 21%. The lead is lost later.

### 5. How the losses occur

- In 56 of 58 prepared losses, one mShogi move takes the reference from level (median −20 cp) to lost (median −320 cp). Phases: 32 middlegame, 21 endgame, 3 opening.
- In Oxidation's losses the same pattern occurs, but only 11 of 64 are in the endgame.
- In about 80% of collapses, neither engine's own score moves by 100 cp in the next two moves. The consequence is beyond the horizon of both engines.
- The 22 endgame positions that I read show a recurring choice: mShogi declines a drawing simplification or a checking sequence while its own score is already −110 to −275 in a reference-level position. Examples: `main-04-08/g1` ply 101 (Ne4+ against Nxe6), `main-04-02/g1` ply 143 (Rb3 against Nxc6), `main-10-08/g1` ply 123 (Be7 against Bxc3), `main-02-00/g0` ply 94 (Re7 against R7f6+).
- At the last give-back in 16 of 24 unconverted +300 games (21 prepared, 3 unaided), mShogi's own score stayed at or above +180 while the reference fell to about 0.

### 6. Mechanical profile of confirmed errors

- Pawn pushes next to the own king are about 4 times more frequent in collapses than in all moves (8.6% against 2.2%). Declined captures are about 3 times more frequent.
- Oxidation's proportions are almost the same (8.9% and 9.4%). I found no error type specific to mShogi at this level.

### 7. Search work (native transcripts)

- mShogi's median final node count per five-second move is 0.57–1.35 million. Oxidation's is 3.5–4.6 million.
- Median reported depth is 8–9 for mShogi and 9–12 for Oxidation (90th percentile in endgames: 13 against 26). Reported depth is not comparable between engines; the node counts are.

### 8. Unaided starts

- All ten mShogi-Black games share at least 22 plies. mShogi-White games have four distinct 12-ply lines.
- Oxidation loses all ten games from one White line, and as Black often plays ...Ng4 and ...Nxf2 or ...Bd6.
- The 15/3/2 result measures a few Oxidation opening lines. mShogi always plays knights first (Nc3/Nf3, ...Nc6/...e6/...Bb4), with small supported losses of .06–.07 at plies 3 and 5.

## Possible mechanisms (not findings)

- **Endgame give-backs:** the evaluation may not recognize drawn material balances, the search may be too shallow for long king and pawn sequences, or the claim-aware draw score may distort choices near repetitions.
- **Single-move collapses:** these may be ordinary horizon effects at this node count, because the faster opponent has more of them.
- **Declined drawing lines:** a pessimistic own score in level endgames may remove the motive to simplify. In level positions with material ≤ 14, 44% of mShogi's own scores are ≥ 100 cp from zero; Oxidation's figure is 48%.

## Discriminating investigations (saved evidence first)

1. Compare own score with reference at all endgame roots by material signature. Find which signatures have the largest disagreement for mShogi but not for Oxidation.
2. For the 20 endgame give-backs, measure how many plies the reference line needs to show the consequence, and compare with mShogi's reported selective depth.
3. Examine the 42 claim endings: did mShogi's move before the repetition lose reference value against a non-repeating move?
4. Calculate pair-clustered uncertainty for the give-back and conversion differences before ranking them.

## Challenges to reference and inference

- Oxidation is not shown to be stronger. On Stockfish screening it errs more per move, and it does not avoid repetitions when winning. The study does not show how mShogi fails against a stronger engine.
- Stockfish E saturates. Many "major" events between +100 and +250 cp are small centipawn changes, so the count of 419 major events overstates severity. Centipawn thresholds give a more conservative picture.
- The existing category counts have no opponent baseline. With a baseline, they do not separate mShogi from Oxidation.
- My rates use the uniform 500k tier. Of 190 mShogi collapses (all games), 146 have 20M support, 43 are unresolved and none is refuted. Screening-only rows are not confirmed.
- Prepared families are purposive. Unaided games are correlated. I did not pool them.
- The rates in items 3 and 4 have no pair-clustered uncertainty. They are descriptive.

## Comparison with the existing synthesis

- **Agreement:** native results, limits on E, separation of unaided games, the examples (h4 at `main-01-00/g0` p48 and Qxh7 at `main-08-01/g0` p28 are also in my decisive-drop list), and that no mechanism is proven.
- **Difference 1:** the synthesis says the evidence gives no conversion rate. The all-move screening does give symmetric conversion counts, and they show the clearest difference between the engines.
- **Difference 2:** the synthesis puts exchange order and quiet defense first. Those behaviors are real, but Oxidation shows them at equal or higher rates. Endgame retention of an advantage is the better first subject.
- **Difference 3:** the synthesis treats claims as neutral censoring. The censoring is one-directional and is worth about 6 points of 180.

## Unresolved questions

- Would Oxidation have won the 12 claimed games? Its PVs contain repetitions at +26, so this is not certain.
- Is the endgame give-back difference stable? It rests on 18 games against 8.
- How much of the node-count difference is an execution cost and how much is the search design?
- Are the declined simplifications caused by evaluation or by search? The saved evidence cannot separate them.

## Evidence paths

Paths are relative to `docs/m5/evidence/msh161/`.

- Method and inputs: `main-method.md`, `owner-request.txt`, `cloud-primary/independent-review-handoff.md`, `sources/main-selection.json`.
- All-move screening: `cloud-primary/local-cache/msh161-primary-v1-batch-NN/jobs/*/attempts/*/artifacts/screen-v1.json`, 100 files, indexed by `cloud-primary/final-analysis/played-coverage.json.gz`. The mirror is Git-ignored.
- Native games and transcripts: `.../artifacts/pair/report.json.gz`, listed in `cloud-primary/primary-cumulative.json` under `native_games` and `optional_claim_draws`.
- 20M statuses and margins: `primary-cumulative.json` under `normalized_selected_events`. Phases: `cloud-primary/final-analysis/phase-summary.json.gz`.

## Reproduction

The scripts in `scripts/` are the exact files used. They read saved evidence only. Run them from `cloud-primary/` with a scratch directory `W` outside the repository. They need the `chess` Python package (version 1.11.2 used) in a directory `LIB`.

- `python3 scripts/build.py W/data.pkl`, then `scripts/build2.py W/data.pkl`: per-move table (screening, 20M status, phase, own depth/score). Both reported zero mismatches against native histories.
- `a1.py`, `a2.py` (run `a2.py` before the later scripts; it adds game fields): native tables, per-move loss, trajectory shapes.
- `a3.py`, `a4.py`, `a7.py`: draws, claims, depth and nodes.
- `a9.py W/data.pkl LIB W/ev.pkl`, then `a10.py W/ev.pkl`, `a11.py W/data.pkl W/ev.pkl`: supported candidate events and decisive drops.
- `a12.py` (adds the conversion fields), `a13.py`, `a15.py`: conversion, collapse and give-back rates, own-score comparison.
- `a14.py W/data.pkl LIB lossdrop LO HI [STEP]`: positions and lines for decisive drops in a material range.
- `a16.py W/data.pkl LIB`, `a17.py W/data.pkl LIB`: unaided starts and mechanical error profile.

Verification level: Level 1, analysis of saved evidence only. The outputs were not saved as files; the numbers above come from one run of these scripts at the stated revision.

This review recommends no engineering and authorizes none.
