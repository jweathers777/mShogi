# MSH-114 shared capacity explanation correction

Implementation source: `1c6b97e2bf7ecf560be1902e01b4e95038ee3482`,
`codex/rewrite`. Base: `e1bba329f01f9f4c23f628c4efd82a7f4dc9776f`.
This resolves MSH-103 F01's misleading resource diagnosis, not native capacity
admission or the complete Xiangqi review. MSH-103 stays In Review pending its
separate acceptance/finalization; MSH-45/M4 remain open and106 unstarted.

## Correction and acceptance mapping

The old shared recovery panel searched diagnostic text for256 before8192.
Both native adapters legitimately mention both bounds in one combined count
diagnostic. A control-only refusal therefore invented256 observations.

`NativeSessionPanels::NativeRecovery` now uses specific observation wording
only for the known unambiguous native retention-preflight reasons. Its count
comes from native snapshot ordinary observations plus Xiangqi exact observations.
All ambiguous/byte/event/replay/other reasons use:

> Application limit reached. Your current game can still be saved.

The existing direction to save the complete document/start a new game and the
statement that Save/Open does not reset structural capacity remain. No resource
is inferred from the appearance of a numeric bound. Raw native capacity and
retained diagnostic information remain in both games' Technical disclosure.
Independent file-operation errors remain visible alongside capacity. No rules,
archive schema, native product admission, limits or retention guarantee changed.

| Task criterion / case | Evidence and truthful behavior |
| --- | --- |
| AC1 Xiangqi control-only,0 observations | Real native8191-control+stop-reserve fixture→Flip candidate refused; generic saveability notice, no256 claim. QA and actual normal-package Open→Resume both verified. |
| AC1 Shogi control-only,0 observations | Corresponding actual journal→Flip refusal; same generic notice; Save As and Save succeed. |
| AC2 genuine observation exhaustion | Actual native255-observation constructor, genuine bundled256th result,257th request refused before acceptance. Notice accurately says256; all256 saved. Both games' native preflight variants have focused frontend coverage. |
| AC2 ordinary/exact accounting | Frontend255+1 and0+256 count DTOs and rerun native mixed-observation archive test; native remains authoritative. The GUI case has256 ordinary/0exact and is not claimed as a mixed-exact GUI case. |
| AC2 ambiguous/control/byte/native-event/replay |27 focused frontend cases use actual native reason text from both adapters; no numeric-substring resource inference, including a combined diagnostic when256 observations are present. Generic wording avoids inventing the cause. |
| AC3 persistence/failure/stop reservation | Real native controller regression covers both fixtures, refusal preserving canonical context/0observations, Save/Open, stopped reopen, persistent Resume refusal, Save As and unchanged original file. Existing256/257 native cases pass. Actual GUI Save/Save As/Open/close results below. |
| AC4 actual application and isolation | Normal and separately labeled macOS QA packages,9 PNG/AX captures, complete source/package/archive hashes in manifest; normal smoke excludes both new fixture markers and existing QA hooks. |
| AC5 independent review/integration | Fresh reviewer source/tests and pixel/AX/manifest assessment, no material findings; exact CI and integration handoff below. |

## Actual packaged macOS interaction

Integrator used Cua native accessibility, pointer/keyboard and file dialogs.
No IPC/JavaScript injection supplied game changes. Current normal source and QA
source are1c6b97e. Environment: macOS27.0.1(26A434),arm64,Rust1.99.1,
Node24.19.0,Tauri/WKWebView. Normal app ID `org.mshogi.desktop`; separately
named QA `mShogi Verification`, ID `org.mshogi.desktop.verification`, explicit
fixture title/content1180×820. This is macOS GUI evidence, not Windows/Linux
resource-limit GUI evidence.

