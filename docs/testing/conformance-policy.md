# Engine-independent conformance policy

Owner decision2026-11-01, recorded with MSH-126. Applies to **Chess, Standard
Shogi, Xiangqi, Tori, Chu and Tenjiku**, including ordinary testing, CI, milestone
acceptance and release. Supersedes earlier mandatory external-comparator wording.

The selected named/versioned rules profile defines behavior. Establish expected
results independently from those rules before comparing implementation output
where practical. Record source/interpretation, exact input/history, expected result,
review, seed and tested revision. Neither another engine's answer nor agreement
between engines establishes correctness; disagreement requires investigating the
profile and concrete independently reviewed position. Never edit expected data
merely to agree with software.

Every game's required evidence must include meaningful:

- Independently expected legal-action/transition and history/adjudication fixtures.
- Deterministic regressions, exact apply/undo and canonical replay verification.
- Property-based tests of actual authoritative state invariants.
- Metamorphic tests of transformations valid for that selected game/profile.
- Adversarial inputs, deliberate negative/mutation tests and failure diagnostics.
- Independent source and implementation review.

Apply techniques where they expose a concrete risk; not every case needs every
technique. Self-consistency alone does not establish rule correctness. Perft needs
independently established expectations; it does not verify history, search,
strength or product integration. Generated invariant/rotation tests supplement
rather than replace hand-established rule examples. Do not assert a symmetry or
conservation rule that the game does not guarantee.

External implementations are **optional investigative aids**, never mandatory
rules oracles or build/test/conformance/CI/package/milestone/release dependencies.
No third-party game engine source, binary, engine-specific language runtime or
build system belongs in the normal product/CI dependency chain. A separately
requested investigation must pin/audit its operation and profile limitations;
its observations remain distinct from independently expected facts. Optional
opponent-based strength/interoperability experiments retain their stated purpose
and do not become rules-conformance gates.

Historical Chess/Shogi/Xiangqi comparisons and completed reviews remain accurate
observational evidence at their original revisions; do not erase, rewrite or
reopen them solely for this policy. Prospective Tori/Chu/Tenjiku verification
must work when no independent implementation exists. No historical acquisition,
new exact-proof or extra planning gate follows from this decision.

## Native adapter and accounting discipline

A fixture loader validates data/provenance, not native behavior. An unimplemented
native assertion is **pending**, never passed. Reports distinguish data validation,
executed/pass/fail assertions, documentary source annotations and outstanding
native/consumer obligations. Constructed observations may test assertion machinery,
but must be labelled as such; they are not Tori conformance evidence.

Adapters project **actual** native results and authoritative state into fixture
assertions, without copying the expected answer, implementing a second rules
engine or inferring outcomes from evaluation/absence of proof. Compare complete
legal-action sets without losing extra/missing choices; keep event/history order
where significant. State properties take actual pre/post/undo/replayed snapshots;
serialized public records need not expose private undo memory. Include caches in
internal restoration tests. Rejected commands must preserve state and history;
semantic undo/reopen must not revive retired execution identities.

The current [Tori harness](../m4/tori/harness.md) supplies strict admission,
independently fixed expectations, mutation tests and native property interfaces.
MSH-127/128/129 later supply actual kernel observations; MSH-77/42 retain real
execution/document/notation consumers. External engines are not needed to run it.
