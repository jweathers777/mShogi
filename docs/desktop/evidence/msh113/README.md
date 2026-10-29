# MSH-113 Take back evidence

Take back is delivered for human-versus-engine Chess, Shogi and Xiangqi. Native
command actors and complete boundaries select a playable human decision in one
admitted transaction. The restored position pauses for explicit Resume. MSH-103,
MSH-106 and other later work are unstarted; closed milestones and game evidence
remain intact.

## Revision and package provenance

- Base: `8f21eb212d1847d75a7d8580882ae4afeacf193b`.
- Reviewed implementation: `ff0b1ce5d2507d60bb28c0739c24fa0a49b2e69b`.
- Final production source: `bfe09cf4fed1827d868467a7cc475a3a175931ba`.
- Integration branch: `codex/rewrite`; normal pushes, no rewritten history.
- Environment: macOS27.0.1 (26A434), Apple arm64, Rust1.99.1, Node24.19.0,
  native Tauri/WKWebView application.
- Final normal host SHA256: `a84b071d556581530986e527a092cf3795ef955b00513ad88cacdc4a6219b868`.
- Final verification host SHA256: `4a24b7a0113a4897a3d5e06e7388c4b8cfb39b9f892f5b7a447a86c8e634681c`.
- Genuine bundled engine SHA256: `a31b22c8928112d1175d0f33540f8ba6e760ac3b3d3682353867ab8d63efd796`.

[Manifest](package-and-gui-manifest.json) records every package file/hash/size,
source, environment, real GUI-created archives and the external delay wrapper.
Normal app: `desktop/src-tauri/target/release/bundle/macos/mShogi.app`;
QA: sibling `mShogi Verification.app`, separate bundle identity and explicit test
state/window title. [Normal package smoke](final-normal-package-smoke.json)
passed inventory/instrumentation exclusion and actual UCI/internal Chess play,
move analysis and bounded Full calls. Package construction/protocol smoke alone
is not GUI evidence.

## Actual packaged macOS interaction

The integrator used Cua native accessibility, pointer and keyboard actions. No
IPC or JavaScript injection supplied moves or changed game state. Every local Cua capture
has its corresponding full `.ax.txt`; CI captures use their recorded `.dom.json`
sidecars. All `final-*` and `qa-*` evidence below uses
the final bfe09cf packages. The few `normal-chess-*` and `intermediate-*` files
precede the presentation correction and are qualified below.

| Journey | Actual result and capture |
| --- | --- |
| Chess completed pair | Human `e2e4`, genuine bundled `a7a5` (depth2/642nodes). [Pair](final-chess-pair.png); Take back restores White at0plies, pauses, retains the engine observation, disables further Take back. [Restored with visible Resume](final-chess-restored-resume-visible.png). |
| Chess Save/Open | Native Save As writes schema3 with complete retained match checkpoint and1historical observation. Reopen remains0plies, stopped, clean; explicit Resume is required. [Saved](final-chess-v3-saved.png), [reopened](final-chess-reopened-stopped.png). |
| Shogi pair/history/inspection | Human `7g7f`, genuine `4c4d`; selecting event0 disables Take back. Return to live explicitly, then inspect Gote pawn3c using native movement inspection. Take back restores Sente0plies and clears old markers, with confirmation and Resume visible. [Pair](final-shogi-pair.png), [history](final-shogi-historical-disabled.png), [inspection](final-shogi-inspection-before.png), [restored](final-shogi-restored-clears-inspection.png). |
| Shogi Save/Open | Native schema2 retains its ordered journal and1observation; actual Open restores clean/stopped0plies. [Saved](final-shogi-saved.png), [reopened](final-shogi-reopened-stopped.png). |
| Xiangqi human second | Genuine Red opening `b2×b9` produces1ply and no eligible Take back. Human Black `a9×b9`, genuine Red `h2×h9` produces3plies. Take back restores Black at1ply and preserves the opening. [Opening](final-xiangqi-opening-preserved-unavailable.png), [pair](final-xiangqi-three-plies.png), [restored](final-xiangqi-restored-opening.png). |
| Xiangqi Save/Open | Schema2 reopens clean, stopped and Black at1ply; no abandoned response is automatically reapplied. [Saved](final-xiangqi-saved.png), [reopened](final-xiangqi-reopened-stopped.png). |
| Concluded decision | Explicit Resume, native confirmed Black resignation at1ply, then Take back restores the playable Black decision at1ply without erasing Red's opening. [Concluded](final-xiangqi-resigned.png), [restored](final-xiangqi-terminal-restored.png). |
| Keyboard activation | Resume, another genuine Black/Red pair; Tab traversal reaches Take back, Return executes it, focus returns to live status and Resume is visible. [Keyboard result](final-xiangqi-keyboard-takeback.png). Final Save retains3observations and the complete terminal/takeback journal. [Saved](final-xiangqi-retained-journal-saved.png). |
| Pending work, each game | Separate QA app delays actual engine process launch8seconds, then would exec the genuine hashed engine in the same supervised process. Human move→pending1ply→Take back retires it→paused0plies. Captures `qa-{chess,shogi,xiangqi}-pending`, `-pending-retired` and `-after-retired-deadline` establish the restored position stayed stopped beyond the8second launch window. These test pending-process ownership, not cancellation at a specific search instruction; native tests inject actual late responses separately. |
| Native quit and compatibility | Disposable QA uses native Cmd-Q→unsaved guard→explicit Discard and exits (owned process exit0). [Guard](qa-native-quit-unsaved-guard.png). Normal app successfully opens the original preserved schema1 Chess document and is left there stopped. [Legacy/user session restored](final-legacy-v1-user-session-restored.png). |

