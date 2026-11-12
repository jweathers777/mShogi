# Completed cloud 20M: integrity, runtime and cleanup reconciliation

Independent read-only review, execution_review, 2026-11-12. Reviewed the exact frozen plan `066c4b94…72f5`, immutable raw job artifacts and local verification receipt `fc9ce7f0…f7fd`; took one completed GCP task/VM/disk snapshot and one normal-access local process-table snapshot. No scientific computation, retries, cloud changes or signals to the completed wrapper/group occurred.

## Scientific execution passed

All 527 planned jobs / 558 queries for 298 decisions completed successfully, including 31 discovery jobs with unrestricted plus played queries. Exact parent and consumed-result hashes, query context/full history, source/build identities, Stockfish options, requested20M budget, forced roots, clear-hash commands, native replay receipts and every selected PV's independent legal replay passed. All successful artifacts were rehashed. No failed scientific attempt, truncated raw query, unsettled query group or unresolved job claim was found.

All eight workers recorded the pinned Linux x86_64 source/build on C3-standard-4, with retry attempt zero. Numerical uncertainty remains: 302 stale-final-update flags, 57 directional-bound conflicts and 497 reports meeting the two-report stability heuristic. These overlapping categories are not confirmed mistakes. Gameplay/episode adjudication remains separate; the coordinator reports neither reviewer found a distinct new discovery alternative requiring another query, which is not inferred merely from this execution pass.

| Resource layer | Wall seconds | Child CPU seconds |
| --- | ---: | ---: |
| All527 job receipts | 12,666.016 | 11,834.948 |
| Nested558 query receipts | 11,820.747 | 11,818.149 |

Mean query wall time was21.184s and maximum27.916s. Job wall includes native replay, process and artifact work. Query totals overlap job totals; never add both.

## Cloud and local completion are distinct

Eight tasks succeeded. First-running to last-success was1,753.639s; submission to last-success was1,795.923s (29m56s). Actual peak concurrency was eight, with eight tasks overlapping for1,607.998s. All eight successful VM creations have completed deletion audit records. Current project VM and disk listings were empty. Total lifecycle was3.817809VM-hours, approximately **US$0.769701 compute** at the recorded$0.201608/VM-hour rate, excluding other charges. Actual billing remains unknown.

The cloud finished at09:52:21 UTC. Local verification output's observed modification time was10:14:55; wrapper completion was10:14:57. The **22m36s post-cloud interval** includes terminal event delivery, retrieval, local integrity/resource verification, durable receipt publication and shutdown. Download-only time was not instrumented. The527 planned job directories contain6,355 objects /952,276,772bytes; this is a local artifact byte total, not a network transfer meter, and excludes additional consumed-success reads/protocol overhead.

Source inspection found repeated whole-cache enumeration in the then-executed controller verification loop. That is a credible contributor to local delay, not measured attribution of the entire interval. The coordinator is separately validating a later controller-only inventory optimization. Original plan/controller hashes remain historical execution provenance; any later faster verifier cannot retroactively shorten this observed run.

## Historical wrapper failure preserved, cleanup reconciled

`notification.json` truthfully records **wrapper status=failure**, child computation returncode0, process-group status unknown after `PermissionError(EPERM)`, cleanup review required, and successful notification queue acceptance. Do not rewrite this receipt as success or call it a failed scientific query.

A separate read-only `ps` snapshot found no matching PID40795, PID40796 or PGID40796. Exact observation time is recorded in the JSON alongside the command/method. Combined with successful child return, complete verified artifacts, clean per-query receipts and empty cloud VM/disk listings, this reconciles the reported cleanup uncertainty for the recorded processes. It is not a claim of a universal descendant-tree census and does not erase the historical permission failure. **No retry is indicated.**

The first independent read-only audit itself was stopped after its use of repeated whole-cache enumeration proved wasteful. The coordinator explicitly authorized stopping that audit process only. Its rerun used one scoped attempt inventory, completed successfully and repeated no engines or experiments. This reviewer overhead is not included in scientific compute totals.

Detailed task intervals, VM lifecycles, actual snapshot state, hashes and resource/quality counters are in `runtime-integrity-review.json`; read-only cloud observations are retained in `runtime-observations.json.gz`.
