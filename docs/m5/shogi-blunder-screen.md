# MSH-193 (Shogi Blunder Screen)

John accepted the classical Shogi engine (main `ce2f79bd`, MSH-191 index about 2765) as the baseline for later learned-evaluation work. This screen documents its clear blunders in the 304 MSH-191 games and applies a fix rule fixed before results. **Result: no single mechanism explains 30% or more of the clear blunders, and a much longer search avoids only 28% of them, so the rule does not trigger a fix round.** The 226 positions are kept as a reusable blunder suite. Evidence: [evidence/msh193](evidence/msh193/).

## Method (fixed before results)

- **Judge:** YaneuraOu v4.79 + elmo WCSC27, the MSH-191 admitted build, at 3,000,000 nodes per position, one thread, no book; used only to score positions, never as a rules authority. All 31,535 positions after the opening prefixes were judged on Google Cloud Batch (eight c3-standard-4 VMs, four judges each): 1.38 hours, US$2.23 conservative, cleanup verified.
- **Win probability for mShogi:** p = 1/(1+exp(−cp/600)); mate scores map to 1 or 0.
- **Clear blunder:** an mShogi move with p_before ≥ 0.30 and p_after ≤ p_before − 0.30.
- **Mechanism (one per blunder, by priority), as implemented:** time trouble (< 2 s on mShogi's clock); allowed a short mate (judge mate in ≤ 7 plies); the judge's refutation captures any piece; the refutation is a drop; the refutation lands next to mShogi's King; other. *Deviation from the frozen wording:* the task text said "capture of an undefended or under-defended piece" and "King-safety collapse"; the code counts any capture and uses "refutation lands next to the King" as the King-safety proxy. A stricter capture class would move some blunders to "other", which has no general fix, so the decision below is unchanged.
- **Fix rule:** one fix round only if one mechanism explains ≥ 30% and maps to a general change.

## Results

mShogi made **226 clear blunders in 15,805 judged moves (1.43 per 100 moves)**, in 116 of 304 games.

| Games | Clear blunders |
| --- | ---: |
| Losses to Gikou (139) | 115 |
| Wins against Gikou (141) | 110 |
| Losses to elmo 1000k (24) | 1 |

The elmo games contain almost no clear blunders: by the judge's view, mShogi fell below 30% winning chances gradually and was outplayed, not tricked. (The judge is the same evaluation family as elmo, which may favor elmo's view.) Against Gikou both sides blunder, and many games swing.

| Mechanism | Blunders | Share | Changed at 10 s | Avoided at 10 s |
| --- | ---: | ---: | ---: | ---: |
| Time trouble | 64 | 28.3% | 31 | 23 |
| Refuted by a capture | 61 | 27.0% | 18 | 14 |
| Refuted by a drop | 53 | 23.5% | 23 | 17 |
| Other | 43 | 19.0% | 13 | 9 |
| Allowed short mate | 4 | 1.8% | 1 | 0 |
| King-zone refutation | 1 | 0.4% | 0 | 0 |
| **All** | **226** | | **86** | **63 (28%)** |

"Changed at 10 s" reruns the baseline at 10 seconds on the position (fresh process, empty table, no history), against about 1–2 seconds in the game. "Avoided" means the judge (same settings, `position sfen … moves <new move>`) no longer rates the new move a clear blunder; provenance in [recheck.py](evidence/msh193/recheck.py).

## Interpretation and decision

- **The fix rule is not triggered.** The largest mechanism, time trouble, is 28.3%. Merging categories after seeing the results would break the frozen rule, so no fix round was run.
- **Modestly more time does not resolve most blunders.** A 10-second rerun (about one or two plies deeper, without game history or a warm table) avoids only 28% of all clear blunders and 36% of time-trouble blunders. This agrees with MSH-192, where reallocating the clock gave no gain. It cannot separate evaluation blind spots from deeper search horizons.
- **The blunders are spread over tactical refutations by capture and by drop** that mShogi does not avoid even with this extra time. No single rule-level gap stands out. Whether evaluation quality (the learned-evaluation direction John chose, MSH-55) or deeper search would help most is not determined by this screen.
- No gross single weakness was found. The baseline is documented as an engine that blunders about once per 70 moves at this control and is outplayed gradually by much stronger opponents.

## Blunder suite

[`shogi-blunders-v1.json`](../../tools/strength/data/shogi-blunders-v1.json) has the 226 positions with the played blunder, the judge's reference move and refutation, the mechanism, mShogi's clock, and a by-game held-out split (164 development, 62 held-out; the two games of one opening pair can fall on different sides). Baseline at 1000 ms per position: avoids the recorded blunder in 73 of 226 positions (50 of 164 development, 23 of 62 held-out) and plays the reference move in 51. Avoiding the played move does not prove a good move; the reference is one good move, not the only one.

## Limits

The judge is far stronger than mShogi but not perfect, and it shares an evaluation family with one opponent. Win-probability conversion and thresholds are conventions. The mechanism classes are heuristic, based on the judge's first refuting move. Blunders were judged in diagram positions without game history (repetition state is ignored).
