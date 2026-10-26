# MSH-74 independent implementation review

2026-10-26; fresh delegated reviewer `msh74_implementation_review`, reviewing
baseline `570ef0e` through the working MSH-74 boundary. Method: source/diff review,
trace normal ProductApp → hook → strict Tauri entry points → native controller,
file admission/archive and observation projection; run focused tests. This is
separate from visual review and packaged platform interaction.

Material findings resolved before the review checkpoint:

- Effect-local teardown retires StrictMode's first polling/listener promises.
- A delayed clean Save receipt cannot authorize discarding newer dirty state;
  the caller receives the latest accepted same-document snapshot, or no authority.
- Confirmation dialogs make the surrounding product inert and check the complete
  native document/version/revision/selection token.
- Selecting a hand piece preserves a board Tab entry; arrow navigation skips
  hand pieces without native choices.
- Generic cancelled work says Search stopped; the UI does not infer that an
  engine-turn cancellation was analysis from turn or paused state.
- Dialog focus capture precedes inert application and restores after removal.

Independent execution: product board/app/hook tests 22 passed at initial review;
updated hook suite 7 passed, including deferred StrictMode teardown; native
session `product_` tests 5 passed; desktop host tests 7 passed with desktop Rust
1.90; diff whitespace check passed. Initial host invocation from repository root
failed because Rust1.89 is below the host requirement; rerun from
`desktop/src-tauri` correctly selected the recorded1.90 toolchain and passed.

Reviewer reports no unresolved material implementation finding in these reviewed
boundaries. This does not establish owner visual approval, GUI interaction or
Windows/Linux success. Those are separate evidence gates.

Follow-up independent review of the delta against `41a3009`: disappearance of
Resume/Cancel/Retry/Return to live now restores focus to the native status after
snapshot commit, and Dialog explicitly prioritizes Cancel. The Ubuntu CI run
found a racy test precondition: public `command_with_receipt` can legitimately
accept its worker completion while constructing its returned snapshot. The test
now queues the same real immutable candidate without polling and explicitly
asserts its receipt is pending before replacement. It still requires retirement
with an error and rejects resurrection through later legacy/shared polls. No
product rule or admission behavior changed. Reviewer independently reran this
native test (passed), ShogiApp/ProductApp17 tests (passed), and diff check (passed).

Final independent boundary audit: the cached-game close listener could miss the
first close request immediately after a native game replacement but before the
next shared poll. The persistent ProductApp now owns native close for both games,
fetching fresh native metadata before confirmation. Embedded Chess suppresses
only its duplicate legacy close guard; standalone Chess preserves the previous
default and its Open/New Game protection is retained. Native close_requested
blocks further game replacement/actions, permits identity-checked Save, and
finishClose rechecks the current document revision under the controller lock.
Reviewer independently traced both transition directions and executed focused
App/ProductApp/useGameSession tests **24 passed**, strict TypeScript and diff
check passed. No material finding. These tests do not substitute for actual
packaged native quit evidence. Complete frontend suite **122 passed** locally.


Final modal follow-up, independently reviewed against b26fd73: native quit
cancels an uncommitted New Game chooser before opening the close guard, avoiding
two active root dialogs. No game change is submitted. Reviewer independently
ran ProductApp **11 tests passed** and diff check passed; complete root frontend
suite **123 passed**. No material finding remains.

Actual Windows21bfe3e GUI exposed a native cross-game Open race (13/14 phases
passed; fresh-process Shogi reopen failed). A shared poll could enter the Chess
branch, commit Shogi Open inside the legacy snapshot poll, then publish a Chess
projection with the newly advanced Shogi token. The correct next Shogi view had
an equal token and was correctly rejected by frontend freshness. Independent
platform/source review corroborated this mechanism. Both native read paths now
recheck the active family after file polling; the legacy path also returns before
polling/submitting retired Chess work. Strict frontend freshness is unchanged.

The new deterministic test uses a real saved native Shogi archive and real file
worker, waits for its completed slot without polling, then forces shared-first
and legacy-first commit. It checks the first published game/geometry/side/token,
stopped restoration, subsequent same-token consistency, and no hidden Chess
request even when the previous Chess engine turn was eligible. The test failed
on the old implementation (Chess != Shogi) and passes with the correction. An
initial test draft had incorrect imports; corrected before the demonstrated
behavioral failure. Fresh independent reviewer traced both read paths and the
bounded redispatch, independently ran this regression and adjacent archive-open
test (both passed), and diff check passed. No material finding remained.
