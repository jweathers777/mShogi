# MSH-67 final independent implementation review

Reviewed 2026-10-26 by the separate `history_final_review` Codex AI session.
The reviewer did not author the implementation or mathematical reviews and
made no source, fixture, task or commit changes. This is independent AI review,
not human expert approval or formal verification.

## Verdict and scope

**The final native implementation is accepted within MSH-67's bounded scope.**
No unresolved material implementation finding was found. The former provisional
mutual-check rejection is replaced by an internal invariant diagnostic, and
the authoritative root/replay boundary enforces the domain assumptions of the
separately accepted proof. This verdict does not itself establish integration,
CI results, application support, playing strength or complete shogi delivery.

The reviewer read AGENTS.md, the owner's resumed MSH-67 instruction attachment,
`docs/design.md`, MSH-67, the selected profile, native history contract, fixture
derivations, historical game/implementation reviews, the new proof resolution
and both independent mathematical reviews. The complete-chain and geometry
verdicts are accepted inputs to this implementation review; this reviewer did
not independently certify their mathematical argument. Their calculation is
conditional necessary geometry, not an exhaustive search of legal states.

Review base was `0d1631db8fe4adac3ac0a9e5aa6956a863a3ba1c`, preserving
`f259bb23834d6156f21aeec53db9ee00ef7242e4`, plus the final working-tree source
identified below. The reviewer inspected the complete existing game/record
implementation and the source diff from `f259bb2`, then traced all changed
prefix call sites. Startup inspection confirmed `codex/shogi-history`, the
configured `jweathers777/mShogi` remote and primary checkout on `codex/rewrite`.

## Root, action and replay assumptions

`Position` fields are private. All public construction validates exactly one
king per side, legal promotion states, base-stock caps across board and both
hands, dead unpromoted ranks and nifu. The king/gold promotion check also
applies when a caller constructs a `Piece` directly. Standard roots use the
validated exact initial position. Full-material admission requires forty
pieces; together with the per-kind caps, this means the standard full stock.
Reduced-material admission permits missing stock and asserts no reachability.

`GameRoot::import` rejects an attacked nonmoving king, while retaining broad
structural `Position` analysis and permitting a checked moving king. This
also excludes adjacent kings in live roots. Imported categories and explicit
supplied/defaulted counters are checked, and import rebuilds a fresh diagram
origin instead of trusting a bare position's retained initial-origin marker.
The new segment begins with one occurrence; a counter never invents prehistory.

`Game::submit` calls native checked `Position::apply`. Structural transition
rejects king capture, handles promotion/capture demotion and hand transfer,
and consumes an owned hand resource for a drop. Checked apply proves the mover
unattacked after the action; that mover is then the nonmoving side. Thus each
accepted continuation preserves the root's local king-safety condition and
stock bounds. No caller can supply a check observation or mutate the ledger.
Check flags are computed from the actual resulting position.

Repetition identity compares exact board ownership/promotion, both hands and
turn. The checks slice covers every action after the first occurrence through
the fourth. Terminal commands preserve the position and conclude immediately,
so they cannot create board-action gaps in a later checking interval. Record
replay constructs the admitted root, submits every command through `Game`,
and compares every derived event prefix, including native check evidence.
Forged wire check flags cannot enter the authoritative ledger. Root and final
conclusions are checked separately. Prefix reconstruction also replays native
commands. The proof therefore applies to standard and imported live roots,
including reduced material and unknown prehistory, without narrowing the
analysis-position API or claiming historical reachability.

## Final diagnostic and regression assessment

`GameError::MutualCheckInvariant` carries the exact candidate `GameContext`,
including its root and the pending actor/action, candidate SFEN, all occurrence
indices and the complete interval's actor/check observations. Read-only context
accessors make this evidence inspectable. The candidate is a private clone and
is discarded on failure: no event, position, counter, conclusion or score is
committed, and no win, draw or tournament penalty is invented.

`Game::prefix` now returns `Result<Game, GamePrefixError>` and retains the
original replay error and event index. Record replay and record prefixes map
the internal error to `RecordError::Invariant` with its complete diagnostic,
rather than erasing it as an ordinary malformed event. Inspection found no
remaining prefix call site relying on the old `Option` contract. The record
mapping is assessed by source tracing; no real legal record is claimed to
exercise the mathematically excluded internal failure.

