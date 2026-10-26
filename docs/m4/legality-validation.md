# MSH-66 complete local shogi legality

This batch implements the **board-position** portion of approved
`mshogi.shogi.csa36.v1`. It does not implement a shogi game/match, history
adjudication, declaration commands, search, USI protocol, console or desktop.
Those remain the explicit dependent tickets; MSH-41–47 remain open.

## Native API and semantics

`mshogi_core::shogi::Position` exposes geometric `attackers`,
`is_square_attacked`, and `is_in_check(side)` separately from
`pseudo_legal_actions`, `legal_actions` and `is_legal`. Candidates check geometry,
resources, promotion, nifu and dead ranks; full choices additionally check king
safety and pawn-drop mate. Both queries return complete actions in deterministic
USI lexical order. Equal pieces in a hand never duplicate a choice. Every legal
promotion and nonpromotion remains distinct, including check evasions.

`apply(Action)` resolves effects and commits atomically; `undo(Undo)` restores
all local fields only against the exact saved post-state. Capture demotes the
victim's base kind into the capturer's hand. Kings cannot be captured or dropped.
Errors distinguish rule rejection and counter representation failure. There are
no chess castling, en-passant or pawn-double-step actions. Private structural
helpers remain inaccessible to external clients; their temporary MSH-65 unused
annotations have been removed.

A checking dropped pawn is adjacent to the opposing king, so a defense must
move that king or capture the pawn. Enumerating all structural replies and
rejecting each that leaves the defending king attacked is exhaustive here:
there is no interposition square and a drop cannot capture. This avoids
recursive pawn-drop-mate checks while retaining promotion alternatives,
compulsory promotion and pinned-defender restrictions. Existing pawn advances
and non-pawn mating drops remain legal. Only a check from the newly dropped
pawn invokes this prohibition, even for imported diagrams already attacking the
other king.

`repetition_identity()` contains board ownership/promotion, both owners' hands
and turn. It excludes counters and provenance and counts no occurrences.
`perft(depth)` and `divide(depth)` traverse local legal choices, without match
conclusions or search pruning; they normalize counters on private copies.
Depth zero counts one; positive depth at an empty-choice node counts zero.
Divide rejects zero depth. Arithmetic/traversal failures are explicit, never
silently skipped. Cost is exponential; callers choose bounded diagnostic depth.

Local structural imports still do not prove reachability or safe prior play.
The declared positive u32 next-ply and exact root provenance are preserved.
Legal queries remain complete at u32::MAX; committing a move there returns
`CounterOverflow`, leaving state untouched. Match length and known-history
adjudication remain MSH-67. No result is invented from missing history.

## Verification boundary and reference-domain correction

MSH-64's original 101 direct fixture observations and review remain historical.
Longer MSH-66 sequences/perft exposed two bishop-evasion roots already checking
**both** kings. After a king evasion, native geometry correctly found three Gote
replies, whereas cshogi's incremental check cache reported no check and twelve
choices; a longer upstream path even enumerated king capture and could not
restore it. This was an unsupported reference root, not an expected-answer vote.

MSH-64 was reopened. Original fixtures, report and admission manifest are kept
as `*-v1.json`. The two current records have explicit version2/supersedes
provenance: the far king is relocated to a reviewed unattacked square, keeping
the independently expected promotion/evasion facts unchanged. The harness now
rejects unsupported roots before move enumeration/push, using source-audited raw
geometric attacks rather than cached check state. Its 13 domain sentinels cover
the old diagrams, local invariants and the reference's narrower signed-int32
counter range. The native u32 contract is unchanged. See
[domain follow-up review](reference-domain-review.md).

The original double-checked diagram is retained in native tests with its
independently derived three-reply expectation. The initial perft disagreement
is preserved in [the failed diagnostic report](reference/native-perft-initial-domain-failure.json).
Current successful comparisons use only the admitted shared domain.

## Actual local execution

Executed on macOS arm64, Rust/Cargo1.89, source bytes recorded in
[legality-validation.json](legality-validation.json). The developer-only cshogi
build/provenance is [recorded separately](reference/README.md). No Windows/Linux
reference execution or fresh GUI verification is inferred from these local runs.

