# Primary cloud batch 01: runtime and provisional forecast

Independent read-only execution_review, 2026-11-12. Job `msh161-primary-b01-v1`, UID `msh161-primary-b01-c12991aa-17f5-43260`. This review queried completed Batch tasks, Compute lifecycle audit logs, current instances/disks and project-visible BigQuery datasets. No jobs/configuration changes or searches were performed. Exact task/VM records and observations are retained in `batch-01-runtime-review.json` and `batch-01-runtime-observations.json.gz`. Artifact integrity and scientific accounting are separate checks; complete stage receipts are now incorporated in the update below.

## Observed runtime and cleanup

All ten task pipelines report success. Submission was 06:32:55 UTC; first task ran 06:41:22; final task succeeded 08:51:57. Elapsed was **2h10m35s from first task** and **2h19m02s from submission**. Aggregate Batch duration happens to agree with task timing in this batch; task evidence remains the authority because the pilot aggregate duration was inconsistent.

Actual peak task concurrency was **six**, not the configured eight. Six tasks overlapped for 3,950.637 seconds. Ten tasks used six successfully provisioned VMs, with later tasks reusing available machines. Task duration sum was 36,467.360 seconds. VM lifecycle sum, from insert initiation to successful deletion completion, was **10.245834 VM-hours**. This includes provisioning/cleanup and is an estimate of lifecycle compute, not billing granularity reconciliation.

The listener's operational notice reports 51 VM creation errors. The bounded lifecycle log query independently found 53 failed insert completion events: 22 in us-central1-b, 12 in a, ten in f and nine in c. These are provisioning failures, **not 53 failed games or query attempts**. The counts have different observation scopes/times and must not be equated. Missing-instance delete errors occurred for unsuccessfully provisioned names; all six successful VM creations have successful deletion evidence. Current project instance and disk listings were empty. Artifact receipts must independently establish game/query failures and completed work.

At the recorded pilot rate of US$0.201608 per VM-hour, estimated batch VM compute is **US$2.065642**. This is not the actual bill and excludes storage/disks/network/logs/registry/tax. `bq ls --format=json --project_id=mshogi` returned empty output with exit zero, so no accessible export dataset was demonstrated in that project. It does not prove that no export exists elsewhere. Actual billed cost remains unknown.

Mac main batch 01 took 6h54m22s. This cloud batch's task span is about 3.17 times shorter, but this is an observed elapsed comparison, **not a controlled speedup**: hardware, concurrent processing and resulting game/query workloads differ. The eight-pair cloud pilot took 1h42m39s for fewer pairs and achieved eight concurrent tasks; this ten-pair batch experienced capacity limits and reached only six. Do not promise that a configured eight-worker limit delivers eight active workers.

## Draft forecast before batch 02

The following is a transparent workload extrapolation, not a completed end-to-end diagnosis estimate or a confidence interval. Cloud 20M adjudication has not yet been measured. The selected 20M workload and measured cloud20M rate must update this draft before presenting it as the final forecast.

| Games + initial screening + selected 5M | Full 200 games | Remaining 180 games |
| --- | ---: | ---: |
| Linear primary-batch-01 lifecycle | 102.46 VM-hours | 92.21 VM-hours |
| VM compute at recorded rate | US$20.66 | US$18.59 |
| Linear prior eight-pair pilot lifecycle | 117.16 VM-hours | 105.44 VM-hours |
| VM compute using pilot workload | US$23.62 | US$21.26 |
| Serial batch elapsed, primary submission-to-exit repeated | 23.17 hours | 20.85 hours |
| Pilot wave scaled proportionally to 20 games/batch | 21.39 hours | 19.25 hours |

Thus the two observed workloads suggest roughly **21–24 processing hours and US$21–24 VM compute** for the base full cohort, or **19–21 processing hours and US$19–22 VM compute** for the remaining base cohort, before adjudication, reruns and human/agent review pauses. These observed-base spans are not upper bounds. Ten-batch boundaries, unavailability and task tails make ideal eight-way workload division misleading.

For illustration, 25% extra base lifecycle work from longer games, selected-query variation or reruns would add about US$5.2–5.9 to the full base compute estimate (US$4.6–5.3 remaining). This is a declared sensitivity, not a measured failure rate. A quarter more elapsed base processing would add roughly 5–6 hours. A sustained regional resource shortage has no defensible finite upper bound from this sample; retain the original machine/semantic conditions and report meaningful capacity failures rather than silently changing hardware or repeatedly retrying unseen work.

