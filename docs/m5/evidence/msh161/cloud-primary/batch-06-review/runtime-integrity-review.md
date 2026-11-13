# MSH-161 (Competitive Gameplay Diagnosis): Batch 06 runtime review

Passed for execution and cleanup. Submission accepted `msh161-primary-b06-v1`, UID `msh161-primary-b06-c50373b2-1bf7-4ab50`, in QUEUED state. Its separate terminal event records SUCCEEDED with ten successful tasks. Verified retrieval, wrapper computation, cleanup and notification are separate receipts. The wrapper returned zero, settled its own group without a residual and recorded queue acceptance with return code zero. A local snapshot found neither wrapper PID68667 nor child PID68669. This is not a general descendant-process census.

One bounded read-only cloud snapshot covered ten tasks, 76 job audit entries and current matching VM/disk inventories. Eight actual C3 workers ran, with peak concurrency eight. One VM insertion failed from resource exhaustion in us-central1-a; two related not-found delete/remove notices remain preserved. These allocation operations are distinct from zero scientific failures. Earlier failures stay in their original records. All eight successful workers have deletion completion receipts; matching live VM and disk inventories were empty. Raw observations, commands and task/VM intervals are preserved in the JSON review.

Cloud submission to final task took 7,809.137 seconds (2 h 10 min 09 s); running tasks spanned 7,769.922 seconds. Successful VM lifetimes totaled 9.737168 hours, estimating US$1.963091 compute at the recorded US$0.201608 per VM-hour. Actual billing remains unknown. Different workloads and allocation conditions prevent a controlled throughput comparison with prior batches.

Existing retrieval verified 29,519 objects / 3,480,053,341 bytes in 281.64 seconds. Submission through verified retrieval took 8,128.19 seconds. No download was repeated. Parent job wall/CPU totals include child query work; nested totals must not be added again.

Primary cumulative resources now total 15,786 scientific jobs, 48,910 reference queries, 82.140953 VM-hours and US$16.560273 estimated compute. These include six base batches, five initial 20M panels and 30 followup queries. One hundred twenty games have finished; the first 100 are fully reviewed. Batch 06 consequential 20M work remains pending. Mac, pilots and abandoned transition costs stay separate.

The mean forecast uses six base batches and five fully measured panels with followups. It gives US$29.21 compute for all 200 games and US$12.65 for the remaining 80 games plus pending Batch 06 deeper work. Retain the conditional inclusive US$35–90 range. Actual billing and ancillary charges remain unverified.

Mean cloud-stage timing gives about 26.08 hours for the full study and 11.08 remaining. These exclude preflight, retrieval, independent reviews and final synthesis. Retain the conditional Saturday 14 November Paris central estimate and late-Friday through Sunday window, with the 29–48 processing-hour scenario. Availability, deeper panels and distinct alternatives can add time. These are estimates, not deadlines, confidence intervals or caps. Exact arithmetic and limitations are in `forecast-update.json`.

No execution or cleanup blocker remains. Independent scientific review and required deeper coverage govern the next stage.
