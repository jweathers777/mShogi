# Chu approved desktop visual checkpoint

## Owner approval — 2026-11-03

John reviewed the actual running macOS application and explicitly approved the
traditional kanji, promoted pieces, King/Prince distinctions, Lion interaction
and responsive 12×12 layouts. Approved integration:
`cf9e50b122cb879d9f01677034fce88bcd92f222`; production:
`c52e253b9f8d61d30434f53b818ad3ae52becf5b`; verification-only correction:
`f43583de84c147591c4e83dd07068aa1779b3e09`.

The approved presentation is preserved. The historical checkpoint below was
captured through stable QA activation while normal New Game/Load still exposed
four games. That historical gate is now resolved by owner approval; MSH-43
now delivers normal fifth-game activation and actual three-platform gameplay.
[Final delivery reconciliation](delivery-review.md) records current evidence and
acceptance; the M4 milestone remains open.

The shared dark shell, player/status/footer, Analysis → Game Actions → Moves,
document controls, frontend session owner, native DesktopController/document
and native-choice interaction controller remain the existing ones. A Chu-specific
piece-art adapter supplies traditional vertical kanji on oriented pentagons,
red promoted faces and King/Prince identities. Chu has no hands. All 144 squares,
both orientations, native ordered routes, capture/return geometry and completion
choices use authoritative native data; TypeScript contains no Chu movement rules.

The bundled OFL font subset retains all 43 characters used by the 29 traditional
face labels. Font provenance, reproducible subset command and hash are in
`desktop/src/assets/fonts/NOTICE.txt`. Owner terminology and native original/
promoted identities remain distinct even where movement faces share glyphs.
Readable native complete-action notation is documented in [notation.md](notation.md).

## Historical visual-checkpoint package and captures

Production source is **c52e253b9f8d61d30434f53b818ad3ae52becf5b**, following
functional engine/console/admission source 8d01768. Actual local macOS arm64
packages are 20.75MiB normal and 21.79MiB QA. Their executable, bundled-engine,
resource and signing hashes are in the single current-source structured record
[evidence/visual-checkpoint.json](evidence/visual-checkpoint.json). The normal
package passed its instrumentation-exclusion and real bundled-engine smoke.
The QA host uses the production renderer, controller, document and engine;
feature-only imported roots and native window-size menu are labeled in its title.

The following are genuine Retina 2× screenshots of the actual running package,
not mockups. File names and accessibility observations are retained alongside
these captures. Logical content dimensions are shown in each native window title.

| State | Capture |
|---|---|
| Standard start, Sente bottom,1180×820 | [01](evidence/desktop-visual/01-standard-1180.jpg) |
| Standard start, Gote bottom,1180×820 | [02](evidence/desktop-visual/02-gote-1180.jpg) |
| Narrow960×720, Gote /Sente bottom | [03](evidence/desktop-visual/03-gote-960.jpg), [29](evidence/desktop-visual/29-sente-960.jpg) |
| Minimum760×650, Gote /Sente bottom | [04](evidence/desktop-visual/04-gote-minimum-760.jpg), [05](evidence/desktop-visual/05-sente-minimum-760.jpg) |
| Minimum sidebar scrolled to Moves | [07](evidence/desktop-visual/07-minimum-moves-sidebar.jpg) |
| Native opponent inspection | [08](evidence/desktop-visual/08-opponent-inspection.jpg) |
| Human move, actual engine reply and telemetry | [09](evidence/desktop-visual/09-real-play-engine-result.jpg) |
| Actual read-only analysis /historical root /stopped reopen | [10](evidence/desktop-visual/10-read-only-analysis.jpg), [11](evidence/desktop-visual/11-history-root.jpg), [12](evidence/desktop-visual/12-reopened-stopped.jpg) |
| Partial Lion capture and alternate quiet route, no action submitted | [13](evidence/desktop-visual/13-lion-partial-capture.jpg), [14](evidence/desktop-visual/14-lion-quiet-route.jpg) |
| Same-endpoint quiet return /capture return | [15](evidence/desktop-visual/15-lion-quiet-return.jpg), [16](evidence/desktop-visual/16-lion-capture-return.jpg) |
| Native dirty Quit protection | [17](evidence/desktop-visual/17-dirty-quit-guard.jpg) |
| Actual promotion choice /Prince alongside King | [18](evidence/desktop-visual/18-native-promotion-choice.jpg), [19](evidence/desktop-visual/19-promoted-prince-king.jpg) |
| Representative imported promoted faces, both orientations | [20](evidence/desktop-visual/20-promoted-kanji-1180.jpg), [21](evidence/desktop-visual/21-promoted-kanji-gote.jpg) |
| Ordered two captures, partial /historical completed cues | [22](evidence/desktop-visual/22-double-capture-partial.jpg), [23](evidence/desktop-visual/23-double-capture-history-cues.jpg) |
| Actual Sente engine move with human Gote | [25](evidence/desktop-visual/25-real-engine-gote-human.jpg) |
| Actual analysis Cancel /Replace /completed replacement | [26](evidence/desktop-visual/26-analysis-cancelled.jpg), [27](evidence/desktop-visual/27-analysis-replaced.jpg), [28](evidence/desktop-visual/28-replacement-analysis-complete.jpg) |

