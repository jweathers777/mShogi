# mShogi design baseline 0.1

Prepared on 2026-10-22 from John's product vision, the source review, the technology recommendations he accepted, and his instruction to use the current LinguaSift working model.

This document records accepted product direction and an initial implementation plan. Its decision register distinguishes accepted direction from working defaults and deferred questions. It does not claim that new code, tests, or supported variants already exist.

## 1. Product requirements

| ID | Requirement |
| --- | --- |
| P-01 | Provide reusable, complete implementations of at least chess, shogi, tori shogi, chu shogi, tenjiku shogi, and xiangqi. |
| P-02 | Develop mShogi's own engines that play well, with measurable progress and game-specific strength criteria. |
| P-03 | Provide a console application for interactive use, control by other interfaces, self-play, and matches against other programs. |
| P-04 | Provide a polished desktop application supporting every implemented game on Windows, macOS, and Linux. |
| P-05 | Make engine behavior inspectable and failures reproducible. Verification, diagnostics, and performance/strength measurements are development infrastructure. |
| P-06 | Keep design and product decisions with John in ChatGPT, implementation in Codex or Claude Code, and durable instructions and Backlog.md work records in Git. |

Full product scope is distinct from milestone scope. An early milestone may support only part of a game's rules, provided the coverage is explicit and the application does not present that implementation as complete. Deferred requirements remain requirements.

The initial product can operate locally. A hosted service, mobile application, public online play service, arbitrary user-defined variant language, and distributed training platform are outside the initial delivery plan. They are not prerequisites for the requested desktop product.

## 2. Accepted technology direction

| Layer | Direction |
| --- | --- |
| Rules, positions, search, native console | Rust and Cargo |
| Desktop host | Tauri 2 |
| Desktop frontend | React, strict TypeScript, Vite |
| Board and surrounding interface | SVG board presentation with HTML/CSS controls |
| Local collections and metadata | SQLite when persistence requirements warrant it |
| Learning experiments | Python/PyTorch, with bindings to the same Rust rules implementation |
| First search and learning path | A verifiable conventional search baseline, then learned evaluation; NNUE-style evaluation is the initial candidate |
| Later search research | Policy/value models and MCTS remain available as a separate research path |
| Tracking | Repository-resident Backlog.md records |

This is not a requirement to scaffold every technology in the first commit. Introduce each dependency when its component is implemented. The first executable milestone needs a Rust library and console only.

## 3. Architectural boundaries

### A-01. One authoritative rules implementation

The rules library owns legal complete actions, action resolution, promotion choices, turn progression, and rule-dependent outcomes. Search, console, GUI, match runner, and training tools use it.

The frontend can manage selection and presentation. It must not contain a second implementation of legality. Training code must not implement a separate approximation of the game.

### A-02. Explicit state and complete actions

An action is one complete player choice for one turn. It can require multiple movement steps or distinguish routes with the same starting and ending squares. Drops, promotions, capture choices, and other rule-dependent options must remain representable.

Distinguish the player's action from its rule-derived effects. The rules implementation determines captures and other effects; a client-provided victim list is not authoritative. Resolution can contain ordered internal effects while appearing atomic to callers. Rejecting an invalid external action must leave the state unchanged.

Partial UI selection is not an already committed partial game turn. A match command such as resignation or a draw claim is not automatically an ordinary board move.

The exact Rust types, action encoding, board representation, and trait structure remain implementation decisions. These semantic requirements do not require a universal event interpreter or one board representation for all games.

### A-03. Position, history, and match state

Specify the boundary between the current position, history needed by rules, and match state such as clocks and player assignments. A board diagram alone may not determine an outcome.

Saved-state identity, repetition identity, and search-cache identity are separate concepts. A convenient hash is not a complete semantic specification. In particular, a result that depends on history must not be reused solely because a board key matches.

An undo record restores every affected field and incremental cache. Public saved-game records must not depend on the engine's private undo layout.

### A-04. Ruleset identity and honest coverage

Every implemented rules profile has an explicit identity and version. Record authoritative references, consequential interpretations, and unresolved cases. Do not silently treat an implementation shortcut as the rules of a game.

Rule-dependent legality and outcomes belong to the profile. Do not impose a universal assumption of one royal piece or conventional chess check semantics on every variant.

Distinguish automatic outcomes from claimable outcomes. Keep full adjudication obligations visible even where an early milestone implements a narrower position-level classification.

### A-05. Search and interface isolation

Search uses private working state or equivalent isolation. A search request and every result identify the request and its base position/version. Superseded results cannot change the current game. Cancellation must not corrupt the game or block behind unbounded progress messages.

