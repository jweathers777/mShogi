# Fresh native archive/session and desktop artifact assessment

Reviewer: `/root/review103_archive`; revision541fc303,2026-10-29. Fresh read-only
source inspection and actual execution; no files/task records changed.

```sh
cargo test --locked -p mshogi-desktop-session shared::xiangqi::archive::tests -- --test-threads=1
# PASS20,112.42s
cargo test --locked -p mshogi-desktop-session takeback -- --test-threads=1
# PASS12,22.91s
cargo test --locked -p mshogi-desktop-session shared::xiangqi::tests -- --test-threads=1
# PASS15,70.38s (includes nested handoff/label tests)
```

Verified actual RawValue dispatch through shared→Xiangqi archive→native strict
replay; loose resource probes never replace original authoritative bytes.
Replay validates full command/result/effect/event/warning/conclusion/current
state, journal ordering, exact-prefix observations and execution binding.
Xiangqi desktop schema2 retains root/provenance, claimant/audits, complete native
events, settings, ordinary and exact observations. Positive exact witnesses
undergo bounded replay; negative/Unknown/cancel/failure remain historical,
never resumed work or a reopened game conclusion.

Candidate admission charges complete actual bytes, native events/commands,
controls, weighted reconstruction/variation/exact work,256 combined observations
and real stop reserve. Failed/interrupted Save/Open retain valid file,dirty
state and live document. Fresh stopped reopen retires old work above both runtime
floors and requires Resume. Take back uses native committed actor/command
boundaries, preserves engine openings and same-ply warnings/claims, retires
execution and retains abandoned historical evidence.

Actual source/artifacts inspected: `XiangqiApp`, `XiangqiBoard`,
`ApplicationShell`, `NativeSessionPanels`, `ProductApp`, `useGameSession`,
`Dialog`, `TakeBackControl`; retained PNG/AX evidence101minimum,102historical
analysis/failure/capacity,113restored-opening/stopped-reopen/keyboard.
The complete intersection board/palaces/river/player areas/footer fit the
qualified minimum package. Event7 read-only Black analysis and Return to live
are clear. Genuine256 capacity state stays saveable. Restored Black one-ply
decision preserves Red opening, reveals Resume and restores status focus.
Native inspection remains non-actionable/token-bound; no frontend legality.
The accepted common shell/dark treatment remains intact.

**F01 P2 discovered:** shared `NativeRecovery` identifies any diagnostic with
`256` as observation exhaustion; native Shogi/Xiangqi count-limit diagnostics
contain both256 and8192. A zero-observation control-only refusal therefore
claims256 recorded observations. Native control-boundary tests prove the
real diagnostic; frontend observation-only tests missed it. Saveability and
outcome separation remain correct, but recovery information is inaccurate.
See [reproduction](capacity-finding.md) and corrective MSH-114. Other inspected
AC7 source/artifacts have no material contradiction identified.

Disposition: native MSH-45 AC2/6 supported; full AC7 acceptance must remain
blocked by F01. This is independent source/pixel/AX review, **not new live GUI
execution**. Actual Xiangqi GUI evidence is macOS; final Windows/Linux all-game
GUI acceptance remains88/46 and historical111 driver failure stays open.

Future106: compare duplicated Shogi/Xiangqi journal/candidate/observation/
lifecycle orchestration and board interaction focus/inspection lifetimes.
WXF warning/claim boundaries, exact validation and native records remain
intentionally distinct. No broad refactor is recommended by symmetry alone.

