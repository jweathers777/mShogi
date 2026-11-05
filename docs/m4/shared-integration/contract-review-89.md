# Independent shared-contract acceptance — MSH-89

Verdict: **all six original MSH-46 obligations have supporting evidence;
no material shared-contract blocker was found**. The fresh independent review of
this consolidated report is approved. The report was normally committed and
pushed at `22b4f409f738cd7dec45a78ab7ecc4cf9661322b`; MSH-89 is Done.
MSH-46 remains To Do and unchecked; this review does not
perform its completion workflow, MSH-108 or MSH-47/M4 closure.

The reviewed integration is `3bcaf013b7e9ea7d37a5a542700de8a1ea041b2b`.
The existing six-platform-job/packaged campaign tested
`bc92bfe5e3b20e9e999df12cbf54b91682c71b8d`. Before this review, production,
frontend, host, observer and workflow inputs had no diff between those revisions.
This assignment changes four stale rustdoc descriptions, documentation and task
metadata only. No new package build, target-OS execution, broad GUI campaign,
rules-conformance campaign or strength experiment is claimed at the review commit.

[contract-review-89.json](contract-review-89.json) is a compact review index,
not another platform evidence ledger. It references the unchanged MSH-88
[verification record](verification.json) and its original reports/packages rather
than copying or replacing them. The [existing matrix](README.md) remains the
shared/per-game ownership starting point; its conclusions were checked against
source and original evidence rather than accepted from child-task checkboxes.

## Independence and method

The primary integrator authored MSH-88, so author rereading was not counted as
independent acceptance. Two new read-only reviewers started without that chat
history: `msh89_native_contracts` inspected native/record/execution/admission
boundaries, and `msh89_consumers_platforms` inspected actual frontend/console
consumers and original platform receipts. Neither changed files. A third fresh
read-only reviewer, `msh89_findings_review`, independently approved this report,
original-criterion coverage, dependency audit and dispositions.

New work consists of source inspection, dependency/criterion integrity checks,
evidence hash and raw-report reconciliation, and one focused native boundary
command: `cargo test --locked -p mshogi-core --test native_envelope --test
native_tenjiku_envelope` passed **11 tests, zero failures** at `3bcaf01` on
macOS27.0.1 arm64, Rust1.99.1, before the rustdoc correction. Formatting and diff
checks passed after that correction. The final six-job results are reused at
their actual tested source, not relabelled as newly executed checks.

## Original MSH-46 AC1–6 traceability

### AC1 — explicit complete native boundaries and bounded execution

**Supported.** The original requirement covers all six library/console/internal
engine/session consumers, immutable profile/version and full history, atomic
admission, claim versus board action, cancellation/stale identity and bounded
reporting without frontend rules or Python/Node search round trips.

`RulesProfile` has seven immutable values for six games; `CanonicalRecord`
preserves concrete native payloads and `dispatch_native` calls native command
owners ([native.rs](../../../crates/mshogi-core/src/native.rs), lines41/165/271/589).
Xiangqi submission/audit events and native declarations/claims are not flattened
into ordinary moves or a generic "rejection never records anything" rule.
Malformed/stale boundary requests and failed admission preserve accepted state;
legitimate domain commands retain their native event semantics.

`ExecutionContext` carries full canonical history, purpose/selected target,
request/session/process identities and limits. `ExecutionObservation::matches`
checks full context and result family
([engine/shared.rs](../../../crates/mshogi-engine/src/shared.rs), lines98/359).
`EngineBridge` is the one supervised family; the server dispatches all six native
searches. Real console game selection and separate machine routing are in
[main.rs](../../../crates/mshogi-console/src/main.rs), lines239/273. Human diagrams,
notation and protocol stdout remain separate contracts.

Shared owners are MSH-69/72/76–79/90 and the accepted116–125 migration; native
rules, search, notation and complete-game obligations remain MSH-41–45. Current
evidence includes the actual console/internal subprocess suites, mandatory
release Tenjiku subprocess/selfplay checks, all three Rust jobs and real packaged
play at `bc92bfe`. Native envelope negative tests additionally passed in this
review. No competitive-strength conclusion follows. MSH-112 is separately
qualified below; no unscored transport failure is treated as a game outcome.

