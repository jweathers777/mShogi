# Independent visual review — common dark application shell

Current product source: `19beb7437f11003734d65ee68f69a5f34bbdd02c`. Final
independent screenshot verdict: no outstanding material visual findings in the
reviewed Mac normal and Windows/Linux normal, minimum, inspection, analysis and
history states. Owner approval remains outstanding; MSH-74 stays In Review.
Earlier sections preserve the review sequence, including findings and provisional
limits; final current-source assessments below supersede them.

## Preliminary source and contrast assessment

Fresh delegated reviewer `msh74_visual_review`, 2026-10-27. Product source:
`a2524c10d2067f9f5ac3294c1a78efcddd771e99`. The subsequent `61afcec` change
affects the external GUI driver only; it is not a new product appearance claim.

Method: read the recorded owner shell-refinement-2 requirement in
`docs/m4/shogi-desktop-product.md`; inspect the actual `ApplicationShell`,
`GameArea`, `PlayerArea` and `InspectorSwitch` composition and current CSS; calculate
sRGB contrast from declared foreground/background colors. This assessment uses
source inspection, not GUI interaction or a screenshot-based layout judgment.

Both game adapters use one dark application palette and shared physical slots
for header/document/file controls, live heading/Flip, upper player, board, lower
player, footer and sidebar. The sidebar shares Analysis, Game history, Game
actions and collapsed technical access. No game-specific shell theme branches
were found in the authoritative `shell.css`; the warm Shogi board, semantic
pieces and hands remain game content.

Sampled contrast ratios:

| Content | Foreground / background | Ratio |
| --- | --- | ---: |
| Muted text on workspace | `#b9cbc7` / `#111e23` | 10.07:1 |
| Muted text on panels | `#b9cbc7` / `#1b2e35` | 8.35:1 |
| Muted text on header | `#b9cbc7` / `#14292e` | 8.97:1 |
| Primary panel text | `#e7ece5` / `#1b2e35` | 11.77:1 |
| Dirty header state | `#e2c58b` / `#14292e` | 9.08:1 |
| Paused/notice text | `#d5e2ce` / `#263d36` | 8.65:1 |
| Historical notice | `#e2d3a9` / `#383a2d` | 7.80:1 |
| Primary control text | `#192e29` / `#d5e4bd` | 10.70:1 |
| Read-only inspection label | `#ffd3df` / `#3c2430` | 10.51:1 |

Global button focus styling remains present for product dialogs after removal of
the old light-theme rules. The shared shell also explicitly styles button, input
and technical-summary keyboard focus. No material source-level visual finding
was identified in this preliminary assessment.

## Required screenshot assessment — pending

Actual current-product Chess and Shogi screenshots must still be compared at
matching normal and supported minimum dimensions. Review header/document and
file-control locations, common heading/Flip, player/board relationship, shared
card geometry/order, complete-board visibility, inspection labels/markers and
focus where captured. Unexplained common-control migration remains a finding;
shared source structure alone does not prove the rendered result.

Earlier light-Shogi or prior-shell screenshots are historical evidence only and
are not proof of this refinement. Current native macOS capture is unavailable
while the Mac is locked; Linux/Windows GUI artifact pairs have not yet been
reviewed. This review remains open pending those screenshots.

This preliminary assessment does not establish full interaction, screen-reader,
rules, engine or platform-CI conformance. It does not constitute owner visual
approval. MSH-74 remains In Review; MSH-75 remains unstarted.

## Actual Linux paired screenshot review — finding open

Product source `a2524c10d2067f9f5ac3294c1a78efcddd771e99`, Desktop CI
`40487524795`. The integrator reports the Linux fourteen-phase job passed;
that functional result is separate from this visual assessment. Unaltered
artifacts were inspected with `view_image` under:

`/private/tmp/msh74-shell2-ci/a2524c10d2067f9f5ac3294c1a78efcddd771e99/gui-evidence/`

Actual pairs, compared at matching sizes:

- `normal-initial-chess-normal-size.webview.png` and
  `normal-shogi-initial-sente.webview.png` — 1180×820.
- `normal-initial-chess-minimum-size.webview.png` and
  `normal-shogi-minimum-size.webview.png` — 760×650.
- `normal-inspect-chess-e7.webview.png` and
  `normal-shogi-inspect-shogi-7c.webview.png` — actual active inspection at
  760×650, not an inference from ordinary minimum-size captures.

Also inspected both ordinary minimum-size `.desktop.png` companions,
`normal-inspect-chess-b8.webview.png` (historical Chess) and
`normal-shogi-reopened-opponent-hand-inspection.webview.png` at normal size.
Associated DOM records corroborate viewport dimensions and explicit inspection
focus names. No contact sheet or alteration of the originals was used.

