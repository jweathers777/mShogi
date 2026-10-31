# Owner-approved desktop UX after R4

Owner decisions recorded **2026-10-31**, against `codex/rewrite`
`446c8b3934940b9b27868eb8e0f6d67fc905334f`. This is planning, not delivered
behavior. MSH-119 stays Done with its evidence and accepted common dark shell.
The owner's five additional clarifications take precedence over the attached
planning request. No production, native contract, archive, test or GUI-driver
implementation is changed by this assignment. Root Codex is the planning
integrator in the primary checkout; future tasks remain unassigned/unstarted.

## Work and scheduling

| Task | Bounded ownership | Complete direct dependencies |
| --- | --- | --- |
| MSH-123 | Standard Moves list, native notation/event mapping and completed last-move cues | MSH-120 |
| MSH-124 | Shared shell order/copy, implemented-game dropdown, preserved side buttons, branding and support access | MSH-119 |
| MSH-125 | Save/Load/New/Quit, direct Resign and one-step Return to live admission | MSH-119 |
| MSH-121 / R6 | Independent integrated architecture and approved product acceptance | MSH-116, MSH-117, MSH-118, MSH-119, MSH-120, MSH-123, MSH-124, MSH-125 |

These are three product tasks, not a reopening of R4 or a fourth UI framework.
R5/MSH-120 retains its original dependency, acceptance criteria and primary
controller responsibility. **Last-move rendering belongs to MSH-123 after R5**:
it consumes the surviving renderer/controller boundary without adding pending
selection state to the application shell or making R5 responsible for this new
product feature. It may add the minimum native-derived presentation needed for
real history; it must not infer geometry by parsing visible labels.

The migration spine remains R5 → R6, with R6 additionally waiting for all three
product tasks. Suggested integration sequence is R5,123,124,125,R6. Tasks124/125
are dependency-ready independently of R5/123; that suggested order is not an
artificial hard dependency. They share ProductApp/ApplicationView files, so one
integrator serializes overlapping edits and records a source-grounded plan on
activation. No task acquires a future variant or M5 gate. No task is activated
by this planning update.

### Current source boundary inspected

At the baseline, `product/ProductApp.tsx` owns the game cards, unchanged native
side buttons and replacement/Quit prompt intent. `ApplicationView`
supplies Save/Save as/Open, history/technical content and Return to live;
`ApplicationShell` orders Analysis/History/Actions/technical. `GameCapabilities`
wraps Resign in its common confirmation. `NativeSessionPanels::NativeHistory`
renders every event as a row. These are the actual consumers for124/125 and123,
not an authorization for a second application assembly.

The current native `shared::PresentedEvent` carries index/board_ply/label;
Chess, Shogi and Xiangqi adapters preserve different command/event granularity.
That label is insufficient for standard notation or authoritative completed-action
geometry. Native Chess action text is coordinate notation; the Shogi console's
readable notation is explicitly custom. Native ReturnLive currently retains a
paused state. The existing board `lastDestination` references serve focus, not
verified historical action identity. Tasks123/125 own the minimum real native
presentation/admission extensions these consumers need;120 owns interaction
consolidation. Concrete type/helper choices belong to each activated worker.

## Common shell and controls

Keep the accepted dark application theme, board/player/footer spatial grammar,
native pieces, Shogi hands and Xiangqi river/palace/intersection art. The sidebar
order becomes **Analysis → Game Actions → Moves** in all games. Remove the
ordinary About & technical details sidebar section. Provide one nonintrusive,
keyboard-accessible application support/About mechanism outside the gameplay
sidebar for diagnostics, exact technical provenance and required license notices.
Do not delete those notices or raw evidence. Actual analysis proof/Unknown,
warnings, unresolved outcomes and useful recovery remain player-facing in their
proper sections, not available solely as a diagnostic dump. Native Quit remains
discoverable after removing its current technical-disclosure placement.

New Game replaces **only** the large game-description cards with a compact
dropdown. It currently lists **Chess, Shogi, Xiangqi**, all implemented/playable.
Future/unsupported variants are absent, including disabled entries. The control
can accommodate six games when implemented, without displaying promises now.
Keep the existing visual treatment and keyboard behavior of side-selection
buttons: **White/Black**, **Sente/Gote**, **Red/Black**, plus **Random**. Do not
replace those buttons with another dropdown or relabel native sides generically.
The brand uses the existing Shogi selector character **歩**, replacing the rook.

