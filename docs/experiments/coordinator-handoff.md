# Parallel runner handoff

MSH-162 (Parallel Experiment Runner) is on `codex/parallel-experiment-runner`.
The branch starts at `c01d54bf`. The study coordinator owns primary integration
and admission of cloud evidence into MSH-161 (Competitive Gameplay Diagnosis).
No full cloud study has started.

## What the branch supplies

The runner reuses the native match/replay tools and the frozen pair, Stockfish
query, screening, and phase helpers. It adds versioned job identities, immutable
local/GCS results, per-attempt failure records, dependency execution, bounded
local workers and GCP Batch tasks. One Batch task runs one opening pair, screening,
and selected deeper queries on one dedicated C3 VM. Separate pairs run in parallel.

The direct progress command is `python3 tools/experiments/quick_status.py`.
`status-settings.json` selects the experiment. The advanced CLI supplies prepare,
submit, status, retrieve, verify, recover, cancel, report, and completion listening.
The listener receives Pub/Sub events and queues the owning chat when work ends.
It does not poll queries or moves.

The storage and scheduler code contains no game rules. Other variants need their
own native adapters; this branch implements only the current Chess protocol.

## Failure behavior

A failed attempt cannot become a game result or a dependency. Success records and
artifacts are immutable. Retries use new attempt paths and verify completed jobs
before skipping them. The actual cloud recovery check saved one intentional failed
attempt, completed a genuine confirmation on retry, and proved that a third call
did not execute it again. An abrupt VM-loss/stale-claim recovery has local tests,
but was not induced in the cloud. Stale claims require a stopped-worker check and
explicit recovery; a Batch retry alone does not remove them.

## Measured pilot results

The initial one-pair pilot passed native replay, screening and 119 deeper queries.
The eight-worker pilot completed 16 games and 1,825 deeper confirmation queries.
All 1,841 jobs passed full local integrity checks after retrieval of 27,401 objects
(3,980,915,946 bytes). Scientific review details are recorded in the linked evidence.

The eight pipelines took 6,159.163 seconds (102.65 minutes), from the first task start
to the last task success. All eight tasks overlapped for 2,535.357 seconds. Their
recorded job wall time totals 30,815.975 seconds (8.56 worker-hours). This demonstrates
parallel work; it is not a measured comparison against a serial run or the Mac.

Batch's aggregate runDuration reports 5,180.046 seconds and an earlier success
timestamp. The last task event, its GCS server timestamps, and VM audit records
agree on the later finish. Use those records for timing. The completion listener
queued its success message after the actual final task had finished. The cause
of the aggregate timestamp discrepancy is not established.

All eight workers used the same Linux x86_64 build, `c3-standard-4` machine type and
Intel Sapphire Rapids CPU platform. There were no failed pilot jobs or Batch
retries. The earlier bootstrap submission failure and deliberate recovery-test
failure remain in the evidence. All pilot VMs and disks have been removed.

Estimated VM lifecycle compute is US$1.88956 for the eight-worker run and US$2.04140
for all pilots combined. These are estimates from audited VM times and the
recorded rate, not billed totals. They exclude disks, storage, reads/egress, logs,
registry, tax and other charges. Actual billing reconciliation remains open.

See `verification.json`, `review.md`, `pilot/eight-runtime-review.json`, and
`pilot/eight-scientific-review.json`.

## Integration boundary

Integrate only after the coordinator has finished or safely paused its current
work and accepted this handoff. Preserve its dirty and untracked evidence. Do not
copy the pilot records into frozen Mac scientific evidence or combine cohorts.

The changes are:

- Add `tools/experiments/` and `docs/experiments/`.
- Add the MSH-162 task record and its verification evidence.
- Preserve the owner language preference added to `AGENTS.md`.
- Remove the four old `tools/study_status` files, as requested by the owner.

No native crate or frozen MSH-161 source/input file differs from the branch base.
The coordinator can merge the isolated branch or cherry-pick its ordered commits.
Resolve documentation/task conflicts against current coordinator state. Do not
replace its AGENTS.md wholesale. Keep its scientific evidence untouched.

The worker image remains pinned to source `51351c48ea1adf9ac96011aec1440e44ceb741a4`:
`us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:ad95d85a8f555acaaef2adafecb92b0277983fb46ce6da83c3c192267dcc43f7`.
Later controller changes fix project-number event matching and simplify status.
Do not prepare a manifest from newer source files and submit it to the old image:
source hash checks will refuse it. Prepare from the exact image source, or package
and verify a new image, then freeze its real registry digest in the manifest.

## Before any full study

The coordinator must review the pilot evidence and decide how the standardized
Linux cohort enters the study. Mac and Linux timing/results are separate. The pilot
is execution validation, not a strength measurement or completed diagnosis.

Obtain a spending scope for the full study. The existing US$30 approval covers
only the initial pair, recovery check and eight-worker pilot. It does not authorize
the 200-game study, sixteen workers, or additional spending. Budget alerts are not
hard caps. Include VM overhead, disks, storage, reads/egress, logs and registry costs.

Cloud Storage evidence and the registry image remain retained. They can incur
small ongoing storage charges. Do not delete evidence as routine cleanup.
The isolated worktree remains for coordinator review and integration; it has not
been merged, removed, or archived.
