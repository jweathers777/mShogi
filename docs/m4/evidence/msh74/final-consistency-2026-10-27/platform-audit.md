# Independent final-consistency platform artifact audit

This bounded review inspects actual packaged GUI artifacts, executable/package
bindings and external-driver observations. It does not change product code,
dispatch CI reruns or substitute build results for GUI behavior. Product behavior
at `abe66c5` is unchanged by the native fixture regeneration and external driver
expectation corrections in final source `1b10277436b94541235e15189a3af7fce867351c`.

## Final 1b10277 Linux: passed all 14 actual GUI phases

[Desktop run 40502381314](https://github.com/jweathers777/mShogi/actions/runs/40502381314),
[Linux job 122182996283](https://github.com/jweathers777/mShogi/actions/runs/40502381314/job/122182996283),
at exact source `1b10277436b94541235e15189a3af7fce867351c` passed **14/14
actual packaged GUI phases**. Runtime was `2026-10-27T12:23:49.838217Z`
to `2026-10-27T12:25:34.212356Z` (104.374139 seconds).

All **223** original GUI file hashes and **13** normal-manifest entries were
independently checked. Both actual Debian packages' ar/tar payloads were parsed
and their host/engine bytes hashed. Every native window PID, exact executable
path and phase executable hash matches its corresponding package identity.
The normal application is actually installed at `/usr/bin/mshogi-desktop` with
sibling `/usr/bin/mshogi`; eleven labeled verification phases use the separately
extracted verification package. Normal-host bytes exclude all verification
instrumentation markers checked by the unchanged package-smoke script.

| Final Linux identity | SHA-256 |
| --- | --- |
| Normal Debian package | `6c43a8b937b6f423d883067ff6244067a8be176a423383344b77af67c2467bd0` |
| Normal host | `b619f16746c9e034ecaa386ac96a62a2a2020668767e28c2800036323de8b50d` |
| Verification Debian package | `84f01b92484d160d16041c2b6fb3d1e705dd3c93e64e03ae50436fc47cb2805f` |
| Verification host | `e2a2fe391a7da22e6e0196530381b449276515f7bee6f0b9417426c911a60cf9` |
| Engine, both packages | `a2c33362d2777647f8801a9f79f7d3bea5966a3583fd68337d7cac4ad981fc00` |

Both native host payload hashes equal the supporting `abe66c5` build; package
container hashes differ. The three normal and eleven verification phases all
record native host absent and immediate/final exact host/engine process lists
empty **before driver teardown**. All 14 host PIDs are distinct. No cleanup kill
is substituted for native Quit proof. There are zero read-only element lookup
retry events in this successful run.

The four paired initial shell observations have document state `unsaved` and
visible UNSAVED labels. All **nine** settled native-save observations use the
exact accepted path, **Unmodified** state and four enabled file controls, with
no Saving/Opening status. Both ordinary Chess/Shogi final close-guard DOM records
show **UNSAVED CHANGES** after a protected modification. Raw persisted/reopened
and hand-inspection captures retain Unmodified. This distinguishes initial,
persisted and subsequently modified native documents without inventing work.

The actual saved Shogi `engine_turn` request is **depth 2 / 5,000 nodes**, not
merely a display caption. Both games' initial native-limit captions and common
position statuses pass exact semantic checks while accepting locale digit
grouping. Singular one-ply promotion completes and the full journey passes.

All **16 full inspection-state before/after objects are equal**, retaining
pieces, history, native token/version, document state, engine/analysis state,
selected event and pause invariants. Canonical Shogi before/after and Linux
source archive bytes match SHA-256
`9278472ea18c3e0e312707c6471c9ce0a8fab473d2058fa9a26458f7c1d97666`.
That archive retains explicit reduced-material/unknown-import provenance and
two events; it is not attributed to standard opening play. Saved historical
analysis uses exactly one event at Gote/event 1 while the live game retains two.

At 1180×820 and 760×650, paired Chess/Shogi measurements have equal computed
dark themes, matching ordered physical slots/cards within three pixels, all
64/81 squares visible, shared footer help at tops 756/594 and a visible common
position footnote. Bounded raw-image inspection confirms complete minimum native
boards, lower-player/footer regions and the persisted read-only hand-inspection
state. Independent paired visual acceptance is recorded separately.

Runtime: Linux 7.0.0-1017-azure, glibc 2.39, Python 3.12.3, runner image
20261026.354.1; X11/D-Bus/Xvfb/Openbox with
`WEBKIT_DISABLE_COMPOSITING_MODE=1`. This does not establish Wayland, other Linux
distributions, accessibility certification or hardware-accelerated performance.

The [retained index](linux/retained-files.json) identifies **73 exact original
files**, including the full report, package manifests/smoke result, canonical
archives, normal/minimum/inspection pairs, active-analysis/history and close
states. No installer or application executable is retained. Full artifact:
`/private/tmp/msh74-consistency-ci/1b10277436b94541235e15189a3af7fce867351c/linux/`.

## Final 1b10277 Windows: passed all 14 actual GUI phases

[Windows job 122182996723](https://github.com/jweathers777/mShogi/actions/runs/40502381314/job/122182996723)
at exact source `1b10277436b94541235e15189a3af7fce867351c` passed **14/14
actual installed/packaged GUI phases**, from `2026-10-27T12:28:58.214823Z`
to `2026-10-27T12:31:00.319728Z` (122.104905 seconds). Final-source Desktop
40502381314 and Rust 40502381395 both completed successfully on all three
platforms; package-only macOS CI does not supply an additional macOS GUI run.

All **196** original GUI file hashes, **two** normal-manifest entries and both
actual NSIS installer byte hashes were independently checked. Three normal
phases bind to the normal package-smoke installation and eleven labeled
verification phases to the separately installed verification application. Every
native image, engine path, host/engine hash and PID matches the corresponding
installed identity. All 14 native host PIDs are distinct.

| Final Windows identity | SHA-256 |
| --- | --- |
| Normal NSIS installer | `e2bde424e4f030a25280fecaf209b33612a2a3c7249389b8cbf77a1d5a00cc12` |
| Normal installed host | `b6e9d0476ca3c77e88a54f9d6b08634c915496301262f61b6743c1fdf1f4d7f2` |
| Verification NSIS installer | `5e2395c437241f70f444402e034251fc0edbf731eebf0a7bc48705195464ccb4` |
| Verification installed host | `f257a92ff01cb4df7c55bc79e2697b51e2520079278aecff44b01025debbe7b6` |
| Engine, both packages | `c0a7928ad664c39d41dd5ef55e6f1c7a051e81b02649f1289fec888f8f6cc69f` |

The normal Windows package-smoke step successfully checks instrumentation
exclusion against its installed host, before the separate verification build.
Those installed-byte checks are supplied by the actual Windows installation
log, package-smoke and PID/image/hash observations. Unlike Debian payloads,
NSIS payloads were not locally extracted or reinstalled on macOS; only the
actual installer bytes were directly rehashed here.

All **14** phases record host absent and immediate/final exact host/engine
process lists empty before driver teardown. All **16 full inspection-state
before/after objects are equal**. All **nine** strict native-save observations
record the accepted path, Unmodified state, four enabled controls and no active
Saving/Opening status. All four initial shell states are Unsaved, and both
ordinary close-guard DOM captures show Unsaved changes after a modification.
The saved actual Shogi engine-turn request again uses **depth 2 / 5,000 nodes**.
There are zero read-only WebDriver lookup transport retry events.

The native Shogi before/after saved archives are byte-identical LF files with
SHA-256 `9278472ea18c3e0e312707c6471c9ce0a8fab473d2058fa9a26458f7c1d97666`.
Windows' checked-out source has CRLF bytes and recorded SHA-256
`6a30bfd7022c136028579dd3bea55a8452e05ef2650e5c98700dcaebceebfc3d`.
That exact hash is reproduced by LF-to-CRLF conversion; canonical JSON content
matches, while raw source/save bytes are explicitly different. The archive
retains its reduced-material/unknown-import provenance. Historical analysis uses
the exact one-event Gote prefix while the live game retains two native events.

Paired initial Chess/Shogi measurements at both sizes have equal computed dark
themes, matching physical slots/cards, complete boards and shared footer help
tops 756/594, with the common position-status row visible. Bounded raw native
minimum-pair and full-webview history inspection confirms complete boards,
lower-player/help/status regions and the correct selected one-ply history label.
Independent visual acceptance is separate. Windows' 1024×768 virtual desktop
clips large native windows; full webview images preserve the complete 1180×820
client, and minimum native captures are complete. Runtime is Windows Server
2022, Python 3.12.10, runner image 20261022.344.1; matching WebView2/Edge driver
131.0.2903.86 and tauri-driver 2.0.6 are recorded by the job/runtime evidence.

The [retained Windows index](windows/retained-files.json) identifies **71 exact
original files**, including full report/manifests, canonical saves, paired
normal/minimum/inspection, analysis/history, persisted and modified states.
No application installer or executable is copied into the durable subset.
Full artifact:
`/private/tmp/msh74-consistency-ci/1b10277436b94541235e15189a3af7fce867351c/windows/`.

## Supporting abe66c5 Linux run: failed, 10/14 phases

Source `abe66c5c40f56bb98041ce0491b1004f5f9498b8`,
[Desktop run 40500443053](https://github.com/jweathers777/mShogi/actions/runs/40500443053),
[Linux job 122176497679](https://github.com/jweathers777/mShogi/actions/runs/40500443053/job/122176497679).
The original GUI report remains **failed: 10/14 phases passed**. Runtime:
`2026-10-27T12:09:33.188934Z` to `2026-10-27T12:11:18.970063Z`.
All **135** original GUI file hashes, **13** normal-manifest entries, both actual
Debian package hashes/payloads and every phase's host/engine hash binding were
independently verified. The normal executable bytes exclude all verification
instrumentation markers checked by `package-smoke.py`.

| Package identity | SHA-256 |
| --- | --- |
| Normal Debian package | `8e6995479700cabea1d1c734f5d76dd48b14d32b101fa4e03f8def9c440b11f8` |
| Normal host | `b619f16746c9e034ecaa386ac96a62a2a2020668767e28c2800036323de8b50d` |
| Verification Debian package | `ef71fbab74d235eb5d8db4b8f8661c9f4f5be111567f0988ac5912005bc2713e` |
| Verification host | `e2a2fe391a7da22e6e0196530381b449276515f7bee6f0b9417426c911a60cf9` |

The ordinary Chess and Shogi phases fail at the new initial play-limit text
assertion. The driver expects `20,000` / `5,000`, while this Linux runner's actual
locale formatting renders `20000` / `5000`. The measured native values remain
depth 2 and the correct respective node budgets. Initial document state is
`unsaved`; shared position status is `Your turnLive position · 0 plies`, and
the normal-size shared footer/help visibility checks pass before the assertion.
Neither normal journey reaches its full play/Save/Open/minimum-size sequence.

Fresh-process Shogi reopen then fails because the interrupted normal Shogi phase
did not create its saved archive. This is dependent missing evidence, not a
successful reopen or a newly established cross-game dispatch defect.

The promotion verification phase completes native `a7a8n`; its failure capture
shows the White knight on a8 and the proven-dead-position conclusion. Its driver
wait still expects `Live position · 1 plies`, while the new shared product footer
correctly renders `Live position · 1 ply`. This is another exact driver text
expectation mismatch; the phase still remains failed rather than being relabeled
from a partial observation.

The other ten verification phases pass and each records exact native host/engine
absence before driver teardown. The four failed phases do not supply successful
native Quit evidence. Failure containment is not substituted for it. No complete
new normal inspection/archive comparison or paired minimum-size result is
claimed from this interrupted run.

The separate [Rust run 40500442835](https://github.com/jweathers777/mShogi/actions/runs/40500442835)
fails its generated-fixture drift gate after tests and clippy pass. The native
generator emits `board_ply` and pretty-printed `play_limits` at their stable
serialized positions; four checked-in Shogi fixtures had appended those same
values at the end. The original Linux job log retains that exact diff. Fixture
regeneration and later driver corrections require their own exact source CI;
these failures remain preserved.

The compact [failed originals](abe66c5-linux-failed/retained-files.json) retain
22 unmodified files: full GUI report, four failure capture/DOM sets, relevant
driver logs, package manifests/smoke result and original Desktop/Rust Linux job
logs. No application package or executable is copied into this durable subset.
The full original artifact remains at
`/private/tmp/msh74-consistency-ci/abe66c5c40f56bb98041ce0491b1004f5f9498b8/linux/`.

## Overall audit scope

Final-source Windows and Linux artifact checks are complete above. Earlier
source-specific successful or failed journeys retain their own scope; none is
relabeled by the final success. Independent paired visual review and acceptance
are recorded separately from these executable/assertion/integrity checks.
Independent macOS actual visual evidence and the owner's approval are recorded
separately; this artifact reviewer performed no local GUI automation.
