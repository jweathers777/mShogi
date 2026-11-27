# Two-human desktop play

In New game, set **Players** to **Two humans**. Select any of the six games and,
for Tenjiku Shogi, one of the two rules profiles. **Bottom side** sets the board
orientation. Both sides remain under local human control; the board does not
turn after each action. Human-versus-engine play remains available in Players.

The player to move can select a complete action. This includes drops,
promotion choices and compound routes where the selected game permits them.
The game controls legality and outcomes. There is no automatic engine
turn in a two-human game. Both player areas show Human.

Resign names the side to move. Draw responses, where available, belong to the
other side of the pending offer. Take back restores the position before the
last complete game command, including a terminal command. It then pauses
play. Select Resume to continue. One game command can contain several events;
Take back does not split such a command.

Save/Open preserves the selected mode and complete game history. Open leaves
play stopped. History inspection is read-only. Return to live before changing
the game. Analysis is optional, starts only on request and does not play its
suggested action. New game, Open and Take back retire old execution contexts.

This mode is local and untimed. It adds no network play, clock or automatic
engine-versus-engine setup.
