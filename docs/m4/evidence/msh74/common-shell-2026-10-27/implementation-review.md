# MSH-74 common dark shell: independent implementation review

Reviewer: independent Codex review agent `msh74_implementation_review`, separate
from the primary builder. Date: 2026-10-27. Method: bounded source/diff review and
independent focused test execution; no implementation edits or integration by the
reviewer.

Baseline: `0c07e64`. Reviewed implementation integrated as
`a2524c10d2067f9f5ac3294c1a78efcddd771e99`. Tests ran against the corresponding
working changes before that commit; the reviewer then inspected the committed
revision and final CSS corrections. This review concerns the owner's second
shell refinement, which replaces the separate application themes with one dark
application composition. Earlier native inspection evidence remains separate.

## Scope and assessment

Inspected the recorded owner refinement in `docs/m4/shogi-desktop-product.md`,
the changes to ApplicationShell, GameArea, PlayerArea and InspectorSwitch, both
game adapters, ShogiBoard composition, authoritative shared styling, and the
packaged GUI driver's new assertions.

Both games now supply content to the same header, live heading, upper-player,
board, lower-player, footer, analysis/history/actions and technical-details
slots. The composition change preserves ShogiBoard's selection state, native
choice routing, hand and board element references, focus capture, promotion
handling, inspection request retirement and keyboard handlers. Chess player
labels and turn indicators use its native selected view. The Shogi analysis tab
changes presentation only: bounded analysis still submits the existing native
read-only operation with the captured context and limits; switching tabs does
not accept or apply engine results.

No native rules, inspection query, canonical archive or execution-acceptance
implementation changed in this refinement. Existing evidence for those paths is
reused rather than presented as newly executed native verification.

## Findings and resolution

1. **Avatar geometry lost during stylesheet cleanup — resolved.** Removing the
   old Shogi application styling also removed the avatar's centering, circle,
   border and font geometry. The authoritative shared stylesheet now supplies
   `display:grid`, centering, the border, circular radius and font for both
   games. Final source inspected.
2. **Initial resignation control had different vertical spacing — resolved.**
   Chess retained the global `.resign-button` top margin of 12 pixels while
   Shogi had zero top margin. The common shell now resets the Chess control to
   `margin:0`; the driver retains its strict cross-game geometry comparison.
   Final source inspected.
3. **Historical notice used different physical regions — resolved.** Chess's
   notice was in the sidebar and Shogi's above its board. Both now populate
   ApplicationShell's `positionNotice` slot below the live heading. Existing
   native history text, Return to live handlers and focus behavior were moved
   without changing their execution semantics.
4. **GUI analysis-tab entry needed adaptation — resolved in the reviewed
   driver.** Shogi defaults to Play details, matching Chess. The driver now
   selects Analyze position before accessing its analysis inputs and waits for
   the actual bounded-analysis control. It does not bypass the UI or invoke a
   mutating native test endpoint.

The final history-card CSS uses the same 236-pixel height (220 pixels at the
compact breakpoint), with scrollable native content, so differing caption or
event content does not relocate the following Game actions card. This is a
source assessment; actual fit and computed geometry require packaged evidence.

No unresolved material implementation finding remains in this reviewed delta.

## Independently executed checks

Environment: local macOS checkout; bundled Node 24 runtime selected through
`/Users/johnweathers/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin`.
Successful frontend commands ran from `desktop/`:

```sh
npm test -- src/product/ShogiBoard.test.tsx src/product/ShogiApp.test.tsx src/product/ApplicationShell.test.tsx src/App.test.tsx
npm run check
```

Results: **29 tests passed across four files**; strict TypeScript checking passed.
After the shared historical-notice move, independently reran:

```sh
npm test -- src/product/ApplicationShell.test.tsx src/product/ShogiApp.test.tsx src/App.test.tsx
```

Result: **16 tests passed across three files**. These are overlapping focused
runs, not additive counts of unique tests. The final avatar, resignation-margin
and fixed history-card corrections were inspected as CSS-only changes.

From the repository root:

```sh
python3 -m py_compile desktop/scripts/gui-smoke.py
git diff --check
```

Both passed. An initial test invocation from the repository root encountered
`ENOENT` because `package.json` is in `desktop/`; the successful frontend runs
above used the correct directory. That invocation was not a failed product test.

## Driver and evidence boundaries