The shared dark palette, header/document/file-control locations, live heading
and Flip, upper/lower player rows, board frames and sidebar card geometry align
across the ordinary paired states. Warm Shogi board/pieces remain distinct game
content. Both complete boards and both player/hand regions remain visible at
minimum size, including active inspection; sidebar Game actions/details can
require independent sidebar scrolling. Native choices, hollow inspection rings,
dashed origins, explicit read-only wording and occupied-piece identity remain
readable. No clipping of the inspected labels was observed.

**Material finding: the common selection/help/inspection status still changes
physical order between games.** Chess renders its board help or inspection bar
directly below the board, before the lower player row (approximately y699 at
normal size and y547 at minimum). Shogi renders the equivalent text after the
lower player row, in the shared footer (approximately y773 and y615). Chess uses
that footer for Your turn/Live position instead. The difference also appears in
actual minimum-size inspection captures. Shared `GameArea` source alone did not
prevent this migration because Chess's `Board` still owns its embedded help.

Move the common help/inspection content into the same physical slot/order for
both games, while preserving standalone verification-board behavior where
needed. Recapture the affected paired normal/minimum/inspection states and
re-review. This finding was sent to the integrator; it is not resolved by passing
geometry assertions for other slots.

Windows and current normal macOS screenshot assessments remain pending. The Mac
lock remains a capture limitation. Owner visual approval remains absent; no
final visual pass is claimed for this refinement while this finding is open.

## Footer correction — actual Linux re-review at `44aeb47`

Product source `44aeb47022d57fff55142799bb6ba55980f4dc33`, Desktop CI
`40489563132`, Linux attempt 1. Independently inspected the unaltered artifacts
using `view_image` under:

`/private/tmp/msh74-shell2-ci/44aeb47022d57fff55142799bb6ba55980f4dc33/linux/gui-evidence/`

Compared the same four ordinary normal/minimum webview filenames listed above,
and actual minimum-size `normal-inspect-chess-e7.webview.png` against
`normal-shogi-inspect-shogi-7c.webview.png`. Associated DOM records confirm
760×650 DPR1 minimum viewports and explicit inspection focus labels.

**The help/status migration finding is resolved in these actual captures.**
Both games now place selection instructions or read-only inspection status after
the lower player row in the shared footer. The recorded driver measurement
places the help container top at y756 normally and y594 at minimum for both
games; visual inspection independently confirms that ordering and alignment.
Chess's native turn/position footnote remains below its help in that footer and
is visible rather than clipped. Shogi's two-line minimum inspection wording and
keyboard hint also fit. Both complete boards, player rows and Shogi hand areas
remain visible; matching header/document/Flip/sidebar card locations and the
single dark palette remain coherent. No further material visual finding was
identified in these Linux captures.

This is a visual pass for the inspected Linux states, not a whole functional-run
pass. The integrator reports this attempt finished 12/14: after the paired
captures, the normal Shogi phase encountered an upstream WebDriver connection
reset during a read-only element lookup, and the dependent reopen phase lacked
the save that had therefore not occurred. Earlier captures remain evidence of
their actual states; this review does not convert those later failures into
passes or independently adjudicate their cause. Windows and current normal Mac
review remain pending. Owner approval is still required.

## Actual Windows re-review at `44aeb47` — two findings open

Independently inspected unaltered Windows captures from the same product source
`44aeb47022d57fff55142799bb6ba55980f4dc33` under:

`/private/tmp/msh74-shell2-ci/44aeb47022d57fff55142799bb6ba55980f4dc33/windows/gui-evidence/`

Method: `view_image` comparison of the ordinary Chess/Shogi 1180×820 initial
pair, ordinary 760×650 minimum pair and actual minimum-size active-inspection
pair using the same filenames as the Linux re-review. Also compared:

- `normal-active-analysis.webview.png` with
  `normal-shogi-active-analysis.webview.png`.
- `normal-historical-analysis.webview.png` with
  `normal-shogi-reopened-historical-analysis.webview.png`.

The initial/minimum/inspection pairs support the corrected footer placement.
Complete boards, player/hand regions, instructions, inspection wording and
Chess's retained native footnote remain visible. Shared dark theme and common
header/document/Flip/card placement remain coherent. Windows UI font metrics and
Chess glyph artwork differ from Linux/macOS, without missing Shogi glyphs or
material clipping in these captures. Sidebar content can require its own scroll.

The actual active-analysis and history pairs expose two material common-action
placement inconsistencies not established by initial-state geometry checks:

