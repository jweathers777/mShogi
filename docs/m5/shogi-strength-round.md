# Shogi strength round

MSH-187 (Shogi Strength Round) improves Shogi position speed, retained-history checks and timed play. The final native source is `ef1c94a3536aeac2feaa85f1722968c9bf864c0c`, compared with task-start predecessor `1d19a544cc3e1ca3e92a0e8f53268758967329bf`. The final source passes 840 consumer tests, with 14 explicitly ignored, and shows measured local speed and depth gains. **The final set passed the sequential predecessor test: 50 wins, 1 draw and 9 losses in 60 games. All games passed native replay.**

Native rules remain authoritative for real moves, declarations, histories and outcomes. No neural evaluator, evaluation-weight tuning or external rating study is part of this task.

## Final implementation

The private Shogi position uses precomputed ray masks and reverse attack lookup. It maintains the existing Pawn-advance and hand-count evaluation terms incrementally. These changes remove repeated work without changing weights. The final source restores the predecessor's exchange estimate and search policy; the shared search-core implementation and tests are byte-identical to the predecessor. The experimental full exchange calculation and quiet-check stage were rejected.

Retained-state admission now compares already validated native records directly. Root provenance, rule policies, every leading event and the earlier endpoint result must agree. Native input validation and game reconstruction remain in place. The protocol server and USI use the same cache-retention mechanism: timed continuations retain state, incompatible histories reset it, and fixed-work probes use fresh state without changing the timed session. This removes extra native replays from cache matching, not from rules admission.

Desktop Shogi play and bare USI `go` use a one-second budget, with depth 64 and five-million-node safety caps. Explicit fixed-work requests remain deterministic. Bounded `go movetime` is supported; full USI game clocks, pondering and tsume are not. Old saved play observations remain readable. New-game and retirement checks prevent stale search results from changing a game.

The transposition table remains 16 MiB per player, keyed by board, hands and side. Native history admission and private root-history construction remain in each request. Pawn value and evaluation weights are unchanged. See the [position work](evidence/msh187/position-work.md), [time integration](evidence/msh187/time-integration.md), and [independent final source review](evidence/msh187/prefix-review.md).

## Final-source measurements

Local measurements used Apple M1 Pro, arm64 macOS, Rust 1.99.1 release. Each timed root used a fresh native request at 1000 ms, with three repeats. Elapsed request time includes process startup and transport. The predecessor and revised runs occurred at separate times; these short measurements have no confidence interval.

| Root | Predecessor median depth | Final median depth |
| --- | ---: | ---: |
| Initial position | 14 | 15 |
| Double static rook | 16 | 17 |
| Fourth-file rook | 11 | 12 |
| Third-file rook | 11 | 12 |
| Central rook | 10 | 11 |
| Active rooks, pieces in hand | 7 | 7 |
| Promoted-piece composition | 64 | 64 |
| Rook ending | 16 | 16 |
| Minor pieces in hand | 10 | 10 |

The promoted-piece composition reaches a mate/depth cap and is not ordinary depth evidence. Fixed-depth work stays at exactly 1,594,093 nodes in every repeat. Median full-request time falls from 1.837619415 to 1.361317250 seconds, **25.92% less time**. Aggregate timed throughput rises from 1.128765 to 1.458681 million nodes/second, **29.23% higher**. Minor-hand throughput falls 2.88%, despite unchanged median depth; the gain is not uniform.

All 21 fixed-work roots match the predecessor for moves, scores, full principal variations, depths and nodes in both the native-game adapter and preloaded private-core paths. Both versions match all 12 exposed tactical cases, which contain only six independent geometries and their rotations. This ceiling-limited suite is a correctness check, not held-out strength evidence. These measurements establish an execution and depth benefit; the separate game comparison must establish playing strength. See the [structured receipts](evidence/msh187/revised/verification.json) and [independent measurement review](evidence/msh187/revised/review.md).

## Verification of the final source

The release consumer run for search, engine, console, desktop session and runner passed **840 tests**, failed none, and ignored 14. The Shogi-filtered native run passed 19 tests; the selected native record, history and legality suites passed 22. These are separate invocations, not a count of unique tests. Coverage includes native/private move and state comparisons, perft, incremental evaluation and state restoration, exact history-prefix identity, retained-state resets and fixed-work isolation.

Clippy with warnings denied and the standalone Tauri host check passed. Earlier release USI checks remain applicable to unchanged timed-default code; the final source adds focused history-prefix checks. The final Linux build uses Rust 1.99.1, locked dependencies and portable `-C target-cpu=x86-64`. Its smoke report identifies the exact clean native source and participant hashes. Both one-step games replayed and cleaned up successfully; their step-limit outcomes are unscored and excluded from competitive results.

Verification is Level 1 for private search and targeted Level 2 for time/process lifetime and persistence. Actual macOS checks and Linux worker evidence do not establish Windows or packaged-GUI compatibility. Those remain for the applicable integration checkpoint. Commands, source identities, limits and logs are linked in the [final-source verification record](evidence/msh187/revised/verification.json).

## Rejected candidates and failed comparisons

