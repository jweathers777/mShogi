# MSH-127 native structural foundation

Baseline `62825dd77576ac2d3b8e18d1d3080a9c08112e82`, primary `codex/rewrite`,
2026-11-01. Root Codex is sole builder/integrator. Pinned Backlog1.55.1; no other
active task owner at startup. Existing111/100/98 worktrees were preserved; no
new task worktree, external engine or GUI campaign was used.

## Acceptance coverage

| MSH-127 criterion | Supported evidence |
| --- | --- |
| AC1 native representation/root invariants | Standard setup and three independently expected structural rejection roots; both owner axes, domain/inventory/provenance tests. |
| AC2 complete structural actions/effects | Nine-face tables,40accepted board fixtures, actual drop/wait/exit sequences and all nonroyal capture/demotion tests;14explicit rejected fixture cases. |
| AC3 executable independent fixtures | Real MSH-126 adapter202/202 fields across54rows, all14primary127rows; remaining layers explicitly pending. |
| AC4 exact reversal and atomic resource rejection | Full before/after snapshots,512-action reverse/redo chains twice, stale/wrong-owner/order/branch tests and checked counter/token capacity. |
| AC5 bounded incomplete-legality boundary | Crate-private mutation/geometry, public compile-fail check, explicit structural-only acceptance tests; no dispatch/capability or consumer changes. |

## Actual implemented boundary

`mshogi-core::tori` owns native Sente/Gote, bounded7×7 Square, nine Face values,
owned Piece, six-base-kind Hand, immutable RootMetadata and structural Position.
Phoenix2/Falcon2/Crane4/Pheasant4/LeftQuail2/RightQuail2/Swallow16 combined inventory
is checked across both hands and board, counting Eagle/Goose by underlying base.
Exactly one Phoenix per side and unique occupied squares are structural invariants.
Standard setup has32pieces; reduced Diagram/Imported roots preserve unknown prior
history, source and declared completed-ply counter without claiming reachability.
A supplied diagram cannot reuse Standard metadata to assert standard provenance.
Profile and edition are immutable constants, not serialization/transport versions.

Native movement tables include all nine faces, owner-relative steps/jumps,
unlimited rays bounded by the board, blockers and Eagle's nonjumping two-square
backward diagonals. Complete typed Move supplies exact source/destination/result
face; Drop supplies exact hand identity/destination. No victim, partial route,
optional promotion flag or caller-supplied owner exists. Captures derive the exact
victim, transfer its unpromoted base to the mover and retain quail handedness.
Falcon/Swallow must promote on a board action whose origin OR destination is in
that mover's last two ranks. Drops never promote or create a timer. Subsequent
within/exit moves promote. Native Effects retains actual captured face and hand
delta. Unsupported face/resources/ownership/geometry and overflow fail atomically.

All mutation, resolution and geometry helpers are **crate-private structural
operations**, explicitly unsuitable for public game admission. A compile-fail
public-consumer doctest proves typed actions cannot call them. Position's public
constructors/read APIs describe structural diagrams, not live games. Royal safety,
Swallow-file/last-rank/drop-mate restrictions, final root admission, complete legal
choices and inspection remain128; history/outcomes/commands/records/replay/dispatch
remain129. No Tori selector, capability, protocol, engine or desktop session exists.

## Exact reversal and resource handling

Logical StructuralState contains all49board slots, both hands, side, immutable
root metadata and accepted plies. There are no incremental rule caches in this
slice. Apply checks a private candidate before changing the position. Exact
before/after snapshots support single-use Undo and Redo, bound to the particular
Position by an Arc identity. LIFO node identity rejects wrong-order/wrong-position
tokens; redo chains match exact pre-state, consume their tokens and issue fresh
Undo authority. Any newly applied branch retires outstanding redo tokens. A
consumed token never revives at an identical diagram. Cloning copies logical
state with independent reversal ownership.

Non-rewound revision/branch/node allocation is deliberately separate from logical
restoration. Every mutation checks counter capacity before committing; failures
preserve logical state, runtime fields and token usability. Declared root counter
plus accepted plies must fit u64. Immutable import-source text is at most1024UTF-8
bytes. Position/effects/token snapshots have fixed board/hand size and this layer
retains no unbounded event log. Native canonical encoded/event/replay admission
and stop reservation are129; outer document capacity remains77. No limits or
saveability promises of existing games changed.

## Actual independent fixture execution

[Compact execution evidence](evidence/native-structural-fixtures.json) derives
from the actual MSH-126 report, with input fixture IDs and field owners retained.
All103data definitions/provenance/profile and the full ownership manifest remain
byte-identical to the independent MSH-81 expectations and MSH-126 anchor.
**202 executed /202 passed /525 pending /0 failed**, four documentary annotations
and83pending later-consumer requirements. All14primary127rows and every202planned
127assertion execute across54fixture rows; no128/129/77/42assertion is certified by structural acceptance.
The default standalone conformance CLI still has no native adapter and correctly
reports0executed/727pending; the core test below supplies actual native operations.

