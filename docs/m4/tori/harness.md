# Self-contained executable Tori conformance harness — MSH-126

`mshogi-conformance` is a development-only Rust workspace library/CLI, not a Tori
rules implementation, search engine or packaged desktop dependency. No external
engine, Python module, engine-specific build system or network access is needed.
Normal Cargo library dependencies are the already pinned serde/serde_json/sha2.
[Project policy](../../testing/conformance-policy.md) supersedes the earlier
mandatory Fairy-Stockfish gate. No external engine was built/invoked for126.

## Reproduce

```sh
cargo test --locked -p mshogi-conformance
cargo run --locked -p mshogi-conformance
cargo run --locked -p mshogi-conformance -- --observations observations.json
cargo run --locked -p mshogi-conformance -- --observations observations.json --require-owner MSH-127
```

With dependencies cached, add `--offline` before the Cargo `--` separator.
The default run validates all103 independently specified fixtures and their full
ownership manifest, then reports **0 executed,0 passed,727 pending,0 failed**
assertions. Four hand-written escape/defense proofs are documentary annotations,
not executable native outputs. All83 later-consumer requirements remain explicitly
pending; a local/native projection cannot establish desktop process behavior.
These are assertion counts, not727 distinct game positions or native tests.

Exit0 means supplied observations matched and admitted fixture data was valid;
**it does not mean unimplemented native behavior passed**. Exit1 means an executed
assertion failed or a requested completion guard found pending work; exit2 means
malformed/conflicting/oversized input or CLI usage. `--require-owner MSH-127`
checks all executable assertions assigned to that owner while other layers stay
pending. Unknown owners reject. `--require-complete` deliberately fails closed
while any assertion OR unimplemented later-consumer requirement is pending. It
cannot currently claim full consumer completion, even if all native field values
are supplied; actual consumer adapters/contracts remain MSH-77/42 work.

## Immutable expectations and strict admission

The exact corpus remains MSH-81 SHA256
`3f9e5ca3b541fc72f562ddd8231408e17e7bd5ba3ccee118e643ce0f543731e3`.
Git checkout attributes force canonical LF for the corpus/ownership on Windows
as well as Unix; hash comparison is never normalized or weakened.
All103 IDs, source citations, method, given input, expected values, original39seed
identities and reviewed ownership are embedded/read-only. Optional `--corpus`
and `--ownership` paths verify identical audited definitions/bytes, not replacement
expectations. Recursive JSON admission rejects duplicates before any map exists;
typed outer definitions reject unknown/missing fields. Expected nested data must
match the independent anchor and pinned hash, so unknown/conflicting nested
fields cannot be normalized into acceptance. Ownership rejects duplicate/missing/
unknown IDs, owners or expected-field lists. No adapter can modify `Corpus`.

Each input is bounded to512KiB (corpus/manifest currently well below this), with
serde's normal depth limit and bounded diagnostics. The CLI reads at most limit+1
bytes before rejection. Full native record tests may use the Rust adapter API
with projected actual state rather than duplicating megabyte records into an
observation file. This bound does not change any native/document capacity.

## Actual native adapters and constructed observations

The Rust `NativeAdapter` receives a read-only fixture and planned `Assertion`
(owner, boundary, field, predicate). It computes an actual native operation and
projects its result. Return `None` for unimplemented work and `Err` for execution
failure; neither passes. Never return `assertion.expected` as an implementation.
The CLI accepts strict observations, for example this **constructed machinery
probe** (not actual native evidence):

```json
{
  "schema": "mshogi.conformance.observations.v1",
  "profile": "mshogi.tori.modern.v1",
  "corpus_sha256": "3f9e5ca3b541fc72f562ddd8231408e17e7bd5ba3ccee118e643ce0f543731e3",
  "observations": [{
    "fixture_id": "TORI-CAPTURE-OWNER",
    "field": "hand_delta",
    "owner": "MSH-127",
    "value": {"s:Cr": 2}
  }]
}
```

It fails because the independently expected capture adds **one**, not two Cranes.
Diagnostics include fixture, boundary/owner, input context, expected and observed
values. Strings are capped768characters each (CLI admission errors1024); original
full fixture/provenance remains available. Unknown fields/owners/identities and
repeated `(fixture,field,owner)` observations reject. Supplying the same field for
127 and128 is valid: structural effects and full local legality are different
assertions, and one cannot discharge the other.

