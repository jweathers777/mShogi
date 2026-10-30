# MSH-45 Xiangqi game-delivery acceptance

Acceptance reconciliation: 2026-10-30, primary integrator Codex, main
`codex/rewrite` checkout. **All eight original MSH-45 criteria are supported.**
This is the separate game-delivery acceptance following completed MSH-103,
not another implementation or complete-game review campaign. M4 stays open.

## Exact source and dependency disposition

- MSH-103's final independently approved assessment is integrated at
  `c80a95a9e9c149556aba6f95055e39cab0d5ccf8`.
- Current production source is unchanged from the independently verified
  MSH-114 correction, `1c6b97e2bf7ecf560be1902e01b4e95038ee3482`.
- The original complete-game source assessment at
  `541fc303fdeaf5017948d14ddf92bf5efee5ab3f`, original blocked verdict and failed
  F01 probe remain preserved in the [delivery review](delivery-review.md).
- All eight direct prerequisites are Done with all criteria/completion items
  checked: MSH-39, MSH-72, MSH-73, MSH-76, MSH-80, MSH-84, MSH-103 and MSH-105.
  Their historical task-time next steps do not override subsequent delivery.

No rules, profile, native implementation, production frontend, archive schema,
work limit, fixture expectation or task dependency changes in this acceptance.

## Original AC1–8 reconciliation

