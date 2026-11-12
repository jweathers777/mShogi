# MSH-161 (Competitive Gameplay Diagnosis): primary cloud study

## Current state: Batch 04 required 20M plan launched

Batches 01–03 are fully reviewed: **60 games, 26 wins / 17 draws / 17 losses**.
Batch 03 finished 9/5/6. Independent gameplay panels and execution review closed
its 692-query 20M panel and seven new alternatives. All required coverage passed;
unstable comparisons remain unresolved. See `batch-03-review/closure.md`,
`closure-summary.json`, the final disposition overlays in `batch-03-adjudication`,
and the independent reviews in `batch-03-followup`. Pre-followup ledgers and the
exact former 40-game cumulative record remain preserved. No searches were repeated.

`primary-cumulative.json` includes all 60 games, earlier/linked uncertainty,
secondary tags and unranked category examples. Selected comparisons are correlated
and do not establish population error rates or independent loss causes. All 17
draws were native automatic claims (16 threefold, one fifty-move). Draws alone are
not weakness evidence; claims can limit observation of advantage conversion.

John restored the original AutoClaim conditions. All remaining 140 games use the
original unmodified mShogi/Oxidation binaries, worker image and cloud configuration.
`autoclaim-restoration.json` and `batch-04-offline-preflight.json` bind those inputs.
The policy transition is abandoned; preserve its failed validation without further
policy work, opponent changes or production engineering. The historical sections
below do not govern current execution.

Batch 03 followup submission, cloud completion, plan-aware verification, cleanup
and notification were independently reconciled. All seven workers were deleted;
no failed attempts or allocation failures occurred. Primary cumulative compute is
43.042446 VM-hours, estimated US$8.677702, not billed/all-in cost. See
`batch-03-followup/forecast-update.json`; the measured full-study compute estimate
is about US$28.93, with ancillary costs and runtime uncertainty retained separately.

Original Batch 04 completed under `msh161-primary-b04-v1`; retrieval and artifact
verification passed for 2,223 jobs / 32,263 objects (4,044,009,484 bytes). Submission,
terminal and retrieval hashes match; the wrapper returned success, settled its
child group and queued the accepted exit notification. See
`batch-04-review/coordinator-reconciliation.json` and the separate receipts.
Independent gameplay panels and native/query/runtime audit are now in progress.
There are 80 played games, of which 60 are fully reviewed; Batch 04 is not yet
admitted into the scientific cumulative record. Keep this distinction explicit.

Independent gameplay and integrity review passed. Native Batch 04 results are
7 wins / 6 draws / 7 losses (prepared 5/6/7; unaided 2/0/0). Three draws are native
dead positions and three automatic threefold claims. All 5,604 initial queries and
2,203 matched 5M queries are accounted for, with no stable missed-error audit trigger.
Uncertain audit comparisons remain explicit. All five actual workers were deleted;
53 resource-pool insertion failures and 100 not-found cleanup notices on failed
allocations are preserved separately from zero scientific failures. Actual peak
concurrency was five, despite eight requested workers. Runtime/cost/forecast is in
`batch-04-review/runtime-integrity-review.md/json`: US$10.72 primary compute so far,
about US$29.07 projected full compute; billed and ancillary costs remain unknown.

The frozen 20M plan in `batch-04-adjudication` contains 480 selected roots,
765 jobs / 793 queries, including 28 unrestricted discoveries. It covers both panels'
consequential and linked comparisons, episode boundaries and contrary-success
controls. No selection is a confirmation. Full decision ledgers, review index and
inventory are losslessly archived as `.json.gz`; their binding manifests preserve
original bytes/hashes. Restore locally if needed; do not download again.
The prepared plan was launched once through `notify_exit.py`, PID 38445, from
reviewed commit `20c70895`; job name `msh161-primary-b04-20m-v1`. See
`batch-04-adjudication/launch.json`. The wrapper performs provenance preflight,
submission, event wait, retrieval and plan-aware verification; actual cloud
acceptance remains separate in `submission-result.json`. Do not start another
wrapper, submit again, or repeat its queries. On exit, reconcile acceptance,
completion, verification, cleanup and notification separately. Review all
selected comparisons against the independent narratives, preserve earlier and linked
uncertainty/secondary tags, and obtain only distinct new alternative obligations.
Close Batch 04 before launching original Batch 05. The scientific cumulative record
remains at 60 fully reviewed games until this gate closes.
Do not duplicate retrieval/searches, run Mac engines, use timers or periodic polling.
Full 200-game coverage remains required. Stop for John after the independently
reviewed diagnosis and roadmap, before engineering. No final priorities yet.