| Executed journey | Actual result / capture |
| --- | --- |
| Xiangqi native control refusal | QA `xiangqi-control-capacity` journals8191 real preference controls,0board events,0ordinary/exact observations. Actual Flip refuses before commitment, pauses and shows generic notice. [Refused](xiangqi-control-refused-saveable.png). |
| Xiangqi Save/Open/close | Save As writes complete journal; Save succeeds. Open restores stopped, Resume refuses at unchanged capacity with truthful generic notice. Native Cmd-Q→Save & continue exits0. [Saved](xiangqi-control-saved.png), [reopened refusal](xiangqi-control-reopened-refusal.png). |
| Shogi native control refusal | Same native preference-journal boundary; Flip refused, board unchanged, generic notice. Save As and Save succeed, native Cmd-Q→Continue exits0. [Refused](shogi-control-refused-saveable.png), [saved](shogi-control-saved.png). Shogi GUI reopen is not newly claimed; the native regression executes it. |
| Genuine observation capacity | Existing QA `xiangqi-capacity` creates255 real incomplete native observations. Actual Start analysis uses the genuine bundled engine at depth1/one visit, retaining256. Next Start refuses; notice says256, no invented score or move. Save As retains all256 and notice survives. [257 refused](xiangqi-observation-257-refused.png), [256 saved](xiangqi-observation-256-saved.png). Native Cmd-Q→Continue exits0. |
| Normal distributable | Actual normal app Opens the same8192-control Xiangqi file, fresh/stopped. Resume refuses with generic notice, Save succeeds. [Normal product](normal-xiangqi-control-refused.png). No fixture environment or verification feature supplies this normal execution. |
| Preserve user's starting document | Before rebuilding, the existing fresh Chess0-ply document was saved to `/private/tmp/msh114-preserved-preverification.mshogi`. Final normal application reopens that exact document, clean/stopped0plies, explicit Resume. [Restored](normal-restored-preserved-document.png). |

The native-only helpers seed actual admitted no-op preferences and ordered
journals, then run normal capacity checks. They do not fabricate a refusal,
snapshot count or engine observation. Only `desktop-verification` compiles these
helpers/fixture names. The user-triggered Flip/Resume and file operations then
use normal admission/replay paths. Normal package smoke adds exclusion checks
for `shogi-control-capacity` and `xiangqi-control-capacity`. Optimized QA binary
strings need not retain these as contiguous markers; runtime fixture behavior,
feature gates and separate identity establish QA provenance.

GUI-created complete saves, measured after interaction:

| Document | Bytes | Controls | Ordinary / exact observations |
| --- | ---: | ---: | ---: |
| Xiangqi controls |3,974,990 |8192 |0 /0 |
| Shogi controls |1,443,736 |8191 |0 /0 |
| Xiangqi observations |1,049,452 |513 |256 /0 |

Xiangqi's final8192nd control is the actual close-pause reservation. Shogi's
saved paused journal ends with its8191st actual preference; no extra stop was
needed at Save. Neither history is truncated or its counts reset. Normal/QA
package files, screenshots and archive SHA256s/sizes are pinned in
[package-and-gui-manifest.json](package-and-gui-manifest.json).

## Executed local verification

At production-equivalent1c6b97e, unless qualified:

- `npm --prefix desktop test`: **235 passed/25files**, including new27-case
  `NativeCapacity.test.tsx`. The focused shared Shogi/Xiangqi suite passed58.
- `npm --prefix desktop run check`: passed, Node24.19.0 explicitly selected.
- `cargo test --locked -p mshogi-desktop-session --features desktop-verification verification_control_limits_refuse_truthfully_and_keep_complete_save_open`:
  **1 passed**, both real native fixtures,2.16s; independent rerun2.33s passed.
- Same command prefix, filter
  `ordinary_observation_256_to_257`: **1 passed**,48.31s;
  `ordinary_and_exact_observations_share_capacity`: **1 passed**,19.51s;
  `actual_256_observations_save_reopen_and_reject_257`: **1 passed**,10.52s.
- `cargo fmt --all -- --check` and
  `cargo clippy --locked -p mshogi-desktop-session --all-targets --all-features -- -D warnings`:
  passed. Existing byte/native-event/replay/atomic-file campaigns remain valid
  because native production admission/replay code is unchanged; only QA-gated
  helpers and its new controller test were added there.
