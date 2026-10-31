# MSH-120 / R5 — one native-choice interaction owner

Final current-boundary verdict: **PASS**. All seven R5 acceptance criteria have
supporting evidence; independent review has no unresolved material finding.
Production source is `7ebab4648ade6d6be6da6220aefc0948558a23a8`; final fixture
source is `39ab0c1b0a145a97c3bd980a2b216ba743cb8f0c`. MSH-120 may close;
MSH-123/124/125/121 remain unstarted and M4 remains open. This verdict reuses
unchanged native evidence and explicitly retains the known MSH-112 CI failure.

Historical implementation checkpoint against `codex/rewrite`
`6509dd3ea7db60a10f95e5fc50768668596f39f3`. This is not yet a final acceptance
verdict: actual current packages, independent completed-boundary review and CI
must be recorded before Done. MSH-123/124/125/121 remain unstarted.

## Surviving boundary and deletion

`useNativeChoiceController` owns selection, ordered prefixes/candidates,
terminal-versus-continuing routes, complete alternatives, cancellation, pending
inspection and generation retirement, exactly-one submission, roving focus,
hand navigation, completion focus/trapping and recovery. It never parses action
strings, computes movement/effects or concludes a game. `boardPresentation`
contains native display identities; renderers supply opaque origins/complete
choices, board keys and geometry-neighbor mapping.

Actual consumers are Chess/synthetic `Board`, `ShogiBoard` and `XiangqiBoard`.
The generic `NativeBoard` presentation adapter and separate synthetic native
verification application also reach that same `Board` controller. All three
renderer selection/prefix/query-generation/inspection/completion/focus state
machines and independent key/cancel/submit handlers are deleted. The old
BoardChoice transport definition becomes the current shared NativeChoice type,
with all actual imports migrated. No obsolete compatibility facade is retained.

Shogi keeps kanji/pentagons/rotated Gote, hands and native completion markup;
Xiangqi keeps intersections/river/palaces/discs and native labels; Chess keeps
its square artwork and promotion presentation. Common GameFrame/application
composition is unchanged. Renderers own no competing session/document controller.
The development-only `>9` scrolling/min-width/scrollIntoView heuristics and
narration are removed: the synthetic16 board must actually fit, not be scrolled
into a passing screenshot. No universal visual renderer or variant DSL is added.

## Admission, inspection and focus

Only supplied complete choices are candidates. Ordered steps distinguish routes
with identical endpoints; a terminal prefix with continuations requires explicit
completion or another step. A single complete native action, including forced
promotion, submits directly; alternatives require the supplied chooser. A
synchronous submission latch and current-selection identity reject duplicate
and retired handlers even before the parent receives a new snapshot.

The semantic context includes the exact supplied native identity, orientation,
capabilities, origin registry and choices. Identical snapshot polling preserves
selection; a changed context immediately hides it and retires requests/handlers.
`useMovementInspection` is now a stable, stateless native query/identity validator
for game/document/version/revision/selection, origin and read_only. It has no
second mounted/request generation. The common controller alone owns pending
queries, supersession, cancellation and unmount retirement. Rejected inspection
never supplies an executable action. Visible inspection wording and hollow
markers remain distinct from actionable targets and focus.

One inconsistent old interaction corner is deliberately reconciled: a marked
read-only inspection destination is traversed before treating the same friendly
piece as a new actionable origin. Chess's synthetic e7→e2 test formerly switched
modes; actual native Xiangqi b7-cannon→b0-horse inspection formerly previewed.
The common route-first policy previews without action, Escape returns to the
inspection origin, and friendly origins outside the current route switch to
native legal selection. Legal occupied captures still precede enemy inspection.
The independent reviewer checked the approved inspection contracts and accepts
this as routine consistent consolidation, not a game-rules change. The changed
Chess assertion is explicit; the real Xiangqi oracle remains unchanged.

Focus is recovered from retired/removed completion or hand controls without
stealing an unrelated application input/dialog. Specialized renderers supply
geometry; one controller owns board/hand arrows, Enter/Space, Escape and chooser
Tab trapping. Chess no longer remounts its board on every selected-prefix token:
the controller retires selection and preserves appropriate focus itself.

## MSH-123 handoff

