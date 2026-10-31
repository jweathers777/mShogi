# MSH-125 desktop workflow acceptance

Scope: Save/Load/New/Quit, direct Resign and native one-step Return to live.
MSH-124 shell, Moves/native notation and last-action geometry, R5 interaction,
native rules, archive formats, resource limits and engine policy remain intact.
Root Codex is builder/integrator in the primary `codex/rewrite` checkout.
Bounded frontend and host/verifier workers owned disjoint files; an independent
reviewer assessed source, executed focused checks and inspected actual captures.
No other task or variant implementation was started.

Initial implementation/package source:
`871a1fa4b6ae1b463aa4c2abd92bb696d5cfb550`.
Corrected final production and verification source:
`58a5d7ad95cba1724118a91d8ad4958fb1c287ed`.
Verification-only observer correction:
`8de41e46c892613c0a44d0d3991ffda0f88e8b5e`. It changes only the external
GUI verifier, its regression fixture/tests and task notes; production58 is
unchanged. Completion/evidence commits add no product behavior. Final verdict: **PASS**
for all seven MSH-125 criteria, with no unresolved material task finding.
MSH-121/R6 and M4 remain open; the known MSH-112 runner race is not fixed here.

## Behavior and authority

- One shared header: **New game / Save / Load**. No ordinary Save As. First Save
  opens the native picker; later Save uses the accepted path. Internal atomic
  Save As remains. Picker cancellation is a rejected receipt, not success.
  Exact native source tokens are rechecked after the picker while serialized;
  old receipts cannot authorize a newer document's close or replacement.
- New/Load intentionally replace without an extra unsaved confirmation. Opening
  or cancelling a chooser, failed validation/admission and stale requests retain
  the installed canonical document. Successful replacement intentionally loses
  unsaved old work. No implicit autosave or recovery storage was added.
- Native `close_requires_save` protects **never-saved or changed** documents.
  It includes path=None even when initial canonical dirty=false. Save Game has
  Save / Discard / Cancel, initial Cancel focus and the shared focus trap.
  Cancellation/failure keeps the guard/game open. Clean saved Quit avoids a
  prompt, with a native exemption only at its exact stop-only post-close token;
  it never labels stop controls disk-saved. Newer changes invalidate that
  exemption. Native file/close reservations, window/menu/macOS terminate and
  actual host/engine shutdown remain authoritative.
- Resign submits once with the rendered token and native human actor. Immediate
  synchronous fencing and operation ownership prevent double submission. Other
  claim/declaration/agreement confirmations remain. Native conclusions supply
  the result, including genuinely unresolved/unscored Chess resignation.
- Native sessions retain transient browse-origin intent, not saved work. Only
  entry from otherwise running play grants one-step Return to live. Historical
  work is retired; eligible engine turns allocate fresh request/version/process
  identities. Return plus the normal journaled Resume is one capacity-admitted
  candidate with existing close headroom. Repeated history navigation grants no
  new authority. Loaded/Take back/cancelled/failed/capacity/concluded states stay
  stopped; a newer stop overrides older recovery. Historical analysis remains
  read-only, and existing concluded Chess Full analysis can finish retirement
  without reopening play or inventing an outcome.

## Acceptance mapping

| Criterion | Supporting evidence |
| --- | --- |
| AC1 Save/Load/atomic persistence | Shared header and hook tests, host12 real file operations, existing atomic archive/admission regressions, actual macOS first/subsequent/cancelled Save; unchanged complete envelope/observations/lineages and limits |
| AC2 deliberate atomic replacement | All3 shared ProductApp/native/host source-token tests; actual chooser cancellation and rejected/cancelled Load retain native context; accepted New/Load replaces without an extra guard |
| AC3 Quit | Exact native close authority, clean-stop/newer-token and untouched never-saved regressions; all3 frontend Save/Discard/Cancel/stale receipt tests; actual macOS CmdQ/window-close and QA Cocoa terminate; actual PID exit checks |
| AC4 direct Resign | All3 action tests incl. synchronous duplicate/stale rejection and unresolved Chess fixture; actual normal packaged Chess unresolved/unscored, Shogi Gote win and Xiangqi Black win supplied by native conclusions |
| AC5 browse-origin | Eleven native workflow regressions cover all3 running human/engine, fresh/retired identities, same-board context, historical analysis, Load/Take back/cancel/failure/conclusion; two near-control-limit tests preserve complete saveability; actual one-step Return and stopped Resume journeys |
| AC6 concrete tests | Focused and full native/frontend/host checks below; existing archive capacity and external protocol tests reused where unchanged |
| AC7 package/review/integration | Identified normal and separately labeled QA packages, actual interactions/captures, independent source/test/pixel review, normal instrumentation exclusion, CI and integration recorded separately below |

