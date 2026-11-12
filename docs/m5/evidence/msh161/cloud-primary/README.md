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

Primary batch01 submitted successfully as `msh161-primary-b01-v1`, UID
`msh161-primary-b01-c12991aa-17f5-43260`. Its manifest is
`batch-01-manifest.json`; the submission response is `batch-01-submission.json`.
The submission response was QUEUED; one startup check then reported SCHEDULED
(`batch-01-initial-status.json`). No completed games or running-worker count is
inferred from these startup states.

The event listener started as PID6829; command and identity are in
`batch-01-listener.json`. It uses the existing subscription
`projects/mshogi/subscriptions/mshogi-pilot-completion`, after confirming that the
prior consumer and all pilot workers had stopped. Only one job consumer uses this
subscription. The listener makes one initial lookup, then waits for Pub/Sub terminal
events and queues this coordinator chat on success or failure. It does not poll
moves or run local engines. Notification acceptance will be recorded separately.

On the completion message:

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