| Command / evidence | Result |
| --- | --- |
| `cargo test --locked -p mshogi-core --test shogi_legal` | Seven groups pass: both independently reviewed corpora, all-choice exact reversal, atomic rejection, counter/identity, attacks/candidates/legality separation, three deterministic96-ply paths, unsupported-reference native regression, and compulsory-promotion pawn defense. |
| `cargo test --workspace --locked` | 260 pass, zero fail; two existing deep chess diagnostics intentionally ignored by the normal suite. |
| `cargo fmt --all -- --check` | Pass. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Pass. |
| `cargo build --locked --release -p mshogi-core --example shogi_probe` | Pass; binary hash is bound by the comparison reports. |
| Admitted reference vs native, seed20261026 plus root-index offsets, one24-ply bounded playout per root | 167 sequences,3,850 executed plies,4,017 compared prefixes, zero mismatches. Each prefix compares the complete LegalAll set, SFEN state, check and native exact unwind. Some terminal local roots end before24plies. |
| `compare_perft.py` with both current corpora and depth2 | All169 cases agree on total and every root divide. Sente/Gote initial totals900 and every divide30 are independently derived; remaining counts are explicitly differential observations. |

The [101 board cases](../../tests/fixtures/shogi/board-rules.json) and
[68 completeness cases](../../tests/fixtures/shogi/legal-completeness.json)
retain rule sources, derivations and actual independent AI review provenance.
The latter supplies complete central/edge movement sets for every base/promoted
kind on both sides, blocked bishop/lance rays, exact mate/noncheck empty sets,
and initial perft/divide expectations. A no-check empty set has no local chess
stalemate result; the selected CSA loss is implemented later in MSH-67.

Detailed reference observations are in
[native-board-comparison.json](reference/native-board-comparison.json),
[native-completeness-comparison.json](reference/native-completeness-comparison.json)
and [native-perft-comparison.json](reference/native-perft-comparison.json).
The harness retains roots, seeds, ordered prefixes and distinct mismatch/failure
reports. Original admission demonstrations remain four API gaps and four injected
missing/extra/exit/truncated-response failures minimized to the first bad prefix.

Independent code review found and verified a perft-tool reporting issue: dedup
previously skipped later independent expectations sharing a root. Dedup was
removed; an intentional901 expectation correctly fails against900. A separate
maximum reference-counter test now rejects before launching its sentinel probe;
replay plus diagnostic depth must fit the reference's signed-int32 domain.
These were verification-tool findings, not altered product rules. Actual review
methods and scope are in [legality-review.md](legality-review.md),
[completeness-fixture-review.md](completeness-fixture-review.md), and the domain
follow-up. No independent human review is claimed.

## Using the library and developer diagnostic

```rust
use mshogi_core::shogi::Position;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut position = Position::initial();
    let undo = position.apply("7g7f".parse()?)?;
    assert_eq!(position.legal_actions().len(), 30);
    position.undo(undo)?;
    Ok(())
}
```

The JSONL example is verification instrumentation, not an implemented shogi
console or engine protocol. It is not copied into ordinary desktop packages.

```sh
cargo build --locked --release -p mshogi-core --example shogi_probe
printf '%s\n' '{"root_sfen":"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1","moves":["7g7f"]}' | target/release/examples/shogi_probe
```

It bounds input to64KiB, prefixes to512plies and optional diagnostic perft to
depth2; malformed or unsupported requests exit with an explicit diagnostic.
No engine work, canonical game restoration or frontend session acceptance is
provided by this example. Existing chess application launch instructions and
M2/M3 evidence remain applicable without a new shogi UI claim.

## Integration and CI

Native source commit `0845392` is integrated and pushed as
`6f6301e5a6b90a1dca986e88c6ce6e022f07e5d7` on `codex/rewrite`.
All recorded native/tool source fingerprints were checked against that actual
integrated tree; the independent final review also checked Git source bytes and
report bindings. Subsequent completion bookkeeping changes documentation/tasks
only and reuse this exact source evidence.

- [Rust workspace CI40305615224](https://github.com/jweathers777/mShogi/actions/runs/40305615224)
  **PASS** on Ubuntu24.04, macOS15 and Windows2022: format, warnings-denied lint,
  workspace tests, isolated desktop constructors, native snapshot fixtures,
  workspace build and existing chess console smoke.
- [Desktop regression CI40305615506](https://github.com/jweathers777/mShogi/actions/runs/40305615506)
  **PASS** on all three platforms, including its normal package/bundled-engine
  and existing application regression checks. This is compatibility evidence;
  no shogi desktop consumer or new manual GUI verification is claimed.

The integrated diff leaves chess rules, search, engine, runner, desktop and
M2/M3 sources/evidence unchanged. Their existing deeper evidence is reused.
MSH-61 lossless persistence admission, MSH-40 roadmap correction and milestone
records remain unchanged. MSH-41–47 remain open with their original criteria.
The next dependency-ready bounded shogi ticket is MSH-67, which this batch does
not start. Its named history/declaration/import fixture gates remain explicit.