## Checks and independent review

Local environment: macOS27.0.1 (26A434), arm64, Rust1.99.1; bundled Node24.19
runtime required by desktop/package.json. Commands use locked Rust dependencies.

- Final `npm --prefix desktop test -- --run`: **322 PASS / 26 files**.
- `npm --prefix desktop run check`: **PASS**.
- `cargo clippy --locked -p mshogi-desktop-session --all-targets --all-features
  -- -D warnings`: **PASS**. Initial adjacent-if formatting warning corrected;
  final format/diff checks pass.
- `cargo test --locked --manifest-path desktop/src-tauri/Cargo.toml --features
  desktop-verification --bin mshogi-desktop`: **12 PASS** on final58 source, actual file operations,
  all3 post-picker identities and cancelled/failed/stale admission.
- `python3 -m unittest discover -s desktop/scripts -p 'test_gui*.py'`:
  **92 PASS** initially; final **94 PASS** after strict Chess match-ledger
  resignation probe and actual-payload negative tests. These are external
  verifier/adapter tests, not GUI evidence.
- `cargo test --locked -p mshogi-desktop-session --all-features`: final
  **212 PASS**, 192.16s (including QA constructors and long real capacity cases).
  Existing resource boundary and
  atomic archive tests are real native payload/replay checks, not frontend rules.
- Independent fresh reviewer read the bounded baseline07c0be6-to-current diff;
  executed focused native workflow/capacity, frontend/hook/action/recovery checks,
  92 verifier checks and independently generated/byte-compared all23 native
  fixtures. Reviewed nine final58a5d7a captures, six earlier871 captures and12each final
  Linux/Windows captures; independently audited their hashes/package manifests
  and12 complete focus outlines. Final acceptance PASS for all7criteria. Verified
  eight final package file hashes and16 normal-host exclusion markers; no
  unresolved material source or visual finding. Full report and separate commands are preserved in
  [independent review](evidence/msh125/checks/independent-review.md).

## Actual macOS packages and interactions

[Package hashes](evidence/msh125/macos/package-manifests.json) identify normal
and QA packages at both871a1fa and58a5d7a. Build commands:
`npm --prefix desktop run desktop:build -- --bundles app` and
`npm --prefix desktop run tauri -- build --features desktop-verification
--config src-tauri/tauri.verification.conf.json --bundles app`.
The known20 local Tao dependency warnings remain recorded, not product errors.

Normal `package-smoke.py` checks at both revisions inspect actual bundle contents
and exercise its real bundled UCI/internal play/analysis/Full paths. Final
[smoke report](evidence/msh125/macos/normal-package-smoke-58a5d7a.json) proves
verification commands/fixtures/menu markers absent from the normal host.
QA is separately named **mShogi Verification**, never claimed a normal package.

Actual native CUA actions and AX observations are in the
[interaction log](evidence/msh125/macos/interaction-log.json). Captures at871
are unchanged-workflow evidence; captures ending58a5d7a establish the corrected
never-saved close behavior and final minimum/native termination checks.
No hidden IPC, injected UI action or frontend rule engine drove these journeys.

- Normal871 shared aggregate journeys across all3, not every case in every game:
  native New selector and side choice without dirty guard; Chess chooser/Save/Load
  cancellation and first/subsequent Save; Shogi Save/Load; Xiangqi Save and malformed
  Load refusal. Successful Load remains stopped until Resume. Complete per-game
  CI phases, when available, are identified separately below.
- Real human move and native engine response; history selection followed by one
  Return to live restores human turn without Resume in all3. Chess Take back
  then history Return stays stopped; loaded Shogi then history Return likewise.
