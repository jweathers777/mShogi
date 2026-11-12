# Primary cloud batch 01 independent scientific execution audit

Offline review, execution_review, 2026-11-12. Reproduce with:

```sh
/private/tmp/msh139/venv/bin/python docs/m5/evidence/msh161/cloud-primary/batch-01-review/integrity-audit.py
```

**Passed.** The script launched no engine, native replay or cloud job. It reused the existing successful native replay receipts bound to exact report bytes, independently legally replayed all game histories and all selected query PVs, checked every consumed artifact against the immutable success record, and reconstructed the complete frozen screen/phase/selection procedure. The only corrected audit assumption was cloud temporary paths differing from retrieved local paths: interpretation comparisons normalize path basenames while retaining receipt/content hashes. No experimental result was changed or rerun.

All 20 games have native conclusions and healthy process records. Candidate native outcomes are 8 wins, 5 draws, 7 losses: prepared 7/4/7 across 18 games, unaided 1/1/0 across two. These are native outcomes, not an adjudication or rating claim. Paired colors, prepared prefixes, exact complete histories, five-second commands, frozen identities and auto-claim policy match. Native conclusions, including claims, remain authoritative.

All 2,410 jobs and their artifacts were verified: ten game-pair jobs, ten reference-screening jobs, and 2,390 matched 5M singleton jobs. There are no failed job attempts or unreconciled claims in the retrieved experiment. Per-VM job intervals do not overlap. All workers report the pinned Linux x86_64 build on C3-standard-4/Sapphire Rapids, with retry attempt zero. Provisioning errors described in the runtime review are separate from these successful scientific attempts.

## Complete coverage

- 2,491 played decisions plus 288 prepared-prefix decisions produce 5,558 initial queries, unrestricted and forced-played at 500k nodes, both players and full histories.
- The frozen selection independently reproduces all 1,417 follow-up decisions and exactly 152 deterministic audit decisions, including each nonempty game/player/phase cell's minimum and rounding rule.
- The resulting 1,569 decisions produce 2,390 singleton 5M jobs after 748 identical-choice deduplications. No alternative is missing. Derived job identities exactly match the verified screen results and selection plan.
- Every query's root/history, node limit, engine/options, fresh-game/clear-hash commands, requested forced move, clean exit and process-group settlement passed. Raw outputs were not truncated. Every selected numerical PV replayed legally from its complete historical root. Missing aligned numerical reports retain a legal returned root move and explicit uncertainty.

The script and detailed `integrity-audit.json` retain source/manifest/index hashes, all audit strata and per-stage totals. Parent retrieval separately recorded 34,461 objects / 4,582,665,719 bytes in 413.085 seconds; this review does not reinterpret transfer time as engine computation.

## Uncertainty and missed-error audit

Initial queries retain 2,305 stale-final-update flags, 436 directional-bound conflicts and seven missing aligned nonbound numerical reports. 5,246 initial query reports meet the two-report stability heuristic. At 5M, 2,268/2,390 reports meet that heuristic, while 1,147 stale-final-update flags and 168 directional-bound conflicts remain. Categories overlap; none is a count of confirmed errors.

The 152 nonflagged audits comprise 83 same-choice controls, 66 eligible stable two-choice comparisons and three uncertain two-choice comparisons. **No eligible stable comparison crosses expected-score loss .05**, using frozen eligibility that permits a lone stale-final-update flag but excludes bound conflicts and other defects. No automatic missed-important-error expansion trigger was found. Same-choice controls are not proof of global best play; uncertain and consequential cases still need completed-panel 20M review. No broader game-quality inference follows from a technical pass.

## Resource accounting

| Stage | Parent job wall seconds | Parent child CPU seconds | Query-only wall seconds | Query-only CPU seconds |
| --- | ---: | ---: | ---: | ---: |
| Native pairs | 12,049.772 | 12,133.196 | — | — |
| 500k screening | 5,974.388 | 5,084.365 | 5,107.976 | 5,084.079 |
| 5M confirmations | 15,781.498 | 12,322.110 | 12,254.752 | 12,244.115 |

Parent jobs sum 33,805.658 wall seconds and 29,539.671 child CPU seconds. Query subtotals overlap parent totals and **must not be added again**. Parent wall includes replay/artifact handling in addition to queries. VM lifecycle additionally includes provisioning/idle/cleanup; the independent runtime audit records 10.245834 VM-hours and actual peak concurrency six. Query averages were about 0.919 seconds at 500k and 5.127 seconds at 5M. Neither establishes actual cloud 20M throughput.

No execution-integrity or frozen-coverage blocker was found. Preserve the consequential 20M, episode, strong-play and final per-decision obligations. The forecast can now use these stage measurements and actual bytes, but remains provisional until cloud 20M work is measured and actual billing is reconciled.