The full source/fixture/execution mapping is the completed MSH-103
[final acceptance matrix](delivery-review.md#final-msh-45-ac1–8-findings), its
preserved clause matrix, and the [independent final reconciliation](delivery-reviews/final-reconciliation.md).
This table records MSH-45's disposition without replacing that evidence.

| Original criterion | Accepted game-specific evidence | Disposition |
| --- | --- | --- |
| AC1 Complete sourced, versioned profile | MSH-80 owner decisions and selected `mshogi.xiangqi.wxf2018.v1`, edition `WXF-2018+XIANGQI-P1-2026-10-27`; [sources](sources.md), [coverage](coverage.md), MSH-91–94/105 source and independent rule/fixture reviews | Satisfied. Native setup, all pieces/actions/effects, turn/royal safety and automatic/claimable history adjudication are covered; no promotion/drop/declaration is invented. |
| AC2 Authoritative reusable rules/state/history | MSH-92–95/84 native state, complete actions, checked atomic admission, exact reversal, WXF history and replay; MSH-98/76 native session and MSH-113 Take back; [native review](delivery-reviews/native.md) | Satisfied. Position, repetition, proof, canonical history and execution identities remain distinct; consumers do not duplicate legality. |
| AC3 Independent conformance | MSH-91 admitted comparator provenance and independent board fixtures; MSH-93 complete legal sets, sequences, perft/divide; MSH-94 source-derived conduct/count/precedence histories; MSH-105 independent exact universes and witness replay | Satisfied. Independently expected facts remain separate from comparator observations; no reference history helper becomes rules authority. See [fixtures](fixtures.md) and [local evidence](local-legality-evidence.md). |
| AC4 Independently usable console | MSH-96 [console contract](console.md), actual subprocess fixtures, complete action/undo/history/status/canonical save/load and explicit bounded exact diagnostics; [consumer review](delivery-reviews/consumers.md) | Satisfied. Human shell and machine UCI/internal output contracts are separate. |
| AC5 Own functional engine and matches | MSH-97 [engine evidence](engine-evidence.md), native history-aware evaluator/search and supervised context/results; MSH-100 [protocol](protocol.md), audited independent opponent and retained own/external match records | Satisfied as functional M4 delivery. Limits, purpose/request/session/process identities, cancellation, stale rejection and unscored incomplete work are preserved. Measured competitive strength remains MSH-54/M5. |
| AC6 Lossless canonical and desktop records | MSH-94/95 native record/routing; MSH-99 [archive contract](desktop-archive.md), strict authoritative replay and measured precommit admission; MSH-102/113 actual Save/Open, exact-event history and retained Take back lineages; MSH-114 boundary saves | Satisfied. Root/provenance, WXF warnings/obligations, claimant/events, policies/settings and historical ordinary/exact observations survive; reopen is fresh/stopped. Atomic file failure and complete saveability remain intact. |
| AC7 Actual desktop product | MSH-98/76 native session/presentation, MSH-101 approved intersection/river/palace/Red-Black board in the common shell, MSH-102 play/analysis/outcomes/recovery/history/documents, MSH-113 Take back, MSH-114 corrected capacity UI | Satisfied by actual game-specific macOS normal/labeled-QA interaction plus native/frontend evidence. Inspection, cancellation/retry, Return to live, Resume and protected Save/Open/quit are evidenced. Final all-game Windows/macOS/Linux packaged acceptance remains MSH-88/46/47. |
| AC8 Complete ledger and independent review | Completed MSH-103 [domain reviews](delivery-reviews/README.md), final matrix and independent acceptance reconciliation; MSH-114 independent correction review and retained GUI evidence | Satisfied. No material required Xiangqi finding remains; no required behavior was made optional or moved to M5. Shared findings/gates remain visible for MSH-46/88/89/47. |

## Consequential rules and honest execution boundaries

The selected official bilingual WXF 2018 profile and per-case reconciliations
remain unchanged. G-CYCLE uses the reviewed third-occurrence classification/
warning and fourth continued responsibility loss or applicable draw, exact
conduct/overlap histories and native event ordering. Natural-count eligibility
is claimant-specific; advisory opportunities, formal commands, audits and
recorded automatic-claim policy are distinct. Imported counters do not create
unknown earlier repetition history.

G-NOWIN uses the owner-selected existential natural-board-win test: a finite
ordinary board-action continuation reaching checkmate or stalemate loss.
It is not forced-win minimax; repetition penalties or voluntary/administrative
results do not provide a positive witness. MSH-105 supplies the actually
executed bounded/terminable exact verifier beyond sound fast recognition,
with proven no-win, positive witness, and Unknown/incomplete distinctions.
Ordinary consumers remain `sound_fast_only`. Exact execution is explicit and
read-only in its supported native/console paths, not routine rendering/reopen
or an invented desktop outcome command. Resource exhaustion/cancellation never
become a chess-family result; execution failure is distinct and unscored.
Unsupported exact restoration combinations reject instead of downgrading.
Saved exact observations preserve target history and proof/work provenance as
historical evidence, never pending work or a new live outcome.

## Reused verification and newly performed checks

Reused without repeating unchanged campaigns:

- MSH-103's original 250 focused checks and three fresh independent domain
  assessments; its final bounded independent acceptance review.
- MSH-114's 235 frontend/four focused native tests, independent source/test/
  artifact assessment, and actual normal/labeled-QA macOS package PNG/AX/save
  evidence at source 1c6b97e. [Correction provenance](../../desktop/evidence/msh114/README.md)
  resolves F01: truthful control/byte/event/replay/ambiguous explanations, genuine
  ordinary/exact observation accounting, 256-to-257 refusal and complete saves.
- Existing component conformance, protocol/matches, archive, product and Take
  back campaigns mapped above. The actual macOS journeys are not replaced by
  controller tests, artifact inspection or package construction.

Newly performed in this reconciliation:

- Git branch/remote/worktree/ownership inspection and fetched remote comparison;
  pinned Backlog 1.55.1 after detecting global 1.55.0.
- All direct dependency status and AC/DoD checks; original criteria-to-final-
  assessment reconciliation; production diff against 1c6b97e (empty).
- Existing CI metadata query at exact c80a95a; all six jobs below succeeded.
- Documentation local-reference/JSON checks, unchanged historical-review check,
  Backlog doctor and final diff/task bookkeeping checks.
- Proportionate independent review of this acceptance and handoff change, recorded
  in the MSH-45 task before finalization. No new product tests, builds, reference
  runs, matches or GUI journeys are claimed.

| Existing run at c80a95a | Windows 2022 | macOS 15 | Ubuntu 24.04 |
| --- | --- | --- | --- |
| [Rust workspace 40965828435](https://github.com/jweathers777/mShogi/actions/runs/40965828435) | Success | Success | Success |
| [Desktop packages 40965828489](https://github.com/jweathers777/mShogi/actions/runs/40965828489) | Success | Success | Success |

These are existing exact-revision CI results inspected now, not suites launched
for MSH-45. Successful CI does not establish new Xiangqi GUI interaction on
Windows/Linux. The failed Ubuntu runner classification at earlier documentation
head 869bee3, run40961719798, remains historical evidence under MSH-112; a later
passing run is not a fix. MSH-112 remains To Do. MSH-111 is already **Done**:
its bounded external-driver correction and successful Linux/Windows Chess/Shogi
GUI evidence are recorded in the [driver diagnosis](../../desktop/linux-gui-proxy.md).
Its earlier failed campaign remains failed and retained; this is not an open
MSH-111 blocker or new Windows/Linux Xiangqi GUI evidence. Earlier assessment
statements about MSH-111 being open are historical, superseded by its actual
completed task record.

## Proportionate independent completion review

Fresh read-only reviewer `/root/accept45_review` inspected AGENTS/design, the
original MSH-45 AC1–8/DoD, all eight direct prerequisites and their checked
completion items, the completed MSH-103 matrix/native assessment/final
independent reconciliation, MSH-114 source and package/GUI provenance, this
acceptance document and all current handoff prefixes. It independently checked
the empty production diff, unchanged historical review/probe and raw c80a95a
CI metadata (all three jobs successful in each workflow).

**Approve acceptance; no material findings.** The only status clarification was
that MSH-111 is already Done; the paragraph above records its actual disposition
while preserving the failures. All eight criteria and DoD evidence are supported;
normal commit/integration/push completes the integrator-owned final bookkeeping.
No new correctness tests, builds or GUI campaigns were run by this reviewer.
MSH-106 remains an unstarted diagnostic review, with no refactor authorization.

## Remaining ownership and stop boundary

No unresolved required game-specific Xiangqi finding remains. Initial live play
is untimed; measured search limits and bounded exact Unknown are explicit.
Archive byte/event/control/observation/replay ceilings preserve complete accepted
data with close/stop reserve; Save/Open does not reset structural capacity.
Competitive strength and tuning are MSH-54/M5, not established by functional
smoke matches. Actual final all-game packaged GUI evidence, cross-game regression
and shared integration remain MSH-88/46, independently assessed by MSH-89 and
closed only through MSH-47. This does not close M4 or another task.

The concrete MSH-106 inputs remain duplicated frontend interaction state,
game-specific presentation branching, native session/record lifecycle, Chess
archive divergence and structured resource diagnostics. They are diagnostic
questions tied to the existing [review observations](delivery-review.md), not
an automatically authorized refactor or deferred required Xiangqi corrections.
After MSH-45 completes, MSH-106's sole dependency is satisfied; it stays Ready,
unassigned and unstarted. Stop at MSH-45. M2/M3, completed Shogi, MSH-61 and the
six-game roadmap remain intact.
