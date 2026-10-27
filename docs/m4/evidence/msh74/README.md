# MSH-74 product review checkpoint

The normal application now supports Chess and untimed standard Shogi. MSH-74
remains open for the explicitly required owner visual review. This is not M4,
MSH-41 or MSH-75 closure; M2/M3 and their unchanged evidence remain closed.

The owner-requested shared shell and native read-only movement inspection are
integrated at `d6d76816932763182eee574c9c241daa6ef9a91a`. See the
[current refinement evidence and nine-image gallery](refinement-2026-10-27/README.md)
for the latest product screenshots, tests, independent reviews and CI. The earlier
revision-specific records below remain historical evidence; their old layouts are
superseded by this refinement. Final owner visual approval remains pending.

## Source, package and launch

Visual source: `60796ec18539f600354594f1fab6ee643b93bede`, integrated and
pushed to `codex/rewrite`; `5de7441` changes only GUI verification labels. Baseline
MSH-73 completion: `570ef0e`. Final close-routing base: `b26fd736e0b2ae5ca3e3b80b7cc1f4f0730cf085`;
Close-chooser revision: `21bfe3e2a3be2d35bd8096b6207f6a8a6af75576`
also retires the uncommitted New Game chooser on native Quit.
Those independently reviewed deltas unify native close ownership across game
view replacement. Final product correction `a9111ae83de1d91149367a136da4ed66efdefee5`
also fixes the native cross-game Open projection race described below. Board/font/
analysis layout and native game rules are unchanged.
The local normal macOS bundle is:

```sh
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app"
```

The first screen is Chess. Choose **New game → Shogi → Sente/Gote/Random → Start
game**. Human pieces start at the bottom; Flip is explicit. Build on another
checkout with Node 24.19.0, root Rust 1.89 and desktop Rust 1.90:

```sh
cd desktop
npm ci
npm run desktop:build -- --bundles app
open src-tauri/target/release/bundle/macos/mShogi.app
```

See [desktop launch/build instructions](../../../../desktop/README.md) for Debian
and Windows installer commands. These are development packages; release signing,
notarization and minimum OS support policy remain future work. Local macOS is
arm64; [the normal package report](macos-normal-package.json) records actual
host/engine hashes, inventory and real protocol/search observations. The font is
embedded, with [reproducible provenance and OFL license](../../shogi-font.md).
Normal package smoke checks exclude verification IPC/fixture/quit-probe markers.

## Working journeys and boundaries

- Explicit normal game selection and both native sides, human-bottom orientation,
  Flip, pointer moves, roving keyboard, Escape, native counted hands/drops,
  contextual optional promotion and direct compulsory promotion.
- Native turn/check/conclusion/resignation/declaration meanings. Eligibility is
  advice; formal declaration is separately confirmed and available only through
  native capability. Failures, incomplete/cancelled work and resource refusal
  do not invent a chess or shogi result. Retry retains native history.
- Read-only bounded analysis: depth 1–4, nodes 0–20,000; native play uses depth 2 and
 5,000 nodes. Results identify target event and perspective and show real final
  completed depth/nodes/evaluation/PV. There is no fabricated live telemetry.
  Replacing/cancelling work and selecting history retire execution identities.
- Exact native history, explicit Return to live and separate Resume. Full v2
  shogi archives retain root provenance, counters/known history, commands/events,
  conclusions, settings and observation contexts. Reopen creates fresh runtime
  identities and stops work; saved observations remain historical.
- Normal native Save As/Quit/relaunch/Open/history/Return/Resume was exercised
  locally, including actual engine responses and capture/drop/promotion archives.
  These are complementary real journeys, not a claim that every corner state was
  reached in one game from the standard opening. Difficult cases use native
  reduced-material roots, never TypeScript rule expectations.
- Unsaved New/Open/native quit/window-close guards retain current work on Cancel
  or failed Save. Capacity rejection precedes commitment and retains complete
  Save/Save As; saving/reopening does not reset structural limits. Unsupported
  restoration remains explicitly rejected by existing native parsing/admission.

No timed play, variations/branch editing, shogi chess-Full operation, strength
claim or additional real variant is added. Raw IDs/profile/native contexts are
collapsed in technical details. Complete 12×12/16×16 display remains the future
product direction; the separate synthetic fixture is not a product variant.

## Normal packaged screenshot set

Open the [visual gallery](gallery.md) for all thirteen current normal-package
views. [Exact capture revisions and hashes](screenshots.json) distinguish visual
evidence from the final functional listener delta.

The `normal-*-macos.jpg` images below are unaltered captures from the
uninstrumented normal bundle: seven at visual source `60796ec` and Sente at final
`a9111ae` (same visual source). Initial Gote is shown after the real
opening engine reply. The capture/promotion diagrams were saved by a separately
labeled native verification package, then opened and operated in the **normal**
package. Their [unaltered canonical archives](records/) retain their provenance;
they do not imply standard-start historical reachability.

