# Independent final platform visual review — MSH-123

Reviewer `/root/moves_review`; screenshot-level comparison of actual packaged GUI artifacts. No product, Backlog or task-status edits. The reviewer copied only the12 unmodified PNGs listed below into repository evidence; complete capture hashes/source paths are recorded in `/private/tmp/msh123-platform-capture-manifest.json`.

## Provenance

Production source `fc7a6ae9e7499c40999c5a0396bc31e87d4e07e4`; tested verification-follow-up source `65975d8ea4f232883ceeb559be9b3cdb677c6e47`. Read the intervening Git diff: only external driver readiness, tests, task/evidence changed; no production UI/native source changed. The author’s R5/native-selection/cue boundary and prior30 macOS screenshot PASS remain applicable.

Desktop workflow [41200359930](https://github.com/jweathers777/mShogi/actions/runs/41200359930) artifacts identify that exact659 revision. Both platform gui-report.json records report15/15 actual GUI phases passed, with W3C pointer/keyboard, native dialogs/close and read-only DOM observations. This review read those reports and the selected screenshot/DOM files; it is not a claim that this reviewer personally drove the platform applications or independently reran the full campaign. Root separately audited the CI runs and driver-readiness correction.

- Linux: `Linux-7.0.0-1017-azure-x86_64-with-glibc2.39`, Python3.12.3, runner20261026.354.1; X11 and `WEBKIT_DISABLE_COMPOSITING_MODE=1`. Normal installed host `/usr/bin/mshogi-desktop`, SHA256 `4c62df6229b8aa114a0bfd3e67147b0998bc7857681c43f03d4c0562a4545560`; engine `/usr/bin/mshogi`, SHA256 `325157b4d14f21cc39b2f1663ef7eff60f3c79d7b4981a9cefb30c36052ff559`.
- Windows: WindowsServer2022 build20348, Python3.12.10, runner20261026.347.1; packaged normal host `D:/a/_temp/mshogi-package-smoke/mshogi-desktop.exe`, SHA256 `e15c6a2929845c999549379111852a01a3e8cda15b80533f032076dddbea357c`; engine SHA256 `07c81d7a1eeb324ba81456a91fa541b5f00bb2bb9c35a7b429d149e433326ab7`. Actual WebView2 version131.0.2903.86 is recorded by native attachment.

## Personally reviewed and retained captures

### Linux

- `docs/review/evidence/msh123/linux/normal-moves-chess-1180-pointer-history.webview.png` — SHA256 `707001fe6a86173f1c6afb8ad5d93178b0c6f02b548b41990b9138e762631b9a`
- `docs/review/evidence/msh123/linux/normal-moves-chess-760-keyboard-history.webview.png` — SHA256 `bb1ecf9ec5b2c509fbb7b0733a5d40f91a417983d670af91eaf5c1fc088ae1ab`
- `docs/review/evidence/msh123/linux/normal-shogi-moves-shogi-1180-pointer-history.webview.png` — SHA256 `8fb21adbdcc2783c874d3753a4f78617ded0fdd5a28d8b052ffc64caa2a5ea84`
- `docs/review/evidence/msh123/linux/normal-shogi-moves-shogi-760-keyboard-history.webview.png` — SHA256 `c849ff5905edeb57fdc9fb3723814b676914cc3bb90a00113238ff58f7e7641a`
- `docs/review/evidence/msh123/linux/normal-xiangqi-moves-xiangqi-1180-pointer-history.webview.png` — SHA256 `c8096837c51da7c723f58b350d4aebb29de69074cca7be8831d9270e3de561e5`
- `docs/review/evidence/msh123/linux/normal-xiangqi-moves-xiangqi-760-keyboard-history.webview.png` — SHA256 `b6cd8743770568a99bd9f08636816d8eed8a9d45b0b82c09c2d340b082514d1f`

### Windows

- `docs/review/evidence/msh123/windows/normal-moves-chess-1180-pointer-history.webview.png` — SHA256 `847c5282496e8bf284d13479b48e8904bb574344aacd0c679ae600e72b95eebc`
- `docs/review/evidence/msh123/windows/normal-moves-chess-760-keyboard-history.webview.png` — SHA256 `e7267cde352ad6c8a82fa2a94ccf95ea55eb2517099abccda9abc2d3318c2f9a`
- `docs/review/evidence/msh123/windows/normal-shogi-moves-shogi-1180-pointer-history.webview.png` — SHA256 `a2ebcd912a7586d67a74e464d98758883581e6401302254deaef971b13643eac`
- `docs/review/evidence/msh123/windows/normal-shogi-moves-shogi-760-keyboard-history.webview.png` — SHA256 `f4dff29bdd3e2c6b92fa359ce13c39b7cde1a8d69ddc7b710405ae7ad8d184a3`
- `docs/review/evidence/msh123/windows/normal-xiangqi-moves-xiangqi-1180-pointer-history.webview.png` — SHA256 `0cd26066e8e932ea1eecdf544c4a7cda7582a17f2ed1150c3d1754d298997a5c`
- `docs/review/evidence/msh123/windows/normal-xiangqi-moves-xiangqi-760-keyboard-history.webview.png` — SHA256 `919b7420cf9f85dbf21fec9bc5262f9589a105831efc7876dea4f207e39b5c8f`

## Findings and bounded verdict

**PASS; no unresolved material MSH-123 visual finding in these12 captures.** Each game was personally viewed at1180x820 pointer-selected history and760x650 keyboard-selected history. Independently read accompanying DOM capture dimensions and exact selected history (Chess event2, Shogi/Xiangqi event1), all selected one-board-ply prefixes; minimum keyboard focus is the actual BUTTON containing e4 / ７６歩 / P9+1. The retained PNGs are byte-identical artifact copies, not edited/cropped mockups.

Full Chess8x8, Shogi9x9 and Xiangqi9x10 boards are visible in their board regions without board scrolling or zoom. Native artwork, Shogi ownership/pieces, Xiangqi intersections/palaces/river and common dark shell remain intact. The selected move and historical board agree: e2→e4,7g→7f,a3→a4, with lighter dashed origin and stronger double-underlined destination. Two-player headings and pair numbering stay native-appropriate; engine-reply text remains in the next player’s column and is not confused with selected earlier action. Japanese Shogi text is readable; Xiangqi WXF text remains legible. Font metrics/rendering differ between platforms, without changing these semantics. Root/provenance and historical-return controls remain accessible.

At minimum size the independent sidebar viewport clips the lower edge of the gold keyboard-focus outline in the Windows frames and Linux Xiangqi frame. The top and sides of the focus indicator, complete move label and selected background remain plainly identifiable, and the actual Tab/Enter campaign establishes successful operation. This is a minor edge-spacing observation, not an unreachable control, obscured board, lost selection or material regression requiring another MSH-123 implementation campaign. Existing MSH-124 sidebar spacing/order refinement may account for it; this reviewer did not broaden or activate that work. Do not describe these captures as proving a fully unclipped focus outline.

The earlier first-run Linux failure is preserved separately: the driver issued the next interaction while native capacity admission kept controls disabled. The corrected external-only readiness wait now requires the changed/restored orientation, selected prefix and enabled controls before further input; the successful later campaign does not erase the failed run. The unrelated known native process-race evidence also remains separately recorded by the integrator. This visual review does not infer rules correctness, native lifecycle success or normal instrumentation exclusion solely from pixels; their tests/package/report provenance remains separate.

Final scope: frontend/source +30 macOS images and12 actual Linux/Windows artifact images, **42 personally viewed package screenshots** overall; no unresolved material frontend or visual finding. Native notation/rules interpretation, overall CI and task-finalization checks retain their separate independent evidence.
