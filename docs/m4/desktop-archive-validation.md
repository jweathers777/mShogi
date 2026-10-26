# MSH-73 validation and independent review

Scope: the source commit introducing this record on `codex/rewrite`, based on
`306f3aa0bacf72d5935bce25cbd9f66fd2fd0e9e`. Task completion pins the final source,
evidence revision and native/desktop workflow URLs. MSH-74/75 remain unstarted;
MSH-41–47 and M4 remain open. Rules, profile, chess compatibility and MSH-61
promises remain intact. See [the archive contract](desktop-archive.md).

## Local focused and regression execution

Environment: macOS27.0.1 arm64, locked Rust1.89 workspace, Rust1.90 Tauri,
Node24.19.0 bundled runtime. The system Homebrew Node26 cannot start because of
its unrelated missing simdjson library; it was not substituted into project locks.

| Actual command / layer | Result |
| --- | --- |
| `cargo test -p mshogi-desktop-session shared::archive::tests` |13 passed: canonical root/history/outcome corpus, hand/drop/promotions, exact same-ply selection, retired branches, inert observations/fresh identities, strict/adversarial identities and nested parsing, frozen v1/tagged chess, interrupted/failed save, stale Open, concurrent Save As/admission, settings retaining pending context/original limits. |
| `cargo test -p mshogi-desktop-session shared::archive::capacity_tests -- --nocapture` |8 passed (20.20s isolated run): real256→257, native320-event record,8192 controls/reservations, weighted replay boundary and stop headroom, reopened active journal/preferences, runtime revision exhaustion, real byte/work measurements and complete retained Save/Open. |
| `cargo test --workspace --locked` |387 passed,2 pre-existing ignored; no failures. Includes actual chess/shogi internal subprocess consumers, existing native rules/search/protocol/runner/archive/capacity regressions. |
| `cargo test -p mshogi-desktop-session --all-features --locked` |78 passed, including separately compiled QA constructors; no failures. |
| workspace fmt/all-target clippy; session all-feature clippy with `-D warnings` |Passed. |
| Tauri fmt, all-target/all-feature clippy and all-feature tests |Passed;5 host tests. Existing vendored Tao dependency warnings remain distinct from product lints; native termination patch/provenance unchanged. |
| `npm run check`; `npm test` |Strict TS passed;86 tests/15 files passed. Three new tests protect cancelled Save-and-close/Save-before-Open and successful exact-revision save/close. Existing chess/shared/synthetic interactions retained. |
| native fixture generation |Only four shared shogi fixtures changed for document/IO/capacity metadata; old chess fixture bytes unchanged. |
| separate QA app build and native macOS interaction |Passed; real bundled engine, native Save/Open pickers and quit guard; scope below. |

### Measured capacity and failed experiments

Sizes from actual native records/observations, in bytes:

| Fixture | Encoded size / result |
| --- | --- |
| Deterministic320-action native shogi ledger |79,733 native;80,219 request;80,497 actual response;241,088 desktop archive with one observation. |
|256 actual zero-node engine analysis observations |579,491 archive;579,235 after historical marking/reopen;257th operation refused before accepted content changes; Save/Open/Save As remain usable. |
|8192 accepted cheap controls including reserved close |698,566 archive; full replay/reopen succeeds. |
| Complete maximum-history byte-only boundary |205 contexts fit under16MiB;206 encode16,803,489 bytes and are refused. |
| Final weighted work boundary |Ten maximum-history observations retained in968,350-byte archive; next completed result refused before commit; exact Save/Open succeeds under the existing15-second budget. |

