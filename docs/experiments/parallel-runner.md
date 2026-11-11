# Parallel experiment runner

MSH-162 (Parallel Experiment Runner) supplies execution for MSH-161 (Competitive
Gameplay Diagnosis). The study coordinator owns interpretation, pilot admission,
full-study launch and integration. This branch does not alter frozen study files,
production Rust, rules, search, evaluation or the 160/80 benchmark.

## Execution model

Use `python -m tools.experiments.cli --help` from the repository root. Python 3.11+
and the packages in `tools/experiments/requirements.txt` are required. Never run
`chess.py` as a script: its name would shadow the diagnostic chess package.

A version-1 JSON job hashes its full specification: experiment ID, source revision,
source-file hashes, native/opponent/reference binary hashes and source identities,
compiler/flags, environment, input references, output contract, opening/history or
analysis selector. Paths are explicit; the Linux image uses `/opt/engines`.
Changing scientific or runtime conditions creates different job IDs. Experiment
IDs cannot be rebound to different manifests.

A pipeline runs one reversed-color opening pair, its all-move screen, then the
frozen deterministic 5M confirmation selections. Separate pipelines run on separate
VMs. A worker never overlaps games and reference analysis. Each GCP task receives
a whole on-demand C3 machine (`c3-standard-4` initially), one task per node, with
maximum parallelism eight initially and sixteen only after measured justification.
The two engines retain the same `go movetime 5000`, native outcomes, histories and
replay. `run_batch.run_pair` and native `mshogi-match` remain authoritative.

`analysis.query`, full-history extraction, `summarize_analysis`, `screen_v1`, and
phase tagging are reused unchanged. The adapter preserves both players, prepared
prefixes, WDL/score kind/perspective/bounds, raw UCI, PV legality, uncertainty and
v0/v1 summaries. Confirmation selection duplicates the short frozen main_batch
selection policy; review tests cover it. Extra 20M adjudications are explicitly
specified confirmation jobs, never automatic scientific findings. No score is
reinterpreted as a probability against the opponent.

Local execution uses the same definitions and store protocol. Multiple Linux
workers use separate processes and disjoint physical-core groups (SMT siblings
stay together). macOS uses one worker because reliable CPU affinity is unavailable.
A host lock prevents two local executor instances from overlapping. Unrelated
host workloads are still outside our control; local results are exploratory.
Other games can add adapters around their existing native protocol/replay; the
storage, identity, allocation and retry services contain no game rules. They are
not claimed implemented for other variants.

## Quick progress report

Use `quick-status` for a progress check. It lists saved completion markers and
reads confirmation plans and possible failed-attempt records concurrently. It
does not download game or analysis artifacts, verify results, or run engines.

```sh
python -m tools.experiments.cli quick-status --manifest docs/experiments/pilot/eight-pair-manifest.json --store gs://mshogi-experiments-216675633801/msh161 --job-name msh162-pilot-eight-v1
```

Use the Python environment with `requirements-controller.txt` installed. The
current local environment is `/private/tmp/msh162-controller/bin/python`.
Add `--json` for machine-readable output. Omit `--job-name` to read saved progress
without a Batch lookup. Local storage paths also work.

The report shows completed games, pairs, screening jobs, confirmation queries,
known confirmation totals, claims, and failed attempts. Confirmation totals grow
as workers publish plans. A claim can belong to a running or interrupted worker.
Counts can change during the read. They are progress records, not verified results.
Use `verify` for a full artifact check. The older `status` command also performs
result checks and can take longer. The old `tools/study_status` scripts have been
removed; they supported only the former local study layout.

## Linux artifacts and image

Build native executables from frozen source
`77101fe3d38712361af8dd99b3ca2bcc6b5c17d3` in a clean **Linux x86_64** clone,
using Rust 1.99.1, locked Cargo dependencies, release profile and explicit portable
flags (record the exact flags; do not silently use the Mac executable or `native`).
The existing command is:

```
cargo build --release --locked -p mshogi-console --bin mshogi -p mshogi-runner --bin mshogi-match
```

Build Oxidation from its frozen source
`46a705b80e7191e7c302838eb8b85053b5994d65` (`cargo build --release --locked -p oxidation`).
Use the same Stockfish19 release/source and networks as the frozen study, with a
recorded Linux build target. Record source archive/network hashes when no Git
revision exists. The compiler/base image must be pinned by digest, not merely tag.
Do not copy a mutable system Stockfish into the study.

