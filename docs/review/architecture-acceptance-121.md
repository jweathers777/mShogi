# MSH-121 / R6 integrated architecture acceptance

Status: **In Review — final source CI and acceptance bookkeeping pending**.
Root Codex owns integration on `codex/rewrite`; startup was clean/aligned at
`13510da9e24b69a340444f272668ecb59478ddbb`. All eight prerequisites
116–120/123–125 are Done. M4/46/47 remain open; no subsequent task is started.

## Verdict and source boundary

Fresh independent source, caller, semantic and packaged-artifact assessments
support the approved target. No material production defect was found. R6
corrected verification coverage/oracles, Windows display/placement/polling, and
macOS application-identity proliferation. It did not redesign the product or
change game rules, search limits, profile identities, archives or frontend UI.

Current reviewed QA source: `5a782bf78c19ebb31d89eed2c141c6715650f452`.
Normal production behavior remains unchanged from `13510da`; native deltas are
feature-gated exact-evidence construction, missing-engine executable suffix,
external startup fixture selection and native geometry observations. External
scripts are not bundled. Final source CI is recorded separately below.

The independent reviewers traced actual owners/callers and executed checks;
these are not an aggregation of old completion claims:

- [Native source/semantics/deletion review](evidence/msh121/checks/native-independent.md).
- [Frontend owner/interaction/product review](evidence/msh121/checks/frontend-independent.md).
- [External verifier findings and corrections](evidence/msh121/checks/verifier-independent.md).
- [Linux artifact review](evidence/msh121/checks/linux-final-independent.md),
  [Windows artifact review](evidence/msh121/checks/windows-final-independent.md),
  [macOS final artifact review](evidence/msh121/checks/macos-final-independent.md),
  [overall independent acceptance](evidence/msh121/checks/final-acceptance-independent.md).

## Acceptance matrix

| AC | Evidence and verdict |
| --- | --- |
| 1 approved ownership | PASS source review: one `DesktopController`, `RuntimeMetadata`, `ActiveDocument::{Chess,Shogi,Xiangqi}`, current execution4/durable observation1 and `mshogi.desktop.document.v1`, one frontend `ProductApp`/`useGameSession`, shared shell/lifecycle and `useNativeChoiceController`. Native rules/search/history and capability content remain specialized. |
| 2 deletion/callers/packages | PASS negative inventory below and independent reports: startup/host registrations/exports/imports/examples/tests/QA/builders inspected. Actual normal package smoke scans instrumentation and retired endpoints. No obsolete executable bridge is retained to satisfy old tests. |
| 3 native semantics | PASS current independently expected native fixtures/transcripts, actual child queries, exact-prefix/current-envelope/lineage checks and game-specific GUI capabilities. Chess Full cooperative proof/Unknown differs from forced win; Shogi advice differs from declaration; Xiangqi warnings/count claims and natural-win exact meanings retain native authority. |
| 4 safety/capacity | PASS current native session/workflow/admission tests, real pending cancellation/replacement/Take back, retained abandoned play controls, authoritative Save/Load, fresh stopped reopen and stale identity rejection. Actual byte/event/control/replay/combined ordinary+exact observation bounds, 256→257, and reserved stop/close/identity headroom are tested. Failed or stale I/O cannot install/clean newer work. |
| 5 actual three-platform GUI | PASS required consumer journeys: Windows/Linux each31 actual installed/packaged phases at `d725f25`; macOS actual normal and separate QA interactions at176/801, with final geometry at5a. No macOS GUI pass is inferred from CI packaging; no remote GUI is attributed to a reviewer who only inspected artifacts. Final CI/current package results are separately indexed. |
| 6 visual/accessibility/synthetic | PASS independent six normal/minimum cross-game comparisons; complete boards/player/hands/footer, stable shell/order and accessible controls. Actual pointer/keyboard/focus/native-dialog transcripts supplement pixels. Windows physical display/containment correction was actually executed. Synthetic routes remain explicitly invented; normal instrumentation exclusion is separate evidence. No personally operated VoiceOver certification is claimed. |
| 7 review/current source/integration | Independent findings resolved; exact tested source/package/environment and failed campaigns retained. Current final native/Desktop CI and final integration commit remain the bookkeeping gate before Done. MSH-112 is unresolved and not waived by later green CI. |
| 8 final architecture | PASS actual consumers and deletion audit; native enum specializations and labeled QA-only synthetic ownership are intentional, not split product applications. Historical evidence is preserved without old executable readers/transports. |
| 9 approved123/124/125 | PASS source/tests and actual product journeys: native standard two-column Moves/nonmove prefixes/geometry cues; supported-only dropdown/native side buttons; Analysis→Game Actions→Moves; support/license disclosure; first/subsequent Save/Load and intentional replacement/cancel; direct native Resign; concise dirty Quit; one-step ordinary browse Return versus stopped Resume/Retry. |