1. **Cancel analysis moves outside Analysis in Chess.** Chess places the active
   cancellation control in the Play-paused notice above the Analysis card;
   Shogi places the same action beside Replace analysis inside that card. Native
   work limits or Chess's additional Full operation do not explain moving this
   shared action to a different application section.
2. **Return to live moves across the historical notice.** Chess puts it on the
   left below explanatory text; Shogi puts it at the right end of its banner.
   Different native event descriptions can explain text/height differences,
   but not an unrelated position for this common action.

Use consistent shared action locations for these operations, then recapture
the affected like-for-like states. Both findings were sent to the integrator.
The earlier footer finding remains resolved; these are additional observations
from the requested expanded actual-state comparison.

The integrator reports this Windows attempt finished 13/14 with a hand-invariance
assertion failure whose baseline lacked sufficient diagnostics. This visual
assessment neither independently explains that failure nor converts the run to
a pass. Current normal macOS screenshots remain unavailable while locked, and
owner visual approval is still required. The visual review remains open.

## Actual Linux re-review at `19beb74` — cancellation resolved, history visibility pending

Product source `19beb7437f11003734d65ee68f69a5f34bbdd02c`. Independently viewed
the normal/minimum/active-minimum-inspection pairs, active-analysis pair,
historical-analysis pair and reopened opponent-hand-inspection capture under:

`/private/tmp/msh74-shell2-ci/19beb7437f11003734d65ee68f69a5f34bbdd02c/linux/gui-evidence/`

The same filenames listed in the prior Linux/Windows sections identify these
states. The footer correction remains supported at normal and minimum sizes,
including actual active inspection. Single dark theme, ordinary common slots,
complete boards and readable native hands/inspection styling remain coherent.

**Cancel-analysis placement is resolved:** both active-analysis captures now
show Cancel analysis beside Replace analysis within Analysis. Different native
work limits, additional Chess Full controls and explanations remain content
within that section rather than moving the common action to a notice.

**Return-to-live visibility still requires confirmation.** Chess's historical
capture clearly shows the action at the right of the common notice heading.
In the Shogi historical capture, the action is present in associated DOM text
but could not be seen in the screenshot. The shared component/source alone is
insufficient proof that this action is visible. The integrator was asked to
confirm it through current actual Mac capture or native driver rectangle/style
evidence before this finding is closed; no speculative source change was
requested.

Also viewed `macos/chess-normal.jpg` (originally a provisional `.png` filename), the current normal Mac Chess capture
reported from this product revision. Its board, shared dark shell, player rows,
sidebar and footer are coherent, but a single game does not establish the
required current Mac paired comparison. Further Mac and Windows assessment is
pending.

The integrator reports this Linux attempt finished 13/14: all three normal
journeys passed, while a verification-claim phase hit `RemoteDisconnected` on a
read-only WebDriver execution. This visual review does not convert that failure
into an overall functional pass. Owner visual approval remains required.

### Original-detail history confirmation and current Mac pair

Reopened the exact `19beb74` Linux Shogi historical screenshot above using
`view_image` with `detail: "original"`. **Return to live is visibly present** in
the notice's upper-right, approximately x717–821/y191–216 in the 1180×820 image,
matching the Chess heading action. The earlier visibility concern was a reviewer
misreading and is corrected here without requiring a source change. Both
history-action and analysis-cancellation placement findings are now resolved in
the actual `19beb74` Linux captures.

Also inspected the current normal packaged Mac `macos/shogi-normal.jpg` at
original detail against `macos/chess-normal.jpg`. Both are 2360×1640 Retina
native-window captures reported from `19beb74` (1180×820 logical outer window).
Header identity/runtime/document/file controls, live heading/Flip, upper/lower
player rows, board region, sidebar card order/treatment and footer occupy the
same composition. Warm Shogi board and semantic pieces/hands remain readable
within the shared dark application theme. Both full boards and player regions
are visible; no unexplained common-control migration or material visual issue
was identified in this ordinary-state Mac pair. This does not assert a current
Mac minimum-size capture or replace the separate Linux minimum/inspection
evidence. Current Windows assessment remains pending; owner approval is absent.

## Final current-product Windows assessment at `19beb74`

Independently viewed the actual Windows captures under:

`/private/tmp/msh74-shell2-ci/19beb7437f11003734d65ee68f69a5f34bbdd02c/windows/gui-evidence/`

