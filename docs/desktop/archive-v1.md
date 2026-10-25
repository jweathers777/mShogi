# Desktop archive v1

MSH-35 implements explicit `.mshogi` documents under profile
`mshogi.desktop.archive.v1`, schema version 1. The archive is strict JSON with
canonical `MatchRecord` and final `GameRecord` JSON strings. Replaying the match
must reproduce the final game record exactly, including known history, policies,
adjudication modes and limits, claims, conclusions, match commands and events.
The canonical match owns its original timestamps; reopening offsets the new
monotonic runtime clock above their saved floor.

The document additionally retains human color, native board orientation and
analysis defaults, actual play limits, an ordered accepted desktop-control
journal, and accepted engine observations with their complete protocol-v2
request context, result, target, history, execution purpose, limits and original
identity. These observations are historical information on reopening. They do
not become pending requests, resume processes, apply moves or recreate an
analysis result for the currently selected position.

Only untimed M2 live sessions with the documented initial play limits are
restorable. Full live adjudication and timed sessions are rejected before
expensive replay. Advanced read-only Full observations retain their original
query caps and M2 target record; this does not convert the live game to Full.
Unsupported profiles, limits, identities or mismatched records fail explicitly.
No FEN/move-list fallback or silent policy downgrade is permitted.

Externally supplied mating witnesses on match commands are also rejected before
replay; this desktop profile does not create them. This avoids routing a long
witness through uncontrolled match adjudication. Saved advanced Full analysis
witnesses are different: their legal geometric route and requested mating side
are checked step by step on the bounded document worker, without rerunning the
reachability solver. Historical rejected engine responses retain their raw wire
evidence and explicit application-failure diagnostic, never a promoted action,
score or proof.

## Resource and replacement rules

One long-lived native document worker has bounded pending and completion slots.
All filesystem access, serialization, validation/replay and historical-prefix
construction run there, separate from the serialized controller and engine
worker. Replacement cancels obsolete work. Byte/event caps and cooperative
15-second checks bound supported work; operating-system scheduling and blocked
filesystem calls are not hard realtime guarantees.

Supported documents contain at most 16 MiB, 4,096 combined initial-history and
match events, 256 accepted engine observations and 8,192 desktop controls. Save
or Open exceeding a cap fails; no history or observations are silently dropped.
The running game can continue beyond the persistence caps, but its complete
document cannot then be saved within this format's supported bounds.

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

The timeline selects exact native event boundaries, including initial known
game events and match commands such as claims or resignation. The native worker
replays that prefix, excluding later moves, repetition occurrences and outcomes.
An initial imported FEN still begins known history; no earlier history is inferred.

Historical boards are read-only, with the live match retained separately.
Analysis uses the selected prefix and selection identity; changing the selection
retires old work, and late replies cannot describe the newly viewed position.
Return to live is explicit and leaves play paused; Resume is separate. There is
no branching or variation editing in MSH-35.