## Actual architecture and negative-reference audit

Native entry points in `desktop/src-tauri/src/main.rs` register one game handler
family and manage one controller and lifecycle. `mshogi-desktop-session/src/lib.rs`
owns common runtime metadata and one native-tagged active document. `shared.rs`
admits exact tokens and dispatches to native commands. `document.rs` retains
RawValue payload bytes for strict native replay; `persistence.rs::install_document`
is the sole current installation transaction. Engine `shared.rs`, `server.rs`
and `bridge.rs` use one current request/result family and supervised pending
slot, with game-specific purposes and meanings.

Frontend `main.tsx` mounts `ProductApp`; `useGameSession` owns subscription,
receipt/operation admission and retirement. `ApplicationView` supplies one
`ApplicationShell` and `GameFrame`, common actions/analysis/Moves/support, and
native capability content. The three renderers and synthetic board use one
`useNativeChoiceController`. Stateless geometry/projection helpers do not own
another session or implement rules in TypeScript.

Independent searches covered production, crate exports, host handlers,
examples, tests, QA and build/workflow callers. Absent executable families:
`WireRequest::Legacy`/`WireResponse::Legacy`, V2/fallback/chess_adapter execution,
paired submit_shared/submit_chess public families, Archive/ArchiveV2/old desktop
readers, decode_chess_compatible, old desktop_snapshot/command/save_as/open IPC,
useNativeSession, R2 capture/install bridges and R3 frontend forwarders.
Historical docs/rejection tests retain names intentionally. Private native
`poll_chess`, current `ArchivedObservation`, console external `LegacyLoad`, and
stateless `NativeBoard` are not obsolete development interfaces.

## Native semantics, documents and execution

Independent current checks executed45 native rules/history fixture tests,
22 current execution-boundary tests,15 actual child-process tests and212 native
session tests (294 total in that initial command set). Later focused28 native
checks, actual exact fixture and public-API warning/lineage probes overlap that
set; they are not presented as additional unique conformance cases. The added
exact-evidence fixture raises the QA session suite to213; current CI supplies
full applicable regressions. Root frontend suite:322 tests/26files plus strict
TypeScript; independent frontend subset158/11files is separately recorded.

The native fixtures assert complete castle/en-passant/four-promotion actions,
Shogi LegalAll promotion/drop/capture/king-safety facts, and reviewed WXF cycle,
count and exact-universe expectations. Existing cshogi/rules-source work is
reused unchanged; no new comparator campaign or strength benchmark is claimed.

Same diagrams with distinct known history remain different contexts. Xiangqi
warning event8 is an immutable partial command prefix with live canonical9;
event9 has a committed selected canonical record and warning. No forged
canonical prefix is invented. Take back retires execution and retains abandoned
Chess match records or native Shogi/Xiangqi initial+ordered play→take_back
controls. Actual saved remote artifacts were independently reopened/replayed
and saved again, preserving all ten controls/settings/observations while
establishing fresh stopped runtime. Return from ordinary browsing is distinct
from explicit Resume after analysis/cancel/failure/Load/Take back.

Admission uses actual encoded envelopes and native replay work, complete events,
controls and combined Xiangqi observations. Native typed resource facts drive
truthful UI messages. Actual macOS zero-observation control exhaustion preserved
Save and produced an unmodified complete document. Saving/reopening is not
suggested as a structural-limit reset. Atomic temp-write/flush/rename and exact
receipt tests preserve valid files, dirty state and live sessions on failure.