The desktop application supervises the native engine executable for substantial search work. Quick rules queries may call the same rules library in process. Move generation, evaluation, and individual search nodes remain native; they do not make frontend or Python round trips.

The console remains independently usable. Human interaction and machine protocol modes have distinct output contracts. Diagnostics must not corrupt protocol stdout.

### A-06. Interoperability and persistence

Use standard engine adapters where appropriate, including UCI, USI, CECP/XBoard, and UCCI when a selected opponent requires it. Preserve a structured mShogi interface for complete internal functionality. Design its concrete schema when a real consumer is implemented.

The canonical game record must preserve the rules profile, initial state and necessary history, and complete actions. Imported/exported notation is an adapter. Do not force every game's canonical state into a chess-specific notation.

Use external engines as independent opponents, reference implementations where appropriate, and optional analysis providers. They do not replace P-01 or P-02.

### A-07. Presentation

The desktop UI must eventually accommodate large boards, square and intersection geometry, multi-stage action entry, promotions, different piece sets, readable labels, keyboard play, and useful textual game records.

Validate the real packaged application on all three target systems. Keep engine progress and expensive work off the UI event path. Test a large-board and compound-action interaction early in desktop development.

### A-08. Learning remains an independently measurable track

Establish a correct rules implementation, a search baseline, and reproducible matches before substantial training. Training and deployed inference must agree on ruleset, feature encoding, and model identity.

Start with separate game-specific evaluators and shared experiment infrastructure. Network architecture, model sizes, features, quantization, runtime representation, and compute budgets remain empirical decisions.

Keep Python outside the shipped application unless a later explicit requirement justifies it. Rust bindings and native data generation must reuse the product rules.

## 4. First milestones

### M0. Repository workflow bootstrap

Import this baseline into an isolated rewrite branch/worktree, preserve existing work and legacy history, validate native Backlog.md records, establish the shared agent entry point, and commit a concrete starting point. MSH-1 records this work.

The recommended repository is the existing `jweathers777/mShogi` repository, with a dedicated `rewrite` branch. An isolated orphan worktree is the preferred fresh-tree arrangement when no rewrite already exists. Existing work takes precedence over this default. The initial bootstrap does not change the default branch or erase the legacy implementation.

### M1. Chess rules kernel and verification console

The first implementation milestone establishes a useful correctness loop before search strength and GUI integration add additional causes of failure.

Included scope is an explicit orthodox-chess position profile; FEN parsing/serialization under a documented validation policy; attack detection; legal moves including castling, en passant, and all promotion choices; exact apply/undo; position-level check/checkmate/stalemate classification; perft and divide; and a headless inspection console.

The milestone includes an independently sourced reference corpus, deterministic state properties, a bounded differential comparison, and a small synthetic compound-action exercise that challenges future interfaces without claiming another variant is implemented.

Deferred from M1 are complete draw and match adjudication, clocks, competitive search, external engine protocols, GUI implementation, the remaining game implementations, and neural infrastructure. Record each gap in the rules coverage ledger. In particular, a small insufficient-material recognizer must not be presented as a complete dead-position implementation.

FEN does not become the universal mShogi position format. The chess parser contract must distinguish syntax/structural validation from claims about historical reachability and match-history information.

#### M1 exit criteria

1. The promised move categories and position-level classifications have independent expected fixtures.
2. Legal move sets and perft results match the selected corpus at documented depths.
3. Apply/undo restores the full position, relevant counters and rights, and any derived caches exactly.
4. Fixed-seed legal sequences exercise restoration and round trips; failing cases can be preserved as compact regression fixtures.
5. The console can inspect a position, list legal moves, apply/undo a sequence, and run perft/divide with documented output and errors.
6. The action-boundary review traces a synthetic compound-action exercise to the actual position/transition interfaces. Its reviewed mapping preserves route identity and multiple effects, and any incompatible contract is resolved or explicitly blocks closure. It does not require a universal representation.
7. Actual build/test/smoke evidence exists for Windows, macOS, and Linux. Missing evidence remains visible and blocks the corresponding completion claim.
8. Independent review has no unresolved correctness issue within the promised scope, and another session can reproduce the evidence.
9. The rules coverage ledger clearly separates implemented, deferred, and unresolved requirements. M1 completion is not labeled complete chess or strong play.

Do not invent a nodes-per-second target before measuring the first correct implementation. Record a baseline with its hardware and build configuration.

## 5. Subsequent roadmap