### AC2 — native records, current documents and explicit rejection

**Supported within the owner-approved current compatibility contract.**
`CanonicalRecord::parse` preserves raw nested bytes, dispatches the strict native
parser, and corroborates game/profile/edition/native schema. Tenjiku native replay
compares complete stored events with derived ordered effects and final state
([tenjiku/record.rs](../../../crates/mshogi-core/src/tenjiku/record.rs), line258).
The same native authority principle applies to every game's codec. The current
desktop envelope is independently versioned in
[document.rs](../../../crates/mshogi-desktop-session/src/document.rs), lines16–46.

Native Chess schemas1/M2 and2/Full remain supported by core records. Console
restoration intentionally accepts its documented M2 move/history scope and
rejects observation-bearing/Full restoration. Desktop live restoration remains
untimed M2/schema1; historical bounded Full analysis/certificates remain supported.
These scopes are not interchangeable. Unsupported/lossy conversions reject;
there is no FEN-only fallback discarding accepted history.

MSH-73's old desktopv1 reader and MSH-69's old transport adapters are preserved
historical achievements, not current compatibility obligations after owner
A-09/A-10 and MSH-116/117 retired development transports1/2/3 and desktop archives
1/2/3. The sole current desktop envelope is `mshogi.desktop.document.v1`, internal
transport4 and durable observation schema1. Restoring retired readers would
contradict the accepted architecture. Native record schemas and supported
console aliases were not retired with those development desktop formats.

MSH-90's canonical console commands, explicit aliases and machine-output
isolation remain applicable. Its original owner-colon human board display was
superseded by the owner's MSH-110
[case/prefix convention](../../console-presentation.md). That presentation change
does not change native record/protocol tokens, complete actions or rule meaning;
the old human-display fixture is preserved history rather than a current reader
or formatter requirement.

`install_document` is the sole **validated current-file Load installation**
transaction ([persistence.rs](../../../crates/mshogi-desktop-session/src/persistence.rs),
line1664). It computes fresh identity floors, checks headroom, retires execution
and installs stopped native state. Saved observations remain historical.
New Game uses admitted per-game paths in the same controller; Shogi/Xiangqi New
admission swaps are not required to call this file-install function. This is one
authority with concrete adapters, not duplicated applications.

Shared owners73/117/118 and game41–45 supply the codecs/replay; current evidence
includes strict duplicate/unknown/cross-profile/schema rejection, Full record and
certificate tests, seven-profile native New/Load/stale-authority tests, actual
cross-game Save/Load/Take back/retained observations and native prior-file
preservation. Both Tenjiku IDs remain explicit in initial/current/retired and
observation records; no independent rule toggles reinterpret an existing game.

### AC3 — actual complete actions and shared interaction

**Supported.** `ProductApp` calls one `useGameSession`; exact rendered authority
is submitted by that session. One `ApplicationView`/`ApplicationShell` owns the
ordinary sections and document lifecycle; `GameBoard` selects actual renderers.
`useNativeChoiceController` submits complete native choices only; route prefixes,
promotion cancellation, hand selection and read-only inspection cannot commit a
partial turn. No second frontend legality/adjudication implementation was found.

Sources: [ProductApp.tsx](../../../desktop/src/product/ProductApp.tsx), line27;
[useGameSession.ts](../../../desktop/src/product/useGameSession.ts), line54;
[GameBoard.tsx](../../../desktop/src/product/GameBoard.tsx), line12;
[useNativeChoiceController.ts](../../../desktop/src/interaction/useNativeChoiceController.ts),
line108. Shared72/76–79/119/120 supplies the mechanism;41–45 supplies native
geometry, hands, promoted faces, complete routes/effects, notation and artwork.

Evidence combines actual Chess special choices, Shogi/Tori hands and promotions,
Xiangqi intersections, Chu Lion routes and Tenjiku compound/igui/Fire Demon
effects for both profiles. Native effects, not SVG geometry, determine victims,
burns and outcomes. Existing current frontend/native fixtures and real49-phase
Windows/Linux reports cover these adaptations; synthetic cases are separately
labelled and do not establish real-game delivery. Current shared names, names
during compound selection, above-art capture outlines, pointer-transparent
decoration, enemy inspection, focus and Escape have retained evidence.

