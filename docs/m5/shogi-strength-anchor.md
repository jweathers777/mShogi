# MSH-191 (Shogi Strength Anchor)

The conditional floodgate-referenced estimate of mShogi is **2765**, with an **approximate 95% interval of 2723–2807**, at 60 seconds plus 1 second per move. It is not an official floodgate rating and not a human rating.

The study scored **304 games in 152 complete opening pairs: 141 wins, 0 draws, 163 losses**. Every game ended in a native conclusion that passed native replay. No game failed, ran out of time or was left unscored. The study stopped at its frozen size rule (about 300 games) after 1.99 campaign hours and US$3.04 conservative compute, within the approved 6-hour and US$15 caps.

**The two opponent families disagree.** Against the Gikou family the estimate is 2772 (2729–2816). Against YaneuraOu with the elmo evaluation at 1,000k nodes, mShogi lost all 24 games, which implies a much lower number. The spread, 374 points, exceeds the 150-point agreement check. The combined number is therefore mostly a Gikou-referenced index; see [Limits](#limits).

## Frozen engine and conditions

Candidate source `342dd1d44633445256124703e44122f1f158f3d9`: main `de5ae4ae` (MSH-188 integrated) plus USI clock controls and the runner's clocked game mode; search and evaluation are unchanged. Linux x86-64 release build, Rust 1.99.1, locked dependencies, `-C target-cpu=x86-64`; SHA-256 in [build.json](evidence/msh191/build.json).

| Condition | Setting |
| --- | --- |
| Time control | 60 s + 1 s Fischer for both sides; controller charges wall time from the written `go` to `bestmove`; 500 ms grace |
| Rules | Native `mshogi.shogi.csa36.v1`; declarations (`bestmove win`), repetition and the 320-move rule are native; opponents set to CSA 27-point entering king and a 320-move draw where supported |
| Engines | One thread each; no pondering; no opponent books. Hash not equal: mShogi fixed 16 MiB, Gikou default, YaneuraOu 256 MiB |
| Openings | MSH-190 catalog (315 openings), in file order, one fresh opening per pair with colours reversed; 152 openings used |
| Execution | Google Cloud Batch, us-central1-a/b, c3-standard-4 (Intel Sapphire Rapids) for every game, eight workers, one game per VM |
| Worker image | `worker@sha256:9f9e39bf0ffc5b908c9e2a15e425ed7cdab88e8739369105ed6c82e0660c1c25` |

## Full-game USI route

MSH-186 found that the external USI route was untimed and restarted the engine every turn. This task adds two pieces before any rated game:

- mShogi's USI interface accepts `btime/wtime/binc/winc/byoyomi` and allocates time with the shared clock policy.
- The runner's `shogi-match` mode keeps each engine process for the whole game, owns both Fischer clocks and admits each reply through the native rules. A reply later than the remaining time plus grace is an unscored clock exhaustion; the accepted profile has no time-loss rule. `shogi-match-replay` re-derives every position, clock command and ledger entry from the native record. Tests cover resignation and false declaration, illegal moves, silence (clock exhaustion), a full mShogi game, and tampered ledgers or scores.

Each opponent passed admission before play: a declaration-eligible diagram (each replied `bestmove win`, admitted natively) and a full game against mShogi with native replay.

## Anchors

Floodgate rating snapshot of 2026-11-18, archived as [floodgate-players-20261118.html.gz](evidence/msh191/floodgate-players-20261118.html.gz). Ratings are fixed in the fit.

| Engine | Floodgate player | Rating | Floodgate W/L | Build and settings |
| --- | --- | ---: | --- | --- |
| Gikou depth 3 | gikou_depth3 | 2637 | 112/138 | Gikou v2.0.2 (`f5773abb`), DepthLimit 3 |
| Gikou depth 5 | gikou_depth5 | 2790 | 103/144 | as above, DepthLimit 5 |
| Gikou depth 7 | gikou_depth7 | 3130 | 107/135 | as above, DepthLimit 7 |
| elmo 1000k | elmo_WCSC27_479_1000k | 3131 | 908/1406 | YaneuraOu v4.79 KPPT (`074f3eb2`) + elmo WCSC27 evaluation, NodesLimit 1,000,000 |

Gikou: GPL v3 source, built with GCC 13.3.0 (image digest in build.json) and relinked statically; evaluation, progress and probability tables from the official v2.0.2 release package (SHA-256 `3bbb667b…`). Options: OwnBook false, Threads 1, FischerMargin 1000, MinThinkingTime 10. YaneuraOu: GPL v3 source, SSE4.2 static build; elmo tables from the author's official `elmo_wcsc27` release on GitHub (`elmo.shogi.zip`, SHA-256 `703de39b…`); Threads 1, Hash 256, BookFile no_book, EnteringKingRule CSARule27, MaxMovesToDraw 320. Every binary and data file hash is checked on each worker.

Floodgate publishes neither versions nor options. The `gikou_depthN` entries are assumed to be Gikou 2 with `DepthLimit N`, and the elmo entry YaneuraOu v4.79 with elmo WCSC27 at 1,000k nodes; these entries may have used opening books. No reproducible rated engine from a third author was found near mShogi's strength: the `komadokun_depth*` ladder has no known source, Bonanza N100 builds and YSS are closed, and NNUE 1000k entries are above 3400.

## Method

Frozen before play in [method.md](evidence/msh191/method.md), with one addendum recorded after bracketing and before the main phase. The model is MSH-185's `anchor_stats.py`, unchanged: one logistic Elo parameter with fixed opponent ratings, all scored games, prior Normal(2900, 500²) chosen from a local calibration scout before play, pair-cluster sandwich interval (routing envelope below 30 pairs). Prior sensitivity: 2766.0 (SD 250), 2765.2 (500), 2765.1 (1000).

## Bracketing and main phase

| Batch | Opponent(s) | Batch W–L | Cumulative estimate | Half-width |
| --- | --- | --- | ---: | ---: |
| b01 | Gikou d5 | 3–5 | 2713.2 | 355.2 |
| b02 | Gikou d3 | 6–2 | 2763.7 | 261.0 |
| b03 | Gikou d5 | 6–2 | 2833.5 | 218.3 |
| b04 | Gikou d5 | 4–4 | 2822.1 | 183.0 |
| b05 | Gikou d5 | 5–3 | 2833.6 | 163.1 |
| b06 | Gikou d5 | 5–3 | 2841.2 | 148.5 |
| b07 | Gikou d5 | 3–5 | 2820.7 | 134.6 |
| b08 | Gikou d5 | 2–6 | 2794.6 | 88.7 |
| m01 | Gikou d5, elmo 1000k | 12–12, 0–24 | 2771.9 | 71.4 |
| m02 | Gikou d5, Gikou d7 | 12–12, 3–21 | 2776.7 | 58.6 |
| m03 | Gikou d5, Gikou d3 | 14–10, 18–6 | 2792.1 | 52.1 |
| m04 | Gikou d5, Gikou d3 | 9–15, 19–5 | 2788.6 | 45.1 |
| m05 | Gikou d5, Gikou d3 | 6–18, 14–10 | 2765.2 | 42.2 |

Bracketing stopped at half-width 88.7 ≤ 100 after eight rounds (0.66 h). Only Gikou depth 5 lay within ±150 of the bracket estimate; elmo 1000k, the only other reproducible family, was 336 points above. The addendum therefore paired Gikou depth 5 with elmo 1000k in m01. After elmo scored 0/24, it was no longer scheduled (its games stay in every fit). m02 used Gikou depth 7, which has the same rating as elmo, as an equal-rating check. Depth 7 then left the 25–75% band and was replaced by the nearer depth 3 for m03–m05. Each decision and its evidence was recorded in the task before the batch it governed. The [rounds record](evidence/msh191/rounds.json) and batch manifests give every selection reason.

## Results

| Opponent | W–D–L | Score | Expected at 2765 | Single-opponent fit |
| --- | --- | ---: | ---: | ---: |
| Gikou d3 (2637) | 57–0–23 | 71.3% | 67.7% | 2795 |
| Gikou d5 (2790) | 81–0–95 | 46.0% | 46.4% | 2763 |
| Gikou d7 (3130) | 3–0–21 | 12.5% | 10.9% | 2797 |
| elmo 1000k (3131) | 0–0–24 | 0.0% | 10.9% | (2398, prior-dominated) |

The three Gikou settings fit consistently: their single-opponent estimates lie within 35 points. elmo 1000k, rated the same as Gikou depth 7, scored 0/24 against 3/24 for depth 7. mShogi scored 66/152 as Sente and 75/152 as Gote. Games lasted 34–229 plies (median 104); 163 ended in checkmate and 141 in resignation.

The elmo games are ordinary: all 24 ended in checkmate of mShogi after 62–119 plies, with no clock exhaustion. After spending its base time mShogi plays near its 1-second increment, about the same pace as elmo's 1,000k-node searches.

## Limits

- **Family disagreement (374 points).** Against Gikou the index is about 2770; against YaneuraOu/elmo at equal floodgate rating mShogi wins nothing. Possible causes include different floodgate conditions for the two entries (hardware, books, time controls, opponent pools), the assumed version and setting mapping, and mShogi weaknesses that elmo exploits more than depth-limited Gikou (for example the extreme King-placement terms noted in MSH-188). These games do not separate the causes. The combined estimate is effectively Gikou-referenced, and one family is a narrow base.
- **Conditional scale.** Floodgate ratings come from other hardware, time controls (mostly longer), books and pools. The depth- and node-limited anchors reduce hardware dependence for the opponents, but not for mShogi, which plays on the clock.
- **Statistical interval only.** The interval excludes anchor rating error (Gikou entries have about 250 floodgate games each), mapping assumptions and systematic family effects.
- No external games were played on floodgate or any public server, and no games are published.

## Compute and cleanup

Thirteen batches (8 bracketing, 5 main): 15.10 conservative VM-hours, US$3.04 conservative compute (eight VMs over each batch's measured interval at US$0.201608 per VM-hour). The campaign took 1.99 hours, below the 4.5-hour and US$10 investigation thresholds. All jobs are verified absent (per-batch cleanup receipts in [batches](evidence/msh191/batches/)). Local development (builds, admission, a 16-game calibration scout under emulation) used no cloud resources and is not part of the fit.

## Reproduction

Build the opponents and image as recorded in build.json; prepare and run batches with `python3 -m tools.strength.shogi_anchor prepare|submit`, and refit all retrieved batches with:

```sh
python3 -m tools.strength.shogi_anchor summarize RUN_DIRS... --output summary.json
```

The collector replays every report with the local native runner before fitting. Raw reports remain under `gs://mshogi-experiments-216675633801/msh191/`.
