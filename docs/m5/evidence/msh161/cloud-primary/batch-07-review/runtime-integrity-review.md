# MSH-161 (Competitive Gameplay Diagnosis): Batch 07 runtime review

Passed for execution and cleanup. Submission accepted `msh161-primary-b07-v1`, UID `msh161-primary-b07-eed9d779-73de-48460`, in QUEUED state. Its separate terminal event records SUCCEEDED with ten successful tasks. Verified retrieval, wrapper computation, cleanup and notification are separate receipts. The wrapper returned zero, settled its own group without a residual and recorded queue acceptance with return code zero. A local snapshot found neither wrapper PID93083 nor child PID93088. This is not a general descendant-process census.

One bounded read-only cloud snapshot covered ten tasks, 69 job audit entries and current matching VM/disk inventories. Eight actual C3 workers ran, with peak concurrency eight. There were no failed VM insertions or related not-found cleanup notices for this batch. Earlier failures stay in their original receipts. All eight successful workers have deletion completion receipts; matching live VM and disk inventories were empty. Raw observations, commands and task/VM intervals are in the JSON review.

Cloud submission to final task took 7,132.371 seconds (1 h 58 min 52 s); running tasks spanned 7,093.006 seconds. Successful VM lifetimes totaled 9.194458 hours, estimating US$1.853676 compute at the recorded US$0.201608 per VM-hour. Actual billing remains unknown. Different workloads prevent a controlled throughput comparison with other batches.

Existing retrieval verified 29,545 objects / 3,581,444,431 bytes in 286.71 seconds. Submission through verified retrieval took 7,438.90 seconds. No download was repeated. Parent wall/CPU totals include child query work; nested totals must not be added again.

Primary cumulative resources now total 18,731 scientific jobs, 57,077 reference queries, 97.625160 VM-hours and US$19.682013 estimated compute. These include seven base batches, six initial 20M panels and 46 followup queries. There are 140 completed games and 120 fully reviewed games. Batch 07 consequential 20M work remains pending. Mac, pilots and abandoned transition costs stay separate.

Mean scaling uses seven base batches and six completed panels with followups. It gives US$29.57 compute for all 200 games and US$9.89 for the remaining 60 games plus pending Batch 07 deeper work. Retain the conditional inclusive US$35–90 range. Actual billing and ancillary charges remain unverified.

Mean cloud-stage timing gives about 26.40 hours for the full study and 8.59 remaining, excluding preflight, retrieval, independent reviews and final synthesis. Retain the conditional Saturday 14 November Paris central estimate and late-Friday through Sunday window, with the 29–48 processing-hour scenario. Availability, deeper panels and distinct alternatives can add delay. These are estimates, not deadlines, confidence intervals or caps. Exact arithmetic and limitations are in `forecast-update.json`.

No execution or cleanup blocker remains. Independent scientific review and required deeper coverage govern progression.
