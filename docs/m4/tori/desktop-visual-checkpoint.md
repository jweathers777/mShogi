# Tori desktop visual checkpoint — MSH-42

**Owner visual approval — 2026-11-02:** John reviewed the running macOS
application and approved the traditional kanji, board, hands, promotion treatment
and layouts at integration `5a6ec9f94c2b1dd3995d3f96d3b874f30e272090`
(packaged production `31cd776e2bde2997b1d87f4e5215749892b8bcfb`). Preserve
that design except for a concrete functional or accessibility defect. This
approval authorizes the remaining normal-product activation and complete
MSH-42 acceptance. The pending-approval/gated-product statements below preserve
the historical visual checkpoint; they are superseded by this owner decision.
Final functional/platform acceptance remains separate from visual approval.


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

The default [traditional Tori glyphs](piece-artwork.md) are the selected profile's
鵬, 鷹, 鵰, 鶴, 雉, 左鶉, 右鶉, 燕 and 鴈. A separate pinned, licensed Noto Serif
CJK JP subset supplies every glyph, independent of installed fonts. Warm
pentagonal tiles and whole-tile ownership rotation relate to the existing Shogi
presentation. Eagle/Goose retain their native face, red engraving and a non-color
promotion cue. Left/Right Quail remain distinct after capture. Full native names
are available through accessible labels, selection text and pointer name aids.
Traditional variant-native glyphs are the owner-approved default for all shogi
variants. Original bird artwork is preserved as an inactive future alternative;
no piece-set chooser is implemented or activated.

Hands occupy the existing player slots. Legal moves and drops, forced promotion,
readonly opponent inspection, keyboard focus and Escape use the existing native
choice controller. Origin and emphasized destination cues come from native exact
history; drops show the destination only. No optional Tori promotion dialog or
frontend legal-drop calculation is introduced. Undo/Take back preserve every
abandoned observation and native branch, while marking their observations
historical so retired work cannot appear as the current engine turn.

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

A QA-only native menu sets exact 1180×820, 960×720 and 760×650 logical content
sizes and reports measured content/frame dimensions. It is excluded from normal
distributables. The actual captures and [structured verification record](evidence/desktop-visual-checkpoint.json)
identify the tested source, signed package, interactions and limitations.
Component tests and package inspection are distinct from those GUI journeys.
No owner visual approval, three-platform Tori gameplay acceptance or completed
MSH-42 is claimed by implementation alone.

## Actual packaged checkpoint

Production source is `31cd776e2bde2997b1d87f4e5215749892b8bcfb`.
Both the normal application and the stable QA package were built from this
revision on macOS 27.0.1 (26A434), arm64. The QA package retains its established
bundle ID/path; selecting standard/contract/standard did not change any signed
file hash. Reuse across these rebuilds and scenarios produced no additional
Computer Use approval prompt during this checkpoint. Platform approval behavior
is not guaranteed by a bundle identifier, and no security policy was changed.
The normal package passed inventory/instrumentation-exclusion and actual bundled
engine smoke checks; those checks are distinct from GUI gameplay evidence.

The actual production session/board accepted a keyboard `Sw3dx3c`, then explicit
Resume started the bundled engine's `Fa4bx3c` (depth 2, 560 visits). Both hands
held one Swallow. The native drop `Sw*3d` received `Fa3c-4b` (723 visits).
Take back returned four plies to two, retained the abandoned canonical lineage
and both searches, marked those observations historical, and stopped execution.
Historical event 1 showed its own exact board, inspection and last-move geometry;
Return to live restored the unchanged two-ply game.

The OS Save/Load dialogs saved and reopened the complete
[29,590-byte game document](evidence/desktop-visual/saved-gui-game.mshogi).
It reopened UNMODIFIED with work stopped until Resume; observations were
historical, not pending requests. Actual read-only analysis at event 2 returned
+0.00 Swallows from Sente's perspective, depth 2 and 1,427 visits, with no move
played. Native Quit → Cancel preserved the unsaved game; a later Discard closed
only the disposable preview.

The existing accepted native contract diagram (declared root counter 31,
earlier history unknown) supplies all nine faces and both hands. Actual
`Fa7cx7b=Ea` captured a Swallow and compulsorily promoted to Eagle without an
optional dialog. Opponent Left Quail hand inspection remained non-actionable.
Native resignation produced Gote's win and a separate Position event while
retaining exactly one board-move row. No rules are computed in TypeScript.

## Screenshot gallery

All captures below are original screenshots from that same packaged source, with
paired accessibility trees, pixel dimensions and hashes in the evidence index.
The native QA title records measured logical content/frame sizes. These are real
sessions; the contract diagram is not claimed as a historically reached game.

