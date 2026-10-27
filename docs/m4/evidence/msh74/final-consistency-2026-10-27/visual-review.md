# Independent final consistency visual review

Final verdict: pass, no outstanding material findings in the reviewed Mac, Linux
and Windows states. Earlier sections preserve the staged review and its then-pending
platform boundaries; the final Windows/all-platform verdict below supersedes them.

Reviewer: delegated `msh74_visual_review`, separate from the builder.
Method: used `view_image` to inspect six unaltered native Mac JPEG captures from
product source `abe66c5`, including the Chess/Shogi normal pair and Shogi
Unmodified, modified, historical and returned-live-paused states. All2360×1640,
corresponding to the same1180×820 native window including title bar. Fixture-only
`7df91d3` changes no product source or expected JSON values.

Both new documents display Unsaved in the same header location. Shogi then
clearly displays Unmodified after Save and Unsaved changes after Flip. Play
limits occupy the same panel position: Chess depth2/20,000nodes, Shogi2/5,000.
Shogi's common footer visibly shows Your turn/Live position0plies, historical
read-only/Selected position1ply, and explicit paused/Live position2plies.

Header, Flip, player/board/sidebar geometry remains coherent. Hands remain with
players; complete9×9/8×8 boards fit. No redesign, clipping or material visual
finding. This verifies captured appearance, not engine-limit execution or full
keyboard/screen-reader conformance. Source/native tests and GUI journey reports
have separate scopes. Current platform minimum-size assessment follows when
actual final-source captures are available.

The owner accepts the major visual design and permits completion after the
three bounded corrections and required review/verification, without another
owner checkpoint. This conditional authorization does not substitute for evidence.

## Actual final-source Linux comparison

Independently reviewed unaltered packaged-GUI captures from integration revision
`1b10277436b94541235e15189a3af7fce867351c`, with product behavior/appearance
unchanged from reviewed `abe66c5` and subsequent external-driver assertion fixes.
Artifact directory:

`/private/tmp/msh74-consistency-ci/1b10277436b94541235e15189a3af7fce867351c/linux/gui-evidence/`

Method: actual `view_image` comparison of:

- `normal-initial-chess-normal-size.webview.png` and
  `normal-shogi-initial-sente.webview.png` at 1180×820.
- `normal-initial-chess-minimum-size.webview.png` and
  `normal-shogi-minimum-size.webview.png` at 760×650.
- `normal-inspect-chess-e7.webview.png` and
  `normal-shogi-inspect-shogi-7c.webview.png`, both actual active inspection
  at 760×650.
- `normal-active-analysis.webview.png` and
  `normal-shogi-active-analysis.webview.png`.
- `normal-historical-analysis.webview.png` and
  `normal-shogi-reopened-historical-analysis.webview.png`.
- `normal-shogi-reopened-opponent-hand-inspection.webview.png` for counted
  hands and native drop inspection after reopening.

Associated DOM records corroborate DPR1 viewport dimensions; footer visibility
was judged from the actual images, not inferred from DOM presence. No image
alteration or source-only layout inference was used.

All three corrections are visibly supported. Both initial documents use Unsaved
at the same header location; both historical saved documents with new work show
Unsaved changes. Shogi's Play details displays depth2/5000 nodes in the same
location/style as Chess depth2/20000. Locale-specific lack of digit grouping does
not change the displayed values. The shared footer visibly retains Your turn
and Live position0 in ordinary play, paused/Live position2 during read-only
analysis, and read-only history/Selected position1ply during historical analysis.
The Shogi footer's native status remains visible below the wrapped inspection
wording at minimum size; Chess's corresponding status and position remain visible.

Both entire boards, player rows and Shogi hand areas remain visible at minimum
size. Counted hands remain beside their respective players; opponent hand origin,
hollow drop rings and explicit read-only wording remain clear. Shared header,
document/file controls, Flip, player/board relationship, card treatment/order and
footer composition remain coherent. Cancel analysis remains beside Replace
inside Analysis, and Return to live remains at the upper-right of the historical
notice for both games. Additional native Chess analysis controls remain inside
their established section. Sidebar content may require its own independent
scroll; this is not board scrolling.

**Linux visual verdict: no material finding or board/hand/layout regression in
the inspected final-source states.** The integrator reports the Linux packaged
GUI job passed; platform audit owns functional phase/CI conclusions separately.
This review establishes captured appearance, not rule correctness, actual search
limit execution, formal accessibility conformance or complete keyboard behavior.
Current final-source Windows packaged evidence remains pending; older unchanged
Windows visual evidence is not relabeled as a new final-package run. No further
owner checkpoint is introduced beyond the latest conditional completion
authorization, and MSH-75 remains outside this work.

## Actual final-source Windows comparison and final verdict

Independently reviewed actual packaged Windows GUI captures from revision
`1b10277436b94541235e15189a3af7fce867351c`, Desktop workflow `40502381314`, under:

`/private/tmp/msh74-consistency-ci/1b10277436b94541235e15189a3af7fce867351c/windows/gui-evidence/`

Method: used `view_image` on the same normal, minimum, actual minimum-inspection,
active-analysis, historical-analysis and reopened opponent-hand filenames listed
in the Linux assessment. Normal comparisons use complete 1180×820 WebView
client captures; minimum comparisons use 760×650 clients. Also viewed
`normal-initial-chess-minimum-size.desktop.png`, showing the actual complete
native minimum window. The Windows desktop is 1024×768, so its large native
window desktop captures are cropped by the screen: no complete normal-size
claim is inferred from those cropped desktop images.

The three bounded corrections are visibly retained. Both initial document
headers say Unsaved in the common location; historical documents with subsequent
changes say Unsaved changes, and the reopened hand-inspection document says
Unmodified. Play details shows Shogi depth2/5,000nodes and Chess depth2/20,000nodes
in the same position/style. The common footer visibly includes native turn or
paused/history status and current/selected board-ply information. In particular,
both minimum active-inspection captures show that footer below the read-only
inspection wording without clipping; this was checked in pixels rather than
inferred from DOM presence.

No board/hand/layout regression was identified. Full boards and both player rows
fit normal/minimum sizes; counted hands remain with players, and the inspected
opponent-hand outline, hollow drop marks and read-only wording remain clear.
The single dark shell, common controls/card hierarchy, Analysis Replace/Cancel
row and historical upper-right Return to live remain consistent. Game-specific
observations and Chess Full controls stay within their established sections.
Independent sidebar scrolling is visible and does not require board scrolling.
Windows font metrics and Chess glyph artwork retain their previously recorded
platform differences, without missing Shogi glyphs or a material visual issue.

**Final independent visual verdict: pass; no outstanding material visual
finding in the inspected final-consistency Mac, Linux and Windows evidence.**
The integrator reports all three Desktop jobs succeeded; platform audit owns
the separate package/interaction/CI verification. This verdict establishes
captured appearance, not rule/search correctness, actual limit execution,
formal screen-reader conformance or all keyboard journeys. Prior review history
remains intact. Under the owner's latest authorization, no further visual-owner
checkpoint is required after the three corrections and required verification;
this review does not add one. MSH-75 remains outside the completed boundary.
