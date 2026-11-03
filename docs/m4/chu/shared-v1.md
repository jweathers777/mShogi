> Current product follow-through: MSH-43 supplies actual native Chu execution,
> the owner-approved traditional-kanji renderer and ordinary fifth-game New/Load.
> [Delivery reconciliation](delivery-review.md) records the final evidence and
> current status. The MSH-78 checkpoint below intentionally preserves its former
> unsupported/gated product scope; the codec, limits and semantic ownership remain
> authoritative. Its consumer table is a historical handoff, supplemented by43.

# MSH-78 Chu shared consumer contract

The accepted `mshogi.chu.jcsa2019.v1` profile/edition and native record1 remain
unchanged. MSH-86's independently expected corpus and native evidence retain their
original revisions. This extension consumes that kernel; it does not complete
MSH-43's engine, console, notation or playable desktop product.

## Routing and ownership

The existing tagged `GameKind`, `RulesProfile`, `NativeSchema`, `NativeSide`,
`CanonicalRecord` and command dispatch now have explicit Chu cases. Native sides
are Sente/Gote. The original/base identity and promoted movement face remain
separate. Coordinates in a native Action serialize as `[file,rank]`; presentation
coordinates such as `12l` are opaque board keys. Ordered one/two-landing routes,
quiet/capture returns and every intermediate victim remain authoritative native
values. Complete action JSON is an opaque choice identity, never parsed for rules
by TypeScript. Native Game owns actors, terminal-command admission, Lion rights,
repetition obligations, royal outcomes and finite mate. No Chess self-check,
Shogi drops/hands, forced-mate search or shared rules model is introduced.

Internal protocol4, canonical envelope1, native Chu schema1 and current desktop
`mshogi.desktop.document.v1` remain distinct identities. Adding an explicit game
tag requires a consumer that recognizes Chu; older consumers reject it rather
than converting it. Chess/Shogi/Xiangqi/Tori payloads and saved observations retain
their representations. Native record parsing uses RawValue; recursive duplicate
checking precedes buffered tagged deserialization. Unknown, missing, conflicting,
forged or unsupported identities reject. Native replay checks redundant event,
effect, obligation and conclusion data, rather than trusting serialized summaries.

One `DesktopController`/`ActiveDocument`, frontend session owner, installation
owner and existing native/file workers remain. Chu adds an immutable validated
payload adapter. Tori/Chu share transaction scheduling/installation plumbing, not
rules. Full candidate admission runs off the application thread before install.
Failed replacement preserves the accepted document and execution authority;
successful changes retire old work. Native Undo and desktop Take back retain full
retired canonical branches. Settings preserve the historical selection; Return to
live is explicit. Exact event prefixes differ from board ply, including draw and
Stop events. Reopening restores selected history and settings with fresh runtime
identities, stopped and without resumable pending work. Observations remain
historical, including after Take back.

## Lossless bounded document policy

The payload stores initial/current native records, human assignment, initial and
current orientation, complete journal, indexed observations, retired branches,
selected event and an execution high-water floor. Replay validates their exact
correspondence. Existing atomic file writing, receipt/dirty-state rules, file and
close reservations remain shared. Accepted operations cannot discard records or
observations to fit. Resource refusal leaves the previous complete session and
valid file intact, with typed resource facts and existing truthful recovery UI.
Reopening does not reset structural usage.

Native `Counter` refusal covers both board counters and runtime identities; it
cannot identify a particular exhausted resource. Its adapter therefore preserves
the existing typed Unknown resource fact with Chu scope and the native diagnostic.
It does not invent an identity limit, illegal action or concluded game. The exact
maximum declared-counter probe preserves the document and its Stop/Resign/Close
save path after refusal.

Outer limits stay **16 MiB encoded input/document, 4,096 retained native events,
8,192 controls, 256 observations, and the 15-second worker deadline**. Every
separately retained initial/current/retired/observation record contributes its
actual native event count. Native limits stay 512 KiB,512 events and4M work.

Chu additionally admits at most **12,000,000 deterministic aggregate replay
units**, including reserved endings. This supports roughly three native maximum
replays for an ordinarily grown journal, rather than copying Tori's budget.
`GameRecord::reconstruction_work()` is a private native-derived cache exposed as a
read-only fact, absent from JSON; no serialized cheap-cost assertion is trusted.
Accounting charges raw parsing of every retained record, initial reconstruction,
actual journal deltas, both rewind-prefix reconstructions, observation-prefix
reconstruction, current runtime installation and selected-prefix reconstruction.
A native call is independently bounded by4M; cancellation/deadline is checked
between native calls. Malformed input can consume at most one additional bounded
record invocation before aggregate refusal. Local threat/choice presentation is
bounded native computation cached on the worker, not recomputed by rendering.
The elapsed deadline independently refuses a candidate; work units are not a
promise of equal wall-clock speed on every machine.

Measurements on macOS27.0.1 arm64, Rust1.99.1 debug, real complete candidates:

| Candidate | Native events/bytes/work | Desktop bytes/retained events/work | Decode + runtime restore |
|---|---|---|---|
| Standard32 deterministic actions, seed17 |32 /27,847 /16,284 |41,901 /32 /48,921 |272+127ms |
| 96-action history, seed3917, observation, Take back to48, selection24 |96 /57,109 /310,231 |159,116 /240 /1,416,949 |1,270+123ms |
| Native near-limit227 history grown from root |227 /132,199 /3,979,825 |161,812 /227 /11,939,711 |2,574+1,248ms |