The new QA-only Xiangqi evidence uses an independently reviewed root and actual
legal f9e9/a8e8/e9e8 continuation (3 board plies,4 native events). Five real native
reports preserve: possible Red natural-board-win witness (not forced), exact
neither-can-win proof, budget Unknown, cancelled Unknown, and unsupported-limit
failure. Targets0/4 use correct prefixes and normal admission. Native public-API
and actual GUI Save/Load retain them as historical evidence, not outcomes.
Xiangqi live exact-start UI is not introduced; exact execution remains explicit
native/console functionality. Ordinary consumers remain sound-fast-only.

## Packaged application evidence

[Remote index](evidence/msh121/platforms/d725-index.json) records exact CI/artifact
IDs, package/host/engine hashes, environments, cleanup and archive hashes.
Exact original reports are gzip-preserved with original SHA256 and selected
normal/minimum native captures, package manifests and audits. Raw installers
and all captures remain reproducibly referenced by their CI artifacts; build
output is not checked into Git. Windows CRLF metadata and selected log views
are whitespace-normalized for repository checks; exact original bytes are
compressed alongside with SHA256 in
[text normalization provenance](evidence/msh121/text-normalization.json).

| Platform | Actual execution established |
| --- | --- |
| Windows2022Server, WebView2/driver versions in artifact | `d725f25`, Desktop41237789041:31/31 actual installed normal/QA phases;599 hashes verified,31 native initial placements size-preserved,243 native captures contained. Enumerated1920×1080 mode, TEST0/APPLY0, workarea1920×1040. Six normal/minimum physical views independently reviewed. |
| Ubuntu24.04, WebKit/X11/Openbox | Same source/run:31/31 actual installed normal/separate packaged QA phases;700 hashes verified, actual native cleanup and six normal/minimum views. GDK x11 and WEBKIT_DISABLE_COMPOSITING_MODE=1 are recorded environment qualifications, not a general hardware-driver fix. |
| Local macOS27.0.1 arm64 | Root CUA-operated normal176 package for all3 games; separate QA176 native special/history/Full/exact/cancel cases, fixed QA801 claim/recovery, final5a six native geometry comparisons. [Campaign summary](evidence/msh121/macos/campaign-summary.json), actual action/AX log, screenshots and package/runtime manifests distinguish each. |

Remote31 phases include normal all3 play, native special choices, current and
intended claims, declaration, warnings/fourth/count/offer/agreement, fast/exact
historical meanings, pending history/Return/Take back/cancel/replacement,
Save/Load/fresh stopped Resume, failure/retry, capacity and synthetic actions.
Chess Full proof and Unknown/cancel are actual native GUI executions. Castling
and en-passant conformance are native fixture evidence, not newly executed GUI
moves. Each passed phase observes host/engine/gate exit before driver teardown;
forced cleanup is never counted as native Quit.

macOS has normal all3 genuine engine replies and bounded analysis, keyboard
play, native notation/history/Return, first/subsequent Save, native picker
cancellation, Load stopped/Resume, Take back and unsaved Quit Save/Discard/Cancel.
Separate QA adds cancelled/committed promotions, Shogi hand inspection/drop,
entering-king declaration, current/intended Chess claims, Xiangqi exact warning
prefixes/count and five historical exact reports through Save/Load, actual Full
proof/zero-budget Unknown/cancel, all3 genuine pending Take back/replacement/
cancel and restored-real-engine Retry. Synthetic same-endpoint distinct routes,
partial cancellation, keyboard/orientation and native close remain isolated.

Final macOS captures are `final-{chess,shogi,xiangqi}-{1180,760}.jpg`.
Native inner/outer sizes are separately queried and DPI-converted; this host
reports both1180×820 or760×650. Raw images are2360×1640 or1520×1300 at2x. These are
observations, not an inference that native inner/frame/DOM viewport are always
equal. Initial raw JPEGs retain their historical `.png` filenames, with actual
format declared in the manifest. No pixels were edited. A recorder's stale176
source label was corrected through an explicit range overlay while retaining
the exact original log gzip and each original label; UI observations/actions/
capture bytes were not changed. [Source overlay](evidence/msh121/macos/source-attribution-overlay.json).

