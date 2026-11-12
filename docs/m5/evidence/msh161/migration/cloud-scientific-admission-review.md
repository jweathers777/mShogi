# MSH-161 (Competitive Gameplay Diagnosis): cloud scientific admission review

Reviewer: opening_sources, independent of parallel-runner implementation. This is a bounded, offline review of the handoff, verification/review receipts, eight-worker scientific audit and relevant adapter/manifest/allocation source. No engines, cloud jobs or external lookups were run. Detailed pilot raw-query revalidation is credited to the recorded independent pilot reviewer, not repeated or claimed here.

**Disposition: scientific execution validation accepted; propose admission of the first fresh 20-game primary Linux batch once the concrete manifest conditions below are satisfied.** No scientific defect requiring another pilot was found. This is not unconditional approval of an unseen submission manifest, a gameplay diagnosis, or spending approval. The owner's new full-study authorization supersedes the handoff's older statement that only pilots were authorized; coordinator must bind the current resource scope to the new run.

## Protocol comparison

The primary checkout and runner worktree contain byte-identical `main-freeze.json` and every file referenced by its `files` map. That preserves the master-supported roots, balance/admission evidence, native source, fixed opponent, full-history analysis and phase helpers. The runner checks every frozen schedule hash before preparing jobs. It consumes the original schedule order, including paired colors and the separate unaided pair in each20game batch; no selection from pilot outcomes is needed or permitted.

Linux native candidate/runner source is `77101fe3d38712361af8dd99b3ca2bcc6b5c17d3`; Oxidation source is `46a705b80e7191e7c302838eb8b85053b5994d65`. Linux executable hashes, architecture, release flags, compiler and Stockfish19 archive are explicit in `docs/experiments/linux-build.json`. Linux hashes appropriately differ from Mac executables; source identity does not imply identical performance or observations across hardware.

The adapter calls the frozen `run_pair`: both colors receive `go movetime 5000`, native `auto_claim`, paired full opening histories,512ply cap,15second reply watchdog and6000second pair watchdog. It does not insert a book, rating handicap or new rules authority. Native replay precedes analysis, and failed/incomplete pairs cannot silently become complete scientific results. One pair pipeline occupies one standard C3 VM; its game, screening and deeper queries are sequential. At most eight such VMs reproduces the demonstrated parallel allocation without intra-VM competing engine jobs.

The frozen analysis helper receives every supplied and played decision of both players with full native history. Screening remains500k nodes, unrestricted plus singleton played; Stockfish options and engine identity are retained. Prepared prefixes remain contextual and excluded from competitor-error selection. The adapter reproduces the frozen all-followup plus deterministic10% control selection, stratified by game/player/frozen phase, with the same seed text. It derives singleton5M played/alternative jobs and deduplicates equal moves. Missing alternatives remain an explicit later discovery obligation: skipping a null derived move does not resolve the scientific uncertainty. Likewise the automatic pipeline does not replace manual episode review, selected strong-play followups, consequential20M comparisons or discovery when required by the frozen method.

The eight-worker independent audit reports16replayed games,2661screening decisions/5322queries,1209selected confirmation decisions/1825queries and1841complete jobs. Histories, root coordinates, options, node limits, process results, binary hashes and regenerated screens/selection passed; bounds and uncertainty remain present, and no raw truncation is reported. These validate execution, not a weakness diagnosis. The pilot has only prepared roots; fresh unaided execution in batch01 still needs the normal completed-batch coverage check. The existing start-history path and fixed schedule need no separate new pilot solely for this distinction.

## Concrete prelaunch manifest conditions

