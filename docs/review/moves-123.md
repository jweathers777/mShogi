# MSH-123: Moves notation, exact positions and completed-action cues

Implementation baseline `0b4218deaa8dbe705b882169d9e4cdab33333b0e`, primary
`codex/rewrite`. Root Codex is builder/integrator. This is the bounded product
follow-up to R5, not MSH-124/125/R6. Final verdict: **PASS — all seven MSH-123 criteria are supported** by the
independent reviews and source/package evidence below. Unit tests alone are not
used as packaged interaction evidence.

## Native ownership and display contract

`mshogi-desktop-session::moves` builds a native projection over admitted actions,
positions and exact events. A transient document/version cache avoids repeated
notation reconstruction during polling. Selection chooses the latest genuine
action at that exact prefix. No canonical record, archive, profile, engine work
limit, stale-result admission or capacity policy changes. The only core addition
is immutable Xiangqi command-prefix range access; it performs no replay.

The projection supplies native pair number/column, standard notation, opaque
complete action, exact post-action event count and separate command start/end.
The shared Moves table consumes those facts. Imported counters never fabricate
moves: Chess uses its fullmove counter, Shogi shows known relative pairs with
its declared SFEN counter and unknown earlier history, and Xiangqi preserves
its supplied/unknown fullmove provenance. Second-player roots leave the first
cell empty. Starting position is an explicit native target.

Non-board commands and derived events stay in Position events. They retain
independent exact selection and associated move annotations, without becoming
move rows. In WXF F022, action-prefix8 and warning-prefix9 have the same board
but different history. The move selects8; native analysis there is explicitly
unsupported because it is inside an atomic command. Committed9 analysis retains
the complete warning history. Neither frontend nor projection normalizes8 to9.

Selected-prefix geometry carries native board/intersection/hand origin, ordered
steps, complete identity and secondary effects. Chess castling includes native
rook movement; drops have only a destination; synthetic routes retain repeated
endpoints and order. Renderers are read-only decoration. Destination corners
plus double underline differ from small dashed origin corners, legal targets,
partial routes, hollow inspection and keyboard focus. The single R5 controller,
query helper and R4 application composition remain unchanged.

## Sources, expectations and independent review

[Independent source research](evidence/msh123/notation-sources.md) directly read
PGN SAN and the official JSA guide; selected English/Chinese WXF PDFs were
hash-verified and visually inspected. SAN uses legal alternatives and native
post-action check/mate. JSA uses numeric file/rank, Japanese promoted identities,
known-history 同, contextual 打, 成/不成 and mover-relative ambiguity, preserving
the explicit no-直 exception for 竜/馬. WXF uses K/A/E/H/R/C/P, mover files,
advance/retreat/traverse, distance versus destination file and front/rear.

