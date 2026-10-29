# MSH-102 execution evidence

Scope: Xiangqi desktop analysis, outcomes, recovery and document journeys.
MSH-45, MSH-103, MSH-88, MSH-106 and M4 are not closed or started by this work.
The accepted MSH-101 shell, board, geometry and inspection conventions remain.
[Final normal playable application](final-normal-playable.png) shows a fresh
Red move and the genuine bundled Black reply at the final source.

## Source and package identity

- Integration branch: `codex/rewrite`.
- Implementation: `6009b25e8f4b3bcbfce2db5421b3e93ef2a5850d`.
- Save As naming correction: `f8149ce06d11844adde8b015baca58d3bad1d0d4`.
- Reviewed capacity/concluded-warning feedback: `6dec5abd2cc1c5da985ec9814029c6b2099831b5`.
- Final production source: `a1f8234e6d82add1c485e7e2537e414cffd4d49d`.
- Final normal host SHA-256: `c1edb03921b366f5c439e09bee252fa09738ddd24ba998c0934b7dc8f4aef847`.
- Final verification host SHA-256: `3b25b3cd4f671373796635a83335bf0e59703ea5aaa1edc39844cd6bbbddd8a5`.
- Bundled engine SHA-256: `b3d51b474f1eb552f1f7740d42b975cdc1efad3e814dc5a60771279383e3569a`.
- Environment: macOS 27.0.1 (26A434), Apple arm64, Rust 1.99.1,
  Node 24.19.0, strict TypeScript, Tauri/WKWebView native packaged application.

Final package files, sizes, hashes, environment and GUI-created archive
measurements are in [the manifest](package-and-gui-manifest.json).
Normal app: `desktop/src-tauri/target/release/bundle/macos/mShogi.app`,
identifier `org.mshogi.desktop`. QA app: sibling `mShogi Verification.app`,
identifier `org.mshogi.desktop.verification`, explicit fixture/test-state title.
No binaries or generated build directories are committed as evidence.

The `normal-*` and `qa-*` captures without `final-` were executed against the
6009b25 implementation packages. The initial normal package smoke records its binary
identity. The initial QA binary was not separately hashed before replacement;
its source, build/configuration and fixture-title provenance are retained. Final
QA binaries and all final correction captures have full hash provenance. Only naming/capacity/concluded-warning presentation changed afterward;
unchanged journeys are reused. Every `final-*` capture is from the final a1f8234
packages. The initial fourth-loss and cheap-capacity captures retain evidence of
findings; their final captures supersede them as acceptance evidence.

## Actual packaged GUI execution

Integrator controlled native applications using Cua accessibility, pointer and
keyboard input; no IPC or JavaScript injection was used to make game changes.
Each PNG has its corresponding full accessibility capture (`.ax.txt`). These
establish actual macOS interaction; they do not establish Windows/Linux Xiangqi
GUI execution. Status announcements were inspected as accessible live/status
content, without claiming a separate screen-reader campaign.