- Direct Resign: Chess shows **Unresolved — unscored**, Shogi Sente resigned /
  Gote wins, Xiangqi Red resigned / Black wins. No confirm dialog or assumed
  frontend score. Complete saved Take back/engine observations retained.
- Native macOS CmdQ Save Game Cancel keeps game; Save actually writes then exits;
  Shogi native window close Cancel then Discard exits. All3 share concise labels.
- Final normal58a5d7a untouched Chess CmdQ opens Save Game despite canonical
  dirty=false/path=None. Save is enabled, picker Cancel retains dialog and game;
  dialog Cancel releases reservation. Real first Save sets path/UNMODIFIED;
  subsequent Save has no picker. Bare CmdQ on saved clean game exits PID91838
  without guard; no post-exit CUA app lookup was used (such lookup can relaunch).
- Final QA58a5d7a actual Cocoa terminate menu opens guard, Cancel preserves Chess;
  all3 full-board minimum760×650 captures retain shared shell; intentional New
  switches game without guard. Cocoa terminate on unsaved Xiangqi then Discard
  exits PID92029. Exact [process checks](evidence/msh125/macos/termination-process-checks.json)
  establish host absence and no matching packaged engine process.

Representative captures:
[never-saved Quit](evidence/msh125/macos/normal-never-saved-quit-58a5d7a.png),
[cancelled Save](evidence/msh125/macos/normal-never-saved-save-cancelled-58a5d7a.png),
[Chess one-step Return](evidence/msh125/macos/normal-chess-one-step-return.png),
[loaded Chess](evidence/msh125/macos/normal-chess-loaded-stopped.png),
[unresolved resignation](evidence/msh125/macos/normal-chess-resign-unresolved.png),
[Shogi Return](evidence/msh125/macos/normal-shogi-one-step-return.png),
[Xiangqi Return](evidence/msh125/macos/normal-xiangqi-one-step-return.png).

## Original failures and corrections

