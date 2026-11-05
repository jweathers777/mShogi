# Six-game shared delivery — MSH-46

MSH-46 reconciles the original six shared-delivery obligations using the completed
component, game, integrated-platform and independent-contract evidence. Its final
completion review is approved; normal evidence integration and final task closure
are pending. No executable source changes are part of this
aggregate acceptance. MSH-88 verifies the delivered games together. Its single structured record is
[verification.json](verification.json). MSH-88 is Done after final independent review and normal evidence integration
at `e68e8e3d1787f2d029e3601259070807bdbab180`. The fresh MSH-89
[contract reconciliation](contract-review-89.md) and compact
[review index](contract-review-89.json) support all six original MSH-46
obligations. MSH-89 is Done; its report was normally integrated/pushed at
`22b4f40`, with final acceptance integrated at
`91115368c6e99f344dee002d9264f4d0b62cc76b`. MSH-108 and MSH-47 retain their
separate architecture-diagnostic and whole-M4 closure obligations. This
current map supersedes the implementation assumptions in the preserved
pre-R1 Shogi planning tables, without reviving retired development readers.

## Current owners and supported identities

One `DesktopController` owns one `ActiveDocument` and `RuntimeMetadata`.
`install_document` is the guarded current-file Load installation transaction.
New Game uses admitted game-specific replacement paths under that same
controller and active-document owner. `ProductApp` calls
one `useGameSession`; one native-choice controller owns board input lifetime.
Native rules, complete actions, effects, history and outcomes remain game-owned.

| Game | Immutable profile | Complete delivery / shared adaptation | Accepted evidence reused for unchanged behavior |
| --- | --- | --- | --- |
| Chess | `mshogi.chess.match.v1` | M2/M3; 69/72/73; 116–125 | [R6 acceptance](../../review/architecture-acceptance-121.md), [current document](../../desktop/document-v1.md) |
| Shogi | `mshogi.shogi.csa36.v1` | 41/75; 69/72/73 | [delivery review](../shogi-delivery-review.md) |
| Xiangqi | `mshogi.xiangqi.wxf2018.v1` | 45/103; 76/95/98/99 | [delivery acceptance](../xiangqi/delivery-acceptance.md), [delivery evidence](../xiangqi/delivery-review-evidence.json) |
| Tori Shogi | `mshogi.tori.modern.v1` | 42; 77/130/131 | [delivery review](../tori/delivery-review.md), [runtime evidence](../tori/desktop-runtime-evidence.md) |
| Chu Shogi | `mshogi.chu.jcsa2019.v1` | 43; 78 | [delivery review](../chu/delivery-review.md), [shared evidence](../chu/shared-verification.json) |
| Tenjiku Shogi | `mshogi.tenjiku.adams1999-modern.v1` | 44; 83/87/79 | [product evidence](../tenjiku/product-verification.json), [shared evidence](../tenjiku/shared-verification.json) |
| Tenjiku Shogi | `mshogi.tenjiku.muller-completed.v1` | same game, distinct whole profile | same accepted records, with explicit profile-qualified receipts |

These seven profiles are six games. The Tenjiku selection is immutable for each
document, not independent rule toggles. Their exact editions travel in native
records, execution contexts, desktop envelopes and snapshots.

## Original MSH-46 acceptance and ownership matrix

Each row preserves the original obligation. MSH-88 evidence supplements the
linked game records; it does not replace native conformance or MSH-89 review.
All six criteria are supported at the exact sources qualified below. The
[AC1–6 source assessment](contract-review-89.md) (the six named AC sections)
independently checks the concrete symbols, tests and original receipts for every
row; completed child-task status alone is not the acceptance basis.

