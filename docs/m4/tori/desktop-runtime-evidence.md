# MSH-131 native runtime acceptance evidence

Baseline `086579f7361c5e1e395ea8f766779688446441b2`; unchanged native implementation
`994521bec0637d6f8f17103f1106c0025d950753`. [Runtime contract](desktop-runtime.md).
This is actual native/controller/process evidence, not a Tori GUI campaign.
Final tested source is `6a2edd71b3136a9c2043f9137eb6d3fb8abf120d`; all applicable
three-platform native and desktop CI passed. Integrated source/provenance below.

## Original acceptance matrix

| MSH-131 | Concrete evidence |
| --- | --- |
|AC1 sole owner/native commands/admission |One ActiveDocument::Tori and RuntimeMetadata; explicit native command/settings arms; worker candidates and immutable codec from initial installation. Native Play/Resign/Stop/unsupported-command fixture tests. |
|AC2 actual 82 requirements / 13 fields |70 independently expected admission fixtures including 35 rotations, seven rejected-input fixtures, C-HISTORY, C-IDENTITY, O-ADMIN, H-IDENTITY and H-REPLAY-UNDO. Actual controller command/inspection/worker/Open/Undo/reapply, exact records/effects/hand deltas and stale authority assertions. [Runtime evidence index](evidence/desktop-runtime-consumer-acceptance.json) links each row to a passing real test; supporting codec/native checks are not duplicate requirements. |
|AC3 exact prefix/inspection/retirement |History fixture distinguishes same board and turn at 0/4/8 events; selected 4 has [0,4] occurrences/no obligation, live 8 has [0,4,8]/Sente obligation. Forbidden 4g5g rejected only in latter history. Immutable inspection/token nonmutation, Undo to 7, Take back to 6, retained 8/7 branches, Save/reopen and old-response refusal. |
|AC4 execution honesty/identities |Actual bundled internal-search→controller UnsupportedMode; actual separate forced-exit and blocked child cancellation/replacement/cleanup. Negatives for purpose, full history/game, request, session, process, limits and all target fields. No Tori result/score family or ordinary execution. |
|AC5 sole install/file/lifecycle |Actual Save/Open and stale validated completion; current identity/revision receipts; corrupted/unsupported/resource-limited/cancelled Open; exact stopped fresh reopening, historical observations; dirty/native close/unsaved replacement tokens. Normal product gate remains closed. |
|AC6 snapshot/capacity/regressions/review |Minimal actual Tori snapshot;256→257 refusal followed by reserved close/Save/reopen; typed byte/control/native-event/replay/identity refusals; interrupted real Save executor preserves old file/no receipt/dirty. Existing-game controller/document/process/host regressions, independent review and exact-source CI recorded below. |

## Consumer accounting and independently expected inputs

The immutable 103-fixture corpus, native ownership manifest and selected profile
are unchanged. Native adapter still reports 713 executed / 713 passed / 14 consumer fields
pending, because it does not operate a desktop controller. MSH-131 separately
executes **13 of those fields and 82 consumer requirements** in the actual controller.
Only C-NOTATION's 1 field / 1 requirement remains MSH-42; MSH-132 still owns complete
presentation/frontend supporting verification and MSH-77 final acceptance stays
open. Historical planning/native reports are preserved rather than rewritten.

The H-IDENTITY handed-quail diagrams are structural repetition-key fixtures:
with Sente to move, a quail at 4d attacks the nonmoving Gote Phoenix 4a, so native
Game admission correctly rejects them. The tests preserve these exact structural
inputs/key assertions and verify actual Open rejection/nonmutation. Separately
labeled supplementary **Gote-to-move** diagrams establish positive runtime
handedness/context preservation. Neither the fixture nor its expectation changes;
structural validity is not misrepresented as a playable root.

## Actual local verification

macOS 27.0.1 build 26A434, Apple arm64, Rust 1.99.1. No application launch, new bundle
identity, third-party engine or Tori GUI verification was needed.

```text
cargo test --locked -p mshogi-desktop-session shared::tori --lib --all-features -- --nocapture
cargo test --locked -p mshogi-desktop-session --all-features
cargo test --locked -p mshogi-core tori:: --lib -- --nocapture
cargo test --locked -p mshogi-console --test desktop_tori --test desktop_shared --test desktop_persistence --test desktop_analysis --test desktop_play --test tori_internal
cargo clippy --locked -p mshogi-desktop-session -p mshogi-console --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
cargo fmt --manifest-path desktop/src-tauri/Cargo.toml --all -- --check
cargo check --locked --manifest-path desktop/src-tauri/Cargo.toml --all-targets
cargo test --locked --manifest-path desktop/src-tauri/Cargo.toml --bin mshogi-desktop
```

Focused final fast controller 14 PASS, plus separate actual long-runtime 1 PASS; additional runtime-floor byte-boundary 1 PASS; native
38 PASS / 713 assertions; six actual console test targets 15 PASS (including 2 new Tori
controller/child checks). Standalone Tauri host 8 PASS; Clippy/format/check pass,
with 20 existing vendored Tao warnings retained. Full integrated session checkpoint 242 PASS (before the last four focused controller
checks and floor-width test); exact-source CI results are recorded below after completion, not inferred from these checks.
No frontend code changed; complete Tori frontend verification belongs MSH-132.

