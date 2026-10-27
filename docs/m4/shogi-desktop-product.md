# Shogi desktop product design — MSH-74

Owner product direction, 2026-10-26. This is the normal mShogi application;
MSH-72/73 contract harnesses remain verification-only and are not its visual basis.
Established native shogi rules/profile, engine, USI, shared session and v2 archive
remain authoritative. MSH-75 and MSH-41–47/M4 closure are outside this task.

## Product and visual structure

One application, with explicit Chess/Shogi selection in New Game. A small
native-qualified choice list can later add Tori, Chu, Tenjiku and Xiangqi without
separate applications or a universal rules model. Human side choices are native
White/Black or Sente/Gote, with Random where supported; the resolved assignment
is recorded. Human at bottom is the initial orientation; Flip Board is explicit.
Preserve complete existing M3 chess rules, play, Full analysis, claims, records,
recovery, accessibility and quit behavior. Chess retains its own pieces/board.

Shogi uses a warm uniform wood-colored field, visible grid lines and pentagonal
pieces. Bundled licensed Japanese typography renders kanji by default; semantic
kind/promotion/ownership remain native fields, never glyph-derived identity.
Gote ownership rotates pieces180 degrees relative to Sente; orientation follows
the selected bottom side. Promoted identities have explicit glyph/shape treatment
and accessible names, with more than color distinguishing state. Player identity,
turn/status and compact counted hand pieces sit near the corresponding board edge.
Zero resources are absent or visually subordinate; available hand pieces have
native legal drop choices and keyboard entry.

The board dominates a two-column workspace. History and dedicated analysis sit
in a subordinate side panel. Normal content sizes are1180×820 and minimum760×650
logical pixels, with intermediate resizing verified. Shogi board size derives
from both available width and height, with adaptive square/piece/type sizing.
All9×9 squares, both player/hand areas and essential controls remain visible
without board scrolling, zoom or pan. Side-panel content may scroll independently.
Future12×12/16×16 must support entire-board display by adapting scale/type rather
than requiring scroll/zoom/pan; the legacy application demonstrates this direction.

Normal chrome contains player concepts: New Game, Save, Save As, Open, Flip,
Resume/Stop, history and outcomes. No milestone names, profile identifiers,
versions, process generations or raw diagnostics dominate ordinary play. Optional
technical details retain full provenance for troubleshooting.

## Native interaction and state

The frontend stages native complete choices (ordered route plus completion),
never legality. Selecting a board or hand origin highlights only native supplied
destinations. A single complete action commits directly, including mandatory
promotion. Multiple optional promotion choices appear contextually and require
explicit selection. Multi-stage routes remain representable; partial selection
never commits. Escape cancels and restores sensible focus. Pointer/keyboard use
has visible focus, selected outlines and destination marks independent of color.
Accessible labels retain actual owner/kind/promotion even for rotated artwork.

History selects exact native event boundaries, including terminal events sharing
board ply. A compact Viewing move/event N notice and Return to live distinguish
inspection from play. Returning stays paused; Resume is explicit. Read-only
analysis targets the selected native prefix and cannot play moves/declarations.
It shares the supervised execution resource under existing explicit purpose,
identity, cancellation and replacement protections. Only actual final observations
are displayed: engine identity, completed depth, nodes, native evaluation
perspective/terminal command-edge distance, PV and analyzed target. No invented
live progress. Raw IDs/encodings belong in optional technical details.

Player-facing native status distinguishes turns/thinking, stopped/incomplete work,
process recovery, resignation, fourfold draw, continuous-check loss, entering-king
and false declaration. Never infer an outcome from turn. Formal declaration is
available only through native capability; advisory eligibility and committed
command remain separate, with confirmation and its loss consequence clear.
Untimed only; no evaluation-based result policy. Resource refusal is distinct
from engine failure and game result and retains a complete save path.

Native MSH-73 Save/Open owns complete game/profile/root/counter/known-history,
commands/events, conclusions, settings and historical observations. No TS replay
or FEN/move-list persistence substitute. Reopen validates natively, uses fresh
runtime identities, and stops work. Failed/cancelled Save cannot discard dirty
work; failed/unsupported/resource-limited/stale Open leaves current game intact.
Unsaved decisions guard New Game, Open and native quit/window-close. Historical
observations are labeled as such and never resume pending work.

## Bounded implementation and review stages

1. Record this design/task plan and inspect consumers; expose normal native product
   game/file/session entry points reusing supervised controller/admission/archive.
2. Build the game-specific shogi board/player/hand composition with native choices;
   inspect early screenshots from the actual packaged macOS application and refine
   board fit, typography, hierarchy and focus before completing all journeys.
3. Complete normal New Game, status/recovery/declaration/resignation, bounded
   analysis, exact history, Save/Open/Resume and unsaved-close integration. Reuse
   chess behavior and native fixtures; keep difficult-state instrumentation in
   separate verification builds.
4. Capture normal packaged macOS screenshots: initial Sente/Gote, both hands,
   drop selection, optional promotion, history, active analysis, conclusion and
   minimum size. Run native/controller/frontend checks, independent implementation
   and visual reviews, normal integration/push and all-three-system CI. Obtain
   Windows/Linux actual packaged GUI evidence, including normal-size screenshots.
5. Present the polished complete surface, screenshot set and launchable macOS
   build to the owner. Leave MSH-74 In Review until explicit owner visual approval;
   passing functionality does not authorize immediate Done. Do not start MSH-75.

Font provenance, exact package/revision/environment, each screenshot/journey,
failed checks and resolved findings will be recorded in the task evidence. Separate
native tests, frontend tests, actual GUI interaction, platform evidence and owner
visual approval. No material departure from the supplied product specification
is currently proposed.

