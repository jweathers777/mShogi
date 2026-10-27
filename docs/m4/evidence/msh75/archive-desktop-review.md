# MSH-75 independent integrated archive and desktop review

Reviewer: delegated fresh agent `msh75_archive_desktop_review`, separate from the
sole integrator. Reviewed source: `80d6b5cc71c8ceb5bf782d93295797aed1e18d66`
on `codex/rewrite`, 2026-10-27. This review found **no material defect or MSH-41
blocker** in its bounded native-session/archive/normal-desktop scope. It does
not independently close MSH-75, MSH-41 or M4.

## Method and authority

Read repository AGENTS, Backlog overview, the full owner MSH-75 request, design
boundaries A-01–07, MSH-41 AC1–8, MSH-75, the shared session/archive contracts,
archive validation, selected shogi coverage and final product specification.
The installed Backlog CLI reports 1.55.0; no task records were mutated by this
reviewer. Existing root-owned MSH-75 task edits were preserved. No product code,
fixtures, task state, live GUI, Git commits or integration were changed.

Traced actual native `GameRecord` → `CanonicalRecord` → shared controller → v2
archive worker → normal native host → ProductApp/ShogiApp/ShogiBoard paths.
Read their tests and the MSH-73 and MSH-74 final-consistency evidence. Independently
executed focused native/frontend/host tests and reproduced retained artifact
hashes, selected report invariants and source applicability. This is independent
AI source/test/artifact review, not human review, new rules-source derivation,
new platform GUI execution or owner visual approval. The integrator separately
owns the fresh normal macOS end-to-end journey and complete-delivery verdict.

## Acceptance mapping and findings

| Original obligation | Integrated finding and evidence |
| --- | --- |
| MSH41AC2: rules authority, complete actions, history and identities | `shared.rs:811` checks exact game/document/version/revision/selection before dispatch. `shared_archive.rs:310` applies native complete commands on a private candidate; `:359` validates and admits responses separately from presentation. `shared.rs:974` matches exact pending request/native record/purpose/limits/session identity. Frontend board entry submits a supplied complete action; it computes no nifu, safety, promotion eligibility or outcomes. Native/test review of game rules remains the separate rules-review track. |
| MSH41AC6: complete canonical persistence/replay | `shogi/record.rs:158` requires selected profile/edition/mode/policies, canonical root/counter/history provenance, every complete actor/command/event/derived prefix, and conclusion/score equality after native replay. `shared_archive.rs:85` and `:528` preserve raw nested native bytes. `:659` replays ordered controls and observations, requires exact current-record/settings equality, and restores exact event selection through the journal. Standard/imported, capture/demotion/hand/drop, both promotion alternatives, forced promotion, repetition/continuous check, declaration/false declaration and resignation records are represented in the existing archive corpus. Those round trips prove persistence, not independently sourced rule correctness. |
| MSH41AC6 and MSH-61: lossless capacity | Private candidate admission checks actual complete encoding before accepted contents change (`shared_archive.rs:228`, `:828`, `:1074`). Existing 16 MiB, 256-observation and 8192-control limits remain. Shogi uses its native 321-event limit. Stop/close reserves one control, 1024 bytes and one replay-work unit, including after stopped reopen/preferences. Maximum-width identity encoding and historical-flag growth are budgeted. No accepted history or observations are truncated. Fresh real 256→257, byte/work/control/reservation, maximum-history and complete Save/Open tests passed; measurements below. |
| MSH41AC7: stopped reopen, history, stale results, failure/recovery | Open validates off-path and installs only against the approved current token (`shared_archive.rs:1010`). It raises runtime identities above current/saved execution floors, clears pending/current analysis, marks observations historical, and pauses until explicit Resume. Saved pending work cannot restart. Same-board/different-history and same-ply/different-event tests exercise distinct contexts; Undo/replay and retired-branch tests reject earlier execution identities. Failed/unsupported/corrupt/stale Open retains accepted contents; interrupted/failed Save retains destination/dirty semantics. Native atomic writer and host exact-revision guards remain authoritative. |
| MSH41AC7: actual normal product and accessible complete entry | Normal host `main.rs:173` onward routes native snapshots, strict raw commands, read-only inspection, native file pickers and New Game; its mutex/command/file/close reservations serialize authority. `ProductApp.tsx` presents native Chess/Shogi and Sente/Gote/Random assignment with dirty/stale guards. ShogiBoard uses native owners/kinds/promoted flags, complete board/hand choices, explicit optional promotion and direct sole mandatory promotion. Arrows, Enter/Space, Escape, hand navigation, promotion/dialog focus and stale partial-choice retirement have focused frontend coverage. Status/recovery/analysis/Return/Resume use supplied native capabilities and observations. |
| MSH41AC7: read-only inspection/analysis | `shared_inspection.rs:61` takes an immutable controller, uses exact selected native prefix, and never polls or starts/accepts/cancels work. Opponent board/hand movement uses native hypothetical-owner position queries, explicitly separated from actionable choices; chess en-passant ownership stays native. `useMovementInspection.ts` and ShogiBoard retire superseded/unmounted/history-changed queries; native/front-end tests and retained complete before/after GUI state comparisons cover this. Analysis submits MoveAnalysis and preserves exact native event/board-ply target; it cannot enter move/declaration admission. |
| MSH41AC7: shared shell/full board | Actual shared ApplicationShell/GameArea components and common dark theme retain game-specific Shogi wood, kanji, promotion and ownership/hand semantics. Current Windows/Linux normal/minimum reports and directly inspected 760×650 images show all 81 squares and both player areas plus help/footer, without board scrolling/zoom/pan. Shell CSS sizes the board from available width and height; sidebar scrolling is separate. Neither DTO geometry nor shell architecture imposes scroll/zoom/pan on future 12×12/16×16 boards. Those future games still need their own fit/readability work. Owner-approved appearance was not redesigned. |
| MSH41AC8 and MSH75AC2–3: honest integrated evidence | Source-specific native/frontend/host tests, actual normal GUI phases, separately gated verification phases, prior independent reviews and owner visual approval remain separate categories. The final CI/artifact/source audit below supports reuse at this integrated source; it does not convert verification fixtures into standard-start normal GUI play or Mac package tests into Mac GUI execution. No required archive/desktop behavior was deferred to strength work. |