`CompletedActionGeometry` is a separate readonly origin category plus ordered
steps, accepted through GameBoard/renderer props. It is **not** controller input,
pending selection, inspection, focus or a remembered destination. R5 does not
derive geometry from history or draw last-move cues. MSH-123 supplies exact native
selected-prefix geometry and owns origin/destination emphasis, destination-only
drops, complete compound paths and thinking/history persistence. Its Moves,
notation and event mapping work is untouched. MSH-124/125's layout/workflow
changes and R6 final acceptance are likewise not implemented here.

## Verification checkpoint

Strict TypeScript and252 frontend checks pass, including20 common-controller
checks plus all specialized native-generated board/route/promotion/drop cases.
Eight focused native inspection tests pass (`cargo test --locked -p
mshogi-desktop-session inspection`), covering strict queries, exact history,
warning state, read-only native actions and Take back retirement. Native rules,
engine, archive, limits and format are unchanged; applicable R1–R4 evidence is
reused rather than claiming a new rules/strength campaign.

Initial failures are retained accurately: generic NativeBoard's old compulsory
promotion test expected a redundant single-choice dialog; it now verifies the
same sole native5b5a+ action directly, as existing product Shogi already did.
Two Xiangqi inspection tests exposed the priority divergence above and now pass
with their original expected native behavior. One application-focus assertion
required DOM remount identity, replaced by retained focus and retired-selection
behavior. No legal-choice or game-result expectation changed.

The separate external GUI verifier now checks full256-position visibility at
normal/minimum sizes and both orientations, actual keyboard/pointer cancellation,
explicit terminal-prefix native136>153 and distinct136>153>136/136>135>136 routes.
Its69 focused boundary tests pass. Unit/driver checks are not actual package
evidence; identified current package journeys and final independent/CI results
remain required and will be appended below.

## Exact revisions, actual package interaction and failure reconciliation

The shared production boundary is `7ebab4648ade6d6be6da6220aefc0948558a23a8`.
The follow-up `39ab0c1b0a145a97c3bd980a2b216ba743cb8f0c` changes only the
separate verification fixture stylesheet. Normal production assets, controller,
native rules/session/engine/archive and limits are identical between them.
Both revisions were integrated and normally pushed to `codex/rewrite`.

Actual local macOS27.0.1 arm64 interaction uses the packaged native WKWebView,
CUA accessibility/pointer/keyboard input, and real bundled engine. It covers
Chess e2e4/a7a5, Shogi7g7f/4c4d and Xiangqi a3a4/b7b0, all with real engine
observations and destination focus after reply; enemy-piece inspection without
commit, Escape and orientation retirement; Shogi historical event1/Return to
live; Chess Take back/Resume/native Save; native unsaved replacement and Quit.
The separate QA package covers optional promotion cancellation/choice, native
hand inspection, hand Escape focus and P*5e with hand removal, plus synthetic
keyboard/pointer cancellation, explicit terminal136>153 and exact distinct
136>153>136/136>135>136 native records. Every launched local QA process exited0
through its native close guard. No real variant support is claimed by synthetic.

[Local interaction/package identities](evidence/msh120/macos/interaction.json)
include full package file hashes, exact source-content revisions, screenshots
and their provenance. Product comparison captures use1180×820 and760×650 logical
content. Shogi minimum is the hand fixture after play, while normal Shogi is an
initial real game; these are not presented as an identical-position comparison.
QA titles retain the launch fixture after native New switches games. Earlier
capture-helper mistakes targeting a retired normal process were excluded and
replaced with explicit-target captures. Screenshots are unedited native output.

Initial Desktop CI41189536158 at7ebab exposed a real synthetic layout issue:
Linux14/15 phases passed, but after Complete action here the stacked diagnostic
page stayed scrolled416px, clipping six ranks. Its initial normal/minimum and
partial-route checks had passed. The [failed audit](evidence/msh120/initial-linux-audit.json)
remains historical evidence, not a passing campaign. The39ab fixture-only fix
bounds the root to the viewport, allocates remaining height to the board and
route controls, and gives diagnostics their own scrolling region. No board
scrolling, zooming, programmatic scroll or weakened probe is used. Twenty-three
focused controller/fixture tests pass after this correction. Actual macOS
normal/minimum, both orientations, partial controls, accepted action and resize
captures now show the complete16×16 board; the independent reviewer inspected
all six corrected captures and found no remaining visual defect.

