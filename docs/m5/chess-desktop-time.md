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
commands, actual outcomes, review and platform/package receipts. The exact tested source is `fac34c9f0ab0618b67cb76fde6703eff6f93e158`.
Actual native CI passed on Ubuntu 24.04, macOS 15 and Windows 2022 in
[run42613601864](https://github.com/jweathers777/mShogi/actions/runs/42613601864),
including all three release desktop-play journeys. The normal packaged macOS
application passed actual play/cancel/save/reopen/Resume/Quit with prior-file
integrity and no remaining bundled host/child observed after Quit.
Independent final receipt reconciliation approved all three acceptance criteria
and the applicable completion checklist. MSH-138 (Desktop Search Limits) is Done
with normal completion publication on `codex/rewrite`.

Owner revision, 2026-11-09: [MSH-139 (Chess Strength Assessment)](chess-strength-assessment.md)
now measures current strength without a predetermined target. Its findings need
John's review before a next objective or MSH-50 (Conventional Shogi)/MSH-54
(Conventional Xiangqi) transfer decision. No assessment matches form part of
this completed MSH-138 (Desktop Search Limits) evidence.

The earlier local package smoke failure is preserved: its external verifier still
expected the material-only opening `a2a3/+0`. The independently specified current
classical fixture is `b1c3/+61`, depth1/21 visits. The narrow verifier update and
PV check were independently approved; no production behavior changed in that
correction. Two external GUI observer mistakes were excluded and retried with
fresh full accessibility state. The successful journey, not those attempts,
supplies cancellation/shutdown evidence.

Required native receipts, run42613601864 attempt1:

| Platform | Job | Outcome |
| --- | --- | --- |
| Ubuntu 24.04 | 129147893842 | Passed |
| macOS 15 | 129147893398 | Passed |
| Windows 2022 | 129147893993 | Passed |

Workspace totals are1,171/1,171/1,168 passes respectively, zero failures and11
intentional ignores each. Each platform also passed all four release controller
journeys and subsequent required native checks. No qualification matches ran.
