# Desktop Take back

**Take back** is available in Game Actions for all six games. The game session
finds the target and restores the exact command prefix in one admitted
transaction. Return to live before using Take back from a historical view.

In human-versus-engine play, Take back returns to the latest eligible human
decision. A completed human move and its engine reply are removed together.
With the reply pending, Take back retires it and removes the human move. The
engine's opening move stays when the human plays second. Repeated Take back
walks earlier eligible human decisions. A terminal decision by the human on
their turn can restore the preceding decision point, including at the root.
An off-turn resignation returns to the latest eligible human board move
when one exists.

In [two-human play](two-human-play.md), Take back restores the prefix before
the last complete game command, including a terminal command. A command can
produce multiple events. For example, Xiangqi warnings stay grouped with their
causing command. Chess uses accepted match command actors and preserves
known position history in the imported root.

The accepted restoration cuts the command list and pauses play. It clears
obsolete selection and inspection, retires engine turns and analysis, and
focuses the live status. Select **Resume play** to start fresh work.
Take back is disabled when the record has no eligible target.

Session observations remain visible as historical results while the document
is open. The [saved document](document-v1.md) contains the surviving game
history and settings. A failed admission preserves the current game and
save path.
