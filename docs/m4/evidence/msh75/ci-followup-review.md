# MSH-75 independent Linux CI failure follow-up

Reviewer `msh75_archive_desktop_review`, 2026-10-27. Source
`4c4145eaacf7fcaf89a338773aa110a3700e4c75`; Desktop workflow
[40512188317](https://github.com/jweathers777/mShogi/actions/runs/40512188317),
[Linux job 122216678483](https://github.com/jweathers777/mShogi/actions/runs/40512188317/job/122216678483).
This is a bounded independent artifact/source review, not new GUI execution.
No product, driver, test or task records were changed by the reviewer.

**Current bounded verdict:** unchanged Linux attempt 2 completed all 14 actual
GUI phases, including the previously interrupted standard Chess recovery and
native Quit sequence. Independent artifact checks below confirm the pass.
The final settled metadata independently inspected below records the overall
workflow and all three platform jobs completed/success. The earlier stale Linux
job-summary observation remains historical evidence below.

**Initial attempt verdict, retained:** the failed attempt establishes a WebDriver transport incident on an
input request, with no concrete product defect demonstrated. Rerun the unchanged
Linux job and retain this failed **13/14** result. Do not call this attempt passed,
silently retry the ambiguous input action, weaken its checks or infer that the
unexecuted Chess recovery sequence passed. Successful completion of the fresh
required CI check remains the integrator's verification gate.

## Original observations

The downloaded original artifact is
`/private/tmp/msh75-linux-attempt1/`, with report and driver diagnostics in
`gui-evidence/`; raw job log is `/private/tmp/msh75-linux-ci-job.log`.
GitHub artifact ID `12737629351` is `desktop-ubuntu-24.04`, uploaded ZIP size
66,427,194 bytes and job-reported ZIP SHA-256
`ee959e046885a7a3c6a6001c8fc344373724173f8f8fac41a3eb773aee520542`.
The reviewer independently rehashed all **220** files listed in the original
GUI report, with zero mismatches. The uploaded ZIP itself was not independently
rehashed by this reviewer.

The report identifies Linux kernel 6.17.0-1022-azure, glibc 2.39, Python 3.12.3,
runner image 20261023.334.3, X11 and `WEBKIT_DISABLE_COMPOSITING_MODE=1`.
It records three successful normal phases (`normal`, `normal-shogi`,
`normal-shogi-reopened`), ten successful verification phases, and one failed
Chess `verification-standard` phase. Every Shogi phase passed, including
hands/drop/recovery, optional/mandatory promotion, declaration, conclusion and
complete capacity Save. The unsuccessful phase has no successful native Quit
observation; its empty post-cleanup process list is explicitly failure containment.

## Exact failing boundary and diagnostic limits

The phase started at 13:35:38.527734 UTC and successfully established its
WebDriver session, exact native host PID 35995/window and frontend readiness.
At 13:35:39.308479 it recorded the launched application. The Python traceback
then identifies `gui-smoke.py:1000` → `Webview.click` at `:156` →
`Webview.request`/`_request_once` at `:104`/`:123`: the **first e2 pointer action**
in the standard Chess fixture failed on `POST /session/<id>/actions` with
`http.client.RemoteDisconnected: Remote end closed connection without response`.
The subsequent e4 action, two-ply wait, deliberate missing-engine test and Retry
sequence were never reached. This is not a failed native-rule, save, recovery,
capacity or visual-state assertion.

The corresponding original driver log ends with:

```text
Error serving connection: hyper::Error(User(Service), client error (SendRequest)

Caused by:
    0: connection error
    1: Connection reset by peer (os error 104))
```

The same WebDriver session then successfully returned the failure webview image,
multiple script/DOM reads, viewport/focus state and text before cleanup. The
complete failure capture has no `capture_error`. Its image and DOM show the
initial White-to-move game, native engine Idle and live 0 plies, with no completed
engine work or game outcome. The reviewer directly inspected that webview image.
There is no native panic/crash diagnostic, driver exit or product failure shown
at the failing boundary. Successful subsequent reads argue against a permanently
lost application/session. They do not prove whether every part of the ambiguous
pointer action was delivered.

The observed reset is in the external driver/proxy request path. The evidence
does **not** establish why the downstream connection reset, nor uniquely identify
tauri-driver versus WebKitWebDriver or an OS condition as its origin. Do not
upgrade the transport classification into a proven internal root cause. The
AT-SPI and EGL/DRI3 warnings are also present in the phase logs; DRI3 warnings
occur in all 14 logs, including successful phases, so this evidence does not
justify claiming they caused the isolated reset.

## Startup/teardown and source review

`gui-smoke.py:262` creates a separate driver per phase, with separate native
driver port, log and Linux process group. Its readiness loop at `:293` verifies
the driver is alive and waits for `/status`; only after session creation, native
attachment and actual frontend readiness does the phase dispatch input.
The initial connection-refused startup observation is followed by readiness
success. Thirteen phases, including this phase, needed two readiness probes;
this startup observation is not the failing action's explanation.

`gui-smoke.py:99` deliberately retries only read-only element lookup requests.
Pointer `/actions` requests get one attempt because a disconnected response can
leave delivery uncertain. This failure correctly propagates. The terminal
transport exception handling at `:55`/`:611` applies only after dispatching a
known native-close guard action and still requires host/engine absence; it
cannot convert this ordinary input failure into successful exit.

The phase exception handler at `:1157` preserves traceback/captures, then the
`finally` path runs cleanup before the next independent phase. Cleanup first
deletes the WebDriver session, terminates/waits for its process group, then
uses `gui_linux.py:398` failure-only host/window-manager containment and verifies
no bound host/engine survives. Successful phases separately prove native Quit
before teardown. The failed phase has only the latter failure-isolation record,
which must not be relabeled as normal shutdown proof. There is no evidence that
previous teardown overlapped this phase's input; prior normal-Shogi-reopened
native absence and cleanup precede its fresh launch.

`git diff --name-only 1b10277..4c4145e -- crates desktop .github Cargo.toml
Cargo.lock rust-toolchain.toml` still yields only `desktop/README.md`. Product,
driver, dependency and workflow source match the earlier complete successful
14-phase evidence. The failed verification host hash
`e2a2fe391a7da22e6e0196530381b449276515f7bee6f0b9417426c911a60cf9` and engine hash
`a2c33362d2777647f8801a9f79f7d3bea5966a3583fd68337d7cac4ad981fc00` equal the
earlier successful Linux report. Earlier success remains valid for its own
attempt/source; it does not erase this failure.

## Initial disposition, retained

One unchanged rerun of the failed Linux job is proportionate: the failure is
isolated transport loss before the intended behavior check, the same session
remained readable, all other phases completed, and production/driver bytes are
unchanged from a successful run. No source fix is justified by the available
evidence. If the same reset recurs, retain both attempts and add external-driver
request/process/socket diagnostics before proposing a bounded infrastructure
fix. Preserve one-shot input semantics and every product assertion. No concrete
MSH-41 correctness, persistence or Shogi product-completeness defect has been
identified by this incident; CI verification is still pending until the rerun's
actual outcome is recorded. This addendum makes no claim about the separately
running Windows job or final workflow conclusion.

Original downloaded file identities for durable retention:

| File under `gui-evidence/` | Bytes | SHA-256 |
| --- | ---: | --- |
| `gui-report.json` | 443001 | `8360994044334db7045b0e58c87d7b6858cc2b6bff203668a038e8b27f4c7c56` |
| `verification-standard-driver.log` | 512 | `e1a903733ef33cdca9530bc07fe5317a818cc0d54550d59b0213eb08dfd7dc9f` |
| `verification-standard-failure.dom.json` | 2738 | `a58e332a0b3ebcbf2c593508b65c6478635be8d61a12e5efa8dba5b97da7de7f` |
| `verification-standard-failure.webview.png` | 94172 | `0310cc795df588abe3fee594f5d30d43fadfef62347e8c40a28db05dc552c789` |
| `verification-standard-failure.desktop.png` | 107314 | `6a3e92f4017742c8d7331cbd93ce138f6354726552ed2984c2aa113ca7948628` |

## Unchanged rerun — actual GUI pass

The integrator reran the failed Linux job without product, fixture, driver,
workflow or assertion changes. Source remains
`4c4145eaacf7fcaf89a338773aa110a3700e4c75`, workflow 40512188317 attempt 2,
[Linux job 122223390154](https://github.com/jweathers777/mShogi/actions/runs/40512188317/job/122223390154).
This reviewer independently audited the exact successful artifact ID
**12739498661**. Duplicate artifact names from separate attempts exist, so its
identity must not be inferred from a name-only download.

The downloaded ZIP `/private/tmp/msh75-linux-attempt2-exact.zip` independently
hashes to `75cd0650063f3b695e537ed9c63f779a9a931ceeb65e99188a8d64a0d17c5d8e`
and has exactly **66,674,491 bytes**. In the corresponding extracted
`/private/tmp/msh75-linux-attempt2-exact/gui-evidence/`, all **223** report-indexed
file SHA-256 values match. The report identifies the exact source/workflow,
`status: passed`, no phase failures, and **14/14 actual GUI phases passed**,
three normal and eleven verification phases. Its GUI execution spans
14:00:32.945496 to 14:02:19.146725 UTC. Every phase's host and engine SHA-256
matches the same phase in the first attempt, including the previously failing
verification host and engine quoted above. There are 16 completely equal
inspection-state comparisons, 14 native host/engine absence observations before
driver teardown, and zero recorded read-only lookup transport incidents.

The previously interrupted `verification-standard` phase now executes fully:

- Real pointer e2e4 and actual bundled engine reply establish two plies.
- The existing test temporarily removes its extracted sidecar, plays d2d4,
  and records a genuine startup-failure alert at exactly three plies, with no
  fabricated move or game outcome. The failure DOM and image show that preserved
  position and a Retry engine turn control.
- Restoring the identical engine hash and clicking Retry produces the recorded
  actual `a5 → a4` response and four-ply position. The result DOM/image show
  native engine Idle, completed depth 2, 841 nodes and the preserved move history.
- At 14:01:38.838913 the phase requests native Ctrl+Q, dispatches the explicit
  Discard-and-continue guard, and at 14:01:39.014496 observes the host absent and
  immediate/final exact host/engine process lists empty **before driver teardown**.

The reviewer inspected the original full DOMs and both webview images directly.
The rerun standard driver log contains the same AT-SPI/DRI3 warnings but **no
connection-reset error**. The unchanged two-probe startup also succeeds. This
supports the initial classification as an isolated external transport incident;
it does not establish a deeper cause for the first attempt's reset. No product
fix, input retry or weakened assertion was needed. The first attempt remains
failed 13/14 with its original diagnostics and partial scope.

The four originals retained in
[ci-linux-attempt2](ci-linux-attempt2/manifest.json) were independently compared
byte-for-byte to the exact downloaded artifact and their manifest sizes/hashes
checked. The successful report is
[gui-report.json](ci-linux-attempt2/gui-report.json); diagnostic/GUI images are
[driver log](ci-linux-attempt2/verification-standard-driver.log),
[genuine engine failure](ci-linux-attempt2/verification-standard-engine-failure.webview.png)
and [successful recovery](ci-linux-attempt2/verification-standard-result.webview.png).

The reviewer read the integrator-fetched current metadata at
`/private/tmp/msh75-integrated-desktop-ci.json`: exact source `4c4145e`, workflow
status **completed**, conclusion **success**. Windows and Mac job summaries are
completed/success. In this snapshot the Linux job summary still reports
`in_progress` with blank conclusion and a zero completion timestamp, although
every applicable Linux step and `Complete job` records completed/success. This
is a disclosed backend metadata discrepancy; overall workflow success is read
from its own observed conclusion, not inferred from step success. The integrator
is retrieving a final settled snapshot and owns the closure record.

The Linux step metadata records native dependency installation from **13:40:41
to 13:52:28 UTC (11 minutes 47 seconds)**, followed by successful locked frontend
and native checks (13:52:38–13:56:23), normal package build
(13:56:23–13:59:29), Debian package/bundled-engine inspection, separate verification
package build, actual GUI interaction (14:00:29–14:02:19), artifact upload and
`Complete job` at 14:02:24. The dependency delay is setup time, separate from
actual GUI execution; it is not a product/test failure. Normal package inspection,
native/frontend tests, actual Linux GUI execution and Mac/Windows job metadata
are distinct evidence categories. This reviewer did not run another platform's
GUI during this follow-up.

A direct reviewer `gh api` job metadata read failed locally with a network
connection error; that read is not a CI failure. The successful metadata above
was inspected from the integrator's saved API response, not independently
fetched over the reviewer's network. The actual Linux GUI verification gap is
discharged by the successful unchanged rerun; no concrete MSH-41 product blocker
remains from this incident. Initial failure evidence and the uncertainty about
its underlying transport-reset cause remain preserved.

## Final metadata settlement

The reviewer subsequently read the durable final
[integrated-desktop-ci.json](integrated-desktop-ci.json), not just the earlier
temporary snapshot. It explicitly records source
`4c4145eaacf7fcaf89a338773aa110a3700e4c75`, workflow 40512188317 **attempt 2**,
status **completed**, conclusion **success**, and exactly three platform jobs,
all **completed/success**: Linux 122223390154 (completed 14:02:27 UTC), Windows
122223393913 and macOS 122223467551. The stale Linux summary has settled; the
earlier observation above is retained as historical, not current status. Combined
with the independently audited 14/14 actual Linux GUI report, this resolves the
incident's final CI verification gate without source or assertion changes.