Preserve first source871 CI runs:
[native41212862703](https://github.com/jweathers777/mShogi/actions/runs/41212862703),
[desktop41212862693](https://github.com/jweathers777/mShogi/actions/runs/41212862693).
Native all3 workspace/QA/lint passed, fixture writer failed by trying Resume
on concluded Idle. Independent reproduction confirmed it; writer now obeys
native admission and all23 generated snapshots match. Native guard was retained.

Linux original12/15 GUI phases passed, three failed: initial Chess probe assumed
canonical dirty=true for never-saved startup; Shogi Moves pointer target was
below the nested clip after malformed-load notice; reopened Shogi lacked that
failed phase's save. Windows14/15 passed; only the same initial Chess assertion
failed. Actual initial reports, failure DOM/pixels and artifact hashes are kept
under `initial-linux`/`initial-windows`. Mac package build passed.

The initial Chess assertion revealed a real missing Quit guard, not just a
probe mismatch: fix native pathNone-or-dirty close authority and Save gating.
The driver now asserts this native flag and unchanged dirty state. History
probe uses real bounded Tab reveal before one pointer click, not DOM scrolling
or synthetic command injection. A local test initially queried both Save
buttons; scope it to the dialog. These original failures are not labeled passes.

Independent F01 (older failure masking newer historical analysis stop) and F02
(fixture generator) are resolved with positive native regressions. No new rules,
archive version, capacity constant, pending-work restoration or UI framework.

## Final CI and handoff

Final verification8de41e4:

| Check | Actual result and provenance |
| --- | --- |
| [Native41215847540](https://github.com/jweathers777/mShogi/actions/runs/41215847540) | Attempt1 **PASS all3**: Linux24.04, macOS15, Windows2022; workspace/lint/QA constructors/fixture regeneration/build/console checks. [Exact metadata](evidence/msh125/native-final-ci.json) |
| [Desktop41215847392](https://github.com/jweathers777/mShogi/actions/runs/41215847392) | **PASS all3**: checks, normal/separate-QA package construction and bundled-engine/exclusion smoke; actual Linux/Windows interaction below. [Exact metadata](evidence/msh125/desktop-final-ci.json) |
| Linux installed/packaged GUI | **15/15 phases PASS**; exact8de source, job124580079419/artifact13048582511. [Full report](evidence/msh125/linux/gui-report.json), [audit](evidence/msh125/linux/audit.json), normal/QA manifests and bounded captures in the same directory |
| Windows installed/packaged GUI | **15/15 phases PASS**; exact8de source, artifact13048413458. [Full report](evidence/msh125/windows/gui-report.json), [audit](evidence/msh125/windows/audit.json), normal/QA manifests and bounded captures in the same directory |
| macOS GUI | Actual normal871 aggregate journeys plus rebuilt normal58 never-saved/clean close and QA58 Cocoa termination/minimum checks, described above. CI packaging is not mislabeled GUI evidence. |

Both remote reports independently establish normal Chess/Shogi/Xiangqi and
separate QA journeys: rejected/cancelled intentional replacement, first and
subsequent Save, cancelled Save retaining Save Game, dirty Quit Save/Discard/
Cancel, clean Quit and actual host/engine absence, direct native resignation,
one-step running history Return, stopped Load/Take back Resume, real analysis,
selection/inspection, focus and minimum layouts. Full report commands and native
contexts supply exact per-phase facts; screenshots alone do not prove freshness.

External evidence auditor verified **891 GUI-file hashes** (469Linux/422Windows),
all13Linux/twoWindows normal-package files and both QA installer hashes, and
**12 complete focus-outline bounds**. Twenty-three bounded captures per platform
are retained here; full artifact inventories remain in the hashed CI artifacts.
Independent source/review and personal pixel assessment remain distinct from
author/driver execution. No GUI failure is hidden by package-build success.

Corrected production58 native
[41214429663](https://github.com/jweathers777/mShogi/actions/runs/41214429663)
also passed all3 after the normal Linux-only failed-job retry124578865084.
[Attempt1](evidence/msh125/checks/native58-attempt1.json) and
[final attempt2](evidence/msh125/checks/native58-attempt2-final.json) remain
separate. Linux originally observed MSH-112's unchanged handshake broken-pipe32/
exitNone as MissingResponse instead of expected UnexpectedExit, after desktop
session200 passed. Identical-source retry passed the exact assertion, QA212,
fixtures/build/console. No runner assertion, timeout or source changed. Independent
[source/log disposition](evidence/msh125/checks/independent-native-ci-disposition.md)
confirms the recurrence; green retry is not a fix. Earlier871 fixture/probe and58
Chess observer failures remain preserved in their separate directories/reports.

`git diff 58a5d7a 8de41e4 -- crates desktop/src desktop/src-tauri Cargo.toml
Cargo.lock .github/workflows` is empty. Final documentation/task/evidence changes
reuse these passed source/package checks rather than repeating an unchanged
production campaign. Final Backlog doctor, evidence links and diff checks pass.
Completion4733f6a initially exposed CRLF and captured-log trailing whitespace
in16 imported text artifacts during the staged diff check; the shell continued
the commit despite that failed check. Preserve those byte-exact originals in
[raw text archive](evidence/msh125/raw-text-artifacts.tar.gz), with original and
normalized SHA256 values in [provenance](evidence/msh125/text-evidence-provenance.json).
Repository text views normalize line endings/trailing whitespace only; all JSON
views parse identically. Original CI ZIP/package/capture hashes and reported
observations remain unchanged. Corrected final diff checks pass; no product
code, assertions or CI outcomes changed.

The primary checkout stays `codex/rewrite`; no temporary task worktree was created,
and unrelated existing worktrees were preserved. Normal integration/push records
are in MSH-125 and Git history; no published history was rewritten.

Launch from the primary checkout with `open desktop/src-tauri/target/release/bundle/macos/mShogi.app`.
Rebuild normal using the command above; Node24 is required. Development:
`npm --prefix desktop run desktop:dev`. The raw release executable may have been
replaced by a later QA build; launch the normal .app, not the QA executable.

Untimed Chess/Shogi/Xiangqi only. Existing bounded work, conservative Unknown
and unresolved outcomes remain. Intentional successful New/Load discards unsaved
work by owner decision; Quit protects it. Limits, explicit stop/retry and no
saved pending execution remain. MSH-121/R6 owns integrated architecture/product
acceptance after125; no M4/variant/M5 closure is implied.
