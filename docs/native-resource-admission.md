# Compact native record bounds

Tori, Chu and Tenjiku save a complete immutable root followed by actor/command
pairs. Derived effects and outcomes are reconstructed during native replay.
Ordinary command admission does not serialize the history or build hypothetical
resignations to measure file size.

The compact record header and root must fit 64 KiB. Root admission checks this
once. Roots contain at most 256 cells, fixed-size piece fields and counters,
bounded hands or Lion protection, and at most 1,024 source bytes. JSON escaping
can expand those source bytes to 6,144 bytes. The maximum board representation
and metadata remain below 64 KiB. Tests overfill the board with every piece kind,
maximum integer fields and escaped source text to check the wire bound.

Each recorded command, including its separating comma, fits 256 bytes. Commands
contain fixed tags, one actor and a bounded action: Tori has a move or drop, Chu
has at most two landings, and Tenjiku has at most three landings or one igui target.
Wire tests cover each command shape with the longest coordinate and enum fields.
Native legality rejects malformed or longer routes before they enter history.

| Game | Event limit | Maximum compact bytes | Record limit |
| --- | ---: | ---: | ---: |
| Tori | 1,024 | 327,680 | 524,288 |
| Chu | 512 | 196,608 | 524,288 |
| Tenjiku | 1,024 | 327,680 | 1,048,576 |

The upper bound is `65,536 + event_limit * 256`. Compile-time assertions check
that it fits each native record limit. Ordinary commands leave one event for
resignation as required by native admission. Input and output byte limits still
apply at record boundaries. An explicit capacity diagnostic computes the actual
encoded size; it is not part of ordinary command admission.

Chu and Tenjiku retain native replay-work counters used by engine validation.
These bound native work and do not count bytes serialized for capacity checks.
Tenjiku reserves one work unit for resignation. Existing stop checks and bounded
native operations keep replay cancellable. A refusal is never a game result.

A private transaction holds the original local owner and the lengths of
append-only history vectors. It works on a local copy. On failure, it restores
the original owner and truncates appended history before returning. Runtime
identity changes are published only after all fallible checks. Chu reply queries
use the same rollback rule. Undo tokens retain a prefix checkpoint or index;
redo retains the command and stable history nodes. Runtime authority advances
through reversal, so old requests cannot become live again.

Captured public contexts still contain exact history for identity comparison.
Forks and explicit historical records can also copy history. Those operations
are distinct from the private admission and reversal transactions.
