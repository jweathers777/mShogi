# MSH-93 executed local-rule conformance

Selected profile `mshogi.xiangqi.wxf2018.v1`. MSH-91/92 source foundations
were integrated/pushed at `16042f291bf3190ff2712c77344b2c6885d65f10`, then
closed in `22c10c18914d3d7b774335163bef1edec3193535` before MSH-93 began.
The MSH-93 task records the later exact integrated/tested revision and final
native CI outcomes. Source-level acceptance is specified in `local-legality.md`.

## Focused native and existing regressions

macOS 27.0.1 arm64, pinned Rust 1.89.0:

```sh
cargo test --locked -p mshogi-core --lib xiangqi
cargo test --locked -p mshogi-core --test xiangqi_board
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo clippy --locked -p mshogi-desktop-session --all-targets --all-features -- -D warnings
cargo test --locked -p mshogi-desktop-session --all-features
```

Results:16 focused unit tests and5 independent fixture/sequence integration
tests passed. The workspace regression run passed430 tests across55 reported
suites; two existing opt-in deep Chess diagnostics remained ignored, not passed.
Workspace log SHA-256 `1a011215408da3dbea44925397b3e0155259c355ea1e3db83ee2275a06ccfcc6`.
All-features desktop-session checks passed92 tests. Formatting and all-target
linting passed. The normal native CI additionally checks final committed source
on its actual Windows, macOS and Linux runners; see the task's exact run link.
No unchanged packaged-GUI campaign is inferred from these local native tests.

The105 frozen independent fixtures (101live,4structural-only) all pass. Tests
assert complete whole-position/origin sets, partial facts, check flags,
source-derived perft/divide and explicit invalid-root rejection. Every generated
choice in those cases is applied both directly and through native resolution,
compared for derived effects and exactly reversed. Three deterministic64-ply
native walks probe every choice and reverse every recorded prefix. F002 check
and quiet prefixes restore the same local board/set without inventing history
outcomes. The public boundary also tests8100coordinate pairs, stale resolutions,
counter exhaustion, invalid geography, facing Generals and the raw24-ply check
counter. Cannon screen insertion/removal and Horse-leg evasions are specifically
covered by B33–34/B38–41 and their Black counterparts. A separate independent
pin diagnostic retains geometric control from an enemy Rook pinned to its General.

## Executable differential evidence

The admitted comparator is the MSH-91 network-free, board-only Pikafish probe
at upstream `9d7833aa113626b3ea63ad83f49fb9b456bd5839`. Its exact source, add-only
patch, compiler/platform, flags, license and binary identity remain in
`reference/pikafish-admission.json`; no history/outcome helper is admitted.
Native developer instrumentation is only `examples/xiangqi_probe.rs`, not a
normal console/engine/desktop capability.

```sh
cargo build --locked -p mshogi-core --example xiangqi_probe
# Copy the just-built executable to a stable absolute temporary path, then:
python3 tools/xiangqi/compare.py --fixtures tests/fixtures/xiangqi/board.json \
  --native-probe /private/tmp/msh93-native/xiangqi_probe \
  --depth 2 --walks 8 --length 64 \
  --report /private/tmp/msh91-reference/native-board-comparison-frozen.json
```

All101 admitted fixtures and520 sequence-prefix observations agree on entire
legal sets, check, board/turn/fullmove and applicable perft/divide. Four structural
cases are explicitly excluded from the reference and explicitly rejected by
native live admission. Native make/unmake and every replay-prefix reversal are
checked by the actual executable. The compact durable report is
`reference/native-board-admission.json`, including exact binary/source/tool
hashes, complete fixture observations, sequence summaries/final prefixes and
the full raw-report hash/size/reproduction command. No large generated raw report
or binary is committed. Final execution uses an immutable temporary copy so
later Cargo invocations cannot replace the executable under observation.
A pre-final documentation build/report is explicitly superseded in the evidence.

`reference/native-depth3.json` retains actual full divide observations. Initial
Red and Black depth-three counts are79,666 each; those are differential
observations, not independently derived oracle totals. The sparse P001 fixture
has independent hand-derived depth1/2/3 totals2/3/7 and depth-three divide5+2;
both native and reference agree. The complete initial44 remains independent.

`reference/native-probe-admission.json` retains15 actual malformed/duplicate/
unknown/byte/depth/replay/domain rejection cases, with requests, diagnostics and
executable identity. The24-ply checking-cycle observation confirms native raw
counter24 versus foreign adjusted counter20 while all19 legal actions and other
local facts agree. The foreign clock is explicitly excluded, never adopted as
WXF native counter/history semantics. All expectation data remains frozen at
SHA-256 `a0724cf66703a63256efd77fe67313259ca8ce0062095155d85286d1f1b4ed4d`.

## Independent review and limits

`reviews/legality-review.md` records separate-agent line-by-line rule/API review,
source-page/fixture reconciliation, independent execution of16unit+5integration
tests, actual perft/counter queries and all520recorded-prefix provenance checks.
Reviewed file hashes are retained alongside. Final verdict PASS; the minor
royal-target attack-query documentation finding was resolved. Earlier MSH-91
B38 expected-data and harness findings remain preserved in their original records.

This establishes local board rules, exact transitions/reversal and diagnostics.
It does not deliver a complete Xiangqi game, prove historical reachability or
measure playing strength. MSH-94/105 still own warning/repetition/chase/overlap,
claimant-specific natural counts, software claim policy and impossible-to-win
semantics/proof. MSH-94 is next dependency-ready native work, still To Do pending
its recorded source/fixture gates. No MSH-94/105, engine, console, protocol,
archive or desktop Xiangqi implementation began. MSH-45 and M4 remain open.