| State | Actual application capture |
| --- | --- |
| Standard start, 1180×820 | [01](evidence/desktop-visual/01-standard-1180.jpg) |
| Gote at bottom, 1180×820 | [02](evidence/desktop-visual/02-gote-1180.jpg) |
| Selected board piece and native legal destinations | [03](evidence/desktop-visual/03-selected-board-1180.jpg) |
| Both hands after actual captures | [04](evidence/desktop-visual/04-both-hands-after-captures-1180.jpg) |
| Selected hand and native legal drops | [05](evidence/desktop-visual/05-selected-drop-1180.jpg) |
| Actual compulsory promotion | [06](evidence/desktop-visual/06-compulsory-promotion-1180.jpg) |
| Exact history and origin/destination cues | [07](evidence/desktop-visual/07-history-and-last-move-1180.jpg) |
| Narrow 960×720 | [08](evidence/desktop-visual/08-narrow-960.jpg) |
| Minimum 760×650 | [09](evidence/desktop-visual/09-minimum-760.jpg) |
| Gote-bottom minimum | [10](evidence/desktop-visual/10-gote-minimum-760.jpg) |
| All nine faces and both hands | [11](evidence/desktop-visual/11-all-nine-faces-hands-1180.jpg) |
| Complete drop and real engine reply | [12](evidence/desktop-visual/12-drop-and-engine-reply.jpg) |
| Historical native opponent inspection | [13](evidence/desktop-visual/13-historical-inspection.jpg) |
| Actual reopened game, stopped | [14](evidence/desktop-visual/14-reopened-stopped.jpg) |
| Take back and historical observations | [15](evidence/desktop-visual/15-take-back-historical-observations.jpg) |
| Real read-only analysis | [16](evidence/desktop-visual/16-readonly-analysis.jpg) |
| Native Quit cancellation | [17](evidence/desktop-visual/17-quit-cancel-preserves-game.jpg) |
| Native opponent-hand inspection | [18](evidence/desktop-visual/18-opponent-hand-inspection.jpg) |
| Minimum history/sidebar access | [19](evidence/desktop-visual/19-minimum-sidebar-moves.jpg) |
| Native resignation and distinct Position event | [20](evidence/desktop-visual/20-native-resignation.jpg) |

## Verification and independent review

Focused frontend checks cover all nine glyphs, board/hands, native complete
choices, compulsory promotion, both orientations, historical inspection,
selection cancellation/focus, current/historical observations and unchanged
Chess/Shogi/Xiangqi presentation. Strict TypeScript and normal frontend build
checks passed. Both Cargo workspaces and the standalone feature-gated host were
checked early; host tests cover the three exact QA sizes and product gates.
The structured record separates each earlier campaign from final source checks.

Actual GUI Take back exposed an abandoned-observation classification defect.
The new native regression failed before the fix, then passed for both Sente
Take back and Gote Undo after clearing only runtime current-observation IDs on
successful rewind. Canonical observations and retired lineage remain intact;
old replies remain rejected. The frontend now truthfully says that earlier
searches are retained as historical observations. The red and green probes are
preserved with the evidence. No rules, archive limits or schemas changed.

Independent review used a separate agent's read-only source, test, provenance,
JPEG and accessibility-text inspection. It passed the final boundary changes and
screenshots 01, 02, 04–11, 18 and 19 with no material finding. The reviewer did
not execute tests, application interactions or CI. It confirmed all 49 squares,
hands and footer at normal, narrow and minimum sizes, ownership/promotion,
distinct actionable/inspection markers and the accepted common shell. At minimum
size, long notation can wrap within its player column; the complete accessible
label remains available. Hand glyphs are smaller than board glyphs and retain
native full-name labels and pointer aids.

Required CI and exact job/source provenance are recorded in
[the CI evidence](evidence/desktop-visual-ci.json): all three Rust workspace jobs
at `7b114daf04bb142c2d71f7883e970c116e7e5669`, plus final-source desktop locked
checks, normal packages and bundled-engine inspection on all three platforms.
Only the two frontend wording/test files differ between those revisions; native,
host and workflow sources are identical. Earlier superseded campaigns and the
actual local failed probes remain recorded. Existing-game automated GUI jobs
are separate compatibility evidence; they do not establish Tori GUI behavior on
Windows/Linux. The full final Desktop workflow **failed**, despite its required
package/check/engine steps passing: Linux and Windows each failed the normal GUI
phase. The preserved [Linux](evidence/desktop-visual-linux-gui-failure.json) and
[Windows](evidence/desktop-visual-windows-gui-failure.json) traces and actual DOM
showed a stopped UNMODIFIED two-ply reopened game and the correct historical-search
message, while the harness still expected the obsolete “No engine search has
completed” text. The verification-only correction at
`05ceba4161a77d15633069d183a211992e0a0ff1` now requires a real saved engine-turn
observation, the truthful historical message and no current Last engine turn.
Independent review, Python syntax and assertions against both actual failed-run
DOM/archives passed. Production/package source remains identical to `31cd776`;
no new full Linux/Windows GUI rerun or full-workflow pass is claimed. This remains
part of final platform acceptance after the owner visual checkpoint.

Search completed before an active Cancel control could be exercised in this
macOS journey. GUI Escape cancellation and native Quit cancellation were tested;
search cancellation and stale-result rejection use the native/controller tests
and unchanged native checkpoint evidence. No new GUI cancellation claim is made.
Existing bounded replay/deadline qualifications and functional-strength limits
remain unchanged.

## Owner decision and remaining work

Ready for owner review: traditional Tori kanji, board, hand/drop/promotion
presentation, shared shell, both orientations and responsive layouts in the
launchable macOS QA build. MSH-42 remains In Progress with all eight original
criteria preserved. Normal Tori chooser/New Game/Open activation, final packaged
Tori gameplay on all three platforms and independent complete-game acceptance
remain after owner visual approval. No subsequent variant or M5 work began.
