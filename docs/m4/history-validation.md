# MSH-67 native history verification

## Current continuation

The [reviewed proof resolution](mutual-check-resolution.md) discharges the
previous mutual-check blocker without changing `mshogi.shogi.csa36.v1`.
The supplement was inspected as a proposed argument, checksummed, executed and
reviewed independently. Complete-chain and geometry reviews establish the
admitted-root theorem, not an exhaustive search of legal shogi states or formal
proof-assistant certification. The original incomplete review remains unchanged.

The preserved `f259bb2` game/history/record implementation is reused. Final source
changes replace provisional mutual rejection with an internal diagnostic retaining
candidate context, pending command, SFEN, occurrences and stored interval checks.
Checked prefix reconstruction and record replay preserve diagnostic type and
context. Private test-only corruption exercises failure atomicity and proves the
actual commands replay to an ordinary one-sided checking loss. Broad structural
analysis and explicit imported-root provenance remain unchanged.

Executed in the resumed macOS arm64 worktree, repository-pinned Rust/Cargo1.89.0:

| Check | Result |
| --- | --- |
| Supplement `shasum -a 256 -c SHA256SUMS` | All seven entries matched. |
| `python3 docs/m4/mutual-check-supplement/verify_geometry.py` | Agreement:23 normalized pairings,20 offsets,46 reflected pairings; two endpoints per bishop, side-exchanged symmetry. Conditional necessary geometry only. |
| Independent graph audit |84 cyclic fixed-king/side/radius graphs, one simple cycle each; arbitrary walks cannot silently combine several cycles. |
| `cargo fmt --all -- --check` | PASS. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | PASS. |
| `cargo test -p mshogi-core --lib --test shogi_history --test shogi_record --test shogi_legal --locked` |35 PASS:13 library (two new private corruption tests), five history, seven legality and ten record tests. |
| `cargo test --workspace --locked` |277 PASS,0 failed, two pre-existing deep chess diagnostics ignored,42 suites including doc tests. |

A fresh [final implementation review](history-final-review.md) independently
checks final source fingerprints, proof-domain enforcement, candidate/error
propagation and the same35 focused tests. Earlier fixture/game/record reviews
remain evidence for unchanged behavior. Only the corpus's mutual-case resolution
metadata changed; `cases`, `declaration_cases` and `command_cases` were compared
to `f259bb2` and are identical. Local checks do not establish other platforms.
Exact source/integration identities and final native CI links/results are recorded
in MSH-67's completion record; source fingerprints are in the final review.

The prior [research-handoff native run40317964799](https://github.com/jweathers777/mShogi/actions/runs/40317964799)
first failed its Ubuntu runner test at `crates/mshogi-runner/tests/runner.rs:172`:
the early-exit case expected `UnexpectedExit` but observed `MissingResponse`
with a handshake broken pipe. A single failed-job retry passed without source
change. This remains evidence of an intermittent classification issue; no
runner change or root-cause resolution is claimed. M1/M2/M3 and MSH-63–66
evidence, MSH-61 complete-save admission and the
roadmap correction remain intact. No engine, console, shared transport/archive
or desktop shogi consumer is delivered by this task; MSH-41–47 and M4 remain open.

## Historical partial implementation evidence at f259bb2

The following report records the earlier blocked stage. Its open-gate status and
next-action statements applied at that revision and are superseded by the current
resolution above. Its executed checks and review provenance remain evidence.


Status: **not accepted for integration or completion**. The mutual-check gate is
open; see [the exact research blocker](mutual-check-proof-review.md). The native
implementation and records are retained on `codex/shogi-history`, based on
`986a5277f26fe2f41c671bc0f14d2da5f749a156`. This evidence does not close MSH-67.

Local environment: macOS27.0.1 build26A434, Darwin arm64, Rust/Cargo1.89.0,
repository-pinned stable toolchain. Checks on the assembled source:

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed after formatting the assembled modules. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed. An earlier constant-assertion lint in a new test was corrected. |
| `cargo test --workspace --locked` | Passed: 275 tests, zero failed, two pre-existing deep chess diagnostics ignored, across42 suites including doc tests. |
| Focused new history tests | Five passed, including21 independent history/import ledgers, nine declaration cases, exact undo/prefixes, atomic rejection and same-position/different-history. |
| Focused new record tests | Ten passed, including all fixture prefixes, strict corruption/unsupported/duplicate/missing/unknown field rejection, same-ply terminal prefixes and complete320-event records. |

The first record tests exposed Serde's internally tagged unit-variant extra-field
behavior. Empty struct variants now enforce rejection; regressions retain this
finding. Maximum-length play and319 actions plus a terminal declaration both
round-trip, supporting the conservative complete-save capacity argument.

Independent review was performed by separate Codex AI sessions, not by a human
rules authority:

- [Fixture derivation](history-fixture-derivations.md): expected facts constructed
  from primary clauses before native adjudication;158 board prefixes corroborated
  by the existing board core, not used as an outcome oracle.
- [Game review](history-game-review.md): five focused tests passed and the minor
  eligibility documentation ambiguity was resolved.
- [Fresh implementation review](history-implementation-review.md):33 targeted
  Rust tests passed; independent Python geometry/arithmetic audit verified all21
  history roots/ledgers and nine declaration diagrams. No additional material
  defect found. Source fingerprints and actual method are in the report.
- [Mutual-check proof review](mutual-check-proof-review.md): bounded source and
  argument inspection did not establish Table1 exhaustiveness. No legal mutual
  cycle or complete impossibility proof was obtained. The provisional rejection
  branch fails the promised complete live-action contract and blocks acceptance.

MSH-63–66 source/reference evidence, the broader structural-diagram regression,
M1/M2/M3 evidence, MSH-61 persistence and the roadmap correction are unchanged.
No console, search, transport, shared archive or desktop shogi work was started.
No new GUI/opponent campaign is claimed or required for this native task.
Cross-platform CI, source integration and completion must be reported separately;
local passing checks do not establish Windows/Linux execution or resolve the
rules gate. The next authorized work is the precise remaining MSH-67 research
obligation, not MSH-68/69/70.