## Verification findings and honest failure history

No material product finding remains from fresh independent review. Resolved
verification findings, with originals preserved under platforms/history:

- 176 campaigns27/31,0b16928/31: warning partial-prefix canonical assumption;
  Chess Match Start event offset; Shogi/Xiangqi control-based lineage wrongly
  treated as a Chess takebacks array. Actual native public APIs/replay and
  independent expectations established corrections, not implementation agreement.
- 368 campaigns31 semantic passes but Windows1024×768 physical display cropped
  normal windows. Full webview layout alone was not accepted as physical proof.
- e778 Windows26/31: sufficient1920×1080 mode exposed four QA cascade placements
  outside workarea and one diagnostics polling timeout after real retry success.
  Fixed supported-mode setup, size-preserving placement/strict capture guard and
  read-only completion polling before one diagnostic read. Fresh d725 passed all31.
- Windows gate ownership was fixed before spawn through a kill-on-close job;
  actual Windows process tests and cleanup evidence verify it. The external
  bounded5s gate executes the unchanged real engine, never fabricated results.
- Native176 Linux runner expected UnexpectedExit but observed unscored
  MissingResponse/handshake/Broken pipe for an exiting child. The original
  failure remains [recorded](evidence/msh121/checks/native176-linux-failure.log.gz).
  Later all3 green runs do not establish an MSH-112 fix. No runner code changed.

Earlier P9/MSH-111/122 environment failures and M1/M2/M3/game delivery evidence
remain historical and qualified. Passing this campaign does not erase them.

## macOS approval investigation and durable maintenance

The author unnecessarily created20 bundles/IDs for fixture scenarios despite
identical QA machine code. Actual saved Computer Use approvals contained exact
approvedBundleIdentifiers for individual clones, not blanket mShogi approval.
Independent code-signing audit found ad-hoc/no TeamIdentifier signatures and
hash-specific designated requirements; original base linker signatures did not
establish complete resource sealing. Those facts are separately recorded.

The maintained solution uses three fixed normal/QA/synthetic slots, deliberately
reuses existing approved legacy QA/synthetic IDs, and signs a staged build once.
A bounded external fixture token changes scenario while stopped without
changing any app bytes or signature. Running replacement/selection is refused.
Missing/malformed configured selectors fail explicitly; no fallback game is
invented. Normal packages exclude the reader/markers. No approval store,
macOS security setting or Computer Use policy was edited.

Actual fixed-path reuse, changed executable/signature rebuild reuse, repeated
unchanged-bundle scenario changes and separate synthetic reuse completed without
an observed new approval interruption. The saved-approval file hash stayed
unchanged. All signed bytes were identical across scenario selection; the real
engine was restored byte-for-byte after failure probes and strict verification
passed. [Actual reuse evidence](evidence/msh121/macos/stable-rebuild-reuse-observation.json).
All20 completed scratch bundles were removed after matching their recorded
identities/host hashes and confirming no process used them. The fixed reusable
slots remain; saved approvals were not deleted or edited.
This establishes behavior of the recorded installed version/session, not a
promise about future managed policies, versions/accounts or rebuilds.
Instructions and tests are in AGENTS.md, desktop/README.md,
[packaged verification](../desktop/packaged-gui-verification.md) and the fixed
macOS helper. Root authored the QA reader; a separate agent authored the isolated
helper, and the native reviewer independently tested both (11 helper tests,
13 host tests and Clippy). No reviewer approved its own implementation.

## Final review, CI and handoff

Current final native/Desktop CI and overall acceptance disposition are pending
completion of the exact-source runs and final independent report review. Source
checks and passed d725 campaigns above remain valid evidence for unchanged
production behavior, not a fabricated pass for a later package.

No competitive-strength claim, timed desktop delivery, future variant delivery,
notarization or full screen-reader certification is made. MSH-112 remains open;
MSH-46/47/M4 and later architecture/game/M5 tasks are not closed or started here.
Normal macOS launch remains the primary checkout's `desktop/src-tauri/target/release/bundle/macos/mShogi.app`;
build with `npm --prefix desktop run desktop:build -- --bundles app` using the
recorded Node/Rust versions. QA preparation does not replace the normal app.