The owner explicitly approved ordinal+file for four/five soldiers and ambiguous
multiple paired files. This source-consistent display extension is documented
in [the product convention](../desktop/post-r4-ux.md#msh-123-selected-notation-completion-2026-10-31),
not misrepresented as literal WXF text. Independent FigureB expectations remain
`36=5`, `26=5`, `16=5`. A candidate rear-soldier fixture before the river was
corrected to a legal crossed-river position; native legality assertions were
preserved rather than changing rules to match a candidate expectation.

[Native independent review](evidence/msh123/native-independent-review.md) by a
separate source researcher/reviewer passed, including independently executed
18 filtered tests (16 new notation/projection and two existing persistence
matches). Another bounded worker added three actual session-consumer tests for
Save/Open/Take back/retired work and Xiangqi exact warning prefixes. The separate
frontend reviewer independently passed106 tests/eight files and strict TS,
found the Shogi capture/completed-corner visual ambiguity, and reviewed the
actual double-underline correction. The final packaged visual review is recorded below.

## Verification log and limits

Initial native-consumer test assumptions about Chess MatchStart index,
manual-engine cancellation and Shogi selection token changes failed. They were
corrected against the actual accepted contracts; the three consumer tests then
passed. Clippy found an eight-argument helper; removing its duplicated event
index makes entry identity derive directly from native geometry, and lint then
passed. These were verification findings, not silently hidden passes.

Commands and final source/package revisions, full native/frontend totals,
actual GUI evidence and CI disposition will be recorded below after execution.
No new Windows/Linux/macOS GUI success is claimed by the source checks above.

Pre-package checkpoint: full native desktop-verification suite198/199 passed;
new Chess cancellation test asserted one race winner incorrectly. Chess really
submits its missing-engine process despite the Shogi/Xiangqi manual fixture.
The corrected test accepts only successful cancellation or the exact already-
stopped rejection with retired execution, and asserts unchanged complete native
context, Moves and cue in both paths. Three focused feature-enabled runs passed
all3 consumer checks each. Earlier failed assertions remain recorded; no product
behavior was changed to satisfy them. Frontend264/23, strictTS, native feature
clippy and formatting pass. External GUI-driver75 unit checks pass. Actual
macOS normal package built and instrumentation-exclusion/real bundled-engine
smoke passed; native CUA Chess play/history/flip/Take back observed. Further
packaged GUI and CI checks remain pending.

## Actual packaged macOS interaction

[Package identities](evidence/msh123/macos-packages.json) pin normal host
`3ba1d8a55d88eb802a7fcd8279b565480cf0685e93cfb204d26e21a5532c7286`
and bundled engine
`92592cb2a4f5ed57ed1b62498b7d288e2a4bc01cb4f0f87827eedf762f9ca5d7`.
Normal and separately labeled QA packages use production source
`fc7a6ae9e7499c40999c5a0396bc31e87d4e07e4`; later65975d8 changes only external
verification driver, task and evidence, not shipped code/assets. The normal
build's feature exclusion and real engine smoke pass independently of GUI.

[Native CUA interaction record](evidence/msh123/macos-interactions.json) and
[30 retained captures](evidence/msh123/macos) establish actual macOS27.0.1 arm64
play, standard Moves, pointer history, orientation and native Take back. Separate
QA native sizing menu measures760×650; normal content is1180×820. All three
boards remain fully visible, with sidebar scrolling allowed. Actual Tab/Enter
selects Shogi root0 and move1 with focus retained. Shogi's legal capture target
at4d and completed4c→4d destination were compared at minimum size: two underline
strokes remain distinguishable. The separately labeled hand fixture commits
P*5e with destination-only cue; temporarily held QA engine produces a genuine
unscored startup failure, and restoring it/Retry yields a real reply. The normal
engine was never moved or replaced.

Separate synthetic captures show partial i9→j10 with accepted count0, then
native `136>153>136` and `136>135>136` actions with distinct ordered numbered
return cues. Exact record strings were read from native AX; opening the record
disclosure does not make its below-viewport JSON visible in the screenshot.
Normal packages contain no fixture or window-sizing controls.

Three expressly named thinking captures use the **existing QA-only** external
engine-path hook. A hash-recorded wrapper sleeps2 seconds then execs the real
bundled engine verbatim; it produces no responses, observations or telemetry.
All three actual native Searching/one-ply frames preserve the human-action cue
and show no completed evaluation; subsequent genuine replies complete normally.
This establishes pending-work presentation, not engine speed or playing strength.

[Independent frontend/visual review](evidence/msh123/frontend-visual-independent-review.md)
personally inspected all30 captures and independently reran106 tests atfc7a6ae.
PASS, initial Shogi cue ambiguity resolved; no remaining material visual finding.

## CI and initial verifier finding

[Rust41199352911](https://github.com/jweathers777/mShogi/actions/runs/41199352911)
atfc7a6ae passes Ubuntu, macOS and Windows. This includes the corrected native
consumer test and applicable existing workspace/desktop regressions, distinct
from packaged GUI evidence. No unresolved native failure is hidden behind the
initial198/199 local run.

[Desktop41199352841](https://github.com/jweathers777/mShogi/actions/runs/41199352841)
atfc7a6ae passes macOS package checks. Its Linux GUI campaign fails a **new
verifier readiness assumption**: exact selection/orientation may render while
native capacity admission still disables controls. The driver attempted Tab
navigation before `.moves-root` became enabled. All three normal phases hit
that assertion; reopened-Shogi subsequently lacked its prerequisite saved file.
[The initial Linux failure record](evidence/msh123/initial-linux-failure.json)
retains those facts and11 separately labeled passing fixture phases. Windows
also reports failure in this original campaign:14/15 phases pass, including
normal Chess/Shogi/reopen, while Xiangqi hits the same disabled-root race.
Neither is counted as an overall GUI pass.

Verification-only65975d8 waits for exact selected prefix, changed/restored native
orientation, enabled controls and completed admission before the next real input.
It never replays a mutation or injects JS input/focus/scroll.78 driver checks
pass, including the concrete disabled-control regression. Corrected packaged
CI41200359930 subsequently passed; its final evidence and independent audit are
recorded below.


The [independent initial platform audit](evidence/msh123/initial-platform-audit.json)
checks all5 macOS manifest files and239 Linux GUI evidence files, real native
exit for11 passing fixture phases, and normal feature exclusion. Initial
Windows evidence has14/15 passing phases: normal Chess, Shogi and reopen pass;
Xiangqi hits the same disabled-root verifier race. Failed phases establish no
successful native graceful close and are not described as full-platform passes.

## Acceptance coverage

| Criterion | Concrete evidence |
| --- | --- |
| AC1 Moves/pairs/root | Native imported/second-side/empty numbering fixtures; shared Moves regressions; actual all3 pointer and Shogi keyboard root/move interaction. |
| AC2 sourced notation |19 new native notation/projection/consumer cases, independently reasoned expected strings; PGN/JSA and hash-verified bilingual WXF research; explicit owner-approved rare-stack extension. |
| AC3 exact history | Native F022 action8/warning9, context/analysis admission tests; frontend move/event detail tests; native command boundaries, not inferred ply; separate terminal-event fixture interaction. |
| AC4 native cues | Native selected-prefix/root/nonmove/castling/drop/route geometry and immutable cache tests; actualall3 normal/min/history/flip/thinking and synthetic distinct-return routes. |
| AC5 interaction distinction | R5 controller/query and R4 shell ownership unchanged; readonly decoration, no handlers;106 independently rerun frontend checks; minimum Shogi target/completed shape comparison, actualTakebackroot/reopen regressions. |
| AC6 verification | Frontend264/23+strictTS; featureclippy; correctednativeconsumer3×3; nativeall3CI; actual30macOScaptures; normal-package exclusion; correctedLinux/Windows full15-phaseGUI campaigns and all3native/desktopCI pass at65975d8. |
| AC7 independent/integration | Separate primary-source/native and frontend/30-image visual reviews PASS, concrete findings resolved; normalintegration/push checkpointsfc7a6ae/65975d8; original failures preserved and finalplatform evidence independently audited below. |

No MSH-124 sidebar/branding, MSH-125 lifecycle simplification, R6 integration,
future variant or M5 implementation is included. Those tasks remain open.


Compatibility: Moves and completed-action geometry are runtime presentation,
never added to native records or desktop archives. Existing archives reconstruct
the projection from their admitted native history on reopen; saved observations,
canonical schemas and archive versions stay unchanged. Existing record/parser,
execution, take-back and capacity regressions remain applicable and pass in the
native CI. R5 controller/query and R4 application composition files have no diff
from the task baseline; only readonly renderer props and the existing history
section's contents change.


## Final integrated platform verification

[Desktop41200359930](https://github.com/jweathers777/mShogi/actions/runs/41200359930)
and [Rust41200359910](https://github.com/jweathers777/mShogi/actions/runs/41200359910)
pass all three platforms at exact
`65975d8ea4f232883ceeb559be9b3cdb677c6e47`. The corrected Linux and Windows
reports each pass **15/15 actual installed/packaged GUI phases**, including all
three games' normal/minimum Moves pointer/keyboard/history/flip/root/live/Take
back journeys and separate native drop/compound fixtures. These are actual
application interactions, distinct from package/engine smoke and native tests.
Native process identities, package manifests, event observations, captures and
successful native shutdown are preserved by the workflow artifacts. Normal
feature exclusion remains verified. macOS GUI is the separately identified
local CUA evidence; hosted macOS establishes package/native checks, not GUI.

The macOS normal package's five manifest files are byte-identical betweenfc7a6ae
and65975d8. Shipped Rust/TS/Tauri/assets/locks have no diff across those revisions;
65975d8 only corrects the external verifier and records evidence. Original
Linux239-file and Windows295-file failing campaigns remain historical evidence,
with their limited passes and failed shutdown qualifications preserved. The
corrected campaign is a new identified execution, not a relabeling of them.

Launch the verified normal macOS build from the primary checkout with:

```sh
open /Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app
```

For a fresh build, from `desktop/` using its pinned Node24 runtime, run
`npm ci` then `npm run desktop:build -- --bundles app`. Verification wrappers,
fixture environment and QA package are not needed for normal play.

Remaining scope is MSH-124 shell/branding and MSH-125 document/lifecycle UX, then
MSH-121/R6 integrated acceptance. This task changes neither rules nor strength;
M4 stays open. No additional production finding is deferred from123 to those
owners. Final independent platform audit and bookkeeping evidence follow. No required
MSH-123 finding remains open.


Final cross-platform visual qualification: at760×650 the lower edge of the gold
keyboard-focus outline clips at the scrollable sidebar viewport in Windows all
three games and Linux Xiangqi. Its top/sides, move label, current-position state
and actual keyboard activation remain clear; independent review finds no
material123 acceptance failure. This is recorded as optional spacing/reveal
polish in the existing MSH-124 shell task, without starting it or changing its
dependencies/criteria. Do not describe these captures as fully unclipped focus
rings. No complete board is clipped and no native target becomes unreachable.


[Final independent platform/acceptance audit](evidence/msh123/final-platform-audit.json)
confirms all7 criteria supported: Linux381/381 GUI evidence hashes and13/13
normal manifest hashes; Windows342/342 evidence hashes and2/2 normal manifest
hashes; both QA installer identities;15/15 native shutdowns per platform before
teardown; exact complete cues, root/prefix/action identity, both supported sizes,
real Tab/Enter/flip/Returnlive/Take back, distinct synthetic routes and native
drop. Linux extracted normal/QA hosts match launched hashes; normal markers are
absent and QA markers present. Windows normal exclusion is the hash-matched
package-smoke result, not an invented independent scan of an unavailable
installed binary. macOS's five normal files remain byte-identical acrossfc/659.

[Final independent platform visual review](evidence/msh123/final-platform-visual.md)
personally inspected12 retained artifact captures, in addition to30 macOS
captures: **42 actual package screenshots** total. It confirms complete boards,
legible native notation, exact historical cues and operational keyboard focus,
with the nonmaterial minimum-sidebar outline-edge qualification above. The
[unchanged capture manifest](evidence/msh123/platform-capture-manifest.json)
provides exact source paths and hashes. Both independent reviewers identify no
remaining material123 requirement gap.

Final bookkeeping: pinned Backlog1.55.1 doctor and `git diff --check` pass;
retained capture/JSON references and checksums validate. Finalization changes
only task/design/review/evidence, with no additional product change or need to
repeat the successful65975d8 campaigns. MSH-123's seven acceptance criteria and
four completion items are supported and checked. Normal commit/push integration
uses `codex/rewrite`; no task worktree was created, unrelated worktrees remain,
and all owned verification hosts/engines/wrappers have exited. MSH-124's optional
spacing note does not activate it. Stop after123; M4 remains open.
