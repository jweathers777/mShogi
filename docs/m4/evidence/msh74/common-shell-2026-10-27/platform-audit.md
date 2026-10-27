# Independent platform artifact audit — common dark shell

Current audited product revision: `19beb7437f11003734d65ee68f69a5f34bbdd02c`.
Earlier `a2524c10d2067f9f5ac3294c1a78efcddd771e99` and
`44aeb47022d57fff55142799bb6ba55980f4dc33` evidence remains separately labeled.
This is an independent audit of actual CI outputs, not an author review of the
current shell changes. No product or CI source was changed during the audit.
Current-source results appear first. Later historical sections preserve their
original provisional findings and limits; the current Linux attempt-2 and
Windows sections supersede their then-pending assessments.

**Intermediate evidence, not final visual acceptance.** The a252 Desktop run
passed all three CI jobs, but a subsequent actual-screenshot review found Chess
help above the lower player while Shogi help occupied the shared footer. The
then-passing slot assertions did not inspect the live help's ownership/location.
Product revision `44aeb47022d57fff55142799bb6ba55980f4dc33` corrects that layout and
adds actual visible footer-help/footnote and cross-game help-top assertions.
Intermediate run [40489563132](https://github.com/jweathers777/mShogi/actions/runs/40489563132)
has failed Linux and Windows attempt-1 journeys described below. The
original a252 files/results below are preserved unchanged.

## Current source 19beb74: Linux attempt 2 passed all 14 actual GUI phases

The root-dispatched, same-source Linux-only rerun
[attempt 2](https://github.com/jweathers777/mShogi/actions/runs/40493284756/attempts/2),
[job 122159602039](https://github.com/jweathers777/mShogi/actions/runs/40493284756/job/122159602039),
passed **14/14 actual GUI phases**. Source remains exactly
`19beb7437f11003734d65ee68f69a5f34bbdd02c`. Runtime was
`2026-10-27T11:29:48.439897Z` to `2026-10-27T11:31:30.361093Z`
(101.921196 seconds). Windows and macOS successful jobs were carried forward;
this audit independently inspected the Linux rerun's newly downloaded artifact.

All **223** GUI file hashes, **13** normal-manifest entries and both actual
Debian ar/tar executable payloads were independently checked. The normal package
was actually installed at `/usr/bin/mshogi-desktop` with sibling
`/usr/bin/mshogi`; the labeled verification package was separately extracted.
Every phase's exact native image, window PID, host/engine paths and executable
hashes match the corresponding payload. All 14 host PIDs are distinct.

| Successful Linux attempt-2 identity | SHA-256 |
| --- | --- |
| Normal Debian package | `af18d94c0f5f8abe18915ee2ade7eacb14cc508ddd619c9ba2c5f67945f94a2d` |
| Normal host | `2485a89495387242e10bd7cc4919165f9c3c1197a1522dbe4dbcf4ce31f9273b` |
| Verification Debian package | `95e51eb071acf7014f20723c2bc10908c1837cda5b8464ff40e0d66161f93733` |
| Verification host | `3b3a7cdc78affc5d789b26982737d3904b6ce161b9ec1532a05ef6e51ab10d5b` |
| Engine, both packages | `a2c33362d2777647f8801a9f79f7d3bea5966a3583fd68337d7cac4ad981fc00` |

Host/engine payload hashes are identical to attempt 1 despite different package
container hashes. All **14** phases record native host absent and exact
host/engine process lists empty before driver teardown, including claim
verification. No failure cleanup is used as successful native Quit evidence.
All **16 full before/after inspection-state objects are equal**; all **nine**
strict saved-document records have the accepted native path, Saved state and
four enabled file controls. The Shogi before/after/source canonical bytes match
SHA-256 `9278472ea18c3e0e312707c6471c9ce0a8fab473d2058fa9a26458f7c1d97666`.
Historical analysis uses one native event at Gote/event 1; the live game retains
two events. There are **zero** read-only WebDriver lookup retry events.

The paired normal/minimum Chess/Shogi observations again have equal computed
dark themes, matching physical slots/cards within three pixels, all 64/81
squares visible, and visible shared footer help/footnote at tops 756/594.
Bounded raw-image inspection confirms complete native normal/minimum boards,
the shared footer and right-aligned historical Return to live. Runtime is Linux
6.17.0-1022-azure, glibc 2.39, Python 3.12.3, runner image 20261023.334.3,
X11/Xvfb/Openbox with `WEBKIT_DISABLE_COMPOSITING_MODE=1`. The job installs
tauri-driver 2.0.6 and WebKitGTK driver 2.52.6-0ubuntu0.24.04.1.

Selected **70 exact originals**, full report/manifests, canonical archives and
paired captures are retained under [linux-final/](linux-final/) with
[retained-files.json](linux-final/retained-files.json). Full original artifact:
`/private/tmp/msh74-shell2-ci/19beb7437f11003734d65ee68f69a5f34bbdd02c/attempt2/linux/`.
This successful bounded rerun does not identify or resolve the earlier backend
transport reset's cause. Its original failed report remains preserved below.

## Current source 19beb74: Linux attempt 1 normal journeys passed, overall failed

Product revision `19beb7437f11003734d65ee68f69a5f34bbdd02c`,
[Desktop run 40493284756](https://github.com/jweathers777/mShogi/actions/runs/40493284756),
[Linux job 122152287793](https://github.com/jweathers777/mShogi/actions/runs/40493284756/job/122152287793):
**13/14 GUI phases passed; overall failed**. Ordinary Chess, ordinary Shogi and
fresh-process Shogi reopen all passed. The failed phase is labeled Chess claim
verification, on a read-only DOM-text observation before the claim assertion.
Runtime was `2026-10-27T11:12:27.279463Z` to
`2026-10-27T11:14:08.261389Z` (100.981926 seconds).

All **223** original GUI file hashes, **13** normal manifest entries, both
actual Debian ar/tar payload identities and every launched executable binding
were verified. Normal package SHA-256 is
`09d076ee93f5feb98bba83b0be22cb137711b0c783e66823c49492a42ed5ce22`;
normal host `2485a89495387242e10bd7cc4919165f9c3c1197a1522dbe4dbcf4ce31f9273b`.
Verification package SHA-256 is
`a2d07684b18f494a917e614a0bc05353a25739064f27b2665cfeb3f030dc417e`;
verification host
`3b3a7cdc78affc5d789b26982737d3904b6ce161b9ec1532a05ef6e51ab10d5b`.
Both contain the Linux engine hash in the intermediate a252 table. The 13 passed
phases each recorded host/engine absence before driver teardown; failure cleanup
is not used to establish a successful claim-phase Quit.

The strengthened saved-document acceptance observation is present **nine** times. Each requires
the exact accepted native document path, Saved status, all four enabled header
file controls, and no Saving/Opening status. The report now contains **16 full
before/after inspection-state comparisons**; all objects are exactly equal.
These include complete-route and Escape observations for board and opponent-hand
inspection, retaining the original full context invariant. The Shogi before and
after archives are byte-identical to the explicit reduced-material source:
`9278472ea18c3e0e312707c6471c9ce0a8fab473d2058fa9a26458f7c1d97666`.
Saved historical analysis again uses one native event at Gote/event 1 while the
live game retains two events. This establishes the current run's successful
comparison; it does **not** identify the unlogged changed field or prove the
cause of the older 44aeb47 Windows assertion failure.

The external helper permits at most one logged retry only for exact read-only
session `POST /element` or `/elements` lookups. Clicks, value changes, keys,
scripts, IPC and application mutations are not retried by that helper. There
were **zero lookup-retry events in this Linux run**. The failed claim phase's
`POST /execute/sync` body-text read was correctly outside that retry scope and
propagated `RemoteDisconnected`; its driver log records backend connection reset
by peer (104). The failure capture still shows the live claim-opportunity state.
No claim conclusion, whole-gate pass or transport root-cause resolution is
inferred.

Selected exact originals, full invariant records, canonical archives and the
claim failure capture/log are retained under [linux-19-attempt1/](linux-19-attempt1/) with
[retained-files.json](linux-19-attempt1/retained-files.json). These files preserve failed attempt 1 at the current product source. Full artifact:
`/private/tmp/msh74-shell2-ci/19beb7437f11003734d65ee68f69a5f34bbdd02c/linux/`.
The older a252 and 44aeb47 attempts remain separate. Root dispatched a Linux-only
same-source retry after this run completed; its outcome requires separate audit.

## Current source 19beb74: Windows all 14 actual GUI phases passed

[Windows job 122152287496](https://github.com/jweathers777/mShogi/actions/runs/40493284756/job/122152287496)
at `19beb7437f11003734d65ee68f69a5f34bbdd02c` passed **14/14 actual GUI
phases**, from `2026-10-27T11:16:45.455923Z` to
`2026-10-27T11:18:52.175232Z` (126.719309 seconds). All **196** GUI file
hashes and **two** normal-manifest entries were independently checked against
the downloaded original bytes. Both actual NSIS installer hashes match:

| Current Windows identity | SHA-256 |
| --- | --- |
| Normal NSIS installer | `cf8fc726b23a304c842ad31a9fa6c9859586a0aa077a1c762d661d1d9bcff4e4` |
| Normal installed host | `ade79f685893114dd9e48302563f96ff73f731babd5421c9eaa228569f3112be` |
| Verification NSIS installer | `a6f0bf3b4747a2ca72f3838f60dad279296bb626e115b674fba9eb7bd1e6dd62` |
| Verification installed host | `f08c5f5ea71b48e84a5e1d7089f9c5244d2563d0b6d550d99710eced83b74635` |
| Bundled engine, both packages | `1d721022f08c31236151fff271b97fe525d0092ebad897c588cdb01a3eff63e6` |

The three normal phases bind to the normal package-smoke installation and the
eleven explicitly labeled verification phases to the separate verification
installation. Every launched native image/path, engine path and executable hash
matches its expected identity; all 14 host PIDs are distinct. Each phase records
host absent and both immediate/final exact host/engine process lists empty
**before driver teardown**. Failure cleanup is not substituted for native Quit.
NSIS installers were hash-checked locally; their installed payload identities
are supplied by the Windows install log, package-smoke and native bindings,
not by a macOS NSIS extraction or reinstall.

All **16 full inspection-state before/after objects are exactly equal**,
including complete-route and Escape handling for opponent-board and hand
inspection. All **nine** saved-document acceptance records contain the accepted
native path, Saved state, four enabled file controls and no Saving/Opening state.
The Shogi before/after canonical saves are byte-identical with SHA-256
`9278472ea18c3e0e312707c6471c9ce0a8fab473d2058fa9a26458f7c1d97666`.
The Windows source fixture has CRLF checkout bytes (reported source SHA-256
`6a30bfd7022c136028579dd3bea55a8452e05ef2650e5c98700dcaebceebfc3d`), while the native saves use LF. Its canonical JSON content
matches both saves; raw Windows source bytes are not claimed identical.
Saved historical analysis contains the exact one-event Gote/event-1 prefix,
while the live game retains two native events. No read-only WebDriver lookup
transport retry occurred; the engine-recovery Retry is a separate product action.
This successful run does not reveal the unlogged changed field in the older
44aeb47 attempt or prove that attempt's cause.

At both 1180×820 and 760×650, paired Chess/Shogi records have equal sampled
computed dark themes, matching ordered physical slots/cards within three
pixels, every square visible, and visible help/footnote in the footer at tops
756/594 respectively. Bounded visual inspection of the raw full-webview active
analysis and historical images confirms inline Replace/Cancel and a right-aligned
Return to live in the shared notice. Native Windows desktop captures remain
limited by its 1024×768 desktop: the normal client can extend beyond the desktop;
the full webview images and minimum-size native images retain their actual scope.
Runtime is Windows Server 2022, Python 3.12.10, runner image 20261022.344.1;
the job installs tauri-driver 2.0.6 and records matching Edge driver/WebView2
131.0.2903.86. The report's empty driver-version string is not treated as a
version observation by itself.

Selected **68 exact original files**, report, manifests, canonical archives and
paired captures are in [windows-final/](windows-final/), indexed by
[retained-files.json](windows-final/retained-files.json). Full artifact:
`/private/tmp/msh74-shell2-ci/19beb7437f11003734d65ee68f69a5f34bbdd02c/windows/`.
This independent audit performed no local macOS GUI work. Root's separate macOS
evidence and owner acceptance are outside this Windows artifact conclusion.

## Corrected footer source 44aeb47: Linux attempt 1 partial evidence, failed

Source `44aeb47022d57fff55142799bb6ba55980f4dc33`,
[Linux job 122139776773](https://github.com/jweathers777/mShogi/actions/runs/40489563132/job/122139776773):
**12/14 GUI phases passed; the job and original report remain failed**.
Runtime was `2026-10-27T10:43:32.008442Z` to
`2026-10-27T10:45:15.734563Z` (103.726121 seconds). All **182** original evidence
hashes, **13** normal manifest entries, both actual Debian payload identities and
all launched phase executable hashes were independently verified. Exact selected
originals, including failure DOM/driver logs, are retained in
[linux-final-attempt1/](linux-final-attempt1/), indexed by
[retained-files.json](linux-final-attempt1/retained-files.json). The directory
explicitly preserves failed attempt 1 at this intermediate product revision; it is not a
passing overall outcome. Full download:
`/private/tmp/msh74-shell2-ci/44aeb47022d57fff55142799bb6ba55980f4dc33/linux/`.

| Corrected Linux identity | SHA-256 |
| --- | --- |
| Normal Debian package | `f1af03e5f33ae3897364c12c9bc89b5cd034eb94686762c35a9d382f7e3f4182` |
| Normal host | `9d92500b61c3e9cd1a6b49649559e66df2c7cb20e568369a53a9c0741ec423e2` |
| Verification Debian package | `f533da0187e82b653f9865d58f37760a46b978339f0337c08abe89425451d82e` |
| Verification host | `0be60579c173adec997fed0a7971c65be0bfb3f5495dafdf6a8ab98c88951878` |

Both packages use the engine hash recorded in the a252 table below. The normal
Chess journey and all eleven labeled verification journeys passed, each with
native host/engine absence before teardown. The ordinary Shogi journey passed
its Sente/Gote initial states, both sizes, common-theme/slot/card checks and
read-only opponent inspection, then failed during a read-only WebDriver
`/element` lookup for the third New Game chooser's Start button. The native host
remained live; the failure capture shows the Gote game and open New Game chooser.
The external driver reports a backend connection reset by peer (104). This
evidence does not establish that reset's root cause or resolution.

Because the interrupted journey never created `normal-shogi-complete.mshogi`,
the dependent fresh-process reopen phase then failed with “Open metadata failed:
No such file or directory”. That is not a successful reopen or evidence of a new
cross-game dispatch defect. The two failed phases' post-teardown isolation is
failure containment, not native Quit proof. Normal Shogi complete Save/Open,
historical analysis and hand-inspection archive equality were **not completed in
this run**; their earlier source-specific evidence remains separate.

The corrected live-footer behavior itself is supported: both games record help
top **756** at 1180×820 and **594** at 760×650, with positive help geometry,
`helpVisible=true` and `footnoteVisible=true`; computed common themes remain
equal and all squares remain visible. Raw paired native screenshots visibly put
the help below the lower player in both games, including minimum size. Thus the
earlier footer discrepancy is corrected in these captures; it does not turn the
interrupted full journey into a pass. macOS package CI passed; no new macOS GUI or
owner approval is inferred. A bounded same-source Linux retry, if dispatched,
must retain this failed attempt and supply separate actual results.

## Corrected footer source 44aeb47: Windows attempt 1, failed

[Windows job 122139776276](https://github.com/jweathers777/mShogi/actions/runs/40489563132/job/122139776276)
at the same full `44aeb47022d57fff55142799bb6ba55980f4dc33` revision completed
**13/14 GUI phases; overall failed**. Runtime was
`2026-10-27T10:47:05.947751Z` to `2026-10-27T10:49:18.198488Z`
(132.250737 seconds). All **195** GUI file hashes, both normal-manifest entries,
the normal installed executable/package-smoke binding and the separately
installed verification identity were checked. Actual NSIS installer bytes match
their manifest hashes: normal
`2b073478d86e62c0c1e05c099afad946549d9ec65d9db67870bf1b144627c1f0`,
verification
`9fe50ff492de81a3a9caa4865cd2e0a72946cfb80d6c4078a514c79b84866f22`.
Normal host `0312897cb678c0f99d6baaa79cf474262b9689aea5ae989593ad05204ed9857c`
and verification host
`49c9e26b6d973acde688e59e4ee8548865c4d1df3a9340389fd300654ff863ad`
are bound to their distinct actual installed paths/PIDs. Both use engine
`3a15ace5f3fc841b0bf083e486d0165bdd2ef43470abcb3d0c33a4bee7c4401c`.
Unlike the Linux Debian audit, NSIS payloads were not locally extracted or
reinstalled on macOS; the Windows installation log, package-smoke and native
image/hash bindings supply that installation evidence.

Ordinary Chess, ordinary Shogi and all eleven verification phases passed,
including 13 pre-teardown native host/engine absence checks. The fresh-process
Shogi reopen/history phase also reached and passed exact historical-prefix
analysis (one analysis event, two live events), Return live/Resume, and opening
the explicit captured-resource diagram. It then failed the full
`inspection_state()==before` assertion after keyboard opponent-hand inspection.
This is not a transport failure and is **not treated as a pass or presumed
flakiness**. The original failed phase and assertion traceback are retained.

The hand-inspection and failure DOM texts differ only in the expected local
inspection announcement. However, the compared `before` object and individual
field differences were not logged, and the hidden native token is absent from
these DOM captures. Therefore these files do not determine which compared field
changed or establish a product mutation versus another cause. A concrete
synchronization concern remains unproven: Save As waits for already-possible
“Saved” text and file existence before sampling the invariant baseline; it does
not explicitly await the new accepted document path/token. Further diagnosis
must retain the complete invariant and capture its before/after fields rather
than relax it. The before archive is valid with canonical SHA-256
`9278472ea18c3e0e312707c6471c9ce0a8fab473d2058fa9a26458f7c1d97666`;
no after-inspection archive was created, so final-source byte equality is **not
established by this run**. This failed phase has no successful native Quit proof.

Both sizes' shell/footer assertions passed: common help tops 756/594, visible
help and footnote, common computed themes and matching slots/cards. Raw desktop
minimum-size captures are complete. Windows' 1024×768 desktop crops larger
1180×820 native windows; paired full client screenshots preserve their complete
viewport and must not be mislabeled as complete OS-desktop captures.

Selected exact originals and their source paths/hashes are in
[windows-final-attempt1/retained-files.json](windows-final-attempt1/retained-files.json).
The full artifact remains at
`/private/tmp/msh74-shell2-ci/44aeb47022d57fff55142799bb6ba55980f4dc33/windows/`.
No package or application binary is copied into the durable subset. A later
rerun or correction must preserve this attempt's failed verdict and provenance.

## Linux: passed within the recorded environment

[Desktop run 40487524795](https://github.com/jweathers777/mShogi/actions/runs/40487524795),
[Linux job 122132950502](https://github.com/jweathers777/mShogi/actions/runs/40487524795/job/122132950502),
completed successfully. The original GUI report records **14/14 passed phases**,
no phase failures, and execution from `2026-10-27T10:28:02.416435Z` through
`2026-10-27T10:29:41.884569Z` (99.468134 seconds). Phase/event timestamps are ordered
and within their recorded execution intervals.

Environment: Ubuntu 24.04.5, Linux 6.17.0-1022-azure, Python 3.12.3, GitHub runner
image `20261023.334.3`; X11 under D-Bus/Xvfb and Openbox, with
`WEBKIT_DISABLE_COMPOSITING_MODE=1`. The report retains actual GTK/WebKit and
external-tool package versions. The job log confirms installation of
`tauri-driver 2.0.6`; its report `--version` output is empty and is not treated as
an independently reported version string.

The full downloaded artifact is at
`/private/tmp/msh74-shell2-ci/a2524c10d2067f9f5ac3294c1a78efcddd771e99/`.
[Artifact 12727285601](https://github.com/jweathers777/mShogi/actions/runs/40487524795/artifacts/12727285601)
is `desktop-ubuntu-24.04`, with GitHub archive digest
`3a689d829a25916673802dd73c883a5a09496ed73d0f76a678d0ca68e8ba41ff`.
Selected exact originals are retained in [linux/](linux/), with original artifact
paths, byte counts and SHA-256 values in [retained-files.json](linux/retained-files.json).
No screenshot was modified. Application packages/binaries are not copied into
this evidence directory.

### Method and package binding

The audit independently:

- Checked every **223 GUI artifact file hash** and all **13 normal package
  manifest entries**, plus the verification package hash and common full source
  revision in both manifests and the GUI report.
- Parsed both actual Debian packages' ar/tar payloads and hashed their host and
  engine bytes, then compared those values with every phase's executable hashes,
  package-smoke result, exact executable/PID binding and launch path.
- Traced the external harness assertions against the measured report values,
  parsed canonical archives, and inspected raw native desktop images plus paired
  webview/DOM captures. W3C pointer/keyboard actions and PID-bound native picker,
  resize, window-close and Quit actions supply input; JavaScript observes DOM.
- Checked native Quit events and host/engine absence **before driver teardown**
  for every phase. All 14 bound host PIDs were distinct. No native failure-cleanup
  event supplied successful shutdown evidence.

| Identity | Normal installed package | Labeled verification package |
| --- | --- | --- |
| Debian SHA-256 | `c9badad171d2b0804d4ea6d4a96ffc16b62a10ccaf6b1e6e7d721dd3b701d60f` | `c9aabd52e902d4dfc27035815909efe29f688a7565a96ecbb705d74bae0bb323` |
| Host SHA-256 | `c1e3c28ba2b18e0db6dae6fc17205bf4ed4ebc22d1b5d9111a3f652f22da8318` | `d046d04b56ca8d65ef2d4ac5ab5b7203389d9fbfba2be99b2bf457fbf074e834` |
| Engine SHA-256 | `a2c33362d2777647f8801a9f79f7d3bea5966a3583fd68337d7cac4ad981fc00` | `a2c33362d2777647f8801a9f79f7d3bea5966a3583fd68337d7cac4ad981fc00` |

The ordinary application was apt-installed at `/usr/bin/mshogi-desktop` with its
sibling engine `/usr/bin/mshogi`. Verification phases launched the separately
extracted, explicitly labeled package. Fixture behavior is not attributed to the
ordinary package.

### Actual shell, inspection and archive findings

The original report contains paired Chess/Shogi measurements at **1180×820** and
**760×650 client sizes**, measured after compensating for native decoration.
Both pairs identify `mshogi-dark`. The eight recorded computed style properties
are exactly equal across games for the seven sampled shell/header/card/title/
button roles. Header, document centerline, file actions, heading, board panel and
sidebar satisfy the recorded common-location comparisons. The ordered upper
player, board, lower player and footer slots, and Analysis → History → Actions
cards, match across games within the asserted three-pixel tolerance in every
recorded axis. Initial Play details selection and collapsed technical details
are present in both games. All **64 Chess / 81 Shogi squares** are within the
measured viewport at both sizes, with no page-width overflow. Native screenshots
show the shared dark surface and the distinct native boards/pieces.

Minimum-size active inspection is actually captured in
`normal-inspect-chess-e7` and `normal-shogi-inspect-shogi-7c` (paired DOM reports
760×650). Normal-size opponent/orientation/history/hand captures report 1180×820.
Hollow burgundy destinations, distinct origin outlines and explicit read-only
labels are visible. The harness completes a read-only route, waits across native
polls, and compares pieces, history, status, dirty/title/runtime/analysis state,
native token or Chess version, selected event and pause state; Escape clears
inspection. The new shell does not turn those observations into executable
opponent actions.

The retained Shogi inspection-before and inspection-after archives are
**byte-identical to each other and the recorded source archive**:
`9278472ea18c3e0e312707c6471c9ce0a8fab473d2058fa9a26458f7c1d97666`.
They explicitly retain a `reduced_material_diagram` root, supplied counter,
unknown imported history and two accepted native events. This is ordinary-package
Open/inspection evidence, not a claim that this diagram arose from standard play.
The saved historical-analysis archive has one native event in the Gote/event-1/
board-ply-1 request prefix while the live game retains two events. Chess canonical
equality after live inspection is a passed harness assertion; only its later
final archive remains, so a separate earlier Chess before/after byte comparison
cannot be reconstructed from this artifact.

Passed phases: ordinary Chess, ordinary Shogi, fresh-process Shogi reopen;
verification standard/recovery, Chess promotion, Chess claim, Chess capacity,
synthetic routes, Shogi hands/drop/recovery, optional promotion, forced promotion,
declaration, conclusion/history and Shogi capacity. Every phase records native
host absence and an empty exact-host/engine process list before teardown.

### Limits and other runs

The verified Linux artifact integrity and recorded assertions pass; the later
live-help placement finding above remains a material visual distinction in this
intermediate source, and is not overridden by its passing CI verdict.
This single X11/software-compositing run does not establish Wayland, other Linux
distributions, hardware-accelerated performance, screen-reader certification or
owner visual approval. Minimum-size sidebar content can scroll; full-board fit
does not claim that every sidebar control is simultaneously visible.

Capture names can be reused within a phase: the retained Chess `b8` inspection is
the later historical capture, and Shogi `7c` is the later minimum-size capture.
Earlier passing assertions remain in the report; overwritten raw images are not
claimed to exist.

The older `0c07e64` Desktop run **40475158549 remains failed**. This later success
neither rewrites that result nor establishes the cause/resolution of its separate
GTK/WebDriver connection reset. Revision
`61afcec45ccd59f78d3df66ed60586846ffecf42` changes the external Linux picker helper
and associated verification material, not this product shell; its newer run
**40488765308 is not audited here**. No success is inferred for it.

Windows at this product source is not yet covered by this audit. The macOS CI
package build passed, but no new local macOS GUI run is claimed while the Mac is
locked. MSH-74 remains In Review for the required owner checkpoint; this artifact
audit does not supply that approval or authorize MSH-75.
