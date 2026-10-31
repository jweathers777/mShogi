# Actual macOS MSH-124 package interaction

Root Codex operated the real macOS application through native accessibility,
keyboard, pointer and screenshot APIs on 2026-10-31. Environment: macOS27.0.1
(26A434), arm64, Retina2x. No browser substitute, private IPC or injected input
was used. `package-manifests.json` identifies every file in normal and separate
verification packages; normal package smoke tests independently establish the
bundled engine and absence of verification markers.

## Initial source and unchanged evidence

Source `de5209d3ac0d987d42396a5b2e3bf4325800784d`:
normal application `/private/tmp/msh124-packages/mShogi.app`; separately labeled
`mShogi Verification.app` alongside it. Normal1180x820 content screenshots are
2360x1640 window pixels; verification native size menu and title explicitly
establish1180x820 or760x650 content (minimum window screenshots1520x1300).

- `normal-{chess,shogi,xiangqi}-1180.png`: same app header, dark theme, native
  board/player/footer slots, document controls and Analysis/Game Actions/Moves
  order, all boards fully visible. Shogi ownership/hands and Xiangqi native
  intersection/river/palace art retained. Independent sidebar scrolling does
  not move the board or common footer.
- `normal-chooser-options.png`: native select offers exactly Chess/Shogi/Xiangqi.
  Shogi and Xiangqi side captures show Sente/Gote/Random and Red/Black/Random.
  Actual Shogi Sente→Right→Gote→Left→Sente keyboard radio selection succeeded.
  The initial Mac popup Down/Return attempt did not change the selection;
  explicit native menu selection did. Do not count that attempt as successful
  dropdown keyboard selection. The external Windows/Linux journeys separately
  exercise native-select keys; unit tests establish its labeled SELECT/contents.
- `qa-{chess,shogi,xiangqi}-760.png`: exact supported minimum content, complete
  native boards and footer, available common controls with independent sidebar
  scroll. `qa-chooser-{shogi,xiangqi}-760.png` fits without dialog clipping.
- `normal-chess-1180-keyboard-history.png`,
  `qa-chess-760-keyboard-history.png`, `qa-shogi-760-keyboard-history.png`,
  `qa-xiangqi-760-keyboard-history-final.png`: actual board move and native
  engine reply, starting-position selection, real Tab/Enter move selection;
  exact selected one-ply position, two-column native notation, origin and
  emphasized destination cue, entire outward gold focus ring with lower gap.
  Xiangqi's first attempt ran before native selection had settled and the
  non-final-named capture shows root0 without a focus ring. That is retained
  as an intermediate observation, not claimed as the passing keyboard proof.
  After settling, Tab visibly focused P5+1; Enter selected exact post-action1.
- About pointer/keyboard disclosures exposed real context/diagnostics and full
  bundled font notices. Keyboard invocation and Escape restored About focus.
  `normal-about-keyboard-license.png` is the original long-URL clipping finding,
  retained as failing visual evidence rather than overwritten.
- Native Cmd-Q on dirty normal Xiangqi and verification games reached unchanged
  Save/Discard/Cancel guard; explicit Discard closed the app. Existing lifecycle
  behavior was not replaced with MSH-125 behavior.

## Final correction and actual recheck

Source `ead30e672f2a2cf1747c02c8daa508babed3b7c2` adds only
`.support-dialog p { overflow-wrap:anywhere; }`. No other production behavior or
geometry differs from the initial checkpoint above. Fresh normal and separately
labeled verification builds are in `/private/tmp/msh124-packages-final/`;
`final-normal-package-smoke.json` and final entries in `package-manifests.json`
record exact identity. Normal main-checkout bundle is also current for launch.

`final-normal-about-license-1180.png` and `final-qa-about-license-760.png` show
complete wrapped pinned-source URLs and hashes. Native keyboard invocation,
Tab→Font license→Return, then Tab→license link→Tab→Quit succeeded.
`final-qa-about-quit-focus-760.png` shows vertical focus reveal and a complete
Quit focus ring inside the minimum About viewport. Escape restored the actual
About invoker. Cmd-Q still reached the existing clean Close guard; Continue
closed. These are actual corrected-package checks, not source-only assertions.

All four applications were closed through native guards after verification.
No engine output or position was fabricated. This record does not claim new
Windows/Linux interaction; those require their own identified CI artifacts.

## Shared minimum card alignment and final keyboard chooser

Final production `4a624d6c4f129ce774d385192fbbe6a9bdcf547a` reserves232px for
all minimum-size Analysis cards. Actual first platform reports measured
Xiangqi221px onLinux and229.046875px onWindows against Chess200px, so the
original minimum did not keep common card positions stable. No comparison was
relaxed. Fresh `aligned-qa-{chess,shogi,xiangqi}-760.png` captures from the
identified aligned verification package show matching card positions, retained
native assessment, full boards/hands/river/palaces and footer. Reviewer viewed
all three, bringing independently viewed Mac captures to22. Exact platform
comparison/focus probes must still pass on the final CI packages.

The main-checkout normal app was also launched at final4a624d6. Keyboard-only
Tab traversal reached the native Game SELECT; typeahead `s` selected Shogi with
Sente/Gote/Random, then `x` selected Xiangqi with Red/Black/Random. Tab and two
Right arrows selected Random; Tab/Tab/Enter started the game. Actual native
choice was Black, with Black orientation and real engine Red-turn work. Capture
`aligned-normal-keyboard-dropdown-random.png` records the selected radio.
This establishes successful native-select keyboard interaction on macOS while
preserving the earlier unsuccessful popup Down/Return attempt honestly.

## Font-independent support icon

Source `91eb13e2451af77150b82b62a448bf97e9d5ac58` changes only the About child
from a font glyph to a decorative, nonfocusable SVG and centers it in the same
23px button. Independent4a Linux image review found the missing-glyph box even
though all GUI phases passed. This remains an open Linux visual finding until
its fresh actual package capture; passing automation alone did not waive it.

Fresh `vector-normal-chess-1180.png` and `vector-qa-chess-760.png` are from the
identified normal/separate verification91 packages. Root verified actual Mac
keyboard About entry, Close initial focus, Escape returning to About, and
unchanged native Quit guard. The final normal smoke passes; exact vector package
hashes are appended to the manifest. Source/rules/archive/board behavior and
prior actual cross-game and full-ring evidence are unchanged.

Final-source Linux recapture resolves F-V03: independent reviewer personally
viewed the six normal/minimum three-game headers plus About/chooser and
confirmed the recognizable SVG info icon. Exact report/manifests and screenshot
hashes are retained in `../vector-linux/` and `../vector-platform-audit.json`.
This closes the Linux finding using Linux pixels, separately from the Mac
keyboard/package observations above. The original tofu captures remain in
`../linux/`.