The first candidate, `e902c31d9445` (native-equivalent cloud source `ed92e6d28df59bc45836f108f24a3c8d9ff47e4e`), combined position speed work with full exchange calculation and a first-quiescence quiet-check stage. Local results were mixed. Its 5 s + 0.2 s comparison stopped after two candidate requests completed no iteration. Thirty pairs completed, but the campaign failed acceptance. The [round 1 record](evidence/msh187/cloud-round1/README.md) preserves all failures, complete pairs, resource receipts and cleanup.

Bounded [ablations](evidence/msh187/position-ablations/README.md) supported removing the exchange and quiet-check changes. The separate [prefix diagnosis](evidence/msh187/prefix-diagnosis/README.md) measured avoidable history-replay cost. Neither experiment alone proved the cloud failures solved. The final native source retains the speed/history/time changes and passed the final checks above.

A separate six-pair pilot of the final source at 5 s + 0.2 s also failed. Four pairs completed; three games in the other two pairs returned no completed iteration. All three failed requests belonged to the **frozen predecessor**, with 285–311 history events and 114–115 ms request budgets. They remain unscored, not candidate wins. The [round 2 record](evidence/msh187/cloud-round2/README.md) preserves these facts and verified cleanup. No game from either failed campaign is pooled into round 3.

Both failed campaigns together used 1.3443796003 measured VM lifecycle hours and US$0.2710376825 estimated compute. Their unchanged conservative charges are 2.2220445130 VM-hours and US$0.4479819502. Lifecycle envelopes and compute estimates are not billed charges.

A [matched-history diagnostic](evidence/msh187/matched-history-diagnostic/README.md) then tested both binaries on identical failed histories and ordinary controls. It completed 96 measured requests and 48 warmups without errors or forced cleanup. Original short budgets also passed locally, so the cloud failure was not reproduced. All 48 requests at 400 ms completed useful iterations. This supported a separate slower-control pilot, not another candidate change or a claimed unique cause.

## Round 3 protocol and accepted result

Round 3 keeps final native source `ef1c94a3536aeac2feaa85f1722968c9bf864c0c` against predecessor `1d19a544cc3e1ca3e92a0e8f53268758967329bf`. Worker tools are frozen separately at `200a391022d2499d0d8ff6a5bd445ecedd424a79`, image `sha256:d95c4322c3c5a60dbe992db6fc623ca1ca3d6ace626edb3d3f2235210e3ab355`. Native binaries were reused; changed Python tools are not attributed to the native build source.

The comparison uses 5 s + 0.5 s, the six frozen opening prefixes with both colors, and SPRT H0=0/H1=15 Elo, alpha=beta=.05, minimum 30 and maximum 600 pairs. Decisions are checked after complete batches in seed order. All launched pairs remain in the record. An unfinished game is unscored and stops the campaign; no cutoff becomes a draw or fallback move.

Execution reuses the existing Google Cloud Batch path in `us-central1`, with `c3-standard-4`, Intel Sapphire Rapids, at most five VMs, no retries and no overlapping batches. Pair wall cap is 420 seconds; task bound is 1320 seconds, with at most two pairs per VM. The fixed deadline is **2026-11-17 09:12:12 UTC**, including a 300-second cleanup reserve. Each submission checks the unchanged aggregate limits of **8 VM-hours and US$2 estimated compute**, including both failed campaigns. The initial prior-plus-new envelope was 7.2220445130 VM-hours and US$1.4560219502. Separate five-VM upper envelopes require complete resource audits and exact provenance; original eight-VM controller receipts remain unchanged. See the [protocol and budget review](evidence/msh187/round3-protocol-review.md).

**Accepted result:** all 30 pairs completed, with 50 wins, 1 draw and 9 losses. Pentanomial counts are [0,0,9,1,20]. LLR is 3.5711588644, above the 2.9444389792 acceptance boundary, with the 30-pair minimum met. The final host check replayed all 60 games through the native authority and found no failures. Independent review checked all 8935 clock transitions and the source, paired openings, outcome and cleanup receipts. No further games were run after acceptance. See the [final campaign record and review](evidence/msh187/round3/README.md).

Round 3 used 1.7779004117 measured VM lifecycle hours and US$0.3584389462 estimated compute. Including both failed campaigns, the task used **3.1222800120 VM-hours and US$0.6294766287 estimated compute**. The separate conservative aggregate envelope is 4.7467197375 VM-hours, below the unchanged 8-VM-hour and US$2 caps. All owned jobs, VMs and disks are absent. Compute estimates exclude storage, registry, network and other charges and are not billed totals.

Decision: retain the final speed/history/time set. Reject the earlier quiet-check and full-exchange additions. This test establishes improvement against the frozen predecessor under this control and opening set; it does not establish an external Shogi rating. Evaluation weights remain untuned. MSH-188 (Shogi Evaluation Round) records the next bounded classical evaluation set and has not started.

## Follow-up

[MSH-188 (Shogi Evaluation Round)](../../backlog/tasks/msh-188%20-%20Shogi-Evaluation-Round.md) is Ready, unassigned and depends on this task. It will select one bounded classical evaluation set, establish corpus provenance and a held-out split before tuning, and test measured benefit against a frozen predecessor. It has not started. The current exposed tactical cases and data-generation demonstration are not an independent training or validation corpus. External rating studies and neural evaluation remain outside that follow-up's scope.