| Milestone | Intended outcome |
| --- | --- |
| M2. Search and match baseline | Complete the needed chess match semantics; add a transparent evaluator, independently checked full-width search, cancellation, replayable self-play and independent-engine matches, and UCI as the first protocol adapter. The bounded match policy is specified by MSH-15; MSH-25 closes the actual integrated M2 path. |
| M3. Desktop vertical slice | Play and inspect a game through the actual native engine; test cancellation/stale results, save/reload, an 8x8 and 16x16 presentation, compound-action selection, and packaged installation on all three systems. |
| M4. Variant expansion and conformance | Complete the remaining games with sourced rules profiles and coverage evidence; validate materially different state/move semantics before deep common-engine optimization. |
| M5. Strength and learned evaluation | Establish per-game opponent pools and strength targets; tune search and evaluation; demonstrate a reproducible improvement from an initial learned evaluator. |
| M6. Further engine research and polish | Deliver required analysis/accessibility, product-quality and distribution polish; keep policy/value search and budgeted learning as optional research. |

The milestone names establish direction, not an obligation to serialize every independent activity. An early bounded desktop or variant-contract experiment can resolve a concrete risk. Do not turn that allowance into an unbounded second implementation effort.

M2 is refined in MSH-15–25 against the actual M1 implementation. The [match/history profile](rules/chess-match-v1.md) records the approved 2026-10-24 policy; MSH-26 supplies an opt-in exact dead-position/side-specific reachability algorithm, with explicit resource uncertainty and unchanged M2 defaults. M2 is closed with its [integration evidence](review/m2-closure.md). M3 is delivered through MSH-27–39 and persistence follow-up MSH-61 against the [desktop contract](desktop/m3-design.md), including all eleven desktop work areas and the historical initial untimed play batch (MSH-27–32). Its [integration verdict](review/m3-closure.md) and MSH-39 record the independent review, actual three-platform application evidence and final closure checks. Next work is M4 scope refinement; all six games and strong engines remain the product target.

### Remaining scope and closure policy (planning correction, 2026-10-25)

Backlog.md's milestone percentage is **Done assigned tasks / currently assigned
tasks**, not a verdict that the milestone's promised outcome is delivered. An
incomplete breakdown can display 100% while substantial scope is absent. M4 did
so when its only assigned task was the completed MSH-26. Preserve that task's
Done status, M4 assignment and evidence; adding missing work corrects the board
without reopening completed work. A zero-task milestone is unplanned work, not
an absence of obligations. Counts are neither effort estimates nor a measure of
how much implementation remains.

The following scope-level tickets represent the accepted remaining roadmap.
They are unassigned **To Do** work, to be refined against the then-current
implementation and recorded design questions before execution. No estimates,
research outcomes, rules interpretations or detailed implementation plans are
approved by creating them. This correction leaves MSH-27–39 and the current M3
implementation batch unchanged and does not authorize starting future games.

| Milestone | Scope and owning tasks | Required closure gate |
| --- | --- | --- |
| M4 | MSH-26: completed chess completeness; MSH-41: shogi; MSH-42: tori shogi; MSH-43: chu shogi; MSH-44: tenjiku shogi; MSH-45: xiangqi; MSH-46: shared six-game integration | MSH-47 depends on MSH-26, MSH-39 and every MSH-41–46 deliverable. |
| M5 | MSH-48: shared reproducible strength/learning experiment infrastructure; MSH-49: chess strength; MSH-50: shogi strength; MSH-51: tori shogi strength; MSH-52: chu shogi strength; MSH-53: tenjiku shogi strength; MSH-54: xiangqi strength; MSH-55: initial learned evaluator, training/export/native inference and demonstrated improvement | MSH-56 depends on M4 closure and every MSH-48–55 deliverable. |
| M6 required product delivery | MSH-57: six-game analysis, accessibility and product polish; MSH-58: release policy and three-platform distribution | MSH-60 depends on M5 closure and MSH-57–58. |
| M6 optional research | MSH-59: policy/value search and budgeted learning experiments | Explicitly excluded from required MSH-60 closure dependencies. Research may remain To Do when required product delivery closes. |

Each M4 game scope owns its complete sourced/versioned profile, authoritative
state/actions/history/adjudication, independent verification, game-specific
library and console interfaces, functional native mShogi engine support,
canonical saved records and actual desktop adaptation. Required rules questions
(D-07) remain in the relevant game task; shortcuts cannot become the profile by
accident. These tickets are complete game deliveries, not movement-only kernels.

Shared boundary extensions, game/profile dispatch, complete-action transport,
engine/session supervision, common record evolution/compatibility, reusable
presentation and cross-game packaged acceptance are owned **once**, by MSH-46.
Game-specific adapters and conformance examples stay with the corresponding game
ticket. Work on shared mechanisms can coordinate with those tickets; MSH-46's
dependencies gate its final integrated acceptance. MSH-47 verifies the combined
result on Windows, macOS and Linux. A shared capability or synthetic fixture is
not evidence that a real game has been delivered.