### Adjudication and review are additional

Mac batch 01 required 144 20M queries for 73 reviewed decisions, using 2,546.393 process-wall seconds. That supplies a **workload example only**, not a cloud query-rate measurement or a promise of the same number of decisions. Other cloud games can need more or fewer consequential/unstable/audit-expansion queries. No full-study total should assume they cost zero.

A useful explicit sensitivity for 200 games is 1,000–4,000 20M queries at 20–60 seconds per query: 5.56–66.67 query-worker-hours, or US$1.12–13.44 at the recorded VM rate before VM startup/idle and other charges. These count/rate bounds are illustrative planning inputs, not evidence-based limits or permission to truncate coverage. Runtime bounds limit individual attempts; retry and follow-up obligations can add work. The first completed cloud 20M panel must replace this sensitivity with measured rate and actual plan size. Efficiently grouping sequential short queries per VM avoids treating every query as a separate VM lifecycle.

Calendar time is base processing plus adjudication waves plus provisioning/recovery delays plus review pauses. For example, a one-hour review pause at each of nine inter-batch boundaries adds nine hours without nine hours of VM work if workers are already deleted. Record the actual pauses instead of hiding them in a compute-speed estimate. Adjudication waves and review cannot be assigned a credible final duration until the first selected cloud panel is known.

### Storage and other charges

The eight-pair pilot retrieved 3.981 GB; linear scaling suggests about 49.76 GB for the base 200 games, or 44.79 GB for 180. This is provisional superseded by the primary batch transfer measurement below. More adjudication, preserved failures, duplicate downloads and retention add volume. Transfer cost depends on destination/route and provider allowances; retained storage depends on duration. Include measured GB-months, downloaded GB, object-operation counts, VM boot-disk GB-hours, registry/log volumes and any taxes at verified applicable rates when reconciling costs. Neither US$21–24 nor the illustrative adjudication compute sensitivity is an all-in dollar forecast. The following official-price check now supplies ancillary rates; compute remains at the recorded pilot rate.

## Official ancillary rates and provisional inclusive scenario

