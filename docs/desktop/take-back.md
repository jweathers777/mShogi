# Desktop Take back

MSH-113 adds the owner-approved **Take back** action to Game Actions for Chess,
Shogi and Xiangqi. The native session selects a human decision point and restores
it in one admitted transaction. The frontend neither counts moves nor reconstructs
rules or history. This is a human-versus-engine action; existing low-level Undo
contracts retain their separate meanings.

After a completed human move and engine reply, both are taken back. With a reply
pending, that execution is retired and the human move is taken back. For a human
playing second, the engine's initial opening is retained. Repeated Take back
walks earlier human decisions. Native actor identity and complete command
boundaries determine the target; one command may have several events.

Concluded games allow restoration when native history establishes a playable
human decision point. A terminal human decision taken on a playable human turn
(such as resignation or a valid claim) restores the preceding decision point,
including a terminal-only decision at the root. An off-turn resignation instead
returns before the latest human board move. No inferred pre-import history is
created. At the beginning, or when no eligible target exists, Take back is
disabled. Historical viewing must first explicitly Return to live.

Restoration pauses play and announces **Position restored. Resume play when
ready.** Resume explicitly re-enables the existing native lifecycle; the old
engine reply is never automatically reapplied. Take back retires engine-turn and
analysis execution and invalidates request, process and selection identities.
Obsolete action selection and native read-only inspection markers clear with the
new context. Keyboard focus returns to the live status heading after acceptance.
No confirmation dialog is added.

## Native boundaries and persistence

Chess uses its canonical match command actors and replayed match prefixes. Shogi
uses its accepted native command events. Xiangqi uses committed command boundaries,
not raw event counts, preserving warning/claim and multi-event adjudication
semantics. Rules profiles, counters, known history, root provenance and native
outcomes remain owned by each game.

Accepted observations are retained against their exact original histories. They
remain historical information, including observations on an abandoned continuation.
They do not become pending execution or a current game outcome after restoration
or reopening. Save/Open retains complete accepted evidence, settings and native
history; successful Open creates fresh stopped runtime identities and requires
explicit Resume. Failed admission and file operations preserve the accepted game
and its complete save path.

Capacity admission includes retained abandoned evidence, actual encoded bytes,
observations, native events, desktop controls and bounded replay work. Take back
does not replenish archive capacity or erase accepted observations. The existing
reserved stop/close headroom and resource-limit notices apply. The implementation
and exact version compatibility are recorded with the task's verification evidence.

## Scope

The common shell, game-specific boards and pieces, inspection wording and native
movement inspection are unchanged. This feature adds no variation editor,
computer-versus-computer control, timed suspension policy or rules generalization.
A concrete architectural-health observation for future MSH-106 is that legacy
Chess commands still coexist with the shared token-bound dispatch. Product Take
back deliberately uses the latter; changing unrelated Chess commands is outside
this task.

The low-level Chess `from_record` constructor treats supplied pre-session
`GameRecord` history as an immutable imported root: those events lack desktop
match-request ownership. Take back does not cross that boundary. Normal desktop
New Game and Save/Open retain the complete actor-qualified `MatchRecord` and
preserve the human decisions described above. Shogi and Xiangqi initial native
records already carry accepted command actors. This adds no new import feature.
