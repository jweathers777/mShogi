# MSH-161 (Competitive Gameplay Diagnosis): Batch 04 20M execution review

Passed for the completed panel. Independent offline checks covered all 765 jobs / 793 queries at 480 roots, including 28 unrestricted discoveries. Exact plan, selection, consumed-success, original-manifest and completion-verification bindings passed. Each query has the accepted full native history, original build/image/Stockfish identity, fixed options, clear-hash commands and 20M budget. Native replay receipts, legal selected PVs, clean exits and settled query groups passed. No scientific attempt failed, no query was truncated and no claim remained. Existing plan-aware artifact verification was reused; no engine, query or retrieval was repeated.

The queries retain 398 selected-report flags, 70 directional-bound conflicts and 724 reports meeting the two-report stability heuristic. These overlapping counts are not confirmed-error counts. Original AutoClaim outcomes remain authoritative; draws alone do not show weakness. Independent gameplay review must preserve earlier and linked uncertainty.

Five unrestricted discoveries returned a distinct alternative: main-04-05 game1 ply103 `c7d8`, game1 ply108 `a4a5`; main-04-06 game0 ply25 `c6c5`; main-04-unaided game0 ply23 `c6e7`, game1 ply41 `f5g6`. The ply108 discovery has a directional expected-score bound conflict; the unaided ply23 discovery has unstable last-two estimates. These flags remain explicit. Equal-work singleton coverage for distinct alternatives and any linked obligations remains separate from this completed-panel pass. The other 23 discoveries returned the played move. No scientific closure is implied.

| Measured resource layer | Wall seconds | Child CPU seconds |
| --- | ---: | ---: |
| 765 parent jobs | 18,064.344 | 16,815.361 |
| 793 nested queries | 16,796.585 | 16,792.833 |

The longest query took 27.034 seconds. Child totals overlap parent totals and must not be added again.

Submission returned zero and accepted `msh161-primary-b04-20m-v1`, UID `msh161-primary-b04-281084b8-d185-48300`. Its separate terminal event reports SUCCEEDED. One read-only cloud snapshot found eight successful tasks on eight C3 workers, with peak concurrency eight. Full eight-way execution lasted 1,863.03 seconds. The panel had five failed VM insertion completions from resource exhaustion in us-central1-a and ten related not-found delete/remove notices. These panel failures are separate from the base stage's 53 failed allocations and from zero scientific failures.

All eight successful workers have deletion completion receipts. Matching current VM and disk inventories were empty. The local snapshot found neither wrapper PID38445 nor child PID38447. The wrapper separately records computation return zero, its own group settled without a residual, and queue acceptance with return zero. This is not a general descendant-process census. Raw cloud observations and exact commands are retained in `runtime-observations.json.gz` and bound by the JSON review.

Cloud creation to final task took 2,894.230 seconds (48 min 14 s); running tasks spanned 2,854.759 seconds. VM lifetimes totaled 5.403564 hours, estimating US$1.089402 compute at the recorded US$0.201608 per VM-hour. This is not billing. The full wrapper took 4,000.653 seconds (66 min 41 s), including 291.482 seconds before the durable submission marker and 813.336 seconds after the last cloud task. The post-cloud span combines event handling, retrieval, verification, publication and shutdown; download-only timing is not separately measured.

Primary cumulative resources now total 11,330 scientific jobs, 33,567 reference queries, 58.586983 VM-hours and US$11.811604 estimated compute. These totals include four base batches, four completed initial 20M panels and nine earlier followup queries. Pending Batch 04 alternatives are not counted. Mac, pilots and the abandoned policy transition remain separate. Eighty games have completed, but only 60 are fully reviewed until Batch 04 scientific closure.

Mean scaling gives US$29.53 compute for all 200 games and US$17.72 for the remaining 120 games, before pending extras and ancillary charges. Retain the conditional inclusive US$35–90 range. Actual billing is unknown. Measured sequential cloud-stage time totals 10.62 hours; the corresponding full/remaining mean estimates are 26.56/15.93 cloud hours. They exclude preflight, retrieval, review and final synthesis. The central Saturday 14 November Paris estimate and late-Friday through Sunday window remain conditional. Capacity failures and deeper scientific obligations can add delay; neither the calendar window nor the 29–48 processing-hour scenario is a deadline or cap.

No execution or cleanup blocker was found. The coordinator retains responsibility for distinct-alternative coverage, independent scientific closure and the next batch.
