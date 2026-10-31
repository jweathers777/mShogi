# MSH-124 shared shell acceptance

Scope: one shared desktop composition, concise presentation, supported-only
native game selector, 歩 branding, support access and the MSH-123 focus-edge
correction. Primary builder/integrator: root Codex in primary `codex/rewrite`;
external-verifier worker `/root/shell124_gui` owned only the two GUI driver/test
files. Independent reviewer `/root/shell124_review` authored the linked source
and screenshot-level accessibility reviews. No competing product builders.

Production source: `de5209d3ac0d987d42396a5b2e3bf4325800784d`, followed by the
one-line About wrapping correction `ead30e672f2a2cf1747c02c8daa508babed3b7c2`.
Source `4a624d6c4f129ce774d385192fbbe6a9bdcf547a` also reserves measured
common minimum card space and corrects two external verification copy probes.
Final production source `91eb13e2451af77150b82b62a448bf97e9d5ac58` replaces
only the font-dependent About glyph with a decorative inline SVG in the same
labeled button, after independent Linux screenshot review found a missing glyph. Task/evidence finalization commits
add no product changes. Baseline `97b9470943107ae842c8184081c5e8e4cd26ef32`
and all prior MSH-119/120/123 evidence remain preserved.

Final verdict: **PASS** for all six MSH-124 acceptance criteria. Independent
source, visual and accessibility review at final production91eb13e finds no
unresolved material issue. All 74 personally reviewed captures, exact packages,
platform observations and original failures are identified below. MSH-125,
MSH-121 and M4 remain separate and open.

## Delivered behavior and boundaries

- ApplicationShell fixes identical Analysis → Game Actions → Moves order and
  retains header/document/player/board/footer ownership and dark theme. No
  separate game page. Board geometry/art, Shogi hands and Xiangqi native
  intersection/river/palace presentation are unchanged.
- New Game replaces only cards with a labeled HTML SELECT offering exactly the
  three playable games. Side radio button markup/styles/arrows, native labels,
  Random and stale-token admission remain. No unsupported variants displayed.
- 歩 is the single application brand. Ordinary slogans, untimed/orientation
  narration, native/reconstruction prose and redundant read-only explanations
  are replaced with short player meanings. The exact shared inspection status
  and all genuine warnings/claims/declarations/uncertainty/recovery remain.
- About mShogi is a keyboard-accessible header button outside the sidebar.
  Diagnostics preserves full context/profile/token/history/native execution,
  observations/errors/resource facts and I/O receipts. Font license preserves
  both bundled font notices, complete OFL and download link for every game.
  Quit invokes the existing native close path. Disclosures participate in the
  shared focus trap; closed content is excluded and controls follow DOM order.
- Xiangqi fast proof/Unknown and saved exact proof/possible witness/Unknown/
  cancellation/failure meanings are readable in Analysis. Historical results
  identify their event and do not alter the current game. Full raw provenance
  remains in About. No new exact execution facility is advertised or run.
- Eight-pixel gutters in Moves/sidebar and bounded non-board focus reveal keep
  the existing outward3px/offset3px ring visible through nested scroll edges.
  Exact native Moves selection, two columns, origin/destination cues and R5
  selection ownership are unchanged.
- No native Rust/rules/archive/capacity/engine contract changed. No MSH-125
  behavior implemented: Save/Save as/Open, dirty replacement/quit guards,
  Resign confirmation, Return to live and explicit Resume remain unchanged.

## Evidence

| Acceptance | Evidence |
|---|---|
| AC1 common composition/native boards | Shared-shell/all3 application regressions; actual macOS normal1180/min760 comparisons and independent visual review; platform evidence below |
| AC2 supported dropdown/unchanged side buttons | ProductApp native admission/native-side tests; real dropdown and side-button captures; external native-key journeys |
| AC3 brand/copy/semantic distinctions | All3 presentation, native recovery/capacity/analysis tests, source review and actual GUI |
| AC4 support/proof/licenses/Quit | SupportDialog/Assessment tests, full raw native payload equality, real keyboard disclosures/link/Escape/focus/native Quit; independent review |
| AC5 focused/package/minimum/exclusion |272 frontend checks, strictTS,73 external verifier checks; exact package manifests/smoke and actual Mac plus identified platform journeys |
| AC6 review/integration | Independent source141focused+TS,5finalSupportchecks,73driverchecks and screenshot-level Mac/Linux/Windows visual/accessibility review; final provenance below |

