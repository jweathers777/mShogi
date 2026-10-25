# M3 desktop integration review

Verdict: the required M3 behavior is implemented and independently verified.
MSH-39 records the final closure integration and native CI gate; the milestone
closes only when those checks pass. M2 remains closed. Desktop deliverables
MSH-27–38 and the focused persistence follow-up MSH-61 are Done; their identities,
acceptance criteria, reviews and original evidence remain in Backlog.md.

The reviewed integrated source is
`475e3b1a2d81db81175891f07da61db7806d6589`. Its product and verification code is
identical to actual GUI-tested source
`2f0e4cee3e1cc05df99ff51178eb4be810fbcb44`; the later commit adds MSH-38
completion evidence and documentation. The desktop builder revision is
`900932f`. Closure bookkeeping and documentation do not change this behavior.

## Coverage of the eleven required work areas

The [accepted desktop contract](../desktop/m3-design.md) retains the complete
mapping and the historical batch boundaries.

| Area | Deliverables | Integrated behavior and evidence |
| --- | --- | --- |
| 1. Session and interaction contract | MSH-27 | Native authority, complete actions, versioned snapshots, exact history and distinct result purposes are specified in the contract and internal engine v2 protocol. |
| 2. Application and packaging foundation | MSH-28 | Tauri 2, React, strict TypeScript, Vite and SVG use the existing Rust workspace. Actual normal packages contain the bundled engine and exclude verification features. Native package inventories and engine checks are separate from GUI interaction. |
| 3. Authoritative session controller | MSH-29 | Rust serializes commands, session changes, result acceptance and action application. Commands require the expected session version; frontend snapshots have monotonic revisions. The frontend presents native legal choices without a second rules engine. |
| 4. Structured supervised engine | MSH-30 | A bounded native worker supervises a fresh bundled child. Exact game context, purpose, request identity, session version and process generation protect acceptance. Actions, claims, conclusions, incomplete work and failures are distinct. UCI remains available. |
| 5. Playable board | MSH-31 | Native legal actions drive click selection, promotion, orientation and keyboard operation. Promotion cancellation does not play a move. Focused frontend checks and actual packaged promotion journeys establish the interaction. |
| 6. Human play and analysis | MSH-32, MSH-33 | Untimed human White or Black games use actual engine replies. Ordinary analysis is read-only with a separate acceptance path. Historical analysis has the correct prefix and provenance. Advanced Full mating reachability is an explicit bounded child operation; cancellation/exhaustion remains unknown, not a game outcome. |
| 7. Outcomes and recovery | MSH-34 | Optional claims, concluded games, resignation, unresolved unscored outcomes, cancelled/incomplete work and process failures have distinct meanings. A genuine missing-sidecar failure followed by Retry preserves authoritative history. Native quit/window-close protection and the macOS termination regression are retained. |
| 8. Save/reopen/history | MSH-35, MSH-61 | Complete canonical match/game semantics, controls, observations, policies, modes, limits and settings are saved atomically. Open validates a candidate off the interaction path, installs fresh runtime identities and leaves play stopped. History is read-only; Return to live and Resume are separate. Capacity is admitted before commit, preserving a complete save path. |
| 9. Synthetic large board and routes | MSH-36 | The labeled verification-only 16-by-16 fixture uses the actual shared board/selection and native acceptance components. Two routes with the same endpoints remain distinct; cancelled prefixes never commit. Its identity and record are separate from chess and other real games. |
| 10. Integrated interaction/accessibility | MSH-37 | Actual macOS default/minimum-size journeys cover focus recovery, keyboard access, status semantics and scrolling. Material focus, persistent-banner and collapsed-inspector findings were fixed. Frontend tests support those checks; they do not substitute for packaged GUI evidence. |
| 11. Three-platform packages and closure | MSH-38, MSH-39 | Actual Windows, macOS and Linux application interaction covers play, analysis/cancellation, recovery, Save/Open, history, native close/Quit and fixtures. Exact source, package, environment, observations and selected raw images are retained. MSH-39 supplies a fresh independent integration verdict. |

## Verification and review

The [first-batch evidence](../desktop/first-batch-evidence.md),
[second-batch evidence](../desktop/second-batch-evidence.md),
[completion-batch evidence](../desktop/completion-batch-evidence.md) and
[final packaged GUI assessment](../desktop/packaged-gui-verification.md)
record the commands, actual results, revisions and appropriate reuse. The
[MSH-38 independent review](../desktop/evidence/m3-platforms/independent-review.json)
found no outstanding material source, verifier or evidence finding.

Fresh reviewer `m3_final_review` independently assessed all eleven areas at
`475e3b1` and reported **PASS, no material findings** in the
[MSH-39 review](m3-independent-review.md). The reviewer independently passed 46
native session feature tests, 22 protocol/real-process tests, strict TypeScript
and 78 frontend tests; recomputed retained platform evidence hashes; checked
canonical history-prefix and 256-observation archives; and verified ordered
native shutdown evidence for all twelve Linux/Windows phases. This was a
source/evidence assessment with focused execution, not a claim of new remote
GUI operation by that reviewer.

