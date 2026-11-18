# MSH-194 (Xiangqi Strength Harness)

The first step of the [per-game classical strength program](strength-roadmap.md#per-game-classical-strength-program-john-2026-11-18) for Xiangqi. It gives Xiangqi the measurement tools that Shogi got in MSH-186, MSH-190 and MSH-191. Evidence: [evidence/msh194](evidence/msh194/).

## Engine and protocol changes

Clocked play exposed two faults. Both had to be fixed before any game test could be trusted.

- **Clock support.** Xiangqi UCI accepted only fixed `depth`/`nodes`. It now also accepts `movetime` and Fischer clocks (`wtime btime winc binc`, Red is white), allocated with the shared clock policy as in Chess and Shogi. A timed search keeps its table while each `position` extends the previous one (`XiangqiSearchSession`), and a timed search that ends before depth 1 returns a native-ordered legal fallback move (as Shogi since MSH-189).
- **Request overhead that grew with game length.** One request cost about 2.5 ms more per ply (65 ms at ply 25, so a 5 s game ran out of time near ply 60). Cause: native `submit` copies the whole history, so each replay of the game was O(n²), and one request replayed the history three or four times (adapter copy, session prefix check, search copy, `position` parsing). Fix: a native `Game::fork()` (exact copy of the committed game with a fresh runtime identity, no replay; a native test compares it with an independent replay and checks that reversal tokens do not cross), an event-prefix check in the session, and incremental `position` handling (only the new suffix is admitted). Overhead is now flat at about 3 ms per request.

## Clocked match route

`mshogi-match xiangqi-match CONFIG REPORT` and `xiangqi-match-replay REPORT`, modelled on the Shogi route. The persistent-process host and clock wait are now shared code (`runner::clocked`) used by both games. Native admission (`auto_valid_claims`) decides every reply and every result. Clock exhaustion, native pending adjudication, the game watchdog and failures are unscored stops. Each report replays natively with its clock ledger. Tests: fixture resignation scoring by colour, illegal and null replies, silence (clock exhaustion), watchdog, and a full mShogi game with tampering rejected. `tools/strength/xiangqi_games.py` plays paired openings with any two UCI engines.

## Openings

[`xiangqi-openings-v1.json`](../../tools/strength/data/xiangqi-openings-v1.json): **112 openings** in 12 families (central cannon with screen horses, same- and opposite-direction cannons, palace-corner defences, elephant, pawn, cross-palace and palace-corner cannon openings and others). Each candidate is an original UCI transcription of a conventional trunk plus one two-move development plan per side ([`make_xiangqi_openings.py`](../../tools/strength/make_xiangqi_openings.py), CC0). Native admission decides legality. The balance rule was fixed before screening: admit when the predecessor engine scores the final position (Red to move) within ±150 soldier hundredths at fixed depth 11. Of 224 candidates, 3 are illegal combinations and 109 are unbalanced. In a first screen 22 candidates were illegal because of transcription errors (blocked cannon or chariot paths); those plans were corrected for legality only and all candidates were screened again under the same rule ([screen](evidence/msh194/opening-screen.json)). The first screen was not archived; the corrected plans are: pawn-pawn-reply Red plan 1 `h2e2`→`b2e2`; horse-opening Red plan 1 `h2e2 b0c2`→`b0c2 b2a2` and Black plan 2 `h7e7`→`b7d7`; elephant-opening Red plan 3 `h0h4`→`h0h1`; cross-palace Black plan 1 `b7e7 a9b9`→`c9e7 a9b9`. The engine scores most candidates in Red's favour (candidate median +148); admitted openings range from −54 to +150 (median +51), and no candidate was rejected for favouring Black. "Balanced" therefore means the less Red-favoured part of a set that this engine scores for Red. The pawn-reply and horse-opening families have no admitted opening. The catalog is not a certified equal-evaluation set; games are paired by colour.

## Tactical suite

[`xiangqi-tactics-v1.json`](../../tools/strength/data/xiangqi-tactics-v1.json): **373 positions** (282 development, 91 held-out; split by source game). Positions were mined from 1,120 native-replayed clocked self-play games (224 at 5 s + 0.1 s, 448 at 1 s + 20 ms and 448 at 1 s + 0; faster games give more tactical chances, and 400 of the 427 mined rows come from the 1 s games) where a native reference prover ([`xiangqi_mate_proof.rs`](../../crates/mshogi-runner/examples/support/xiangqi_mate_proof.rs), independent of the search crate) finds a forced checking mate of 3, 5 or 7 plies. At most three positions per game are kept, longest mates first, and equal boards once. The answer set is complete: every first move that forces a win (checkmate or stalemate, both losses in Xiangqi) within that length, from the same prover with a full-width attacker. In 96 of the final positions it contains non-checking or stalemating moves; 5 positions with a one-ply win are excluded. Native rules act on a repetition only from its third occurrence, so lines of at most seven plies cannot be affected. Tests: hand-built mates including facing Generals, memoized against plain recursion, and every checking answer of the 3- and 5-ply rows re-derived without memo.

**Baseline:** 373/373 at 200 ms and 1000 ms, 370/373 at 20 ms, 361/373 at 5 ms ([results](evidence/msh194/)). The positions come mostly from fast mShogi self-play. The suite is a correctness and regression guard; it is **saturated** at playing budgets and cannot measure search or evaluation improvement. A first version scored only checking first moves and marked all 47 non-matching choices as failures, although each was a winning move (some shorter, by quiet or stalemating moves); the complete answer sets correct that.

## Game-test control and depth

**Control: 5 s + 0.1 s per side.** In 224 self-play games at this control, mShogi spends a median 107 ms per move and 91 ms from reply 60 on ([move times](evidence/msh194/move-times.json)). A fresh search (empty table, 10 parallel searches on 10 cores) of 1,696 positions from ply 60 on reaches median depth 9 at 50 ms, **10 at 100 ms** (10th percentile 8) and 12 at 200 ms ([depth](evidence/msh194/)). The kept table in real games adds to this. The control therefore meets the program rule (median late-game depth ≥ 8). All 1,120 harness games ended by native conclusion: no clock exhaustion, failure or pending adjudication ([games](evidence/msh194/games.json)). At 5 s + 0.1 s a game takes about 40 s; 600 pairs on eight c3-standard-4 VMs (four games each) take roughly half an hour.

## Texel export

`xiangqi_strength export REPORTS OUT EVERY` writes one row per report with every N-th position after the opening and the native Red result (½ per half point); unscored games keep no samples. `texel_data.py split --game xiangqi --by-opening` splits by catalog opening. Check: 224 games, every 4th ply, 10,212 positions; split and deduplication succeeded ([manifest](evidence/msh194/texel-split-demo.json)). The corpus of about 500,000 quiet positions is MSH-196 (Xiangqi Evaluation Round) work.

## Proposed opponents and judge (for John's approval)

No game against an external engine runs in this task.

| Role | Engine | Licence | Protocol | Notes |
| --- | --- | --- | --- | --- |
| Judge (blunder screen), strong anchor end | Pikafish (NNUE) | GPL-3.0 | UCI, the Pikafish 0.9 dialect the runner already speaks | Far stronger than any human; useful as a measurement tool, and at very low node limits as a ladder opponent |
| Classical anchor opponent | Fairy-Stockfish with `Use NNUE = false`, `UCI_Variant = xiangqi` | GPL-3.0 | UCI with a different square notation (ranks 1–10) | Needs a small, tested move-notation adapter in the runner |
| Classical anchor opponent | ElephantEye 3.x | GPL/LGPL (to be confirmed from the repository) | UCCI | Needs a UCCI adapter; the classic open-source classical engine |

**Rating reference.** There is no current public rating list for Xiangqi engines comparable to CCRL (Chess) or floodgate (Shogi). Older UCCI league ratings exist but predate the current engines; the [Fairy-Stockfish wiki](https://github.com/fairy-stockfish/Fairy-Stockfish/wiki/Playing-strength) gives only relative estimates (about +100 over ElephantEye 3.31 with classical evaluation), and the [Pikafish FAQ](https://github.com/official-pikafish/Pikafish/wiki/Pikafish-FAQ) states that a human-scale rating is not measurable. Licences and versions are to be confirmed from each repository before admission. An MSH-197 (Xiangqi Strength Anchor) result would therefore be a **relative ladder** (mShogi against classical ElephantEye and classical Fairy-Stockfish at stated limits, and Pikafish at node limits), not a rating on a human or published scale. This limit must be stated in the report.

**Proposed caps for MSH-197**, to be approved: about 6 hours and US$15, c3-standard-4 like MSH-191, 60 s + 1 s. Clock exhaustion is unscored (the profile has no time-loss rule), so external studies must report unscored stops per engine.

## Limits

- The tactical suite is saturated (see above). The discriminating tactical measure for Xiangqi will be the judged blunder suite of MSH-198 (Xiangqi Blunder Screen).
- **Observed weakness, descriptive:** 47 of 1,120 games (4%) were lost by the perpetual-chase rule. The private search scores chase cycles as draws (recorded approximation of MSH-181), so mShogi can walk into a native chase loss. This is a candidate for MSH-195 (Xiangqi Selectivity Round) or a separate task; no cause beyond the recorded approximation is claimed.
- The opening balance screen and depth figures use mShogi's own evaluation; Red's apparent advantage in many candidates may partly be an evaluation bias.
- Measurements ran on macOS arm64; cloud machines differ in speed.
