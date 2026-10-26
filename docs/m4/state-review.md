# MSH-65 independent native-state review

Date: 2026-10-26. Reviewer: fresh Codex review agent, separate from the state
author and integrator; independent agent review, not human review. Worktree:
`next-task`, branch `codex/shogi-board`. The state source was reviewed as working
changes; HEAD at final focused verification was
`bd374dd7fca6b01b86ae4aa468e5e95a8e159a14`.

**Verdict: PASS for MSH-65. No material findings remain.** This review covers
native local state, strict coordinate/text primitives, structural transition
checks and exact reversal. It does not establish complete legal enumeration,
king safety, pawn-drop-mate exclusion, history adjudication, match outcomes,
reference conformance or full MSH-41 delivery.

## Scope and method

Read all of `shogi/mod.rs`, `action.rs`, `sfen.rs`, `tests.rs` and the `lib.rs`
export diff, and compared them with MSH-65's four acceptance criteria and the
approved `mshogi.shogi.csa36.v1` position/action/import contract. Inspected
constructor visibility and invariants, all error exits, movement geometry,
promotion/drop/capture paths, counter arithmetic, provenance and undo matching.
Manually checked the expected initial placement, center movement sets and the
opening capture/demotion/drop sequence rather than treating test success alone
as a rules oracle.

Independently ran, on macOS with the repository's configured Rust toolchain:

- `cargo test -p mshogi-core shogi::tests -- --nocapture`: 11 passed, 0 failed;
  other integration binaries had their tests filtered out.
- `cargo test -p mshogi-core --lib shogi::tests -- --nocapture`: 11 passed,
  0 failed at the final review boundary.
- `git diff --check -- crates/mshogi-core/src/lib.rs crates/mshogi-core/src/shogi`:
  passed.

No full workspace/CI, external reference, opponent, GUI or packaged application
checks were run by this reviewer. Only this report was edited. The integrator
owns subsequent integration and native CI evidence.

## Acceptance assessment

**AC1 — state and provenance:** Sente/Gote and their directions are native;
bounded squares map one-based files/ranks to 81 SFEN-ordered cells. Base kind,
promotion and ownership are independent. Private position/hand storage prevents
unchecked external mutation; `from_diagram` revalidates even directly constructed
public `Piece` values. Combined inventory, kings, promotion, dead ranks and nifu
are checked. Sparse material, adjacent kings and other unproven diagrams are
explicitly structural analysis inputs, not proofs of legality or reachability.
Initial origin and imported diagram origin are distinct, with root counter and
supplied/defaulted provenance retained as the current counter advances.
The chess module/contracts are untouched; `lib.rs` only adds the shogi export.

**AC2 — complete structural choices:** Move and Drop include all information
needed to identify the chosen board action. Capture is derived from the actual
destination; a promoted victim contributes its own base kind to the capturer's
hand. No caller-supplied capture identity or king capture is accepted.
Side-relative geometry and ray blocking, camp entry/exit promotion availability,
mandatory promotion, friendly occupancy, held resources, dead drops and nifu
are explicit. Both sides and promoted movement receive focused coverage.
The hand increment is safe under the admitted combined-inventory invariant:
the captured resource was already counted on the board, and all paths conserve
its base kind. The next-ply increment is checked before mutation.

**AC3 — atomic transitions and exact reversal:** Structural resolution builds
an independent candidate and commits only after all checks succeed. Rejections
therefore preserve board, hands, side, counter and origin. Successful transition
records complete before/after states; undo requires the exact complete post-state
and rejects stale/out-of-order use atomically. Nested undo and conservation are
tested through an actual opening capture, promoted capture and drops, with an
independently checked final SFEN. There are no hidden incremental hashes/caches
to restore in this implementation. Undo deliberately does not distinguish two
contexts with equal complete state, as its documentation states.

The helper contract is appropriately narrow: `apply_structural` and
`undo_structural` are crate-private and explicitly do not provide full legality.
The temporary method-local `dead_code` allowances name their MSH-66 consumer.
`SelfCheck`/`PawnDropMate` error variants reserve the legal layer's checks; they
are not represented as implemented here. No public structural transition can
be mistaken for a public fully checked game move at this review boundary.

**AC4 — text and import boundaries:** SFEN accepts exactly four ASCII fields
within a 512-byte bound; rank widths, promotion markers, side, duplicate hands,
inventory and positive u32 counter are validated. Decimal hand accumulation is
checked and capped before conversion. Strict syntax rejects noncanonical
single-resource count prefixes, leading-zero counters, malformed spacing and
trailing fields. Canonical output orders hands RBGSNLP by owner. USI board-action
parsing checks ASCII and exact lengths before slicing, distinguishes optional
promotion and uppercase drops, and excludes resign/win/null protocol results.

Supplied counters including n321 and u32::MAX remain admissible for analysis;
counter overflow is an explicit structural transition error. This does not
claim match classification or permit continued live play past the selected
limit. Defaulted diagram counters are explicitly one. SFEN serialization itself
cannot encode origin/history; the API documents preservation of `history_origin`
separately for future canonical game records. No prior repetition, checking
history or terminal outcome is invented from the counter or diagram.

## Reviewed source identity

SHA-256 of the working source at final review, before later legal-layer edits:

| File | SHA-256 |
| --- | --- |
| `src/lib.rs` | `47609273a847a105e02a532267f95caeadce6286c8b6d4441268f18073fa0ab0` |
| `src/shogi/mod.rs` | `8af2fe4ebda92242d3293014f47d8a0f228fd1557383a7f7c3ea486225d77ca7` |
| `src/shogi/action.rs` | `a026ff32f4e5e1592a0794bc4a55eb10297d0cebfe6bac4fdaa2e98a52b9bee7` |
| `src/shogi/sfen.rs` | `63a25a516fa58b4279a4a2436236c4dabcd67c4bd8037c5f8af542721ccb7bf8` |
| `src/shogi/tests.rs` | `efe48aa6fb2f59f69eb191cdfb869669eae748fc9acc7183575a3403bdcd8403` |

Paths in this table are relative to `crates/mshogi-core/`. MSH-66's full legality
and BOARD-PIN fixtures, and MSH-67's history/declaration/import adjudication
gates remain required independently of this PASS.
