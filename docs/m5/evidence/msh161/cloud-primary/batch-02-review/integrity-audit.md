# Primary cloud batch02 independent integrity audit

Offline execution_review, 2026-11-12. Reproducible command:

```sh
/private/tmp/msh139/venv/bin/python docs/m5/evidence/msh161/cloud-primary/batch-02-review/integrity-audit.py
```

**Passed.** No engines, cloud jobs or new native replay ran. The helper adapts the completed cloudbatch01 audit, uses scoped inventories, hashes all artifacts and independently replays complete histories and selected PVs. Existing successful native replay receipts are reused and bound to the exact compressed/uncompressed report bytes. No scientific attempt was rerun.

All20 native games concluded normally. Candidate outcomes were9wins/7draws/4losses: prepared8/7/3 across18games, unaided1/0/1 across2games. All seven draws are native claims: six threefold claims (00g1,01g1,02g0,02g1,03g0,07g0) and one fifty-move claim (08g1). Preserve those outcomes and the frozen unconditional auto-claim policy; Stockfish self-play continuations do not override them. These recorded results are not a rating or completed gameplay diagnosis.

All2,376 scientific jobs have immutable successful artifacts: ten paired-game jobs, ten screening jobs and2,356 singleton5M jobs. No failed scientific attempt, unresolved claim, raw-query truncation, unhealthy native game, query cleanup failure or per-VM job overlap was found. The runtime report separately establishes cloud VM/disk cleanup; it does not infer that from scientific status alone.

## Complete frozen coverage

- 2,840 played decisions plus288 prepared-prefix decisions generated6,256 initial500k queries: unrestricted and forced-played, both players, full histories.
- All1,364 follow-up decisions and exactly190 deterministic nonflagged audit decisions match independent reconstruction of the frozen rule, including all83 nonempty game/player/phase audit strata and rounding/minimum policy.
- The1,554 selected decisions generated2,356 singleton5M jobs after752 identical-choice deduplications. No alternative was missing. All derived IDs, selected actions and parent report dependencies match.
- Every raw query's root/history, mover, requested budget, engine/options, forced move, legal returned root, selected PV, clean exit and settled process group passed. Fresh-game and clear-hash commands were checked. All regenerated screens and mechanical phase tags agree; location-only path differences are normalized while hashes and basenames remain bound.

The full data are in `integrity-audit.json`, including exact manifest/index/script hashes, native conclusions, audit strata and resources. The audit stays within primary cloud evidence; Mac and pilot games are not mixed into its denominator.

## Uncertainty and audit gate

Initial queries retain2,369 stale-final-update flags,429 directional-bound conflicts and14 missing aligned nonbound numerical reports;5,858 reports meet the two-report stability heuristic. At5M,2,243/2,356 reports meet that heuristic, while1,014 stale-final flags and141 directional-bound conflicts remain. Categories overlap and are not confirmed error counts. Missing numerical reports explicitly retain uncertainty and a legal returned root.

The190 audits comprise93 same-choice controls,95 eligible stable two-choice comparisons and2 uncertain two-choice comparisons. **Zero eligible stable comparisons cross expected-score loss.05** under the frozen rule permitting only a lone stale-final-update flag among uncertainty flags and excluding directional conflicts. No automatic missed-important-error stratum-expansion trigger was found. Controls do not prove global best play, and the two uncertain comparisons are not silently promoted. Consequential20M, episode/claim-policy interpretation and final per-decision dispositions remain open.

| Stage | Parent wall seconds | Parent child CPU seconds | Nested query wall seconds | Nested query CPU seconds |
| --- | ---: | ---: | ---: | ---: |
| Native pairs | 13,843.902 | 13,938.940 | — | — |
| 500k screening | 6,688.117 | 5,673.549 | 5,700.232 | 5,673.240 |
| 5M confirmations | 15,418.623 | 11,878.617 | 11,812.719 | 11,802.549 |

Parent sums are35,950.642wall seconds and31,491.107child CPU seconds. Query figures are nested and must not be added to parent totals. Retrieval separately verified34,751objects /4,441,259,122bytes in443.251seconds. No execution-integrity or frozen-coverage blocker was found; this pass does not close the scientific20M review gate.
