# MSH-113 verification evidence

This record is in progress. No acceptance or platform success is inferred from
implementation alone. The primary integrator works on `codex/rewrite` from
`8f21eb212d1847d75a7d8580882ae4afeacf193b`.

## Ownership and review method

Root Codex owns frontend, host integration, documentation, verification and Git.
An isolated native builder owns native session/core changes. A separate reviewer
read the completed frontend boundary, owner specification and existing lifecycle
and ran focused tests. That reviewer identified a P2 error-propagation finding:
product Chess Take back used shared dispatch but its refusal error did not reach
the legacy Chess notice. The correction routes shared errors to visible existing
notices and adds stale/capacity refusal regressions; the corrected frontend was independently approved (59 tests/5 files and TypeScript).
Packaged evidence and final CI will be recorded before finalization.

The pre-existing user app was saved through the native Save As dialog at
`/private/tmp/msh113-preserved-preverification.mshogi` before verification.

## Review findings addressed during implementation

- Native retained Chess checkpoints originally bypassed the live record's
  untimed/witness restoration restrictions. They now enforce the same root and
  restoration restrictions before replay and bind to the native control timestamp.
- An unavailable Shogi Take back could retire pending initial engine work before
  asynchronous rejection. Synchronous eligibility now preserves the complete
  accepted context and pending opening on refusal, matching Chess and Xiangqi.
- The Chess lineage replay budget needs mandatory stop/close headroom in addition
  to encoded byte/control reserves. The independent reviewer requested an exact
  budget-boundary regression; the final exact-boundary regression passed independently.

Intermediate checks: full frontend207 tests/24 files passed. An early workspace
run passed587 tests (2 intentionally ignored) before the final review corrections.
An overlapping workspace clippy run encountered a temporary type mismatch in the
native test file while its builder was still adding cases; this is not final
verification. Final source checks and exact revisions will supersede these runs.

## Completed independent review

Native/archive reviewer: independent read-only diff/source audit against8f21eb2,
not the native builder. After the findings above were resolved, independently ran
`cargo test --locked -p mshogi-desktop-session takeback -- --nocapture`:
**12 passed,0failed,123filtered,4.74s**. The final efficient replay-limit fixture
contains1320 genuine Observe entries,123 admitted Preferences and128controls;
333246encoded bytes, actual mandatory close and strict Open passed. The same
production boundary had also passed the older equivalent slower fixture
(12tests/117.04s); this is superseded by the efficient test result. No remaining
material frontend or native/archive review finding. Actual GUI and final CI are
separate obligations.

## Packaged interaction finding

The first reviewed implementation package (`ff0b1ce`) genuinely played Chess
`e2e4` / bundled `a7a5`, took both back to0plies, paused and retained its
observation. The first capture `normal-chess-restored.png` exposed a visibility
finding: the sidebar remained scrolled down to Game Actions, clipping the
existing Resume notice above the viewport. It is intermediate finding evidence,
not final visual acceptance. The tiny shared-control correction preserves status
focus and reveals the existing paused notice with nearest scrolling, with no
layout redesign. A fresh independent source review approved it after4 focused
control tests; the final package must establish visible Resume.

The same first package saved `/private/tmp/msh113-chess-taken-back.mshogi` through
native Save As (`intermediate-chess-v3-saved`); complete native v3 semantics are
unchanged by this presentation-only correction.