| Journey | Executed result / evidence |
| --- | --- |
| Ordinary play and analysis | Normal Red `e3e4`, actual bundled Black reply `b7b0`; read-only move analysis at event2 returned actual Red evaluation/depth/nodes/continuation. Live board remained2plies. [Analysis](normal-analysis.png). |
| Analysis replacement and cancellation | QA real-engine delay wrapper: depth3/512 Analyze, Replace, Cancel; stopped at0plies without a result/action. [Running](qa-delayed-analysis-running.png), [cancelled](qa-analysis-replaced-cancelled.png). |
| Cross-purpose ownership | Final QA: human move→pending engine turn; start read-only analysis for Black/event1, cancelling the engine turn; cancel analysis at1ply; explicit Retry launches a new real engine turn. [Cancelled at1ply](final-qa-cross-purpose-cancel-keeps-ply-one.png), [actual retried reply](final-qa-cross-purpose-retry-actual-engine.png). Native context/stale tests establish exact request retirement. |
| Incomplete bounded work | Actual depth3/one-visit result: Search incomplete, No completed evaluation, actual node-limit status, no invented move/claim/result. [Incomplete](qa-incomplete-unscored.png). |
| Warning vs conduct loss | Actual F022 warning requires Red to vary; F023 fourth occurrence concludes Black wins/perpetual check. Final concluded view suppresses live obligations, preserving all warning events. [Warning](qa-repetition-warning.png), [final loss](final-qa-fourth-loss-no-live-obligation.png). |
| Natural-count claim | Actual native100-ply fixture: claimant Red opportunity, confirmation, formal claim, native Draw/natural-count conclusion at same100boardplies. [Confirmation](qa-natural-count-confirmation.png), [conclusion](qa-natural-count-concluded.png). |
| Draw agreement and resignation | Native Black offer alone is not a draw; actual Decline continues, actual Accept concludes agreement. Resignation Cancel preserved game; confirmed resignation concludes Black win. [Offer](qa-draw-offer-opportunity.png), [declined](qa-draw-declined.png), [agreement](qa-agreement-concluded.png), [resignation](qa-resignation-concluded.png). No automatic engine agreement policy was invented. |
| Sound fast no-win | Actual bare-General native record concludes a no-win draw; Technical identifies sound-fast proof and distinguishes saved exact reports. [Proof](final-qa-sound-fast-no-win.png). |
| Failure and retry | Actually missing executable produced engine failure/unscored at0plies. Restoring an external test symlink to the genuine bundled engine and pressing Retry produced a real move; native history/focus retained. [Failure](qa-engine-failure.png), [recovered](qa-engine-retry-recovered.png). |
| Exact event history | Native event8 is an intra-command board action before warning9: same board, different semantic prefix, viewable with analysis disabled and explicit explanation. Committed event7 analyzes with Black perspective and its own prefix. [Reopened partial prefix](final-normal-warning-reopen-partial-prefix.png), [event7 analysis](final-normal-historical-analysis.png). No TypeScript reconstruction. |
| Return to live / Resume | Return selects live event9, restores its warning and pauses; explicit Resume restores live play. Focus returns to live status. [Live resumed](final-normal-live-warning-resumed.png). |
| Save / close / reopen | Normal Save As wrote complete native record+settings+two observations while viewing event0; native clean-window close completed; reopened selection remained historical and engine work stopped. [Reopen](normal-reopened-selected-history.png). Later final normal package reopened native warning and capacity documents created by the QA package. Saved searches stayed historical. |
| Open failure | Duplicate/malformed unsupported archive rejected; same normal document/title/selected history retained. [Retained](normal-failed-open-retained.png). Native tests also cover strict corrupt/conflicting/resource-limited admission. |
| Save failure | Controlled temporary directory made nonwritable after a valid Save, then Flip changed document. Save failed, dirty state remained, previous valid file SHA-256 unchanged before/after: `cb054baa48a37e0ec172ba5290a89330619bb8f2f4f0f1862994b389be1b1f44`. Permissions restored, subsequent Save succeeded. [Failure](normal-failed-save-retained.png). |
| Unsaved replacement / quit | Open and New protect dirty state; native Cmd-Q and window-close use the shared guard; Cancel retains game. Final keyboard Tab→New game→Return→Escape restores focus to New game. Native Save As now defaults `xiangqi.mshogi`; Escape cancels it. [Open guard](normal-unsaved-open-guard.png), [native quit](normal-native-quit-unsaved-guard.png), [keyboard focus](final-normal-keyboard-guard-focus.png). |
| Persistence capacity | QA constructor makes255 genuine native one-node observations; GUI accepts256th, refuses257th before history changes, then successfully saves all256. Final normal package reopens all256 stopped/historical, refuses more analysis, and Save preserves the notice and disabled analysis. [256 saved](qa-capacity-256-saved.png), [final refusal](final-normal-capacity-reopened-refusal.png), [after Save](final-normal-capacity-save-keeps-feedback.png). |