M4 engine support means legal, history-aware functional search/play, safe
requests/cancellation and product integration. Measured playing strength belongs
to the six explicit M5 game tasks. Targets, opponent pools and fair comparison
conditions remain game-specific D-09 questions; no numeric targets are invented
here. MSH-55 requires an initial learned evaluator's reproducible improvement,
not one predetermined architecture for all six games. Its initial game(s),
features, model/export choices and compute budget remain empirical decisions.
NNUE-style evaluation remains a candidate; optional policy/value/MCTS research
is not a prerequisite for the promised engines or product delivery.

Required M6 product gaps must be inventoried and closed against P-03–P-05;
release licensing, redistribution, minimum OS, signing and update arrangements
remain D-10 decisions in MSH-58. Optional experiments cannot substitute for these
requirements. An M6 board percentage below 100% can coexist with required-scope
closure if only explicitly optional research remains; record that distinction.

A milestone closes only when its **complete promised scope**, required task
criteria, applicable independent review and integrated verification have an
explicit evidence-based verdict in its closure task. Refine dependencies when
scope is split, keeping the closure gate connected to every required deliverable.
Missing game implementations, unresolved required rules or required integration
block M4 closure regardless of its percentage. The same rule applies to required
M5/M6 outcomes. Do not erase gaps by renaming scope, archiving unfinished required
work or moving it into optional research. MSH-40 records this planning correction
and its verification, separately from future product delivery.

### Shogi-first M4 planning refinement (2026-10-26)

MSH-62 supplies the [bounded implementation and rules proposals](m4/README.md).
Standard shogi is the first additional game; the provisional later delivery
order is xiangqi, tori shogi, chu shogi and tenjiku shogi. Independent source
research can proceed separately; an evidence-based order change must be recorded.
This planning batch implements no game and closes no M4 delivery.

The [task/ownership map](m4/delivery-plan.md) preserves MSH-41–47 and all their
original criteria, and adds bounded tasks MSH-63–89. Early shared MSH-69/72/73
are prerequisites for concrete shogi consumers without depending on completed
MSH-41 or MSH-46. Later real-game extensions and final all-game/platform review
retain the full shared integration and M4 closure obligations.

The owner selected option A **SHOGI-P1**, `mshogi.shogi.csa36.v1`, on
2026-10-26 against integration `6db8500e92129b95e502e7b4d5c4a26072e38652`.
Its coherent CSA software-event convention and explicit untimed/import/request
adaptations are accepted; other event conventions remain unselected alternatives.
MSH-67 still requires full legality and its own history/outcome fixtures.
Independent board fixtures/state/legality can proceed without settling every
later game's profile. Existing chess documents, exact history, cancellation and
MSH-61 lossless capacity admission remain compatibility requirements. Current
chess search/archive limits are defaults to evaluate for each new consumer.

## 6. Verification contract

| Evidence category | What it establishes |
| --- | --- |
| Curated rules fixtures | A selected interpretation is implemented correctly in a specific case. |
| State properties | Transitions, reversibility, derived state, and serialization remain consistent. |
| Perft and differential comparison | Move generation agrees with independent expectations over specified positions/depths. |
| Search oracle tests | A search algorithm returns the intended result on independently solvable positions or trees. |
| Performance benchmarks | The effect of a change on time, throughput, or memory under recorded conditions. |
| Match experiments | Playing performance for a particular game, opponent pool, limits, and starting-position distribution. |
| Application integration | Commands, progress, cancellation, state changes, persistence, and presentation remain coherent. |

Perft is not evidence of correct search, complete adjudication, strength, or GUI behavior. A speedup is not evidence of stronger play. Generated tests must not be the only source of expected rule behavior.

Use `cargo test` and deliberately selected fixtures from the first behavior change. Add property testing, parser fuzzing, and Criterion measurements where they address concrete risk. Keep fast local checks separate from deeper verification runs. Record tool versions, seeds, relevant ruleset/profile identifiers, code revisions, and fixture provenance.

For search, use small independently solvable positions or synthetic trees before relying on pruning or transposition reuse. Test score perspective, terminal scoring, and returned action legality. Add targeted regressions when an optimization introduces a new failure mode.

For matches, preserve executable/model identity, parameters, resource limits, opening positions, colors, results, and failures. Define strength separately per game. Where possible, pair starting positions with reversed colors and test against multiple baselines.

For desktop integration, include starting a search, changing the position, cancelling, and receiving a delayed result. Include persistence and compound action entry. Use real application evidence rather than treating a browser-only test as proof of native packaging behavior.

