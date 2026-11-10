# MSH-155 (Chess Time Scaling): harness and evidence verification

2026-11-10. PASS: no blocking evidence-integrity or driver findings in the examined experiment. This check used only saved files and source inspection: no engine execution, build, new search, or changed production source. Commands were Python JSON/hash/legality checks and source reads. Machine results are in `/tmp/msh155-harness-verification.json`; the principal reproducible check is `/tmp/msh155-verify-harness.py`.

Attribution: this verifier authored the original scaling driver. This is a fresh evidence verification, not independent review of its own driver implementation. The separate methodology reviewer supplies independent driver/method review. The coordinator requested this bounded final check after all experiments completed.

## Coverage and integrity

- Exactly 72 unique baseline cells cover all eight roots × all three engines × 1/5/15 seconds, with no missing or duplicate tuple. Four intermediate native samples, six selected 30-second cells, three synthetic smokes and 24 adjudication attempts bring the ledger total to 109 attempts.
- All 683 per-attempt artifact hashes match. Current binary fingerprints match all frozen manifests. All 96 recorded native source hashes match current files (checked against each of four native manifests); recorded native source is d53f34413d808dc2b727a06a770b92f24191eabf. The unchanged native binary is 55b38b7411eed17038c1426630fd34a22a4f31d67fd5c3f9ce74612b5fd58ebf. Main/smoke drivers each match their own frozen identity; smoke correctly retains its earlier driver version. The separately retained failed Stockfish driver also matches the pre-fix manifest.
- All 31 native results retain Conventional mode, depth ceiling 128, no node ceiling, 4096 table entries, requested wall allowance, and six false diagnostic switches. Native root-restoration/PV-admission assertions hold. Raw result fields agree with parsed output; numeric stats parse exactly; total visits equal main_nodes + q_nodes; explicit completed depth agrees with iteration flags.
- All 77 successful UCI searches contain one go/bestmove pair and ucinewgame, valid legal bestmoves, and the expected reference memory configuration. Every raw position command exactly preserves the supplied initial FEN or startpos and complete supplied history. Raw GO→bestmove timestamp differences agree with recorded monotonic active timing to within 0.081 milliseconds.
- All 2,826 parsed UCI events match their JSONL streams; all 1,779 native/reference PV lines were replayed legally. Selected last-bestmove-aligned non-bound events match recomputation with multipv=1 and exclusion of lower/upper bounds. All 23 Stockfish successes align, retain one thread/16 MB/no tablebases/full strength, and their last two distinct aligned depths vary by at most 21 cp.
- Derived all-summary moves, timings, native depth/visits, reference selected depth, unfinished fractions and resource totals match their originating results. Non-mating native quiescence shares span 55.255–94.776%; post-last-completed-iteration shares span 5.794–72.057%, supporting the report's rounded ranges.

## Resource reconciliation

| Scope | Attempts | Charged active seconds | Process CPU seconds | Process wall seconds |
| --- | ---: | ---: | ---: | ---: |
| Matched baseline | 72 | 463.560509 | 468.217617 | 473.997514 |
| Intermediate native | 4 | 10.000898 | 10.339729 | 10.552049 |
| Selected 30-second | 6 | 180.013217 | 179.227975 | 180.857711 |
| Synthetic smoke | 3 | 0.306146 | 0.614477 | 0.714039 |
| Stockfish adjudication, including failure | 24 | 69.294263 | 76.955829 | 77.907108 |
| Total | 109 | **723.175033** | **735.355627** | **744.028422** |

Each local aggregate exactly equals the cumulative ledger sum. The 900-second allowance applies across folders, not separately per manifest. Independent mate proof is separately recorded at approximately 0.132 CPU seconds; including it conservatively leaves approximately 176.69 seconds below the operational ceiling. No inference from CPU time to search time is needed.

The one Stockfish failure is query 000: engine initialization exited with signal 25 before any `go`. Its raw log, traceback and failed-driver identity are preserved. The full 0.287249917-second process wall time was charged, and no result or chess judgment is taken from it. The later separate query 023 records `reviewed_retry_of: 0`; no failed evidence was overwritten. The exact file triggering the size-limit signal remains unknown. Current source uses the reviewed larger inherited file limit, explicit controller-log limits, CPU/wall watchdogs and process-group cleanup. No watchdog failure occurred among the successful evidence cells.

## Interpretation limits and small nonblocking observations

Native main/q visits are entry counts, not total native operations or normalized work units. TT/defense/quiet-projection counters describe events with different costs; they cannot establish cross-engine throughput or strength ratios. Native completed flags establish completion under its own selective search semantics, not exact minimax correctness. Nominal depth 128 on the solved mate control is the configured safeguard, not useful tactical depth.

Reference UCI supplies no completed-iteration flag. A non-bound score aligned with final bestmove is a useful finite snapshot, not proof of completed iteration, exhaustive root coverage, score stability beyond sampled depths, or optimal play. Reference incomplete-work fractions are unavailable. The final report states these limits correctly.

Native elapsed time excludes reporting/PV replay; reference active time is observed GO→bestmove; separate process wall includes startup/replay/setup/cleanup. Process CPU uses waited child resource usage and includes worker/startup/reporting costs. These remain distinct measurements. The logged timing is from one unisolated Mac, not statistical repeatability or hardware-normalized performance.

The main driver preserves malformed-result status in its ledger. The separate Stockfish supervisor parses a result file without that extra malformed-JSON guard; a future truncated result could therefore require manual orphan reconciliation. No truncated or malformed result occurred here, and all present ledgers/artifacts reconcile, so this is not a blocker for this completed evidence. If reused, add that guard before further experiments. Local folder locks and coordinator serialization were relied on for the complete campaign; the supplied ledgers have durations, not absolute monotonic start/end receipts, so strict no-overlap cannot be independently reconstructed solely from these ledgers. This does not change the observed sequential execution contract or current conclusions.

The final report's empirical and causal caveats are proportionate: this is a selected-position time-scaling diagnosis, not a new strength estimate, proof of monotonic quality, or broad longer-time competitiveness. Review of the tablebase/mate authorities and broader scientific conclusions belongs to the independent panel/methodology reviewers.
