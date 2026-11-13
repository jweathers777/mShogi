# MSH-161 (Competitive Gameplay Diagnosis): Batch 08 runtime review

Passed for execution and cleanup. Submission accepted `msh161-primary-b08-v1`, UID `msh161-primary-b08-01372bf6-b9f7-4ad60`, in QUEUED state. Its separate terminal event records SUCCEEDED with ten successful tasks. Verified retrieval, wrapper computation, cleanup and notification are separate receipts. The wrapper returned zero, settled its own group without a residual and recorded queue acceptance with return zero. A local snapshot found neither wrapper PID35390 nor child PID35394. This is not a general descendant-process census.

One bounded read-only cloud snapshot covered ten tasks, 71 job audit entries and matching current VM/disk inventories. Eight actual C3 workers ran, with peak concurrency eight. No VM insertion failed and no related not-found cleanup notice occurred. Earlier failures remain in their original records. All eight successful workers have deletion completion receipts; matching live VM and disk inventories were empty. Raw observations, commands and task/VM intervals are in the JSON review.

Cloud submission to final task took 6,457.158 seconds (1 h 47 min 37 s); running tasks spanned 6,412.123 seconds. Successful VM lifetimes totaled 9.234587 hours, estimating US$1.861767 compute at the recorded US$0.201608 per VM-hour. Actual billing is unknown. Different workloads prevent a controlled throughput comparison with other batches.

Existing retrieval verified 29,109 objects / 3,364,069,394 bytes in 361.91 seconds. Submission through verified retrieval took 6,856.21 seconds. No download was repeated. Parent wall/CPU totals include child query work; nested totals must not be added again.

Primary cumulative totals are 21,705 scientific jobs, 65,217 reference queries, 113.582984 VM-hours and US$22.899238 estimated compute. This includes eight base batches, seven initial 20M panels and 55 followup queries. There are 160 completed games and 140 fully reviewed games; Batch 08 consequential 20M work remains pending. Mac, pilots and abandoned transition costs stay separate.

Mean scaling uses eight base batches and seven completed panels with followups. It gives US$29.96 compute for all 200 games and US$7.06 for the remaining 40 games plus pending Batch 08 deeper work. Retain the conditional inclusive US$35–90 range. Actual billing and ancillary charges remain unverified.

Mean cloud-stage timing gives about 26.80 hours for the full study and 6.10 remaining, excluding preflight, retrieval, independent reviews and final synthesis. Retain the conditional Saturday 14 November Paris central estimate and late-Friday through Sunday window, with the 29–48 processing-hour scenario. Availability, deeper panels and distinct alternatives can add delay. These are estimates, not deadlines, confidence intervals or caps. Exact arithmetic and limitations are in `forecast-update.json`.

No execution or cleanup blocker remains. Independent scientific review and required deeper coverage govern progression.