Remove the game-description slogans, start-at-bottom/untimed narration,
reconstruction explanations and implementation-oriented result prose. Preserve
concise information needed to understand the current game, bounded work,
uncertainty, capacity and recovery. The inspection convention remains
`Inspecting {side} {piece} · Esc clears`, with native labels, distinct
non-actionable markers and accessible announcements.

## Moves, exact positions and completed-action cues

The visible **Moves** panel is a conventional numbered two-player list with
native side headings, one move per player cell, clickable cells and selected
position highlighting. It includes an explicit starting-position target. Empty
games, odd final rows and a root whose next actor is the second player must
align correctly. Do not fabricate moves before an imported root from its
declared counter; show known-history scope and appropriate numbering clearly.

Only actual complete board actions populate the two columns. WXF warnings,
claims, resignations, agreements and other non-move events remain authoritative
history, **not additional move rows**. Associate annotations with their actual
move or selected position. A click resolves through a native exact post-action
prefix, not `row * 2`, board ply alone, current diagram or parsed label. Keep
command-start/action/derived-event/command-end distinctions. In particular, do
not silently turn a click on an action into a later same-ply warning prefix.
Position/event detail or navigation must still reach distinct non-move prefixes;
same-board/different-warning/claim contexts remain selectable and distinguishable.
Historical analysis uses exactly that known prefix, without future information.
Browsing and analysis never change or branch the live game.

Last-move presentation is native-derived from the **selected prefix**: highlight
origin and destination subtly, with stronger destination emphasis. It survives
thinking, polling and orientation changes; historical navigation shows the last
completed action at that position. A drop has only its destination. Compound
actions preserve authoritative ordered geometry, including returning to the
origin and distinct routes sharing endpoints. Castling and other special
effects likewise use supplied native geometry, not frontend movement rules.
Non-move events retain the previous real action's cue or none at the known root.
Cancelled partial selection is never a completed action. Distinguish this cue
from legal destinations, inspection markers, partial routes and keyboard focus;
do not rely solely on color. Replacement, Take back and Return to live refresh
it from the accepted native context, not a remembered pointer destination.

### Sourced display conventions

Notation is a native-derived **display adapter**, not the canonical action,
record format, input parser or game-rules authority. Keep source-linked expected
examples separate from implementation output and comparator observations.

