# Native record admission

Games have no synthetic history-work budget or arbitrary command/event quota.
Rule-based endings remain native rules, including Standard Shogi's selected
move-count rule. Operational cancellation or resource refusal is not a result.

Tori, Chu and Tenjiku store an immutable root followed by actor/command pairs.
Effects, history-dependent rights and outcomes are reconstructed by replay.
Input and output byte limits apply to actual encoded records at the record and
document boundaries. Root admission checks its encoded bytes once. Ordinary
commands do not encode hypothetical endings or predict future size from a
per-command allowance. Desktop admission checks the resulting document before
publishing a change. Cancellation checks and elapsed-time deadlines keep file
validation and engine processes interruptible.

A private transaction holds the original local owner and the lengths of
append-only history vectors. On failure it restores the owner and truncates
appended history. Runtime identity changes are published only after the fallible
checks. Undo tokens retain a prefix checkpoint or index; redo retains the command
and history nodes. Forks and explicit historical records can copy history for
private analysis without granting permission to change the live game.
