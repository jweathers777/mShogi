# Six-game shared integration — MSH-88

MSH-88 verifies the delivered games together. Its single structured record is
[verification.json](verification.json). MSH-88 is Done after final independent review and normal evidence integration
at `e68e8e3d1787f2d029e3601259070807bdbab180`. MSH-89 retains the later
independent shared-contract acceptance; MSH-46 and MSH-47 remain open. This
current map supersedes the implementation assumptions in the preserved
pre-R1 Shogi planning tables, without reviving retired development readers.

## Current owners and supported identities

One `DesktopController` owns one `ActiveDocument` and `RuntimeMetadata`.
`install_document` is the guarded replacement transaction. `ProductApp` calls
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

| Original obligation | Current shared implementation and owner | Executed evidence / MSH-88 addition | Qualification or remaining owner |
| --- | --- | --- | --- |
| AC1: all-six library, console, engine, session; explicit profile/version, complete actions/history, atomic claims/actions, identity/cancel/stale and bounded work | `mshogi-core/src/native.rs`; native per-game rules/search; `mshogi-console`; `mshogi-engine/src/shared.rs`, `bridge.rs`; `mshogi-desktop-session/src/lib.rs`, `shared.rs`; shared 69/72/76–79 plus game 41–45 | Accepted game records above; current native/console/execution suites; new `six_game_integration_tests.rs` seven-profile same-controller New/reverse-Load ring; actual normal and delayed-engine cross-game GUI phases | No frontend legality or Python/Node product runtime. Manual native allocation proves rejection, separately from real bundled subprocess receipts. Functional strength only; M5 remains later. |
| AC2: current canonical Save/Open all-six, strict unsupported/lossy rejection, original Chess modes/records | `document.rs` envelope/RawValue; `persistence.rs` atomic worker and guarded installation; concrete native document codecs; 73/117/118 and each game adaptation | Accepted per-game document/resource measurements; all-seven current-envelope New/Save/Load; malformed Load preserves pending authority and prior bytes; ordinary/Full Chess evidence, timed/live-Full refusal in existing mandatory native suites | Only `mshogi.desktop.document.v1`; retired desktop archive1/2/3 and internal execution1/2/3 reject. Native Chess schemas1/2 retain their own supported meanings. No development compatibility converter. |
| AC3: real geometry/actions, hands/drops/promotion/compound/profile choices, keyboard and readable text, no partial turn | `ProductApp`, `ApplicationShell`, `useGameSession`, `useNativeChoiceController`; native legal choices and inspection; per-game board renderers; 72/76–79/119/120 plus41–45 | Accepted real special-action fixtures; final shared UI tests and actual packaged baseline plus new same-host journeys; selected friendly names, above-art capture rings, pointer-transparent decoration, Escape/inspection/history | Native effects supply capture/burn/royal conclusions. Minimum760×650 unchanged; dense Tenjiku kanji and secondary badges remain limited; about960×1000 recommended. Labels/keyboard checks do not establish complete assistive-technology access. |
| AC4: explicit shared/per-game ownership and cross-game flows, versions and Chess transitions | This matrix; shared mechanisms46 through69/72/73/76–79/116–125; per-game adaptations remain41–45 | New normal same-host Chess→Shogi→Xiangqi→Tori→Chu→Adams→Muller→Adams→Chess; reverse cross-game Load and stopped Resume/history/Return/Take back; new direct stale-token regression | MSH-88 owns integrated execution evidence. MSH-89 later reconciles contracts;46 is not Done from88 alone. |
| AC5: actual installed/packaged binaries and bundled engines on all3OS, six-game play/cancel/stale/persistence/outcome/recovery/accessibility, exact provenance | Existing fixed normal/QA/synthetic launch methods and `.github/workflows/desktop.yml`; 88 | Final Windows/Linux actual native-WebDriver campaign and local macOS fixed-slot CUA campaign, exact package hashes/actions/source/CI receipts in `verification.json`; QA isolated, normal instrumentation exclusion required | Hosted macOS15 package/native CI and local macOS27 actual interaction are distinct. No browser/unit/package-only substitution; no release signing/notarization or screen-reader audio claim. All six required CI jobs and Windows/Linux49-phase reports pass at `bc92bfe`; local Mac evidence is separately qualified. |
| AC6: independent shared integration acceptance and complete contract reconciliation | MSH-89, then MSH-46; later MSH-108/47 gates unchanged | MSH-88 obtains its own fresh bounded source and platform-evidence review | Not completed by this task. No M4 closure, architecture refactor or M5 work. |

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
pushed at `e68e8e3`; MSH-88 is Done. The later MSH-89/46/47 gates remain open.