The prior user session was first preserved with native Save As at
`/private/tmp/msh113-preserved-preverification.mshogi`, then restored in the latest
normal app after testing. Verification documents in `/private/tmp` are separate
from this preserved document. No user work was discarded.

## Native/archive contract and capacity

Chess uses actor-qualified MatchRecord prefixes; Shogi uses accepted command
events; Xiangqi uses committed command boundaries, keeping derived warning and
claim events indivisible. Terminal-only own-turn decisions may be reversed
alone; off-turn resignation instead returns before the last human board move.
Native history, provenance and rules profiles remain authoritative. New requests,
selection and process generations retire execution even when the restored board
or canonical history matches an older target.

Chess writes existing schema1 until its first Take back, then explicitly writes
`mshogi.desktop.archive.v3`/schema3. It retains each complete abandoned native
match bound to its Take back control/timestamp, replay-derives the restored
human decision and checks subsequent branch extensions. Observations must target
exact admitted current or retained lineage prefixes. Old readers reject3;
current readers preserve1/2/3 compatibility. Shogi/Xiangqi stay schema2 and replay
their complete ordered journals. Native game record schemas are unchanged.

Limits were not raised:16MiB encoded bytes,256observations,8192controls,
game-specific native event bounds and16000normalized replay-work units. Chess
also retains the4096combined-event ceiling and charges all abandoned checkpoints,
observation histories/variations/witnesses and mandatory stop/close reservation.
Shogi/Xiangqi keep their existing conservative native replay and byte/headroom
charges, extended to Take back. Ordinary admission validates before acceptance;
resource refusal preserves the complete previous saveable document. No accepted
observation or canonical continuation is silently deleted, and Take back does
not replenish storage capacity.

The exact new Chess boundary test uses1320real Observe entries and admits123
additional Preferences:128controls,333246encoded bytes. A further candidate is
refused before acceptance; actual mandatory close and authoritative Open succeed.
The actual GUI archives measure4072bytes (Chess,1checkpoint/1observation),
6053bytes (Shogi,1observation),17102bytes (final Xiangqi,3observations). Hashes,
counts and paths are recorded in the manifest. Existing MSH-61/MSH-99 byte/event/
control/observation256→257/replay/failure campaigns re-executed in the final
144-test all-feature session suite; unchanged broader evidence is reused.

## Executed verification and review

- Final `npm --prefix desktop run check`:PASS.
- Final `npm --prefix desktop test`:208passed/24files,11.65s.
- `cargo test --locked -p mshogi-desktop-session --all-features`:144passed,
  0failed,91.62s at the final source; includes all12new focused tests and existing
  Chess/Shogi/Xiangqi archive/capacity/stale-result regressions.
- Independent `cargo test --locked -p mshogi-desktop-session takeback -- --nocapture`:
  12passed/0failed/123filtered,4.74s.
- Workspace regression run before final review corrections:587passed,
  2intentionally ignored. Final-source platform CI separately reruns the workspace.
- Final workspace all-target Clippy, native session all-target/all-feature Clippy,
  host Clippy, formatting and `git diff --check`:PASS. Existing vendored Tao
  dependency warnings remain; patch/provenance and native quit behavior unchanged.
- Host `cargo test --locked`:5passed; separately labeled QA package build succeeds.
- Pinned Backlog1.55.1 `doctor`:no duplicate IDs, self-dependencies or cycles.
  Installed1.55.0 was not used for mutations.

