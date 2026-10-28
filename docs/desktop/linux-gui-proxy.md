# MSH-111: Linux packaged GUI proxy diagnosis

The task isolates external verification transport from application behavior.
No product rules, native IPC, frontend, Tauri configuration, package contents or
journey assertions change. Windows retains the upstream driver. The external
Linux driver is built from checksum-pinned tauri-driver 2.0.6 with one change:
`.pool_max_idle_per_host(0)`. Its existing
`.retry_canceled_requests(false)` remains intact. The
[upstream pool setting](https://docs.rs/hyper-util/0.1.19/hyper_util/client/legacy/struct.Builder.html#method.pool_max_idle_per_host)
controls the number of retained idle connections. Build preparation refuses an
unexpected source or archive. Source, lockfile, compiler and binary identities
are retained in the CI `provenance.json`; the actual GUI report also records the
executed driver SHA-256.

Clicks, input, generic scripts, Save/Open, Close/Quit and session lifecycle
commands remain single-attempt. An ambiguous transport failure aborts the
phase; it cannot trigger an automatic action replay. The existing read-only
DOM/screenshot/element-lookup observation allowance remains one logged retry.
The outer state wait now propagates semantic errors and exhausted observations
instead of swallowing them and implicitly starting another observation budget.
False state observations still poll. Final native shutdown keeps its narrow
existing transport-close qualification and independently requires host and
engine absence before teardown; cleanup never counts as graceful exit.

## Retained failures and provenance

[Original evidence index](evidence/msh111/prior/index.json) records the original
artifact ZIP hashes, tested revisions, workflow URLs and byte-for-byte retained
file hashes. Each directory contains the original failed report, driver logs,
failed-state DOM/webview/desktop captures, normal package smoke and normal /
verification package manifests. Successful later captures do not erase the
original failures. Package binaries and all successful captures remain available
in the named original workflow artifacts; those large packages are not copied
into Git.

| Evidence | Source | Outcome |
| --- | --- | --- |
| baseline | `74a63915c1331439b95172fc77500b9d6d27458c` | New game click reset; subsequent missing-save Open failure |
| integrated attempt 1 | `2b50ac2f8c7fea6174be71a479cf532324347a3e` | Shogi screenshot reset; subsequent missing-save Open failure |
| integrated attempt 2 | same source | Chess claim DOM observation reset |
| read-only retry correction | `6f6ea696e37b0be44288086aed0368bcc7e01b04` | New game click reset; subsequent missing-save Open failure |

The captured Ubuntu 24.04 environment includes WebKitWebDriver /
libwebkit2gtk-4.1 `2.52.6-0ubuntu0.24.04.1`, GTK
`3.24.41-4ubuntu1.3`, Xvfb `2:21.1.12-1ubuntu1.9`, Openbox
`3.6.1-12build5`, xdotool `1:3.20160805.1-5build1` and scrot
`1.10-1build2`. Per-phase reports retain executable hashes, OS and renderer
settings, package and sidecar identities, exact X11 PID/window binding and
assertion failures. The retained [original Linux workflow log](evidence/msh111/prior/read-only-retry/workflow-linux.log) identifies installation of
locked tauri-driver 2.0.6, Hyper 1.8.1 and hyper-util 0.1.19. Its old `--version`
stdout was empty, and its binary hash was not recorded; neither is invented.

The driver logs show `hyper::Error(User(Service), client error (SendRequest))`
caused by downstream `Connection reset by peer (os error 104)`. The verifier's
Python `RemoteDisconnected` is the upstream symptom of that failed forwarding.
Same-session failure capture succeeds after the aborted request. This locates
the observed transport failure at the proxy-to-native-WebDriver boundary; it
is not a recorded application/native IPC error. A later missing Save follows
from an aborted earlier phase and does not establish an archive defect.

## Controlled transport experiment

`desktop/scripts/test_gui_proxy.py` runs both actual original and corrected
Linux proxy binaries against an independent controlled HTTP peer, with no app
or WebKit process. The peer records the fully received request and connection
identity before resetting it. Six experiments distinguish:

- A deterministic keep-alive stress peer resets the second request on a reused
  connection. The original proxy must fail; the corrected proxy must use fresh
  connections and succeed. This experiment explicitly uses HTTP keep-alive;
  the actual Python verifier sends `Connection: close`, so this is a mechanism
  reproduction, not a claim that WebKit's intermittent race has been proven.
- A peer resets after fully accepting one click. Both proxies must return a
  fatal failure with exactly one received click. This uses the actual verifier
  transport, with no client or proxy replay and no recovery event.
- A semantic rejection through the actual verifier inside its outer wait must
  be fatal after one observation for both proxies.

Disabling connection reuse removes the implicated pooling boundary rather than
recovering by replaying requests. A passing controlled experiment alone does
not establish a complete real application journey or the exact internal WebKit
cause. That distinction remains explicit even after a successful actual Linux
campaign.

## Verification record

Implementation `b8beae446b1ab303f91275c8b89603164275b7a0` was reviewed, then
review found that the synthetic pooling seed used urllib's forced
`Connection: close`. Revision `2611d09` fixes only that experiment to use explicit
keep-alive, records the request header/caller and asserts connection reuse.
The initial CI runs 40706231031 (Desktop) and 40706230906 (Rust) were superseded
and cancelled; they are not passing evidence.

Local macOS / Python 3.11.0:
`python3 -m unittest discover -s desktop/scripts -p 'test_gui*.py'`: 36 passed;
`python3 -m py_compile desktop/scripts/test_gui_proxy.py` and `git diff --check`
passed. Independent reviewer `msh111_review` separately ran the 36 tests on
macOS / Python 3.14.7 and reviewed the bounded diff, requirements and guardrail.
The harness finding and resolution are recorded above. Local results do not
establish Linux, Windows or actual packaged GUI success.

[Desktop CI 40706737407](https://github.com/jweathers777/mShogi/actions/runs/40706737407)
and [Rust CI 40706737330](https://github.com/jweathers777/mShogi/actions/runs/40706737330)
passed at exact tested revision `2611d0921dd3c637e76b7bb08bc0a23b49d84f33`.
The [successful evidence index](evidence/msh111/success/index.json) records
artifact IDs, GitHub-declared archive digests, retained file hashes and exact
source/workflow identities. Final CI metadata is retained beside the evidence.

| Verification | Actual result |
| --- | --- |
| Linux external-driver unit boundaries, Python 3.12.3 | 36 passed |
| Both real Linux proxy binaries against controlled peer | All six experiments passed; expected failures remain recorded |
| Linux Ubuntu 24.04 installed normal Debian / separate packaged verification app | All 14 current GUI phases passed |
| Windows Server 2022, Python 3.12.10, upstream tauri-driver | All 14 actual installed GUI phases passed |
| macOS 15 Desktop CI | Native/frontend checks and normal package smoke passed; no GUI campaign in this job |
| Rust workspace CI | Linux, Windows and macOS jobs all passed |

The [Linux report](evidence/msh111/success/linux/gui-evidence/gui-report.json)
records normal Chess play, analysis/cancellation/replacement, native Save/Open,
history, inspection and native close/Quit; normal Shogi creation/play,
analysis/cancellation, Save/fresh Open, historical analysis/Return to live/Resume;
Chess recovery/promotion/claim/capacity/synthetic phases; and Shogi
hands/drop/recovery, optional/forced promotion, declaration, concluded/history
and capacity phases. All existing assertions completed. All 223 GUI evidence
files were checksum-verified before selecting durable captures; the original
report retains the full file-hash map. Complete textual GUI records and saved
archives, selected native screenshots, package identities, job log and all six
proxy records remain in Git. Full package binaries and other captures remain
referenced by the named original artifact.

Independent review verified all 14 Linux phases, all native host/engine absence
events before teardown, cleanup isolation, all 223 GUI hashes, all 13 normal
package manifest files and the verification Debian hash. Phase host/engine
identities match normal package smoke. Executed corrected driver SHA-256
`94e7345a51842e7a3f78bbd2194b1675cc02f86a9b7ae7ca986b0bf447b754ab`
matches [build provenance](evidence/msh111/success/linux/gui-driver-provenance.json).
The controlled original driver SHA-256 is
`f2673b1e3f50b44cf8bd5a083b9eef56cf573c148a79eddf51d54b812338b1cc`.

No ordinary DOM/screenshot/lookup recovery retries occurred in the successful
Linux campaign. One promotion-phase connection-refused startup log corresponds
to bounded read-only driver readiness polling. Five final guard-click transport
closures were qualified separately, followed by actual host/engine absence;
none replayed the click or substituted cleanup for graceful exit.

The [Windows report](evidence/msh111/success/windows/gui-evidence/gui-report.json)
separately records all 14 passing phases and native shutdown events. All 196
Windows evidence-file hashes were verified using their original Windows path
semantics. Windows remains on the upstream driver, so this is compatibility
evidence for shared verifier fatality/provenance changes, not a Linux-derived
platform claim. The retained macOS workflow log records package smoke identity;
it establishes no new macOS GUI result.

The final mechanical workflow adjustment copies driver provenance to
`gui-driver-provenance.json` inside the workspace before boundary tests and
uploads that relative file. This preserves existing artifact root paths and
failure-time provenance without changing the tested driver, test scripts,
packages or journey execution. Independent review found no need to repeat the
unchanged behavior tests for that file-copy adjustment.

See the [independent review](evidence/msh111/review.md). MSH-111 records final
integration into `codex/rewrite` and relevant commit references. MSH-99's active
primary-checkout work is separate. This evidence establishes one complete
successful Linux campaign and bounded proxy behavior, not permanent elimination
of every possible transport failure or an exclusive historical root cause.