No actionable defect was identified, so there is no defect file/line/reproduction
or proposed product change to resolve. The source locations above identify the
positive contract boundaries inspected, not review comments. Complete rules,
console, USI and actual-opponent conclusions belong to their other MSH-75 tracks.

## Fresh execution

Environment: local macOS/Darwin 27 arm64; workspace Rust 1.89.0; Tauri Rust
1.90.0; bundled Node 24.19.0. All commands used source `80d6b5c` with no product
changes. Frontend commands prepend
`/Users/johnweathers/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin`
to PATH and run in `desktop/`.

| Actual command | Result |
| --- | --- |
| `cargo test -p mshogi-desktop-session shared::archive:: --locked -- --nocapture` | 23 passed, 0 failed, 14.75 s. Includes all 15 archive and 8 capacity tests. |
| `cargo test -p mshogi-desktop-session shared::inspection --locked` | 5 passed, 0 failed. |
| `cargo test -p mshogi-desktop-session shared::tests --locked` | 16 passed, 0 failed. |
| `npm test -- src/product` | 66 passed in 6 files, 0 failed. |
| `npm run check` | Strict TypeScript passed. |
| `cargo +1.90.0 test --locked --all-features --manifest-path desktop/src-tauri/Cargo.toml` | 8 passed, 0 failed. Existing vendored Tao warnings remain visible. |

The first host invocation omitted `+1.90.0` from repository root and failed before
compilation because Cargo selected the workspace's 1.89 toolchain. The explicit
correct host toolchain then passed. This was an invocation error, not a passing
run or a product defect. The initial ad hoc artifact audit verified Linux and
Windows, then stopped because macOS uses a dictionary rather than list index;
the corrected audit handles both formats and passes all three.

Fresh actual archive measurements reproduce the existing evidence:

