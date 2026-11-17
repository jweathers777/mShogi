# Shogi request overhead

MSH-189 (Shogi Request Overhead) removes repeated full-history validation from continuing timed requests. The frozen implementation is `b45ab4d43620e3b3cdc0d6db5541086145a2fd59`; the task-start baseline is `52e078e6d837807e8f02516a518d342a669ee08c`. Shogi is the first consumer of a shared native-session and transport mechanism. Evaluation and search selectivity are unchanged.

## Behavior

The child retains an authoritative native game. The first request uses the full protocol and validates the complete record. Its response can advertise the exact prepared endpoint. The bridge then sends only new native events, bound to the last acknowledged request, generation, host version and event count. A count alone is not identity. An older engine that does not advertise support continues to receive full requests.

The host checks that the new validated record extends the exact prior record. The child admits each new action through native rules and compares its actual effects and conclusion with the suffix. A rejected suffix rolls back atomically. Takeback, imported or incompatible history, and process replacement use full validation. Cancellation retires the child and its endpoint. Compact responses must match the submitted metadata and expected endpoint before the bridge attaches its original immutable context. Fixed-work requests keep their existing encoding and do not change retained timed state. Saved native records and observations keep their formats.

`native::incremental::Session<R>` owns the native lifecycle. `incremental::SessionServer<A>` owns the process protocol lifecycle. Adapters supply game-specific native types and target/limit checks. Other games can adopt these mechanisms without a separate reset, suffix or acknowledgement algorithm. See [shared search documentation](shared-search-core.md).

Timed Shogi search retains a native legal root-order choice if time or the safety node limit stops the first iteration. This choice reports depth zero, no score, actual nodes and stop reason, and one unsearched move. It does not claim a completed search. Explicit cancellation suppresses it. Native finished results and entering-king declarations remain distinct. Fixed-work incomplete results retain their prior semantics. Native root checks can exceed an extremely short time budget; this change guarantees a valid result, not hard real-time completion.

## Measured benefit

The same accepted histories, two new actions, nine samples and two warmups were used on an idle Apple M1 Pro, arm64 macOS, Rust 1.99.1 release. The complete non-search cycle includes host record construction, prefix checks, encoding, strict decoding, native admission, legal root/private-history setup, response encoding, context reattachment and matching. Actual production bridge measurements also include child pipes and polling. Samples count only if the child reached the exact intended no-search callback and returned its exact marker.

| History events | Baseline non-search | Incremental non-search | Baseline bridge | Incremental bridge |
| --- | ---: | ---: | ---: | ---: |
| 50 | 5.713 ms | 0.226 ms | 7.076 ms | 2.619 ms |
| 150 | 29.109 ms | 0.549 ms | 31.331 ms | 2.633 ms |
| 300 | 76.217 ms | 0.793 ms | 78.753 ms | 2.654 ms |

At 300 events, the complete non-search cost is **1.04% of baseline**, below the required 10%. Bridge cost is 3.37% of baseline. From 50 to 300 events, bridge latency grows by 0.034 ms and the in-memory path by 0.567 ms. This is a small absolute growth, not a mathematical constant-time claim. Host prefix comparison/context copies, private search-history setup and native repetition checks still inspect bounded history. Their cost is included.

Cold full resets still cost about 5.648, 28.108 and 73.522 ms at these sizes. They are reported separately, not removed from the record or represented as steady requests. These measurements establish request-latency improvement, not playing strength or an external rating. Decision: retain the coherent session change on its large measured benefit; do not expand this task into private-history or evaluation tuning.

## Verification and limits

The native core suite passed 386 tests, with two ignored. The final search suite passed 171, with five ignored; the engine suite passed 94, with two ignored; affected actual console suites passed 22. The earlier broader consumer checkpoint passed 678, with nine ignored. Counts belong to separate invocations and overlap. The final engine and console checks cover the later capability-negotiation correction. Strict lint checks and the standalone Tauri host build passed.

Checks include exact incremental/full native equality, illegal and corrupt suffix rollback, terminal commands, repetition, takeback/import/mismatch, wrong/stale endpoint, fixed-work isolation, process cancellation/restart, minimum-budget in-check choices, and the actual 311-event failure history from MSH-187 (Shogi Strength Round). Saved observation checks preserve old formats and reject fabricated fallback scores. Actual current bridge probes against both the frozen older engine and the new engine passed three timed requests, native move admission and clean shutdown.

Verification is Level 2 for protocol and process lifetime. Targeted Linux and Windows process checks and local macOS execution passed on the frozen source. Packaged GUI and release campaigns are deferred because this task changes neither UI nor saved-record schemas. Independent source and method reviews and exact commands are linked by the [verification record](evidence/msh189/verification.json).

The next scoped task is MSH-190 (Shogi Selectivity Round). It has not started here. MSH-188 (Shogi Evaluation Round) remains separate. No new external-opponent study or extended game campaign was run for this protocol task.
