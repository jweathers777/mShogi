# Batch02 two-query follow-up completion audit

Independent execution_review,2026-11-12: **PASS; no execution-integrity or cleanup blocker for the batch03 gate.** This was an offline artifact audit plus one read-only cloud task/VM/disk snapshot. No engines, retries, submissions or scientific reruns occurred. Scientific closure remains the gameplay reviewers' separate decision.

Both new singleton20M queries match plan`5fa4df35…ad10` and verification`bf49a87c…35ad`: `main-02-02/g0/p65 g8g7` and `main-02-08/g0/p96 c5e4`. Exact original native histories, hashes, source/build/options, requested budget/moves, legal selected PVs, clean exits and settled groups passed. Both new reports satisfy the two-report stability heuristic and show no directional-bound conflict or stale-final-update flag. This does not itself establish which move is superior.

The corresponding existing played20M queries (`f6f8` and `f4f5`) were independently rebound to their immutable success/artifact bytes and checked for identical historical root, budget, options and executable. Their PVs replay legally. They were **reused, not rerun**, and are counted only in the original panel. No failed new attempt, unresolved claim or truncated raw query was found.

The two cloud tasks succeeded on two matching workers. Peak concurrency was two, overlapping22.049s. First task to final task was29.828s; submission to final task82.345s. Both successful VM creations have successful deletion records; no failed insert or resource-pool notice was observed. Current project VM/disk listings were empty. The wrapper returned0, recorded a settled child group and successfully queued its one notification. This is not a new local process census.

Actual resource layers:

| Layer | Wall seconds | Child CPU seconds |
| --- | ---: | ---: |
|Two new job receipts |42.469|34.003|
|Nested query receipts |33.937|33.927|

VM lifecycle totaled0.040590hours, estimating **$0.008183 compute** at the recorded$0.201608/VM-hour rate. Do not add nested query and job figures. Actual billing and ancillary charges remain unverified.

The sequential driver took131.555s in total. Preflight to the durable submission marker was13.761s; post-cloud event/retrieval/verification/publication/shutdown was33.708s. Download-only timing is not separately instrumented. This small workflow's startup/transfer fraction should not be extrapolated as throughput for a large panel.

## Cumulative primary accounting

The two primary base batches, two initial20M panels and these two actual follow-up queries now total40games,5,942 scientific jobs and17,783 reference queries, with no failed scientific attempts. Combined VM lifecycle is **29.409754hours**, estimating **$5.929242 compute**. Mac and pilot work are excluded; old played comparisons are not counted twice.

Scaling the observed mean gives a compute-only reference of **$29.65 full200 /$23.72 remaining160**. The prior conditional inclusive scenarios remain **$35–90 full /$30–80 remaining before tax**, including the stated variable panel/recovery and ancillary assumptions. They are not bills, caps or confidence intervals; later follow-up counts and availability can differ. Detailed totals are in `forecast-update.json` and `runtime-integrity-review.json`; raw observations are retained in `runtime-observations.json.gz`.
