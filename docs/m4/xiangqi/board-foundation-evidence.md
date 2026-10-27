# MSH-91 and MSH-92 board foundations

Selected profile: `mshogi.xiangqi.wxf2018.v1`. Baseline integration
`53e90a93aa8954965cd69e336a9806ea5f59acaa`; the task records identify the
subsequent integrated commits. These are developer board foundations, not a
playable Xiangqi game or a history/adjudication implementation.

## Independently expected fixtures and admitted reference (MSH-91)

105 source-reasoned fixtures, including both sides and the complete initial
44 actions. 101 lie in the restricted reference domain; four deliberate
General-threat/capture diagrams are structural diagnostics requiring native
live-root rejection. Complete position sets, complete selected-origin sets and
partial predicates are explicitly distinguished in `board-fixture-design.md`.
The corrected frozen corpus SHA-256 is
`a0724cf66703a63256efd77fe67313259ca8ce0062095155d85286d1f1b4ed4d`.

The pinned Pikafish board-only probe was actually compiled and executed on
macOS 27.0.1 arm64 with Apple clang 21.0.0. Exact upstream revision, tree,
source hashes, compiler commands, license, add-only patch and executable hash
are in `reference/pikafish-admission.json`; reproduction is in
`../../../tools/xiangqi/README.md` (repository `tools/xiangqi/README.md`).
There are no weights, network downloads, search or outcome-helper calls.
MoveList<ALL>, including check evasions, is the admitted enumeration.

Executed commands:

```sh
python3 tools/xiangqi/test_reference.py
python3 tools/xiangqi/compare.py --fixtures tests/fixtures/xiangqi/board.json \
  --depth 2 --walks 8 --length 64 \
  --report /private/tmp/msh91-reference/corrected-board-comparison-final.json
```

Results: 15 admission/failure sentinels passed; all 101 admitted fixtures and
520 deterministic sequence-prefix observations passed. Comparator make/unmake
restoration was checked. Compact durable observations and exact full-report
hash/reproduction details are in `reference/board-admission.json`. The large
raw report and executable remain reproducible local evidence, not Git assets.

Independently hand-enumerated P001 perft depths 1/2/3 are 2/3/7, depth-three
divide 5+2. Initial depth-one 44 is independent; initial depth-two 1920 is a
reference observation. These are board-only counts, ignoring history and
impossible-to-win outcomes. Deliberate mismatch, malformed requests, process
failure, deadline, descendant cleanup and output limits were exercised.

Actual disagreements/findings were retained, not hidden: B38's overlooked
Horse interposition was source-reviewed before correcting its expected set;
foreign check-adjusted FEN counters are excluded from raw native counter
comparison; recorded sequence-prefix aliasing and process cleanup were fixed
with regressions. Initial/superseded evidence remains identified separately.

Independent review: `reviews/fixture-review.md` and
`reviews/reference-review.md`, by a separate agent, including source-page
inspection, manual expected-action review, code review and independently run
sentinels/corpus. Both final verdicts PASS, with resolved findings retained.
No Fairy-Stockfish runtime admission or WXF history conformance is claimed.

## Native state and codecs (MSH-92)

See `position-api.md`. The native module owns Red/Black, 90 intersections,
river/palaces, seven pieces, complete coordinate actions, derived capture
and exact reversal. Structural diagrams, declared counters, immutable root
provenance and known-history start remain distinct. Strict bounded position
JSON preserves duplicate fields for rejection. This is not a canonical game
record. Structural transition helpers remain crate-private until MSH-93.

Executed on macOS 27.0.1 arm64:

```sh
cargo test --locked -p mshogi-core --lib xiangqi
cargo clippy --locked -p mshogi-core --all-targets -- -D warnings
cargo fmt --all -- --check
```

12 native tests passed; clippy and formatting passed. The earlier full core
library run passed 31 tests before the twelfth focused test was added. The
independent reviewer separately ran the 12 focused tests, clippy and fmt.
Coverage includes 90 coordinates, 8100 action encodings, malformed/duplicate/
missing/unknown identity fields, explicit-null versus missing counters,
provenance, exact undo, atomic stale/counter/ownership/capture rejection.
Review: `reviews/state-review.md`, with exact reviewed-file hashes alongside.
Initial setup spelling and nullable missing-field findings were corrected.

No Chess/Shogi semantics or fixtures changed. MSH-93 establishes full local
legality next; MSH-94/105 retain all history, claims and no-win obligations.
This evidence does not establish a desktop consumer or playing strength.