These are observations, not performance guarantees. The same227-event record
imported as initial history requires about four full native replays and is
refused explicitly at aggregate work capacity. This preserves a save path rather
than accepting an unsaveable import. Short compound, capture-return, promotion,
royal-change, draw, repetition and branch journeys are also exercised through
actual controller admission. No constant was raised.

Admission reserves actual complete encoded endings for both actors and the
widest runtime-floor spelling, including early Close/reopen, Stop, Resign and
final Close. `null` is the longest supported selected-prefix spelling. Native
Stop/Resign admission is used, not estimated piece/event sizes. Close consumes a
prevalidated immutable candidate without replay on Quit. Focused tests cover
256→257 observations,4,094 retained events plus two ending events, control and
byte boundaries, all actor combinations and null/short selected prefixes.

## Presentation and unsupported execution

The existing board/choice controller consumes 144 native squares, both
orientations, complete route alternatives, terminal prefixes, promotion
completion, partial selection, Escape, keyboard, stale selection and read-only
inspection. `CompletedActionGeometry.captures`, omitted for the existing games,
retains each victim's native owner/base/face/promotion in capture order. The cue
renders intermediate route/capture positions without granting action authority.
Moves contain actual complete board actions; administrative events stay separate.
Chu notation is explicitly technical until MSH-43.

Native decimal strings preserve declared root and selected/live total counters
above JavaScript's safe-integer range. Native provenance flags distinguish unknown
prior history from proven reachability. Technical Moves number known-root pairs
only (including a first-known Gote move), while the caption retains the exact
declared counter; no missing historical rows are invented.

Native metadata separates selected/live identities, King/Prince survivors,
individual local capture influence, `has_capture_all` advisory/history query,
Unknown diagnostics, Lion protection, occurrences, obligations, offers,
conclusions, incomplete state and ordered effects. Individual threats do not
imply all royals can be captured or a Chess-style check/mate. Hypothetical
opponent queries are advisory geometry; actual-turn queries apply native history.
No frontend inference creates a game outcome.

Chu execution capabilities remain empty. The actual bundled internal entry point
returns unscored `UnsupportedMode` with the complete submitted context. Verification
can exercise supervised requests; wrong history/purpose/limits/request/session/
process/selection and retired responses reject. There are no fabricated scores,
variations or fallback actions. Failures, native Stop and chess outcomes remain
distinct. Ordinary New Game/Load still expose only the four playable games.

The separately compiled `desktop-verification` selectors `chu-contract` and
`chu-standard` use the existing stable QA identity and shared page. The surface is
labelled **CHU DEVELOPMENT CONTRACT** and is semantic verification only: technical
letter codes, layout and piece rendering are not MSH-43's design or visual approval.
The QA file route can reopen Chu; normal product admission cannot. Fixture roots,
constructors and that host activation are absent from normal builds. No new
application identity, workflow owner or rules engine was added.

## Evidence and MSH-43 boundary

[shared-verification.json](shared-verification.json) is the single structured
source/check/review/CI record. Native-generated JSON comes from
`write_chu_frontend_fixtures`; CI regenerates it. Consumer test cases supplement,
not recount, MSH-86's1,076 native checks across240 rows.

| MSH-82 consumer row | MSH-78 evidence / remaining owner |
|---|---|
|SAME_BOARD_DIFFERENT_HISTORY|Native routing+controller repetition/record identity tests; exact histories preserved. |
|STALE_REQUEST|Actual child/controller negative identity probes, Cancel, replacement, Take back and stopped reopen. |
|WRONG_PURPOSE|Actual execution context rejection; analysis never reaches a move handler. |
|WRONG_PROCESS_GENERATION|Actual controller negative probe and fresh restoration identities. |
|HISTORICAL_PREFIX_WITHOUT_FUTURE|Exact native selected context, immutable inspection, royal metadata and explicit Return live. |
|SAVE_REOPEN_STOPPED|Actual atomic worker/controller with historical observation, branch and selected-prefix restoration. |
|RESERVED_CLOSE_HEADROOM|Actual byte/work/count accounting and all-actor closing boundary tests. |
|COMPLETE_ROUTE_PARTIAL_SELECTION|Native-generated routes through existing pointer/keyboard controller; prefix/Cancel/Escape never commit. |
|NO_NORMAL_UNSUPPORTED_CHOOSER|Native New/Load gates, host QA-vs-product test and four-option frontend chooser regression. |
|FULL_BOARD_WITHOUT_SCROLLING|Native144-cell/two-orientation contract verified; finished traditional12×12 normal/minimum packaged window evidence remains MSH-43. |

MSH-43 owns actual functional evaluation/search, execution results and scheduling,
usable console/notation, traditional kanji product rendering in the accepted
shell, normal activation, analysis/recovery/Resume, complete games and packaged
GUI evidence at supported sizes/platforms. It must preserve this whole-document
admission policy and native history, and extend real result validation rather
than accepting arbitrary scores/actions. MSH-43, M4, shared46/47/88/89, later
architecture107, Tenjiku and M5 remain open.
