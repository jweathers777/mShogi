# Batch02 completed20M integrity, runtime and cleanup

Independent execution_review,2026-11-12. **PASS for the completed panel.** The audit used one scoped attempt inventory, exact immutable hashes, full-history legal/PV replay and one read-only completed-cloud task/VM/disk snapshot. No engines, retries, submissions or scientific reruns occurred.

All627 jobs /663 queries for371 decisions, including36 discovery jobs, match plan SHA`15ba357c…ce99` and verification SHA`bb01e689…4397`. Parent/consumed-success hashes, exact historical roots, mover/played action,20M budget, frozen source/build/image/options, clear-hash commands, requested alternatives, native replay receipts, all selected legal PVs, clean query exits and settled groups passed. No failed scientific attempt, unresolved claim or raw-query truncation was found.

Uncertainty remains explicit:322 stale-final-update flags,44 directional-bound conflicts and590 reports meeting the two-report stability heuristic. These overlap and are not confirmed error counts. Native auto-claim outcomes remain authoritative. The coordinator's reviewers identified **two further singleton20M comparisons**; this completed-panel pass neither runs nor closes them.

| Resource layer | Wall seconds | Child CPU seconds |
| --- | ---: | ---: |
|627 job receipts |14,595.225|13,589.311|
|Nested663 query receipts |13,572.212|13,569.055|

Mean query wall time20.471s; maximum27.269s. Query totals overlap parent totals and must not be added again.

## Cloud execution and cleanup

Eight tasks succeeded on eight matching C3-standard-4 Linux x86_64/Sapphire Rapids workers with retry attempt zero. Actual peak concurrency was eight, sustained for1,806.185s. First task to last success was1,976.481s; submission-to-last-task was2,019.056s (33m39s).

All eight successful VM insertions have successful deletion audit records. No failed insertion or resource-pool operational notice was found in this job/window. Current project VM and disk listings were empty. Lifecycle totaled4.400461VM-hours, estimating **$0.887168 compute** at the recorded$0.201608/VM-hour rate. This is not an actual bill and excludes ancillary charges.

The wrapper returned success, child returncode0, settled child process group and accepted one completion notification. This review reuses those local receipts rather than claiming a new local process census. Cloud raw observations are retained in `runtime-observations.json.gz`; detailed hashes, task/VM intervals and counters are in `runtime-integrity-review.json`.

## Submission and post-cloud timing

The single driver wrapper covered submission preflight, cloud wait and retrieval/verification. Its total elapsed was2,936.360s (48m56s):

- Wrapper start to durable submission marker:241.575s (4m02s). This includes local checks, consumed-success reads/hash checks and plan publication; it is not engine time.
- Wrapper start to cloud job creation:243.064s. The extra approximately1.49s follows the durable marker.
- Cloud creation to final task:2,019.056s (33m39s).
- Final cloud task to wrapper completion:674.240s (11m14s), combining event delivery, retrieval, verification, publication and shutdown.

The last three spans reconcile the complete wrapper interval. Download-only timing is not separately instrumented. The planned job directories contain7,560objects /1,133,730,952bytes; this is local artifact volume, not a meter for all network reads. Post-cloud time is lower than batch01's22m36s, but different data volumes/cache state and the verifier change prevent attributing the entire difference to one optimization.

## Cumulative resource forecast

Two completed primary base batches plus both completed initial20M panels comprise40games,5,940 scientific jobs and17,781 reference queries, with no failed scientific attempts. Their VM lifecycle totals **29.369163hours**, estimating **$5.921058 compute**. Prior Mac work, cloud pilots, and the two not-yet-executed follow-up queries are excluded.

Repeating the observed mean workload gives a compute-only reference of **$29.61 for the full200**, or **$23.68 for the remaining160**, before extra follow-ups and ancillary charges. The conditional inclusive planning envelopes remain **$35–90 full /$30–80 remaining before tax**, under the previously explicit panel-size, recovery, transfer/operation and retention assumptions. They are not bills, caps or confidence intervals. Actual billing remains unreconciled, and future availability/panel size can exceed these scenarios. Record the two extra comparisons' actual resource use when completed rather than counting planned work as incurred.

The latest33m39 cloud /11m14 post-cloud panel timings refine the measurement basis, but the previous conservative elapsed forecast remains appropriate until more stages establish stable rates. Submission preflight is now explicitly measured and must be included in later whole-workflow projections rather than hidden in model activity.

No completed-panel integrity or cleanup blocker was found. The remaining two comparisons and independent scientific closure still govern progression to batch03.