## Historical transition records (superseded)

John authorized the full cloud study on12November2026 after reading the runner
handoff. The coordinator admitted the existing pilot evidence and integrated
MSH-162 (Parallel Experiment Runner) at `bb5e698e`. All64 local integration tests
passed. Independent runtime/scientific reviews are in `../migration/`; the actual
manifest review is `manifest-admission.json`. No new pilot was needed.

## Current blocking compatibility finding

The corrected image was published, but the short actual cloud validation FAILED.
Unchanged Oxidation ends internally at50moves/threefold and cannot continue under
the requested controller policy. mShogi returned a searched legal move; native
replay passed. See `policy-transition/oxidation-draw-policy-review.md` and the
preserved validation report. No retry or batch03 is running. John has been asked
to select a narrow reference draw-policy patch or a compatible replacement.
Await this material reference decision; do not activate staged manifests that
still bind the incompatible reference. This gate supersedes older continuation
instructions below. All40completed games and the failed validation are preserved.

## Current owner-authorized policy transition

The12November draw-policy direction supersedes the AutoClaim freeze for future
batches. Batch02 and its two-query followup are complete; no workers remain and
batch03 has not started. The40completed primary games remain authoritative under
their original conditions. Their12optional-claim draws are potentially affected.
Do not replay or discard them. See `policy-transition/owner-direction.md`.

Before batch03: verify the bounded search/UCI/runner correction, build corrected
Linux binaries and pinned worker image, run one short actual two-engine match,
and admit amended manifests for the remaining160games. Opening pairs, equal5s
thinking time and full reference coverage remain unchanged. Future results form
a separate policy/source stratum; do not pool draw/conversion rates as uniform.
Earlier continuation instructions below are historical and cannot bypass this
deployment gate. No unrelated engineering is authorized.

## Original frozen primary sample

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

## Completed batches and current deployment gate

Batch02 review is now closed, including both distinct20Mfollowups and independent
cross-reviews. `batch-02-review/closure.md` and `primary-cumulative.json` record
40games:17wins,12draws,11losses. `policy-transition/policy-strata.json` binds all
native records and flags11threefold/one50-move claim draws. The110supported
candidate comparisons are selected correlated events, not population error rates
or final priorities. Earlier/linked uncertainty remains.

Reviewed correction commit: `ab7ef8f10d9b9023dbe525fcbc47e54a40c423e0`.
The bounded Linux build succeeded through `policy-transition/build_linux.py`
and the exit notifier; see its launch/state/notification records. Do not duplicate
it. Native binary hashes and wrapper exit/notification passed. Build/push the worker image, perform the
short cloud two-engine validation, admit amended manifests and continue batch03.
No new owner approval is required. Preserve the original40games and all failures.

The older batch narrative below is retained for provenance. Its pending-followup
and frozen-policy continuation wording is superseded by this current gate.

The Linux build is verified. One image-push/short-cloud-validation chain is now
running from worker source `d9271a294cb92e1e87d366d49dfd409bbfd662b6`; see
`policy-transition/deployment-launch.json`. Its exit callback owns the next review.
Do not duplicate it or start batch03 before success/admission.

### Batch01 and historical batch02 execution records

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

