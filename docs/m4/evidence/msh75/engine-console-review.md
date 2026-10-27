# MSH-75 independent engine, console and interoperability review

Reviewed revision: `80d6b5cc71c8ceb5bf782d93295797aed1e18d66`, `codex/rewrite`.
Review date/environment: 2026-10-27, macOS 27.0.1 arm64, Rust
`1.89.0 (29483883e 2025-08-04)`. This is a fresh independent AI review of the
integrated delivery, performed separately from its implementers and the root
integrator. No product code, fixtures, task records or commits were changed.

**Verdict within this review's scope: no material defect found.** The engine,
console, canonical game record and USI experiment paths support MSH-41 AC4–6
and their part of AC8. This does not close MSH-75, MSH-41 or M4. Native rules
conformance, desktop/archive UI and final platform verification have separate
review owners; those findings remain decisive for delivery closure.

## Scope and method

Read the owner request, AGENTS.md, Backlog overview, MSH-75/MSH-41, relevant
design boundaries and the engine/console/shared-context/USI contracts. Inspected
actual native search, shared result and wire acceptance, supervisor lifetime,
console admission/restoration, USI worker/parser/publication and runner
admission/cleanup/replay implementations and focused tests. Evaluated the real
native-game → search → structured child → console/USI → experiment runner
composition; completed child task statuses were not acceptance evidence.

Fresh builds and tests used `/private/tmp/msh75-engine-review-target`, isolated
from the root integrator's workspace test/build work. No additional worktree was
created. Backlog CLI was 1.55.0 while the managed instruction block identifies
1.55.1; this reviewer only read task records and made no lifecycle mutation.

Expected search examples reused the previously independently established
MSH-64/67/70 fixtures: initial 30 choices and 30 replies, capture/demotion
arithmetic, optional/mandatory promotion, protected mating Gold drop, same-board
history divergence, continuous-check winner and declaration root/leaf choices.
The private finite-tree tests independently specify minimax values and injected
descendant failures. No expected fixture or opponent was reconstructed/replaced
from the implementation under review. Those search fixtures establish specific
search behavior, not a new independent proof of every rules clause.

