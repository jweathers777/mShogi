# Cloud runtime admission review

Independent bounded review, execution_review, 2026-11-12. Read the isolated runner handoff, verification/review records, eight-worker runtime/scientific evidence and relevant prepare, pipeline, claim, recovery and Batch allocation source. No cloud jobs, searches, configuration changes or independent rerun of the reported tests occurred in this review.

**Recommendation: admit the tested eight-worker route for the newly authorized full cloud study.** No unresolved runtime defect in the inspected evidence requires rebuilding the worker or another pilot. The latest owner instruction to proceed supersedes the handoff's historical pilot-only authorization; US$30 is not silently repurposed as either a full-study authorization or a new hard cap. Preserve that history and record the new authorization with the actual submitted plan.

## Concrete execution route

1. Integrate the reviewed controller/infrastructure branch while preserving primary evidence and owner instructions. Keep the tested worker source separately addressable: `51351c48ea1adf9ac96011aec1440e44ceb741a4`.
2. Prepare a new experiment manifest from that exact source snapshot and `docs/experiments/linux-build.json`, using the tested C3 configuration and pinned image:
   `us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:ad95d85a8f555acaaef2adafecb92b0277983fb46ce6da83c3c192267dcc43f7`.
   Omit `--pilot-pairs`; verify 100 pair pipelines / 200 static pair-screen jobs, 90 prepared and ten unaided pairs, exactly the frozen opening/color schedule, before submission. Use a fresh experiment identity; retain pilot and Mac data separately.
3. Submit with the reviewed newer controller/listener, the same manifest/image/config/store identity, parallelism eight, `c3-standard-4`, one task per VM. Each pipeline plays one pair, then screens it and executes its selected 5M jobs serially. Do not increase to sixteen workers or alter the scientific policy for throughput. Attach the event listener and owning-chat exit notification.
4. At completion retrieve immutable artifacts, verify every static and derived obligation and receipt, reconcile failures/retries and VM/disk cleanup, and independently review completed batches. Preserve required consequential 20M adjudication, missed-error audit expansion, episodes, strong-play examples and final per-decision dispositions: the automatic pipeline ends at selected 5M queries and is not the complete diagnosis by itself.

**Actual launch blocker to avoid:** preparing from newer integrated Python files and submitting to the old image. `make_manifest` hashes all top-level experiment Python files; `run_job` checks their exact bytes. A different controller does not require a different worker, but a manifest built from that controller's changed files will be rejected by the old worker. Prepare from the exact worker snapshot (including its source revision), then use the newer controller for submit/listening. If worker changes become necessary, rebuild and verify the real new registry digest and manifest; do not loosen the checks. The controller project-number event fix is needed for reliable live notification and is already separately validated.

## What is actually validated

The eight-worker evidence contains 16 native games, 5,322 initial queries and 1,825 deeper queries. All 1,841 jobs passed retrieved-artifact verification; independent scientific review replayed histories and regenerated screening/selection. All eight workers were Linux x86_64 C3-standard-4 on Sapphire Rapids with the same build/source. No failed pilot jobs, retries or remaining claims were recorded; deletion audits and empty VM/disk listings support cleanup. These are execution checks, not a strength result or proof of arbitrary cloud hardware equivalence.

Recorded task events span 6,159.163 seconds; all eight tasks overlapped for 2,535.357 seconds. Job wall sum is 30,815.975 seconds and audited VM lifecycle is 9.37245 hours. Use task/receipt/server/VM observations: aggregate Batch `runDuration` and its earlier terminal timestamp are inconsistent. The final notification arrived after the actual last task. Do not use the aggregate timestamp alone to claim every artifact complete, or turn wall-sum divided by elapsed into a measured serial/Mac speedup.

The actual recovery probe retained one deliberate application failure, completed a real query on retry, then reused immutable success without reexecution. Abrupt VM loss/stale claims were tested locally, not induced on GCP. A stale claim therefore requires evidence the owning worker stopped and explicit recovery; `--workers-stopped` is an operator assertion, not an automatic liveness proof. Preserve failed-attempt accounting. A scheduler retry alone does not clear a stale claim. Immutable success validation and generation-conditional claim removal are meaningful safeguards; do not delete results or replay successful jobs to simplify recovery.

The reported suites cover 50 tests on the exact packaged x86 worker and 64 tests on the later controller on Mac/Linux ARM. These are cited prior verification, not tests independently rerun here. No unrelated desktop/Windows campaign is needed for this bounded cloud admission.

## Measured-resource extrapolation, not a quote or cap

Using only the recorded eight-pair observations and rate US$0.201608/VM-hour:

| Quantity | Eight-pair measurement | Linear 100-pair illustration |
| --- | ---: | ---: |
| Job wall work | 8.560 hours | 107.000 worker-hours |
| VM lifecycle | 9.37245 hours | 117.156 VM-hours |
| Estimated VM compute | US$1.88956 | US$23.6195 |
| Retrieved artifact bytes | 3,980,915,946 | about 49.76 GB decimal |

At sustained eight-way occupancy the scaled lifecycle work corresponds to about 14.64 hours; this is an ideal occupancy illustration, not a promised finish time. Game lengths, selected-query counts, provisioning, storage operations, long-tail tasks and retries change both elapsed time and cost. Scaling the observed longest wave instead would be more pessimistic; neither is a controlled full-study measurement.

The estimate excludes disks, retained storage, object operations, network/egress, logs, registry, taxes and later 20M/adjudication work. Actual billing remains unreconciled. Do not describe US$23.62 as the full study's bill. No new price lookup was performed; the calculation uses the recorded pilot rate, not a claim of today's tariff.

The inspected configuration has a six-hour task bound and one retry. Across 100 tasks, 600 task-hours without retries or 1,200 with one full retry each correspond arithmetically to roughly US$120.96 or US$241.93 at the recorded compute rate, before VM overhead and other charges. Those are configured task-time illustrations, not likely costs or hard billing ceilings. Eight-way concurrency limits simultaneous allocation, not total spending. Record the concrete bounds, retain actual usage/failures, and respond to meaningful exceptional runtime/cost evidence; do not invent a fresh arbitrary spending cap or treat budget alerts as enforced caps.

## Scientific and integration guards

The full cloud cohort should be a fresh, explicitly identified 200-game primary execution. Preserve the 40 Mac main games as separate exploratory evidence and the 18 cloud pilot games as execution validation; do not silently count either toward the new primary denominator. Keep source/opening selection independent of observed outcomes. Verify native time commands, worker/hash policies, no book/ponder/handicap, history/claim semantics, native replay and actual machine metadata throughout. A machine-type guard does not itself assert every observed CPU platform is identical; retain and review those records.

Finite Stockfish bounds and missing aligned scores remain uncertainty, not execution failures to be discarded or successful-game substitutes. Native auto-claim outcomes remain authoritative even when a self-play continuation estimate differs. Report absent/failed attempts explicitly and retain grouped opening/game attribution; parallelism does not increase the independent sample size.

The handoff's integration boundary preserves native crates and frozen MSH-161 inputs. Use its existing runner and adapters; no competing runtime or production engine change is justified by this admission. After integration, record the exact controller commit, exact worker image/source and submitted manifest separately. The coordinator can now carry out the authorized study through this route without another permission loop, while retaining the stated recovery, cost and scientific limitations.