An initial100,000 unweighted allowance admitted205 long-history observations
(16,717,897 bytes), whose Open exceeded15 seconds. Reducing unweighted work to
16,000 still admitted46 contexts (3,875,686 bytes) and failed the same deadline.
These failed experiments are not successful persistence evidence. The final
policy charges native events at4 units within16,000 total; cheap controls remain1,
Undo/Inspect reserve321 events, observations add8, and active journals reserve1
for close. A raw count/work preflight precedes native validation. PV validation
clones the already proven exact selected native state, avoiding redundant replay.
Both control and byte limits remain unchanged. This conservative M4 policy
prioritizes saveable/reopenable bounded documents over observation throughput;
long histories can reach the work cap well before256 observations or16MiB.

The tests cover exact apply-before-commit refusal, no truncation, stable receipts,
last-slot stop/close, saved identity growth/headroom and UTF-8 diagnostics. The
complete existing chess archive corpus, Full observations/unsupported restoration,
atomic replacement/race and MSH-61 capacity tests ran unchanged.

## Actual macOS packaged host journey

Primary builder used native macOS accessibility/computer interaction on the
separate **mShogi Verification.app**, `MSHOGI_DESKTOP_FIXTURE=shared-contract`.
The visible **MSH-73 DEVELOPMENT CONTRACT** explicitly disclaims product design.
Resume enabled7g7f; the actual bundled engine replied4c4d. A real bounded ordinary
analysis retained the two-event game. Native Save As wrote the complete v2
archive, Cmd-Q Save-and-close shut down the app, and relaunch/Open restored both
native events, observations and paused state. Unsaved replacement required an
explicit decision. Root inspection showed event0 with the live two-event timeline;
Return to live stayed paused; explicit Resume became Idle. Native quit Keep-open
preserved the modified document, then Save-and-close used its existing path.

Package hashes, exact fixture/settings/steps and saved archive are recorded in
[evidence/msh73/gui-macos.json](evidence/msh73/gui-macos.json). Screenshots:
[saved](evidence/msh73/saved-macos.png),
[reopened stopped](evidence/msh73/reopened-macos.png),
[historical root](evidence/msh73/history-root-macos.png).
This is actual host I/O evidence, not merely a serializer test. It is **not**
MSH-74 visual acceptance, accessibility certification or Windows/Linux shogi GUI
evidence. The ordinary application remains the chess product. Separately gated
contract controls are absent from normal frontend/host packages.

## Independent review method

A separate read-only agent `msh73_archive_review` traced the completed diff and
requirements, strict RawValue/native dispatch, native journal replay/provenance,
identity/selection/execution retirement, cross-game/file/admission races, atomics,
chess v1 compatibility and resource policy. It independently executed13 final
archive tests, three targeted reservation/watermark regressions, three frontend
Save/Open/close tests and session clippy/diff checks. No remaining material finding.
This is a separate reviewer executing tests and reading source, not author
self-review. Capacity measurements and actual GUI interaction are builder evidence.

Resolved findings: (1) journal-derived stop reservation survives runtime Open
pause/preferences; (2) runtime-only UI/I/O revision growth cannot poison Save;
(3) replay work reserves the close action as well as bytes/control; (4) cancelled
Save-and-close cannot discard dirty work. Raw count-first parsing and stop checks
also prevent hostile observation arrays doing native replay before count/work
refusal. The reviewer's superseded full-session run was interrupted in the old
expensive fixture and is explicitly not recorded as passed.

## Platform evidence and remaining ownership

The task completion record pins native and desktop workflows on all three target
systems. Native/controller tests, frontend tests, package/instrumentation checks,
bundled-engine checks, existing chess GUI smoke and local shogi development GUI
interaction establish their named layers only. Existing M1–M3/MSH-63–72 unchanged
evidence and the macOS termination regression remain applicable. No native rules,
source adjudication expectations, USI policy or Tao patch changes are introduced.

MSH-74 next owns the actual shogi application design and packaged interaction,
including full9x9 visibility, deliberate shogi pieces/ownership/promotion, hands,
status hierarchy, polished inputs and separated diagnostics. Future12x12/16x16
must support full-board display without requiring zoom/pan. MSH-75 final shogi
integration and the broader complete-delivery obligations remain open.