| Exercised boundary | Observation |
| --- | --- |
| 256 actual zero-node native observations; reject 257 before accepted mutation | 579,491-byte complete archive; reopened Save As 579,235 bytes; all 256 retained/historical; Resume still refused at capacity. |
| Reserved final close/control | 8192-control complete archive 698,566 bytes; replay/reopen succeeds; additional control refused. |
| Native 320-action ledger | Native 79,733; request 80,219; actual response 80,497; archive 241,088 bytes. |
| Long-history byte boundary | 205 contexts byte-fit; 206 encode 16,803,489 bytes and are refused. The fixture repeats a real native result with distinct contexts; it does not claim 205 separate engine executions. |
| Earlier-binding replay-work boundary | 10 observations accepted; next completed response refused before accepted mutation; complete retained 968,350-byte Save/Open passes. |

Full workspace/protocol/format/lint and fresh normal macOS GUI verification are
the integrator's separately recorded checks; they were not silently inferred
from these focused runs.

## Exact-source platform evidence applicability

`git diff --name-only 1b10277..80d6b5c -- crates desktop .github Cargo.toml
Cargo.lock rust-toolchain.toml` yields only `desktop/README.md`. The intervening
commit is completion/evidence documentation, with no production source,
dependency lock, build configuration, platform workflow or verifier-code delta.
The final-consistency production change is `abe66c5`; `7df91d3` only regenerates
native fixture serialization, and `1b10277` only corrects external driver locale
and singular-ply expectations. Therefore retained platform behavior applies to
the reviewed production source without repeating unchanged platform interaction.

Reproduced artifact audit:
[archive-desktop-artifact-audit.json](archive-desktop-artifact-audit.json).
All retained sizes/SHA-256 values match: Linux 73, Windows 71, macOS 31 files.
Both original GUI reports explicitly identify `1b10277436b94541235e15189a3af7fce867351c`
and pass 14/14 phases, comprising **3 normal application phases and 11 separately
labeled verification phases per platform**. Both retain 16 fully equal
inspection before/after state objects and 14 host/engine absence observations
before driver teardown. Normal Shogi minimum/normal measurements report 81 visible
squares, both player areas, essential controls and loaded local font. The review
also directly viewed both current 760×650 Shogi client images. The reduced-material
capture/hand archive keeps supplied counter 17 and unknown imported prehistory;
it is not evidence of standard-start reachability.

Retained final [Rust CI 40502381395](https://github.com/jweathers777/mShogi/actions/runs/40502381395)
and [Desktop CI 40502381314](https://github.com/jweathers777/mShogi/actions/runs/40502381314)
metadata each records success on Windows 2022, macOS 15 and Ubuntu 24.04 at
`1b10277`. This reviewer audited the retained original metadata rather than
dispatching or independently fetching new CI runs. The prior independent
[platform audit](../msh74/final-consistency-2026-10-27/platform-audit.md) supplies
the broader original artifact/package/PID binding inspection; this fresh audit
rehashes the durable subset, not absent installers or all full CI artifacts.

Mac retained normal CUA journeys, package hashes and AX/screenshot evidence are
separate from package-only macOS CI. Windows large native captures are clipped
by its 1024×768 virtual desktop; full client images and complete minimum native
captures retain the documented fit evidence. Linux evidence uses X11/Xvfb with
WebKit compositing disabled; no Wayland claim is made. Earlier failed driver
expectations, fixture-drift CI and cancelled superseded runs remain failures or
cancellations; current success does not relabel them.

## Limits and disposition

This bounded review supports MSH41AC2/6/7 and archive/desktop portions of AC8,
subject to the separate complete-delivery mapping and fresh end-to-end evidence.
There is no outstanding material correctness, persistence, interoperability,
accessibility or product-contract finding from this track. Automated keyboard,
focus/ARIA and actual pointer/keyboard platform interaction are supported;
formal screen-reader certification is not claimed. Untimed local play, bounded
native engine/analysis and immutable selected profile remain the scope. This
review establishes neither competitive strength nor timed play, variation editing,
signed release distribution or other-game delivery. MSH-46/47 and M4 remain open.
