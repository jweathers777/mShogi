> Current MSH-102 implementation: [analysis, outcomes and document journeys](desktop-journeys.md).
> The MSH-101 checkpoint/deferral statements below are preserved historical evidence.

# Xiangqi desktop product — MSH-101

Owner direction,2026-10-28; visual approval and completion continuation,2026-10-29. The selected native profile remains
`mshogi.xiangqi.wxf2018.v1`. Existing MSH-74 Chess/Shogi ApplicationShell,
GameArea, common dark theme and stable controls are the product authority.
The owner's board/piece direction does not approve an alternative application
shell. MSH-101 has a mandatory early actual packaged visual checkpoint; it
remains unfinished until that presentation is explicitly approved and its
remaining criteria are verified. MSH-102/103/106 are not started here.

## Native ownership and board presentation

New Game admits the native standard record under Explicit claims through the
existing private-candidate archive/capacity worker and journaled Resume. Both
human assignments are native Red/Black; initial orientation follows the human.
Flip changes native document preferences, not game state. Moves submit a full
`xiangqi` command with the opaque complete action; they never use Shogi's Play
encoding. The existing supervisor, fresh identities, bounded execution and
read-only token-bound inspection remain authoritative. Product Open remains
gated for Xiangqi until MSH-102's full document journeys.

The dedicated SVG intersection renderer consumes the native90 points,
orientation orderings, river separation and both owned palaces' diagonal paths.
It does not call the square renderer or calculate movement rules. The board is
warm honey wood with restrained texture, fine brown lines, subtle framing and
subordinate 楚河/漢界 lettering. Exterior coordinates retain native identities.
Width and height both constrain scale while preserving the49:54 outer artwork
ratio and square spacing between intersections. The complete board must fit
1180×820 and760×650 content workspaces and intermediate resizing, with no board
scrolling, panning or zooming. Player areas and the common bottom status remain.

Round ivory discs have a dark rim, subtle relief and upright Chinese characters
in either orientation. Decoration maps native kinds and Red/Black owners to the
approved differentiated artwork: 帥/將,仕/士,相/象,傌/馬,俥/車,炮/砲,兵/卒.
Native kind/owner/coordinate and English accessibility names remain independent
of glyphs. No Shogi hands, promotion, drops or declaration controls are added.

Primary Chinese typography uses a separately renamed6,804-byte subset of Noto
Serif CJK TC Regular2.003, rather than platform font fallback or changes to the
accepted Shogi JP subset. The [font notice](../../../desktop/src/assets/fonts/NOTICE.txt)
records the pinned official source, source/subset hashes and reproduction.
The existing OFL1.1 is byte-identical to that source's license. All18 required
piece/river characters have verified nonempty outlines; two subset generations
match. `subset-xiangqi.py` reproduces the asset while retaining upstream credit
and license metadata and using a distinct modified family/PostScript name.

## Interaction

Friendly origins expose only Rust-supplied complete choices. A sole complete
action commits atomically. In particular an occupied legal destination commits
the selected friendly capture before considering opponent inspection. Native
opponent inspection is read-only, uses hollow burgundy destination rings and a
dashed origin ring, leaves occupied pieces readable and announces Inspecting.
It cannot submit a player action. Escape cancels and restores origin focus;
arrows navigate presentation order, Enter/Space select. Orientation/context
changes and origin switching retire delayed inspection replies. Historical
inspection, when exposed later, remains bound to the exact native selection.

The original header, document controls/title/state, live heading/Flip,
upper/lower player areas, board/footer and Analysis→Game history→Game actions→
collapsed technical slots remain shared. Real native play limits and completed
engine observations use the existing Native engine panel. Raw contexts,
profile/transport identities and diagnostics stay in technical details.

## Early checkpoint boundary

The checkpoint integrates actual normal packaged Xiangqi New Game, native
human/engine moves, Flip, pointer/keyboard selection and native opponent
inspection. Existing Save and unsaved-replacement/quit admission are retained;
their full Xiangqi document journey is not claimed here. Analysis execution,
historical navigation, expanded outcome/actions and recovery/Resume UI remain
MSH-102 work; their unavailable controls are not represented as working features.
No rules, evaluator, transport or archive schema is replaced to build this UI.

Required actual screenshots: initial Red/Black-bottom orientations, friendly
selection and native inspection at normal/minimum sizes where practical,
intermediate sizing and matching current Chess/Shogi comparisons. Record native
frame/content dimensions distinctly, source/package hashes, environment,
commands, tests, review and any clipping/contrast/interaction findings. Unit tests
do not substitute for packaged visual inspection. No verification-only surface
or generated mockup supplies this checkpoint evidence.

The owner explicitly approved the actual3a1aefc checkpoint on2026-10-29. That
satisfies its visual gate; the checkpoint evidence remains a historical record.
Remaining MSH-101 criteria and independent verification still govern completion.
MSH-102 stays unstarted; MSH-45/M4 remain open. MSH-103 is the later
complete-delivery review and106 follows45.

## Completion refinement — 2026-10-29

The owner-approved shared visible inspection status is exactly
`Inspecting {side} {piece} · Esc clears`, formatted once over the native label in
`movementInspection.ts` for Chess/Shogi/Xiangqi. Native promotion/hand terminology
remains native. Read-only explanations stay in accessible announcements; hollow
markers, stale-token protection and keyboard/Escape behavior do not change.
This is the durable all-game convention in design A-07, including future
Tori/Chu/Tenjiku. No separate inspection framework or rules API is introduced.

Native Xiangqi played-event labels use the existing intersection display
notation: `Red: a3–a4`, `Red: b2×b9`, and an authoritative ` · check` suffix.
The Rust presenter exhaustively labels warning/retirement, claimant-specific
claim submission/acceptance/rejection, offers/agreement/resignation, unresolved
unscored adjudication and native conclusions. Labels never replace the canonical
record, exact event/board-ply indices or native action/diagnostic payloads. No
replay, exact proof or frontend rules inference is needed to format them.
Full interactive history/outcome/document journeys remain MSH-102.

Completion source `be3055a1bc8d4b0d0fdf11f8509b97076d13fa43` is integrated on
`codex/rewrite`; verifier-only follow-up `1c8a1682db5b98e765eb755b7978537c20a2ddd2`
has passing native/desktop CI on all three systems. The [completion evidence](../evidence/msh101/completion-2026-10-29/README.md)
records actual normal macOS played/inspection captures, independent review,
resolved Debug labels, preserved initial CI failures and reused sizing provenance.
MSH-101 is complete; MSH-102 remains unstarted and owns the explicitly deferred
full journeys. This does not close MSH-45 or M4.
