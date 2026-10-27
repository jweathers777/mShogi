# Independent packaged-GUI artifact audit

Separate fresh reviewer `/root/platform_gui_research`, not a refinement implementer
or the GUI-driver author. Method: independently parse full downloaded reports,
verify evidence-file digests and package manifests, unpack actual package payloads,
match host/engine binaries to every GUI phase, examine native process bindings and
shutdown timestamps, reproduce canonical-file equality, inspect native screenshots
and source-trace assertions. The author merely rereading output is not the review.

## Linux — d6d7681 / Desktop CI40472402431

Job122082390118 passed (completed08:26:54 UTC,2026-10-27). Actual14/14 GUI phases,
empty failures; recorded interaction08:25:11.505796→08:26:48.453974 UTC (96.948s).
All217 GUI evidence-file hashes and13 normal manifest file hashes verified. The
reviewer parsed BOTH actual Debian ar/tar payloads and matched every phase host
and engine hash: normal host `dc28a722…`, QA host `d4fb225f…`, common engine
`a2c33362…`; normal deb `57d449a1…`, QA deb `80d750d5…`. Full hashes are retained
in original reports/manifests. All14 phase hosts had unique PID-bound identity;
all14 pre-teardown observations recorded absent host and empty engine processes,
with timestamp-consistent14 native Quit requests and zero failure-cleanup exits.

Shared-shell observations match Chess/Shogi header/actions/live heading/board-panel/
sidebar left/top/width at equal viewport; document centerline is40.5px in both.
Actual760×650 and1180×820 observations show81 visible squares, player/hand regions
and loaded bundled font. Native images corroborate the DOM/geometry. Both native
sides, historical inspection and hollow read-only markers are exercised. Inspection
assertions compare stable pieces/history/status/dirty/title/runtime/analysis/token
or Chess version/events/paused state across polls, full read-only route and Escape.

Shogi inspection-before, inspection-after and source capture archive are BYTE
IDENTICAL (`9278472e…`), independently reproduced; native root provenance is
`reduced_material_diagram` / `imported_unknown` with2 events. Historical analysis
retains an exact1-event Gote/event1/ply1 prefix while live record keeps2 events.
Normal and QA packages remain separately identified; tauri-driver2.0.6 was verified
from the actual job log (report version string is empty).

No material finding. Limits: Ubuntu24.04 X11/WebKitGTK/software-compositing CI desktop,
not consumer hardware/display coverage or screen-reader certification. The reused
Chess b8 screenshot path contains the FINAL historical capture; the earlier live
assertion passed but its separate raw image was overwritten. Chess canonical-save
equality is a passed driver assertion with the final archive retained; the two
Shogi comparison archives permit independent byte-equality reproduction. No owner
approval inferred. The normal minimum and normal-size inspection are separate
captured states, not a combined minimum/inspection image.

## Windows — d6d7681 / Desktop CI40472402431

All 14 actual GUI phases passed, no failures, interaction 08:30:13.732118→
08:32:14.977574 UTC (121.245456s). The reviewer verified all 190 GUI file hashes,
both normal manifest entries and actual normal/QA NSIS installer bytes. Normal
host `c00b9228…` matches the installed normal package-smoke and all three normal
exact-PID/image bindings. QA host `51564d36…` is distinct and binds all eleven
separate verification phases to the separately installed path/PID. Common engine
`d19192f9…` matches the recorded hashes. All 14 native mouse-menu Quit events lead
to timestamp-consistent host/engine absence before driver teardown; no failure
cleanup was counted as normal exit. Actual WebView2 descendants and matching driver
are 131.0.2903.86. The pinned tauri-driver2.0.6 install is confirmed in the actual
job log; the empty report version string was not reinterpreted as independent proof.

The reviewer followed installer execution → installed normal package-smoke →
PID/image/hash bindings using actual job logs and reports, and hashed the original
NSIS installers. The Windows artifact contains installers rather than loose
installed executables: this audit did not install/extract NSIS on macOS, unlike
the direct Debian payload parsing above. No stronger provenance method is claimed.

Shared-shell coordinates match across games. Full canonical before/after Shogi
inspection saves are byte-identical to Linux (`9278472e…`). The Windows checkout
source hash `6a30bfd7…` differs solely by CRLF: converting repository LF bytes to
CRLF reproduces that hash exactly. Parsing yields identical reduced-material root,
supplied counter17/imported-unknown history and two events; native saved output
canonicalizes LF. Historical analysis retains the exact Gote/event1/ply1 one-event
prefix with two live events. No material product or assertion finding.

The Windows virtual desktop is 1024×768. Native desktop captures of 1180×820 windows
clip the offscreen right/bottom; paired WebDriver client captures contain the full
actual WebView viewport. The 760×650 native desktop capture is complete. Retained
paired images preserve this distinction: cropped desktop images are not labeled
as full-window proof. Static visual review, actual interaction, package construction
and native controller tests remain separate evidence. No physical-display,
screen-reader certification or owner approval is inferred.