## Independent assessment method

A separate bounded reviewer agent (`/root/r4_independent`), with no implementation
ownership, examined the completed baseline-to-source diff, native-choice ownership,
deletions, rules/inspection semantics, context retirement, synchronous submission
latch, focus, renderer boundaries and MSH-123 separation. It independently ran95
frontend checks across eight relevant files and56 GUI-driver boundary checks;
all passed. The complete driver set run by the builder has69 checks. The reviewer
explicitly assessed the changed old assertions described above rather than
calling implementation-mirroring replacements independent conformance evidence.
It also directly inspected six same-size product screenshots and six corrected
synthetic screenshots, finding no unexplained movement of common controls or
board clipping. This is agent review, not a new owner visual approval or a human
accessibility certification.

At this checkpoint the final platform workflow/artifact audit was pending;
the final accepted results are recorded below.
The original planning baseline6509 native Linux run41187557967 failed with the
existing MSH-112 missing-response/Brokenpipe versus UnexpectedExit subprocess
race; Windows/macOS passed. No runner source is changed by R5, and that failure
is not erased or represented as fixed.

## Acceptance mapping and bounded handoff

| Criterion | Supporting boundary/evidence |
| --- | --- |
| AC1 one actual interaction owner | All three renderers call `useNativeChoiceController`; native/synthetic adapter reaches the same `Board`; old renderer state machines and query lifecycle owner deleted; independent source review and20 controller tests. |
| AC2 specialized content/common shell | Chess square art, Shogi hands/kanji/rotation/promotion views, Xiangqi intersection/river/palace art and native labels stay in renderers; shared R4 composition unchanged; independent normal/min visual comparison. |
| AC3 complete actions/routes | Native-generated promotion/forced/decline/drop/capture tests; ordered routes and terminal alternatives; native actual QA promotion/drop; exact synthetic136>153 and two equal-endpoint routes; no partial commits or action decoding. |
| AC4 inspection/context/focus | Stateless authoritative query validator plus one generation/lifecycle owner; document/history/orientation/capability/unmount/late-result cases, native8 inspection tests; actual three-game inspection, Escape, history, hand/board/completion focus. |
| AC5 deletion/full-board visibility | Old imports/controllers and >9 scrolling heuristics removed; unchanged specialized game geometry; corrected verification-only remaining-height layout; all256 positions and SVG checked without scroll/zoom at both sizes/orientations/partial/accepted states. |
| AC6 meaningful tests/actual packages |252 frontend checks,69 driver checks, independent95/56 subset; actual identified normal macOS three-game and separate QA journeys; identified Linux/Windows campaigns and normal instrumentation scans are required in final platform verdict. |
| AC7 review/CI/integration | Exact production7ebab and fixture39ab; independent source/visual/behavior review; native all3PASS, current package outcome below; original failures retained and unchanged historical evidence reused. |

[Native CI audit](evidence/msh120/native-ci-audit.json) records attempt1 of
Rust41189535914 at7ebab: Linux, macOS and Windows all pass. Each platform's
locked workspace and desktop-verification suites include168 default and180
all-features desktop-session tests plus fixture-gate checks. The previously
flaky MSH-112 runner case passes here; this is verification, not a race fix.

MSH-123 becomes dependency-ready when R5 closes;124 and125 are already
independently ready. Suggested integration order remains123,124,125,121.
R6/121 retains all recorded dependencies and independent final architecture/
three-platform product acceptance. No variant, M5 strength work or subsequent
migration/product task is started. M4 remains open. Existing MSH-112 evidence
and the local Tao patch provenance/native termination regression remain intact.

Launch the normal current-source application from the primary checkout:

```sh
open "desktop/src-tauri/target/release/bundle/macos/mShogi.app"
```

To rebuild with the recorded Node24 runtime available:

```sh
cd desktop
npm ci
npm run desktop:build -- --bundles app
```

The separate `mShogi Verification.app` and `desktop-verification` build feature
are test instrumentation; normal packages exclude them. Native board/actions,
profiles, canonical history, archive format/admission and measured engine limits
are unchanged by R5. The new123 last-move/Moves UI is intentionally future work.


