# Xiangqi complete-delivery review — MSH-103

Review date: 2026-10-29. Audited integration source:
`541fc303fdeaf5017948d14ddf92bf5efee5ab3f`, `codex/rewrite`.
**Verdict: MSH-103 remains In Review**, blocked by P2 finding F01 / MSH-114.
The independent delivery assessment is recorded, but complete-game acceptance
is not granted while the control-limit message invents an observation count.
No production correction or follow-up implementation was started in this task.

Review integration commit: `afd20981a04e88833f214010689cf885ac6eef77`,
`codex/rewrite`; subsequent handoff edits record this actual revision and
remove review-file EOF whitespace. All changes remain documentation/Backlog only.

This is a source, fixture, execution and product-evidence review, not a new
rules implementation or a playing-strength assessment. MSH-45 remains its own
reconciliation task; this review does not close it, M4 or any shared integration
gate. Earlier dossier statements about unstarted consumers are historical
handoffs, superseded by the completed task records and this current assessment.

## Review method and authority

The primary integrator inspected the accepted design, original MSH-45 AC1–8,
MSH-103 and its seven completed dependencies (76,84,96,97,100,102,105), current
desktop source, actual packaged screenshots/accessibility sidecars, package
manifests and CI. Three fresh independent read-only agent sessions separately
examined native rules/history/proofs, execution/console/matches, and native
session/archives/Take back. Their source inspection and newly executed tests
are recorded below; this is independent agent review, not an external arbiter
ruling or a new independent GUI execution campaign.

Rules authority is the selected [profile](profile-proposal.md),
[owner decisions](owner-decisions.md), official bilingual WXF2018 and the
individually reconciled [source clauses](sources.md). The [source manifest](source-manifest.json)
pins both rulebooks. No blanket Chinese-language precedence, AXF/CXA hybrid or
reference-engine adjudication is adopted. The independently expected fixtures
precede comparator observations; a passing differential comparison alone is
not rules conformance. Historical interpretations and expectation corrections
remain available rather than being rewritten to resemble current results.

## Original MSH-45 acceptance matrix

The verdict concerns the **game-specific** obligation in each original
criterion. Later common integration, distribution and measured-strength work
is listed explicitly. A source pointer alone is not execution evidence.