| Original obligation | Current shared implementation and owner | Executed evidence / MSH-88 addition | Qualification or remaining owner |
| --- | --- | --- | --- |
| AC1: all-six library, console, engine, session; explicit profile/version, complete actions/history, atomic claims/actions, identity/cancel/stale and bounded work | `mshogi-core/src/native.rs`; native per-game rules/search; `mshogi-console`; `mshogi-engine/src/shared.rs`, `bridge.rs`; `mshogi-desktop-session/src/lib.rs`, `shared.rs`; shared 69/72/76–79 plus game 41–45 | Accepted game records above; current native/console/execution suites; new `six_game_integration_tests.rs` seven-profile same-controller New/reverse-Load ring; actual normal and delayed-engine cross-game GUI phases | No frontend legality or Python/Node product runtime. Manual native allocation proves rejection, separately from real bundled subprocess receipts. Functional strength only; M5 remains later. |
| AC2: current canonical Save/Open all-six, strict unsupported/lossy rejection, original Chess modes/records | `document.rs` envelope/RawValue; `persistence.rs` atomic worker and guarded installation; concrete native document codecs; 73/117/118 and each game adaptation | Accepted per-game document/resource measurements; all-seven current-envelope New/Save/Load; malformed Load preserves pending authority and prior bytes; ordinary/Full Chess evidence, timed/live-Full refusal in existing mandatory native suites | Only `mshogi.desktop.document.v1`; retired desktop archive1/2/3 and internal execution1/2/3 reject. Native Chess schemas1/2 retain their own supported meanings. No development compatibility converter. |
| AC3: real geometry/actions, hands/drops/promotion/compound/profile choices, keyboard and readable text, no partial turn | `ProductApp`, `ApplicationShell`, `useGameSession`, `useNativeChoiceController`; native legal choices and inspection; per-game board renderers; 72/76–79/119/120 plus41–45 | Accepted real special-action fixtures; final shared UI tests and actual packaged baseline plus new same-host journeys; selected friendly names, above-art capture rings, pointer-transparent decoration, Escape/inspection/history | Native effects supply capture/burn/royal conclusions. Minimum760×650 unchanged; dense Tenjiku kanji and secondary badges remain limited; about960×1000 recommended. Labels/keyboard checks do not establish complete assistive-technology access. |
| AC4: explicit shared/per-game ownership and cross-game flows, versions and Chess transitions | This matrix; shared mechanisms46 through69/72/73/76–79/116–125; per-game adaptations remain41–45 | New normal same-host Chess→Shogi→Xiangqi→Tori→Chu→Adams→Muller→Adams→Chess; reverse cross-game Load and stopped Resume/history/Return/Take back; new direct stale-token regression | MSH-88 owns integrated execution evidence. MSH-89 supplies the subsequent independent contract reconciliation;46 retains its own completion workflow. |
| AC5: actual installed/packaged binaries and bundled engines on all3OS, six-game play/cancel/stale/persistence/outcome/recovery/accessibility, exact provenance | Existing fixed normal/QA/synthetic launch methods and `.github/workflows/desktop.yml`; 88 | Final Windows/Linux actual native-WebDriver campaign and local macOS fixed-slot CUA campaign, exact package hashes/actions/source/CI receipts in `verification.json`; QA isolated, normal instrumentation exclusion required | Hosted macOS15 package/native CI and local macOS27 actual interaction are distinct. No browser/unit/package-only substitution; no release signing/notarization or screen-reader audio claim. All six required CI jobs and Windows/Linux49-phase reports pass at `bc92bfe`; local Mac evidence is separately qualified. |
| AC6: independent shared integration acceptance and complete contract reconciliation | MSH-89 and final MSH-46 acceptance; later MSH-108/47 gates unchanged | MSH-88 source/platform review plus fresh MSH-89 [contract assessment](contract-review-89.md) and independent findings/DAG review; fresh final MSH-46 completion assessment below | No unresolved material shared-contract finding. MSH-112 remains separately open as qualified below; no M4 closure, architecture refactor or M5 work. |

## Resource and compatibility constraints

The current execution family is protocol4 with durable observation schema1.
Native payloads preserve complete actions, histories, administrative events,
observations and retained Take back branches. Load installs fresh stopped
authority; saved observations cannot resume requests. Strict identity includes
the whole profile/history/purpose/target/request/session/process/limit context.

Existing per-game budgets and ending/Close reserves remain unchanged. Tenjiku
retains1MiB native record,1,024 events,512M native deterministic units,160M whole
document reconstruction units and the15-second worker deadline. Mandatory
successful measured journeys remain distinct from near-limit typed refusals.
Unchanged per-game measurements are reused with original source provenance;
the new controller and actual cross-game paths supply additional evidence.

The final record must preserve cancelled/superseded attempts and failed checks,
exact source and package identity, native terminal versus incomplete/failure
results, and local/hosted macOS separation. All required receipts and fresh independent final reconciliation support
functional/platform acceptance. The complete packet was normally committed and
pushed at `e68e8e3`; MSH-88 is Done. The subsequent MSH-89 contract assessment is
independently approved. MSH-46 closes only shared delivery;108/47 remain separate gates.

## Exact accepted source and platform receipts

The final runtime/platform campaign tested
`bc92bfe5e3b20e9e999df12cbf54b91682c71b8d`. Unchanged game implementation is
`eca9f9cfd43cf83ff4e3489292f6c25f8951bb30`; MSH-88 added integration tests and
external verification. The MSH-46 starting integration is
`91115368c6e99f344dee002d9264f4d0b62cc76b`. Between the tested source and that
integration, the only runtime-tree diff is four neutral `GameKind` rustdoc
descriptions in `crates/mshogi-core/src/native.rs`. There is no executable,
test, frontend, host, observer, workflow, schema, rule or limit change. This
closure changes documentation and task records only, and claims no binary built
or target-OS checks newly executed at a documentation revision.

| Required platform | Rust run42028972679 / attempt1 | Desktop run42028972586 / attempt1 |
| --- | --- | --- |
| Ubuntu24.04 | [127250225725 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972679/job/127250225725) | [127250225410 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972586/job/127250225410) |
| Windows2022 | [127250225944 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972679/job/127250225944) | [127250225967 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972586/job/127250225967) |
| macOS15 | [127250225640 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972679/job/127250225640) | [127250226038 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972586/job/127250226038) |

