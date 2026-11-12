# Batch 02 independent completion audit

Reviewer: execution_review, 2026-11-12. Offline audit against the unchanged frozen main method; no engines, native replays, 20M searches or new batches launched. Reproducible command:

```sh
/private/tmp/msh139/venv/bin/python docs/m5/evidence/msh161/main/batch-02/verification-audit.py
```

The command passed. `audit.json` records hashes, environment, actual resource totals, uncertainty, all 82 audit strata and native conclusions. The script adapts the prior batch 01 audit solely in this new directory and adds exit-notification and absent-batch-03 checks. It does not modify frozen execution helpers.

## Completion and preservation state

All ten pairs / 20 native games completed with native conclusions, no unscored result, diagnostic failure, transcript truncation or operational cap termination. Exact report hashes match the existing successful native replay receipts; full histories independently replay legally and match the prepared prefixes plus actual recorded bestmoves. This reuses native replay evidence; it is not a new native invocation.

| Native outcome stratum | Games | Candidate wins | Draws | Candidate losses |
| --- | ---: | ---: | ---: | ---: |
| Prepared | 18 | 9 | 3 | 6 |
| Unaided | 2 | 2 | 0 | 0 |
| Total | 20 | 11 | 3 | 6 |

These are recorded outcomes only, without a strength or gameplay interpretation. Along with the previously audited batch 01, 40 of the planned 200 main games are complete. All 2,224 batch 02 orchestration steps and 8,243 reference attempts succeeded. No failed retry, unresolved in-flight marker, temporary artifact, missing alternative or recorded residual process group was found. Every frozen source, schedule and executable hash checked matched.

The batch 02 wrapper receipt records successful computation exit, settled child process group and successful one-shot queue acceptance. Helper receipts independently record engine cleanup. No batch 03 directory exists at audit time. This establishes completed recorded execution and cleanup, not an independent live process census or a universal descendant-tree guarantee.

**Owner pause/cloud migration supersedes earlier continuation instructions. Preserve this completed batch; do not launch engines, 20M adjudication or batch 03.** No analysis uncertainty is being cleared merely because execution completed.

## Coverage and conditions

- All 2,732 played decisions and 288 prepared-prefix decisions have unrestricted and forced-played 500k queries: 6,040 initial queries, both players and every historical root.
- All 1,249 v1 follow-up decisions and exactly 202 deterministic nonflagged audit decisions match the frozen selection reconstructed from the raw screens and regenerated phase tags. Every nonempty game/player/phase audit cell meets its minimum and rounding rule.
- The resulting 1,451 decisions have 2,203 singleton 5M queries after 699 identical-choice deduplications. Every full root/history/mover, forced move, node budget, options, manifest, report and receipt identity matches. Confirmation artifacts retain `confirmed_event=false` and the requirement for later adjudication.
- All 2,213 query manifests were verified. Resource ledgers reconcile to individual attempts; execution intervals show no recorded native/reference overlap. Native requests were exactly `go movetime 5000`, with frozen auto-claim policy, paired colors and bounds.

Candidate/Oxidation response medians were 5.001701/5.000437 seconds, maxima 5.007244/5.024650 seconds. Earlier returns remain recorded. The shared unpinned host and load observations remain a limitation for hardware comparisons.

## Data quality retained at pause

Initial queries contain 2,172 stale-final-report flags, 420 directional-bound conflicts and 12 missing aligned numerical/PV reports; these categories overlap. The 12 missing aligned reports retain explicit uncertainty and legal root bestmoves. Played screens contain 1,186 uncertain rows, 220 raw loss-threshold rows and 309 negative raw differences, also overlapping. None is converted into a confirmed error count here.

At 5M, 2,122/2,203 query reports satisfy the two-report stability heuristic; 1,021 retain a stale-final-report flag and 118 have directional-bound conflicts. None lacks a numerical estimate. Stability alone does not establish correctness or clear conflicting bounds.

No stable two-choice nonflagged audit comparison crosses expected-score loss .05 under frozen eligibility (a lone stale-final-report flag allowed, directional conflicts and other defects excluded). Thus no automatic missed-important-error stratum-expansion trigger was found by this check. Same-choice controls do not establish global best play. Unresolved comparisons and consequential adjudication remain outstanding and must be preserved through migration, without starting them during the pause.

## Actual resources

| Component | Count | Process wall seconds | Child CPU seconds |
| --- | ---: | ---: | ---: |
| Native games, replay and cleanup | 20 games | 13,163.047 | 13,151.964 |
| Initial 500k queries | 6,040 | 3,497.200 | 3,332.858 |
| Matched 5M queries | 2,203 | 9,111.501 | 9,011.508 |

Native response time totals 13,160.648 seconds. Chain elapsed is 27,424.393 seconds (about 7h37m). Query maxima were 2.452 seconds initial and 8.065 seconds at 5M, within the reviewed bounds. Stage and chain durations overlap process totals and must not be added again. Artifacts were 351,775,151 bytes at audit time before this review and archival.

No execution-integrity or coverage defect was found. This is an evidence-preservation checkpoint, not authorization to resume computation or a final gameplay adjudication.