The accepted760×650 minimum is unchanged. Dense16×16 kanji/secondary badges are
not all comfortably readable there; around960×1000 remains a recommendation.
Keyboard/native AX/accessible labels do not establish comprehensive screen-reader
support. These are the recorded owner limitations, not omitted functional scope
silently transferred to M5 or a request for another aesthetic checkpoint.

### AC4 — single ownership and actual cross-game flows

**Supported.** Shared mechanisms remain owned once by46 through69/72/73/76–79/90
and the completed architecture migration. Game41–45 owns native adaptations.
There is one `DesktopController` and one active native variant
([lib.rs](../../../crates/mshogi-desktop-session/src/lib.rs), lines346/1207),
not a dormant Chess game behind each other game or a universal native rules DSL.

The source-level seven-profile regression
([six_game_integration_tests.rs](../../../crates/mshogi-desktop-session/src/six_game_integration_tests.rs),
line151) exercises same-controller New/reverse Load, exact old command/inspection/
replacement/Open/result rejection, and malformed Load preserving pending authority,
dirty/path/history and previous file bytes. It uses **manual execution allocation**;
it is not misrepresented as subprocess or GUI evidence.

Original raw Windows and Linux reports separately prove nine same-host New visits
across seven profiles, two complete moves plus a real engine observation per
played state, seven exact cross-game stopped Loads, fresh authority, historical
observations and retained Take back evidence. Eight gated process replacements
span Chess→Shogi→Xiangqi→Tori→Chu→Adams→Muller→Adams→Chess Load. New authority is
stable beyond the old gate interval and old exact gate PIDs disappear. Process
retirement is not delivery of an obsolete response; deterministic native tests
separately submit obsolete authority. Console game/profile selection and machine
routing tests preserve the console side of the same explicit native dispatch.

### AC5 — actual installed/packaged three-platform behavior

**Supported at the recorded sources/environments.** The six final required jobs
all passed attempt1 at `bc92bfe`, not at a later documentation commit:

| Platform | Rust job / run42028972679 | Desktop job / run42028972586 | Actual GUI evidence |
| --- | --- | --- | --- |
| Ubuntu24.04 |127250225725 PASS |127250225410 PASS | Apt-installed normal app plus separately extracted QA,49/49 phases |
| Windows2022 |127250225944 PASS |127250225967 PASS | Actual normal/QA NSIS installations,49/49 phases |
| macOS15 |127250225640 PASS |127250226038 PASS | Hosted native/build/package/real-engine checks; local GUI is separately below |

The [MSH-88 record](verification.json) retains exact runner images, step results,
commands, package/engine hashes, artifacts, original report hashes and failures.
The fresh reviewers independently verified all202 retained evidence hashes, both
decompressed original49-phase report hashes, exact source, all13 normal-phase
host/engine hashes per platform against package smoke, normal inventories
(Linux13/Windows2 files) and two exact QA package copies per platform.
`package-smoke.py` checks normal-host instrumentation exclusion; normal and QA
feature/entry paths stay isolated. Native pickers, actual pointer/keyboard input,
process retirement and graceful host/engine absence are in original reports.

Local macOS27.0.1 arm64 gameplay used the fixed normal and QA identities. Its
captured normal package was built at `eca9f9c`; all four actual bundle file hashes
equal the later `bc92bfe` rebuild. The fixed sealed QA files remain unchanged
through scenario/profile journeys. Both profiles, seven-profile normal New/Load,
real engine play/read-only analysis, stopped history/Resume/Take back and actual
native Quit have AX/screenshots/records and process-absence evidence. Original
native JPEG bytes are retained. Earlier incorrectly bound normal/QA observer
attempts are explicitly excluded. Local source-active observations precede input;
they are not upgraded to exact activity at commit or obsolete-reply delivery.
Hosted macOS15 and local macOS27 claims remain distinct.

### AC6 — independent assessment and honest remaining scope