| Original criterion | Concrete implementation and independent expectation | Execution/product evidence | Verdict |
| --- | --- | --- | --- |
| AC1 Complete named, sourced rules profile | `mshogi.xiangqi.wxf2018.v1`, edition `WXF-2018+XIANGQI-P1-2026-10-27`; native `xiangqi::{legal,conduct,game,assessment,exact}.rs`; clause matrix below; independently reviewed board/history/precedence/count corpora | MSH-91/93/94 source/fixture reviews; MSH-105 goal and completed-boundary reviews; fresh native review below | Supported selected rules contract |
| AC2 Authoritative reusable library, complete actions, atomic rejection, exact reversal and distinct identities | `xiangqi::{action,legal,game,record,exact_request}.rs`; typed native dispatch; full context versus local position/repetition/proof keys; `preceding_playable_decision` and committed-prefix restoration | `xiangqi_board`, `xiangqi_history`, `xiangqi_game_boundary`, `xiangqi_inspection_prefix`, `xiangqi_exact`; native handoff84, routing95, session98 and Take back113 evidence | Supported authoritative native boundary |
| AC3 Independently expected fixtures, transitions/history, reversal, records, perft/divide and audited comparison | [Fixtures](fixtures.md), [board design](board-fixture-design.md), [local legality](local-legality-evidence.md), [history evidence](history-evidence.md), [reference artifacts](reference/); board-only comparator domain distinct from WXF outcomes | Frozen complete44 initial choices, both-side geometric cases, deterministic sequence reversal, shallow perft/divide, admitted pinned comparator; source-derived conduct/count/precedence histories; exhaustive independent small exact universes | Supported independent conformance evidence |
| AC4 Standalone console and diagnostics | `mshogi-console/src/xiangqi_session.rs`, native complete command batches, undo/history/status, strict canonical save/load, explicit bounded exact diagnostics; human shell separate from machine UCI/internal entry points | `xiangqi_cli` actual subprocess fixtures; [console contract/review](console.md); current-source independent subprocess rerun below | Supported actual standalone console |
| AC5 Own functional evaluator/search, full context, bounded execution and real matches | `mshogi-search/src/xiangqi.rs`, `mshogi-engine/src/{xiangqi,shared}.rs`, console `xiangqi_uci.rs`, runner `xiangqi.rs`; native full-history child requests, no score cache/fallback | [Engine evidence](engine-evidence.md), [protocol contract](protocol.md), independent [playing-opponent admission](protocol/opponent-admission.md), recorded own/external matches and deterministic native replay | Supported functional baseline; competitive strength remains MSH-54/M5 |
| AC6 Canonical lossless records and desktop saves | Native schema1 record, envelope1/transport3; desktop schema2 journal and native replay, exact event prefixes, settings, observations, retained Take back lineages; duplicate-preserving RawValue dispatch | MSH-99 measured admission/strict replay/atomic file tests; MSH-102 actual Save/Open/capacity/failure journeys; MSH-113 stopped reopen and retained-history tests/captures | Supported lossless native/game-specific archive boundary; shared future codecs remain MSH-46 |
| AC7 Actual desktop play, analysis, actions/outcomes/recovery/documents/history and usable game presentation | `ProductApp`, `ApplicationShell`, `XiangqiApp`, `XiangqiBoard`, `NativeSessionPanels`; native session, inspection, archive and supervised engine | Actual normal and labeled QA macOS packages101/102/113, source/test/artifact inspection below, focused frontend rerun; normal/minimum/intermediate sizing provenance retained | **Blocked by F01/MSH-114** inaccurate capacity reason; other inspected game-specific macOS journeys supported. Final all-game three-platform GUI acceptance remains MSH-88/46, not inferred from CI |
| AC8 Complete ledger and independent delivery review, no required behavior demoted to M5 | This matrix, clause matrix and independent source/test/evidence assessment; [coverage](coverage.md); explicit remaining gates and diagnostic observations below | Review records and exact provenance below | Review recorded; full acceptance blocked by F01/MSH-114 |

MSH-45's eight criteria and all existing dependency sets remain unchanged.
MSH-46 owns shared six-game integration; MSH-88 actual final all-game packaged
Windows/macOS/Linux acceptance; MSH-89 independent shared-contract review; MSH-47
M4 closure. MSH-106 is future diagnostic work after MSH-45. MSH-41/75, M2/M3,
MSH-61's admission guarantee and all six promised games remain intact.

## Consequential selected-profile obligations