The original42accepted specifications partition into **40board transitions** and
**two resignations**, whose complete command semantics remain129. All40board cases
execute exact before/after, undo, redo, material and metadata preservation. This
clarifies the original plan quantity without changing fixtures or acceptance.
Both orientations, all faces, capture/demotion/hand resources, blockers and jumps,
mandatory entry/within/exit promotion and TORI-P-DROP-EXIT are covered.14negative
structural cases (seven independently classified fixtures and their rotations)
assert their exact rejection and nonmutation. Safety/drop-file/mate cases stay
explicitly incomplete; some intentionally pass structural resolution only.

## Reproduce and local results

macOS arm64; rustc1.99.1 `58cf8f9c3`, cargo1.99.1 `5f94df478`:

```sh
cargo test --locked -p mshogi-core --lib tori:: -- --nocapture
cargo test --locked -p mshogi-core --doc tori::
cargo test --locked -p mshogi-core -p mshogi-conformance
cargo clippy --locked -p mshogi-core -p mshogi-conformance --all-targets -- -D warnings
cargo fmt --all -- --check
# Optional explicit test-only full report export:
MSH_TORI_127_REPORT=/tmp/tori127-report.json cargo test --locked -p mshogi-core --lib tori::conformance -- --nocapture
# pinned Backlog1.55.1:
backlog doctor
python3 docs/m4/tori/evidence/verify-native-structural-integrity.py
git diff --check
```

With cached dependencies, local runs used `--offline`. Focused native suite:
**12passed,0failed,0ignored**; public-boundary compile-fail doctest **1passed**.
Full core/conformance regression command passed 247checks across32reported
suites,0failed and2existing deep Chess checks ignored by their established
release-only opt-in policy (`deep_independent_divides_match_in_order` and
`deep_fixed_seed_sequences`). These were not run anew for an additive Tori
module; no Tori check is ignored. Lint and format passed.
Native properties execute512deterministically selected structural actions twice,
full reverse/forward restoration at every prefix, normalized material conservation,
real apply/undo and rejection closures, capture/demotion of every nonroyal face,
explicit actual drop/wait/exit sequences and owner exchange. Geometry symmetry
covers all9faces×49origins×49destinations, and real fixture transitions are rotated
independently in native state rather than through the JSON transformer.
These are structural sequences, not claims of safety/history-legal whole games.

Initial adapter execution failed on a mismatched royal-error presentation label;
projection now uses the established `royal_inventory` label. Initial tests failed
compilation on a private token-field assertion; replaced with behavior checks.
Initial focused run11passed/1failed because it expected42board cases; corrected to
the independently verified40+2partition. No selected expected data was changed.

## Independent review

Separate read-only `tori127_review` inspected owner request, selected T01–T07,
immutable expectations/manifest, complete bounded diff, adapter independence,
all movement/ownership/promotion semantics, token lifecycle, resource failures and
public incomplete-legality exposure. Independently ran final native12tests PASS,
compile-fail1PASS, core-library56tests PASS, core all-target clippy-Dwarnings PASS,
and diff check PASS; verified corpus/manifest hashes.

One material verification finding resolved: the initial negative-fixture loop
checked nonmutation only if rejection happened. Replaced it with an independently
classified7×2case mapping requiring exact rejection and full logical/runtime
nonmutation through the real shared helper. The reviewer's initial example list
included TORI-O-ROYAL; its actual expected failure is own-Phoenix safety and remains
128, not a structural rejection. No fixture interpretation changed. Final verdict
**PASS, no material findings**. Exact reviewed file hashes are in execution evidence.

## Integration, CI and handoff

Reviewed source integrated and pushed at
`ce1801d7801f12e447b625739acd6a0c53aa2640`. Native workspace CI
[41324873187](https://github.com/jweathers777/mShogi/actions/runs/41324873187)
**PASS** at that exact source on Ubuntu24.04, macOS15 and Windows2022.
[Exact job/step metadata](evidence/native-structural-ci.json) records successful
format, lint, workspace tests, isolated QA constructors, snapshot fixtures, build
and console smoke. No platform result is inferred from another system. Existing Chess/Shogi/Xiangqi rules and consumers are unchanged;
core/conformance regressions and normal native CI provide compatibility evidence.
The automatically dispatched Desktop packages workflow is not this task's
required GUI evidence, and no new GUI journey is claimed from its construction.
No packaged GUI or engine-strength evidence is claimed. MSH-127 is **Done**
under all five original acceptance criteria and four completion items; there
are no unresolved material review findings or profile decisions. Final task/doc
bookkeeping changes no reviewed code and uses the established `[skip ci]`
convention; the tested production source remains ce1801d. MSH-128 is now the
first dependency-ready slice, unassigned and unstarted.129/85final acceptance/
77/42/other variants remain unstarted. M4 stays open.
