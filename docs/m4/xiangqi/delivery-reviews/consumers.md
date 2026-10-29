# Fresh consumer/execution assessment

Reviewer: `/root/review103_consumers`; revision541fc303,2026-10-29. Fresh
read-only source/test/report audit, no edits. Examined search/evaluation,
engine typed acceptance/server/wire/bridge, native shell, UCI worker and runner
admission/replay/cleanup rather than relying on completion reports.

```sh
cargo test --locked -p mshogi-search --test xiangqi -p mshogi-engine --test xiangqi_routing --test xiangqi_handoff -p mshogi-console --test xiangqi_cli --test xiangqi_uci --test xiangqi_internal -p mshogi-runner --test xiangqi_runner --test xiangqi_recorded
```

PASS63 tests across8 suites: search13,routing7,handoff6,actual shell11,UCI7,
internal child7,runner9,retained reports3. Includes real hostile-process cases.

Supported MSH-45 AC4/5 boundaries:

- Console native complete batches, command-specific admission, exact undo,
  canonical Save/Load and explicit bounded `nowin` diagnostic. Machine UCI and
  structured entry points remain separate from human shell output.
- Native Red/Black evaluation and full-history minimax, complete actions and
  optional moving-participant claims, exact semantic root restoration and no
  board-only score cache. Same-board warned versus fresh history changes actual
  choice/value. Defaults depth1/128 visits, ceilings3/512 remain measured limits,
  not strength or latency claims.
- Fast evidence stays Fast, including Unknown. Advanced native
  `search_requiring_root_exact` retains original exact source/request/report.
  Negative proof is read-only/unscored; positive witness is not forced win;
  incomplete/cancelled/failed work supplies no fallback decision.
- Shared child advertises only EngineTurn/MoveAnalysis. Whole request equality
  includes history,purpose,target,request/session/process and limits. RawValue
  native parsing, one supervisor, cancellation/replacement/retirement and
  cross-game/reopen/selection tests preserve freshness.
- Admitted Pikafish UCI0–9 only; actual tests cover native stalemate tactics,
  warned histories, bounded infinite/stop,replacement,zero work and unsupported
  controls. No implicit Fairy/UCCI conversion or exact/Chess Full operation.
- Fresh child per runner turn, full canonical digest/event prefix/board history,
  native proposal/saveability admission, bounded stop/quit/EOF/exit before
  acceptance; replay validates actual ordered traffic and cleanup.

Retained reports replayed now:12 own/Pikafish/repeat games,33 accepted actions,
5 native stalemate wins and7 unscored action cutoffs; incompatible-network
failures and2 actual zero-work cases remain explicit. These are historical
actual executions newly replayed, **not a fresh opponent rebuild or match**.
The independently built playing opponent pin
`4c17cee11f888ae1d48a9494f2e2239f019f0a1f` and its matching weights/build/licenses
remain distinct from the earlier board-only comparator pin. Its outcome helpers
are excluded as WXF authority. See `protocol/opponent-admission.md`, source/
admission/execution manifests and preserved full reports.

No unresolved material consumer defect found. MSH-54/M5 owns competitive
strength. External runner's untimed auto-valid-claims/fresh-per-turn/32-action
domain and explicit rejection of unrepresentable live command histories are
documented restrictions, not dropped native history. Original100 Linux GUI
driver failure remains separate from protocol results. Historical consumer
documents' older toolchain/future-consumer sentences are task-time context.

Future106 questions: shared supervisor/envelope invariants already provide real
reuse. Compare telemetry validation and runner protocol-phase/replay/cleanup
policy for genuine shared semantics, preserving native scores/claims/adjudication
and admitted dialects. Separate small rendering loops do not justify universal
board or rules machinery.
