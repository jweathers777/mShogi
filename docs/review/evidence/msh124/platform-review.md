# MSH-124 independent Linux / Windows packaged visual review

Reviewer `/root/shell124_review`, 2026-10-31. Personally viewed all28 retained platform PNGs using view_image; read gui-report.json, manifests/package-smokes and platform-audit.json. Read-only source/evidence review; no product/Backlog edits. These are actual installed/packaged GUI artifacts driven by the CI W3C/native driver, not mockups or browser-only render tests. Reviewer did not personally operate the remote applications or rerun their15-phase journeys.

## Exact provenance and independent checks

Desktop workflow41207699334, source `4a624d6c4f129ce774d385192fbbe6a9bdcf547a`. Both actual gui-report.json records identify that source, status passed, all15 phases passed, and the native-input/read-only-DOM method. Platform/package identities:

- Linux7.0.0-1017-azure x86_64 glibc2.39, Python3.12.3, runner20261026.354.1; X11/WebKit compositing disabled per recorded renderer environment. Installed normal `/usr/bin/mshogi-desktop`, SHA256 `ad8b5a86405d1a506bafaba0fbf54068bc6447a860c0a1289d214f18e362aaf0`; engine SHA256 `325157b4d14f21cc39b2f1663ef7eff60f3c79d7b4981a9cefb30c36052ff559`.
- WindowsServer2022 build20348, Python3.12.10, runner20261026.347.1. Packaged normal `D:\a\_temp\mshogi-package-smoke\mshogi-desktop.exe`, SHA256 `91c5ec1c75e2fb6973e112b647034b4cee94e9a2e365e0e9de169f5061768c8d`; engine SHA256 `e8574faf581980869af33cd84b9268eb13dfd01e4697904f398113c59d272cce`.

Root's artifact audit records435 Linux and396 Windows report-file hashes verified and13/2 normal manifest entries verified respectively, no missing manifest files. This reviewer **independently verified all28 retained PNG hashes** against the exact original gui-report files map. The broader435/396-file audit remains attributed to root, not misrepresented as a second complete artifact extraction.

Independently recomputed all12 actual keyboard focus probes using the current pure focus_outline_visible predicate: six per platform, all pass; actual3px outward outline+3px offset is inside every effective clipping ancestor and viewport. Read six initial layouts per platform: Chess/Shogi/Xiangqi card rectangles match exactly within each normal/minimum category, with shared Analysis218px normal and232px minimum. Linux minimum cards left506/width236 and Windows left506/width221 reflect real platform scrollbar differences; each platform's three games match. Thus **F-V02 is resolved by actual platform comparison**, not by weakening comparisons or removing native assessment text. All report-supported initial board counts64/81/90 are visible, and ordinary control/header/theme/sidebar facts are common.

Read support records (six/platform) and chooser records (24/platform). They preserve exact2 disclosure labels, native profile/context tokens, supported-only option identities and native side/radio meanings; screen captures corroborate their actual visible states. Native font licenses, diagnostics and Quit/close journeys retain separate report evidence. Normal inventories/package-smokes and separate verification manifests retain instrumentation exclusion; pixels alone do not establish binary feature exclusion.

## Actual files personally viewed

All14 filenames below were personally viewed in **both** `docs/review/evidence/msh124/linux/` and `docs/review/evidence/msh124/windows/` (28 total):

1. normal-initial-chess-normal-size.webview.png
2. normal-initial-chess-minimum-size.webview.png
3. normal-shogi-initial-sente.webview.png
4. normal-shogi-minimum-size.webview.png
5. normal-xiangqi-initial-red-normal-size.webview.png
6. normal-xiangqi-initial-red-minimum-size.webview.png
7. normal-moves-chess-1180-keyboard-history.webview.png
8. normal-moves-chess-760-keyboard-history.webview.png
9. normal-shogi-moves-shogi-1180-keyboard-history.webview.png
10. normal-shogi-moves-shogi-760-keyboard-history.webview.png
11. normal-xiangqi-moves-xiangqi-1180-keyboard-history.webview.png
12. normal-xiangqi-moves-xiangqi-760-keyboard-history.webview.png
13. normal-xiangqi-support-xiangqi-760x650.webview.png
14. normal-shogi-chooser-shogi-760.webview.png

## Visual / accessibility findings