| Obligation / primary authority | Native implementation and independent fixture | Verification and boundary |
| --- | --- | --- |
| WXF1–2: setup, every piece, capture, turn, royal safety | `mod.rs`/`codec.rs`/`legal.rs`/`action.rs`; board F001/F004–F008, both owners, horse leg/elephant eye, palace/river, cannon screens, facing Generals, pins/evasions, mate and stalemate | Complete action sets and exact apply/undo in `xiangqi_board`; independent perft/divide and reference artifacts. Ninety intersections; no promotion/drop/hand/declaration invented. Structural diagram admission is not historical reachability. |
| WXF3.1A: checkmate and no-legal-action loss | `local_status`, native game automatic adjudication; source-derived terminal board fixtures | No General-capture continuation; both checkmate and stalemate are native losses. Same diagram's local action query is distinct from live WXF command permission. |
| WXF3.1C/3.2B/19.8: repetition thresholds and G-CYCLE | `game.rs` repetition anchors, persistent piece identity, warnings and native event order; F002/F022–F026 and `history-cycles.json` | Selected third-occurrence classification/warning, fourth continued responsibility loss or applicable draw; contiguous mixed/overlapping histories, equal faults, varying/retiring warnings and exact same-ply prefixes. No generic threefold shortcut. |
| WXF19–20/chapter5: check, mating threat, chase, exchange/block/offer/idle, roots and exceptions | `conduct.rs`; `history-conduct.json`, `history-mutual-check.json`, source-derived board observations; [independent source review](history-reviews/independent-source-review.md) and [conduct review](history-reviews/conduct-source-review.md) | Real legal protection versus pinned/apparent roots, persistent victim, General/Soldier and uncrossed-Soldier exceptions, protected Rook/Cannon/Horse cases, counter-chase frequency, mixed labels. Official38 alternating and109 simultaneous threat/chase remain distinct; correction trail for109/110 preserved. Comparator helpers are excluded as outcome authority. Permitted natures need not expose a fabricated exact Idle/mating-threat telemetry classification; the implemented contract is their selected adjudicative consequence. |
| WXF8/3.2D: natural count and claimant | `NaturalCountEligibility`, known capture window and per-side checks; F003/F017/F027, `natural-count.json` |100 total noncapture plies; only claimant checks above10 deducted; no Pawn reset or invented imported prior history. Advisory opportunity is not conclusion. Native formal claim/audit and recorded `auto_valid_claims` policy remain distinct, Red-first when both eligible. |
| Native command/event precedence and untimed adaptations | `Game::submit`, `history-precedence.json`, source/owner profile ordering | Admitted action first, native board terminal before repetition/no-win, automatic valid claims afterward; serialized explicit participant resign/offer/accept/decline/claim. Terminal actor semantics are independent of board-turn permission; invalid requests reject atomically and claim audits retain reserved capacity. Physical tournament/clock penalties remain outside the approved untimed software profile. |
| WXF3.2A / G-NOWIN quantifier and positive goal | `assessment.rs`, `exact.rs`, `NATURAL_WIN_GOAL`; F016/F028, F002, independently reviewed exact goal/fixtures | Existential finite ordinary board-action path to3.1A mate/stalemate only; not forced win. Conduct penalties, resignation, agreement, time/fault and the tested draw do not create a positive witness. Full known history remains actual-game authority. |
| Fast versus exact versus Unknown/failure | `NoWinAssessment`, `ExactVerdict`, `ExactRequest`/`ExactAnalysis`; `xiangqi_exact` | Fast sound invariant/actual witness or explicit Unknown. Exact closed graph gives proven no-win; replayable finite terminal path gives positive witness; exhaustion/cancellation gives Unknown and invariant/input failure is separate. No heuristic, fast miss or incomplete work becomes a draw/win/loss. |
| Explicit bounded/terminable exact execution | `exact.rs` cooperative deadline/resource/cancellation checks; `exact_request.rs`; console exact worker; native `search_requiring_root_exact` API | Actual105 negative/positive/cancel/resource fixtures and20k/100k measurements; ordinary search/render/reopen stay `sound_fast_only`. Exact is read-only proof evidence, not a desktop live-game conclusion command. Default20k states/200k edges/250ms/4MiB/4096witness plies; request records actual limits/traversal. Larger positions may honestly remain Unknown. |
| Root, history, version and restoration identity | `PositionRoot`, native `GameRecord`, `CanonicalRecord`; envelope and transport parser; desktop schema2 journal | Imported FEN starts known history and preserves declared counters/provenance; native replay checks duplicated derived events/conclusions. Profile/schema/transport are separate explicit identities. Unsupported `bounded_exact` restoration rejects; no silent downgrade. |
| Saveability, cancellation and exact historical observations | Candidate archive admission, shared full request equality and retired execution floor; native exact observation validation; Take back journal | Actual complete bytes, commands/events/controls,256 observations, weighted replay and stop headroom charged before commit. Saved exact reports preserve target prefix, goal/limits/work/proof/witness/Unknown/cancel/failure; remain historical and cannot conclude a reopened game. Undo/selection/reopen retire work even when diagram or canonical bytes repeat. |

