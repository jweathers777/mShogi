# MSH-124 independent packaged visual review

Reviewer `/root/shell124_review`, screenshot-level independent review, 2026-10-31. Reviewer personally opened the PNGs below with view_image; no pixel editing/cropping or product/Backlog changes. Root drove native applications; this reviewer does not claim personal GUI execution or platform interaction solely from screenshots.

## Initial macOS package review

Checkpoint `de5209d3ac0d987d42396a5b2e3bf4325800784d`. Normal package smoke identifies `/private/tmp/msh124-packages/mShogi.app`, host SHA256 `687e2568c4f2b02ec31b43ce4556868297d4b1bb782b86ef8c79981bc8cbd23a`, bundled engine SHA256 `92592cb2a4f5ed57ed1b62498b7d288e2a4bc01cb4f0f87827eedf762f9ca5d7`. QA screenshots are separately labeled mShogi Verification standard with actual760x650 content in title; they are not misrepresented as normal-distributable instrumentation. Root must preserve final package metadata, actual interaction and normal instrumentation-exclusion evidence separately.

Personally viewed13 files in `docs/review/evidence/msh124/macos/`:

- normal-chess-1180.png
- normal-shogi-1180.png
- normal-xiangqi-1180.png
- normal-chess-1180-keyboard-history.png
- qa-chess-760.png
- qa-chess-760-keyboard-history.png
- qa-shogi-760.png
- qa-shogi-760-keyboard-history.png
- normal-about-keyboard-license.png
- normal-chooser-options.png
- normal-chooser-shogi-sides.png
- normal-chooser-xiangqi-sides.png
- qa-chooser-shogi-760.png

### Visible findings

Normal1180 all3 views read as one application: identical header/background/theme/brand/About/document controls/heading placement, common Analysis → Game Actions → Moves card order and styling. Chess squares, warm Shogi9x9 with correct rotated ownership/hands, and Xiangqi intersections/river/palaces remain native and fully visible. Player/footer regions survive. No board scroll/zoom shown. The Xiangqi fast assessment Unknown remains available as a gameplay Analysis disclosure, not hidden solely in About.

Chess historical normal1180 and actual760 minimum show complete outward gold keyboard ring around e4: all4 sides including lower edge have visible space before the clipping boundary. Shogi actual760 historical shows the corresponding complete ring around７６歩 with the native completed destination and dashed origin on7f/7g. Thus these captures establish improved actual focus rendering, not only a visible label. Two-column move notation and earlier-position/read-only distinctions remain recognizable. The sidebar itself scrolls when contents exceed minimum height; the board and status footer do not scroll out of view. This is accepted independent-sidebar behavior, not board clipping.

The compact native menu visibly lists exactly Chess/Shogi/Xiangqi. Native White/Black/Random, Sente/Gote/Random and Red/Black/Random side-button presentation stays consistent. Actual760 chooser fits and has unclipped game-select focus. The large cards, slogans and unnecessary orientation/untimed descriptions are gone.

**F-V01, actual visible clipping:** original normal-about-keyboard-license.png has a long pinned-source URL clipped at the right edge of the About notice paragraph. Required license content is available, but this is a concrete presentation defect. Root independently identified it and has a minimal `.support-dialog p { overflow-wrap:anywhere; }` correction pending commit/rebuild. Do not present the original screenshot as corrected evidence. Need final actual package recapture at normal/minimum and source/hash identification before closing this finding.

### Pending at this initial review

Xiangqi760 normal/historical complete-focus screenshots, corrected About captures with source/package provenance, and final GUI reports/CI remain pending. No final all-criteria visual verdict yet. Source/accessibility verdict is recorded separately in `/private/tmp/msh124-review-source.md`.

## Final macOS follow-up — reviewed current production source

Final production source `ead30e672f2a2cf1747c02c8daa508babed3b7c2`. Independently inspected the complete production delta from de5209d: only `.support-dialog p { overflow-wrap:anywhere; }` changed. This has no effect on previously reviewed board, chooser, history, focus-reveal, native or lifecycle boundaries; unchanged initial package images remain applicable to those parts.