All six raw job receipts share the exact tested source. Original Windows/Linux
reports each contain49 passed actual packaged GUI phases and no failures;
labelled synthetic phases do not supply real-game proof. Seven-profile normal
play and delayed-engine cross-game transitions supply that proof. Local
macOS27.0.1 arm64 actual gameplay is separate from hosted macOS15 native/package
CI. Its four normal bundle files match the tested-source rebuild byte-for-byte;
fixed normal/QA identities, signature qualifications, excluded observer attempts
and final process absence remain in the unchanged platform record.

The master [verification.json](verification.json) remains byte-for-byte unchanged
with SHA256 `097118de9a7c9507900f554da7c91dde97ad35729bce73c9f491b52a7a9cd985`.
Its202 retained evidence hashes, failed/cancelled/superseded runs and original
package/report identities remain authoritative. The MSH-89 report/index are also
preserved as their historical checkpoint; their references to46 being open
describe that checkpoint. This README is the current aggregate completion record,
not a second platform ledger.

## Final completion assessment and downstream handoff

The MSH-46 integrator reconciled every original criterion against the matrix,
the source-qualified MSH-89 AC1–6 assessment, the original MSH-88 raw platform
receipts and the linked game41–45 acceptance records. The single current
controller/document/execution/frontend/session/shell/native-choice owners remain
as mapped above; native semantics remain game-owned. Compatibility, resource and
ending/Close reservations remain exactly as accepted in
[MSH-89](contract-review-89.md#execution-saveability-and-resource-accounting).
Native conformance, search correctness, execution performance, functional
integration and competitive strength retain their separate meanings.

Fresh independent final review: **approved, no material blocker** by
`msh46_completion_review`, a new read-only reviewer without the integrator's chat
history. Reviewed HEAD was `91115368c6e99f344dee002d9264f4d0b62cc76b` plus the
six-file closure candidate: this README, delivery-plan/implementation-map and
tasks46/108/47 (candidate diff SHA256
`27e8e364e59b2a4b1bab3f09b6225d837cd5b991ea1b55f9d5761cced28c8322`).
The reviewer independently checked all six original criteria,
DoD1–3, source equivalence,202 hashes/raw reports,16 completed prerequisites,
original46/108/47 criteria/dependencies and unchanged downstream ownership.
DoD4 is conditional only on ordinary evidence integration/push and the durable
completion summary; neither downstream task nor M4 is closed by this verdict.

New closure checks passed: master SHA/all202 retained hashes; six raw job receipts,
runner acquisition, source/attempt and successful-or-skipped step conclusions;
original Windows/Linux decompressed report hashes with49/49 phases each;
original AC/DoD texts, dependency sets and non46 ownership for19 selected records
against Git `9111536`; separate complete16-prerequisite status lookup (including39),
106/107 Done,112 open;113 resolved graph nodes;86 local documentation link targets;
compatible1.55.1 `backlog doctor`; and `git diff --check`.
The first closure audit harness omitted39 from its selected19-record dictionary
and stopped with `KeyError`; the complete prerequisite lookup was corrected and
passed. This was an audit-script selection error, not an application failure,
product change or reason to repeat a product campaign.

These checks are limited to
source equivalence, evidence hashes/raw receipts, original criteria/DoD/dependency
integrity, local links, `backlog doctor` with compatible1.55.1 and `git diff --check`.
They run on local macOS27.0.1 arm64 with Python3; existing MSH-89 native11/11 and
the platform campaign retain their original source/OS provenance. No broad
native/frontend/GUI/CI campaign is repeated for this aggregate bookkeeping.

All16 original dependencies remain unchanged:
MSH-39,41,42,43,44,45,69,72,73,76,77,78,79,88,89,90. They are Done, and game-specific
adaptations remain with41–45. MSH-108 still depends on46/106/107; after46 completion
it is the next dependency-ready diagnostic assignment, unassigned and unstarted.
MSH-47 still depends on26/39/41–46/**108**, remains To Do and owns independent
whole-M4 acceptance. Neither task is activated here; M4 remains open.

MSH-112 remains **To Do and unfixed**, a separately owned functional runner
diagnostic race. Later successful retries do not fix it. Its recorded unscored
MissingResponse/UnexpectedExit distinction, recurrence evidence and rationale
for not blocking these shared application contracts are preserved in
[the independent disposition](contract-review-89.md#msh-112-remains-a-separate-unresolved-functional-diagnostic-issue).
No assertion, timeout, resource budget or acceptance requirement is weakened.
Dense Tenjiku labels at the owner-approved760×650 minimum, lack of comprehensive
screen-reader certification, development signing/notarization qualifications and
untuned functional engines remain explicit limitations. None establishes a new
M4 blocker or a playing-strength claim. MSH-53 and the other M5 strength owners
remain later work; no advisory architecture recommendation becomes a refactor
prerequisite here.
