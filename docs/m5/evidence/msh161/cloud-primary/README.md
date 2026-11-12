# MSH-161 (Competitive Gameplay Diagnosis): primary cloud study

John authorized the full cloud study on12November2026 after reading the runner
handoff. The coordinator admitted the existing pilot evidence and integrated
MSH-162 (Parallel Experiment Runner) at `bb5e698e`. All64 local integration tests
passed. Independent runtime/scientific reviews are in `../migration/`; the actual
manifest review is `manifest-admission.json`. No new pilot was needed.

## Frozen primary sample

`cohort-freeze.json` covers all100original pairs /200games, in ten20-game batches.
Each batch has9prepared pairs and1unaided pair. `prepare_cohort.py` exported the
exact tested worker source, prepared the full original sample, then selected each
original batch without changing pair specifications or job identities. The
independent review checked every pair, job, source hash and allocation. No Mac or
pilot result is counted as a primary game. No opening was changed after review.

Worker source: `51351c48ea1adf9ac96011aec1440e44ceb741a4`.
Worker image: `us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:ad95d85a8f555acaaef2adafecb92b0277983fb46ce6da83c3c192267dcc43f7`.
Use the integrated controller for submission and event handling. Its later event
matching fixes do not change the worker image. Do not prepare new job hashes from
newer controller files and send them to the old image. The exported source hashes
and path are in `worker-source-export.json`; the original Git revision can recreate
that export if the local temporary directory is lost.

The primary store is `gs://mshogi-experiments-216675633801/msh161-primary-v1`.
It is separate from the pilot prefix. It was empty at preflight. All jobs use
`c3-standard-4`, one task per VM, at most8workers, with the tested Linux binaries
and unchanged5s/move method. All scientific thresholds, claims and source histories
remain frozen. Each batch runs games, full500k screening and selected matched5M
queries. Consequential20M work and independent episode review remain separate
obligations; completion of a pipeline alone does not confirm chess mistakes.

## Current batch and next review

Primary batch01 completed successfully as `msh161-primary-b01-v1`. All10cloud
pipelines reached success. The event listener queued the coordinator; its terminal
response is `batch-01-listener.stdout.log`. Runtime review records2h10m35s from
first task start to last completion and2h19m02s from submission. Only6VMs started
because of resource shortages, despite requested parallelism8;53failed insertion
operations are separate from game-attempt failures. Task/VM records govern timing.

Raw retrieval completed and independently verified. State:
`batch-01-review/retrieval-state.json`; notification:
`batch-01-retrieval-notification.json`. The ignored `local-cache/` contains all
34,461 objects / 4,582,665,719 bytes. Immutable cloud objects remain authoritative.
Generation/CRC checks and every result-artifact SHA passed. Retrieval used no
engines. Do not repeat it. `batch-01-review/review-index.json` locates all results.

The independent offline audit (`batch-01-review/integrity-audit.md` and `.json`)
passed all 2,410 jobs and 7,948 queries: 5,558 initial and 2,390 selected 5M queries.
The 20 games finished 8W/5D/7L; native histories contain 2,491 played and 288 supplied
prefix decisions. All frozen selections matched, with zero stable missed-error
audit crossings. This does not establish absence of other errors. Gameplay reviews
and full decision dispositions are under `batch-01-review/`. Their proposed 20M
panels retain linked opponent events, successful controls and unresolved earlier
choices. No final first/largest labels or weakness priorities are established.

Large review metadata is stored byte-for-byte in `batch-01-review/*.json.gz`.
`metadata-archives.json` records the expanded and compressed SHA-256 values. Restore
an expanded file with Python `gzip.decompress` before a tool that expects JSON; keep
it beside its archive. The expanded index and inventory are ignored by Git. These
are metadata, not replacements for the immutable raw cloud evidence.

Next gate: independent scientific/gameplay review of retrieved artifacts, required
cloud20M adjudication, and the requested full200time/cost forecast BEFOREbatch02.
The runtime estimate alone is not the final forecast: analysis/review pauses,
ancillary costs and uncertain capacity remain material. No batch02 submitted.