Read `macos/package-manifests.json`: actual environment `macOS-27.0.1-arm64-arm-64bit`. Final normal `/private/tmp/msh124-packages-final/mShogi.app`, host SHA256 `3d62fa0565fd1bf39bb7c3b3ea5be1f5f8921469404f122a05fc19aa427e4258`; final separately named verification host `/private/tmp/msh124-packages-final/mShogi Verification.app`, SHA256 `593d7de024db4b65377d5f44cdda4e8a6445e71045eeb12938d41044fecb5f4f`; both bundle engine SHA256 `92592cb2a4f5ed57ed1b62498b7d288e2a4bc01cb4f0f87827eedf762f9ca5d7`. Normal package-smoke evidence is separately identified in `final-normal-package-smoke.json`.

Personally viewed these6 additional actual macOS PNGs,19 total for this review:

- qa-xiangqi-760.png
- qa-xiangqi-760-keyboard-history-final.png
- qa-chooser-xiangqi-760.png
- final-normal-about-license-1180.png
- final-qa-about-license-760.png
- final-qa-about-quit-focus-760.png

**Final bounded visual verdict: PASS, no unresolved material macOS source/visual/accessibility finding.** Xiangqi minimum-size full90-intersection board, both palace geometries, river, player/status/footer and common header/control placement stay visible. Its historical P5+1 move has the complete gold outward ring including the lower edge, clear space to the clipped ancestor boundary, correct selected first-side move column and distinct origin/destination cues. The earlier `qa-xiangqi-760-keyboard-history.png` was explicitly identified by root as a premature root-selected frame; it was not used as keyboard-move focus proof. The `-final` frame is the actual selected-move proof.

**F-V01 resolved:** final normal1180 and minimum760 About license captures visibly wrap the formerly clipped source URL, digest and notice paragraphs within the dialog. Required text is retained rather than hidden. The final760 Quit-focused frame shows the complete outward ring, accessible Quit and Close buttons, retained license link and internally scrollable disclosure content. About scrolling is intentional support-content scrolling; the game board behind the modal remains separate. The source fix is the one-line paragraph wrapping rule above.

Root reports actual native Mac Tab → About, summary Tab/Return, license-link and Quit traversal, Escape → restored About invoker, and Cmd-Q existing native guard interaction. Screenshot review corroborates those visible states and source/focused tests corroborate the trap; reviewer does not relabel root's native interactions as personal executions. CI/platform-wide outcome remains the integrator's separately audited evidence. This macOS review alone makes no new Windows/Linux GUI claim or native correctness claim.

## Subsequent Linux finding — prior macOS verdict has a platform-specific limit

Root supplied the actual first Linux GUI report `/private/tmp/msh124-linux-initial-report.json`, source de5209d, workflow41206493902. This reviewer read its failure: the strict same-shell comparison at760x650 measured Chess Analysis card200px high versus Xiangqi221px, both left506/top102/width236. Xiangqi's retained player-facing assessment made its minimum card taller. The earlier macOS screenshot review did not establish Linux layout equality; its PASS must not be represented as an all-platform pass. This is a concrete shared-shell alignment finding under124, not a reason to hide the assessment or weaken exact comparison.

Root proposes raising only the shared max-height700 Analysis minimum to222px for all3games, keeping native content, board geometry, R5 and MSH125 behavior unchanged. Reviewer considers that bounded correction appropriate, pending inspection of the actual source delta and refreshed actual minimum captures/report. The initial failure remains evidence. Separate stale verifier copy failures (old read-only/Recorded native evidence expectations) belong to external verification maintenance, not product semantics.

The actual first Windows report also fails Analysis card equality: Chess200 versus Xiangqi229.046875px at221px width (initial log line526), larger than Linux221. Root therefore uses shared232px, not the initial proposed222. Reviewer inspected the one-line200→232 source delta and approves its bounded scope; the strict comparison remains in place. Final actual minall3 captures/platform outcomes are pending before this finding is closed.

