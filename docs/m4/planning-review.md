# Independent MSH-62 architecture and delivery-plan review

Reviewed 2026-10-26 by a fresh reviewer who did not author the planning proposal.
Scope: current-code mapping, architectural boundaries, native task scope and
ownership, original acceptance traceability, dependency executability and
preservation of completed evidence. The separately assigned primary-rule,
reference-source and detailed fixture audit is additional evidence; this review
does not substitute for it.

**Verdict: PASS within this review scope. No material unresolved finding.**
This is acceptance of a planning refinement, not approval of SHOGI-P1, a rules
conformance result, implementation authorization or closure of a game/M4.

## Inputs and provenance

Read `AGENTS.md`, `docs/design.md`, all M4 proposal documents and the native
records for MSH-41, MSH-46, MSH-62 and MSH-63–89. Inspected the actual chess
action/state, search and coordination, structured engine, desktop session,
capacity/persistence and frontend board/analysis boundaries cited by the map.

The reviewed worktree is `next-task`, branch `codex/m4-planning`, based at
`2b9849c`. Comparison with closed-M3 integration
`6f29713795804eed3605114d90fed37aa5f554a0` confirms identical product sources
under `crates/` and `desktop/`. The root workspace manifest still excludes
`legacy/rust-prototype`; the integration revision removed that obsolete entry.
The map correctly qualifies this provenance instead of claiming an identical
whole checkout. Existing integration cleanup is outside this batch.

Used the recorded Backlog.md 1.55.1 binary, read its overview, and obtained all
89 current records through `backlog task view <id> --json`. Independently
compared the 61 original records with the pre-change structured snapshot
`/private/tmp/msh-m4-planning-baseline.json`; that temporary path records the
comparison input, not a durable repository dependency. Protected original fields
match exactly. MSH-41–46 retain their old dependencies and documentation, with
only the intended additional dependencies, documentation and appended notes.
MSH-47 is unchanged. MSH-25/26/27–40/61 remain Done with their original acceptance
and evidence fields; no completed work was reopened.

## Architecture and ownership assessment

The map accurately identifies the concrete chess assumptions: White/Black and
8×8 coordinates, board-origin actions, chess records/outcomes/claims and search
errors, protocol v2 contexts, native `ChessMatch`, chess score/move formatting,
and archive-v1 restoration policies. It also recognizes existing route-choice
presentation and bounded worker/process mechanisms that can be extended.

The proposed first extension has two real consumers and explicit typed native
dispatch. Shogi owns its state, legal complete move/drop/promotion choices,
derived captures, exact reversal, history, conclusions and canonical replay.
The frontend receives native choices; hand origins are explicit. Native sides,
orientation, first mover and ownership are separate. Declaration and resignation
remain match commands. Analysis capabilities preserve chess Full while exposing
only actually supported shogi operations. No universal rules DSL, plugin ABI,
mandatory common board storage or universal negamax obligation is introduced.

Shared ownership is concrete and nonduplicative: MSH-69 owns wire/context/routing,
MSH-72 session and reusable selection/presentation, MSH-73 common archive and
admission mechanisms, MSH-76–79 later real consumer extensions, and MSH-88/89
final acceptance/review. Game tickets own semantics and native adapters. In
particular, MSH-73 dispatches shogi replay supplied by MSH-67; MSH-72 presents
drops whose legality belongs to MSH-66.

Compatibility is an acceptance obligation rather than an incidental migration:
strict old chess archive/record readers, duplicate-field rejection, historical
purpose/context and advanced observations, explicit unsupported restoration,
fresh runtime identity and stopped Open are preserved. Exact purpose, history,
request/session/process generation and limits remain acceptance context.
MSH-61's immutable complete candidate, prior-saveable-state guarantee, identity
and shutdown headroom, stable receipt and atomic-write boundaries remain
explicit. Current chess search/archive caps are correctly treated as selected
policies, with real shogi sizing and bounded reconstruction required later.

## Acceptance traceability

The following checks use the original criteria, whose text and checked state
were preserved, rather than treating child-task counts as delivery evidence.