Product source `19beb7437f11003734d65ee68f69a5f34bbdd02c`. Using `view_image`,
compared ordinary Chess/Shogi normal and minimum pairs, actual minimum-size
opponent-inspection pair, active-analysis pair and historical-analysis pair with
the same filenames listed in the earlier assessments. DOM records corroborate
1180×820 and 760×650 DPR1 client dimensions. Inspected the actual native desktop
minimum capture as supporting evidence. The Windows desktop is 1024×768, so
large native-window desktop captures are cropped by that screen: complete
1180×820 appearance is assessed from the unaltered full WebView client captures,
not inferred from those cropped desktop images.

**All three recorded visual findings are resolved in the current actual
Windows states:** both games place help/inspection after the lower player,
Cancel analysis beside Replace inside Analysis, and Return to live at the
upper-right of the shared historical-notice heading. Header/document/file
controls, live heading/Flip, upper/lower players, board region, shared card
treatment/order and footer form a consistent composition. Native Chess Full
controls, descriptions and differing available observations remain content
within the common sections. Complete boards and readable inspection/footnote
text remain visible at minimum size. Sidebar content can require its own scroll.
Windows general UI typography and Chess glyph artwork differ from other native
platforms, without missing Shogi glyphs or a material layout issue in these
captures. No outstanding material visual finding was identified.

The integrator reports all fourteen Windows GUI phases passed; separate
platform/implementation audit owns that functional conclusion. This review
establishes actual captured appearance and does not infer functional correctness
from screenshots. The Linux bounded rerun remains a separate verification item.

Also inspected current Mac `shogi-historical-analysis.jpg`,
`shogi-opponent-inspection.jpg` and
[`shogi-node-limited-analysis.jpg`](macos/shogi-node-limited-analysis.jpg). Their theme,
history/inspection/result hierarchy and full board remain coherent. Despite its
earlier provisional filename `shogi-active-analysis.png`, the latter depicts an
already completed node-limited analysis result (Start move analysis, Analysis
result available, 20,000 nodes), not active work. The integrator renamed it to
`shogi-node-limited-analysis.png`, then corrected its provisional suffix to
`.jpg`, without changing the exact image bytes; package
and screenshot provenance/hashes are recorded separately. Actual active-state appearance is
established by the reviewed current Linux/Windows captures. No current Mac
minimum-size claim is made.

Subsequently inspected current normal Mac
[`shogi-opponent-hand.jpg`](macos/shogi-opponent-hand.jpg): both native counted
hands, stopped two-event history, selected opponent-hand dashed outline, hollow
drop-destination rings and explicit read-only footer remain visible and readable.
The shared composition and dark theme remain coherent; no material visual
finding. This is a normal application Open of a native capture archive, not a
claim of standard-start reachability. Mac fast submit/cancel attempts did not
establish successful cancellation and are not counted as such; separate current
Windows/Linux GUI evidence owns the successful cancellation claim. The retained
Mac zero-node observation remains `shogi-incomplete-analysis.jpg`. All current Mac
captures are CUA JPEG data; provisional `.png` suffixes were corrected to `.jpg`
without image alteration. [The retained-file index](macos/retained-files.json)
records exact hashes, byte lengths and 2360×1640 pixel dimensions.

**Final bounded visual verdict:** the reviewed current-product Mac normal pair
and Linux/Windows normal, minimum, inspection, analysis and history states
support the shared dark application composition, with all material findings
resolved. This is independent static visual assessment, not formal screen-reader
or WCAG conformance, complete keyboard interaction, rules correctness or owner
approval. MSH-74 remains In Review until the explicitly required owner approval;
MSH-75 remains unstarted.

## Final same-source Linux attempt 2 confirmation

Independently reopened the actual normal/minimum/active-minimum-inspection pairs
and active-analysis/historical-analysis pairs from:

`/private/tmp/msh74-shell2-ci/19beb7437f11003734d65ee68f69a5f34bbdd02c/attempt2/linux/gui-evidence/`

Product source remains `19beb7437f11003734d65ee68f69a5f34bbdd02c`; filenames
match the prior pair lists. Used `view_image`, including original detail for the
common historical actions. The corrected footer, Replace/Cancel row inside
Analysis and right-aligned Return to live are visibly retained. Both complete
boards, player/hand regions, readable markers and wrapped footer text fit at
minimum size. Shared header/document/Flip/card geometry and single dark palette
remain consistent. No new or outstanding material visual finding was identified.

The integrator reports this same-source Linux attempt passed all fourteen GUI
phases and the overall Desktop workflow. This supersedes the earlier failed
functional attempt without erasing its recorded failure; this review's own
conclusion remains bounded to actual captured appearance. Current Mac native
Save/Open and quit/process-absence evidence is recorded separately by the
integrator, not inferred from these screenshots. No current Mac minimum-size or
successful rapid-cancellation claim is added. Explicit owner visual approval
remains the final MSH-74 gate.