## Aligned actual minimum-size package follow-up

Personally viewed3 additional actual captures (22 macOS PNGs total): `aligned-qa-chess-760.png`, `aligned-qa-shogi-760.png`, `aligned-qa-xiangqi-760.png`, all labeled actual760x650 content and sourced from `4a624d6c4f129ce774d385192fbbe6a9bdcf547a`. Shared Analysis, Game Actions and Moves now begin/end at the same visual positions across all3 games. Xiangqi keeps visible Unknown assessment inside the common232px Analysis card. All3 full boards, Shogi hands/ownership, Xiangqi90intersections/river/palaces, native player/footer/status, shared brand/header/file controls are intact. No new minimum-size board clipping or layout divergence is visible. Thus F-V02 is visually resolved on actual macOS; the separate failed Linux/Windows comparisons still require successful current-source platform evidence before claiming their resolution.

Read updated manifest: aligned normal `/private/tmp/msh124-packages-aligned/mShogi.app`, host SHA256 `3d5d9d91793c9ab71d1254c2a5d8e0c4db58c006916e4367d446e6626754c1b9`; aligned verification host SHA256 `fb74eac91227ccbf704b96a16593885ec7d506f09b1dd9f35dd4369607d2902e`; engine remains92592cb2a4f5ed57ed1b62498b7d288e2a4bc01cb4f0f87827eedf762f9ca5d7. Final normal protocol/instrumentation smoke is separately `aligned-normal-package-smoke.json`. No new package keyboard/Moves focus regression is inferred from this small height-only delta: previously reviewed actual focused captures remain valid for unchanged focus implementation, with platform focus probes still required.

## Font-independent icon actual macOS follow-up

Personally viewed `macos/vector-normal-chess-1180.png` and `macos/vector-qa-chess-760.png` from `91eb13e2451af77150b82b62a448bf97e9d5ac58`. The circle/stem/dot SVG renders as a recognizable circled information icon at both dimensions; header/control placement,232px minimum Analysis alignment and full Chess board/footer are intact. No new visual finding. Updated manifests identify vector normal `/private/tmp/msh124-packages-vector/mShogi.app`, host SHA25629132ead620992bc6d769d9dbde8923bcff368f89b084d5b4c47db1a28f41ebf; separate vector QA host442e03d418824223b10d2afcd9f0714f2951cb333097f2ea18c33430a91a5a37; unchanged bundled engine92592cb2a4f5ed57ed1b62498b7d288e2a4bc01cb4f0f87827eedf762f9ca5d7. Root's native About/Close/Escape/restoration/Cmd-Q rechecks and `vector-normal-package-smoke.json` are separate evidence, not inferred solely from these pixels.

24 macOS screenshots personally viewed,52 total including28Linux/Windows. F-V03 specifically concerns Linux and remains pending an actual current-source Linux capture; successful Mac vector rendering does not by itself close that platform finding.

Actual final91Linux follow-up closes F-V03: personally viewed8 new normal/minimum all3/About/chooser frames and verified recognizable vector info icon; independently checked14retainedPNG hashes,6focusprobes and exact initialcard equality. See `/private/tmp/msh124-review-platform.md` for source/package/provenance. Current60imagesreviewedoverall; no unresolved material visual finding in currentMac/Linux or unchanged priorWindows source paths. Final91Windows execution remains separate/pending.

Final91Windows actual14image follow-up and all3 Desktop/native metadata are reviewed in `/private/tmp/msh124-review-platform.md`. **Final independent MSH124 visual/accessibility PASS; no unresolved material finding** at91eb13e2451af77150b82b62a448bf97e9d5ac58. Total74personallyviewedimages; methods/platform limitations/provenance remain explicit. Earlier failed/obsolete images are retained, not overwritten or represented as final success.
