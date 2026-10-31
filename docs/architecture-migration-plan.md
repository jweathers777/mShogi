# Approved architecture migration plan

## Current scheduling refinement — 2026-10-31

MSH-116–119 and122 are Done; their evidence remains historical. MSH-120/R5
and121/R6 remain unassigned Ready work, with unchecked criteria and no active
implementation. The owner-approved [post-R4 UX contract](desktop/post-r4-ux.md)
adds three bounded product tasks without reopening119 or expanding R5:

| Task | Complete direct prerequisites | Scope |
| --- | --- | --- |
| MSH-123 | MSH-120 | Standard two-column Moves, sourced native display notation/exact event mapping and native completed-action cues |
| MSH-124 | MSH-119 | Shared shell order/copy, supported-only game dropdown, unchanged side buttons, branding and support access |
| MSH-125 | MSH-119 | Save/Load/New/Quit, direct Resign and one-step native browse Return while preserving intentional stops |
| MSH-121 / R6 | MSH-116, MSH-117, MSH-118, MSH-119, MSH-120, MSH-123, MSH-124, MSH-125 | Existing independent integrated architecture/platform acceptance plus approved final product journeys |

All three new tasks remain unassigned Ready future work with unchecked AC/DoD,
not started implementation. R5's complete prerequisite stays MSH-119, and its
existing criteria are unchanged. Last-move rendering is123's dependent product
work, consuming R5's surviving controller/renderers. Suggested integration order
is120,123,124,125,121;124/125 are independently dependency-ready and one integrator
serializes their overlapping application files. No artificial123→124→125 graph
chain or variant/M5 prerequisite is introduced. A-11/D-19 refines visible sidebar
order and document workflows; technical evidence/native semantic safety remains.

The original creation baseline below and later R1–R4 handoffs retain their
then-current wording and evidence. Use this current overlay for task status,
additional R6 prerequisites and approved post-R4 presentation requirements.

## Original creation baseline — 2026-10-30

Owner approval recorded 2026-10-30 against current `codex/rewrite` baseline
`ed34855dc1e13eb96eeb1f79d0f5e7f08f07738b`. This assignment changes planning
only: no production code, tests, runtime contract, codec, GUI driver or new game
implementation changes. [MSH-106](review/architecture-health-106.md) and
[MSH-115](review/architecture-target-115.md) stay Done and historical; their
then-current proposal/approval restrictions are superseded by the owner decision
in [design A-09/A-10, D-17/D-18](design.md).

## Approved outcome and scope

One application/runtime controller; exactly one active native-tagged document;
one current internal execution request/response family; one current desktop
envelope with authoritative native payloads; one frontend session/operation
owner; one shared board interaction controller. Remove obsolete development
interfaces, old transports/readers, compatibility wrappers and duplicate paths.

Native game-specific actions, rules, histories/adjudication, exact analysis,
search semantics and justified optimized structures remain native. Required
external UCI/USI/Xiangqi interoperability remains. Pre-release internal breakage
is permitted; lost functionality, weakened native replay, forgotten Take back
lineage, invented outcomes and unsaveable accepted work are not.

## Native task map and dependency sets

All seven tasks are **Ready, unassigned, M4, with unchecked AC/DoD and no
implementation plan**. They are future implementation/verification assignments,
not a claim that any new architecture exists.

| Stage | Task and title | Complete direct prerequisites | Ownership/outcome |
| --- | --- | --- | --- |
| R1 | MSH-116 — Establish the single internal execution and saved-observation contract | MSH-115 | Current native-tagged execution and durable observation contract, actual consumers/child and removal of V2/dual-family/fallback adapters |
| R2 | MSH-117 — Establish the current desktop envelope and native semantic retention | MSH-116 | Current writer/parser, authoritative native replay/lineage/proof retention, measured admission and removal of development-era readers |
| R3 | MSH-118 — Establish native application runtime document ownership and the current host contract | MSH-116, MSH-117 | Single active native document plus common metadata/install/admission/file/close services, authoritative typed resource facts and complete current native command/snapshot contract |
| R4 | MSH-119 — Deliver one frontend owner and shared application UI across all games | MSH-118 | One product owner and shared sections/dialogs/lifecycle, complete Chess behavior migration and removal of legacy product frontend/IPC |
| R5 | MSH-120 — Replace board interaction copies with one native-choice controller | MSH-119 | One board interaction lifetime, specialized renderers/completion views and native complete action selection |
| R6 | MSH-121 — Independently verify the integrated current architecture and packaged product | MSH-116, MSH-117, MSH-118, MSH-119, MSH-120 | Independent complete-target/deletion and semantic review, actual three-game packaged journeys on all three systems |
| P9 | MSH-122 — Triage the Linux verification-standard missing-e2 GUI failure | MSH-106 | Separate GUI verification ownership; High priority; intended first regression-baseline assignment |