| Original scope | Native obligations and final gate |
| --- | --- |
| MSH-41 AC1, complete profile | MSH-63 decision/profile; MSH-67 selected history/conclusions; MSH-75 completeness review |
| MSH-41 AC2, authoritative state/actions/history | MSH-65 state/reversal, MSH-66 full legality, MSH-67 history/match; MSH-69/72 transport and acceptance |
| MSH-41 AC3, independent evidence | MSH-64 independent corpus/reference admission, MSH-66 comparisons, MSH-67 history/record corpus and MSH-75 integrated review |
| MSH-41 AC4, console/notation | MSH-68 actual standalone console and notation/record adapters; MSH-69 common routing |
| MSH-41 AC5, own functional engine | MSH-70 native evaluation/search/child results and MSH-71 actual USI/self-play/opponent smoke; MSH-69/72 preserve supervision/acceptance |
| MSH-41 AC6, canonical records | MSH-67 game-specific records/replay, MSH-68 adapters and MSH-73 common compatibility/admission |
| MSH-41 AC7, actual desktop | MSH-74 actual game journeys, MSH-72/73 shared mechanisms; MSH-88 retains final cross-game three-platform evidence |
| MSH-41 AC8, independent complete-game review | MSH-75 checks all original obligations before MSH-41; required shared work also gates delivery |
| MSH-46 AC1, all-six contexts/actions | MSH-69/72 and actual later MSH-76–79 consumers, integrated through MSH-88 |
| MSH-46 AC2, canonical compatibility | MSH-73, later typed extensions and MSH-88, with game-specific encodings remaining game-owned |
| MSH-46 AC3, real presentations/choices | MSH-72, real MSH-76–79 and MSH-88; synthetic evidence alone is explicitly insufficient |
| MSH-46 AC4, ownership and cross-game flows | The delivery ownership matrix plus actual MSH-88 library/console/engine/record/desktop regression |
| MSH-46 AC5, packaged three-platform acceptance | MSH-88 requires actual installed/packaged GUI and bundled-engine journeys on all three systems |
| MSH-46 AC6, independent integration review | MSH-89, with unresolved material findings blocking MSH-46; MSH-47 retains whole-M4 closure |

All 27 future tasks have stated ownership, acceptance and verification
expectations, no checked acceptance criteria and no speculative detailed
implementation plan. Readiness distinguishes actionable scope from satisfied
dependencies. MSH-63 is To Do pending SHOGI-P1. The CSA bundle remains an
unapproved recommendation; creating this graph does not choose it for John.

## Dependency executability

An independent full DFS over the 89 CLI records found no missing dependency
and no cycle. Read-only `backlog doctor` also returned: “No duplicate IDs,
self-referential dependencies, or dependency cycles found.”

The concrete shogi path is executable: planning MSH-62 permits independent
MSH-64 board/reference work and MSH-65 state/reversal, then MSH-66 legality.
MSH-67 also waits for MSH-63 ratification. The real canonical consumer then
permits MSH-69 routing, MSH-68 console and MSH-70 search, followed by MSH-71
interoperability, MSH-72 sessions, MSH-73 archives, MSH-74 application delivery
and MSH-75 independent review. None of MSH-69/72/73 depends directly or
transitively on final MSH-41/46/47.

Later source tasks MSH-80–83 each depend only on planning; they are not chained
behind another game's completed delivery. Native handoffs MSH-84–87 depend on
their respective source task. Shared MSH-76–79 depend on the corresponding real
handoff and early shared work, rather than their aggregate game delivery.
MSH-79 also depends on the MSH-78 mechanism, without requiring completed chu
delivery. Native handoffs and later shared extensions remain To Do and require
further concrete refinement before Ready. Their broad existence does not
pretend that the later full engine/console/desktop deliveries are already split.
The original later-game AC4–8 remain open and their further refinement is named.

Final MSH-46 retains its dependencies on all five complete game deliveries and
now explicitly includes its shared acceptance/review work. MSH-47 retains
MSH-26, MSH-39 and all MSH-41–46 gates. Final packaged acceptance depends on
complete games; the earlier game-native/application work needed to reach them
does not depend on that final packaged gate. M5 strength and M6 obligations
remain distinct and unchanged.

## Evidence limits and handoff

No Rust/frontend test, packaged GUI rerun, reference build, opponent match or
game implementation was performed for this planning assessment. Existing M3
evidence covers unchanged behavior; future tickets explicitly require new
runtime evidence for their changes. No source-audited reference, proposed
fixture, bibliography or task status is represented as executed conformance.

This review found no required architecture/task-graph correction. Primary-rule
edition/interpretation, reference-operation and detailed fixture correctness
remain subject to their separate independent assessment. The planner must also
complete the durable validation, normal integration and push evidence before
claiming MSH-62 complete. SHOGI-P1 and all MSH-41–47 delivery obligations remain
open at this review boundary.