Create `linux-build.json` (outside Git for machine-specific paths) with:

```json
{
  "source": "77101fe3d38712361af8dd99b3ca2bcc6b5c17d3",
  "compiler": "exact observed rustc/compiler versions and build image digest",
  "build_flags": "exact Rust and Stockfish build flags",
  "architecture": "x86_64",
  "system": "Linux",
  "engines": {
    "candidate": {"path":"/opt/engines/mshogi","sha256":"64 observed hex digits","source":"77101fe3d38712361af8dd99b3ca2bcc6b5c17d3"},
    "runner": {"path":"/opt/engines/mshogi-match","sha256":"64 observed hex digits","source":"77101fe3d38712361af8dd99b3ca2bcc6b5c17d3"},
    "oxidation060": {"path":"/opt/engines/oxidation","sha256":"64 observed hex digits","source":"46a705b80e7191e7c302838eb8b85053b5994d65"},
    "stockfish19": {"path":"/opt/engines/stockfish","sha256":"64 observed hex digits","source":"verified frozen Stockfish19 source/archive and network identity"}
  }
}
```

Additional compiler/build receipt fields are retained in each job. The native
runner also embeds its actual compiler/profile/target/revision and fingerprints
its participants. The pair audit rejects dirty or wrong-revision native builds.
Linux and Mac hashes will differ: compare source, options, architecture and flags,
then measure actual wall time/throughput; never pool Mac and cloud results.

Prepare a staging manifest without cloud arguments; package it with
`python -m tools.experiments.package --manifest staging.json --binaries BINARY_DIR --output CONTEXT`.
This copies only frozen inputs, executor source and verified binaries, not live
Mac results. Build with the supplied Dockerfile:

```
docker build --platform linux/amd64 --build-arg PYTHON_BASE=python:3.13-slim@sha256:VERIFIED_DIGEST -f CONTEXT/tools/experiments/Dockerfile -t REGISTRY/IMAGE:BUILD CONTEXT
```

Push only to the owner-approved registry and record the returned image digest.
Then prepare the final manifest with that digest and approved cloud config. The
image does not embed the final manifest, avoiding a recursive image hash.

## Pilot and normal commands