1. Freeze a **new primary Linux experiment and result namespace**, with an explicit cohort ledger binding100pairs/200fresh games to the existing ten frozen schedules. Retain all Mac40games and cloud18pilot games as separate exploratory/validation cohorts. Reuse roots and fixed procedures, not prior game results or outcome-based root substitutions. Linux primary game identities must not resolve to pilot successes.
2. Admit only the first ten schedule pairs for the initial submission: nine prepared roots in their frozen order and one unaided pair,20games total. Preserve the full200game target in the cohort ledger and the subsequent batch schedule. The present CLI's `--pilot-pairs 10` labels a manifest `gcp-pilot`, whereas default preparation/submission includes100pairs. Therefore the coordinator must prepare and validate an explicit **primary batch manifest** containing those ten pair roots, their ten screening dependencies, automatic confirmations and correct primary-cohort metadata. Do not submit the100pair manifest believing parallelism8 limits the total to20games; it limits concurrency only. A reviewed immutable manifest slice is sufficient; this finding does not require a new orchestration framework.
3. Match manifest executable source hashes to the actual image. The validated worker is source `51351c48ea1adf9ac96011aec1440e44ceb741a4`, digest `us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:ad95d85a8f555acaaef2adafecb92b0277983fb46ce6da83c3c192267dcc43f7`. Later controller Python files differ, and prepare hashes all `tools/experiments/*.py`. Prepare worker-bound identities from the exact image snapshot, or package/verify a new image and freeze its real digest. Keep later controller revision/provenance separate. An arbitrary newer source SHA cannot be substituted for the validated worker bytes.
4. Freeze standard Linux x86_64 `c3-standard-4`, one task per VM, parallelism≤8, image/build hashes, resource/watchdog settings and actual cost-monitoring scope. Preserve runtime machine/CPU identity checks and compare returned worker receipts with the frozen allocation. Eight-worker throughput is demonstrated;16workers and serial-versus-parallel equivalence were not tested or inferred.
5. On completion retrieve and verify all static and derived obligations, native replay and raw coverage, review actual elapsed-time/resource receipts and perform the completed20game scientific review. Keep failures, caps and retries visible. Preserve claim-policy counterfactual limits and actual native outcomes. Require consequential20M/discovery work before final first/largest and ranking claims, without favorable-result retries. Subsequent authorized batches use the same procedure and separate batch receipts.

These are concrete admission conditions for manifest/execution provenance. There is no remaining scientific requirement to repeat already validated pilot games. This review does not certify a manifest not yet presented or claim that the primary200games have started.

## Limits retained

The pilot's eight concurrent task intervals, matching C3/Sapphire Rapids receipts and cleanup are documented by the separate runtime audit. Aggregate Batch runDuration disagrees with later task/GCS/VM timestamps; use the reconciled task evidence rather than the aggregate for duration claims. No speedup relative to Mac or serial execution follows. Application-failure recovery was exercised; abrupt cloud VM-loss/stale-claim recovery remains locally tested only. Actual billed charges remain unreconciled and are not established by the compute estimate.

A homogeneous Linux cohort reduces a known source of Mac timing variation but does not prove a universal fair-performance equivalence, a rating or population error rate. Standard hardware and fixed practical move time are the intended measurement conditions. Observed competitive behavior must still be independently adjudicated across all200fresh games before final priorities. No production changes are authorized by this admission review.

## Reviewed evidence identities

Paths below are relative to the parallel-runner worktree at review.

- `docs/experiments/coordinator-handoff.md`: SHA-256 `a69f5580a1bc53255037203ee5c221fb80b81748382a28bad095bb89cb9e3bd3`
- `docs/experiments/verification.json`: SHA-256 `6ce1e920ed79b4debce86a33dd9d631676bd7f477a5ab411c1b81c3bde981182`
- `docs/experiments/review.md`: SHA-256 `f6362bbef07e27413762189f94ab1e1588c40a349b1437da1721eea675c9817c`
- `docs/experiments/pilot/eight-scientific-review.json`: SHA-256 `8897634aa80249826cf5858e83b007d9221a009cce1430c15c53cee29bb18f2d`
- `docs/experiments/linux-build.json`: SHA-256 `b6dfc7d44e03b38171cdb42c24f3a477c388e97a188d9b322d4ed59e5e0be4e0`
- `tools/experiments/chess.py`: SHA-256 `6d3725584cc73c1fce418b732440c266911ce7c7c05e86c43e1a94431e6b0fc8`
- `tools/experiments/cli.py`: SHA-256 `8bb004983b9d2a990b1ebdb67b3c656a4ed249f5cd2523dfd7ffab5b736bbde1`
- `tools/experiments/core.py`: SHA-256 `34e4c395f6889da485d278b1dc85dc1f3a15e0e0b4ebdecb5ea405a013962fed`
- `tools/experiments/gcp.py`: SHA-256 `f973fe25231f6254b4216901fabb7409f02e9095d13913ed3e170265fad3bdc3`

## Coordinator's accepted manifest construction

The coordinator accepted the following design during review: prepare from the exact image-source snapshot with fresh primary experiment identities; retain `cohort=gcp-primary` and `pilot_pairs=null`; preserve a global200game cohort manifest mapping all ten batch manifests. Derive each immutable batch manifest by selecting only its ten original frozen-schedule pair roots and their dependent screening jobs, with automatic confirmations retained. Verify that the ten batch memberships partition all100pairs exactly once, and execute only the first20games initially, followed by20M/independent-review cadence. No pilot success is eligible for reuse. This design satisfies the manifest-scoping concern above; actual generated manifest hashes, membership and image identity remain the prelaunch receipt to inspect.
