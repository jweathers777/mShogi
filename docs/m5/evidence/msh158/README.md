# Chess decision correction evidence

See [the report](../../chess-decision-corrections.md) for decisions and limitations.
`candidate-freeze.json` records source, binary, corpus and procedure before held-out
access. `native-build.json` authenticates the production source; final local checks
and independent source/trace reviews accompany it. Only the cached threat-context
correction remains in production. Temporary observers and four rejected trials are
preserved for audit, not shipped.

`decision-runs.tar.gz` retains actual interior traces, all native attempt ledgers,
finite-reference runs and experimental controls. Extract to a scratch directory:
`native/` contains run directories, `reference/` reference analysis, and `control/`
the observer patch, frozen experimental sources, hypotheses, drivers and receipts.
The original absolute paths are execution provenance. Remap them to extracted
artifacts and reconstructed source fixtures; archive hashes authenticate stored
bytes and original hashes authenticate inspected execution artifacts.

Reconstruct benchmark inputs using the [existing corpus references](../msh156/corpus/README.md).
STS upstream rights remain unclear, so its source rows, inputs and raw position
commands are omitted, and generated FEN fields are removed from stored STS results.
The bundle manifest explicitly lists these transformations and omissions. STS IDs,
decisions, scores, timing and original fingerprints remain. No new corpus was made.

Trace limits matter: no retained trace dropped events, but prefix filters deliberately
observe only selected paths. A synthetic-null path element is `null`; distinguish
it from real native history. Event node IDs identify the caller invocation, while
child events can carry the child path/FEN before pop. `exact-window` describes a
call's alpha-beta window, not game-theoretic truth. A TT continuation is not proof
that its endpoint supplied the returned bound. Timed tracing has overhead; use the
uninstrumented frozen candidate for the primary comparison. Depth-four traced/plain
identity checks are retained separately.

All timed native/reference searches ran sequentially. Compilation and local tests
were separated from timed measurements. `resource-receipt.json` accounts for timed
work, reference reservations and a conservative correctness-test debit. Preparatory
CLI errors made no engine calls; failed test/compile attempts are disclosed in the
verification record. No matches, platform campaign or GUI campaign ran.

The predecessor is production `3b52314`, using unchanged MSH-157 candidate measurements
and binary. Those earlier valid runs are linked, not charged again or presented as
new experiments. Published preferences and finite reference judgments are separate
from exact tactical/tablebase outcomes. A local correction is not a strength rating.

To rebuild a diagnostic variant in an isolated checkout of `17529cd`, apply
`control/trace.patch`, then replace `chess_conventional.rs` with the selected
`control/experimentN-search.rs` for trials 1–4. Trial 5 uses
`control/baseline-traced.rs` plus `control/experiment5-eval.rs`. Build the
`chess_frontier_profile` release example using the retained command/toolchain.
The temporary observer added an Engine field; tests of an instrumented variant
also require `trace: None` in the test-only Engine constructor. Production tests
and the final candidate contain no observer field. Run manifests retain input
filters and original conditions; rebuild hashes can differ across toolchains.
