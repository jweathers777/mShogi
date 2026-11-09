# MSH-138 (Desktop Search Limits)

Ordinary Chess play is time-primary. The current default is one second per move.
A fixed-time request searches until its cooperative deadline and returns its last
completed iteration; an interrupted later iteration cannot replace it. No completed
iteration means an honest incomplete result, with no fabricated desktop move or
score. Native root conclusions remain native conclusions.

The existing wire/archive shapes are unchanged. Ordinary play records depth 128
and 9,007,199,254,740,991 visits as **technical** guards: bounded native recursion
and the largest exactly representable JSON/JavaScript integer. Neither is intended
to preempt productive normal timed search. Using a JSON-unsafe u64 maximum would
break exact cross-boundary identities. Explicit small node requests remain
meaningful diagnostics; analysis and other-game budgets are unchanged.

Fixed `movetime` and supervised thinking allowances use the full hard duration.
Separate UCI game-clock allocation retains its own soft/hard clock reserve.
Deadline polling remains cooperative: native operations and OS scheduling can
extend wall time. The 15-second process watchdog is separate and unchanged;
bounded TT/state memory, stream caps, cancellation, replacement and cleanup remain.
Longer player-selected times belong to MSH-140 (Player Thinking Time), including
alignment with supervision before exposure.

## Documents and presentation

Both prior desktop configurations remain readable: untimed depth 2/20,000 visits,
and timed 1,000ms/depth 128/20,000 visits. They reopen stopped, preserving saved
settings, historical observation contexts and prior-file bytes. Explicit Resume
alone adopts the new defaults for future work. Full request/session/purpose/history,
selection, generation and limit identities continue to reject old results.

The ordinary timed engine panel presents thinking time, rather than enormous
implementation ceilings. Untimed other-game presentation remains unchanged.
All 16 regenerated Chess fixtures changed only `play_limits.nodes`.

## Verification checkpoint

Local checks passed: both legacy configurations with nonempty historical evidence,
last-completed cancellation, zero time and explicit low-node behavior, actual
release DesktopController→EngineBridge→native subprocess play/cancel/retry/cleanup,
UCI/session regressions, frontend tests/typecheck and workspace lint/format.
The actual standard opening exceeded 20,000 visits and stopped on its time allowance;
that is execution evidence, not a playing-strength estimate.

Required actual CI is Rust Ubuntu 24.04/macOS 15/Windows 2022, including the newly
explicit release desktop-play journey. Standalone Tauri checks and one focused
normal packaged macOS play/cancel/save/reopen journey accompany it. Reuse accepted
unchanged six-game GUI/platform evidence; no new all-six packaged campaign is
required. Never infer actual Linux/Windows GUI interaction from a macOS run.

The canonical [verification record](evidence/msh138/verification.json) pins source,
commands, actual outcomes, review and platform/package receipts. Final source and
platform/package acceptance remain pending at this checkpoint.

MSH-139 (Chess Strength Qualification) separately gates MSH-50 (Conventional Shogi)
and MSH-54 (Conventional Xiangqi). No qualification match campaign has run here.
