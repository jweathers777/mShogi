# Desktop archive v1

Historical MSH-35 contract. MSH-117 replaces this development format with the
[current desktop document](document-v1.md); its reader/writer is removed.
The original record below remains evidence of prior behavior.

MSH-35 implements explicit `.mshogi` documents under profile
`mshogi.desktop.archive.v1`, schema version 1. The archive is strict JSON with
canonical `MatchRecord` and final `GameRecord` JSON strings. Replaying the match
must reproduce the final game record exactly, including known history, policies,
adjudication modes and limits, claims, conclusions, match commands and events.
The canonical match owns its original timestamps; reopening offsets the new
monotonic runtime clock above their saved floor.

The document additionally retains human color, game board orientation and
analysis defaults, actual play limits, an ordered accepted desktop-control
journal, and accepted engine observations with their complete protocol-v2
request context, result, target, history, execution purpose, limits and original
identity. These observations are historical information on reopening. They do
not become pending requests, resume processes, apply moves or recreate an
analysis result for the currently selected position.

Only untimed live sessions with the documented initial play limits are
restorable. Timed sessions are rejected before expensive replay.
Unsupported profiles, limits, identities or mismatched records fail explicitly.
No FEN/move-list fallback or silent policy downgrade is permitted.

Externally supplied mating witnesses on match commands are also rejected before
replay; this desktop profile does not create them. This avoids routing a long
witness through uncontrolled match adjudication. Historical rejected engine
responses retain their raw wire evidence and explicit application-failure
diagnostic, never a promoted action, score or proof.

## Resource and replacement rules

One long-lived document worker has bounded pending and completion slots.
All filesystem access, serialization, validation/replay and historical-prefix
construction run there, separate from the serialized controller and engine
worker. Replacement cancels obsolete work. Cooperative cancellation and
15-second checks bound supported work; operating-system scheduling and blocked
filesystem calls are not hard realtime guarantees.

Documents have no byte, event or game-length limit (MSH-268). An immutable
candidate is fully encoded and validated on a separate worker before an
ordinary command or engine/analysis response changes the accepted document.
The current accepted snapshot remains authoritative while this check runs.
Cancellation, replacement and late results cannot install a superseded
candidate. Runtime-identity headroom preserves the stop/save/close path.

An operation which would exhaust runtime identities or the validation time
budget is refused before acceptance. Work
pauses with an **Application limit reached** notice, separate from an illegal
move, engine failure or chess outcome. The complete previously accepted game,
command history and observations remain available to Save and Save As. Nothing
is silently discarded. Save and reopening retain the observation and history
counts; they do not replenish capacity. Save the complete document, then
explicitly start a new game/document or open a different document to continue.
Retrying the same save is not the remedy for a structural capacity limit.

Save captures an immutable complete document and its contents revision. An
exclusive same-directory temporary file is written, flushed and synchronized;
replacement happens only after success and a cancellation check. Failure before
replacement removes the temporary file and preserves an existing valid save.
Only the exact successfully saved contents revision is marked clean. A newer
move, setting or observation remains dirty even if an earlier save succeeded.

Open first captures the approved live version and document revision. Validation
constructs a candidate without changing the current game. The controller commits
only after full success and an exact current precondition check. It establishes
fresh session, request, selection and engine-generation identities above both
current and saved watermarks, with monotonic frontend snapshot revisions.
Reopened work is stopped until explicit Resume. Failed, stale, corrupt,
unsupported or resource-limited opens preserve the current live document.

New game, Open, window close and ordinary Quit resolve dirty documents through
Save/Discard/Cancel. Confirmations bind the approved version and contents
revision; settings changes cannot authorize discarding a newer document through
an old confirmation. Save failure or cancellation cannot complete a destructive
intent. Native close approval blocks further commands atomically before the
existing bounded child-shutdown path. See [macOS Quit](macos-quit.md) for the
preventable Dock route. Forced termination is outside interactive guarantees.

## Historical provenance

The timeline selects exact game event boundaries, including initial known
game events and match commands such as claims or resignation. The worker
replays that prefix, excluding later moves, repetition occurrences and outcomes.
An initial imported FEN still begins known history; no earlier history is inferred.

Historical boards are read-only, with the live match retained separately.
Analysis uses the selected prefix and selection identity; changing the selection
retires old work, and late replies cannot describe the newly viewed position.
Return to live is explicit and leaves play paused; Resume is separate. There is
no branching or variation editing in MSH-35.

## Chess Take back extension (MSH-113)

Chess documents without Take back continue to write schema1/profile
`mshogi.desktop.archive.v1`. A document retaining a taken-back Chess continuation
writes schema3/profile `mshogi.desktop.archive.v3`. This is a narrow Chess lineage
extension, not a replacement game record schema or a common game rules model.
Older readers reject the new schema explicitly; the current reader accepts the
existing schema1, shared schema2 dispatch, and this schema3. Shogi and Xiangqi
continue to use their shared schema2 archive and control replay.

Schema3 retains the complete authoritative Chess match before each Take back,
bound to the corresponding ordered desktop control. Game replay independently
derives the restored human decision point. The next retained match and final
match must extend that restored decision, with the same root and provenance.
Every observation must target an exact prefix of the admitted current or retained
lineage. An unrelated parsed record is insufficient. These retained
records count toward byte, event and replay admission; undo does not reclaim the
observations or abandoned accepted history. Runtime identities and pending work
are never resurrected from this evidence.