Batch02 completed successfully as `msh161-primary-b02-v1`, UID
`msh161-primary-b02-fff81736-b9df-43bd0`. Its original frozen manifest is unchanged.
The combined event/retrieval wrapper returnedzero, settled its own process group
without a residual, and successfully queued this chat. Computation, cleanup and
notification are independently recorded in `batch-02-notification.json` and
`batch-02-wait-retrieve.json`; preserve those receipts. Do not duplicate retrieval.

Retrieval verified2,376jobs,34,751objects /4,441,259,122bytes in443.25seconds.
Independent audit passed8,612queries (6,256initial500k plus2,356matched5M), all
2,840played+288prefix decisions and exactly frozen selection/audit coverage. Native
result9W/7D/4L, with six threefold and one fifty-move automatic claims. No scientific
failures or eligible stable missed-error audit trigger; two uncertain audit
comparisons remain uncertain. Runtime:8actualworkers,1h49m02cloud elapsed,
10.90506VM-hours/~$2.19855compute, no failed VM insertions and verified cleanup.

Both gameplay panels and cross-reviews passed. The frozen consequential20M plan
is `batch-02-adjudication/plan.json`:371decisions,627jobs/663queries,36discovery roots.
It retains linked opponent events, constructive controls, literal frozen thresholds,
earlier uncertainty and native claim-policy limits. First-half02g1p40 rose above
.05at5M but was already selected by v1-followup; it was not a missed audit-stratum
case. No absolute first/largest labels or final weakness priorities are established.

There are40completed primary games, but onlybatch01's20are reviewed through required
20M and included in cumulative scientific categories. Batch03 has not been launched.
Batch02's initial20M workflow completed successfully:627jobs/663queries, wrapper
returnzero, own process group settled and queue accepted. Independent source/hash/
history/PV audit and both gameplay cross-reviews passed. Eight actual cloudworkers,
no allocation failures, all VMs/disks deleted. The panel used4.400461VM-hours,
estimated$0.88717compute. The two completed base batches plus their initial20M
panels total29.369163VM-hours/~$5.92106compute, not actual billing. See
`batch-02-adjudication/runtime-integrity-review.*` and `forecast-update.json`.

Two further singleton alternatives are required beforebatch02closure:
- main-02-02 game0 ply65: forceg8g7, from unrestricted20M discovery; reuseplayedf6f8.
- main-02-08 game0 ply96: forcec5e4, from the newly clarifiedp95continuation;
  reuseplayedf4f5. The originalf5/Ke4 comparison cannot settle this new alternative.

Both are legal at the exact full-history roots and independently reviewed.
`batch-02-followup/plan.json` (SHA begins5fa4df35) adds onlytwoqueries and binds the
existing equal-work played receipts and supportingp95evidence. It uses twoC3tasks,
not eight with empty assignments. The small controller-only change allows1..8
nonempty bins; independent tests/admission passed. Frozen engine/worker code,
scientific thresholds and original completed plans remain unchanged.

The full20M overlays and detailed legal continuations are under
`batch-02-adjudication/`. Earlier/unqueried uncertainty, supersession, sevennative
claim draws and strong play remain explicit. In particular02g0p65's existing
same-choice control has a pending new comparison; it is not wholly settled.
Batch03 has not been launched; cumulative categories still cover20fullyreviewed
primarygames while40games have completed. The two-query followup is now running under exit-wrapperPID81834, targeting
`msh161-primary-b02-followup-v1`; checkpoint `8b0e99df`. Exact command/hashes are
in `batch-02-followup/launch.json`; cloud UID/acceptance appears in
`submission-result.json` after driver submission, and computation/cleanup/queue
states are separate in `notification.json`. Do not duplicate the wrapper, queries
or retrieval. On its exit review the two results with reusedplayed evidence,
update primarycumulative/resources and closebatch02, thencontinuefrozenbatch03.
Use the same single exit-notification workflow; no timer, polling, Mac engine or
production engineering. Preserve full200coverage and all failure/provenance records.

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
Its settings point to the completed primarybatch02 base graph; they do not enumerate
its additional20M plan. For that stage inspect the adjudication submission/plan
receipts and exact Batch job identity. It is a read-only snapshot, not an
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
