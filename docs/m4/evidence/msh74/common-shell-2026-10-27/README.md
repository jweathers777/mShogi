# MSH-74 common dark application shell

Product source: `19beb7437f11003734d65ee68f69a5f34bbdd02c`, integrated and pushed
to `codex/rewrite`. MSH-74 remains **In Review**, with owner visual approval pending.
MSH-75 is unstarted. This refinement supersedes the earlier light Shogi workspace;
it does not invalidate the unchanged native inspection/rules/archive evidence.

## Application composition

Both games render the same `ApplicationShell`, `GameArea`, `PlayerArea` and
`InspectorSwitch`. One dark palette controls the surrounding application, header,
workspace, cards, typography, buttons, inputs and disclosures. The same four physical
game-area slots contain the upper player/hand, board, lower player/hand and help.
Chess retains its board and pieces; Shogi retains its warm board, bundled kanji,
pentagonal owner-oriented pieces, promotion identity and counted hands.

New game, Save, Save as, Open, document title and unsaved status occupy the common
header. Live status and Flip occupy the common heading. The right sidebar is
Analysis → Game history → Game actions → technical details for both games.
Historical selection notices use one shared HistoricalNotice with the same right-aligned
Return to live control. Both games use one AnalysisActions row with Start/Replace
and Cancel together inside Analysis. Historical selection occupies one position-notice slot. Live board help
and inspection labels occupy the shared footer below the lower player. Its common
48-pixel allocation preserves wrapped help and the Chess native position footnote. Initial
analysis defaults to Play details; the common Analyze position switch exposes each
game's native operations without inventing telemetry or sharing Chess-only Full
operations with Shogi. Bounded history cards scroll their contents, preserving the
location of following controls.

No native rules, game-session controller, inspection query, engine acceptance or
archive schema changed. Opponent-piece/hand inspection remains an immutable native
query bound to the exact selected context; hollow inspection markers and explicit
read-only labels remain distinct from actionable friendly choices. Native event
history, stale-response retirement, promotion, keyboard and hand-selection handlers
are preserved.

## Verification status

- Node 24.19.0: strict TypeScript and **149 frontend tests / 21 files passed**.
  Shared slot/orientation/hand keyboard tests exercise the real board components;
  they do not substitute for rules conformance or actual packaged GUI evidence.
- [Independent source/test review](implementation-review.md) examined the bounded
  diff and independently ran focused tests. Source findings were fixed: avatar base
  geometry lost with obsolete Shogi chrome, and a legacy Chess resign margin that
  would move the actions card. Independent actual paired screenshot review then
  caught Chess help above the lower player while Shogi help was below it. The
  optional Board-owned footer portal fixes that ordering without changing native
  selection or inspection; a focused regression covers focus, Escape and cleanup.
  Expanded review then caught Cancel-analysis and Return-to-live placement;
  shared action composition resolves both. A final CSS specificity correction
  ensures the historical action has the same colors in both games.
- [Independent actual screenshot review](visual-review.md) compares paired
  normal/minimum/active-inspection and analysis/history states on Windows/Linux,
  plus current normal packaged Mac states. No outstanding material visual finding.
  This is separate from owner approval and formal accessibility certification.
- The normal macOS arm64 application builds with the desktop Rust 1.90 toolchain.
  [Package report](macos-package.json) retains host/engine hashes, actual bundled
  engine requests and normal-package instrumentation exclusion. This establishes
  package/protocol checks. [Actual normal Mac GUI evidence](macos/README.md) separately
  covers game switching, native play, read-only board/hand/keyboard/history/Flip,
  bounded historical analysis, Save/Open stopped and native saved-game Quit.
- Unchanged native regression evidence at
  [d6d7681](../refinement-2026-10-27/README.md) remains applicable: 408 native tests
  across 56 groups, 8 all-feature host tests, rules/history/archive/inspection
  invariants and earlier packaged journeys. It does not establish the new shell's
  visual appearance.