Primary protocol text was freshly consulted: [Shogidokoro USI](https://shogidokoro2.stars.ne.jp/usi.html)
and the [original USI text with extensions](https://hgm.nubati.net/usi.html).
Its ASCII transport, start-root/full-suffix positions, complete move/drop/
promotion notation, declarations, bestmove and stop distinctions agree with the
implemented documented subset. The delivered adapter explicitly rejects clocks,
pondering, movetime and searchmoves, and reports tsume as not implemented; this
review does not describe it as an unrestricted general-purpose USI engine.

## Acceptance mapping and integrated findings

| Requirement | Inspected implementation and fresh evidence | Assessment |
| --- | --- | --- |
| MSH-41 AC4: independently usable selected console, actions, history, undo and diagnostics | `main.rs`, `shogi_session.rs`, `record_io.rs`; `common_console` 9 tests, `shogi_cli` 8 tests; direct `--game shogi shell` journey | Supported. Native action admission; complete source/promotion/drop labels; terminal events distinct from board actions; diagnostics explicitly position-only; strict shared record commands preserve failed-load state. |
| AC5: authoritative history and terminal scoring | Search clones complete `Game`, asks native conclusions first, enumerates all native legal actions and submits/undoes native commands; 8 search fixture tests, 2 private-tree tests, actual structured process/USI tests and direct continuous-check terminal execution | Supported. Winner is read from native conclusion, not inferred from mover or alternating negation. Same SFEN can have different recorded-history results. |
| AC5: declaration edges and complete choices | `shogi.rs` minimax and native declaration submission; root/leaf declaration, optional promotion, drop branching and actual USI mapping tests | Supported. Eligible declaration is submitted as a terminal command, including at static leaves; root perspective remains stable. All legal board alternatives come directly from native `legal_actions`, with no selective filter. |
| AC5: limits, restoration, cancellation and completed observations | Root equality after each completed/limited/stopped/failed private iteration; descendant push/pop unwinds before propagating halt; exact work fixtures and supervisor tests | Supported. No completed iteration means no score/choice. Node32 retains depth1 while recording 32 visits. Zero nodes visits nothing. A native transition failure yields a diagnostic; it cannot award a game result. |
| AC5: exact identity and stale rejection | `shared.rs` full request equality, strict `RawValue` wire/native record parsing, shared supervisor monotonic request/generation and one current owner; `shared_internal` 3 real subprocess tests, `shogi_engine` 3, `shogi_protocol` 6 and supervisor unit tests | Supported. Purpose, game/profile/record, event prefix, analysis selection/side/version, process generation and exact limits remain bound. Cancellation/replacement retire even an unpublished/completed response, and semantic undo does not restore request identity. Host command admission remains separately native. |
| AC5: functional play and recorded experiments | Fresh release selfplay and exact admitted external opponent, each six games plus replay; retained report replay and failure fixtures | Supported as functional baseline. Conclusions, action/time cutoffs and failures remain separate; unfinished or failed games have no competition score. No measured competitive-strength claim. |
| AC6: canonical native records and lossless replay | Strict game-selected envelope, native policy/root/event replay and restored undo; direct capture/promote → resign → save → load → print equality → undo twice, plus strict duplicate and no-clobber checks | Supported for console/native game records. Explicit profile/edition/schema, imported unknown history, declared counter, same-ply terminal event and complete actions survived. Desktop v2 archive/document semantics are reviewed separately. |
| AC8: honest coverage and review | Source inspection and tests against the accepted profile; protocol capabilities advertised exactly; retained evidence hash audit | No required engine/console behavior found silently deferred. Material limitations below are explicit and do not turn a functional baseline into a strength claim. Full rules/desktop coverage mapping belongs to the integrated MSH-75 report. |

Important implementation points checked directly:

- `crates/mshogi-search/src/shogi.rs:184–230` keeps immutable caller state,
  private complete history, completed-iteration publication and final root
  equality. `:319–390` scores the actual winner, unwinds descendant transitions,
  checks conclusions before declarations/static evaluation and backs up from the
  stable root perspective. The stop/error private-tree tests exercise failure
  below the root without inventing a public legal mutual-check state.
- `crates/mshogi-engine/src/shared.rs:233–263` accepts only the exact submitted
  request/context and correct native result family. Shogi observation checks
  validate native root side, settings/identities, declared/finished/incomplete
  families and completed work. Acceptance does not claim to be a second legality
  engine; actual game changes still require native submission.
- `crates/mshogi-engine/src/bridge.rs:114–156,210–237` permanently advances
  request/generation identity and suppresses retired publication. The real
  subprocess tests cover cross-game/cross-version replacement, forged echoes,
  historical selections and cancellation.
- `crates/mshogi-console/src/usi.rs:256–336` checks completion identity/version/
  native context/exact limits, privately re-admits its proposal, publishes `win`
  only for the native declaration and maps only actual checked no-action loss to
  terminal `resign`. Other unrepresentable native terminal states fail explicitly.
  Terminal scores are not mislabeled as USI mate and declaration tokens are not
  injected into the board-action PV.
- `crates/mshogi-runner/src/shogi.rs:807–1011,1045–1106` admits into a private
  candidate, sends native-derived gameover notices, retires/drains/reaps before
  committing, rejects missing retirement EOF/duplicate late reply and distinguishes
  native conclusion from experiment limits/failures. Offline replay independently
  replays accepted complete commands and checks native conclusion/score against
  captured context/process/command/reply evidence. It does not launch an opponent
  or certify competitive strength.

## Fresh execution

All listed commands ran against the reviewed revision. There were **68 passing
focused Rust tests**, with no failed/ignored tests in these selected runs:

```sh
cargo test --locked --target-dir /private/tmp/msh75-engine-review-target \
  -p mshogi-search -p mshogi-engine -p mshogi-console -p mshogi-runner \
  --test shogi --test shogi_protocol --test shogi_engine --test shared_internal \
  --test common_console --test shogi_cli --test usi --test shogi_runner \
  --test shogi_recorded
# 63 passed: console34, engine6, runner15, search8
cargo test --locked --target-dir /private/tmp/msh75-engine-review-target \
  -p mshogi-search --lib shogi::tests
# 2 passed, 4 unrelated tests filtered
cargo test --locked --target-dir /private/tmp/msh75-engine-review-target \
  -p mshogi-engine --lib
# 3 passed
cargo build --locked --release --target-dir /private/tmp/msh75-engine-review-target \
  --bin mshogi --bin mshogi-match
# passed
```

The `shogi_recorded` test replayed all eight retained supported reports (36 games)
and explicitly rejected the historical pre-fix incomplete-retirement report.
It also verified that the historical counterexample's underlying native game is
still independently inspectable. This preserves failure history rather than
presenting the pre-fix report as a current success.

[Direct journey](engine-console-journey.json) retains exact subprocess arguments,
input, stdout/stderr and exits for fresh console save/reopen/undo and structured
engine execution. It checks capture of a promoted pawn into an unpromoted pawn
hand, rook promotion, supplied counter17→18, resignation at unchanged board ply,
canonical re-export equality and restored undo. Initial read-only analysis
returns `1g1f`, depth2, 962 visits with exact echoed selection/context. Node32
retains depth1, zero-work is incomplete/unscored, and the known continuous-check
terminal with Sente as checker scores **+1,000,000 for root Gote**, its actual
winner. Duplicate nested `schema_version` is rejected without changing the
current game; an existing file cannot be replaced by `record save`.

[Execution manifest](engine-console-execution.json) retains commands, actual
binary SHA256, settings, environment, test counts and experiment summaries.
Fresh configs only changed the experiment ID and native executable paths from
the documented retained configs to the isolated fresh build; all work bounds,
opening definitions and opponent settings stayed unchanged. Each actual command
and fresh config is reproducible from the full retained report:

```sh
/private/tmp/msh75-engine-review-target/release/mshogi-match shogi-run \
  /private/tmp/msh75-engine-review/selfplay.config.json \
  docs/m4/evidence/msh75/engine-console-selfplay.report.json
/private/tmp/msh75-engine-review-target/release/mshogi-match shogi-replay \
  docs/m4/evidence/msh75/engine-console-selfplay.report.json
/private/tmp/msh75-engine-review-target/release/mshogi-match shogi-run \
  /private/tmp/msh75-engine-review/opponent.config.json \
  docs/m4/evidence/msh75/engine-console-opponent.report.json
/private/tmp/msh75-engine-review-target/release/mshogi-match shogi-replay \
  docs/m4/evidence/msh75/engine-console-opponent.report.json
```

Both reports contain two standard-root action-limit stops with null native
conclusion/score, two native Sente mating-drop wins and two native Sente
entering-king wins. All 12 games retained complete traffic and no execution
failure. Both paired assignments were exercised. Every fresh canonical game
matched its respective retained MSH-71 report; this is an observation of these
runs, not a promise of external deterministic reruns.

The external process was **the same previously admitted candidate**, verified
before launch: YaneuraOu commit `0f421391ae311df9df57dcfcb2b4a78a1972bdf8`,
tree `0844b429c4fb89e0c4f9160aa65447dcc17bcd58`, executable SHA256
`74eded93711841dbdef41c0d38c8b85de9af77b9925274269c7b42a6d3987f86`.
`git status --short` showed only its untracked built executable. No replacement,
rebuild or source modification was performed. Reports capture its actual
MaterialLv1 identity and advertised/configured options. Its decisions are
external interoperability observations; native mShogi admission owns outcomes.
Artificial decisive opening diagrams and two eight-action initial games are
functional smokes, not fair competitive-strength experiments.

[Provenance audit](engine-console-provenance.json) freshly hashed every entry in
`docs/m4/usi/execution-manifest.json`: all nine recorded console/runner source
files and all eight retained reports still match exactly at the reviewed HEAD.
The original binary hashes remain historical; fresh binary hashes are recorded
separately, not substituted into the old manifest. Historical failed fetch,
unsupported-reference-domain, retired-reply and pre-fix cleanup attempts remain
historical evidence in their original locations.

## Limits, failures and closure boundary

No MSH-41 blocker or required repair was found in this bounded review. No product
change was requested on aesthetic grounds. The following remain explicit limits:

- Search is material-only deterministic full width, depth1–4/nodes0–20,000 with
  measured defaults2/5000; MSH-50 owns competitive strength.
- USI exposes the documented finite untimed subset. `go infinite` holds the
  bounded actual result until stop; it does not imply unbounded work. No-result
  work/failure has no fabricated resignation, move or score.
- This reviewer freshly executed native tests and subprocesses on **macOS arm64
  only**. Windows/Linux opponent execution, fresh platform GUI interaction,
  desktop v2 archive capacity proof, complete rules differential auditing and
  full-workspace formatting/lint/CI were **not run by this reviewer**; their
  independent owners/root integrator must report their actual evidence.
- AI source review, independently derived fixtures, actual native subprocesses,
  external-opponent runs, platform CI and owner visual approval are distinct
  evidence categories. None is upgraded to another by this verdict.
