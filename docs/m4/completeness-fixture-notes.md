# MSH-66 independent board completeness evidence

The separate [legal-completeness.json](../../tests/fixtures/shogi/legal-completeness.json)
corpus adds68 manually derived records to the current101-case board corpus.
An independent Codex reviewer checked all expected facts before reference or
native execution; see [completeness-fixture-review.md](completeness-fixture-review.md).
Expected choices were never filled from cshogi or mShogi output.

The68 records contain56 exact central/edge move sets: all eight base kinds and
all six promoted kinds, on both sides. The named geometric targets are retained
in each derivation. Nonking matrices include the independently enumerated five
safe auxiliary-king choices; central bishop/horse kings are deliberately off
diagonal rays. Promotion suffixes are separately counted complete choices.
Six additional records cover friendly/enemy bishop/lance blockers. Four records
prove an empty legal board set with check, or without check, by exhausting king
escape/capture/interposition and all other available pieces. Those facts do not
adjudicate a game or import a chess stalemate convention. Two initial-position
records independently establish perft1=30, perft2=900 and each depth2 divide
entry=30, including ownership-reversed first mover.

The900 proof uses30 independently named first choices. Every first action stays
on its own half and leaves all opponent blockers unchanged. Pawn advances remain
on rankf; back-rank actions remain behind the pawn wall. The only newly opened
long bishop ray after7g7f follows8h7g6f5e4d3c2b, missing the opponent king5a
and its4b/5b/6b destinations. Thus all30 opponent choices survive every first
move, giving30×30=900. Deeper counts or other diagram perft counts are reference
observations unless independently stated otherwise.

## Domain failure, correction and retained history

An initial long seeded comparison failed reference reversal before producing
valid native-conformance evidence. Historical SH-F022:evasion-B-sente had
root `8k/9/4b4/5B3/9/9/K8/9/9 b - 1`: bishop4d already attacked the nonmoving
king1a along3c2b1a while bishop5c checked the moving king9g. The mirrored root
had the same defect. cshogi's incremental-check representation assumes the
nonmoving king is not already checked. The failing seed20350827 path began
`9g8h 5c8f 4d1a 8f6h+ 1a6f`; it admitted a king capture and later produced a
missing/reappearing king on undo. No native correctness conclusion is drawn
from that unsupported reference path. An independently produced initial perft
failure report is retained separately by the integrator.

Only those two current board-corpus records were corrected to version2, moving
the opposing king1a→1b before rotation, outside that diagonal. Their independently
expected bishop captures of5c with both promotion choices are unchanged. The
correction was reviewed geometrically before corrected execution. Original
fixture, run and admission bytes remain in board-rules-v1.json,
reference-run-v1.json and cshogi-admission-v1.json, with the supersession rationale
on each current record. See [reference-domain-review.md](reference-domain-review.md).

The source-audited admission guard now validates bounded ASCII SFEN, piece caps,
nifu, dead unpromoted pieces, nonadjacent kings and a signed32-bit next-ply budget
before C++ board construction. It then uses the public dlshogi feature API's raw
`pos.attacksFrom` planes for every piece kind, including kings, to reject any
already-attacked nonmoving king before LegalAll enumeration or push. This is a
reference-input limitation; native mShogi's u32 counter and analysis-position
contract remain independently specified. Thirteen actual rejection sentinels
include both historical bad roots and range/material/syntax boundary cases.
The reviewer also independently audited every current101+68 root geometrically.

## Actual commands and results

Commands ran from the next-task worktree on macOS arm64, with the same isolated
cshogi1.1.0 binary pinned to6985fe5b516c34072e27c63cf429847f03193522 and admitted
in [cshogi-admission.json](reference/cshogi-admission.json). Python/build/compiler
provenance and the unchanged reference extension hash remain recorded there.

```sh
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --diagnostics-demo --report docs/m4/reference/reference-run.json
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --fixtures tests/fixtures/shogi/legal-completeness.json \
  --report docs/m4/reference/completeness-reference-run.json
```