## Desktop source and actual product assessment

`XiangqiBoard` renders the native90-point topology, owned palace diagonals and
river separation. Its coordinate-to-SVG projection and native-kind-to-artwork
mapping are presentation; choices and movement inspection come from Rust.
Red/Black artwork remains upright when flipped. The accepted dark shell,
document/header/status hierarchy, Analysis→History→Actions→Technical ordering,
upper/lower player areas and bottom live/historical footer are common.

Pointer/Enter/Space submit only a complete native choice; an occupied legal
destination retains capture precedence over inspection. Opponent inspection is
non-actionable, token-bound and independently retired on context/orientation or
selection change. Visible language is the shared product convention
`Inspecting {side} {piece} · Esc clears`; accessible announcements retain the
read-only meaning. Arrow traversal, Escape origin focus, roving tab stop,
polite status announcements and modal inert/focus behavior have focused tests.
No TypeScript legality or outcome engine exists in this path.

Source inspection covers `useGameSession` monotonic snapshot acceptance and
captured-token commands; `ProductApp` document-replacement/quit guards;
`NativeSessionPanels` purpose-specific read-only analysis, real telemetry,
history selection, retry/Resume, capacity and file errors; and
`TakeBackControl` native one-command restoration, status focus and revealing
the existing Resume control. Historical event selection is separate from
changing live state. Atomic native intra-command prefixes are inspectable but
explicitly not analyzable; committed prefixes have their own exact history.

Actual packaged evidence was inspected, including pixels and AX sidecars:

- [MSH-101 normal/minimum/intermediate visual checkpoint](../evidence/msh101/visual-checkpoint-2026-10-28/README.md)
  and [completion](../evidence/msh101/completion-2026-10-29/README.md): native
 1180×820 normal frame,760×650 minimum and960×740 intermediate packages,
  complete board without scrolling/zoom, both assignments/orientations,
  pointer/keyboard captures and inspection, common Chess/Shogi shell comparison.
  Minimum/intermediate artifacts are explicitly normal-feature packages with
  size/name/ID overrides, not successful runtime resizing of the normal artifact.
  Board geometry/CSS/shell are unchanged; the later board diff changes shared
  visible inspection wording only. Initial raw Debug history was corrected by
  the native presenter; earlier screenshots retain that historical finding.
- [MSH-102 journeys](../evidence/msh102/completion-2026-10-29/README.md): normal
  play/analysis/historical-prefix/Return-to-live/Resume, Save/Open/failed files,
  unsaved guards; separate native-fixture QA warning/fourth loss/count/agreement/
  resignation/fast proof/failure-retry/incomplete work and delay-wrapper
  cancel/replace/cross-purpose journeys. The256th observation is accepted,
 257th refused and the whole document saved/reopened; Save does not reset
  structural capacity or hide the warning. No invented telemetry is used.
- [MSH-113 final packaged evidence](../../desktop/evidence/msh113/README.md):
  final normal Xiangqi engine opening, human pair, Take back preserving the
  opening, stopped Save/Open, same-ply resignation recovery and keyboard
  Take back; separately labeled QA pending-process retirement beyond the
 8-second launch delay. Native tests, rather than a screenshot, establish
  rejection of injected late results. Normal package excludes QA entrypoints.

The root review reused these actual executions rather than generating new
screenshots or presenting a web harness as a packaged application. This is
artifact/source/test review; no new screen-reader or Windows/Linux Xiangqi GUI
campaign is claimed. Sidebar scrolling for controls is distinct from prohibited
board scrolling. Normal/QA binary hashes, environment, native frame dimensions
and exact source qualifications remain in their original manifests.

## Verification, revisions and platform scope

Current source has **no production diff** from CI revision
`79de582fb42cdd30f6bcfa6e15a1c5dd2263e4de`; its production source is
`bfe09cf4fed1827d868467a7cc475a3a175931ba`. Revision79de582 changed only the
external GUI verifier after Take back legitimately added a terminal action.
Revision541fc30 added completion evidence. MSH-103 changes documentation/task
records only; it does not invalidate unchanged native/product evidence.