## Owner refinement — 2026-10-27

The owner approves the overall visual quality and distinct Chess/Shogi character,
but requires the following before final approval. MSH-74 stays In Review; MSH-75
is not started. Preserve the earlier evidence as revision-specific history.

Both games use one actual shared shell: global identity/runtime and New Game,
Save/Save As/Open in the header; a single consistent document title/dirty-state
location; shared live heading/player status/Flip; board/player region left;
sidebar ordered Analysis, Game history, Game actions; collapsed technical access.
Chess retains dark styling/checkerboard/pieces, Shogi light styling/warm grid,
kanji/orientation/hands. Responsive sizing is game-aware without relocating
common controls simply because the game changed.

Movement inspection is an educational read-only native query bound to the exact
selected native token/history prefix. It returns complete semantic actions with
ordered routes/completions for the selected board or hand origin. The native
position adapter may evaluate that owner's hypothetical turn on a private
position; it does not change live turn, counters, rights, history, outcomes or
execution. Chess en-passant remains available only to its actual entitled side;
changing the hypothetical inspected side cannot transfer that transient right.
These are local movement choices, not predictions, history-dependent outcomes,
claims or engine threats. No query polls, starts, cancels or accepts engine work.

Friendly actionable selection remains native legal-choice entry. Opponent
selection, including available Shogi hand resources, uses hollow burgundy rings
and an explicit Inspecting label; occupied destinations retain readable pieces.
Pointer/keyboard support switching origins/modes, Escape and read-only multi-step
continuations without submitting any action. Historical inspection uses the exact
selected prefix and is always read-only. Superseded query responses cannot appear
on another position or revive executable choices. Native and frontend tests,
packaged Mac screenshots of both shared-shell themes and all selection modes,
independent reviews and applicable CI precede another owner visual checkpoint.

## Owner shell refinement 2 — one application theme (2026-10-27)

This supersedes the earlier dark-Chess/light-Shogi workspace direction, while
preserving those records and the approved native movement-inspection contract.
The surrounding application now uses one dark theme based on the Chess treatment:
header, workspace, panels, type hierarchy, controls, status surfaces and details.
Warm Shogi wood, kanji/pentagonal pieces, ownership rotation and counted hands
remain game content; selecting a game does not select an application theme.

Both adapters render the same ApplicationShell and GameArea composition. The
header owns identity/runtime, title/dirty state and New/Save/Save As/Open. The
left region owns a shared live heading/Flip and stable upper-player, board,
lower-player and help/status slots. Player labels remain native; Shogi fills the
hand content while Chess omits hands without moving other controls. The right
sidebar uses identical card treatments and order: Analysis, Game history, Game
actions, then collapsed technical details. Game-specific operations remain inside
these slots. Ordinary initial states and historical states must compare like
with like; differences in native capability/content do not justify unrelated
control migration. Board size remains game-aware within the same region.

Capture both games at matching normal and supported minimum dimensions; compare
actual screenshots side by side independently. Review theme, card geometry,
header/document controls, live heading/Flip, player/board relationship and sidebar
order; unexplained migration is a finding. Existing native/query behavior is
reused unchanged. MSH-74 remains In Review for final owner visual approval and
MSH-75 remains unstarted.

Expanded active-state comparisons apply the same rule to common secondary
actions: Start/Replace and Cancel analysis share one action row inside Analysis,
and historical viewing uses a common notice header with Return to live aligned
at its right edge. Native history explanations and Chess-only Full operations
remain game content. Live selection/inspection help follows the lower player in
the common footer, including when native inspection is active at minimum size.

## Owner final consistency pass — 2026-10-27

The owner accepts the major shared-shell visual design. Preserve its layout;
only these bounded consistency corrections precede completion:

- One document label model for both games: no accepted disk path is **Unsaved**,
  regardless of the native dirty flag; an accepted saved/opened path with no
  changes is **Unmodified**; a changed saved/opened document is **Unsaved changes**.
  These are presentation states, separate from native dirty/guard policy and
  in-progress, failed or cancelled file operations. A failed Save cannot establish
  a disk identity or clear native dirty state.
- One Native engine panel structure: title and actual lifecycle status, then
  **Play limit: depth N · M nodes**, then actual observations/actions. Native
  game budgets differ: Chess2/20,000 and Shogi2/5,000. Shogi's snapshot and
  submitted play requests use the same native constant; analysis defaults and
  past observations do not substitute for the current play budget.
- One bottom status row after selection/inspection instructions, preserving
  live/history, concluded, paused and human/engine turn meanings. Position counts
  use native known board actions at the selected prefix, not event indices or
  imported declared counters. Shogi hands remain in player/hand slots and do not
  displace this row. No new board or layout design is introduced.

The owner authorizes MSH-74 Done after these corrections pass focused regression,
updated same-size actual captures and required independent review/verification,
without another owner visual checkpoint. This supersedes the procedural gate in
older sections; it does not waive their behavioral or evidence criteria.
MSH-75 remains unstarted until MSH-74 is actually Done.

## Accepted shell requirement for subsequent games (2026-10-27)

The owner requires the actual shared MSH-74 composition/theme for subsequent
games. Application/document controls, live/status footer and sidebar structure
remain in stable slots. A different game may have a different board, pieces,
native labels and hand/player content without changing the surrounding app.
The [Xiangqi handoff](xiangqi/handoff.md) requires an intersection/river/palace
renderer and a complete visible board at normal/minimum desktop sizes; it does
not authorize redesigning the accepted Chess/Shogi shell or recreating rules
in TypeScript. This is future planning, not Xiangqi visual implementation evidence.
