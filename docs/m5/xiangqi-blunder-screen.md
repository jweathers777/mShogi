# MSH-198 (Xiangqi Blunder Screen)

The blunder-screen step of the [per-game classical strength program](strength-roadmap.md#per-game-classical-strength-program-john-2026-11-18) for Xiangqi, as MSH-193 (Shogi Blunder Screen). **Result: in 18,856 judged moves, mShogi made 26 clear blunders and 161 mistakes; for 85% of these errors the judge's first reply is neither a capture nor a short mate, so no class that maps to a general change reaches the pre-fixed 30% and no fix round runs. A 10-second rerun avoids 42% of them.** Evidence: [evidence/msh198](evidence/msh198/).

## Method (fixed before results)

Recorded in the task plan before any error was classified. The clear-blunder count (26) was already visible in MSH-197; nothing else was.

- **Data:** mShogi's moves in the 578 scored MSH-197 (Xiangqi Strength Anchor) ladder games at 60 s + 1 s, judged by full-strength Pikafish 2026-09-06 (1,000,000 nodes, one thread), a measurement tool only.
- **Definitions (MSH-193):** p = 1/(1+exp(−cp/600)), mate scores 1 or 0, mover's view. Clear blunder: p_before ≥ 0.30 and p_after ≤ p_before − 0.30. **Mistake** (added because clear blunders are rare and MSH-197 showed mShogi's gap is gradual loss): p_before ≥ 0.20 and loss ≥ 0.15, not a clear blunder.
- **Classes**, one per error by priority: time trouble (< 2 s on mShogi's clock); repetition rule (game lost by a native repetition fault within 20 plies); allowed short mate (opponent mates in ≤ 7 plies by the judge); judge's refutation captures a Chariot, Horse or Cannon; captures an Advisor, Elephant or Soldier; other.
- **Fix rule:** one fix round only if one class explains ≥ 30% of clear blunders plus mistakes and maps to a general change.
- **Rerun check:** mShogi at 10 s per position (fresh process; the plan said "no history", but the game history was sent as moves so that repetition rules apply, a deliberate change); a changed move is re-judged with the same judge settings; avoided if it is no longer an error by the same definitions ([script](evidence/msh198/recheck.py)).

## Results

| Class | Clear blunders | Mistakes | Share | Changed at 10 s | Avoided at 10 s |
| --- | ---: | ---: | ---: | ---: | ---: |
| Other (no capture or short mate in the judge's first reply) | 21 | 138 | 85.0% | 75 | 64 |
| Refutation captures Chariot/Horse/Cannon | 1 | 9 | 5.3% | 6 | 5 |
| Refutation captures Advisor/Elephant/Soldier | 2 | 8 | 5.3% | 5 | 3 |
| Allowed short mate | 2 | 6 | 4.3% | 7 | 6 |
| Time trouble | 0 | 0 | 0 | — | — |
| Repetition rule | 0 | 0 | 0 | — | — |
| **All** | **26** | **161** | | **93** | **78 (42%)** |

Rates: 0.14 clear blunders and 0.85 mistakes per 100 moves ([errors](evidence/msh198/errors.json), [rerun](evidence/msh198/recheck.json)). Descriptively, the errors fall at a median ply of 72, mostly with 5–8 Chariots, Horses and Cannons still on the board (92 of 187), and most when mShogi was already behind (106 of 187 with p_before < 0.4).

## Interpretation and decision

- **The fix rule is not triggered.** The largest class is the residual "other" class, which maps to no general change; no other class reaches 30%. The decision is the same with clear blunders only (other: 21 of 26). No fix round was run.
- **No clock or rule problem remains:** no error came from time trouble or from the repetition rules (MSH-195's chase-aware scoring holds in these games).
- **Deeper search helps:** 42% of errors (13 of 26 clear blunders) are avoided with 10 s per move, against a median of 1.5 s (90th percentile 3.4 s) in the games. Consistent with MSH-197, mShogi mostly loses ground without an immediate tactical refutation; the cause of these errors (search depth, evaluation, or tactics starting with a quiet move) is not determined here.
- **Recommendation for John's stop-or-continue decision:** Xiangqi now has a classical baseline with no gross single weakness. Further gains would come from general search speed or depth and from evaluation quality (the learned-evaluation direction, MSH-55), not from a specific fix.

## Blunder suite

[`xiangqi-blunders-v1.json`](../../tools/strength/data/xiangqi-blunders-v1.json): the 187 positions (as move lists from the standard root) with the played move, the judge's move and refutation, kind, class and clock; split by game: 132 development, 55 held-out. Baseline at 10 s: avoids 93 played moves; 78 avoided errors by the judge. The two games of one opening pair can fall on different sides of the split (9 pairs here). Built with `xiangqi_blunders suite` from the classified errors.

Rerun environment: mShogi main `269841f7` (the MSH-196 engine), macOS arm64 release build, four positions in parallel; judge: Pikafish 2026-09-06 macOS release binary with the same network, 1,000,000 nodes, one thread.

## Limits

The judge is far stronger than mShogi but is one engine; the probability conversion is a convention, and the Pikafish centipawn scale differs from MSH-193's judge, so rates are not comparable across games. Classes are heuristic, based on the judge's first refuting move. The rerun uses no warm table and a fixed time, unlike game play. Games are from one control and one opening catalog.