Author commands in `desktop`, Node24.19 runtime, Rust1.99.1/macOS27.0.1arm64:
`npm test -- --reporter=dot` **272PASS/25files**;
`npm run check` **PASS**.
`python3 -m unittest discover -s desktop/scripts -p test_gui_smoke.py`
**73PASS** after the two focused CI-probe regressions; Python syntax compilation **PASS**. Independent source reviewer
executed141focused tests and TS, not misreported as the full author suite.
Normal `npm run desktop:build -- --bundles app`, separate verification
`npm run tauri -- build --features desktop-verification --config
src-tauri/tauri.verification.conf.json --bundles app` **PASS** at the recorded source
checkpoints, including final91eb13e. Known vendored Tao compiler warnings are unchanged; no hook bypass.
Normal `package-smoke.py` **PASS**, including excluded test markers and actual
bundled UCI/internal engine-turn/move-analysis/bounded Full queries.

[Independent source review](evidence/msh124/source-review.md),
[actual Mac interaction/provenance](evidence/msh124/macos/interaction.md),
[Mac independent visual review](evidence/msh124/macos/independent-visual.md),
[package manifests](evidence/msh124/macos/package-manifests.json),
[final vector normal smoke](evidence/msh124/macos/vector-normal-package-smoke.json).
Actual screenshots live beside these records. Native verification window titles
establish content dimensions; Retina window pixels are separately recorded.

## Findings preserved and resolved

Initial frontend failures were old copy/order expectations plus a meaningful
new About focus-order regression. A test-event adjustment alone did not fix the
latter; the explicit DOM-order trap correction did. Both independent failures
and their successful rerun remain recorded. No failed check was waived.

Original actual About notice clipped a long source URL. Independent visual
finding F-V01 agrees with root's observation; final ead30e6 adds wrapping.
Fresh normal1180 and separate verification760 captures show the complete text,
license link and focused Quit; reviewer confirms resolution. Original failing
capture is preserved. Xiangqi's first Mac Tab/Enter attempt preceded settled
native selection; its root-only capture is retained but excluded as focus proof.
The subsequent exact one-ply P5+1 capture establishes the actual passing case.

The original MSH-123 minimum lower-edge ring issue is corrected: all4sides are
visible with space below in actual Chess/Shogi/Xiangqi760 captures. The external
verifier additionally measures the computed outline against every actual
clipping ancestor and viewport; it does not accept only button-label visibility.

The first actual Linux/Windows CI run41206493902 found3of15phases failing
on each platform;12passed. Two obsolete verifier strings (historical prose and
old evidence title) are replaced with current copy plus stronger exact native
prefix/count checks. The strict initial-card comparison exposed F-V02: minimum
Xiangqi Analysis content needed221px onLinux/229.046875px onWindows versus the
shared200px minimum. Final4a624d6 reserves232px for **all** minimum-size Analysis
cards; no per-game shell or relaxed comparison. Original failed-run summaries
remain in [initial CI evidence](evidence/msh124/initial-ci/). This finding required
fresh minimum package/CI proof; the passing4a run and independently reviewed normal/minimum captures below establish it.

## Platform CI and final integration

