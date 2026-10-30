# MSH-122 — Linux GUI startup target triage

## Scope and disposition

This is a verifier correction, not a product or architecture migration. Production
Rust, frontend behavior, driver/proxy, selectors, timeouts and native lifecycle
checks are unchanged. MSH-111's completed proxy diagnosis and MSH-112's separate
native runner race retain their ownership and historical results.

The original failure remains **failed**. Its exact instantaneous target state is
not recoverable: the original observer returned `null` both for an absent square
and for an `aria-disabled` square. The full traceback establishes that failure
preceded the standard fixture's first pointer command. The later capture shows
the correct initial Chess position and an idle, healthy interface. There is no
recorded transport exception at this boundary. A driver reset, bad fixture,
illegal move or persistent rendering failure is not established by this evidence.

A controlled frontend reproduction demonstrates a concrete verifier defect:
`ProductApp` can deliver its shared Chess snapshot before the separately mounted
`App` command hook becomes connected. The document card and all 64 squares then
exist, while `e2` is correctly disabled. The old document-card startup condition
passes, and the old immediate pointer observer returns exactly `null`. Connecting
the command hook enables the same square without any mutation. This establishes
a reachable startup-readiness race consistent with the original failure, **not
proof of which transient branch occurred in that particular run**.

The evidence-grounded disposition is to fix this demonstrated readiness defect
and preserve the original absent-versus-disabled uncertainty. New observations
separate those states rather than assigning an unsupported historical cause.

## Original evidence and provenance

[Run 40994147366](https://github.com/jweathers777/mShogi/actions/runs/40994147366),
source `2f17510e4c5b8c3b92a051adfa070b4d5d834b21`, Linux job `123863236532`.
The source of production `desktop/src`, `desktop/src-tauri` and verifier files is
unchanged between that revision and startup `311ea53` (empty scoped Git diff).

Original Linux artifact `12950842851` (`desktop-ubuntu-24.04`, 72,803,877 bytes),
GitHub digest `sha256:7b1be5e0ba05c047bc870dda6f7ff125064005ae2d61985841f787f474f3b3c0`.
Selected original bytes, the full report, full phase traceback, failure screenshots,
DOM capture, host/driver log, package manifests and proxy provenance are retained
under [evidence/msh122/original](evidence/msh122/original/artifact-index.json).
Large Debian packages remain in the referenced artifact.

- Normal Debian SHA-256: `e4936e0a9e4e4708e2175ca30e905a4fcf82f386b81ca657fec70a39b436e4fd`.
- Verification Debian: `cef997ec769056abece507ac8dd777ad92abc2cf9141eb17eed1d4f09822a57f`.
- Normal host: `4e76379f04b9766f7bafb2738a9bf81084081d140bced67659bdb2e317057464`.
- Verification host: `13c24189b015b17ae18e46a43002e92136e43d70d7e93d75dface4a9f5d28b0c`.
- Both bundled engines: `b111f972e8ce0ad74b3d3cdea203ec5f2c93ffbd2133a67e5c76d8f1c36ad474`.
- Ubuntu 24.04.5, runner image `20261026.354.1`, Linux `7.0.0-1017-azure`,
  Python 3.12.3, WebKitGTK/WebKitWebDriver `2.52.6-0ubuntu0.24.04.1`, GTK
  `3.24.41-4ubuntu1.3`, Xvfb `2:21.1.12-1ubuntu1.9`, Openbox 3.6.1, X11
  display 1440×1100, `WEBKIT_DISABLE_COMPOSITING_MODE=1`.
- tauri-driver 2.0.6 with the existing MSH-111 no-pool patch; archive, lock,
  Rust build and original/patched source/binary hashes in retained provenance.
  Patched driver `94e7345a51842e7a3f78bbd2194b1675cc02f86a9b7ae7ca986b0bf447b754ab`;
  WebKitWebDriver `488118541ba1dbeaca658073af84322900485b9435bc4fbb43eb1faf21db8b70`.

At 03:02:09.044690Z the native window identifies `standard` and the correct host
PID. The first `e2` call failed at `fixture` line1043 / `click` line160. The
aggregate workflow traceback abbreviates the failure, but the report retains the
originating traceback; it did not swallow an underlying transport exception.
The delayed failure DOM/screenshot show White to move, 0 plies, initial pieces,
Match started, Local engine/Idle, no failure/outcome card. The capture has no
per-square disabled attributes or failing-observation DOM; those facts cannot be
reconstructed. AT-SPI/EGL warnings in the driver log are not causal evidence.
All 13 other phases passed; failed-phase cleanup proves isolation only, not Quit.

## Correction and focused verification

`Webview.click` now observes the specific target before pointer dispatch: present,
enabled, visible, and actually hit-testable. It uses the existing `wait` budget
(25 seconds); no timeout, selector or success assertion is weakened. Diagnostics
retain the first pending observation, successful precondition, and final timeout
reason, including actual label, runtime, heading and square count. Absent and
disabled are distinct. Permanent absence, disabled state, occlusion or offscreen
state still fails with zero pointer input.

Only read-only observations are polled. Exhausted transport/semantic errors
remain fatal. `/actions` is outside the wait, dispatched once; uncertain delivery
is never retried. Native file/quit and pre-teardown host/engine absence assertions
are unchanged. The verifier and its diagnostics are external to the packages.

Local macOS checks (Node 24.19.0, Python 3.14.7):

- `python3 -m unittest discover -s desktop/scripts -p 'test_gui*.py'`: **41 pass**.
  Covers pending absent/disabled/offscreen/occluded → ready, permanent refusal,
  timeout diagnostics, fatal observation transport and no pointer replay; reuses
  existing proxy/client lifecycle and bounded observation regression coverage.
- `node node_modules/vitest/vitest.mjs run src/App.test.tsx`: **9 pass**, including
  the staged real App/native-snapshot startup reproduction.
- `node node_modules/typescript/bin/tsc --noEmit`: **pass**.
- `git diff --check`: **pass**.

During development, the first Python test run exposed four diagnostic-string
expectation errors (nested repr escaping); corrected by parsing the actual retained
observation. The first frontend runs exposed missing fields in the minimal test
prop; the final controlled test supplies the consumed token/capability fields.
These were test setup failures, not evidence of a product or driver defect.

## Packaged baseline and independent review

Pending actual current-source Linux package CI. No new platform or Xiangqi GUI
claim is made at this stage. Task completion requires the existing 14 phases,
normal instrumentation exclusion, exact identities and native termination proof.

Independent reviewer `/root/msh122_review` inspected the original artifacts and
bounded diff in a separate agent session. Provisional source review found no
material source defect; required the historical uncertainty above and actual
package/evidence reconciliation before final approval.