The first saved capacity file measured1,049,452 encoded bytes,513 desktop
controls,256 observations and0native board events; after normal reopen/Save it
measures1,049,196 bytes with the same counts. Normal played+analyzed Save is
13,066 bytes (6controls/2native events/2observations). Warning-prefix Save is
7,930 bytes (2controls/9native events). File hashes are in the manifest.
No constant was raised and no accepted history/observation was discarded.
MSH-99's byte/event/control/replay/close-headroom boundary campaign remains
applicable; the complete all-feature session suite re-executed it successfully.

## Acceptance coverage

| Criterion | Supporting boundary evidence |
| --- | --- |
| AC1 analysis | Native actual_read_only_analysis context-mismatch suite and cancellation/replacement/retirement suite; frontend command/target/limits tests; normal/historical real analysis and QA cross-purpose/Replace/Cancel captures above. |
| AC2 outcomes/recovery | Native draw advice/formal admission, terminal audit, claimant, history and retry suites; authoritative warning/fourth/count/agreement/resign/fast-proof/failure GUI journeys; defensive pending DTO explicitly qualified below. |
| AC3 documents/history | All-feature native archive regression suite (strict identity/replay/failures/capacity/fresh identities), host close guards and ProductApp/useGameSession tests; actual normal Save/Open/close/unsaved/failed-write/bad-open/history/Resume and full256 save/reopen journeys. |
| AC4 verification/review | Actual Mac normal and labeled QA packages with manifests/AX/PNG, real native engine, independent source/test/artifact review with both findings resolved, final frontend/native checks and exact platform CI in completion JSON. |

## Verification instrumentation and limits

QA only: native fixtures `xiangqi-standard`, `warning`, `fourth`, `count`,
`offer`, `agreed`, `resigned`, `fast-proof`, `failure`, `capacity` (all names use
`xiangqi-`). They construct authoritative games using the reviewed source
corpora and native commands, not fabricated conclusions or scores. Capacity
constructor responses come from actual native one-node execution.

For deterministic UI access to pending work, the QA host alone accepts
`MSHOGI_VERIFICATION_ENGINE`. The external executable used here was:

```python
#!/usr/bin/python3
import os, time, sys
time.sleep(8)
os.execv(REAL_BUNDLED_ENGINE, ["mshogi"] + sys.argv[1:])
```

`REAL_BUNDLED_ENGINE` was the normal package's hashed engine above. The bridge
supervises this same process; replacement/cancel kills it. This delays launch,
not fake telemetry, and proves pending-process control rather than search
performance or a particular point of interruption inside the search algorithm.
The ordinary completed/incomplete searches and retry use the genuine engine.
Missing-path and restored-symlink recovery are explicitly external fault
injection. All wrappers/symlinks are outside normal distributables.

Defensive unresolved-adjudication presentation is tested with a clearly labeled
presentation DTO. No admitted actual native fixture was found that reaches that
pending guard, so no actual GUI proof of reaching that defensive state is claimed.
Invalid/stale refusal and fresh process identity protections are exercised by
native/controller tests; ordinary UI exposes native-admitted capabilities.
Explicit exact analysis remains a separate bounded native/console operation;
this task does not invent desktop Chess Full or Xiangqi exact execution. Saved
exact proof/witness/Unknown/cancel/failure evidence remains native MSH-99 data.

## Focused checks and independent review

Commands were run on the implementation committed in6009b25 (unchanged native
source througha1f8234), except final frontend checks explicitly ata1f8234:

- `cargo test --locked -p mshogi-desktop-session --features desktop-verification`:
  **132 passed**,106.19s, including actual native history/purpose/limits/stale
  response/selection, strict Open/restoration and archive-boundary regressions.
- `cargo test --locked -p mshogi-core --test xiangqi_history`: **16 passed**,
  including advisory/formal draw-admission equivalence.
- `cargo clippy --locked -p mshogi-desktop-session --all-features --all-targets -- -D warnings`:
  passed; `cargo fmt --all -- --check` passed.
- Native host `cargo test --locked --manifest-path desktop/src-tauri/Cargo.toml`:
  **5 passed**; native close/quit guards preserved. Host fmt passed. The existing
  Tao patch and its provenance are unchanged; dependency warnings were retained.
