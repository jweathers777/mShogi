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

Primary batch01 is reviewed through required20M. Native20-game result:8W/5D/7L.
`batch-01-closure.md` and `primary-cumulative.json` preserve the full dispositions,
provisional unranked categories, contrary successes and limitations. Both independent
gameplay panels and cross-reviews passed. No final first/largest claim or engineering
priority is inferred. Mac and pilot evidence remains separate.

Base integrity passed2,410jobs/7,948queries:5,558initial500k and2,390matched5M.
All2,491played and288supplied context decisions remain in full native histories.
There was no eligible stable missed-error audit crossing. The selected20M panel
passed527jobs/558queries for298decisions. All31discoveries chose the existing played
move, so both followup-selection files are empty; no further singleton is missing.
Selected candidate finite comparisons:58of178queried decisions across18games;
44selected candidate decisions remain unresolved. Related decisions are not
independent loss causes and unqueried earlier uncertainty remains explicit.

The20M job `msh161-primary-b01-20m-v1` succeeded. Local verification also succeeded.
`batch-01-adjudication/notification.json` retains the wrapper's failure caused by
EPERM during post-exit process-group inspection, and its accepted queue notification.
Independent reconciliation found no recorded PID/process group or cloud VM/disk
remaining. This does not require a scientific retry. See runtime-integrity-review
under that directory. Do not relaunch the completed wrapper or searches.

Measured main run:2h19m cloud elapsed, actual6of8workers and53failed allocation
operations. Measured20M:29m56s cloud elapsed, actual8workers, estimated$0.7697compute.
Both cleanup and failure histories are retained. Actual billing remains unknown.
`full-study-forecast.md` supplies the required full200/remaining180 forecast before
batch02; earlier unmeasured ranges in historical records are superseded. The
combined admission passed and the forecast was presented toJohn beforebatch02.

A reviewed controller-only repair lists the local cache once, preserving exact
verification output and failed-attempt coverage;10offline tests passed and the
existing527-job result was byte-identical after a4.97s warm-cache recheck. It changes
neither frozen worker files nor engines, science, plans or historical measurements.
Do not attribute all historical22m36s post-cloud delay to scans; transfer was not
separately timed. Use existing immutable worker-source manifests for later batches.

Raw cloud evidence is authoritative inGCS and in ignored `local-cache/`. Original
retrieval and review indexes are under `batch-01-review/`;20M receipts and overlays
are under `batch-01-adjudication/`. Large metadata has byte-exact `.json.gz` archives
and expanded/hash records in `metadata-archives.json`. Restore compressed JSON
beside its archive when a script needs the expanded form; never substitute Mac
reports that happen to share the same pair identifiers.

Batch02 has passed its review and forecast gates; launch its existing frozen
20-game manifest after the recorded empty-prefix preflight. A single
`notify_exit.py` wrapper must wait on the cloud terminal event and retrieve/verify
the completed batch before queueing this chat. No timer, periodic watcher or Mac
engine execution. Inspect receipts before restarting any process.

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
