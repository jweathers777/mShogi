# Primary cloud batch02 runtime and forecast update

Independent read-only snapshot, execution_review,2026-11-12. All ten tasks for `msh161-primary-b02-v1`, UID `msh161-primary-b02-fff81736-b9df-43bd0`, succeeded. Detailed task/VM records and exact observation hashes are in `runtime-review.json`; raw snapshots are retained in `runtime-observations.json.gz`. No engines, submissions, retries or cloud changes occurred.

First task to last success was6,502.789s (1h48m23s); submission to last success was6,541.838s (1h49m02s). Actual peak concurrency was **eight**, with eight tasks overlapping3,714.278s. Ten pipelines reused eight VMs. Total VM lifecycle was10.905060hours, including startup/idle/cleanup; all eight successful creations have successful deletion records. Current project VM and disk listings were empty.

No failed VM insert completion or resource-pool operational notice was found for this job/window. This differs from batch01's six-worker peak and provisioning failures; it does not promise future regional availability. Scientific attempt failures remain separately checked: the completed offline audit found none across2,376 jobs /8,612 queries.

Estimated VM compute is **$2.198547** at the recorded$0.201608/VM-hour rate, not an observed bill. Compute is slightly higher than batch01 despite shorter elapsed time because concurrent capacity improved and game/analysis workload changed. Parent stage wall totals were13,843.902s native,6,688.117s screening,15,418.623s5M; their sum35,950.642s is **nested inside lifecycle accounting**, not additional VM time. Detailed CPU and nested-query totals are in the integrity audit.

Retrieval verified34,751objects /4,441,259,122bytes in443.251s. Submission through verified retrieval was6,998.529s (1h56m39s). Wait/retrieve and notification receipts both succeeded; the recorded local child group was settled and the queue accepted its one notification. This snapshot does not claim an independent current local process census. Manifest, terminal and retrieval receipt hashes match the saved chain. No retry is indicated.

## Forecast after40 primary games and one20M panel

Using the mean of the two measured base batches and repeating the first measured298-decision20M panel yields:

| Compute reference | Estimated USD |
| --- | ---: |
| Mean base20-game batch | 2.1321 |
| First20M panel | 0.7697 |
| Full200 games plus ten equivalent panels | 29.0180 |
| Remaining160 games plus all nine pending panels, including batch02 | 23.9841 |
| Completed primary work so far: two base batches and one panel | 5.0339 |

These are lifecycle-based compute references, excluding ancillary charges. Batch02's20M selection/work remains pending; a later panel need not match the first. The inclusive conditional envelope remains **$35–90 before tax for the full200**, and is **$30–80 for the remaining160 plus all outstanding adjudication** under the earlier0.5–2x panel workload,0–25% extra compute/recovery, transfer/operations/one-month-retention provisions. In the remaining scenario, compute alone spans roughly$20.52–38.64; retained/downloaded evidence and unmeasured request costs remain additional. These are sensitivities, not bills, spending caps or confidence intervals. Actual billing remains unreconciled; the prior project-visible export check is not repeated or presented as a fresh observation.

The updated full-workflow reference is **30.64processing/retrieval/verification hours**, or **25.38hours remaining**, before review pauses. This uses measured submission-to-last-task plus retrieval for the two base batches and conservatively retains the first20M panel's actual22m36post-cloud retrieval/verification interval. It does not pretend the later4.97s warm-cache verification test measured network improvement. Nine one-hour inter-batch reviews would give about39.64hours full; eight future one-hour review boundaries would add eight hours to the remaining reference. Larger panels, slower retrieval, capacity shortages, recovery and human review can extend these times.

No execution-integrity or runtime-cleanup blocker was found. Consequential20M/episode review and the frozen scientific gate remain required before further primary progression. Preserve native claim outcomes: this batch's seven draws comprise six threefold and one fifty-move claim, irrespective of Stockfish continuation estimates.
