# MSH-120 / R5 — one native-choice interaction owner

Implementation checkpoint against `codex/rewrite`
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