**Supported; fresh final report review is approved.** The two
fresh independent source/evidence assessments above found no material shared
contract defect. The separate final reviewer checked this consolidated finding,
coverage and DAG rather than counting completed child tasks. Low-risk wording
corrections below preserve current behavior and existing evidence. No required
six-game functionality is moved to strength research or omitted by splitting.
This supplies89's review evidence for46AC6;46 still owns its own criteria/DoD
completion, and47 retains whole-M4 acceptance.

## Execution, saveability and resource accounting

Full-context equality, not matching board placement, admits a result.
`EngineBridge::submit` requires increasing request/generation identities;
`cancel` retires pending and already published completion, and the worker checks
response identity/result family before publication (`bridge.rs:107/135/396`).
Native admission rechecks the exact selected historical record and session
version (`shared_tenjiku.rs:525`); Close retires runtime work (`:684`). Thus a
saved historical observation or an old same-profile document cannot become
current execution authority.

Whole-document admission preserves common16MiB encoded,256-observation and8,192
control limits while retaining different reconstruction policies:

| Native document | Concrete current policy and source | Accepted qualification |
| --- | --- | --- |
| Chess | Existing16,000 retained-branch units plus worst selectable prefix; bounded Full certificate verification local to one operation (`persistence.rs:654–737`; regression `full_archive_tests.rs:113/162`) | Branchless supported Full observations keep their own bounded witness rules; board equality cannot reuse another history's proof. |
| Shogi |16,000 weighted native/control/observation units (`shared_archive.rs:268–376`) | Full candidate encoding and stop/Close reserves remain required. |
| Xiangqi |16,000 units with native history and ordinary/exact observation accounting (`shared_xiangqi_archive.rs:447`) | Combined evidence allowance is not counted as separately exhausted component quotas. |
| Tori |262,144 units, precharging every retained native record/context (`tori_document.rs:16/648`) | Mandatory180-action successful journey differs from360-action near-limit deadline refusal; no guaranteed boundary latency. |
| Chu |12M aggregate reconstruction units (`chu_document.rs:13/811`) | Actual ending/Close encodings and native replay remain distinct. |
| Tenjiku, both profiles |160M document units (`tenjiku_document.rs:941–1072`), separate native1MiB/1,024-event/512M limits | Actual eight actor/ending combinations, early/final Close and prevalidated Close (`:665`) retain their measured reservations; rejected provisional600M policy is not counted as successful evidence. |

The cooperative15-second file/admission-worker budget is unchanged. The deadline
classifier preserves an already committed atomic Save instead of falsely
reporting post-rename failure (`persistence.rs:285–304/357–391`). Atomic publication
uses an exclusive same-directory temporary file, flush/sync, cancellation before
rename and error cleanup (`:1105`). Failed capacity admission or invalid/stale document replacement preserves
accepted native state, complete history/branches/observations, dirty/path/runtime
authority and prior-file bytes; progress/I/O receipt revisions may legitimately
advance to report that failure. Reusable successful Full-certificate validation
is bounded to one operation and never stored as a trusted flag across Open.

The existing game-specific records linked by [verification.json](verification.json)
preserve mandatory successful representative measurements and typed near-limit
refusals separately. This review inspected those policies and their consumers;
it did not increase a limit, promise every near-limit operation succeeds, rerun
large measurements, or infer strength from performance. Engine request/response/
diagnostic bounds and watchdog/cleanup behavior remain separately enforced by
the existing supervisor.

## Findings and dispositions

| ID / severity | Required versus observed; affected scope | Evidence and bounded disposition | Blocks46? / regression |
| --- | --- | --- | --- |
| D89-01 low, resolved | Current public identity documentation must not describe delivered Xiangqi/Tori/Chu/Tenjiku functionality as unavailable. Four `GameKind` rustdoc comments still carried historical delivery gates. | `native.rs:30–36`; both fresh reviews independently found this. Replaced only four comments with current neutral profile descriptions. The header/record constructors correctly keep record wrapping separate from capability advertising and need no behavior change. | No material runtime blocker. Compare current capability/server consumers; formatting/diff checks. No new task, refactor or target-OS campaign. |
| D89-02 low, clarified | One application authority does not imply literally every New Game calls the Load installation function. The88matrix's generic replacement sentence could suggest that. | Narrowed README wording to current-file Load and admitted New paths under the same controller. `shared_archive.rs:1602` and `shared_xiangqi_archive.rs:1656` demonstrate the distinction; native/GUI cross-game tests cover actual behavior. | No. Documentation precision only; no structural uniformity refactor. |