- Finala1f8234 `npm test`: **197 passed**,23files; `npm run check`: passed.
  Focused capacity/final Shogi/Xiangqi suite: **29 passed**. Actual output is
  retained here. Node24.19.0 was selected explicitly because the machine's
  unrelated Homebrew Node installation has an unavailable ICU dylib.
- Native-shaped frontend fixtures regenerated by
  `cargo run --locked -p mshogi-desktop-session --features desktop-verification --example write_frontend_fixtures -- desktop/src/__fixtures__`.
- Normal/QA release package builds passed. Final
  `python3 desktop/scripts/package-smoke.py <normal-app> --report ...` passed:
  normal host lacks QA fixture/override/verification entrypoint markers; bundled
  engine UCI/internal-play/read-only-analysis/bounded Chess Full compatibility
  checks passed. Package checks are not GUI proof.
- `git diff --check` passed.

Independent reviewer `/root/xiangqi102_boundary_review` reviewed the bounded
095f27a→a1f8234 source changes, requirements and tests; independently executed
50initial frontend boundary tests, TypeScript, targeted native advice/terminal/
retired-response/Open/fixture-replay tests, then29final focused frontend tests.
They inspected root-executed PNG/AX artifacts against accepted101 evidence,
including all8final correction/history/focus images. This was independent
source/test/artifact review, **not independent GUI execution**.

Two P2 findings were resolved and independently rechecked: capacity must not
hide unrelated failed Save feedback; a concluded conduct loss must not show
current warning obligations. Final captures and tests support both corrections.
Review also checked retained-observation feedback lifetime (Save vs fresh
Open/New) and count-vs-storage wording. No outstanding material finding.
Concrete future diagnostic duplication findings are in
[desktop-journeys.md](../../../xiangqi/desktop-journeys.md); MSH-106 is not started.

## CI and platform scope

Final source **a1f8234** CI is PASS on all three platforms:
[Rust workspace40874262186](https://github.com/jweathers777/mShogi/actions/runs/40874262186)
and [Desktop packages40874262241](https://github.com/jweathers777/mShogi/actions/runs/40874262241).
Exact platform steps/results are retained in [ci-results.json](ci-results.json).

Historical failure retained: intermediate [Rust run40874033735](https://github.com/jweathers777/mShogi/actions/runs/40874033735)
at6dec5ab failed only Linux's unchanged runner test
`process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored`:
mode `exit` expected `UnexpectedExit`, observed `MissingResponse`, engine0,
handshake, `Broken pipe (os error32)`, no exit metadata. See
[actual failed log](intermediate-linux-runner-failure.log). Runner source is
unchanged from095f27a througha1f8234. Final all-platform PASS does not resolve
this intermittent boundary; MSH-112 (future To Do, unassigned, no new gate) records it separately and
preserves MSH-25's earlier, different idle-other-engine fixture-race correction.
No test/outcome contract was weakened or runner implementation changed here.
 Native/controller tests on three systems, package/bundled-engine
checks, and actual Windows/Linux Chess/Shogi regression GUI results must remain
distinct from the new **macOS Xiangqi GUI** execution above. This task does not
claim new Windows/Linux Xiangqi GUI completion or all-game three-platform closure.
MSH-111's recorded Linux driver-reset failure is preserved, not rewritten by a
later independent CI pass. MSH-88 owns final all-game packaged platform acceptance.

## Launch and remaining boundaries

From the primary checkout, launch the normal product:

```sh
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app"
```

Choose New game→Xiangqi→Red or Black. Rebuild with Node24.19.0 and Rust1.99.1:
`cd desktop && npm ci && npm run desktop:build -- --bundles app`.
Development launch: `npm run desktop:dev` from that directory.

Untimed play only. No new strength claim, timed play, branching, automatic engine
agreement policy, exact desktop solver, invented outcome or live progress. Archive
capacity is bounded and refusals preserve a complete Save path. Full native
contexts/encodings remain in Technical. Minimum/intermediate window geometry and
opponent inspection reuse unchanged accepted MSH-101 evidence, with its original
package/resize qualifications; no new minimum-size campaign is claimed here.
