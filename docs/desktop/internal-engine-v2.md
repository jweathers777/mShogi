# Bundled engine interface v2 — explicit execution purpose

MSH-33 extends the [first-batch boundary](internal-engine-v1.md). The bundled
`mshogi internal-search` remains one strict JSON request/response per process;
`mshogi uci` is unchanged. Internal v2 requires `protocol: 2` and a tagged
`purpose`, alongside nonzero increasing request/process identities, session
version, exact canonical GameRecord and deterministic `limits: {depth,nodes}`.
Old/missing-purpose envelopes fail explicitly; this is not a UCI change.

| Purpose | Execution | Host acceptance |
| --- | --- | --- |
| `engine_turn` | Existing bounded native search | Exact outstanding engine-turn context only; apply a legal move/claim through ChessMatch |
| `move_analysis` | Same native search on exact target/history | Separate `accept_analysis`; suggestions/observations only, no game command |
| `full_reachability` | Advanced exact cooperative Article-3 mating reachability | Separate analysis acceptance; proof/unknown only, never change the live match |

Analysis purposes include native target provenance: `selection_id`, `ply`,
`session_version`, human-readable `label` and side-to-move `turn`. The request
also carries the entire target record. The child validates the record/target;
the host checks the exact pending envelope, active selection and current context
again before accepting. A matching FEN alone is insufficient. The frontend
cannot provide its own target record or direct an analysis response into play.

Move-analysis limits are depth 1–8 and at most 5,000,000 attempted nodes; zero
nodes explicitly permits incomplete work. Full purpose adds `mate` (`either`,
`white`, `black`) and mandatory `limits: {max_positions,max_transitions}`, capped
at 100,000 positions/1,000,000 transitions. Its outer minimax work limits are the
explicit non-executed sentinel `{depth:1,nodes:0}`; no minimax observations are
claimed for that operation. Zero Full budgets are valid bounded unknown work.
Every configuration is retained in the exact response context.

All reconstruction/replay and expensive Full solving occurs inside the supervised
child. Live desktop sessions remain M2. A Full GameRecord is explicitly rejected
before replay rather than downgraded. Advanced Full analysis uses the exact M2
history as provenance, then asks the existing complete native Article-3 solver
about its position with the requested cooperative target and limits. This does
not alter the live adjudication mode or apply a discovered proof as an outcome.
Cooperative reachability is distinct from forced mate/minimax and ignores optional
claims and hypothetical future repetition/50/75-move conclusions by its core
profile. A possible result supplies a legal cooperative witness; impossible is
an established proof; exhausted resources return unknown with actual work.

`EngineResult` retains move, claim, finished, incomplete and failure, and adds
`reachability: {result: MateReachability}` using the core possible/impossible/unknown
encoding. Ordinary suggestions keep real completed-iteration depth/nodes/PV and
root perspective. A partial completed iteration remains labeled with its actual
stop reason. No result can imply progress that was not reported natively.

One bridge owns all execution. Starting/replacing analysis retires previous turn
or analysis acceptance and pauses play. Completion/cancellation does not resume
play. Explicit Resume retires analysis and starts a fresh eligible engine turn,
or re-enables human play. Finishing analysis of a concluded game cannot reopen
it. New game and resignation invalidate analysis. Cancellation suppresses even
already published obsolete completions. Wrong-purpose result types fail safely.

The existing 1 MiB request/2 MiB response/4 KiB diagnostic caps, 15-second process
watchdog, bounded completion/pending slots and direct-child cleanup policy remain.
These also bound potentially expensive reconstruction and Full work. A killed
analysis has no final solver counts: it is cancelled/unknown with absent final
observations, not a fabricated zero-work result. Watchdog/process failures remain
distinct diagnostics and cannot become chess conclusions. OS scheduling is not a
hard realtime guarantee; graceful-exit limitations remain as documented in v1.

Public-controller actual-process tests are in
`crates/mshogi-console/tests/desktop_analysis.rs`; protocol/session tests exercise
purpose isolation, exact history, limits and stale work. External package smoke
runs UCI, engine-turn, read-only move-analysis and bounded Full queries on each
native platform. Frontend and actual packaged GUI evidence remain separate.
