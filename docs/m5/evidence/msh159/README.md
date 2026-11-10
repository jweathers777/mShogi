# Classical evaluation decision evidence

See [the report](../../classical-evaluation-decisions.md) for the experimental mechanisms,
actual decisions and limitations. `candidate-freeze.json` records source, binary,
corpus and procedure; it was committed in `9831230` before held-out results.
`native-build.json` authenticates experimental source `96e4bcd`. The candidate was
rejected after wider regression review; `retained-source.json` verifies that production
was restored to the tested predecessor. No subvariant was selected after held-out
access. Search and native rules were not changed.

`decision-runs.tar.gz` contains native attempt ledgers, temporary interior traces,
finite-reference queries, experimental source variants, diagnostic input histories,
static term decompositions, tablebase responses, failure receipts and drivers.
Extract into a scratch directory: `native/` holds timed campaigns, `reference/`
new finite analysis, `control/` variants and scripts, and `tablebase/` exact endpoint
lookups. Original absolute paths record execution provenance; remap them to the
extracted files when auditing. Manifest hashes authenticate stored bytes and the
original execution artifacts separately.

Reconstruct benchmark inputs through the [MSH-156 corpus references](../msh156/corpus/README.md).
STS upstream rights remain unclear. Its source inputs and raw position commands
are omitted; stored generated STS results omit FEN fields. The archive manifest
lists every transformation/omission and preserves original fingerprints. Mixed
STS source panels are omitted conservatively. No new benchmark was constructed.

The frozen predecessor is production `e93a750`, with the existing
[MSH-158 candidate receipts](../msh158/README.md) reused for development160,
heldout80 and fast40. Equal-three-second root reference pairs are reused from
MSH-157 by actual move and identical full history, not old role labels. Their
legal continuation summaries are in `reused-root-reference-findings.json`; raw
queries are retained in MSH-157 evidence. New intermediate reference queries have
six receipts in `reference/reference-initial/`; six final changed-choice queries are
in `reference/reference-final/`. These finite opinions are separate
from endpoint tablebase WDL and native game adjudication.

Observers are not shipped. To reproduce the trace binary, use the archived
`traced-chess_conventional.rs` and `traced-chess_frontier_profile.rs` with the
archived experimental `control/final-eval.rs` from candidate `96e4bcd`
(SHA256 `5b4d9bed4cd86f3096bfefc98dbfb14c9762e421281ae1dd49164b124d15d007`),
not the restored production evaluator. The build script records exact substitutions
and restores the files it saved. Two-root depth-four traced/plain equivalence is recorded.
Zero dropped events means no overflow within selected prefixes, not whole-tree
coverage. A node ID can refer to the caller while a child event carries child
state. TT bounds and `exact-window` are search facts, not exact Chess outcomes.
The pawn trace did not observe the corrected endpoint or a selected race-premium
leaf; the bishop trace does directly show changed frontier terms backed up to the
completed root. Timed instrumentation has overhead and is not the primary result.

`resource-receipt.json` audits timed native/reference work and the global cap.
`correctness-static-accounting.json` identifies the conservative120-second debit
covering tests, static probes and equivalence checks. That debit is not measured
engine-only CPU. Reference reservation180 remains charged even when unused.
Failed tests and compiler/fixture corrections are preserved; builds are not
presented as engine search time. No matches or platform/GUI campaign ran.
