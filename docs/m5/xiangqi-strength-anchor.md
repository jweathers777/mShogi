# MSH-197 (Xiangqi Strength Anchor)

The measurement step of the [per-game classical strength program](strength-roadmap.md#per-game-classical-strength-program-john-2026-11-18) for Xiangqi, by the method for games without a public rating list (John, 2026-11-19): a fixed reference ladder (relative Elo) and move quality judged by a much stronger engine, reported separately. **No rating on a human or published scale is claimed.** Evidence: [evidence/msh197](evidence/msh197/).

**Result: mShogi (main `e15bdd63`, the MSH-196 engine) is +18 Elo (approx. 95% −35 to +66) relative to Pikafish limited to 1,000 nodes per move, about 110 Elo below ElephantEye 3.31 and about 270 below Fairy-Stockfish 14 with classical evaluation, at 60 s + 1 s. Its mean win-probability loss per move (0.0130) is between Pikafish at 1,000 and at 300 nodes.**

## Approval and caps

John approved the ladder and caps on 2026-11-19 ("Yes, approve caps": about 6 hours and US$15) and the engine downloads (official sources and the Pikafish release). Used: ladder 43 minutes and US$1.14, judging 78 minutes and US$2.10 (conservative), **total US$3.24**, about 2 hours of the 6-hour window; all jobs verified absent. No public server was used and no game was published.

## Frozen ladder

Frozen before any study game in [ladder.json](evidence/msh197/ladder.json); players, versions and binary hashes in [ladder-run.json](evidence/msh197/ladder-run.json).

| Player | Engine | Settings |
| --- | --- | --- |
| mshogi | mShogi `e15bdd63` | UCI, default |
| eleeye | ElephantEye 3.31 (`xqbase/eleeye` 61c2f8b4, LGPL-2.1), static build | UCCI, book off, 64 MB hash |
| fairy | Fairy-Stockfish 14 (`fairy_sf_14`, GPL-3.0), large boards, static build | `UCI_Variant xiangqi`, `Use NNUE false`, 1 thread, 64 MB |
| pika300 / pika1000 / pika3000 | Pikafish 2026-09-06 (GPL-3.0), static build from the tagged source, release network | 300 / 1,000 / 3,000 nodes per move, 1 thread, 64 MB |

All games: 60 s + 1 s Fischer per side on the controller's clock (node-limited players are also charged), xiangqi-openings-v1 with colours paired, eight c3-standard-4 VMs with four games each. The runner was extended for these engines: per-engine wire formats (UCCI handshake, options and `go time … increment …`; UCCI `resign` suffix as a native resignation and `draw` suffix ignored; Fairy-Stockfish ranks 1–10 translated; fixed `go nodes`). Native rules admit every reply and decide every result; every report replays natively.

The rungs were chosen from a local calibration pilot at 10 s + 0.1 s (8 pairs each), in which mShogi scored 72% against Pikafish at 300 nodes, 25% at 1,000 and 0% at 3,000 and 10,000, and almost nothing against ElephantEye and Fairy-Stockfish.

## Results

290 pairs, 580 games; 578 scored. Two games (both ElephantEye against Fairy-Stockfish) are unscored: ElephantEye answered `nobestmove` in positions where native rules allowed moves. No clock exhaustion and no other failure.

| mShogi against | Score |
| --- | ---: |
| Pikafish 300 nodes | 60.0 / 80 |
| Pikafish 1,000 nodes | 39.0 / 80 |
| Pikafish 3,000 nodes | 11.5 / 60 |
| ElephantEye 3.31 | 22.0 / 60 |
| Fairy-Stockfish classical | 11.0 / 60 |

Bradley-Terry maximum likelihood over all scored games (draws as half points), Pikafish 1,000 nodes fixed at 0, 500 pair-cluster bootstrap samples ([fit](evidence/msh197/ladder-fit.json), [table](evidence/msh197/ladder-summary.json)):

| Player | Elo | approx. 95% |
| --- | ---: | --- |
| Pikafish 3,000 nodes | +349 | 280 to 431 |
| Fairy-Stockfish classical | +290 | 208 to 385 |
| ElephantEye 3.31 | +129 | 67 to 193 |
| **mShogi** | **+18** | **−35 to +66** |
| Pikafish 1,000 nodes | 0 | reference |
| Pikafish 300 nodes | −203 | −281 to −137 |

Historical tie (weak, older conditions): the Fairy-Stockfish wiki estimates classical Fairy-Stockfish about +100 over ElephantEye 3.31; this ladder gives about +161.

## Judged move quality

Full-strength Pikafish (1,000,000 nodes, one thread) judged the position before and after every move of the 578 scored games (65,011 positions; 454 gave no score line and are skipped) with the MSH-193 definitions ([results](evidence/msh197/move-quality.json), [run](evidence/msh197/judge-run.json)).

| Player | Moves | Clear blunders / 100 moves | Mean win-probability loss |
| --- | ---: | ---: | ---: |
| Pikafish 3,000 nodes | 10,822 | 0.15 | 0.0080 |
| Fairy-Stockfish classical | 8,139 | 0.23 | 0.0093 |
| ElephantEye 3.31 | 10,373 | 0.12 | 0.0105 |
| Pikafish 1,000 nodes | 11,821 | 0.29 | 0.0123 |
| **mShogi** | 18,856 | 0.14 | 0.0130 |
| Pikafish 300 nodes | 5,901 | 0.39 | 0.0179 |

The mean loss orders the players almost as the ladder does (mShogi and Pikafish at 1,000 nodes are swapped, within their Elo uncertainty). Clear blunders as defined are rare for every player here and do not separate them; the Pikafish centipawn scale differs from the Shogi judge's, so these rates are not comparable with MSH-193's 1.43 per 100 moves.

## Interpretation

- mShogi now plays at about the level of Pikafish limited to 1,000 nodes, and clearly below the two established classical engines. Its weakness is gradual (mean loss), not frequent clear blunders.
- The ladder is relative: it is not a human-scale rating, and the Pikafish rungs are node-limited rather than time-limited, so their place depends on the time control.

## Next

MSH-198 (Xiangqi Blunder Screen) can reuse these judged positions: its own work is the mechanism classes, the fix rule and the blunder suite.

## Limits

The ladder holds at one time control and with one opening catalog. Node-limited rungs do not scale with time like the others. The judge is far stronger than every player but is an engine of one family; its centipawn-to-probability conversion is a convention. Two ElephantEye games are unscored. The fit has one reference point fixed by choice.
