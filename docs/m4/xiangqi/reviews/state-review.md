# MSH-92 independent native state/codec boundary review

Verdict: PASS for state/action/reversal foundation, explicitly not full legality or complete game delivery.

Reviewer `/root/xiangqi_boundary_reviewer`, independent of primary native builder/integrator. Date2026-10-27. Reviewed completed `crates/mshogi-core/src/xiangqi/{mod,action,codec,tests}.rs`, sole module-export change in core/lib.rs, and `docs/m4/xiangqi/position-api.md`. Source baseline53e90a93aa8954965cd69e336a9806ea5f59acaa with bounded uncommitted task changes. Exact reviewed file hashes in `state-reviewed-sha256.json`; integration commit will be assigned by integrator after this review.

## Requirements and method

Read latest explicit user batch attachment, existing92criteria, AGENTS/design, selected XIANGQI-P1 profile, and directly inspected WXF board-clause page images. Line-by-line native source/serialization review against those boundaries, source-derived expected fixture context and focused execution. No reference engine was used as codec/rule authority and no repository/task files were changed by reviewer.

- Native Red/Black, seven identities,90bounded intersections, river4/5, two owner palaces and forward direction are explicit. Native actions contain only from/to; effects derive mover/destination victim. No Chess/Shogi side/type dependencies, promotion/hands/drop scaffolding or future shared consumers were introduced.
- Structural constructor requires exactly one General per side, inventory caps and unique occupied points but deliberately admits off-home diagnostic placement. Sparse material is representable. Constructor/FEN/root-origin documentation does not claim live-root admission or proven historical reachability;93must enforce its distinct static domain.
- Scoped FEN pins n=Horse/b=Elephant, uppercaseRed, w=Red adapter token, literal unused fields, canonical field spacing/runs, bounded ASCII/counters. Position-only root JSON separately identifies profile, governing interpretation edition, position schema, root origin and known-history start. No full-game restoration implied.
- Direct typed root parsing rejects duplicate/unknown/missing/unsupported fields; no intermediate normalization erases duplicate fields. Explicit-null unknown counters remain distinct from missing fields. Standard/imported/analysis root provenance validity is checked, and immutable root is separate from cloned mutable position.
- Crate-private structural prepare/commit checks ownership/capture/General capture/representation and binds exact before/after. It makes no movement-legality promise and has no public unchecked commit path. Undo checks full poststate before atomic restoration; counters and every owned state field are value-restored. Local value safety is expressly separate from future game/session/process execution identities.
- Quiet/capture/fullmove/unknown semantics are documented. CounterOverflow remains representational failure. The future legal query must ignore these declared-counter limits when deciding board legality and no-action state; this is explicitly recorded and93test obligation.

## Findings resolved during implementation

1. Provisional initial FEN soldier ranks omitted final Soldier. Flagged before final tests; corrected to source/fixture setup with32pieces, full inventory test now passes.
2. Provisional transparent Option-newtype counter could silently interpret a missing field as null. Raised before completed review. Replaced with custom deserialize_any visitor accepting explicit null/u32 only. Added dedicated analysis-root missing-counter test asserting JSON rejection even when provenance would otherwise accept unknown values; generic standard-root origin rejection alone would not establish strict presence.

## Independently executed checks

On macOS27.0.1 arm64:

- `cargo test -p mshogi-core --lib xiangqi::tests`:12passed,0failed. Tests include all90coordinate roundtrips and8100action coordinate combinations, strict malformed/duplicate/unknown/missing/identity cases, diagram versus provenance, atomic counter/stale/ownership/capture rejection, exact capture and Black-turn reversal.
- `cargo clippy -p mshogi-core --all-targets -- -D warnings`:PASS.
- `cargo fmt --all -- --check`:PASS.

No meaningful unresolved92finding. Full local legal-action conformance remains93, history/adjudication94/105 and native/shared consumers their existing later tickets. No unchanged desktop/Chess/Shogi platform campaign was repeated or inferred from these checks.
