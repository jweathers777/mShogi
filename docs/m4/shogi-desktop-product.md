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
