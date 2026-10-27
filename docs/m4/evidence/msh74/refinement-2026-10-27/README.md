# MSH-74 owner refinement — shared shell and movement inspection

Product source: `d6d76816932763182eee574c9c241daa6ef9a91a`, integrated and pushed
to `codex/rewrite`. MSH-74 remains **In Review** for owner visual approval; no MSH-75
work is authorized or started. M2/M3 and prior completed task evidence are preserved.

The two games now render the same `ApplicationShell`: global identity/runtime,
document title and dirty state, New/Save/Save as/Open, live heading and Flip,
board/player area, right sidebar ordered Analysis → Game history → Game actions,
and collapsed technical access. Chess keeps its dark/checkerboard character;
Shogi keeps the light/warm grid, licensed kanji pieces, owner rotation and counted
hands. Native game adapters and complete action/promotion semantics remain authoritative.

Movement inspection calls the normal bounded native `game_inspection` query with
an exact document/version/revision/selection token and opaque origin. The immutable
controller uses the selected native history prefix and a private hypothetical
owner-to-move position. Chess en passant remains available only to its genuinely
entitled side. These are local legal movement choices, not engine prediction,
history adjudication or authorization to play the other side. Complete action routes
and completion alternatives remain represented; no TypeScript movement rules were
added. Opponent hands use native drop legality, including nifu, dead ranks and
pawn-drop mate. Empty resources reject.

Inspection never polls, starts, cancels, accepts or retires engine work, and cannot
commit a move. Frontend context/origin/request retirement prevents late queries
from painting a different position. Hollow burgundy rings, dashed inspected origins
and explicit “Inspecting … / Read-only” wording differ from actionable friendly
markers in shape as well as color. Capture rings surround occupied squares.

## Actual normal packaged macOS evidence

Normal uninstrumented Tauri bundle, macOS 27 arm64, 1180×820 native window
(2360×1640 Retina captures), external CUA pointer/keyboard and native file dialogs.
The [package report](macos-normal-package.json) retains executable hashes, inventory,
normal-package instrumentation exclusion and actual bundled-engine observations.
The nine images below are unaltered captures of the exact product source above.

| Surface | Capture |
| --- | --- |
| Chess shared shell | [Image](chess-shared-shell.png) |
| Chess friendly actionable selection | [Image](chess-friendly-selection.png) |
| Chess opponent knight inspection | [Image](chess-opponent-inspection.png) |
| Shogi shared shell / full initial board | [Image](shogi-shared-shell.png) |
| Shogi friendly actionable selection | [Image](shogi-friendly-selection.png) |
| Shogi opponent silver inspection | [Image](shogi-opponent-inspection.png) |
| Opponent pawn in hand / native drops | [Image](shogi-opponent-hand-inspection.png) |
| Exact historical prefix / occupied destination | [Image](shogi-historical-inspection.png) |
| Opponent hand inspection after Flip | [Image](shogi-opponent-hand-flipped.png) |

Observed in the running normal package: friendly→enemy switching, enemy-route
completion without a played move, Escape, hand selection by Enter, Flip, native
Open, historical event 1 of a two-event game (unpromoted Gote rook at 4e and no
future Gote hand pawn), Return to live (promoted rook at 4g and Gote pawn in hand),
and saved/unsaved/native quit confirmation. Querying and completing an inspection
route left the saved live two-event game unchanged. Native controller tests provide
full-state and pending-request equality evidence; GUI screenshots alone do not.

Hand/history cases open the pre-existing [native capture archive](../records/msh74-native-captures.mshogi)
through normal Open. Its reduced-material root provenance is explicit; these views
do not claim standard-opening historical reachability. The original archive remains
unchanged. A normal Save As preserved the final historical/view/orientation settings
in [a separate complete archive](../records/msh74-refinement-inspection-flipped-final.mshogi).
A temporary historical-view session was also preserved under `/private/tmp` before
quit after automatic approval review rejected discarding unsaved state. No user
session was discarded. Native resize attempts did not reliably resize this window;
there is no new Mac minimum-size claim. Current minimum-size/platform evidence is
recorded separately when obtained from the actual Windows/Linux GUI runs.