The executable plan separates geometry, transition, local legality, history,
record and consumer assertions. All103 row IDs have immutable primary owners
127=14,128=69,129=20, with126 owning independent corpus infrastructure. Field-level
supplemental projections give127 real transition-effect and snapshot checks;
128 owns local admission/reversal;129 owns actual Game commands/conclusions/
lineage/records. Impasse/agreement rejection and resignation never become local
move admission. Full process/session/purpose/limits and actual live-document
preservation remain77; human notation remains42. Supplemental obligations in the
manifest remain visible, never silently certified by a primary row pass.

## Comparisons, properties and transformations

- Complete action/geometry sets reject missing/extra/duplicate members and ignore
  irrelevant enumeration order. Ordered history/index sequences remain ordered.
  No perft count is invented.
- Named capture/hand/promotion fields compare independently specified effects.
  Native adapters must expose actual deltas, not infer effects from notation.
- `reversal_contract/apply_then_undo` and `/redo` require actual `before`/`after`
  state snapshots, not a boolean or repeated instruction string. Include all
  authoritative logical state/caches; canonical replay equivalence uses the same
  snapshot machinery. `/retired_execution` compares different **native** lineage
  snapshots; actual process acceptance remains a later77 requirement.
- `properties::round_trip` runs supplied native apply/undo closures and checks a
  complete caller-provided projection. `properties::rejection` actually calls a
  rejected operation and detects acceptance or mutation. Project native logical
  state separately from deliberately never-rewound runtime identity.
- `equivalent`, `snapshots` and `preserves_material` compare native-provided state
  or normalized base inventories. They implement no capture/demotion rules.
  Use native base-kind identities for promotion/material conservation checks.
- `rotate_square`/`rotate_data` transform only7×7 fixture coordinate/owner data;
  they do not generate moves. Tests exhaust49squares, all103 input involutions
  and all35 independently authored owner-rotated pairs. Explicit side-exchange
  checks prevent a vacuous identity transform passing involution alone. Future
  native adapters must independently rotate native state and compare behavior;
  fixture-data rotation alone is not native metamorphic conformance.

Current18 tests include missing/extra/duplicate actions, wrong movement/capture/
promotion/demotion/quail/hand count/owner/history, strict malformed/conflicting
fixtures/observations, CLI nonzero failures, bounded adapter errors,2000deterministic
generated snapshot mutations, bad undo/rejection and valid-data transformations.
Constructed observations exercise assertion machinery only. At MSH-126 completion, native apply/legality/history properties were pending.
MSH-127 now executes structural apply/undo/redo and native properties as recorded
below; complete legality, repetition and canonical replay remain128–129.

## Integration and evidence

The crate is part of normal Rust workspace format/lint/test/build CI, but no
product crate depends on it and no Tauri bundle/resource configuration references
it. No GUI journey or other engine is needed. See [MSH-126 evidence](harness-evidence.md)
for exact revisions, independent review, failures/corrections and CI disposition.
At MSH-126 completion, MSH-127 was the next bounded task; its completion
and the subsequent MSH-128 evidence are recorded below. No later task starts
automatically. All downstream complete-game/shared/M4 obligations remain open.


## MSH-127 native structural execution

The [actual core adapter and evidence](native-structural-evidence.md) now execute
202MSH-127assertions across54fixture rows:202pass,0fail,525assertions pending,
83later-consumer requirements pending. Run `cargo test --locked -p mshogi-core --lib tori:: -- --nocapture`. All14primary127rows and40accepted board transitions
are covered; the2accepted resignation specifications remain129. The default CLI
without a native adapter still truthfully reports0executed/727pending. Geometry/
structural transitions and token reversal do not establish complete legality,
history or playable delivery. Native128/129 and consumer77/42 remain incomplete;
none is automatically activated by127completion.


## MSH-128 complete local execution

The [actual local adapter and evidence](native-local-evidence.md) execute483native
assertions across83fixture rows:202MSH-127 and281MSH-128 pass,0fail,244remain
pending under history/record/consumer ownership;83later-consumer requirements
are separately pending. Run `cargo test --locked -p mshogi-core --lib tori::local_conformance -- --nocapture`.
All69primary128rows have actual local evidence. The default standalone data CLI
still has no native adapter and reports727pending, not a complete native pass.
Local board legality/inspection does not satisfy129Game permission/conclusions/
replay or77/42execution, document, notation and product obligations. None starts
automatically. Prior126/127counts above remain evidence for their dated boundaries.
