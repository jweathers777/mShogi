# Chu working desktop visual checkpoint

MSH-43 remains open. This checkpoint implements the actual traditional Chu board
and functional native engine through the existing application, then stops for
John's aesthetic approval. Independent technical review is not owner approval.
Normal New Game/Load still expose only Chess, Shogi, Xiangqi and Tori. Chu uses
the separately compiled, clearly identified stable QA activation until approval.

The shared dark shell, player/status/footer, Analysis → Game Actions → Moves,
document controls, frontend session owner, native DesktopController/document
and native-choice interaction controller remain the existing ones. A Chu-specific
piece-art adapter supplies traditional vertical kanji on oriented pentagons,
red promoted faces and King/Prince identities. Chu has no hands. All144 squares,
both orientations, native ordered routes, capture/return geometry and completion
choices use authoritative native data; TypeScript contains no Chu movement rules.

The bundled OFL font subset retains all43 characters used by the29 traditional
face labels. Font provenance, reproducible subset command and hash are in
`desktop/src/assets/fonts/NOTICE.txt`. Owner terminology and native original/
promoted identities remain distinct even where movement faces share glyphs.
Readable native complete-action notation is documented in [notation.md](notation.md).

## Actual package and captures

This section is completed with actual source/package hashes, window dimensions,
interaction states and screenshot links after the running macOS verification.
Required sizes are1180×820,960×720 and the current760×650 minimum; geometry alone
is not evidence of legibility. Both orientations, promoted faces, King+Prince,
Lion route alternatives, captures/returns, history/last-action cues and analysis/
Moves visibility are included. A measured minimum-size defect must be resolved
or reported with a concrete proposed solution, not hidden by smaller text.

Use only the fixed QA bundle ID
`org.mshogi.desktop.verification.r6.shogipromotion` at
`desktop/src-tauri/target/verification/macos/mShogi Verification.app`.
Select an external `chu-*` fixture only while that app is stopped; do not create
an identity or bundle per scenario. See
[stable verification instructions](../../desktop/packaged-gui-verification.md).
The normal package contains neither QA roots nor the fixture selector reader.

Before final owner handoff, fill this record from actual packaged interaction,
include the supported launch command, integrate/push reviewed source and clearly
identify unavailable/failed checks. Windows/Linux build or component results do
not establish new Chu GUI gameplay. Normal activation, final platform gameplay
and final MSH-43 acceptance resume only after owner visual approval.
