# MSH-161 (Competitive Gameplay Diagnosis): forecast before cloud batch02

Estimate as of12November2026, after20primary cloud games and their required20M
review. The full200-game study remains authorized. These are planning ranges,
not a bill, deadline, statistical confidence interval or compute cap.

| Scope | Processing, retrieval and verification | Cloud cost before tax |
| --- | ---: | ---: |
| Full200games | 29–48hours | US$35–90 |
| Remaining180games | 26–43hours | US$32–82 |

The measured batch01 workload repeated ten times gives33.1processing hours and
US$28.35VM compute alone (remaining29.8hours andUS$25.52compute). The inclusive
cost ranges add storage, data transfer, operations, disks, addresses, registry
and logs, plus workload/recovery sensitivity. Actual billed charges are not yet
available. Previous pilots and Mac computation are excluded from this primary
cohort estimate; ongoing retention after the assumed month costs extra.

Review adds elapsed time without keeping completed workers running. Nine
one-hour review windows would add nine hours. Allowing an additional2–4hours
for final synthesis and independent report review gives a conditional full-study
calendar estimate of roughly40–61hours, or37–56hours for the remaining work.
These review allowances are assumptions, not measured service guarantees. Owner
availability, longer reviews or regional capacity shortages can extend them.

## Measurements and assumptions

- Base20games plus500k/5M analysis:2h19m from cloud submission to completion,
 10.2458VM-hours, estimatedUS$2.0656. Actual peak concurrency was6of8requested;
 preserve53failed VM insertion operations, distinct from scientific attempts.
- Consequential20M:558queries/298selected decisions,29m56s cloud elapsed,
 3.8178VM-hours, estimatedUS$0.7697; eight actual workers. Mean query21.184s.
 All31discoveries returned the played move, so this panel needs no extra
 discovered-alternative singleton. Later panels may need them.
- Initial retrieval/verification:6m53s. After20M cloud completion:22m36s for
 event handling, retrieval, verification and publication together. Do not call
 the latter download time. The conservative forecast retains it.
- A controller-only scan repair produced the exact same verification bytes in
 4.97seconds on warm local data. It removes repeated cache scans; it does not
 measure future network or end-to-end savings. No engine/worker changed.
- Full base-plus20M artifacts scale to51.55GiB. One complete Internet copy at the
 recorded Europe tariff is aboutUS$6.19; one full month of storage aboutUS$1.03.
 Repeated downloads, dependency reads, extra panels and retained failures add cost.
- The range permits later20M panels at0.5–2times this workload,0–25% extra compute,
 and the documented50–200GiB transfer /50–100GiB-month storage and request-cost
 scenarios. No free allowance is assumed. These are sensitivities, not bounds
 on scientific coverage or unavailability. The recorded C3 rate isUS$0.201608/VM-hour.

The full arithmetic, ancillary price sources and resource records are in
`batch-01-runtime-review.md/.json` and
`batch-01-adjudication/runtime-integrity-review.md/.json`. Provider sources include
[Cloud Storage pricing](https://cloud.google.com/storage/pricing),
[Compute disk pricing](https://cloud.google.com/compute/disks-image-pricing), and
[VPC network pricing](https://cloud.google.com/vpc/network-pricing).

Cloud processing was faster than the earlier Mac batch in observed elapsed time,
but the end-to-end study also includes retrieval, deeper checks and independent
review. The earlier Mac base batch took6h54m versus2h19m for this cloud base batch;
different games, query workloads and hardware prevent treating this as a
controlled speedup. Configured eight-way parallelism is not guaranteed availability.

## Update at the batch02 policy boundary

Two reviewed batches, both20M panels and the two distinct followup queries total
29.409754VM-hours and US$5.929242estimated compute for40primary games and17,783
reference queries. These are measured VM lifetimes priced at the recorded rate,
not reconciled bills. See `batch-02-followup/forecast-update.json` and its runtime
integrity review; reused played queries are counted only once.

Mean-workload scaling gives US$29.65compute for200games, US$23.72remaining. Retain
US$35–90full /US$30–80remaining160 as conditional inclusive planning ranges. The
new automatic-threshold-only policy can lengthen games and change which events
need deeper analysis; those effects are not yet measured. The prior29–48processing
hours is an AutoClaim-based scenario, not a forecast guaranteed for the amended
policy. Update timing/cost after the first corrected batch. Build/deployment,
short validation and review pauses are separate overhead, to be recorded when
observed. Scientific coverage remains200games regardless of these estimates.


## Batch 03 update under restored original AutoClaim conditions

Batch 03 completed its games and initial/5M analysis in 1 h 35 min 34 s from
submission to the final task, using eight actual workers. Retrieval and
verification took 305.25 seconds. All workers were deleted; no allocation
failure was observed for this batch. See `batch-03-review/runtime-review.md/json`.
The abandoned policy change no longer affects future game-length assumptions.

Primary compute so far totals 37.845525 VM-hours, estimated US$7.629961. This
covers 60 completed games and base analysis, but only the first 40 games' required
20M panels and followups. Mean workload scaling gives US$28.21 full-study compute
and US$20.58 remaining compute, including Batch 03's pending deeper analysis.
These exclude the separate transition validation and are not bills or limits.
Retain the conditional US$35–90 full-study inclusive estimate and the original
29–48 processing-hour scenario, with independent review and synthesis adding
calendar time. The conversational completion estimate remains approximately
30–60 hours from the 12 November afternoon checkpoint, conditional on continued
capacity, review workload and followup needs; it is not a deadline guarantee.