Before **any billable action**, obtain project, region, credentials/service-account,
bucket/registry and spending authorization. No setup or IAM changes are automatic.
Use Application Default Credentials; never commit or print credentials. The worker
needs GCS get/list/create and conditional deletion of its own `claim.json` objects;
restrict deletion permissions to claim objects. Result writes use
[create-only generation preconditions](https://docs.cloud.google.com/storage/docs/request-preconditions).
The submitter needs Batch creation/status/deletion and service-account use, plus
access to the chosen private image. The Batch service agent must be enabled.

Cloud config example (placeholders must be replaced):

```json
{"project":"APPROVED_PROJECT","region":"us-central1","store_uri":"gs://APPROVED_BUCKET/msh161",
 "service_account":"WORKER@APPROVED_PROJECT.iam.gserviceaccount.com",
 "machine_type":"c3-standard-4","parallelism":8,"max_run_seconds":21600,"max_retries":1,
 "boot_disk_gb":30}
```

The six-hour per-attempt limit is an operational failure bound, not a shortened
scientific game. A timed-out/incomplete result never counts as a draw or loss.
Increase a bound only through a new frozen execution configuration if the pilot
shows it is insufficient. Every required opening remains scheduled.

```
python -m tools.experiments.cli prepare --build linux-build.json --experiment msh161-cloud-pilot --source-revision FULL_INFRA_COMMIT --pilot-pairs 1 --cloud-config cloud.json --image REGISTRY/IMAGE@sha256:DIGEST --output pilot.json
python -m tools.experiments.cli submit --manifest pilot.json --store gs://BUCKET/msh161 --config cloud.json --image REGISTRY/IMAGE@sha256:DIGEST --job-name msh161-pilot-01 --spending-approved
python -m tools.experiments.cli cloud-status --manifest pilot.json --store gs://BUCKET/msh161 --config cloud.json --job-name msh161-pilot-01
python -m tools.experiments.cli status --manifest pilot.json --store gs://BUCKET/msh161
python -m tools.experiments.cli retrieve --manifest pilot.json --store gs://BUCKET/msh161 --output retrieved
python -m tools.experiments.cli verify --manifest pilot.json --store retrieved
python -m tools.experiments.cli report --manifest pilot.json --store retrieved --hourly-rate APPROVED_OBSERVED_RATE
```

First validate one complete pair, native replay, every move, selected confirmations,
retrieval, interruption recovery and no recomputation of successful jobs. Then
prepare an eight-pair pilot (`--pilot-pairs 8`) to measure eight concurrent workers.
Use a distinct experiment ID and retain all pilot costs/failures. Sixteen workers
requires measured throughput/allocation justification and spending authorization.
The eventual primary manifest omits `--pilot-pairs` and includes **all** 100 frozen
pairs/200 games; coordinator review is required before submission. Pilot results
are not silently merged into the primary cohort.

## Exit notification without polling

For an unattended cloud run, pre-create an owner-approved Pub/Sub topic and a
**dedicated subscription** before submitting; add `notification_topic` (full
`projects/PROJECT/topics/TOPIC` name) to the frozen cloud config. Grant the job worker
service account publish access and the local completion consumer subscribe access.
Install `tools/experiments/requirements-controller.txt` in the local control
environment (it pins google-cloud-pubsub); receipts record resolved package versions. Then start `wait-cloud` with the same manifest/store/config/job
arguments plus `--subscription projects/PROJECT/subscriptions/SUB --thread CHAT_UUID`.
It performs one initial status lookup, blocks on streaming terminal notifications,
checks the authoritative terminal state, records the completion, and queues
`codex queue --thread ... --message ...` on success or failure. There is no periodic
watcher or guessed wake time. Local `run`/`resume` likewise require `--thread` and
queue on exit. Notification acceptance/failure is recorded separately from execution.
Use normal local access escalation for Codex database access; never change policy.
A local listener crash requires reattaching it; the pre-existing subscription retains
events, and the initial lookup handles an already-complete job. Do not share the
subscription with another job consumer (unrelated messages are acknowledged).

## Failures, recovery, cancellation and cost

Every attempt has a start record, environment, elapsed/child CPU use, output hashes
and receipt. Success is published only after adapter completeness, native replay
and required outputs. GCS generation-zero writes and local atomic links prevent
replacement. Existing successful outputs are hash-verified before reuse. A failed
or incomplete attempt cannot supply a dependency or a game result. Corrupt successes
stop execution; they are not silently overwritten or rerun.

Claims exclude simultaneous execution of one job. Ordinary failures release their
own claim and Batch can retry them. SIGTERM reaches frozen child cleanup. SIGKILL,
VM loss or a storage outage can leave a claim; it remains visibly
`running_or_interrupted`. First establish from Batch/task records that the owning
worker has stopped, then use `recover --job JOB_SHA --workers-stopped` with the
same manifest/store arguments. This preserves a reconciliation receipt and removes
only the matching claim. Resubmit with a fresh Batch job name. Completed jobs are
verified and skipped; incomplete jobs restart in fresh immutable attempt directories.
Successful queries inside an incomplete screening job may be repeated on retry;
that extra work is retained/accounted, never merged into a successful result.

`cancel` writes a durable stop-before-next-job marker, allowing active work to
finish safely. `cloud-cancel` requests Batch deletion for urgent cancellation;
its returned asynchronous operation is **not proof** that workers have stopped.
Inspect terminal cloud state before recovering a claim. Preserve the cancellation
marker; resuming a deliberately cancelled study requires a new experiment ID.

The report estimates active compute cost from recorded attempt time and an
explicit rate. Startup/idle VM time, storage, logging, network and abrupt unfinished
attempts require Cloud Billing/Batch records. Report both estimated and actual billed
cost separately; never label a mock/local measurement cloud throughput. Batch
releases job VMs at termination; GCS evidence remains and continues incurring storage
cost until deliberately retained/removed under owner policy.

## Verification and integration

See `verification.json` and `review.md` for exact source/checks and outstanding gates.
Level 2 targets macOS and Linux process/storage behavior. Linux ARM container tests
do not establish Linux x86_64 C3 behavior. No desktop/GUI or other-game campaign is
needed for this execution-only addition.

Coordinator integration is a normal cherry-pick/merge of the reviewed isolated
branch after its evidence is accepted. Added paths are `tools/experiments/`, this
documentation directory, and the MSH-162 task. There are no modifications to frozen
MSH-161 inputs or native crates. Keep MSH-162 unfinished until the authorized cloud
pilot and required review gates pass; do not claim improved study throughput before
measuring it. The isolated worktree remains for this explicit ongoing purpose.
