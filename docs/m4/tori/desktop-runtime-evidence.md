# MSH-131 native runtime acceptance evidence

Baseline `086579f7361c5e1e395ea8f766779688446441b2`; unchanged native implementation
`994521bec0637d6f8f17103f1106c0025d950753`. [Runtime contract](desktop-runtime.md).
This is actual native/controller/process evidence, not a Tori GUI campaign.
Exact integrated source and CI results are appended after actual commits/runs.

## Original acceptance matrix

| MSH-131 | Concrete evidence |
| --- | --- |
|AC1 sole owner/native commands/admission |One ActiveDocument::Tori and RuntimeMetadata; explicit native command/settings arms; worker candidates and immutable codec from initial installation. Native Play/Resign/Stop/unsupported-command fixture tests. |
|AC2 actual82requirements/13fields |70independently expected admission fixtures including35rotations, seven rejected-input fixtures, C-HISTORY, C-IDENTITY, O-ADMIN, H-IDENTITY and H-REPLAY-UNDO. Actual controller command/inspection/worker/Open/Undo/reapply, exact records/effects/hand deltas and stale authority assertions. [Runtime evidence index](evidence/desktop-runtime-consumer-acceptance.json) links each row to a passing real test; supporting codec/native checks are not duplicate requirements. |
|AC3 exact prefix/inspection/retirement |History fixture distinguishes same board and turn at0/4/8events; selected4has[0,4] occurrences/no obligation, live8has[0,4,8]/Sente obligation. Forbidden4g5g rejected only in latter history. Immutable inspection/token nonmutation, Undo7, Take back6, retained8/7branches, Save/reopen and old-response refusal. |
|AC4 execution honesty/identities |Actual bundled internal-search→controller UnsupportedMode; actual separate forced-exit and blocked child cancellation/replacement/cleanup. Negatives for purpose, full history/game, request, session, process, limits and all target fields. No Tori result/score family or ordinary execution. |
|AC5 sole install/file/lifecycle |Actual Save/Open and stale validated completion; current identity/revision receipts; corrupted/unsupported/resource-limited/cancelled Open; exact stopped fresh reopening, historical observations; dirty/native close/unsaved replacement tokens. Normal product gate remains closed. |
|AC6 snapshot/capacity/regressions/review |Minimal actual Tori snapshot;256→257refusal followed by reserved close/Save/reopen; typed byte/control/native-event/replay/identity refusals; interrupted real Save executor preserves old file/no receipt/dirty. Existing-game controller/document/process/host regressions, independent review and exact-source CI recorded below. |

## Consumer accounting and independently expected inputs

The immutable103fixture corpus, native ownership manifest and selected profile
are unchanged. Native adapter still reports713executed/713passed/14consumer fields
pending, because it does not operate a desktop controller. MSH-131 separately
executes **13of those fields and82consumer requirements** in the actual controller.
Only C-NOTATION's1field/1requirement remains MSH-42; MSH-132 still owns complete
presentation/frontend supporting verification and MSH-77 final acceptance stays
open. Historical planning/native reports are preserved rather than rewritten.

The H-IDENTITY handed-quail diagrams are structural repetition-key fixtures:
with Sente to move, a quail at4d attacks the nonmoving Gote Phoenix4a, so native
Game admission correctly rejects them. The tests preserve these exact structural
inputs/key assertions and verify actual Open rejection/nonmutation. Separately
labeled supplementary **Gote-to-move** diagrams establish positive runtime
handedness/context preservation. Neither the fixture nor its expectation changes;
structural validity is not misrepresented as a playable root.

## Actual local verification

macOS27.0.1 build26A434, Apple arm64, Rust1.99.1. No application launch, new bundle
identity, third-party engine or Tori GUI verification was needed.

```text
cargo test --locked -p mshogi-desktop-session shared::tori --lib --all-features -- --nocapture
cargo test --locked -p mshogi-desktop-session --all-features
cargo test --locked -p mshogi-core tori:: --lib -- --nocapture
cargo test --locked -p mshogi-console --test desktop_tori --test desktop_shared --test desktop_persistence --test desktop_analysis --test desktop_play --test tori_internal
cargo clippy --locked -p mshogi-desktop-session -p mshogi-console --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
cargo check --locked --manifest-path desktop/src-tauri/Cargo.toml --all-targets
cargo test --locked --manifest-path desktop/src-tauri/Cargo.toml --bin mshogi-desktop
```

