# MSH-100 independent protocol/runner review and verification

Production source reviewed/tested: `4d6481bffbb7c83ee9461e4036ccaff71057b6ee`.
Initial source was `84625a0`; protocol-transcript correction `8d6a0dd`; final
option-advertisement correction `4d6481b`. Later evidence-only files add no
production behavior. Exact integrated revision and native CI are recorded in
MSH-100 by the sole integrator after both MSH-98/100 reviews pass.

## Method and findings

Independent agent `/root/xiangqi100_review` reviewed the bounded source diff,
AGENTS/task/profile/native consumer requirements, native expected fixtures and
actual evidence. It did not implement this boundary. Review covered native
command admission/history and profile dispatch, explicit unscored results,
current native search limits, exact/fast separation, bounded process lifetime,
fresh-generation retirement, strict record/report parsing, transcript replay,
source/network/license provenance, existing Chess/Shogi compatibility and
instrumentation separation. It inspected the independently built external
source pin and verified twelve source file hashes plus binary/NNUE identities.

Two material replay findings were independently reproduced and resolved:

1. Runtime rejected a late/duplicate bestmove, but replay initially accepted
   an inserted `phase=retired` bestmove in an otherwise valid actual report.
   Token presence also failed to establish response order. `replay_traffic`
   now validates the ordered handshake/options/readiness/newgame/position/go/
   reply/stop/quit phases and rejects duplicate/retired replies, invalid token
   order and unknown directions/phases. Six new negative variants reproduce
   the old gap. The reviewer's original forged reports now fail CLI replay.
2. Removing an advertised configured option from a report still passed replay,
   although runtime rejected unadvertised/unsupported options. Replay now
   applies the same option advertisement name/type/value checks as runtime,
   including the bounded handshake collection. Missing/invalid advertisement
   regressions pass; the reviewer's forged missing-Threads report now fails.

Verification-helper review also tightened final experiment verdicts: a failed
third replay or third game must fail reproduction even when repeat semantics
match. Two mocked negative script journeys verified that guard. Native reply
admission now additionally checks the exact played acknowledgement and native
canonical accepted command/board-action event. The shell already accumulates
errors into a failed exit status; these assertions make the evidence precise.
All five real opponent admission probes were rerun successfully after the
strengthened assertions. No fixture expectation or game rule was changed.

Reviewer final source verdict: **no unresolved material finding**. The reviewer
independently executed the7 actual UCI tests,9 runner tests and2 then-existing
recorded suites, rechecked both forged-report failures, and reran actual own/
external paired4game experiments. Canonical games, conclusions, accepted
replies, context digests, position/go matched retained evidence under documented
finite deterministic settings; PID/timing observations were deliberately not
compared for equality. A third straightforward recorded test subsequently
retains the real own-engine zero-work case and passed the author's full suite.

## Actual local checks

Environment: macOS27 arm64, stable repository Rust, pinned Backlog1.55.1.
The following commands passed against final production source; local output
logs were inspected, not inferred from exit of a later chained command:

```sh
cargo fmt --all
cargo clippy -p mshogi-console -p mshogi-runner --all-targets --locked -- -D warnings
cargo test -p mshogi-console -p mshogi-runner --locked
cargo build --release --locked --bin mshogi --bin mshogi-match
git diff --check
```

The full console/runner run passed **153 tests across25 suites**, including
7 new actual UCI subprocess tests,9 new native runner process/negative tests
and3 retained actual-match replay tests. Existing Chess/Shogi console, UCI,
USI, runner and retained-record regressions passed. The focused actual-process
checks include independent Red/Black stalemate tactics, full warned history
versus the same fresh board, claims/conclusions and same-ply resignation,
stop/held-result replacement/quit, malformed/illegal/null/missing/late/crashed
responses, explicit zero-work/unresolved failures, resource caps/cleanup,
unsupported policy/dialect/counters and strict outer/nested canonical rejection.

Actual playing evidence, producer revisions and artifact hashes are retained
in [opponent admission](opponent-admission.md), [admission JSON](admission.json),
[execution manifest](execution.json) and [real zero-work manifest](no-work-execution.json).
The main matches were produced at `8d6a0dd`; the final change only strengthens
report option replay, so those unchanged executed matches were reused and
replayed under `4d6481b`. All12 successful bounded experiment games replay;
5 native stalemate wins and7 unscored action cutoffs,33 accepted suggestions.
Two real zero-visit own-engine requests at `4d6481b` remain explicit unscored
incomplete work. Failed incompatible-network probes remain labeled historical
failures, never counted as opponent admission. No playing-strength claim is made.

Python helper syntax checks passed; mocked final-replay/final-game failures
both produced failed helper verdicts. The strengthened real admission checks
also passed against the actual current native console. Test scripts and named
hostile child are separately labeled verification artifacts; no product Python
or instrumentation path is added to normal package selection.

Native three-platform CI/integration are deliberately pending at branch review.
The root integrator owns those exact final checks and status completion; this
local macOS evidence does not imply Windows/Linux execution or GUI delivery.
MSH-45/M4 stay open; no later Xiangqi task or architectural review is started.