## 7. Development workflow

`AGENTS.md` is the shared execution policy. `CLAUDE.md` imports it. Backlog.md is the work ledger and human-visible board; there is no parallel issue system.

Each task has a concrete goal, scope, dependencies, requirement references, observable acceptance criteria, and expected verification. A worker researches and records an implementation plan when the task becomes active. Completion includes evidence, review results, integration context, and follow-up work.

Use one primary builder and bounded independent review. Run focused verification per change and broader integrated verification at the appropriate boundary. Reuse still-applicable evidence rather than mechanically repeating costly checks.

When code reveals a design problem, report the specific conflict and a proposed resolution. Routine internal design choices are autonomous. Product-scope changes, disputed rule conventions, or public semantic changes return here for a documented decision.

## 8. Decision register

| ID | State | Decision and provenance |
| --- | --- | --- |
| D-01 | Accepted | All six named games, own capable engines, console interoperability, and three desktop platforms, from John's stated product vision. |
| D-02 | Accepted | Rust core/console; Tauri 2, React/TypeScript/Vite desktop direction; native rules reused by learning tools, from the technology discussion accepted on 2026-10-22. |
| D-03 | Accepted | Design in ChatGPT, implementation in Codex or Claude Code, and Backlog.md in the repo, explicitly requested on 2026-10-22. |
| D-04 | Working default | One shared agent policy, disposable bounded sessions, a primary builder, independent review, routine non-destructive Git integration, and repository-resident evidence, carried forward from the requested LinguaSift workflow. |
| D-05 | Working default | Isolated rewrite branch/worktree in the existing repository, preserving legacy history. Confirm actual repository state during bootstrap; do not manufacture an empty starting state over existing work. |
| D-06 | Working default | M0 and M1 sequencing and the initial ticket decomposition in this package. Refine technical task boundaries without weakening their outcomes. |
| D-07 | Deferred | Exact disputed historical rules, versioned conventions, and full conformance criteria for each game. Resolve before implementing affected behavior or claiming conformance. |
| D-08 | Deferred | Specific public API/ABI, board and action encodings, generic trait hierarchy, and complete wire/save schemas. Implement only the needed stable semantic contracts. |
| D-09 | Deferred | Per-game release-strength targets, evaluator architecture, training hardware, and compute budget. Establish baselines and pilot measurements first. |
| D-11 | Accepted | John 2026-10-24: optional core claims with recorded unattended auto-claim policy, FEN-limited known history, sound incomplete M2 dead recognition with MSH-26 M4 completion, explicit unscored mating uncertainty, untimed/increment clocks and ordered replayable match events. Evaluation adjudication stays disabled. UCI plus an independent-engine match and focused MSH-25 integration closure are required in M2. See [match v1](rules/chess-match-v1.md). |
| D-10 | Deferred | Release licensing, redistribution choices, minimum OS policy, and signing/update arrangements. Preserve existing notices; resolve applicable choices before copying restricted dependencies or distributing releases. |
| D-12 | Accepted | John 2026-10-26: bounded M4 planning only, standard shogi first; provisional later order xiangqi, tori, chu, tenjiku with independent source research possible. Preserve complete MSH-41–47 obligations and closed M2/M3; stop for concrete profile discussion before affected implementation. |
| D-13 | Accepted, SHOGI-P1, 2026-10-26 | John explicitly approved option A `mshogi.shogi.csa36.v1` against integration `6db8500e92129b95e502e7b4d5c4a26072e38652`, including untimed local play, advisory/committed declarations, atomic rejection and declared-counter/bounded-history adaptations. JSA professional and other event conventions remain separate unselected alternatives. See [selected profile](m4/shogi-profile.md) and [coverage](m4/shogi-coverage.md); approval is not delivery evidence. |

Accepted records reflect actual owner direction. Working defaults are implementation proposals consistent with it, not fabricated owner decisions. Deferred questions only block work that depends on their resolution. A routine choice within accepted requirements does not need a new approval ceremony.

## 9. References and provenance

- Original project: https://github.com/jweathers777/mShogi
- Current Backlog.md baseline used for this package: https://github.com/MrLesk/Backlog.md/tree/v1.55.1
- Backlog.md CLI reference: https://github.com/MrLesk/Backlog.md/blob/v1.55.1/CLI-INSTRUCTIONS.md
- Backlog.md configuration: https://github.com/MrLesk/Backlog.md/blob/v1.55.1/ADVANCED-CONFIG.md
- Future rule fixtures must add their own precise primary rule references. This design is not a substitute for the rules of any game.