| Surface | Normal packaged macOS evidence |
| --- | --- |
| Initial Sente | [Screenshot](normal-sente-macos.jpg) |
| Initial Gote / opening engine reply | [Screenshot](normal-gote-macos.jpg) |
| Both hands and promoted identities | [Screenshot](normal-hands-macos.jpg) |
| Native drop selection | [Screenshot](normal-drop-macos.jpg) |
| Optional promotion | [Screenshot](normal-promotion-macos.jpg) |
| Exact historical event | [Screenshot](normal-history-macos.jpg) |
| Actual active bounded analysis | [Screenshot](normal-analysis-macos.jpg) |
| Native concluded game | [Screenshot](normal-concluded-macos.jpg) |
| Supported minimum 760×650, normal installed Linux | [Screenshot](normal-minimum-linux.png) |
| Normal 1180×820, normal installed Linux | [Screenshot](normal-sente-linux.png) |
| Normal 1180×820, normal installed Windows | [Screenshot](normal-sente-windows.png) |
| Supported minimum 760×650, normal installed Windows | [Screenshot](normal-minimum-windows.png) |

The Linux/Windows screenshots are unaltered captures from the passing
`5de74413f43c610dbf0f8fd6f1d462f82a8952ed` Desktop CI 40408728525 artifacts. Their
visual product source is byte-identical to `60796ec`. Independent visual review
inspected these replacement copies; passing journeys are established separately
by the retained GUI reports and package validation below. The retained [earlier minimum Mac image](stage2-minimum-macos.jpg)
is actual 760×650 earlier-stage evidence, **before** the final contrast refinement;
it is not relabeled as current-source capture. Retained stage1 images show the
first product rendering, including the subsequently corrected header contrast.
Later Mac resize attempts did not reliably resize the native window and are not
counted as new minimum-size evidence.

## Verification and independent review

Local checks: full native workspace `cargo test --locked --workspace --all-targets
--all-features` **396 passed** across 56 groups; root fmt and workspace clippy
all-target/all-feature `-D warnings` passed. Frontend strict check and **123 tests**
passed, including 37 new product/close-routing cases using Rust-produced board fixtures and
presentation-only DTOs for status formatting. These are frontend interaction
checks, not independent rule conformance. Native focused product/context/archive
and actual child-process game/analysis/save/reopen tests passed. Final normal desktop host tests **4 passed**; the separate all-feature host
scope, including verification isolation, **7 passed** with pinned Rust 1.90.

[Implementation review](implementation-review.md) records fresh delegated source
review and independent test execution, including resolved receipt/lifecycle,
modal/focus and cancellation-language findings. The numeric editing correction
was also independently reviewed: blank strings stay local during editing;
HTML work-limit validation precedes numeric serialization, and native admission
is unchanged. The first frontend run exposed the focus-return regression; it was
fixed and the complete suite rerun successfully.

[Visual review](visual-review.md) records independent inspection of actual
screenshots plus calculated contrast. Its material finding was resolved by
raising affected text to 5.30–6.25:1. Static normal-size appearance was independently
reviewed on all three platforms, plus current Windows/Linux minimum sizes;
Windows general text is thinner/somewhat smaller in apparent metrics, and native
number-control rendering differs. No material clipping or missing glyphs was found. This is a bounded visual/accessibility
assessment, not formal screen-reader or WCAG conformance and not owner approval.
Local CUA observed real active analysis; several cancellation attempts completed
before the click or invalidated the transient AX target. Those attempts are not
counted as passing GUI cancellation. Actual native/controller cancellation and
platform GUI evidence are identified separately.

## CI and actual platform interaction