R3 genuinely consumes current execution and document contracts. R4 consumes the
single current native owner; R5 consumes R4's application slots/current operation
and focus contract so it cannot create a parallel product UI. These are contract
dependencies, not changes to native variant sequencing. R6 explicitly depends
on all required stages. No existing task prerequisite changes, including M4
closure, MSH-107–109, MSH-48 or Tori/Chu/Tenjiku research/delivery.

One primary integrator owns overlapping R1/R2/native/host/frontend cutovers.
R1/R2 may use coordinated integration with independently reviewable subunits
where saved observations require it. Keep the DAG acyclic, record separate
criteria/evidence, retain a working complete current save path at each landing,
and do not mark either task Done with missing acceptance. Routine representation
and staging choices remain autonomous when a task is activated.

## R4 and R5: one product UI, no ownership gap

| Responsibility | R4 / MSH-119 | R5 / MSH-120 |
| --- | --- | --- |
| Overall composition/theme/control placement | Own one shared implementation for all games | Consume those slots; do not create game page/layout variants |
| Document controls/application close and one session owner | Own New/Save/Save As/Open/unsaved/lifecycle and snapshot/operation authority | Use current callbacks; no second session or document guard |
| Player, turn/status, analysis/history/Game Actions/Technical Details | Own common sections, hierarchy, language and surrounding presentation | Supply native board/completion content within those sections |
| Native-specific capabilities | Compose Chess claims/Full, Shogi advice/declaration, Xiangqi warning/claim/pending/proof content into common sections | Specialize board/hand/drop/promotion/route visual completion where native choices require it |
| Notices/errors/recovery/dialogs | Own common non-board presentation and focus/accessibility lifetimes | Delegate board-completion focus and board-specific interactions through agreed hooks |
| Selection/cancellation/inspection/complete-action lifetime | Define app disabled/history/operation context and current native contract; retire old product owner | Own one shared controller, board pending-query retirement, prefixes/completions and board keyboard/focus transitions |
| Board presentation | Shared application placement, full-board sizing expectations | Native topology, square/intersection decoration, piece glyph/art/orientation, hands and actual native completion views |

Current `ShogiApp`/`XiangqiApp` each invoke `ApplicationShell` and choose their
own sections; Chess `App` also mounts its legacy session. Sharing that chrome
alone is insufficient. R4 replaces duplicated application orchestration/section
ownership across **all three**, while native capability renderers remain small
composed content. R5 replaces selection/controller copies. R6 checks both,
including same-size normal/minimum captures and actual control/focus journeys.
No additional UI-refactor ticket is needed.

R3 explicitly produces typed native capacity/resource category and applicable
count/limit metadata through the current host contract, retaining unknown or
multiple causes and bounded technical text. R4 consumes these facts in shared
notices/errors/recovery without parsing diagnostic strings. Real control-only
zero-observation, ordinary/exact accounting,256-to257 and byte/event/replay
payload tests preserve truthful feedback, independent file errors and Save/
Save As availability. This completes the producer/consumer responsibility for
the approved structured-resource target rather than leaving it as an advisory
comment from MSH-106.

R4 must migrate actual Chess claims (including intended moves), ordinary and
Full analysis with proof/Unknown, history/observation provenance, unresolved
resignation, settings, telemetry, retry/Resume and Take back. The current shared
Chess projection's empty common observation fields do not satisfy this. Native
meanings stay explicit; no generic rules interpreter or uniform score model.

## Deletion endpoints and finite bridges