**PASS for common composition, complete native boards, current Moves/cues and focus repair.** All3 games share header/document controls/theme, player/board/footer spatial grammar and Analysis → Game Actions → Moves order. Native Chess pieces/squares, Shogi warm9x9/kanji/rotation/hands and Xiangqi intersections/palaces/river/Red-Black pieces survive. Whole boards remain visible normal/minimum, including historical mode. Platform font metrics differ; game-specific names and notation remain readable. Sidebar overflow permits controls/history access without scrolling the board out of view.

All12 focused historical frames visibly show the entire rounded gold ring, including lower edge, around e4 /７６歩/P9+1; accompanying exact-prefix/geometry probes agree. This corrects the123 lower-edge finding rather than merely keeping labels visible. Native origin/destination cues and separate history read-only status remain distinct from focused move buttons; unknown assessment stays player-visible. Initial same-category common cards align across all3 games. About minimum captures wrap long notice/source text, preserve disclosure focus and license text/link; chooser minimum captures fit, with recognizable native select and unchanged native side buttons. Remote report supplies actual keyboard operation; this screenshot review does not infer it solely from gold pixels.

**F-V03, new actual Linux visual finding:** in the4a Linux captures, Unicodeⓘ is a missing-glyph/tofu rectangle next to mShogi. Windows renders the intended circled-i. Accessible name/title and keyboard route still function, but a visible empty box does not reliably convey support/About. Reviewer reported it rather than accepting GUI green as visual success; root independently confirmed and retained original images. No other material finding in these28 images.

## Bounded F-V03 source correction review

Independently reviewed committed `91eb13e2451af77150b82b62a448bf97e9d5ac58` versus4a: support button child only changes from Unicodeⓘ to18px inlineSVG circle+stem+dot;23px button retains exact aria-label/title/disabled/callback/placement. SVG is aria-hidden and focusable=false, so no new Tab target or duplicate speech. CSS only centers child in the existing button. Three cross-game test assertions require vector child/shape and no font-dependent text.

**Source correction PASS.** This is appropriate code-native rendering, not a rules/lifecycle/shared-framework change. Independently executed5 SupportDialog tests PASS (`/private/tmp/msh124-independent-svg-support.log`) plus strict TypeScript PASS (`/private/tmp/msh124-independent-svg-typescript.log`). Current4a full semantic/platform journey evidence remains valid for unchanged paths; source-only review does not prove Linux pixels are corrected.

**Pending before final all-platform visual acceptance:** actual identified Linux91-or-later package capture showing recognizable info icon with preserved header/focus and normal build exclusion evidence. Root plans fresh final-source CI15phases; preserve original4a images and F-V03 failure. Native runner attempt1/retry disposition remains separate `/private/tmp/msh124-review-native-ci.md`; no native correctness or race fix is inferred from desktop green. Overall50 personally viewed screenshots so far:22Mac+28remote.

Mac vector follow-up: personally viewed91 source normal1180 and labeledQA760 icon captures; both show recognizable centered circled-i and unchanged header/board. Separate exact vector package identities are recorded in `/private/tmp/msh124-review-visual.md`. Current count52 personally viewed images (24Mac+28remote). F-V03 Linux recapture remains pending; no inference of Linux success from Mac pixels.

## F-V03 closed — actual final-source Linux evidence

Final source `91eb13e2451af77150b82b62a448bf97e9d5ac58`, Desktop41208986515, actual Linux artifact13045906546. Read `vector-platform-audit.json` and `vector-linux/gui-report.json`. Actual report has15/15 phases passed, Linux7.0.0-1017-azure/Python3.12.3, same recordedX11/compositing configuration. Installed normal host SHA256 `0a82442eb854ba82e7ce6902a73656b745cdb79cf39256bd68c0c5072b6bacc1`; bundled engine remains325157b4d14f21cc39b2f1663ef7eff60f3c79d7b4981a9cefb30c36052ff559. Root's new435reportfile/13normalmanifest hash audit is separate from this reviewer's independently repeated14 retained-PNG hash checks. Independently recomputed six whole-outline focus checks and exact all3 initial card-rectangle equality at1180/760: PASS.

Personally viewed these8 actual final-source `vector-linux/` images:

- normal-initial-chess-normal-size.webview.png
- normal-initial-chess-minimum-size.webview.png
- normal-shogi-initial-sente.webview.png
- normal-shogi-minimum-size.webview.png
- normal-xiangqi-initial-red-normal-size.webview.png
- normal-xiangqi-initial-red-minimum-size.webview.png
- normal-xiangqi-support-xiangqi-760x650.webview.png
- normal-shogi-chooser-shogi-760.webview.png

The actual SVG is plainly a circled information `i` in all6 initial header captures, including minimum size; no missing-font rectangle remains. Root did not overwrite historical4a screenshots. Same header/name/brand/controls and complete native boards remain intact. The normal/minimum Analysis → Game Actions → Moves cards align; native Unknown remains visible. About and chooser actual minimum images retain wrapped license text, disclosure focus and the supported-only compact select/native side-button appearance.

**F-V03 resolved; final Linux source/visual/accessibility verdict PASS, no unresolved material Linux visual finding.** Same-source current Windows GUI is still running as reported by root; prior4a Windows semantic/focus/layout evidence remains valid for unchanged paths, but this reviewer has not claimed final91 Windows execution yet. Current total60 personally viewed screenshots:24Mac+28priorremote+8finalLinux. Overall independent source review remains PASS; task finalization/CI retry outcomes are root's separate auditable responsibility.

## Final Windows and overall bounded verdict

Final source91eb13e2451af77150b82b62a448bf97e9d5ac58, Desktop41208986515, actual Windows artifact13045872473. Personally viewed **all14** `vector-windows/` PNGs (same14 names explicitly enumerated above for original platform review). These are additional actual final-source views, not a claim of14 new personal GUI executions. Recognizable centered SVG info icon, identical shared header/controls/theme, full native boards at1180/760, native hand/river/palace content, Analysis → Game Actions → Moves order and complete focused e4/７６歩/P9+1 outlines all PASS. About license wrapping/focus and supported-only chooser/native side buttons remain intact. No new material finding.

Independently verified all14 retained final Windows PNG hashes against its exact gui-report files map, recomputed all6 final whole-outline bounds, and compared native initial card rectangles for exact3game equality atnormal/minimum: PASS. Actual report source91/status passed/all15phases passed; WindowsServer2022build20348/Python3.12.10, runner20261026.347.1. Normal host SHA256 `11cfef8e6749a6b21bb1058c9b8f1e7409523e0cde30e40d4e6d9d74ef665e50`; engine SHA256 `02c658043263cea84fd776bfdf130d8bbd7daeaf8ea8038abc4667154df8c66c`. Root's complete396reportfile/2manifestfile audit is distinctly attributed; this reviewer repeated retained-PNG/focus/layout checks rather than re-extracting the full archive.

Read final persisted CI metadata `native-final-ci.json` and `desktop-final-ci.json`: both identify exact91source, attempt1 success, each with all3 platform jobs completed success (native41208986510, desktop41208986515). Native current-source green neither fixes nor erases earlier timing/classification race failures. Remote GUI reports substantiate actual Windows/Linux interaction; macOS CI package/engine success remains distinct from root's local native Mac interaction and independently reviewed Mac screenshots.

**Final independent source/visual/accessibility verdict: PASS for MSH124 at91eb13e2451af77150b82b62a448bf97e9d5ac58; no unresolved material MSH124 finding.** F-V01 wrapping, F-V02 common minimum card heights, F-V03 Linux font-dependent About icon, and the123 focus-edge handoff are resolved by actual identified package evidence. Native race limitations remain out of scope and documented, not accepted as fixes. Review method: bounded source diff, independently executed focused tests/TS/driver unit tests, actual artifact/report/hash/focus/layout inspection, and **74 personally viewed screenshots** (24Mac,28priorLinux/Windows,8finalLinux,14finalWindows). Root drove Mac GUI; CI native drivers drove remote GUI. No rules conformance, engine strength, MSH125 lifecycle completion, R6/M4 closure or personally operated remote GUI is inferred.

Proportionate handoff-document review: read `docs/review/shell-124.md` acceptance matrix/scope/commands/findings/CI/launch/downstream ownership. Its six-criterion mapping matches the reviewed source/evidence; meaningful initial failures/provenance and separate source/package/platform/native distinctions are preserved. Final-source Windows/CI text was being finalized while read; root should retain the exact91source and report results above. No substantive evidence or scope conflict found. Marking task Done still requires root's supported Backlog finalization/integration/push; MSH125 remains unstarted, MSH121 separately owns integrated acceptance, M4 open.