At `5de74413f43c610dbf0f8fd6f1d462f82a8952ed`, [Rust CI 40408728368](https://github.com/jweathers777/mShogi/actions/runs/40408728368)
and [Desktop CI 40408728525](https://github.com/jweathers777/mShogi/actions/runs/40408728525)
passed Windows Server 2022, macOS 15 and Ubuntu 24.04. The desktop job runs strict frontend,
host fmt/clippy/tests, normal package construction and real bundled-engine queries.
Windows/Linux additionally run **14/14 actual GUI phases each**, using installed
normal packages and separately labeled difficult-state verification packages.
macOS CI package/protocol checks are supporting evidence; actual Mac GUI evidence
comes from local CUA operation of the normal packaged application on macOS 27 arm64.
Windows interaction uses the Windows Server 2022 CI desktop with WebView2; Linux
uses Ubuntu 24.04 with Xvfb/Openbox and WebKitGTK. This does not establish physical
display behavior or every consumer OS version.

The external Windows/Linux driver uses W3C pointer/keyboard actions, native
pickers/resize/close/quit, and read-only DOM observations. It does not inject game
IPC or rule behavior. Phases cover normal Chess, normal Shogi, fresh-process
Shogi reopen/history/analysis/Return/Resume, promotion, claims, recovery, synthetic
routes, declarations, terminal same-ply history and both games' capacity pauses.
Normal Shogi explicitly exercises real analysis cancellation/replacement and
Save As/Quit/relaunch/Open. Each phase records host and engine absence **before**
driver teardown; containment cleanup is not counted as successful native Quit.

Retained compact evidence:

- [Combined revision/package validation](platform-validation-5de7441.json).
- [Linux complete GUI report](gui-linux-5de7441.json), [normal package manifest](normal-linux-manifest-5de7441.json), [verification package manifest](verification-linux-manifest-5de7441.json).
- [Windows complete GUI report](gui-windows-5de7441.json), [normal package manifest](normal-windows-manifest-5de7441.json), [verification package manifest](verification-windows-manifest-5de7441.json).

Actual GUI host/engine hashes match the inspected normal package; all normal
manifest hashes and separate QA installer/package hashes were validated. Full
packages, driver logs, DOM observations and additional unaltered screenshots are
in the linked run's `desktop-ubuntu-24.04` and `desktop-windows-2022` artifacts
(retention 14 days). Compact reports and selected screenshots above are durable
repository evidence; package binaries are not committed.

Earlier failed/superseded checks remain historical evidence:

| Source | Run / result | Resolution |
| --- | --- | --- |
|41a3009|Rust40404674273: Ubuntu failed a timing-dependent pending-admission test; Mac/Windows passed|3803c1d explicitly establishes pending receipt before replacement, independently reviewed; unchanged native behavior|
|41a3009|Desktop40404674118: Mac passed, Windows/Linux11/14 GUI phases passed|60796ec fixes real numeric-field editing and semantic verification; missing reopen archive was a dependent failure|
|3803c1d|Rust40405654993 all three passed; Desktop40405654896 cancelled|Desktop superseded, never claimed as passed|
|60796ec|Rust40406696598 all three passed; Desktop40406696600 Mac passed, Windows/Linux11/14 phases passed|5de7441 corrects stale verifier labels for completed analysis and Saved; actual cancellation/replacement/capacity save had already occurred|

Earlier `21bfe3e` local normal package build and package-smoke passed. CUA then
re-exercised normal Chess native Quit/Cancel, New Game→Shogi/Sente, Shogi native
Quit/Cancel and explicit Resume. At `21bfe3e`, native Quit while New Game was open
replaced the uncommitted chooser with one Close dialog; Cancel preserved the
original Chess game, and a subsequent New Shogi game succeeded. Only one guard was present, with Cancel initially
focused; the game remained at event 0. The final app is left ready for owner play.
The [package report](macos-normal-package.json) identifies this actual bundle.
Earlier [Rust CI40412052082](https://github.com/jweathers777/mShogi/actions/runs/40412052082)
at `21bfe3e` passed all three platforms. That desktop attempt failed its Windows reopen phase and is described below;
5de supporting evidence is not used to claim that later attempt passed.
Intermediate b26fd73 Rust40411273387 also passed all three; its desktop
run 40411273334 was cancelled as superseded by `21bfe3e`, not counted as a pass.


## Acceptance coverage and remaining gate

| MSH-74 criterion | Evidence |
| --- | --- |
|1,10: explicit games/sides/real play/font|Normal Mac/Windows/Linux journeys, native product tests, New Game/board frontend tests, font provenance and package checks|
|2: complete choices/outcomes/recovery|Native fixture-backed frontend tests, separately labeled platform hands/promotion/forced/declaration/conclusion/failure phases; normal Mac native archives/drop/promotion/resignation|
|3: exact analysis/execution context|Existing native/process regressions plus product tests and normal platform cancellation/replacement/history analysis|
|4: complete persistence/history/protection|MSH-73 unchanged archive tests and current normal Save/Quit/fresh-process Open/history/Return/Resume; final close-owner tests and actual Mac checks|
|5: actual application/capacity/unsupported modes|14-phase Windows/Linux reports, local normal Mac journeys, unchanged native unsupported-restoration regressions|
|6–9: full-board visual/semantic interaction|Screenshot set, native geometry checks at 1180×820/760×650, independent visual review, keyboard/pointer GUI phases; harness remains separate|
|11: screenshots/review/owner approval|Screenshot set and independent assessment present; **owner visual approval remains pending**|

Implementation is integrated; task remains **In Review**. No MSH-75 work has been
started. After owner approval and any resulting bounded refinements, close 74
only against all its criteria; MSH-75 is the next dependent delivery review.
MSH-41–47 and M4 remain open. There is no material product-design departure;
current-source minimum-window screenshots are Windows/Linux, with the earlier
Mac minimum retained explicitly as historical evidence. Timed play, branching,
formal screen-reader certification, release signing and measured engine strength
remain outside this delivered surface.


## Native cross-game Open correction

Final Windows run 40412052098 at `21bfe3e` passed 13/14 GUI phases; fresh-process
Shogi reopen failed. Native Open reported success, the real saved 9374-byte
archive existed and validated, but the visible view stayed Chess 25 seconds later.
The [failed GUI report](gui-windows-21bfe3e-failed.json), [compact validation](windows-21bfe3e-failed-validation.json),
[actual failed view](windows-21bfe3e-reopen-failure.json) and [unaltered saved archive](records/regression-shogi-reopen-21bfe3e.mshogi)
preserve this failure. Only the other 13 phases establish successful native
Quit; cleanup of the failed phase is not counted as quit proof. Linux passed
its delayed dependency step, but was cancelled as superseded before final GUI
verification; Mac package CI passed. The run is not presented as all-platform
success.

Both primary and independent platform reviewers identified the native race:
shared_snapshot could enter Chess, commit Shogi Open inside the legacy poll,
and publish Chess geometry with the newly advanced Shogi token. The next correct
Shogi snapshot had an equal token and was correctly rejected by strict frontend
freshness. a9111ae rechecks active game after both native read paths poll files;
the legacy path also returns before starting retired Chess execution. The real
file-worker regression failed on old code and now passes for shared-first and
legacy-first polling, stopped restoration, consistent game/token projection,
and absence of hidden Chess requests. Independent implementation review and
adjacent archive regression passed; full native suite 396 passed. No frontend
freshness check was relaxed. Final [Desktop CI40414800679](https://github.com/jweathers777/mShogi/actions/runs/40414800679)
and [Rust CI40414800449](https://github.com/jweathers777/mShogi/actions/runs/40414800449)
at **a9111ae83de1d91149367a136da4ed66efdefee5 passed all three systems**.
[Exact six-job CI record](ci-a9111ae.json). Final Windows and Linux each passed
**14/14 actual packaged GUI phases**, including the formerly failing fresh-process
Chess→Shogi reopen and native quit. Independent [artifact validation](platform-validation-a9111ae.json)
confirmed every phase's native host/engine exit before teardown, matching normal
GUI/package executable hashes, normal and QA manifests, and 187 retained Linux /
164 Windows file hashes. macOS normal package manifest hashes were also validated.

Durable final reports: [Linux](gui-linux-a9111ae.json), [Windows](gui-windows-a9111ae.json),
[normal Linux manifest](normal-linux-manifest-a9111ae.json), [normal Windows manifest](normal-windows-manifest-a9111ae.json),
[QA Linux manifest](verification-linux-manifest-a9111ae.json), [QA Windows manifest](verification-windows-manifest-a9111ae.json).
Full final packages/logs/additional captures are in the linked final run's platform
artifacts, retained 14 days. Earlier source-specific screenshots/reports remain
explicitly labeled; they are not relabeled as final-source execution.

The final a9111ae normal macOS package was rebuilt and passed package-smoke,
including normal instrumentation exclusion. Local actual GUI reopened the exact
Windows regression archive in a fresh Chess process: it switched to Shogi,
showed two events/two historical observations and stopped work, selected event 1,
returned explicitly to live and resumed. The original archive was not modified;
verification-only runtime changes were discarded through the native guard.
[Corrected reopen screenshot](normal-reopened-macos.jpg). A fresh Sente game is
left open for owner play. The current package report identifies a9111ae.


Independent [final canonical-record validation](canonical-record-validation-a9111ae.json)
checked both platform saves: the complete archive retains 2 events/2 observations;
historical analysis contains the exact 1-event prefix while the live archive
retains 2 events, with Gote perspective. Saved execution floor 11 is retired:
fresh request/process generation 13 and session version 14 establish new runtime
work. Capacity saves retain 256 observations / 579575 bytes. These concrete
observations supplement the broader native archive/history/capacity regressions;
they do not substitute for those tests or make historical observations pending.

## Owner common-shell refinement (2026-10-27)

The subsequent [common dark application shell evidence](common-shell-2026-10-27/README.md)
records the owner's requirement for one theme and one composition across Chess
and Shogi. It supersedes the earlier light Shogi surroundings while preserving
the native inspection and game-specific board/piece work. The intermediate paired
screenshot review caught a remaining Chess help-row migration; the corrected
source moves live help/inspection into the shared footer. Each capture and CI
result retains its own revision and platform scope. MSH-74 remains In Review for
final owner visual approval; MSH-75 is unstarted.
