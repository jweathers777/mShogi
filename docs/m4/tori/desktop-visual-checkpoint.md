# Tori desktop visual checkpoint — MSH-42

This continues the [native playable checkpoint](native-playable-checkpoint.md)
with the production Tori board in the existing mShogi application. MSH-42 remains
In Progress. The owner must review the actual presentation before normal product
activation, final packaged Tori acceptance or complete-game closure.

## Presentation and ownership

The existing square-board renderer, native choice controller, player/hand frame,
Moves, analysis and application shell render Tori's native 7×7 geometry. There
is no Tori application page, frontend rules implementation or alternate session.
The native snapshot supplies complete actions, names, face/promotion/owner,
hand counts, history, outcomes and exact completed-action geometry.

[Original vector bird engravings](piece-artwork.md) distinguish all nine native
faces. Warm pentagonal tiles relate to Shogi; original bird images and engraved
notation aids give Tori its own presentation. Whole-tile rotation shows ownership.
Eagle/Goose use their actual native face, red engraving and a non-color promotion
cue. Left/Right Quail retain distinct artwork and identity aids after capture.
Native full names are available through accessible labels, selection text and
pointer name aids. Artwork has no operating-system font dependency.

Hands occupy the existing player slots. Legal moves and drops, forced promotion,
readonly opponent inspection, keyboard focus and Escape use the existing native
choice controller. Origin and emphasized destination cues come from native exact
history; drops show the destination only. No optional Tori promotion dialog or
frontend legal-drop calculation is introduced.

The surrounding dark shell, document controls, live/historical footer and
Analysis → Game Actions → Moves order remain shared. Moves uses the native Rust
formatter. Non-board events stay Position events. Actual engine observations
retain score perspective, depth and visits. Raw Tori PV encodings remain in
About/technical evidence rather than ordinary continuation text; this checkpoint
does not add replay work to polling to produce a new readable PV projection.

## Preview and approval boundary

Normal New Game, replacement and Open still exclude Tori. The stable macOS QA
application runs the same production renderer, controller and bundled engine;
its external `tori-standard` and `tori-contract` selectors change accepted native
starting states while the application is stopped. The contract diagram is labeled
by its native provenance, not represented as a historically reached standard game.
The separately labeled application's title identifies verification provenance.

Use the primary integration checkout and the fixed QA identity/path documented
in [packaged verification](../../desktop/packaged-gui-verification.md):

```sh
python3 desktop/scripts/macos-verification.py select qa tori-standard
open "desktop/src-tauri/target/verification/macos/mShogi Verification.app"
```

Quit through the native guard before selecting `tori-contract` or rebuilding.
The fixed bundle ID is
`org.mshogi.desktop.verification.r6.shogipromotion`; it is reused across scenarios.
Changing the selector does not rewrite or re-sign the bundle. A rebuild can still
trigger a platform-controlled approval prompt; no security settings are changed.

Actual normal/narrow/minimum application captures and a structured verification
record will identify the tested source, package, interactions and limitations.
Component tests and package inspection are distinct from those GUI journeys.
No owner visual approval, three-platform Tori gameplay acceptance or completed
MSH-42 is claimed by implementation alone.