Current board corpus:101 expected-fact cases passed,13 domain rejection
sentinels passed,4 API-gap sentinels passed and4 deliberate mismatch/failure
minimization demonstrations passed. Separate completeness corpus:68 expected-fact
cases passed,13 domain and4 API-gap sentinels passed. The basic fixture command
checks named complete legal sets, membership/check/state facts and exact
reference reversal. Optional expected perft/divide fields are checked by the
integrator's dedicated compare_perft.py command, not silently claimed by the
basic fixture command.

```sh
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --probe 'target/release/examples/shogi_probe' \
  --playouts 1 --plies 24 --seed 20261026 \
  --report docs/m4/reference/native-board-comparison.json
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --fixtures tests/fixtures/shogi/legal-completeness.json \
  --probe 'target/release/examples/shogi_probe' \
  --playouts 1 --plies 24 --seed 20261026 \
  --report docs/m4/reference/native-completeness-comparison.json
```

The corrected101 corpus produced99 unique-root seeded sequences,2327 played
plies and2426 compared prefixes, with zero mismatches. The68 completeness roots
produced68 sequences,1523 plies and1591 prefixes, with zero mismatches. A path
stops when LegalAll is empty, so not every path reaches24 plies. At each prefix
the subprocess compares exact canonical SFEN, complete move/drop/promotion set,
check and native undo-restored status. Roots, deterministic seeds, ordered
paths and tool/probe hashes are retained in the two reports. The comparison
never uses cshogi parser acceptance, is_legal, is_draw or is_game_over as a
legality/outcome oracle.

The native probe was built by the integrator using stable Rust1.89. Its actual
SHA-256 for both commands was
`9352288fe27729cfa31984bb1d1d659f13c2a201c3a6d443040312be35f2d76c`.
The comparison executed at Git HEAD368c613348b07dd37f05415ded63bedb6ee26b29
with the MSH-66 source changes still in the working tree. Source fingerprints
below record that bounded comparison state; the integrator records the later
integration commit. GPL reference code remains in the separate development
process and is not bundled with mShogi.

[Native perft/divide evidence](reference/native-perft-comparison.json) is produced
by the integrator across both corpora; independently expected900/30 facts are
identified separately from other differential observations in that report.
This bounded evidence does not close MSH-41, history/outcomes in MSH-67, engine
integration, user interfaces or future match procedures.

## Compared source fingerprints

| File | SHA-256 |
| --- | --- |
| `tests/fixtures/shogi/board-rules.json` | `6a5356e77fe9508df27afcbb6ea66d2e3ea6397ed7f40994161b5583144dcbb1` |
| `tests/fixtures/shogi/legal-completeness.json` | `a562a265da4c49ca09702b14c2ca16b1a9eb7db9b779f426d0da3cd782cfc902` |
| `tools/shogi/reference.py` | `658f7a978c6678ca957115fa39a576ae056f299dcb414f89ca5a8c7f9f47aba1` |
| `crates/mshogi-core/src/shogi/action.rs` | `569fee1952af078f55aa192b864c4dc588ff1faf2a15af166bd637b7a2519235` |
| `crates/mshogi-core/src/shogi/legal.rs` | `a79edeef800bfc6dc316b6363cc408011189fd9b9018f05987b413ba3bea96cd` |
| `crates/mshogi-core/src/shogi/mod.rs` | `5d307c27662518e4b22b71b441ef449baac5cbd4771d35d8c41e38cae0de7ef8` |
| `crates/mshogi-core/src/shogi/perft.rs` | `bc98a1ae0384ac3f1b53cc0bf0b5af49f3ef46f39f20cb42a0a6152da832f0fd` |
| `crates/mshogi-core/src/shogi/sfen.rs` | `63a25a516fa58b4279a4a2436236c4dabcd67c4bd8037c5f8af542721ccb7bf8` |
| `crates/mshogi-core/src/shogi/tests.rs` | `efe48aa6fb2f59f69eb191cdfb869669eae748fc9acc7183575a3403bdcd8403` |
| `crates/mshogi-core/examples/shogi_probe.rs` | `0af107c745054e2f891623d395afb441ac7a18e23c364fb1b0b377ef55dbed5d` |
| `Cargo.lock` | `abcf13906b91498b343c21b7da9b35fb8d046ed09952a64536435a8260899693` |
