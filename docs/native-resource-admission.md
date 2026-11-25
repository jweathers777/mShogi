# Native record admission

Games have no synthetic history-work budget and no command, event, ply or
byte limit. Native records, saved documents, console input lines, engine
requests and engine replies have no size limit (MSH-268). Rule-based endings
remain native rules, including Standard Shogi's selected move-count rule.
Operational cancellation or resource refusal is not a result.

Tori, Chu and Tenjiku store an immutable root followed by actor/command pairs.
Effects, history-dependent rights and outcomes are reconstructed by replay.
Limits that do not depend on game length stay: root admission checks its
encoded bytes once, one command has a small fixed size, position strings and
diagnostics are bounded, and runtime counters refuse on overflow. Ordinary
commands do not encode hypothetical endings. A very large or damaged file or
input can use much memory. Cancellation checks and elapsed-time deadlines keep
file validation and engine processes interruptible.

A private transaction holds the original local owner and the lengths of
append-only history vectors. On failure it restores the owner and truncates
appended history. Runtime identity changes are published only after the fallible
checks. Undo tokens retain a prefix checkpoint or index and history nodes.
Forks and explicit historical records can copy history for private analysis
without granting permission to change the live game.
