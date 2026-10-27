# MSH-74 final shared-shell consistency pass

Product correction: `abe66c5c40f56bb98041ce0491b1004f5f9498b8` on
`codex/rewrite`. Exact native-generated fixture correction:
`7df91d37abb6ff872779188f83d03132bab3df11` (no product source or JSON value change).
Final external-driver correction: `1b10277436b94541235e15189a3af7fce867351c`
(product sources unchanged). Its exact-revision final CI is recorded below.
MSH-74 is complete under the owner's conditional authorization after final
review and verification passed. MSH-75 remains unstarted.

The owner accepts the major common-shell visual design and authorizes completion
after these three bounded corrections, focused checks, updated paired captures
and required review/verification. No additional owner visual checkpoint is
required. No layout redesign, game rules, archive schema, guard or execution
acceptance changes are included.

## Corrected presentation

| Shared field | Contract |
| --- | --- |
| New document with no disk path | Unsaved, regardless of native dirty flag |
| Saved/opened document identical to disk | Unmodified |
| Saved/opened document changed since disk | Unsaved changes |
| Native engine panel | Title/status → actual Play limit → actual observations/actions |
| Live/history footer | Shared turn/concluded/paused/read-only status and selected native known board ply |

Native dirty/IO/admission and unsaved-close behavior remain authoritative. Labels
do not pretend a pristine untitled game already has a save. Failed/cancelled Save
cannot establish a path or clear native dirty state. Existing file operation
progress/failure and historical-observation meanings remain intact.

Shogi's native snapshot exposes its actual2-depth/5,000-node play budget from
the same constant as submitted play work. Chess retains2/20,000. Analysis defaults
and historical observations cannot substitute for that budget. Game-specific
scores, native side names and analysis operations remain distinct.

Both boards use the actual shared GamePositionStatus row after instructions;
Shogi hands remain in the player regions. Native selected-prefix board-ply is
separate from event count, terminal commands and imported SFEN counters. Historical
selection, concluded outcomes and explicit pause take precedence over Your turn.

## Focused verification and review

[Local verification](local-verification.json): Node24.19 strict TypeScript and
161 frontend tests/21files pass; native desktop-session85tests pass; extended
selected-prefix/move/terminal-ply regression passes; all-target/all-feature
session clippy with warnings denied passes. Exact native fixture generation
matches20 default-feature outputs; independent verification-feature generation
matches22 including capacity/synthetic. These are native/controller/frontend
checks, separate from actual application interaction.

[Independent source/test review](implementation-review.md) traces disk labels,
unchanged native guards, actual submitted budgets, selected prefix/terminal ply
and shared presentation. Reviewer independently ran34 frontend tests, strictTS,
one native test and12 general driver tests, all passing, with no material finding.
[Independent visual review](visual-review.md) inspected actual Mac pairs and disk,
history and pause states; no material finding. Platform checks have separate
provenance in [platform audit](platform-audit.md).

The normal Mac build and bundled-engine/package smoke pass. Original
[abe package report](macos-package.json) and
[fixture-corrected rebuild report](macos-package-final.json) have identical host
SHA-256 `a2f5fa4f05745e12e14f4e80b0349b6e203e266b2a59c4cd68fc7986d8611dcc`
and engine `5d0fa23fdd7ca5683c848c7117379aabed033288c6d9114f697492153f2f5615`.
The normal inventory excludes verification instrumentation. Existing Tao vendor
warnings/provenance and native termination regression evidence remain applicable.

## Actual normal Mac application

External CUA accessibility, pointer/keyboard and native Save/Open dialogs;
macOS27 arm64; no development harness or injected state. Paired1180×820 native
windows include the titlebar, with unaltered2360×1640 JPEG bytes. The fresh final
pair followed the native saved-game Quit guard and fresh default process;
[runtime identity](macos/runtime.json) binds exact path/hash. The prior host
terminated, but CUA may relaunch during observation: no new host-absent interval
is inferred. Earlier exact process-absence evidence and current platform native
close checks are separate.