No new unresolved material shared-contract finding remains. Required unavailable
checks would block completion; none was substituted or inferred as passed here.

### MSH-112 remains a separate unresolved functional diagnostic issue

MSH-112 is To Do, not fixed or reclassified as optional M5 optimization. Its
historical nonzero-exit fixture expected `UnexpectedExit` but observed unscored
`MissingResponse`/Broken pipe without exit status. Later unchanged-source green
runs or retries do not repair it. The preserved
[R4 recurrence](../../review/evidence/msh119/runner-112-recurrence.json) and
[R6 failure](../../review/evidence/msh121/checks/native176-linux-failure.log.gz)
remain linked; this review does not reproduce or fix the race.

Current `runner/run.rs:295–318` samples `observe_exit` after a write error and can
classify MissingResponse when no non-success status is yet observed;
`runner/process.rs:215–237` uses a separate bounded EOF/status path. This is
consistent with the recorded ordering problem, not a newly established root
cause. `runner/tests/runner.rs:153–180` still requires the meaningful categories
and unscored/no-conclusion behavior. No assertion or timeout was loosened.

The six-game desktop execution family is `mshogi-engine::EngineBridge`; neither
the host nor desktop-session runtime depends on `mshogi-runner`. Shared current
internal result/identity safety is separately supported by native/subprocess/GUI
evidence. The existing separately owned runner classification issue therefore
does not establish a material defect in these shared application contracts.
No new dependency gate or M2 reopening is invented. MSH-46/47 must retain this
honest disposition rather than claiming every process diagnostic is repaired.
MSH-112's existing bounded reproduction/classification/cleanup/independent
platform regressions remain the corrective recommendation;89 does not start it.

## Dependency and milestone integrity

Read-only Backlog1.55.1 `doctor` reports no duplicate IDs, self-dependencies or
cycles. Programmatic CLI checks resolve all19 selected records and their graphs,
verify required dependency sets and compare original acceptance text against
Git `3bcaf01`. Components69/72/73/76–79/90, game41–45 and88 remain Done. No
criterion text, original dependency, game scope or ownership is removed.

- 46 preserves39, all five41–45,69/72/73/76–79,88,89 and90.
- 89 preserves its88 prerequisite; it does not depend on46 and create a cycle.
- 47 preserves26/39/all five game deliveries/46/**108**.
- 108 preserves46/106/107; it remains the diagnostic architecture gate before47.
- 46 remains To Do/unassigned and unchecked. Completing89 makes its dependencies
 satisfied, not automatically Done.108 and47 remain gated; neither starts here.

M4 requires functional complete games/native semantics/shared products. M5 owns
measured strength, tuning, learned evaluation and later optimization; current
bounded functional engines are not claimed competitive. M6 owns the accepted
further accessibility/product/distribution polish, including release policy.
No incomplete M4 feature is made optional by that distinction. Full screen-reader
certification, release signing/notarization and guaranteed near-limit latency are
not manufactured obligations of this bounded acceptance review. Task percentages
and counts were not used to decide completeness.

## Verification and limitations

New executed checks: focused native11/11; Backlog doctor; selected DAG/original
criterion preservation; exact source diff; retained-file/report/package hashes
and native-profile/history/process assertions; formatting/diff checks. A reviewer
initially assumed Windows's `role` field existed in Linux process receipts;
the audit was corrected to exact executable paths from the actual receipt schema
and passed both platforms. That audit-script assumption is not an application
failure or a reason to repeat the product campaign.

Reused checks retain their original source/OS/attempt identity. Old failures,
zero-runner cancellations, superseded campaigns, MSH-112 recurrences and excluded
observer attempts are not erased by current success. No behavioral implementation, new three-platform CI, packaged GUI execution,
rules ratification or performance/
strength claim is implied by this documentation review.