| Stage | Required retirement/deletion | Only permitted cross-stage bridge |
| --- | --- | --- |
| R1 | Legacy/Shared wire duplication, V2 wrapper/unknown-version fallback/chess_adapter, paired bridge methods and old observation consumers | R1/R2 observation/codec cutover may be coordinated under one owner; no permanent V2 reader or incomplete current Save/Open |
| R2 | Old desktop Archive/ArchiveV2/schema1/2/3 read/write branches and compatibility readers | Existing runtime → new current writer; validated new-format document → existing installation, with no second live state/old-format reader; deleted in R3 |
| R3 | Dormant Chess metadata owner, replaced installation/admission orchestration and R2 codec/runtime bridge | Thin new-controller → one known old frontend shape/command forwarder, no second session or arbitrary legacy clients; deleted in R4 |
| R4 | Old App orchestration, useNativeSession, duplicated guards/DTOs/embedded snapshot and Chess-specific host snapshot/command/file routes plus R3 forwarder | Board interaction copies may remain only until their named R5 replacement; not a second application UI |
| R5 | Board interaction copies/adapters, obsolete controller imports and development >9 scrolling assumptions | None at the final controller boundary; specialized renderers remain intentionally |
| R6 | Audit startup/handler/export/import/example/test/harness/package callers for all prior obligations | Audit, not a place to postpone stage-owned deletion |

Do not delete common native menu/window-close/request/cancel/finish behavior
because an existing helper name starts with `desktop_`. Migrate that behavior
into the neutral common application lifecycle. Native MatchRecord/commands,
Take back lineages, all accepted observations, known-history/proof context and
rules profile/mode distinctions must survive encoding/owner changes. Old-file
compatibility itself need not survive.

## P9 order and verification dependencies

MSH-122 is intended **before substantial refactoring**, to establish a reliable
existing GUI regression baseline. It is unassigned; the next investigator claims
it on activation. Its exact input remains the failed existing run40994147366
at2f17510, phase `verification-standard`, selector `.square[aria-label^="e2,"]`,
retained in the [MSH-106 metadata](review/evidence/msh106/existing-ci-status.json)
and [failure excerpt](review/evidence/msh106/linux-failure-excerpt.txt).
Cause is unknown. No investigation occurs in this planning assignment.

No blanket P9 dependency is added to R1–R6. If a later task truly cannot verify
its changed boundary without the baseline/fix, record the specific dependency
then; do not infer that every architecture/source task is blocked. R6 cannot
claim successful required Linux GUI acceptance from a package build or a failing
campaign. MSH-111's Done external-driver diagnosis and MSH-112's separate To Do
runner classification race remain unchanged, not assumed explanations for P9.

## Verification and completion discipline

New tasks require focused semantic/differential regressions and independent
review at their actual implementation revision, with meaningful tests ported
from old interfaces. Preserve independently expected native fixture facts;
delete tests whose sole purpose is supporting retired internal shapes. Reuse
unchanged rule/reference campaigns; new architecture behavior needs its own
focused evidence, not an old green CI claim.

R2 records actual packaged file/history/Take back/failure interaction; R3 native
runtime/file/close integration; R4 all-three-game shared UI/sizing/native claims/
analysis/recovery/document journeys and independent visual review; R5 actual
board/hand/promotion/inspection/routes/focus plus separate synthetic evidence.
Stage evidence is explicitly qualified by platform/revision/package. R6 requires
actual installed/packaged **Chess, Shogi and Xiangqi on Windows, macOS and Linux**
with play, native capabilities, analysis cancellation/replacement, recovery,
Save/Open/history/Return to live/Resume/Take back and native quit/window-close.
CI, controller tests and bundled-engine smoke do not substitute for GUI.

All stages retain real encoded-byte/event/control/combined ordinary-exact
observation/replay and stop/close headroom, 256-to257, atomic Save, current-format
fresh stopped reopen and stale-result rejection. Normal packages exclude test
instrumentation. Unknown/failure/exhaustion never yields an invented game result.
No task's completion automatically authorizes the next. R6 does not close M4,
MSH-46 or MSH-47; their existing gates remain intact.

## Planning provenance and review

Pinned Backlog.md1.55.1 used after detecting PATH1.55.0; overview/creation/
execution guides read. Started clean ated34855, fetched origin aligned0/0,
no active task owner. Read accepted design/review/roadmap and relevant current
frontend section ownership, nested Chess session and host/archive boundaries.
Searched existing task records before creating P9; no duplicate triage record
exists. MSH-111/112 remain separate.