| State | Chess | Shogi |
| --- | --- | --- |
| Fresh final normal, Unsaved | [Image](macos/chess-final-normal.jpg) | [Image](macos/shogi-final-normal.jpg) |
| Saved, Unmodified | [Image](macos/chess-unmodified.jpg) | [Image](macos/shogi-unmodified.jpg) |
| Flip, Unsaved changes | [Image](macos/chess-modified.jpg) | [Image](macos/shogi-modified.jpg) |
| Exact history | — | [Selected1ply](macos/shogi-history.jpg) |
| Return to live | — | [Paused2plies](macos/shogi-live-paused.jpg) |
| Reopen own complete native save | — | [Unmodified/work stopped](macos/shogi-reopened.jpg) |
| Both hands | — | [Open native capture archive](macos/shogi-hands-reopened.jpg) |
| Opponent hand read-only inspection | — | [Footer retained](macos/shogi-hand-inspection.jpg) |

Each image has a companion actual accessibility text file. Shogi first-save
cancellation retained Unsaved; successful Save became Unmodified; Flip became
Unsaved changes; Save restored Unmodified. Native7g7f and real engine4c4d gave
live2plies; selecting event1 showed1ply; Return to live stayed explicitly paused.
Own complete Save/Open retained both native events and stopped work. The existing
reduced-material capture archive was opened without modification to check both
hands and opponent-hand inspection/Escape; it makes no standard-start reachability
claim. [Own QA archives](macos/records/) retain complete native semantics.
The prior open session was separately preserved in `/private/tmp` before replacement;
no user session or source fixture was discarded.

The local final normal app is left running fresh Sente for review/use. No new
Mac minimum-size or successful rapid-Cancel claim is made; actual platform
minimum/cancellation evidence is recorded separately. Unchanged earlier complete
promotion, declaration, conclusions, recovery, analysis, capacity and persistence
journeys remain in the [original evidence](../README.md) and
[previous common-shell evidence](../common-shell-2026-10-27/README.md).

## CI and retained failed attempts