| Game | Display selection and source | Required independent examples |
| --- | --- | --- |
| Chess | English SAN, [PGN specification §8.2.3](https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm). Native pre-action legal context determines ambiguity; post-action native facts determine check/mate. Coordinate actions remain canonical. | Captures, en passant, both castles, legal rather than merely attacking alternatives, underpromotion, check and mate. |
| Shogi | Japanese notation following [JSA 棋譜の表記方法](https://www.shogi.or.jp/faq/kihuhyouki/), Basic matters and the illustrated ambiguity cases. Use its stated JSA recording form with numeric file/rank, native Japanese piece names, 同, promotion/nonpromotion and contextually required 打. This is not a claim to implement KIF/KI2 file interchange. | Both movers' perspectives; 同 from known preceding action; promoted piece names; drop versus board action; compulsory/optional promotion; movement-first then left/right/straight disambiguation, with the guide's explicit **no 直 for 竜/馬** exception. Unknown earlier root history cannot establish 同. |
| Xiangqi | Selected [WXF2018 English Article7.1–7.5, printed15–17 / PDF23–25](https://www.wxf-xiangqi.org/images/wxf-rules/2018_World_XiangQi_Rules_English2018.pdf). Use7.2 direction symbols with7.4 English letters K/A/E/H/R/C/P and7.5 disambiguation, including its front/rear and stacked-soldier examples. This is notation selection, not a rules-profile change. | Both Red/Black file perspectives; straight distance versus diagonal destination file; advance/retreat/traverse; front/rear; three/four/five and multiple-file soldier ambiguity. Pin each expected string to source and concrete native position. |

The JSA guide distinguishes its numeric recording style from newspaper/book
rank kanji; this is an ordinary sourced presentation choice, not a new owner
checkpoint. Its examples also make automatic promotion marking and contextually
required drop marking explicit. The old source-explicit console Shogi text is
documented as custom in [the console contract](../m4/shogi-console.md#explicit-readable-notation)
and cannot be relabeled standard by reuse.

For Xiangqi, the selected rulebook itself illustrates `C2=5`, `H8+7` and the
stacked-soldier forms `36=5`, `26=5`, `16=5`. Do not substitute an older AXF/XQR
alphabetic-pawn convention merely because it is hosted by WXF: the older file
format proposal's Appendix A expressly contains uncertainty about stacked pawns.
Rare multiple-file cases must be reconciled against concrete sourced fixtures
in123, not guessed or silently declared conformance. Exact native origin/action
identity remains available independently of any short notation ambiguity. This
is a bounded fixture/research obligation of123, not a blocker for R5/124/125.

Research method: JSA official page and PGN primary specification text read;
selected, previously acquired WXF PDF pages23–25 rendered and visually inspected.
The rulebook's actual7.4 examples support `=`, rather than choosing `.` from a
different tutorial. Rendering succeeded with existing Poppler JP2/font warnings.
The FIDE Handbook retrieval timed out and the indexed RCC PDF returned404;
neither is represented as a successfully read source. No notation implementation,
fixture execution or GUI verification is claimed here.

### MSH-123 selected notation completion (2026-10-31)

The planning text above is retained as the original contract. The owner has now
approved the rare-soldier display completion after independent inspection of
both English and Chinese WXF2018 Article7.5: extend ordinal+file to stacks of
three through five soldiers, and to paired soldier stacks on multiple files
where front/rear would be ambiguous. Count from the mover’s front within the
file, then use the mover-relative file number. Red soldiers c5/c6 and g5/g6 give
c6→d6 `17=6` and g6→f6 `13=4`. This is an **owner-approved source-consistent
display extension**, not an assertion that WXF explicitly specifies these rare
cases. The three-soldier FigureB strings remain `36=5`, `26=5`, `16=5`.
Neither selected rules profile nor native canonical action/history changes.

MSH-123's native projection owns notation, exact post-action event prefixes,
separate command boundaries, imported-root numbering provenance and selected
completed-action geometry. A Xiangqi action at prefix8 and its derived warning
at9 remain separately selectable; execution at the intra-command action prefix
is explicitly unsupported rather than silently retargeted. Position events stay
in a disclosure, outside the two-player move rows. Selected move and event
access remains through native admission, with no frontend replay.

The completed destination uses strong corners plus two underline strokes;
origins use smaller dashed corners. This separates completed movement from
Shogi capture targets, legal dots, inspection rings and keyboard focus without
relying only on color. Castling secondary movement and ordered compound routes
come from native geometry; drops have destination only. See the bounded
[implementation and evidence review](../review/moves-123.md).

## Document, resignation and historical return behavior

Ordinary controls are **Save** and **Load**. First Save chooses a path; later
Save writes that path. Remove ordinary Save As; the underlying atomic mechanism
may remain internal. Failed/interrupted Save preserves any prior valid file and
does not clear unsaved state. Picker cancellation is not successful saving.

**New Game and Load are intentional replacement actions without an extra
unsaved-game confirmation.** A successful valid replacement intentionally loses
the previous unsaved game. No automatic save or retained previous-game document
is promised. Until validated replacement is admitted and installed atomically,
the current document remains intact: chooser/picker cancellation, failed new
game, unsupported/corrupt/resource-limited Load or a stale request cannot discard
it. Source tokens, file/close reservations, accepted Save receipts, bounded
replay/admission and fresh stopped reopen identities remain authoritative.
Reopened observations are historical, never pending work; explicit Resume is
still required for a loaded session.

Unsaved Quit/window-close keeps a concise **Save / Discard / Cancel** safeguard.
Remove Save & continue, Discard & continue and redundant paragraphs. A clean
document needs no unsaved warning. Save failure/cancellation retains the game;
stale clean receipts cannot authorize closing newer dirty work. Native macOS
termination/window/menu protection and cancellation/cleanup remain required.

**Resign has no confirmation dialog.** Submit once through native actor/context
admission and busy/double-submit protection. Show the actual concise conclusion
or unresolved-unscored result, never an assumed loss. Other claims/declarations/
draw-offer semantics and their confirmations are outside this specific removal.

One-step historical return uses native admission, not frontend auto-clicks:

| Before/while browsing | Return to live |
| --- | --- |
| Otherwise running human or engine play, suspended solely for browsing | Restore live view and eligible running-play intent in one action; human may play or a **fresh** engine-turn request starts. No second unexplained Resume. |
| Already stopped after Load, Take back, explicit cancellation or failure | Restore view, keep the named stopped state; Resume/Retry remains explicit and meaningful. |
| Concluded game | Restore view only; do not reopen the game or invent play. |
| Intervening explicit stop/newer stop reason, replacement, conclusion, capacity refusal or stale operation | New authority takes precedence; no automatic retry or resumption. |

Entering history always retires execution. Retained browse intent is runtime intent,
not a resumable engine request; no request/session/process identity is revived.
Explicit historical analysis remains read-only and independently cancellable.
Returning from it retires that work before any fresh eligible engine-turn
admission; it cannot act as an engine move or restart expensive Full/exact work.
This policy resolves ordinary browsing friction while retaining intentional
stops. No additional material owner decision is identified at planning time;
a concrete contradiction discovered during implementation must be recorded.

## Future verification and current planning evidence

Each product task requires focused native/frontend tests for its changed
boundary, independent review, actual identified packaged interaction and normal
integration/applicable CI. Keep source/package/environment/platform and unit/
GUI evidence distinct. Use separate QA packages for hard states and verify
normal instrumentation exclusion. Check normal/minimum layouts, keyboard,
focus/announcements, complete board visibility, exact history/analysis provenance,
stale work, Take back lineages, current envelope and atomic Save/Load guarantees.
R6 finally exercises the combined interface on Windows, macOS and Linux; its
existing required platform boundary is preserved, not replaced by unit tests.

### Completed planning checks

Used pinned Backlog **1.55.1** after detecting global1.55.0, reading overview,
creation, execution and finalization guides. Fetched origin and verified the
clean primary `codex/rewrite` aligned at the baseline; no active task owner was
displaced and unrelated worktrees were preserved. All task creation/updates use
the CLI, with the complete R6 dependency set supplied.

Newly performed: Backlog doctor and a read-only Python frontmatter/DAG check of
**125** records establish unique/resolved IDs, no missing/duplicate/self edges or
cycles, and the exact dependency sets above. Original120 criteria and121's first
eight criteria/DoD remain verbatim;121 has one additional clarifying product
criterion. The other **120** old task files, including119 and its evidence, are
byte-unchanged from the baseline. All five future coordination/product records
are Ready, unassigned, unchecked and without implementation plans. Local design/
task documentation links resolve; A-11/D-19 are unique; `git diff --check` passes.
Production crates/frontend/host, manifests/lockfiles, workflows and AGENTS are
unchanged. No variant/M5 or other task prerequisites changed.

Separate read-only reviewer `/root/r4_independent` inspected the owner request,
planning diff, current consumers, all125 task relationships, JSA/PGN sources and
WXF page images. **PASS; no unresolved material finding.** Its source-precision
recommendation for JSA's 竜/馬 no-直 exception was incorporated in123 and the
source table. It reviewed intent/dependencies and fixture expectations, not an
unimplemented UX. No product tests, GUI/build campaign, new correctness/CI claim,
R5 implementation or UX implementation is made by this planning assignment.
Ordinary integration/push carries these planning records only; existing failed
checks and completed evidence remain historical and unchanged. Stop after
planning; no new owner decision is required by the recorded direction.

## MSH-124 implementation handoff

The shared shell now renders Analysis → Game Actions → Moves for all three
implemented games. The header uses **歩** and one small **About mShogi** control;
About contains keyboard-operable Diagnostics and Font license disclosures,
full native context/provenance/resource facts, bundled notices and Quit. Fast
Xiangqi board-win assessments and historical exact result meanings remain in
Analysis, with Unknown, incomplete work, failure and possible versus forced
wins distinguished. The game chooser is a labeled native select with exactly
Chess/Shogi/Xiangqi; the existing native side radio buttons remain unchanged.

Moves and the outer sidebar reserve eight pixels for the existing outward
focus ring; focus reveal is restricted to sidebar controls and their nested
scroll viewports. This corrects the MSH-123 lower-edge presentation boundary
without adding a board controller or changing exact history selection.
MSH-125 remains separate: Save/Save as/Open, replacement/quit guards, Resign
confirmation, and Return to live followed by explicit Resume retain their
existing behavior. Package evidence and independent review are recorded in the
MSH-124 task and its review record; implementation alone does not establish
platform or visual acceptance.

Final MSH-124 acceptance evidence is recorded in the
[shared-shell review](../review/shell-124.md). Final production
`91eb13e2451af77150b82b62a448bf97e9d5ac58` uses a decorative SVG About icon
after actual Linux review found a missing font glyph; the original failed
captures remain preserved. Exact common minimum Analysis geometry is supported
by a shared 232px minimum, measured from actual Linux/Windows content. Final
normal/minimum package comparisons, complete outward focus bounds, supported
native select/side-button journeys, licenses and normal instrumentation
exclusion are recorded separately for each platform. MSH-125 remains unstarted
and MSH-121 retains integrated architecture/product acceptance.
