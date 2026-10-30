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

## Actual packaged Linux baseline

[Desktop run 41063294647](https://github.com/jweathers777/mShogi/actions/runs/41063294647),
Linux job `124088161908`: **passed** at source
`1d84232c425270a6517711dbaa598106fc08aa31`. Actual GUI interaction ran
2026-10-30 13:35:56–13:37:39 UTC. The normal Debian package was installed with apt
and its `/usr/bin` host/engine hashes checked against package inspection. The
separately labeled verification Debian was extracted and launched from that
extraction; no claim that the QA package was installed with apt.

The workflow's unchanged command was:

```sh
GDK_BACKEND=x11 WEBKIT_DISABLE_COMPOSITING_MODE=1 dbus-run-session -- \
  xvfb-run -a -s '-screen 0 1440x1100x24' \
  python3 desktop/scripts/gui-smoke.py \
  --normal-host /usr/bin/mshogi-desktop --normal-engine /usr/bin/mshogi \
  --verification-host "$PWD/gui-extracted/usr/bin/mshogi-desktop" \
  --verification-engine "$PWD/gui-extracted/usr/bin/mshogi" \
  --output "$PWD/gui-evidence"
```

All existing phases passed without skipping assertions:

| Phase | Actual scope | Result |
| --- | --- | --- |
| normal | Installed Chess play, inspection, normal/minimum geometry, active analysis cancel/replacement, Save/Open, exact history analysis, Return to live/Resume, unsaved replacement/window-close cancellation and native Quit | Pass |
| normal-shogi | Normal package, native game/side selection, full board/hand inspection and keyboard operation, actual search, save and native window-close protection | Pass |
| normal-shogi-reopened | Open saved Shogi in fresh process with work stopped, history-prefix analysis, Return to live/Resume, save/reopen, normal-package opponent-hand inspection and archive equality | Pass |
| verification-standard | e2e4 and actual engine reply; genuinely missing sidecar produces unscored failure at3plies; identical engine restored and Retry preserves history through4plies; native Quit | Pass |
| verification-promotion | Cancel promotion then complete native Knight underpromotion/outcome | Pass |
| verification-claim | Optional intended-move draw claim without playing intended move | Pass |
| verification-capacity | Observation256 acceptance/257 refusal and complete save | Pass |
| verification-synthetic | Labeled16×16 compound route selection/cancellation/native complete action | Pass |
| verification-shogi-hands | Native capture/drop, hand selection cancellation and engine recovery | Pass |
| verification-shogi-promotion | Optional complete promotion choice/cancellation | Pass |
| verification-shogi-forced | Compulsory native promotion | Pass |
| verification-shogi-declaration | Formal declaration and semantic Take back | Pass |
| verification-shogi-concluded | Concluded record, terminal-event history and Take back | Pass |
| verification-shogi-capacity | Combined supported Shogi capacity admission and save | Pass |

Every phase records actual native host absence and empty final matching-process
lists **before** driver teardown. All14 cleanup-isolation checks also pass, but
are not substituted for that exit evidence. Two final guard transport closures
were accepted only with subsequent native absence. There were no ordinary
observation retries and no pending pointer-precondition samples in this campaign:
its success is baseline evidence, not proof of the original race's timing.

The downloaded artifact is `12980863590`, `desktop-ubuntu-24.04`, 72,999,292 bytes,
reported digest `sha256:d46fb1d99f1ed98b1a5cc6a8712ceb12313ca389607042b46aa03d8f1fcb38e5`.
[Retained report, manifests, proxy cases and native screenshots](evidence/msh122/baseline/artifact-index.json)
are separate from the original failed artifact. Root independently checked229 GUI
file hashes,13 normal-manifest hashes and the QA Debian hash; exact audit in
[integrator-audit.json](evidence/msh122/baseline/integrator-audit.json).

- Current normal Debian: `eefc0d076a46c1ab8cc58ec123f3f8ba8649a698f01c34f6e86e7133efb135e1`.
- Current verification Debian: `dc41e349f51b2c06638f5a8ed9962e2f9e99ad71ed08f95ad0ff46355a8a3642`.
- Normal/QA host and engine SHA-256 are **identical** to the original identities
  above. Debian archive metadata hashes differ; no product-binary change is claimed.
- Runner image, Linux/Python, GTK/WebKit, native tools, renderer environment and
  no-pool driver binary/provenance match the original environment recorded above.
- Normal-package inspection passed filename and host verification-marker checks.
  Root also scanned the actual normal host for all16 prohibited QA markers:
  absent. Test sources, external verifier and QA constructors stay out of normal
  distributables. Six actual original/patched-proxy boundary experiments passed,
  including no mutation replay after ambiguous delivery and fatal semantic errors.

## Independent review, CI and acceptance disposition

Independent reviewer `/root/msh122_review` used a separate agent session to inspect
original full report/traceback/DOM/screenshots, bounded source/tests/docs, and the
actual downloaded passing Linux package evidence. This is agent source/evidence
review, not a human or separately executed GUI campaign. The reviewer independently
ran41 Python tests, checked229 GUI/13 normal-manifest hashes and QA Debian digest,
all14 native pre-teardown exit proofs, actual missing-engine/retry transcript and
native screenshot. **No material findings remain.** The reviewer accepts the
qualified evidence-grounded disposition: exact original absent-versus-disabled
state is irrecoverable, demonstrated readiness defect is corrected, full Linux
baseline is established. Source approval and final evidence approval are distinct.

Applicable CI at `1d84232`: **all three platforms passed** in both
[Desktop41063294647](https://github.com/jweathers777/mShogi/actions/runs/41063294647)
and [Rust41063294530](https://github.com/jweathers777/mShogi/actions/runs/41063294530).
Exact job/step metadata are retained in the baseline evidence directory. Desktop
includes native/frontend format, lint, tests, normal-package inspection and bundled
engine queries; Linux also ran41 Python GUI boundary tests, six actual proxy cases
and the14-phase GUI baseline above. Windows's actual installed-package GUI step
also passed in its own CI job; it is not inferred from Linux. This task's detailed
artifact audit covers Linux, not a new Windows/Xiangqi acceptance campaign. macOS
CI establishes native/frontend/package/engine checks, not new macOS GUI interaction.
Earlier startup311ea53 Rust41053672140 failed the unchanged runner test at
`crates/mshogi-runner/tests/runner.rs:172` (MissingResponse versus UnexpectedExit,
handshake Broken pipe). That remains failed historical MSH-112 evidence, not a
GUI finding or a defect fixed here.

The Linux campaign establishes Chess/Shogi/synthetic verification only. It makes
no Xiangqi GUI claim and does not infer any other operating system's behavior.
No unresolved demonstrated MSH-122 GUI baseline blocker remains. MSH-116 is the
next dependency-ready assignment; it remains unstarted. No architectural migration,
new dependency gate, MSH-111 reopening or MSH-112 implementation is included.

## Final acceptance handoff

| MSH-122 criterion | Evidence/disposition |
| --- | --- |
| AC1 | Original artifact/identities/runtime/log/DOM/screenshots and full originating traceback retained; failed status preserved. |
| AC2 | Controlled App ordering reproduces null target; narrow verifier precondition correction; independent reviewer accepts evidence-grounded disposition with original instantaneous branch explicitly unknown. |
| AC3 | Single pointer dispatch, fatal transport/semantic behavior and unchanged25s budget verified; actual native14-phase exit proofs and normal QA exclusion. |
| AC4 | Actual installed normal/extracted labeled QA Linux baseline14/14 with exact source/package/runtime/driver identities and full report. |
| AC5 | Focused41Python/9App/TS checks, independent source/evidence review and green three-platform Desktop/Rust CI;111/112 remain separate. |
| AC6 | Only122 changed; no R1–R6 implementation or dependency gate. Normal integration source1d84232 on codex/rewrite; evidence/finalization-only commit follows. |

`backlog doctor` and `git diff --check` pass. Existing M1–M4 rules/engine/product
behavior and milestone ownership are unchanged. No disposable worktree was created;
unrelated pre-existing worktrees were preserved. This evidence handoff closes only
MSH-122. MSH-116 may proceed without a demonstrated GUI-baseline blocker, under its
own separate authorization and task contract.