All task changes use the supported CLI. Seven records are newly created; no
prior task, completed evidence, production source, runtime schema, test or
workflow changes. This document and design/roadmap record approval and planning,
not implementation results. Final graph/preservation/reference checks and
independent planning review are recorded below before integration.


## Final planning validation and independent assessment

At baseline ed34855, pinned Backlog1.55.1 and read-only Python checks validated
**122 unique/resolved task IDs**, no missing/self/duplicate-edge/cyclic
dependencies, exact new dependency sets, all seven new records Ready/unassigned
with every AC/DoD unchecked and no speculative implementation plans. All115
older task files, including completed106/115 and existing M4/variant/M5 gates,
remain byte-identical. Their complete historical reports/evidence are unchanged.
Thirty local documentation links resolve; design boundary/decision IDs are
unique. Backlog doctor and diff checks pass. Production code, tests, contracts,
formats and workflows are unchanged, so no product test/build/GUI/CI campaign
was run or claimed for this planning-only assignment.

Fresh independent reviewer `/root/migration_plan_review` read the owner request,
new decision/plan, all seven task scopes/dependencies, MSH-115/106 boundaries,
existing111/112 and representative actual product/board/native host source.
It independently checked graph, status/ownership, links, preservation and the
shared-UI allocation. One coverage finding was resolved through CLI: R3 now
explicitly owns native structured resource metadata and boundary tests; R4
consumes it in shared notices without text guessing. The reviewer approved the
revised planning with no remaining material discrepancy. A final all-game
wording refinement changed “chess outcomes” to “game outcomes” in that clause.
No tests/builds/GUI or task mutations were performed by the reviewer.

The intended next bounded assignment is **MSH-122/P9**, High priority, before
substantial refactoring. MSH-116/R1 is also dependency-ready without a mechanical
P9 gate. Neither has been activated. Stop after normal planning integration and
push; no architecture stage, Linux investigation, next variant or M5 work begins.

## P9 completion handoff — 2026-10-30

The planning-state statements above remain historical. MSH-122/P9 is now
complete: verifier-only readiness correction at `1d84232c425270a6517711dbaa598106fc08aa31`,
independent source/evidence review, actual Linux14-phase installed/packaged GUI
baseline, and green three-platform Desktop/Rust CI. The original failed run and
its unrecoverable absent-versus-disabled distinction remain explicit in the
[triage evidence](review/linux-gui-triage-122.md). No architecture migration or
product behavior changed. MSH-116/R1 is the next dependency-ready assignment,
still unstarted; no new dependency gate is added. MSH-111/112 ownership is unchanged.

## R1 completion handoff — 2026-10-30

The historical planning and P9 handoffs above remain intact. MSH-116/R1 is now
complete at production source `900544cf813ebb6b11428ac0fc9d5d45399dee1f`: one
protocol4 native-tagged execution family, semantic schema1 durable observations,
real consumer migration and obsolete transport deletion. Independent source,
persistence and artifact review found no material remaining finding; three-platform
Rust/Desktop CI and actual Linux/Windows Chess/Shogi package GUI checks passed.
See the [exact evidence and limitations](review/execution-migration-116.md).
Current-stage Save/Open is complete without activating R2. MSH-117 is next
ready but unstarted; outer archive consolidation, runtime/document ownership and
shared frontend work retain their existing later-stage ownership. No new macOS
or Xiangqi GUI campaign is claimed, and MSH-112 remains separate.

## R2 completion handoff — 2026-10-30

MSH-117/R2 establishes the sole current
[`mshogi.desktop.document.v1` envelope](desktop/document-v1.md), with strict
native identity/raw-payload dispatch and specialized Chess MatchRecord,
Shogi journal and Xiangqi WXF replay. Desktop1/2/3 readers and writer switches
are removed; obsolete development saves reject atomically. Schema1 observations,
selected history, complete Take back lineages, native proof evidence, admission
and stop/close headroom survive the cutover. No rules/search/engine change or
native ownership restructuring is included.