The independently reviewed first 20M plan is `batch-01-adjudication/plan.json`:
298 decisions, 527 jobs, 558 queries, including 31 unrestricted-plus-played discovery
jobs. Both gameplay panels use the same sensitivity/consequential selection rule.
Cross-reviews and semantic admission pass; `coordinator-execution-review.json`
records the controller review and nine passing offline tests. Only explicit
confirmation jobs run in the existing image; no games are repeated. Eight balanced
task assignments retain the existing C3 allocation and process bounds.

`adjudication_control.py wait-verify` uses the existing Pub/Sub terminal event,
then fetches only plan results and verifies them locally. It publishes separate
immutable plan verification/completion receipts. Run it through `notify_exit.py`
with normal local access to queue this chat on success or failure. The ordinary
manifest status does not enumerate these additional jobs. Distinct alternatives
found by discovery still need a later matched singleton query if not already in
the plan; initial completion cannot close that scientific obligation. No timed
watcher or local engine is needed. Inspect submission and notification receipts
before restarting any process; an ambiguous submission must be reconciled.

On retrieval/adjudication completion:

1. Read the completion/submission/attempt records. Verify the actual terminal state
   and cleanup. If failed, preserve all attempts and reconcile claims only after
   proving their workers stopped. Do not blindly resubmit.
2. Retrieve the completed batch to a separate local cloud-evidence cache. Use the
   generation-pinned store reads and bounded parallel I/O for large retrievals;
   preserve all objects and verify every static and derived result. Run long data
   retrieval unattended with exit notification, not model polling.
3. Have independent agents review completed games and reference evidence. Preserve
   flags, missed-error audit triggers, full histories, native outcomes, uncertainty,
   successful play and automatic-claim limits. Keep source phases and denominators.
4. Schedule required matched20M and missing-alternative discovery on the cloud,
   with reviewed immutable query specifications and verified report dependencies.
   Preserve earlier/linked uncertainty. Do not repeat games to obtain analysis or
   overwrite the original manifest/success objects. See any adjudication execution
   notes added here before the first such submission.
5. Before submitting batch02, provide the owner-requested full200-game time and
   cost forecast (`forecast-request.json`). Use actual batch01 task/VM lifecycle,
   analysis and data measurements; include remaining20M work, ancillary charges,
   retry/startup overhead and review pauses. Give total and remaining ranges,
   assumptions, and distinguish estimated from billed costs. Then review the
   completed adjudication and update primary categories and resource totals. Continue this cycle through all200games.
   No final ranking or production correction before full independent diagnosis.

For a user-requested progress view, run `python3 tools/experiments/quick_status.py`.
Its settings now point to this primary batch. It is a read-only snapshot, not an
integrity check. Do not turn it into a periodic model watcher. Local Mac computation
remains stopped; the old local batch02 callback cannot authorize localbatch03.

## Resource scope and limits

The latest owner instruction authorizes the full200-game cloud study and its
necessary analysis. The earlierUS$30approval applied to pilots only. It is not a
new primary-study cap or a claim that the primary campaign has already been paid.
At the recorded pilot rate and measured VM lifecycle time,100pairs extrapolate to
about117.16VM-hours /US$23.62compute. This estimate excludes later20M work, disks,
storage, reads/egress, logs, registry, taxes and other charges. It is not a bill or
guarantee. Pilot retrieval volume scales to roughly49.76GB for100pairs; preserve
raw evidence in GCS and use ignored local caches, not giant generated Git bundles.

Record actual attempts, task/VM lifecycle time, query CPU/wall time, retained data
and billed charges when available. Avoid counting parent elapsed time twice. Batch
aggregate runDuration disagreed with task/GCS/audit records in the pilot; use those
records for actual timing. Actual pilot billing is still open and must not be
reported as reconciled. The tested6h per-task watchdog and one retry are operational
bounds, not a scientific truncation rule. Preserve incomplete outcomes as failures,
never as chess results. Review substantial unexpected resource growth at each batch.

The original native/opponent source and scientific inputs did not change. Mac,
pilot and primary cohorts remain separate. The final report must retain both
observed weaknesses and contrary successes, separate hypotheses from evidence,
update the roadmap and stop for John before production engineering.