Desktop run [41207699334](https://github.com/jweathers777/mShogi/actions/runs/41207699334)
at4a624d6 passes all3package jobs, including15/15actualGUIphases each onLinux
andWindows. The root [artifact audit](evidence/msh124/platform-audit.json)
verifies every435Linux+396Windows report file and all13/2/5normal package
manifest entries (Linux/Windows/macOS), plus12actual full-outline measurements
and exact shared initial card rectangles atnormal1180/min760. The reviewer
personally viewed28actual platform PNGs and independently checked their hashes
and12focus bounds; this is distinct from root's831file audit and native input
execution in the CI runner. See [platform review](evidence/msh124/platform-review.md).

**F-V03, independently caught despite green automation:** the Linux font lacks
ⓘ and showed an empty About box. Source91eb13e uses a decorative unfocusable
SVG circle/i/dot in the unchanged23px button with the same aria-label, title
and callback. Independent source review,5support tests andTS pass; author full
272tests andTS pass again. Actual fresh normal1180/min760 Mac captures confirm
the final icon, keyboard About/Close focus, and native Quit. Final Linux91eb13e package passes15/15GUIphases. The retained
[vector artifact audit](evidence/msh124/vector-platform-audit.json) verifies
all435reportfile hashes,13normalpackageentries,sixfull-outline bounds and six
shared card layouts. New vector-linux screenshots preserve the original4a
images for direct comparison. Final independent/platform disposition follows.

Current-source4a native run41207699395 attempt1 failed two unchanged runner
timing/classification tests. Linux matches the existingMSH-112 early-exit
Brokenpipe/MissingResponse-vs-UnexpectedExit case; macOS matches the separate
MSH-72/76 retirement355 LateReply-vs-ResourceLimit race. NoRust/Cargo/tauri
production diff exists in124. Both focused tests pass locally, not claimed as
platform success or fixes. [Independent disposition](evidence/msh124/initial-ci/native-independent-disposition.md)
and original failures are retained. Earlier identical native sources passed
all3 at41206493928/de5209d and41206889275/ead30e6. The supported same-source
failed-job rerun is attempt2; Windows passes and its unaffected job is retained.
Attempt2 passes all3platforms on identical4a source; this is a successful
retry, not a fix of either race. Original failed logs and the reviewer diagnosis
remain. No assertion/timeout change or new native implementation is made in124.

Final native run [41208986510](https://github.com/jweathers777/mShogi/actions/runs/41208986510)
passes Linux, macOS and Windows at exact production91eb13e, including workspace
checks and isolated QA-constructor checks. [Final CI metadata](evidence/msh124/native-final-ci.json)
and [prior same-source retry](evidence/msh124/initial-ci/native-aligned-retry.json)
retain exact jobs, steps, attempts and source identities. Unchanged runner races
remain documented limitations; this task does not claim to resolve MSH-112 or
the separate historical retirement classification race.

## Final-source acceptance reconciliation

Final Desktop run [41208986515](https://github.com/jweathers777/mShogi/actions/runs/41208986515)
passes all three jobs at `91eb13e2451af77150b82b62a448bf97e9d5ac58`.
Linux and Windows each pass all 15 actual installed/packaged GUI phases.
The [final artifact audit](evidence/msh124/vector-platform-audit.json) checks
all 435 Linux and 396 Windows report-file hashes, every 13/2/5 normal package
manifest entry, 12 complete outline bounds and the exact common card rectangles
across three games at both content sizes. No missing manifest file is waived.
[Exact workflow metadata](evidence/msh124/desktop-final-ci.json), final
`vector-linux/`, `vector-windows/`, and `vector-ci-macos/` records identify
source, package/engine hashes, host environment, input method, screenshots,
actual observations and normal-build instrumentation exclusion.

These are actual remote application interactions, with native input and
read-only DOM observations, not browser simulations. Linux/Windows exercised
normal packages and separately labeled verification packages. macOS CI proves
packaging and bundled-engine checks; the local macOS native CUA journeys and
identified normal/verification packages separately establish Mac interaction.
The original failed probes, failed native attempt, unready Mac key attempt and
all three visual findings remain preserved rather than overwritten by green
runs. Final independent source/visual/accessibility review is **PASS**, with no
unresolved material MSH-124 finding. The reviewer personally viewed 74 actual
screenshots (24 Mac, 28 earlier remote, eight final Linux, 14 final Windows),
independently verified retained image hashes, and recomputed complete focus
bounds and cross-game geometry. This review is distinct from root's full
artifact audit and the CI drivers' actual native input execution. The linked
reports preserve the original findings and final disposition.

## Launch and remaining ownership

Normal macOS launch from the primary checkout:

```sh
open /Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app
```

For a rebuild use Node24 per `desktop/.node-version`, then `cd desktop && npm ci
&& npm run desktop:build -- --bundles app`. Verification packages are explicitly
labeled and unnecessary for normal play. Both packages bundle the real engine.
MSH-125 is the next bounded UX task; MSH-121 retains final migration acceptance.
No later variant or M5 work is started, and M4 remains open.

Evidence text normalization: staging detected Windows CRLF and trailing spaces
in retained raw logs. Repository text now uses LF without trailing whitespace;
JSON semantic equality was checked, and no event text, timestamp, diagnostic,
image or image hash changed. Original artifact ZIP identities and
[before/after text hashes](evidence/msh124/text-normalization.json) retain byte
provenance. Both artifact extraction scripts reproduce LF text. The initial
staged whitespace failure and subsequent correction are recorded here.