- Normal `npm run desktop:build -- --bundles app` and QA
  `npm run tauri -- build --features desktop-verification --config src-tauri/tauri.verification.conf.json --bundles app`:
  passed. Existing local Tao vendor warnings remain; patch/provenance unchanged.
- `python3 desktop/scripts/package-smoke.py <normal-app> --report ...`:
  passed instrumentation exclusion and actual bundled-engine compatibility;
  [report](normal-package-smoke.json). This is distinct from actual GUI above.

Initial new native test compilation failed because its assertion accessed
an extra `.game` on `SharedSnapshot.context` (already a CanonicalRecord) rather
than comparing the canonical context directly. The test-only accessor was corrected before the passing executions
and source commit. No product contract or expected behavior was changed to pass. The final staged
whitespace check caught trailing spaces in the preserved CI-log excerpt; that
excerpt was whitespace-normalized before the final passing diff check.

## Independent review and remaining boundaries

Fresh read-only reviewer `/root/capacity114_review` independently inspected the
bounded diff, native diagnostic producers, frontend cases, QA gating/journals
and unchanged product admission. It ran27 frontend tests and the actual native
control-boundary test, and passed diff checks. An initial reviewer invocation
with `--exact` matched0tests; that was not counted, and its corrected invocation
executed/passed1. It then inspected all9 actual PNG/AX captures and independently
matched9 screenshot hashes,4 archive hashes/counts and8 package-file hashes.
No material source, wording, layout or artifact finding remained. This is
independent source/test/artifact assessment, not independent live GUI execution.

Bounded design limitation: current native errors expose text rather than a
structured resource discriminator. Unknown/combined future wording therefore
falls back to truthful generic text. A concrete future106 diagnostic question is
whether a small native resource-reason carrier would reduce presentation-string
coupling; no architecture/refactor work is started or made a new gate here.
No limits were raised, records discarded, timed mode added or rules changed.

## CI and integration

Exact source CI: [Rust40959072793](https://github.com/jweathers777/mShogi/actions/runs/40959072793)
and [Desktop40959072831](https://github.com/jweathers777/mShogi/actions/runs/40959072831).
Both runs are **completed/success at1c6b97e on Windows2022,macOS15 and
Ubuntu24.04**; all6jobs passed. [Exact CI results](ci-results.json) retain head,
steps and platform conclusions. Current native workspace/all-feature session
regressions and frontend/native-host/package/bundled-engine checks passed;
Windows/Linux actual installed/packaged Chess/Shogi GUI regressions also passed.
These latter results do not establish new resource-limit or Xiangqi GUI
campaigns there. Native/controller, frontend/package/bundled-engine and actual
GUI evidence remain distinct. Existing Windows/Linux Chess/Shogi GUI regressions
do not establish new resource-limit or Xiangqi GUI campaigns there.

During this task, the previously completed documentation-head native run
[40949231746](https://github.com/jweathers777/mShogi/actions/runs/40949231746)
ate1bba32 was found failed on Ubuntu: unchanged runner test
`process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored`
expected `UnexpectedExit`, observed handshake `MissingResponse`/Broken pipe,
no exit metadata. This matches the preserved MSH-112 investigation; it is not
fixed or hidden by the current correction. MSH-111 Linux GUI driver failure is
also unchanged. MSH-103's earlier source-qualified green evidence remains its
historical evidence, not a claim that this later run passed.

MSH-103 F01 is reconciled by an additive correction record. Its original
failed probe, independent assessment, unchecked remaining acceptance and In Review
status are preserved. MSH-114 is complete at reviewed/tested/integrated source1c6b97e; the evidence
closure changes only documentation/task records. Resume103 separately; its
corrective dependency is satisfied but it remains In Review. Do not start45/106
here. The normal branch remains codex/rewrite; no temporary worktree was created.

## Launch

Normal app:

```sh
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app"
```

The final normal application is left on the preserved Chess document, stopped.
Use New game for Shogi/Xiangqi. The QA fixture package is development verification
only and is closed after this campaign.