Official pages checked 2026-11-12. The bucket metadata confirms Iowa/us-central1 Standard storage; its soft-delete retention is seven days, so retained deleted objects can also incur storage. No free-tier credit is assumed. Regional flat-namespace rates are: Standard storage $0.000027397/GiB-hour (about $0.020/GiB-month), Class A $0.005/1,000 operations, Class B $0.0004/1,000; ordinary Internet download to Europe $0.12/GiB for the first 10 TiB. The pilot-derived 49.76 GB is **46.34 GiB**, about $5.56 for one full download and $0.93 for one full month at constant volume. Repeated downloads and extra evidence increase both. [Google Cloud Storage pricing](https://cloud.google.com/storage/pricing).

Balanced boot disks cost $0.000136986/GiB-hour; 30 GiB across 102.46–117.16 VM-hours is about $0.42–0.48. [Disk pricing](https://cloud.google.com/compute/disks-image-pricing?hl=en). Standard VM external IPv4 costs $0.005/hour if present, about $0.51–0.59 across that lifecycle; applicability must be reconciled rather than assuming a free address. [Network pricing](https://cloud.google.com/vpc/network-pricing).

Registry retention costs $0.000136986/GiB-hour above any available allowance, roughly $0.10/GiB-month; same-location worker pulls are free of transfer charges. [Artifact Registry pricing](https://cloud.google.com/artifact-registry/pricing). Logging ingestion/storage is $0.50/GiB outside available allowances, with extra retention beyond 30 days priced separately; required audit logs have special exclusions. [Observability pricing](https://cloud.google.com/products/observability/pricing?hl=en). Standard Pub/Sub throughput is $40/TiB beyond its allowance. [Pub/Sub pricing](https://cloud.google.com/pubsub/pricing). Usage is not yet reconciled, so none of these categories is silently set to zero.

A **provisional full-campaign planning scenario of approximately $30–85 before tax**, rather than a bill prediction or cap, can include the following explicitly assumed quantities:

- Base compute $20.66–23.62, plus 0–25% extra base work ($0–5.91).
- Illustrative 20M compute $1.12–13.44 from the count/rate sensitivity above, plus up to ten VM-hours ($2.02) of additional 20M startup/idle.
- 50–200 GiB of total billed Internet downloads: $6–24. This includes repeat retrieval and added evidence; it is not measured transfer yet.
- 50–100 GiB retained for one month: $1–2, including additional scientific evidence but not indefinite retention.
- 0.4–1 million Class A operations and 1–5 million Class B operations: $2.40–7.00. Counts are scenario inputs pending actual request accounting; object count alone is not operation count.
- About $1–2.50 for boot disks/IPs including extra runtime, $0.10–1 for retained registry data, and $0.10–1.10 combined logging/Pub/Sub allowances in this scenario. These dollar provisions must be replaced by measured chargeable volumes; they do not assert those services stay under them.

The arithmetic spans about $32.5–82.6 and is rounded outward to $30–85 as a planning envelope, not a statistical interval. For the **remaining 180-game base workload plus all still-pending campaign adjudication**, a comparable provisional envelope is **about $30–80 before tax**: only the completed base compute is removed; already-created evidence may still need retrieval/storage and batch 01 adjudication is outstanding. Prior pilot spend and earlier Mac work are not included in these new-cohort figures. Taxes, currency conversion, longer retention, larger panels, prolonged unavailability or exceptional retries can exceed the envelope. Free-tier savings, if actually available, can reduce it. No unused budget is inferred.

For elapsed time, a conditional planning scenario is **roughly 22–41 processing hours for the full cohort, or 20–38 for the remaining base cohort plus all outstanding adjudication**, using the observed base spans, 0–25% extra base elapsed, and roughly 1–11 hours for the illustrative 20M work at six to eight occupied workers including bounded startup/tails. These are sensitivities, not validated throughput bounds. Add review pauses separately: nine one-hour inter-batch pauses would make the full calendar scenario approximately **31–50 hours**, while longer human review or regional scarcity can add unbounded delay. The first cloud 20M panel and retrieved stage totals must replace these assumptions before calling this the final end-to-end forecast.

## Admission implication

The runtime evidence shows successful task completion and cleanup, with a significant availability limitation but no demonstrated runtime defect requiring worker changes. Before batch 02, combine this record with complete artifact/stage verification and a concrete first-20M plan, then issue the requested updated end-to-end estimate with measured versus assumed components explicit. Preserve all 200 games and required adjudication; do not fit scientific coverage to the favorable base-cost estimate.

## Completed stage audit update

The independent offline scientific execution audit now passes all 2,410 jobs and 7,948 query attempts, with no failed attempts or unresolved claims. Exact frozen coverage is 5,558 initial queries and 2,390 5M singleton queries for 1,569 selected decisions. Zero eligible stable nonflagged audit comparisons cross .05. See `batch-01-review/integrity-audit.json` and its reproducible helper.

Measured parent wall time is 12,049.772s native pairs, 5,974.388s screening and 15,781.498s confirmations; parent CPU totals are 12,133.196s, 5,084.365s and 12,322.110s respectively. Query-only wall time is 5,107.976s screening and 12,254.752s confirmations; these are nested subtotals. Thus replay/artifact overhead is material, especially for the many individual confirmation jobs, and cannot be removed from the cloud forecast. The 5M query average is 5.127s; a linear fourfold node extrapolation suggests about 20.51s per 20M query, **not a measured20M rate**. Preserve the wider sensitivity pending the first panel.

Actual retrieval was 34,461 objects / 4,582,665,719 bytes in 413.085 seconds. A ten-batch scaling is 45.827GB decimal =42.68GiB, versus the pilot illustration46.34GiB; remaining nine batches are approximately38.41GiB. At the verified $0.12/GiB tariff, one complete Internet copy is about $5.12 full / $4.61 remaining before any allowance; retained full-month storage is about $0.85 full / $0.77 remaining at the Standard rate. These are base artifacts only; extra20M outputs, repeat downloads and retention remain additional. Object count is not an exact operations count because dependency verification repeatedly reads existing artifacts.

The provisional inclusive dollar and calendar scenarios remain unchanged: observed base resource costs fit them, while actual20M work, retrieval repetition, request totals and billed usage are still unknown. The update resolves the stage-data gap; it does not close the20M forecast gap or any scientific adjudication obligation.