Local verification found and corrected an extra inspection-message row that pushed
the Chess footer outside the window. The final message reuses the existing board
help area. Shared-main padding specificity and light-theme saved/paused contrast
were also corrected and recaptured. Earlier source-specific evidence remains in
the parent directory, with its original revision labels.

## Verification and review

- Root stable Rust 1.89: `cargo test --locked --workspace --all-targets --all-features`
  **408 passed / 56 groups**; root fmt and all-feature workspace clippy `-D warnings`
  passed. Tests cover immutable queries/pending requests/exact historical selection,
  native movement/promotion/drop expectations and existing rules/process/archive regressions.
- Node 24.19.0: strict TypeScript and frontend **147 passed / 21 files**. New tests
  exercise native DTO interaction only; they do not independently establish rules.
- Desktop Rust 1.90: all-feature host **8 passed**, including strict bounded/duplicate
  query parsing and read-only dispatch; host clippy and fmt passed. Existing vendored
  Tao warnings/provenance are unchanged.
- Normal `npm run desktop:build -- --bundles app` and `package-smoke.py` passed.
  Normal distributables exclude test constructors, verification IPC and fixture hooks.
- [Independent source/test review](implementation-review.md) and separate actual
  screenshot/contrast review found no material remaining implementation or normal-size
  visual issue. GUI-driver assertions were independently reviewed too. These reviews
  are distinct from owner approval and formal screen-reader certification.

## Final CI and actual platform evidence

At exact product revision `d6d76816932763182eee574c9c241daa6ef9a91a`:

- [Rust CI40472402479](https://github.com/jweathers777/mShogi/actions/runs/40472402479)
  passed Windows2022, macOS15 and Ubuntu24.04; [original job results](rust-ci.json).
- [Desktop CI40472402431](https://github.com/jweathers777/mShogi/actions/runs/40472402431)
  passed all three systems; [original job results](desktop-ci.json). macOS CI proves
  package/native checks; actual Mac GUI evidence is the local normal package above.
- Windows2022/WebView2 and Ubuntu24.04/Xvfb/Openbox/WebKitGTK each passed **14/14
  actual GUI phases**, including the added shared-shell/inspection assertions and
  existing play, cancellation, recovery, Save/Open, history, capacity and native
  close/quit journeys. Normal applications and separately labeled difficult-state
  verification packages have distinct recorded identities.
- [Independent artifact audit](platform-audit.md) verifies source, package/binary
  bindings, file hashes, canonical inspection invariance and native shutdown before
  driver teardown. [Original Linux report](linux/gui-report.json) and
  [Windows report](windows/gui-report.json), manifests, package reports and selected
  canonical archives are retained without rewriting their source observations.
- [Independent final visual review](visual-review.md) found no outstanding material
  finding in reviewed Mac/Windows/Linux states. Current normal minimum captures:
  [Linux760×650](linux/normal-shogi-minimum-size.webview.png),
  [Windows760×650](windows/normal-shogi-minimum-size.webview.png).
  Windows CI has a 1024×768 virtual desktop: normal-size native desktop images
  are cropped, paired WebDriver client images show the full actual viewport,
  and the minimum-size native capture is complete. See the audit for provenance.
  These ordinary minimum states and normal-size active inspection were captured
  separately; no combined minimum/inspection or new Mac minimum claim is made.

The `retained-files.json` in each platform directory maps copied files to original
artifact paths and exact SHA-256. Full original build artifacts are run-linked
(retention14 days); large installers/binaries are not committed. Prior M1/M2/M3,
MSH-61/73 persistence, native profile and licensed-font evidence remains unchanged.
No rules, archive schema or engine result-acceptance contract changed in this refinement.

## Launch

```sh
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app"
```

From a checkout with the documented Node/Rust toolchains: `cd desktop && npm ci &&
npm run desktop:build -- --bundles app`. Choose New game → Shogi → Sente/Gote/Random.
Existing untimed/bounded-search, unsigned development-package and no-branch-editing
limitations remain. Full 12×12/16×16 product fit remains future variant work; this
change does not turn the development fixture into a supported real game.