Captures24–28 use the established **external QA-only five-second engine gate** to
make real pending execution observable. It forwards the actual bundled engine;
it invents no move, score or progress. Its source/binary hashes and exact staged
environment are recorded. Ordinary engine timing was restored before handoff.
The remaining captures use the direct bundled engine. The actual saved two-ply
standard game with two observations is retained as
[real-game.mshogi](evidence/desktop-visual/real-game.mshogi):53,418 bytes, eight
controls, exact native actions/history and root/analysis provenance. Native
Open left it unmodified and stopped, retaining searches as historical information.

Partial route selection/Escape preserved zero plies. Actual quiet return
`Ln6f-6e-6f` and capture return `Ln6fx7e-6f` retained distinct complete identities
and notation; Take back restored their decision point with work stopped.
Actual `Ln3cx2bx2a` removed two pieces in order; selecting its exact history
prefix restored the native position before the engine's `Ln1bx2a` response.
The promoted-face diagram is artwork coverage, not a claim of played reachability.
Actual Elephant promotion and both surviving King/Prince were separately played.

## Responsive findings and independent disposition

The first source 8d01768 screenshots exposed two material findings: Chess CSS
specificity overrode the intended uniform board, and in-cell coordinates were
unreadably small/overlapping. Historical captures remain in
[evidence/desktop-visual/superseded-8d01768](evidence/desktop-visual/superseded-8d01768).
The reviewed correction strengthens only Chu selectors and adds an optional
outside coordinate frame to the existing renderer. Default four-game geometry
is unchanged; route/capture/history centers use the same adjusted origin.
Independent final review ran 62 focused tests and inspected the actual corrected
normal/minimum/oriented/promotion/route/cancellation screenshots. Both findings
are resolved; no new material visual defect was found.

All 144 squares fit without board scrolling or zoom at all three sizes. The board
is approximately 430 logical pixels at 1180×820 and 310 at 760×650, with minimum
cells about 24 pixels and dense two-character kanji about 11–12 pixels. Coordinates
are now outside pieces, about 9 pixels at minimum. Normal-size kanji are intact
and readable in the actual capture. Minimum-size reading comfort remains an
explicit owner visual judgment; fit alone is not an accessibility claim. No
minimum size or product requirement was silently changed. A larger recommended
playing window is available without zooming the board.

Native accessible names retain exact square, side, original/promoted face and
choice distinctions. Pointer selection, keyboard Up/Enter play and Escape
cancellation/inspection clearing were actually exercised; automated tests cover
ported Complete/Cancel focus and route-keyboard behavior. Full VoiceOver and all
keyboard journeys were not independently operated in this checkpoint. The common
live footer survives both orientations and route controls. Moves is accessible
by scrolling the shared sidebar at minimum size; capture 07 also has the outer
page partly scrolled and does not establish a permanently fixed header.

## Verification boundary

