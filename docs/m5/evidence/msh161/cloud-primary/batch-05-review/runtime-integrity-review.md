# MSH-161 (Competitive Gameplay Diagnosis): Batch 05 runtime review

Passed for execution and cleanup. Submission accepted `msh161-primary-b05-v1`, UID `msh161-primary-b05-c99ebe6c-0f4e-41fc0`, in QUEUED state. Its separate terminal event records SUCCEEDED with ten successful tasks. Verified retrieval, wrapper completion, local cleanup and queue acceptance are separate receipts. The wrapper returned zero, settled its own group without a residual and recorded queue acceptance with return code zero. A local snapshot found neither wrapper PID43389 nor child PID43393. This does not claim a general descendant-process census.

One bounded read-only cloud snapshot covered ten task records, 71 job audit entries and current matching VM/disk inventories. Eight actual C3 workers ran, with peak task concurrency eight. No failed VM insertion or related not-found cleanup notice occurred in this batch. All eight workers have deletion completion receipts; matching live VM and disk inventories were empty. Earlier allocation failures remain in their original records and are not attributed to Batch 05. Raw observations, exact commands, task/VM intervals and bindings are preserved in the JSON review.

Cloud submission to final task took 5,548.642 seconds (1 h 32 min 29 s), about 48 minutes shorter than Batch 04. Different workloads prevent a controlled speedup claim. Running tasks spanned 5,510.418 seconds. Successful VM lifetimes totaled 8.704339 hours, estimating US$1.754864 compute at the recorded US$0.201608 per VM-hour. Actual billing remains unknown.

Existing retrieval verified 26,191 objects / 2,980,337,096 bytes in 345.21 seconds. Submission through verified retrieval took 5,908.01 seconds. No download was repeated. Parent job wall/CPU totals include child query work; the nested totals must not be added again.

Primary cumulative resources now total 13,091 scientific jobs, 40,504 reference queries, 67.454951 VM-hours and US$13.599458 estimated compute. These include five base batches, four initial 20M panels and their 19 followup queries. One hundred games have finished; the first 80 are fully reviewed. Batch 05 consequential 20M work remains pending. Mac, pilots and abandoned transition costs stay separate.

The current mean forecast uses five base batches and four fully measured panels with followups. It gives US$29.12 compute for all 200 games and US$15.52 for the remaining 100 games plus pending Batch 05 deeper work. Retain the conditional inclusive US$35–90 range. Actual billing and ancillary charges remain unverified.

Mean cloud-stage timing gives about 25.70 hours for the full study and 13.49 remaining. These exclude preflight, retrieval, independent reviews and final synthesis. Retain the conditional Saturday 14 November Paris central estimate and late-Friday through Sunday window, together with the 29–48 processing-hour scenario. Availability, deeper panels and distinct alternatives can add time. These are planning estimates, not deadlines, confidence intervals or caps. Exact arithmetic and limitations are in `forecast-update.json`.

No execution or cleanup blocker remains. Independent scientific review and required deeper coverage govern the next stage.