At GUI-tested `2f0e4cee`,
[Rust CI 40234257686](https://github.com/jweathers777/mShogi/actions/runs/40234257686)
and [desktop CI 40234257669](https://github.com/jweathers777/mShogi/actions/runs/40234257669)
passed on Ubuntu 24.04, Windows Server 2022 and macOS 15. These establish locked
native format/Clippy/tests/build, isolated verification-feature checks, native
snapshot fixtures, strict frontend checking and 78 frontend tests, package
inventory and actual bundled-engine protocol/search checks. They also execute
the actual Windows/Linux GUI journeys. macOS GUI interaction is separately
recorded on macOS 27.0.1 arm64, rather than inferred from its macOS 15 CI job.

At the reviewed integration, all three jobs passed
[Rust 40235926930](https://github.com/jweathers777/mShogi/actions/runs/40235926930)
at `475e3b1`. The corresponding
[desktop 40235927049](https://github.com/jweathers777/mShogi/actions/runs/40235927049)
and closure integration verdicts are recorded in MSH-39. Later documentation
and task records preserve this distinction without changing product code.

The persistence follow-up tests the observation 256-to-257 boundary, event and
control limits, real serialized byte capacity, save/reopen identities and
reserved stop/save/close headroom. Actual GUI journeys save all 256 accepted
observations after refusing the next operation. No accepted observation or
canonical history is discarded. Saving/reopening preserves those counts;
the application explains that the user must start another document to continue.
An application resource pause is neither an illegal move nor a chess result.

Linux and Windows each pass six separately launched phases. Every phase records
the native Quit request, its specific guard, explicit completion and absence
of host/engine before teardown. Failure cleanup is never counted as Quit proof.
Selected original reports, complete saved archives, package identities and raw
native screenshots are committed with hash inventories. Earlier failed runs
remain failed, including their findings and corrections. Normal packages never
contain the difficult-state constructors, synthetic entry point or GUI driver.

The [macOS termination regression and Tao patch provenance](../desktop/macos-quit.md)
are preserved. Actual guarded Quit and window-close interaction supplement the
separately labeled Cocoa `terminate:` probe; no actual Dock-click claim follows.

Unchanged [M1](m1-closure.md) and [M2](m2-closure.md) correctness, interoperability,
match-record replay and independent-review evidence remains applicable. M3
does not repeat those deep audits or turn application integration into a
competitive playing-strength claim.

## Capabilities, limits and launch

The normal application plays untimed standard chess against the native mShogi
engine with either human color. It supports complete promotion/claim choices,
orientation and keyboard selection; bounded read-only move analysis and explicit
advanced Full analysis; recovery/retry; complete Save/Open; and read-only history
with explicit Return to live and Resume. Technical details retain request and
process identities and real native search observations.

Interactive play retains the M2 adjudication mode and its sound incomplete dead
recognition. A missing proof is not proof that mate is possible. Resignation
can remain unresolved/unscored. Full analysis is bounded cooperative mating
reachability, not forced-win analysis or automatic live adjudication. Unknown,
cancelled and exhausted work never supplies an invented chess result.

Timed live play and timed/Full live restoration are explicitly unsupported.
Evaluation-based resignation/draw policy, branching/variation editing and crash
recovery are not supplied. Current resource ceilings are 256 observations,
4,096 game/match events, 8,192 controls and 16 MiB, with reserved runtime and
shutdown headroom; these are enforced by admission, not by dropping old data.

Evidence establishes the recorded Windows Server 2022/WebView2 environment,
Ubuntu X11/Xvfb with the documented compositing override, and macOS 27.0.1 arm64.
It does not establish every OS/compositor, screen-reader audio or formal
accessibility certification. Final Windows Quit uses the native mouse menu;
the cause of an earlier automated Ctrl-Q failure is unestablished and keyboard
Quit success is not claimed. Development packages have no promised release
signing/notarization or supported-minimum-OS policy; MSH-58 owns that later work.

See [desktop launch and build instructions](../../desktop/README.md). Build with
the pinned Node 24 and desktop Rust 1.90 toolchain, then launch the native `.app`,
installed Debian executable or NSIS-installed Start menu application. Embedded
production assets require no development server. The synthetic fixture is only
in the separately labeled verification package and claims no real-game support.

## Milestone boundary and next planning

The MSH-40 roadmap correction remains intact. M4 is not complete merely because
MSH-26 was delivered early. MSH-41–45 own complete shogi, tori shogi, chu shogi,
tenjiku shogi and xiangqi deliveries; MSH-46 owns their shared six-game consumer
integration; MSH-47 owns independent M4 closure. They remain future **To Do**.
Next work is to refine the M4 scopes and their unresolved sourced-rule questions
against the completed desktop/core boundary before selecting implementation.
M5/M6 tasks MSH-48–60 also remain future To Do. No variant, strength, learning
or release-policy implementation is part of M3 closure.