- Final-source [Rust CI40493284840](https://github.com/jweathers777/mShogi/actions/runs/40493284840)
  passed Windows/macOS/Linux; [original results](rust-ci.json).
- [Desktop CI40493284756](https://github.com/jweathers777/mShogi/actions/runs/40493284756)
  is successful: Windows and Mac passed on attempt1; the unchanged Linux-only
  attempt2 passed after a read-only WebDriver text-request reset in its claim
  fixture. [Original final results](desktop-ci.json),
  [first attempt](desktop-ci-19-attempt1.json). The first Linux13/14 failure remains
  [separately retained](linux-19-attempt1/retained-files.json); it is not relabeled.
- Windows and Linux each passed **14/14 actual installed/packaged GUI phases**.
  [Independent artifact audit](platform-audit.md) verifies exact package/executable
  bindings, all retained hashes, sixteen equal complete inspection-state comparisons,
  nine settled saves and fourteen native shutdowns before driver teardown per
  platform. Shogi before/after saves are byte-identical; source canonical content
  matches (Windows checkout CRLF differs from saved LF; Linux source bytes also
  match). Historical analysis retains its one-event prefix while the live save retains two events. Normal
  packages exclude verification instrumentation. See the original
  [Linux report](linux-final/gui-report.json) and [Windows report](windows-final/gui-report.json).
- Intermediate `a2524c10` passed all three systems in
  [Rust40487524835](https://github.com/jweathers777/mShogi/actions/runs/40487524835)
  and [Desktop40487524795](https://github.com/jweathers777/mShogi/actions/runs/40487524795).
  [Original results](desktop-ci-a252.json), [audit](platform-audit.md) and
  [retained originals](linux/retained-files.json) preserve that provenance.
  Paired screenshot review caught the Chess help ordering despite those passes.
- Intermediate `44aeb47` passed all native Rust jobs and the Mac package job.
  [Desktop40489563132 attempt1](desktop-ci-attempt1.json) **failed**: Linux12/14,
  Windows13/14. [Linux originals](linux-final-attempt1/retained-files.json) and
  [Windows originals](windows-final-attempt1/retained-files.json) preserve exact
  reports, failure evidence and valid layout captures. Independent visual review
  confirmed the footer fix, then expanded active-analysis/history comparisons
  found Cancel analysis and Return to live in different locations. Final source
  supplies actual shared action composition, retaining native command semantics.
- The Linux failure was an upstream reset during a read-only WebDriver element
  lookup; its missing save caused the later dependent Open failure. The Windows
  inspection-state assertion did not log the exact changed field. A Save As
  baseline could be sampled while an old Saved document was still displayed;
  that is a plausible risk, **not a proven explanation** of the old mismatch.
  Final external-driver code requires the exact accepted native path and settled
  controls before the unchanged invariant, logs complete before/after states,
  and permits one logged transport retry only for read-only element lookups.
  Clicks, keys, scripts, session creation/deletion and other commands never retry.
  All22 external-driver unit cases pass; they do not turn old GUI failures into
  successes. Final captures additionally assert common Cancel-row ancestry and
  right-aligned Return placement.

The Mac was initially locked and was later unlocked by the owner. The rebuilt
normal package was then freshly launched and exercised. The original open session
and generated test sessions were saved separately before replacement/quit; no
user game or source fixture was discarded. Current Mac screenshots and records
are linked below. No current Mac minimum-size or successful rapid-Cancel claim is
made; actual current Windows/Linux minimum and cancellation journeys supply that
separate evidence. Owner approval remains outstanding.

The intervening evidence-only revision `0c07e64` had a failed Desktop run40475158549:
two Linux external GTK fallback activations raced a picker that had already closed;
a separate reopen phase encountered an upstream driver connection reset. Saved
files/screenshots and a live host do not turn these phases into passing checks.
The earlier exact-source `d6d7681` successful runs retain their original provenance.

## Paired final application captures

All images below are actual normal applications at source `19beb74`, with native
game content. Mac pairs use the same1180×820 native window, including its title
bar (2360×1640 Retina pixels). Linux/Windows pairs measure client dimensions.

| State and size | Chess | Shogi |
| --- | --- | --- |
| Mac normal1180×820 native | [Chess](macos/chess-normal.jpg) | [Shogi](macos/shogi-normal.jpg) |
| Linux normal1180×820 client | [Chess](linux-final/normal-initial-chess-normal-size.webview.png) | [Shogi](linux-final/normal-shogi-initial-sente.webview.png) |
| Linux minimum760×650 client | [Chess](linux-final/normal-initial-chess-minimum-size.webview.png) | [Shogi](linux-final/normal-shogi-minimum-size.webview.png) |
| Windows minimum760×650 client | [Chess](windows-final/normal-initial-chess-minimum-size.webview.png) | [Shogi](windows-final/normal-shogi-minimum-size.webview.png) |
| Linux active minimum inspection | [Chess](linux-final/normal-inspect-chess-e7.webview.png) | [Shogi](linux-final/normal-shogi-inspect-shogi-7c.webview.png) |
| Windows actual active analysis | [Chess](windows-final/normal-active-analysis.webview.png) | [Shogi](windows-final/normal-shogi-active-analysis.webview.png) |
| Windows historical analysis / common Return | [Chess](windows-final/normal-historical-analysis.webview.png) | [Shogi](windows-final/normal-shogi-reopened-historical-analysis.webview.png) |

At minimum size the entire board/player/footer region fits; the independent
sidebar can scroll to later controls. Windows CI's virtual desktop is1024×768:
large native desktop captures are cropped, full WebDriver client captures are
complete, and the minimum native window is complete. Native `.desktop.png`
companions and exact original paths/hashes are indexed in
[Linux](linux-final/retained-files.json), [Windows](windows-final/retained-files.json)
and [Mac](macos/retained-files.json). Full original run artifacts have14-day
retention; installers/build outputs are not committed. Older intermediate captures
retain their own revision labels and are not final visual approval evidence.

## Launch

The locally built normal application contains the product source above:

```sh
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app"
```

If an older process is already running, save its game and quit it before launching
the rebuilt bundle. Choose New game to switch Chess/Shogi. Both remain untimed
local games against the bundled native engine, with bounded analysis, complete
canonical saves and historical viewing. This is an unsigned development package;
no strength, release-signing or formal screen-reader certification is claimed.