The current structured record distinguishes local native/controller, actual
engine subprocess, archive admission, frontend, standalone host, packages and
actual GUI evidence. Existing native MSH-86/shared MSH-78 evidence is reused unchanged.
All three production-source Rust jobs passed. The corrected Desktop campaign
passed all three jobs at verification-onlyf43583d. The exact required CI evidence
is [Rust41623916090](https://github.com/jweathers777/mShogi/actions/runs/41623916090)
and [Desktop41627192949](https://github.com/jweathers777/mShogi/actions/runs/41627192949). Earlier source 8d01768 Desktop CI failed
because a newly added QA test lacked its feature guard in normal host tests;
c52e253 fixes the test gate. Its Linux GUI regression campaign subsequently
failed the existing Tori pending scenario: a depth-field lookup ran while Play
details was still active after Resume. The actual
[failure DOM](evidence/ci-linux-tori-readiness/failure.dom.json),
[screenshot](evidence/ci-linux-tori-readiness/failure.webview.png) and
[focused follow-up record](evidence/gui-readiness-f43583d.json) are preserved.
Verification-only revision **f43583de84c147591c4e83dd07068aa1779b3e09** waits for
settled native Resume and active analysis inputs, with one action dispatch per
step. The precise missed-click mechanism is an inference; the corrected actual Linux and Windows GUI
journeys passed, including the previously failing pending-Tori phase. Successful
readiness, replacement/cancellation and fresh reopen observations are retained
in the focused record. No production file
changed after c52e253, so native and local packaged Chu evidence remain valid.
The redundant native campaign caused by that Python-only commit is cancelled;
it is not counted as passed. No failed run is described as a pass.

The following verdict describes the pre-approval visual checkpoint. The owner
approval above and [current product acceptance](delivery-review.md) supersede its
pending-approval and normal-activation gates without relabeling its checks.

Only macOS Chu GUI interaction is established here. Existing four-game packaged
Windows/Linux CI regression journeys do not establish new Chu GUI gameplay.
Normal fifth-game activation, final packaged three-platform Chu gameplay and
all-eight-criterion product acceptance await owner visual approval. A 32-ply
real-engine smoke is not a completed full game or a playing-strength measurement.

## Historical QA checkpoint launch

The stable QA package is already staged at the path below, selected to the
standard native Chu root with ordinary engine timing. Press **Resume play** to
play Sente. Analysis pauses play explicitly; complete Lion routes use the native
landing choices and **Complete action here** control. Save/Load and native Quit
use the existing guarded document workflows.

```sh
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/verification/macos/mShogi Verification.app"
```

To select another native verification root, first quit that app through its
normal guard, then from the primary checkout run—for example:

```sh
python3 desktop/scripts/macos-verification.py select qa chu-promoted
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/verification/macos/mShogi Verification.app"
```

Available Chu roots are `chu-standard`,`chu-gote`,`chu-contract`,`chu-promotion`,
`chu-promoted`,`chu-royals`,`chu-captures` and`chu-captured`. Imported diagrams
retain missing earlier history; they do not prove historical reachability.
To rebuild this checkpoint, use the documented pinned Node24 runtime and run:

```sh
npm --prefix desktop ci
npm --prefix desktop run desktop:build -- --bundles app --features desktop-verification --config src-tauri/tauri.verification.conf.json
python3 desktop/scripts/macos-verification.py prepare qa
python3 desktop/scripts/macos-verification.py select qa chu-standard
```

Use only the fixed QA bundle ID
`org.mshogi.desktop.verification.r6.shogipromotion` at
`desktop/src-tauri/target/verification/macos/mShogi Verification.app`.
Select an external `chu-*` fixture only while that app is stopped; do not create
an identity or bundle per scenario. See
[stable verification instructions](../../desktop/packaged-gui-verification.md).
The normal package contains neither QA roots nor the fixture selector reader.

At this historical working checkpoint, MSH-43 remained unfinished. Owner aesthetic approval
of the actual traditional-kanji presentation is the next step. Normal activation,
final platform gameplay and final MSH-43 acceptance resume only after that approval.

Historical technical verdict: PASS for this working visual checkpoint. MSH-43 is
In Review for owner aesthetic approval, with its original eight criteria and
final product acceptance still open. No required technical finding remains at
this checkpoint. Minimum-size reading comfort remains explicitly presented for
owner judgment; later normal activation and three-platform Chu gameplay are
not established by the existing four-game CI campaigns.
