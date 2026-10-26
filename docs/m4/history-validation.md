# MSH-67 partial implementation verification

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