Corrected production source `eea8ce1c0e7b51bf02b42aa62f69d9c833c5735e` is
byte-identical to the product tree verified at CI-only integration
`ad14caed788f105e27a95868646db8d60cebd310`. Final Rust41088858739 and
Desktop41088858764 pass all three systems, including161 native all-feature
checks per system and Linux/Windows packaged Chess/Shogi GUI interaction.
An independently audited corrected-source Linux14-phase campaign retains exact
packages, saved documents and stopped/reopened/Resume captures. The original
Windows Open failure, subsequent CI timeout, bounded native proof-validation
correction and CI-only execution-budget adjustment remain explicit in the
[review and evidence](review/desktop-envelope-117.md). No product limit rose.

The only temporary bridge captures existing native runtime state for the current
writer and installs validated current documents into existing runtime owners.
It owns no separate live state and reads no obsolete format. **MSH-118/R3 must
delete it** while establishing application/runtime/document ownership; that task
is next dependency-ready and remains unstarted. R3–R6, variants and M5 have not
been activated. No new macOS or Xiangqi GUI campaign or MSH-112 resolution is
claimed; final integrated three-game/three-system GUI acceptance remains R6.


## R3 completion handoff — 2026-10-30

MSH-118/R3 replaces the always-present Chess runtime/optional overlays with one
native `ActiveDocument` and common `RuntimeMetadata`. One worker admission and
current-document installation path owns accepted changes, fresh stopped identity
and historical observations. Native ApplicationLifecycle owns reservations and
exact close approval; native menu/window/Quit/Tao behavior remains. Typed bounded
resource facts preserve actual category/measurement/count/reserve/limit instead
of requiring diagnostic parsing. Current document.v1, native semantics and limits
are unchanged. The R2 three-install bridge and obsolete native ownership are deleted.

Production source `b568ea7103e283155a851df16d3b382b3842eebc` and verification-only
integration `6e9b2e20a0315bbe902189376a6c0241dfd810f1` have separate source/artifact
review, green three-platform Rust/Desktop CI (166default/178all-feature native
checks each), normal local macOS lifecycle interaction and15-phase actual
Linux/Windows installed/QA GUI campaigns. Original test/verifier failures and
corrective evidence remain in the [R3 acceptance review](review/native-application-118.md).
The existing Chess frontend DTO/IPC is the sole finite forwarder; it owns no native
state or old-file reader. **MSH-119/R4 deletes that forwarder and its actual callers**,
consumes resource facts and consolidates application frontend ownership. It is
next dependency-ready and remains unstarted. R5/R6, future variants and M5 also
remain unstarted; this does not close M4 or final integrated R6 acceptance.

## R4 implementation handoff — 2026-10-30

MSH-119 replaces the finite R3 frontend/IPC forwarder with one ProductApp /
useGameSession / ApplicationView owner and common GameFrame/panels/capability
slots. Chess now supplies native capability details through the current tagged
snapshot and direct native operations. Parallel product pages, legacy owner/DTOs,
duplicate guards and old host endpoints are removed with actual consumers
migrated. The [R4 review](review/frontend-application-119.md) records the current
functionality/deletion matrix and verification. R5 retains the existing board
selection/inspection/route controller copies and consumes these shared slots;
this migration does not authorize or start that task or R6.

### R4 completed acceptance

Production `0487945b628314eed6d1be49d396dce41a362332` and verification revision
`eb027be7cec0f3206094c477a31f0cd9b9f73f6f` pass independent source/visual review,
225 frontend checks and the focused native/host/consumer regressions. Rust
CI41131548408 and Desktop CI41131548392 pass on all three systems. Linux native
attempt1 exposed the existing MSH-112 early-exit race; unchanged attempt2 passes.
The failure is preserved and MSH-112 remains open, not fixed by R4.

Actual Linux/Windows installed-normal/separate-QA campaigns each pass15 phases,
including native exit; normal/minimum comparison evidence covers all three
games. Local actual macOS package interaction and six same-size captures are
separately identified from CI construction/smoke. Full GUI Unknown evidence is
local macOS, not claimed Linux/Windows Full interaction. Exact package hashes,
review methods, original failures and bounded verification-only corrections are
in the [R4 review](review/frontend-application-119.md). Normal packages exclude
QA instrumentation. The completion commit changes documentation/evidence/task
records only. MSH-120/R5 is now next dependency-ready and remains unstarted;
R6, variants and M5 remain unstarted and M4 remains open.