The integrator queried GitHub again during this review:
[Rust40910846668](https://github.com/jweathers777/mShogi/actions/runs/40910846668)
and [Desktop40910846682](https://github.com/jweathers777/mShogi/actions/runs/40910846682)
are complete/success at79de582 on macOS15,Windows2022,Ubuntu24.04. Rust CI runs
format, lint, workspace/all-feature session tests and fixture freshness; desktop
CI runs frontend/native-host/package and bundled-engine checks plus recorded
actual installed/packaged Chess/Shogi GUI regressions on Windows/Linux.
These latter GUI results do **not** establish Xiangqi GUI operation there.

The actual new Xiangqi GUI evidence is macOS27.0.1(26A434),arm64,Tauri/WKWebView,
Rust1.99.1,Node24.19.0. MSH-111 retains the earlier Linux driver-reset failure;
later CI success neither repairs nor replaces it. MSH-112 retains the intermittent
unchanged runner process-exit classification failure. MSH-113's initial
Windows/Linux verifier failures and superseded cancelled runs also remain
recorded; its final verifier correction did not weaken native contracts.

New checks at541fc30: **250 passed** (64 native rules/history/exact,
63 search/engine/console/runner,47 native session/archive/Take back,76 frontend),
plus strict TypeScript. The separate capacity probe **failed as expected**,
establishing F01; it is not counted as a pass. Full commands and review scopes
are in [audit records](delivery-reviews/README.md) and
[machine-readable evidence](delivery-review-evidence.json). Original source/
comparator builds and old GUI interactions were inspected/reused, not re-executed.
Nineteen101 screenshot hashes and eight current113 normal/QA package file
hashes matched their recorded manifests. Backlog doctor and diff checks pass;
no production changes require another native platform campaign in this review.
Root frontend command:
`npm --prefix desktop test -- src/product/XiangqiApp.test.tsx src/product/XiangqiBoard.test.tsx src/product/ApplicationShell.test.tsx src/product/TakeBackControl.test.tsx src/product/ProductApp.test.tsx src/product/useGameSession.test.tsx`
passed **76 tests /6 files**; `npm --prefix desktop run check` passed with
Node24.19.0 explicitly on PATH. No new playing-strength or performance claim
follows from these checks. Native/comparator/match measurements retain their
original versions, limits and platforms.

## Findings, remaining boundaries and future architecture observations

**F01 (P2): incorrect user-facing capacity reason — corrective MSH-114 Ready.**
`NativeSessionPanels.tsx:55–59` selects the256-observation explanation whenever
its diagnostic contains `256`. Xiangqi `shared_xiangqi_archive.rs:459–465` and
Shogi `shared_archive.rs:226–233` return a combined message containing both256
observations and8192 controls when either limit is exceeded. An active document
at8191 controls with its reserved stop slot, but **zero observations**, refuses
another control correctly; the UI then falsely says it has recorded256 engine
observations. Native admission, stop/close reserve and complete Save remain
correct. The misleading recovery diagnosis is still a required product defect,
not M5 work or a platform-infrastructure issue.

The existing native
`control_replay_and_native_count_boundaries_reserve_complete_stop_and_terminal_paths`
test independently reaches this control-only boundary and passed. A temporary
frontend probe rendered the real native diagnostic with the normal zero-
observation native snapshot; the truthful-explanation expectation **failed** and
printed the incorrect256-observation claim. Its exact reproduction and retained
failure are in [capacity finding](delivery-reviews/capacity-finding.md).
The probe was removed afterward; it is not a silently failing product suite or
an acceptance test asserting that the bug is correct. The regular76 frontend
regressions passed; their observation-limit-only coverage missed this case.

Two fresh independent reviewers confirmed this finding and its P2 disposition.
MSH-114 owns the bounded shared explanation correction, both consumers' actual
native diagnostic tests, true256→257 coverage, preserved losslessness and
packaged interaction verification. It depends on completed99/102/113;
MSH-103 retains all seven old dependencies and adds114. No implementation of114
was started. AC1/AC3 and DoD1/DoD2 of103 remain unchecked; AC2/AC4 have the
platform/profile evidence recorded here. MSH-45 cannot close through this
blocked review, and MSH-106 remains unstarted.

**Resolved review-document findings:** current status pointers now distinguish
MSH-80/84/98/99 task-time handoffs from delivered consumers; the archive
compatibility addendum records113's Chess v3-after-Take-back behavior without
rewriting99's baseline. Independent synthesis review corrected an overbroad
“structured root exact” phrase: only the explicit native API/console diagnostic
execute exact analysis; the shared child/UCI/desktop do not advertise it.
No new rules-source conflict, material native/engine/record defect, or missing
required game-specific platform campaign was identified. This is a review
conclusion within the inspected source/evidence, not a formal proof over all
possible positions or a claim that every historical failure was repaired.

[Independent audit records](delivery-reviews/README.md) identify each reviewer,
commands, boundaries and limitations. No broad refactor or another task is
started here.

Concrete inputs for future MSH-106, not refactor requirements:

1. `ShogiBoard` and `XiangqiBoard` repeat selection/inspection generation,
   focus restoration, arrow traversal and stale-query retirement. The reusable
   semantics are bounded interaction lifetimes and native-complete-choice
   admission. Shogi promotion/hands versus Xiangqi intersection projection and
   distinct pieces remain intentional. Chess's older `NativeBoard` path needs
   the same semantic comparison, not component-name symmetry.
2. `ApplicationShell`, `GameArea`, `NativeSessionPanels`,
   `movementInspection` and `TakeBackControl` already own genuine common shell,
   recovery, status and interaction language. Chess retains older session and
   analysis presentation. Assess demonstrated maintenance cost before further
   sharing; the approved visual composition is already common.
3. `shared_xiangqi_archive.rs` and Shogi's archive adapter independently replay
   journals, observations, candidates and stop reserve. Potential reuse is
   orchestration/resource accounting; native events, exact-proof validation,
   claimant/warning semantics and Take back boundaries remain game-specific.
4. `ProductApp`, typed context/result dispatch, archive routing and native host
   contain explicit game branches. New variants will touch these admission
   points; that is not evidence a plugin ABI/universal rules model is needed.
   Review exhaustive dispatch and duplicated lifecycle policy separately from
   deliberately distinct native rules and record schemas.

These are source-backed diagnostic questions. “No material changes recommended”
remains valid for MSH-106. No filenames or line counts impose an architecture
requirement, and unaffected future game work gains no new gate.

## Launch and next boundary

Launch the normal current macOS package from the primary checkout:

```sh
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app"
```

New game→Xiangqi→Red/Black starts untimed human-versus-mShogi. Reopened and
restored games require explicit Resume. Build with Rust1.99.1 and Node24.19.0:
`cd desktop && npm ci && npm run desktop:build -- --bundles app`;
`npm run desktop:dev` launches development mode. The native console is
`cargo run --release --locked --bin mshogi -- --game xiangqi shell`.

No timed Xiangqi, competitive-strength target, desktop exact-solver button,
variation editing or extra external dialect is claimed. Exact verification
is available explicitly through the native `search_requiring_root_exact` API
and the console `nowin` diagnostic; the shared child/UCI/desktop do not advertise
an exact-operation capability;
ordinary play and desktop move analysis use measured depth1/128 defaults,
depth3/512 ceilings and honest incomplete/unscored results. Archive capacity
is bounded and lossless before acceptance; Take back does not reclaim it.
The immediate dependency-ready corrective assignment is **MSH-114**. After
its correction and review, resume MSH-103 acceptance; only then is MSH-45's
separate reconciliation dependency satisfied. MSH-106 cannot start until MSH-45
is Done. This task stops at MSH-103 and leaves MSH-45/M4 open.