Additional initial-platform evidence:7ebab Windows also failed only the
synthetic whole-SVG minimum-size probe (SVG bottom652.78 exceeded650 logical
pixels, although all256 cell elements were visible). The required whole-SVG
check was retained. The39ab bounded layout addresses this as well as the Linux
post-completion page scrolling. These are failed initial campaigns, not new
Windows/Linux passing GUI claims.

Rust41190406750 at39ab initially reproduces MSH-112 on Linux: the unchanged
runner test at runner.rs172 receives MissingResponse/Brokenpipe instead of
UnexpectedExit (nine other runner tests pass). The earlier7ebab all-platform
native pass remains valid for identical native source. Independent review confirms that the exact native/Rust/runner/core/engine,
fixtures and toolchain inputs are unchanged from the all-three-platform7ebab
pass; the39ab diff is exclusively a verification stylesheet. That existing
native evidence is reused, as the user requested. The initially proposed
failed-job retry was not issued: repeating an unchanged campaign until the
known race happens to pass would not add R5 evidence. MSH-112 stays open and
no all-green39ab Rust workflow or runner fix is claimed. Final39ab Desktop
package/GUI checks remain required gates.


Current39ab Desktop41190406764 Linux and macOS jobs pass. The independent
[Linux artifact audit](evidence/msh120/linux-audit.json) verifies all327 report
file hashes and13 normal package files, real host/engine identities,15/15 actual
GUI phases with native cleanup, normal instrumentation exclusion, seven strict
whole-board observations and exact native synthetic action arrays. Twelve
independently inspected product/synthetic captures are retained in
[evidence/msh120/linux](evidence/msh120/linux). No package construction is
substituted for those actual installed application interactions.

The [macOS CI audit](evidence/msh120/macos-ci-audit.json) verifies the normal
package, all five manifest file hashes, absence of16 verification markers and
bundled-engine smoke. The hosted normal host and engine are byte-identical
between7ebab and39ab, consistent with the verification-only stylesheet diff.
macOS GUI evidence is the separately identified local native CUA campaign above;
no hosted macOS GUI campaign is claimed.


## Final platform and independent verdict

[Desktop41190406764](https://github.com/jweathers777/mShogi/actions/runs/41190406764)
at39ab passes Linux, macOS and Windows. The final
[Windows audit](evidence/msh120/windows-audit.json) verifies all288 report-file
hashes, both normal manifest files, separate QA installer identity,15/15 actual
installed GUI phases and native exit cleanup. Twelve independently inspected
product/synthetic captures are retained in [the Windows evidence](evidence/msh120/windows).
Linux and Windows both exercise real Chess, Shogi and Xiangqi play, keyboard/
pointer inspection, orientation, analysis/cancellation, Save/Open/history/Resume,
Take back, recovery, native guards, promotion/drops/claims and labeled synthetic
routes. These are actual application interactions, distinct from engine smoke.
All seven strict synthetic full-board probes pass, including the formerly
failing post-terminal acceptance state. Exact native records distinguish the
terminal action, capture-return route and alternative equal-endpoint route.
Normal distributables exclude verification instrumentation.

[Independent final review](evidence/msh120/independent-review.json) records PASS
for source ownership/deletion, complete native actions, stale/read-only/focus
semantics, specialized rendering, actual interaction, visual fitting, evidence
provenance and the123 handoff. Material initial fixture findings are resolved by
39ab and actual corrected platform checks; original failed probes remain above.
No required R5 finding remains open.

Applicable native evidence is
[Rust41189535914](https://github.com/jweathers777/mShogi/actions/runs/41189535914)
at7ebab, all three platforms PASS. The extra automatically triggered
[Rust41190406750](https://github.com/jweathers777/mShogi/actions/runs/41190406750)
at39ab completes with macOS/Windows PASS and Linux FAIL on the unchanged
MSH-112 runner race. It was not retried after independent proportionality review:
all relevant native source/fixtures/toolchains are identical to the preceding
all-platform pass. This is an explicit remaining CI limitation, not an R5 fix,
a hidden failure or a claim that every39ab workflow is green.

Final bookkeeping checks pass: pinned Backlog1.55.1 doctor; evidence JSON and
retained screenshot checksum verification; source/deletion consistency; and
`git diff --check`. No further correctness/build/GUI campaign is required for
the documentation/task-only completion commit. Integration is on `codex/rewrite`
with normal pushes; no temporary worktree was created for R5 and unrelated
worktrees are preserved.
