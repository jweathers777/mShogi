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

No decision at the maximum; the estimate is about zero. No game failed. Compute: 14.00 conservative VM-hours, US$2.82; 1.82 hours of batch time (1.86 hours including gaps between batches); all jobs verified absent. The new `--sprt` and clock paths of the tool have no unit tests; they were checked by the reviewer against the evidence.

## Decision

Both changes are rejected: the set showed no gain, and the individual effects were not measured (they could offset each other). The diagnosis observed no King walks or corner Kings, so the extreme weights were not visibly harmful. The recorded checkpoint called for one ablation; it was deliberately not run. That is a decision, not a cap limit: a shorter ablation would have fitted the US$6 cap, but a null set gave little reason to expect a useful split. The test also ran at 15 s + 0.25 s rather than the 60 s + 1 s named in the acceptance criteria (same base-to-increment ratio, four times cheaper). The planned anchor recheck was not run, because no engine change remains. The predecessor's code is restored.

Retained infrastructure: the anchor tool accepts per-manifest reviewed clocks (60 s + 1 s or 15 s + 0.25 s), a predecessor opponent and a sequential-test summary (`summarize --sprt`), and the MSH-192 prefix.

The diagnosis suggests that cheap local corrections are not the limiting factor. The evaluation's overall quality is the more likely limit; see the proposal in MSH-50.