First Rust run40500442835 atabe66c5 failed the native-generated fixture drift
check on all three systems after tests/lint passed. Four manually appended fields
had correct values but different field ordering/format. The fixture correction
retains the drift gate and exact generator output. Final fixture-source Rust
[40501219346](https://github.com/jweathers777/mShogi/actions/runs/40501219346)
passes all three target systems.

The first supporting Linux Desktop attempt atabe66c5 failed10/14. Its driver
wrongly required a comma in native locale-dependent5000/20000 output and expected
1plies instead of1ply after promotion. Interrupted ordinary phases never created
the later reopen file, so that dependent phase failed. Native promotion actually
completed. These are failed checks, not passing GUI journeys; original report,
DOM and captures remain source-labeled. The external-driver correction preserves
exact numeric work budgets, accepts locale grouping, and uses singular1ply;
25 focused driver unit tests pass. No product behavior or invariant is weakened.
Unfinished superseded Desktop runs40500443053 and40501219202 were cancelled
only after preserving their available reports/metadata, to free runners for the
corrected final revision. Cancellation is not a pass. Rust7df all-three-system
success remains valid. Final exact-source Rust40502381395 and Desktop40502381314
results follow; no success is inferred from construction or another platform.
Final-source [Rust40502381395](https://github.com/jweathers777/mShogi/actions/runs/40502381395)
is successful on Windows, macOS and Linux; [original result](rust-ci.json).
Final-source [Desktop40502381314](https://github.com/jweathers777/mShogi/actions/runs/40502381314)
is successful on all three systems; [original result](desktop-ci.json). Actual
Windows/Linux GUI reports, artifact audit and paired visual reviews establish
their interaction scope separately from Mac CI package/native checks. Final
Windows artifact audit and independent screenshot assessment also passed.

[Independent acceptance reconciliation](closure-review.md) found no additional
functional gap. Its final-platform conditions have now passed, as recorded in
the final gate-fulfillment section.

## Final integrated verification verdict

Final verification source `1b10277436b94541235e15189a3af7fce867351c` is integrated
and pushed on `codex/rewrite`. [Rust CI](rust-ci.json) and
[Desktop CI](desktop-ci.json) both pass all three systems. Linux and Windows each
pass **14/14 actual packaged GUI phases**. Their independent artifact audits
verify package/host/engine/PID bindings, actual native Shogi2/5,000 play requests,
all three document states,16 complete equal inspection-state comparisons,
nine settled exact-path saves and14 native host/engine shutdowns before driver
teardown per platform. Normal distributables exclude the checked instrumentation.
Mac actual GUI and normal package/native CI scopes remain separately labeled.

[Final independent visual review](visual-review.md) compares normal, minimum,
active-inspection, analysis/history and counted-hand states from actual final
Linux/Windows packages, plus the actual normal Mac pairs and native state journeys.
No outstanding material visual or implementation finding remains. These actual
results satisfy the prior conditional acceptance verdict; all14 AC and4 DoD
items are supported, including the latest owner authorization. This closes
MSH-74 only, with no MSH-75 implementation or M4 closure.

| Final same-size application captures | Chess | Shogi |
| --- | --- | --- |
| Mac1180×820 native including titlebar | [Image](macos/chess-final-normal.jpg) | [Image](macos/shogi-final-normal.jpg) |
| Linux1180×820 client | [Image](linux/normal-initial-chess-normal-size.webview.png) | [Image](linux/normal-shogi-initial-sente.webview.png) |
| Linux760×650 minimum client | [Image](linux/normal-initial-chess-minimum-size.webview.png) | [Image](linux/normal-shogi-minimum-size.webview.png) |
| Windows760×650 minimum client | [Image](windows/normal-initial-chess-minimum-size.webview.png) | [Image](windows/normal-shogi-minimum-size.webview.png) |

Unaltered original captures/reports/manifests are indexed with source paths,
hashes and bytes: [Mac31files](macos/retained-files.json),
[Linux73files](linux/retained-files.json), [Windows71files](windows/retained-files.json).
Full CI artifacts have14-day retention; no installers or build outputs are
committed. Original CI logs retain native trailing spaces and Windows reports
retain CRLF bytes for hash integrity. Authored source/document whitespace checks
pass; exact original logs are excluded from that formatting check. Windows native desktop1024×768 crops large1180×820 native captures;
complete client images establish normal appearance, and minimum native captures
are complete. Sidebar content may scroll independently; the boards/player/footer
region fits. Earlier failures remain recorded separately, never relabeled.

## Acceptance and completion evidence map

| MSH-74 criteria | Evidence and retained scope |
| --- | --- |
| AC1–5 | Earlier complete native/product/three-platform play, outcomes, recovery, bounded analysis, canonical Save/Open and capacity journeys remain applicable; final-source GUI reports recheck the integrated paths. Native game/record/acceptance code is unchanged in this pass. |
| AC6–10 | Prior product board/font/hand/promotion/key/pointer evidence plus the final native Mac pair, updated native limit/footer data and current Windows/Linux normal/minimum checks. Technical details stay collapsed; MSH-72 harness stays excluded. |
| AC11 | Existing complete normal-package Sente/Gote, hands/drop/promotion/history/active-analysis/conclusion screenshot set, later actual shared-shell/state pairs and independent visual reviews; owner accepts the major design and conditionally authorizes Done after these corrections pass, with no further checkpoint. No screenshot is relabeled as another revision. |
| AC12–14 | Actual shared composition/components and one dark theme retained; new shared document/engine/footer components; native read-only opponent/history/hand tests and actual current normal captures; final paired normal/minimum visual review required before closure. |
| DoD | Focused tests and normal CI, bounded independent source/test/visual/artifact reviews, current design/task evidence, integrated source and normal push. No remaining material finding or unchecked criterion is permitted at Done. |

The [original gallery](../gallery.md), [native source/test review](../implementation-review.md),
[inspection refinement](../refinement-2026-10-27/README.md) and
[common-shell review](../common-shell-2026-10-27/README.md) retain exact earlier
provenance. MSH-74 completion is a desktop-product task verdict; it does not
perform the MSH-75 complete standard-shogi/M4 integration review.

## Launch and limits

```sh
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app"
```

New game selects Chess/Shogi and native human side. Untimed local play, bounded
read-only analysis, complete canonical Save/Open, exact history/Return/Resume and
read-only opponent inspection remain available. No additional real variant,
strength claim, release signing, timed play, variation editing or formal
screen-reader certification is introduced. M2/M3 remain closed; M4 and MSH-41–47
remain open. MSH-75 is outside this bounded pass.
