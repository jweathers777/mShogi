# Functional Xiangqi evaluator, search and structured child (MSH-97)

Profile remains `mshogi.xiangqi.wxf2018.v1`, edition
`WXF-2018+XIANGQI-P1-2026-10-27`. This delivers a functional M4 baseline, not
measured competitive strength. MSH-54 owns strength/tuning. Native rules,
history, warnings, claims, canonical records and exact proof remain authoritative.
MSH-45/M4 remain open; review106 starts after complete Xiangqi delivery.

## Evaluation

`mshogi.xiangqi.material-advance.v1` has explicit Red/Black totals. One unit is
one hundredth of an unadvanced Soldier. Positive `score_for(side)` favors that
named side, independent of turn. Ordinary search observations use the analyzed
root's native side, never the last mover or an inferred terminal winner.

| Native piece | Value |
| --- | ---: |
| General |0 (native outcomes govern royal loss) |
| Advisor / Elephant |200 |
| Horse |400 |
| Rook |900 |
| Cannon |450 |
| Soldier |100 |

The only positional term is Soldier advancement: `20 * max(forward_rank-3,0)`,
where Red forward rank is the native rank and Black forward rank is `9-rank`.
There are no hidden tables, learned weights, mobility/king-safety terms or
material-based no-win approximations. Initial totals are4800 each. Scores are
heuristics, not WXF points, win probabilities, adjudication or a draw policy.

Actual terminal scores are `+/- (1,000,000 - command_edge_distance)` from the
root side; draws score0. This is native win/loss distance, **not necessarily
checkmate**: stalemate and repetition penalties are losses too. Claim edges
count; resignation already in the source is an actual native root conclusion.
A root conclusion has distance0. Heuristic magnitudes cannot reach that band.

## Search and work

`mshogi.xiangqi.full-width.v1` performs deterministic full-width iterative
minimax, depth1 upward. It maximizes/minimizes using native side-to-move with a
stable root perspective; it never assumes each terminal command changes turn.
No transposition or memoized score cache exists. A board-only key would lose
known history, persistent conduct identities, warnings and claimant state.

Search reconstructs one private native Game from the complete canonical
record; iterations reuse its restored semantic root. Every branch calls
`Game::submit`, then consumes its native
undo even after stop/error. Canonical semantic root equality is verified after
every attempted iteration, including failed/partial ones. The caller's state
and execution context are immutable. Private undo deliberately retires runtime
revisions; those tokens are not semantic equality or reusable execution work.

All complete native legal board actions are searched. A valid **moving side's**
optional natural-count claim is a separate zero-valued choice, examined first,
including at leaves. It is not automatically a draw: a better continuation may
be chosen. Equal values retain the first choice, then native legal-action order.
The host still admits non-turn terminal commands according to native semantics;
ordinary turn search does not choose another participant's voluntary command.
`AutoValidClaims` native submissions retain their actual automatic order and
outcomes. Agreement/resignation are not heuristic engine choices.

The native conclusion is checked before evaluation. A locally legal action
that causes its mover to lose under WXF is scored as that native loss. Native
pending adjudication anywhere discards scored decisions and returns unresolved,
unscored evidence. Capacity/representation/transition failures are errors,
never illegal-move losses or invented chess-family outcomes.

| Setting | Depth | Total admitted visits |
| --- | ---: | ---: |
| Default |1 |128 |
| Configurable ceiling |3 |512 |

Depth must be1..3; visits0..512. Visits count iterative revisits, claim nodes,
terminal nodes and abandoned partial work. Zero work emits no move or score.
A partial iteration publishes only the last complete iteration with the actual
stop reason and total attempted visits. Cooperative stop is polled before
reconstruction, each node and transition. Individual native replay/query work
is bounded by existing core limits; the real child additionally has an
independent watchdog/termination path. Finite work is not a latency guarantee.

## No-win evidence and operational policy

Ordinary search and the structured child execute the record's existing
`sound_fast_only` mode. They retain the root `NoWinAssessment` in telemetry.
A fast miss (`UnknownExactUnavailable`, historical wire spelling) is neither
an ongoing/winnable verdict nor a draw. A heuristic score under this explicitly
limited operational policy is not a completed G-NOWIN adjudication. No implicit
exact graph runs at a root, leaf or replay. Existing unsupported exact-record
restoration stays rejected; modes/limits are not silently upgraded.