Long deterministic generator seed `0x129ca11ab1e5`, same native history as prior
measurements. Isolated actual controller installation 5455 ms, Stop 6869 ms,
Resign 6882 ms, Save 4141 ms and fresh stopped Open 5704 ms; close plus snapshot 6 ms and Save enqueue 0 ms. These are
observations, not guarantees. Document usage is 313906 B / 196404 units initially,
314540 B / 196409 units after reserved endings. The final isolated read-back measures
**314541file bytes**, because Save refreshed the retired floor to 11; the extra
one byte is runtime metadata width, not discarded/added game history. A dedicated
maximum-floor-width reservation and scalar exact-byte/+1 check protects this even
after ending headroom is consumed. [Actual long measurement](evidence/desktop-runtime-measurements.json)
records both accepted candidate and saved-file usage. Concurrent full-suite/reviewer timings
may differ and are not represented as isolated benchmarks.

## Independent review and resolved findings

Separate read-only `tori131_review` source/runtime/admission/security review and
independent command execution. Root alone built/integrated. Reviewer reproduced
10 controller checks including the real long runtime path, then independently
passed the added typed-identity/interrupted-Save regression. Reviewer also independently passed the outgoing replacement and floor-width
checks, and independently checked the exact 82/13 mapping. Final production
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
bridge correctly rejected; changed the separate test helper's valid sentinel to 7.
An initial H-IDENTITY test incorrectly assumed structural quail diagrams were live
roots; corrected as described above without changing rules/expected data. One
Clippy boolean-style finding corrected. A broad 241-test checkpoint had 239 PASS and
2 FAIL solely from earlier 129/130 tests expecting native Tori to remain unsupported;
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


## Integrated source and exact-source CI

Implementation integrated/pushed at `253e4c949614d133b5295f6a362e3d5279adcef3`;
final production formatting at `ce48e84e64b9631343416acec865505dbbab570d`;
final reviewed test/evidence source **`6a2edd71b3136a9c2043f9137eb6d3fb8abf120d`**.
Production is unchanged between the last two revisions. Final task/evidence
bookkeeping changes no production, tests, workflows, fixture inputs or expectations.

- [Native run41417589059](https://github.com/jweathers777/mShogi/actions/runs/41417589059):
  **PASS** on Ubuntu24.04, macOS15 and Windows2022. Each passed **247 session tests**
  in both workspace and separate all-features invocations, with zero failed or
  ignored tests. Actual Tori subprocess tests, lint/format, snapshot regeneration,
  build and console smoke passed.
- [Desktop run41417589127](https://github.com/jweathers777/mShogi/actions/runs/41417589127):
  **PASS** on all three platforms:323frontend tests,8normal and13verification-host
  tests per platform, normal package construction, instrumentation exclusion and
  bundled-engine/package checks. macOS also passed11stable-QA infrastructure tests.
  Linux and Windows passed actual installed/packaged Chess/Shogi/Xiangqi GUI
  journeys. macOS has package/host evidence here, **no new macOS GUI campaign**.
- [Actual CI metadata and package hashes](evidence/desktop-runtime-ci.json) records
  exact source, jobs/steps, test log counts, normal host/engine SHA256 and artifact
  links. Normal packages are Debian, macOS app and Windows NSIS; separately labeled
  Linux/Windows verification packages are distinct from normal distributions.
  None of these results claims Tori gameplay GUI, final artwork or engine delivery.

### Preserved failed and superseded attempts

Initial [Desktop41415490841](https://github.com/jweathers777/mShogi/actions/runs/41415490841)
failed the separate Tauri workspace format check at the newly added host gate
before package construction. Root workspace formatting passed but omitted that
separate manifest. Standalone formatting fixed it in `ce48e84`; both checks then
passed. No package/GUI success is claimed for the failed attempt.

Initial [Native41415490915](https://github.com/jweathers777/mShogi/actions/runs/41415490915)
reported245PASS/1FAIL on each platform: Ubuntu125178184982 and macOS125178184540
reached the existing elapsed budget at Resign; Windows125178184817 reached it at
initial installation. The360-action test wrongly required success despite the
unchanged15-second deadline. The test correction changes neither production,
rules, independent fixtures nor the262144-unit document budget.

A mandatory180-action journey now unconditionally requires Stop, Resign, prepared
Close, actual Save and fresh stopped Open. Only the separate360-action near-limit
probe permits an alternate refusal with exactly one authoritative
ValidationBudget/Application/Milliseconds fact: limit15000, measured count at
least15000. It must preserve canonical bytes, native context, settings/metadata,
observations, selected history, pending work, stable authority tokens, phase and
dirty/close state. Retained bytes and prepared close remain strictly decodable;
failed Save also preserves the prior valid file. Other errors still fail.
Independent bounded test review PASS; local14fast checks and360complete Save/Open
journey passed (37.24s total). Intermediate
[Rust41415982048](https://github.com/jweathers777/mShogi/actions/runs/41415982048) and
[Desktop41415982022](https://github.com/jweathers777/mShogi/actions/runs/41415982022)
were cancelled/superseded after this correction to avoid duplicate campaigns.

Normal CI captures passing stdout: a passing near-limit probe establishes its
success-or-precise-refusal assertions, not which branch ran or a completed360-action
journey on each platform. The local uncaptured measurement establishes actual360
completion. No wall-clock rerun guarantee is inferred from either result.