Focused final fast controller13PASS, plus separate actual long-runtime1PASS; additional runtime-floor byte-boundary1PASS; native
38PASS/713assertions; six actual console test targets15PASS (including2new Tori
controller/child checks). Standalone Tauri host8PASS; Clippy/format/check pass,
with20existing vendored Tao warnings retained. Full integrated session checkpoint242PASS (before the last three focused controller
checks and floor-width test); exact-source CI results are recorded below after completion, not inferred from these checks.
No frontend code changed; complete Tori frontend verification belongs MSH-132.

Long deterministic generator seed`0x129ca11ab1e5`, same native history as prior
measurements. Isolated actual controller installation5451ms, Stop6907ms,
Resign6868ms, Save4142ms; close plus snapshot6ms and Save enqueue0ms. These are
observations, not guarantees. Document usage is313906B/196404units initially,
314540B/196409units after reserved endings. The final isolated read-back measures
**314541file bytes**, because Save refreshed the retired floor to11; the extra
one byte is runtime metadata width, not discarded/added game history. A dedicated
maximum-floor-width reservation and scalar exact-byte/+1 check protects this even
after ending headroom is consumed. [Actual long measurement](evidence/desktop-runtime-measurements.json)
records both accepted candidate and saved-file usage. Concurrent full-suite/reviewer timings
may differ and are not represented as isolated benchmarks.

## Independent review and resolved findings

Separate read-only `tori131_review` source/runtime/admission/security review and
independent command execution. Root alone built/integrated. Reviewer reproduced
10controller checks including the real long runtime path, then independently
passed the added typed-identity/interrupted-Save regression. Reviewer also independently passed the outgoing replacement and floor-width
checks, and independently checked the exact82/13 mapping. Final production
source review PASS with no unresolved material finding; final documentation/CI
reconciliation follows actual results below.

Resolved findings:

- Full archive clone/encoding during Save/close preparation moved off controller
  lock: Arc capture, existing-worker floor refresh, precomputed close candidate.
- Document validation units distinguished from finite additional runtime native
  reconstruction; actual long controller timing and deadline limitations recorded.
- New admission identity exhaustion uses authoritative typed resource facts.
- Outgoing Tori→Chess/Shogi/Xiangqi replacement refuses before retiring current
  work; destination worker fault tests and public Chess capacity refusal preserve
  complete source data/pending work. Successful cross-game swaps cancel it.
- Actual cancellation waits for a separate child PID/start marker, proving running
  process termination rather than only cancellation of queued work.
- Repeated close retains the exact saved-document stop-only exemption; no dirty
  revision is falsely marked saved.
- A completed response refused by capacity cannot remain falsely Thinking; its
  unaccepted result is not silently added/truncated or converted into an outcome.

Historical intermediate failures, not passing claims: initial compile errors
from !Clone native Game, missing exhaustive test helper, private fact getter and
incorrect FailureKind names; corrected using immutable drafts/native reconstruction
and actual APIs. A blocked-child test initially used zero nodes, which the existing
bridge correctly rejected; changed the separate test helper's valid sentinel to7.
An initial H-IDENTITY test incorrectly assumed structural quail diagrams were live
roots; corrected as described above without changing rules/expected data. One
Clippy boolean-style finding corrected. A broad241-test checkpoint had239PASS and
2FAIL solely from earlier129/130tests expecting native Tori to remain unsupported;
those gates now assert the authorized native-contract versus normal-product split.
The original failing checkpoint remains historical evidence, not a native defect
or a reason to weaken any game fixture. A later separately compiled child-start marker
probe exposed a test-helper publication race (file visible before its PID content
was written); publishing that marker by rename fixes the test instrumentation.
It changes no engine or product file-save semantics. Final passing results are appended below.

Actual command transcripts: [local results](evidence/desktop-runtime-local.txt).
Static audit (not execution): [scope/source/graph](evidence/desktop-runtime-validation.json).

## Handoff

MSH-132 must complete semantic metadata, native Moves/event/last-action projection,
strict frontend types and shared interaction verification using this actual
controller. It remains unassigned/unstarted. No Tori normal chooser, final artwork,
engine/search/notation or gameplay GUI claim is introduced. MSH-77 final acceptance,
MSH-42 complete Tori and M4 remain open. Stop after MSH-131.
