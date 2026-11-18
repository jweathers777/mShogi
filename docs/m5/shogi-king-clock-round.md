# MSH-192 (Shogi King and Clock Round)

MSH-191 (Shogi Strength Anchor) suggested two possible weaknesses: extreme fitted King-placement weights (MSH-188) and clock use that spends the base time early. This round tested a fix for both as one set against the frozen predecessor, main `ce2f79bd`. **Result: no measurable gain; both changes were rejected and reverted.** Evidence: [evidence/msh192](evidence/msh192/).

## Diagnosis (MSH-191 games, descriptive)

| Group | Time left at ply 30 / 60 / 90 | King advanced ≥ 3 ranks | King in back-rank corner |
| --- | --- | ---: | ---: |
| Losses to elmo 1000k (24) | 41.2 / 14.3 / 3.4 s | 1.2% | 0.1% |
| Losses to Gikou (139) | 44.3 / 18.3 / 3.7 s | 2.5% | 3.2% |
| Wins against Gikou (141) | 44.6 / 21.2 / 9.2 s | 2.6% | 0.8% |

Medians of mShogi's remaining time; shares of all positions. The extreme King weights did not show up as King walks or corner Kings. Clock use did show the expected pattern: mShogi reaches the late middlegame with little time.

## The tested set

- **Clock allocation:** Shogi USI clocks allocate over an expected 50 remaining moves instead of the Chess-style 30.
- **Bounded King weights:** King rank and file weights limited to ±100 and refit on the MSH-188 corpus with the shared Texel tool. Validation 0.21815 (MSH-188 fit 0.21809); held-out 0.21386 (0.21398).

## Game test

Candidate `de66ac9e` against predecessor `ce2f79bd`, both through the clocked USI route (`mshogi-match shogi-match`), 15 s + 0.25 s (the anchor control's 60:1 ratio), MSH-190 openings, SPRT H0 = 0, H1 = 10, maximum 315 pairs. Google Cloud Batch, c3-standard-4, eight VMs.

| Pairs | Pentanomial | LLR | Elo estimate |
| ---: | --- | ---: | ---: |
| 40 | [12, 1, 17, 2, 8] | −0.24 | −30.5 |
| 120 | [26, 2, 57, 6, 29] | 0.20 | 14.5 |
| 200 | [53, 4, 89, 9, 45] | −0.48 | −9.6 |
| 315 | [77, 6, 142, 14, 76] | −0.09 | 3.3 |

No decision at the maximum; the estimate is about zero. No game failed. Compute: 14.00 conservative VM-hours, US$2.82, 1.86 hours; all jobs verified absent.

## Decision

Both changes are rejected: neither the clock allocation nor the bounded King weights improve play measurably, and the King weights were not causing visible misplays. The frozen checkpoint allowed one ablation; it was not run, because splitting a null set is unlikely to be informative and would exceed this round's cap. The planned anchor recheck was not run, because no engine change remains. The predecessor's code is restored.

Retained infrastructure: the anchor tool accepts per-manifest reviewed clocks (60 s + 1 s or 15 s + 0.25 s), a predecessor opponent and a sequential-test summary (`summarize --sprt`), and the MSH-192 prefix.

The diagnosis suggests that cheap local corrections are not the limiting factor. The evaluation's overall quality is the more likely limit; see the proposal in MSH-50.