`search_requiring_root_exact(game, search_limits, exact_limits, stop)` is the
separate explicit advanced native entry when a host requires the root's exact
answer. It uses native `ExactAnalysis::begin/run/accept`, checking full source
history/context, purpose/target/runtime/limits; `AssessmentEvidence::Exact`
retains the original observation/request/source/report, not merely `assessment()`.
The exact limits and elapsed/work observations are preserved. Descendants still
follow the recorded fast policy; this is not an exact-all-nodes minimax mode.

- Exact positive evidence is an existential natural-board witness, **not a
  forced win**. Ordinary search can proceed; its minimax score does not come
  from the witness.
- Exact negative evidence returns `Decision::ExactNoWin`, unscored/read-only.
  It establishes the proof question but cannot silently install a conclusion
  into the fast-mode canonical Game. The host must retain that proof separately.
- Required exact Unknown, including any resource/witness/deadline exhaustion,
  returns no score or choice and retains the precise reason. Cancellation is
  separately stopped; verifier failure is a concrete unscored error.
- Already concluded actual games take precedence. Exact evidence never
  overwrites resignation, repetition or other native conclusions.

If an experimental outcome genuinely requires unresolved exact adjudication,
the host holds it unresolved/unscored. It cannot use a heuristic fallback to
announce an exact result. The advanced native entry is not a newly advertised
shared engine operation. The console's explicit bounded `nowin` diagnostic is
also available (MSH-96); its observations do not become pending/resumable work.

## Real internal engine boundary

Transport3 and legacy Chess2 remain unchanged. Xiangqi advertises exactly
`EngineTurn` and `MoveAnalysis`. `mshogi internal-search` now runs the real
native Xiangqi search; external UCCI/protocol/matches remain MSH-100. Chess Full
is explicitly unsupported for Xiangqi and never interpreted as native exact
reachability. There is no advertised shared Xiangqi exact-operation capability.

`SharedResult::Xiangqi` retains native `Choice` (move versus claim), `Finished`,
`Incomplete` and `Unresolved`. Failures remain the shared unscored failure
family. Telemetry includes evaluator/search IDs, Red/Black perspective, optional
score, actual completed depth/all visits/PV/stop, and root fast evidence.
Acceptance checks exact whole submitted request: native profile/root/full
history, purpose, target prefix/selection, request/session/process identities
and limits. Read-only analysis uses its explicit result acceptance path; search
never applies its own proposal. Native host admission owns application.

Only replay-validated record metadata is cached for cheap acceptance: side,
conclusion and fast assessment. This is not a score cache or a second rules
engine. RawValue outer/nested parsing and strict native replay remain unchanged.
Missing/duplicate/unknown/conflicting fields do not normalize through a map.
Request1MiB, response2MiB, record-envelope1MiB, diagnostics4KiB remain unchanged.
Actual encoded maximum-ledger and padded-boundary tests still apply; the small
result/PV/one-ply fast witness fit existing limits. No accepted history is lost.

One existing supervisor owns all games/purposes. Cancellation/replacement
retires publication before cleanup, including completed-but-unpolled results;
old identities cannot be submitted again. Queues/diagnostics/child cleanup and
watchdogs are unchanged. Historical selection, undo and reopen require fresh
host identities. A late result cannot become current by returning to an equal
board. No Xiangqi desktop consumer is added; desktop changes only make the new
native result family an explicitly foreign family in Shogi admission.

## Reproduction and evidence

```sh
cargo run --release --locked -p mshogi-search --example xiangqi_baseline
cargo test --locked -p mshogi-search --test xiangqi
cargo test --locked -p mshogi-engine --test xiangqi_routing --test xiangqi_handoff
cargo test --locked -p mshogi-console --test xiangqi_internal
cargo run --release --locked --bin mshogi -- --game xiangqi shell
```

The shell remains the native diagnostic command interface. Programmatic clients
send one strict bounded SharedRequest JSON line to `mshogi internal-search`;
the request supplies the game, so `--game` is not accepted for this mode.
The actual child tests construct canonical records and demonstrate complete
requests and native admission. Current desktop remains Chess/Shogi; launch via
`cd desktop && npm run desktop:dev` with pinned Node24.19/Rust1.90 host. Xiangqi
presentation/archive/product integration belongs to MSH-98 onward.

[Measurements and independent expectations](engine-evidence.md) give hardware,
work/settings/observations, review method and scope. Final tested source and
CI/platform results are recorded in MSH-97, not inferred from another OS.