The reviewed driver reads actual computed themes and element geometry, compares
the same measured viewport and initial inspector state across games, checks the
real shared slot order, and exercises minimum-size inspection through normal UI
input. It preserves strict card/slot comparisons instead of weakening them to
accept differing layouts. No hidden state mutation or unsupported evidence claim
was identified in that delta.

This document does **not** establish actual screenshots, cross-platform package
interaction, visual fit, or owner approval. At review time the Mac GUI was locked;
the primary builder continued independent checks while awaiting unlock. Final
packaged application and visual evidence must identify their own exact revision
and environment. MSH-74 remains In Review and MSH-75 is not started.

The external Linux picker TOCTOU fix was not present in this reviewed revision
and is not covered by this verdict. Any later review of that change must be
recorded explicitly rather than attributed to this review.

## Later bounded review: Linux external picker fallback

The same independent reviewer subsequently inspected the uncommitted
`desktop/scripts/gui_linux.py` fallback change against `a2524c10`, plus the
retained `desktop/scripts/test_gui_linux.py` adapter simulations. This later
section extends the scope explicitly; it does not change the product-source
verdict above.

Read the concrete failed `0c07e64` Linux run's downloaded
`desktop-ubuntu-24.04/gui-evidence/gui-report.json` from Desktop run
40475158549. It records two `Refusing native input to an unbound or missing
window` errors (normal-shogi and verification-capacity), with fallback activation
in their traces. The separately recorded normal-shogi-reopened WebDriver
connection reset is not addressed or relabeled as passed by this fix.

The adapter now rechecks the exact bound picker ID, process and title, tolerates
its disappearance during fallback activation without sending another key, and
requires that same picker to remain present and focused before sending its
Open/Save mnemonic. It does not choose an arbitrary replacement window. An
activation failure while the picker remains open, changed identity, unrelated
focus, or host exit still fails. The existing final picker-close/host-alive and
exact Save pathname checks remain. The caller's native Save completion,
saved/dirty-state checks, archive JSON/schema/content assertions and native Open
state checks also remain; picker disappearance alone is not application success.

Independently executed from the repository root:

```sh
python3 /private/tmp/msh74-picker-fallback-test.py
python3 -m unittest discover -s desktop/scripts -p test_gui_linux.py -v
python3 -m py_compile desktop/scripts/gui_linux.py
git diff --check
```

Results: **10 adapter simulations passed** in each of the temporary and retained
copies; these are the same ten cases, not twenty unique tests. The retained
copy resolves its adjacent adapter through `__file__` and uses a portable mocked
output path. Compilation and diff checking passed. No material finding remains
in this driver-only change. These simulations establish its branching and
identity checks, not actual GTK GUI success or an atomic X11 focus/input guarantee.
A fresh packaged Linux run must establish the real interaction outcome.

## Later bounded review: shared live-help footer

Reviewed the subsequent uncommitted five-file delta on
`61afcec45ccd59f78d3df66ed60586846ffecf42`: Board, App, its focused test, shared
footer CSS and the external GUI driver. An actual screenshot review identified
that Chess help remained inside the board region before the lower player while
Shogi help occupied the shared footer. This was a real spatial inconsistency,
not a reason to relax the visual comparison.

Board now retains ownership of the live help text and inspection state while
optionally rendering that noninteractive text into App's footer target through
a React portal. Omitted targets retain inline standalone/fixture behavior;
explicit null suppresses an interim product fallback while the footer ref is
established. The change does not alter Board identity, native actions, inspection
request retirement, focus references or keyboard handlers. Portal content is
removed when Board unmounts. Shared dark inspection styling remains applicable
at the footer location. The native Chess footnote remains separate below the
help. The driver now requires an actual help node inside the shared footer and
compares its top coordinate across the two initial game views.

Independently executed from `desktop/`:

```sh
npm test -- src/Board.test.tsx src/App.test.tsx src/product/ApplicationShell.test.tsx
npm run check
```

Results: **20 tests passed across three files**, including the added live portal,
inspection, focus, Escape, nonplay and unmount regression; strict TypeScript
checking passed. From the repository root, Python compilation of
`desktop/scripts/gui-smoke.py` and `git diff --check` also passed. No material
implementation finding remains in this delta. These checks do not establish
rendered line wrapping, final footer fit or actual cross-platform GUI results;
the changed product source requires its own fresh packaged evidence.