Actual independent review method: fresh reviewer sessions, separate from the
native builder and root integrator. Native reviewer audited the completed diff,
actor/terminal boundaries, strict replay/lineage, capacity and stale safety, and
independently ran the12tests. Frontend reviewer inspected routing, exact tokens,
availability/focus/errors/marker invalidation, ran focused checks and approved.
A final read-only artifact assessment examined all three restored screenshots,
AX, keyboard/pending evidence and package provenance: Resume visible, complete
boards/common theme preserved, correct human positions, Shogi markers cleared;
no material remaining findings. That assessment is not independently performed
GUI interaction.

Resolved review findings: shared Chess refusal error was initially disconnected;
abandoned Chess checkpoints needed live-record restoration restrictions;
unavailable Shogi requests needed pre-retirement refusal; Chess replay admission
needed reserved close work. Each has a focused regression. Actual GUI found the
scrolled sidebar could clip Resume; the small shared-control correction reveals
the existing notice while retaining status focus. `normal-chess-restored.png`
records that intermediate finding; final captures supersede it. An overlapping
pre-handoff Clippy encountered a transient test type mismatch; final Clippy and
native tests passed. Superseded ff0b1ce CI runs were explicitly cancelled in favor
of the final bfe09cf runs; cancellation is not reported as a pass.

The first bfe09cf Linux desktop run failed only two obsolete GUI assertions that
concluded Shogi games contain no action buttons. The new approved Take back is
correctly present. [Original failure](ci-bfe09cf-linux-failure.json) preserves
tracebacks and artifact provenance. The original Windows job also failed the same
two named Shogi phases, independently confirmed in its job log. Neither was
attributed to the unrelated
MSH-111 driver issue. The external runner now requires exactly one enabled Take
back action, retains all native conclusion/history assertions, and additionally
executes declaration/resignation takeback to paused Sente0events. Independent
review, Python compilation and24runner self-tests passed. Product code and local
packages are unchanged from bfe09cf. Actual corrected GUI CI subsequently passed on Linux and Windows, including
both new native Shogi Take back journeys (14 GUI phases per platform).

[Final CI](ci-results.json) at verification revision
`79de582fb42cdd30f6bcfa6e15a1c5dd2263e4de` passed all six jobs:

| Workflow | Windows2022 | macOS15 | Ubuntu24.04 |
| --- | --- | --- | --- |
| [Rust workspace40910846668](https://github.com/jweathers777/mShogi/actions/runs/40910846668) | PASS | PASS | PASS |
| [Desktop40910846682](https://github.com/jweathers777/mShogi/actions/runs/40910846682) | PASS | PASS | PASS |

The verification commit changes no product source from the locally packaged
bfe09cf build (checked with an empty diff over crates, desktop/src and host
sources). Final evidence/task bookkeeping does not change product or verifier.
[Linux GUI report](ci-linux-gui-summary.json) and
[Windows GUI report](ci-windows-gui-summary.json) preserve exact revision,
environment, driver/package/engine hashes,14passed phases and the2actual Shogi
Take back phase records. Their `ci-{linux,windows}-*takeback*` desktop/webview
captures have matching DOM observations and hashes. This establishes additional
Shogi terminal-takeback GUI evidence on those systems, not new three-game GUI
campaigns. Windows desktop captures are clipped by the1024×768virtual desktop
while the application content is1180×820; matching webview captures preserve the
complete render. Those desktop images do not prove complete physical-window
visibility. The earlier bfe09cf Rust workflow also passed; its desktop workflow
failed the documented obsolete assertions before the verifier correction.

## Limits and launch

All three new Take back GUI journeys were directly exercised on macOS. Corrected
Windows/Linux CI additionally exercises Shogi terminal Take back; other native/
package and existing Chess/Shogi GUI checks remain separate claims; prior MSH-111 Linux driver
and MSH-112 runner failure records remain unmodified. No new all-platform
Xiangqi closure or playing-strength claim is made.

Low-level Chess `from_record` pre-session GameRecord history remains an immutable
import root, because it lacks desktop match-request ownership. Normal desktop
New/Save/Open retains full actor-qualified match history. No new import,
variation editor, timed-play policy, search strength or generalized rules layer
was introduced. Concrete legacy/shared Chess dispatch duplication is recorded
for future MSH-106 without starting that task.

Launch the normal app:

```sh
open "/Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app"
```

Use New game for Chess/Shogi/Xiangqi; Take back is in Game Actions. The current
preserved Chess session is stopped after reopening; Resume play is explicit.
From `desktop/`, rebuild with Node24.19.0 and pinned Rust using
`npm run desktop:build -- --bundles app`. Use `npm run desktop:dev` for development.