The new private corruption test first executes eleven genuine actions of an
independently established one-sided checking cycle, then alters cached checks
only under `cfg(test)`. Its genuine final checking action triggers the guard.
Assertions verify exact atomic preservation, no conclusion, occurrences
`[0,4,8,12]`, twelve retained interval observations, the entire pending command
context and the candidate SFEN. Replaying those same real commands recomputes
the actual checks and gives the expected ordinary Sente checking loss. This
demonstrates cache corruption, not a legal mutual-check counterexample. The
second private test corrupts an event actor and verifies prefix error/index
preservation, out-of-range distinction and unchanged original state.

Existing independently expected history/declaration/import cases, legal-action
boundaries, exact undo, prefixes, strict record corruption rejection and full
320-event save/replay regressions pass with the new behavior. Earlier fixture
derivations and historical implementation reviews remain applicable to their
unchanged source and corpus; this review does not substitute native outcomes
for those independently established expectations.

Two documentation clarity issues were reported to the integrator and are
resolved in the inspected final documents: the old blocked validation evidence
is explicitly historical with the current gate verdict above it, and coverage
describes fixtures for either checking side without implying a simultaneous
checker fixture. The corpus change updates only mutual-case resolution
metadata; the actual history/declaration/command case arrays remain unchanged.
Final integration/CI evidence belongs in the current validation/task record,
rather than being inferred from this source review.

## Executed evidence and exact identities

Observed environment: macOS 27.0.1 build 26A434, arm64;
`rustc 1.89.0 (29483883e 2025-08-04)` and
`cargo 1.89.0 (c24e10642 2025-06-23)`.

```text
CARGO_TARGET_DIR=/private/tmp/msh67-finalreview-target cargo test -p mshogi-core --test shogi_history --test shogi_record --test shogi_legal --lib
git diff --check
shasum -a 256 <reviewed paths below>
```

Final focused result: **35 passed, zero failed, zero ignored**: thirteen library
tests (including the two new fault tests), five history tests, seven legality
tests and ten record tests. The same command also passed all thirty-three
preserved-source tests before the final source changes. `git diff --check`
passed. The reviewer did not run workspace CI, tests on another OS, packaged
applications, timing/transport tests, performance or playing-strength checks.
No such result is implied. The historical runner classification failure/retry
is not resolved by this native shogi review.

| Reviewed file | SHA-256 |
| --- | --- |
| `crates/mshogi-core/src/shogi/game.rs` | `b5aaf5cada447bc1e6e57b409714f99af63b3a06706bc0c1cc7a26f2df68726d` |
| `crates/mshogi-core/src/shogi/record.rs` | `fcec3f3aa28d3dfaecc2aa8f814e4ef98cc7c9a024cfe548a9bfe3d341605ec1` |
| `crates/mshogi-core/src/shogi/mod.rs` | `ea93ed3df7173d0a0c2f1efb7d5865f2c32509477c388e4b9f74fa41d8fa1a41` |
| `crates/mshogi-core/src/shogi/legal.rs` | `a79edeef800bfc6dc316b6363cc408011189fd9b9018f05987b413ba3bea96cd` |
| `crates/mshogi-core/tests/shogi_history.rs` | `cd5f3841e3f73798368863df32f429600b6bc05e316b25144506e34b1892f77f` |
| `crates/mshogi-core/tests/shogi_record.rs` | `04208aaed6008b4c8478abbefbf18c92eee72254fbb37d3415c9f9c5a0364a63` |
| `tests/fixtures/shogi/history-outcomes.json` | `c88fe0e02140c8a094a9bce001201e3a00dbcfc9f543518c6f0d7664c06e9116` |
| `docs/m4/mutual-check-chain-review.md` | `cf3fdb729e76fd8a6778d16e6363067e5af8785357ea03556a556356d3f8bd92` |
| `docs/m4/mutual-check-geometry-review.md` | `48a3dc214e9974641807797eac9d38e3825d62746c069c2f09742a7d12ce6b51` |

Only this review report was added by this reviewer. Normal integration and
required completion evidence remain the primary builder's responsibility;
MSH-41–47 and later shogi consumers are not closed by this verdict.
