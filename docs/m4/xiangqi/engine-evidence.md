# MSH-97 functional evidence

Local environment: macOS27.0.1 (26A434), arm64 Apple M1 Pro, workspace
Rust1.89.0, release optimized baseline,2026-10-28. Benchmark construction is
committed in `crates/mshogi-search/examples/xiangqi_baseline.rs`; canonical
known-history reconstruction is included in measured search time. Single
observations below are not distributions or guaranteed latency. No strength
claim or comparator evaluation was used.

| Case | Local choices | Default1/128: visits/depth/ms | Ceiling3/512: visits/completed depth/ms |
| --- | ---: | --- | --- |
| Standard substantial branching |44 |45/1/23.879 |512/1/305.509 (node limit) |
| Open rook lines |18 |19/1/0.286 |384/3/5.194 |
| Cannon screen/capture |18 |19/1/0.982 |512/2/31.749 (node limit) |
| Horse leg/check constraint |3 |4/1/0.176 |325/3/12.180 |
| Check evasions |1 |2/1/0.072 |73/3/1.450 |
| Repetition warning,10 known plies |18 |19/1/0.894 |512/2/11.809 (node limit) |
| General+Advisor exact-negative domain |2 |3/1/0.075 |25/3/0.299 |

Explicit required root exact: General+Advisor closes255states/674edges,
105152owned bytes, returning unscored read-only proof in0.662ms including
request/accept. Limits1000states/200000edges/5000ms/4MiB/4096witness/DFS.
Standard with0states returns Unknown(States), no choice/score in0.011ms.
Native exact defaults/ceilings are unchanged; this measurement uses explicit
5s limits to avoid treating a timed cutoff as deterministic proof evidence.
Independently rerun release observations by the reviewer were about30ms/301ms
for standard default/max and0.7ms for255-state exact closure. Timing variation
is expected. Defaults1/128 and maximum3/512 are deliberately conservative for
this unoptimized full native-history baseline, not copied Chess/Shogi limits.

## Independently expected fixtures

Separate fixture author `console96_tests` established arithmetic/native facts
in `crates/mshogi-search/tests/xiangqi.rs`, independently from the product
builder. Expectations derive from WXF geometry and previously reviewed
MSH-91/94/105 fixtures, not Pikafish scores/adjudication or search output.

- Starting material4800 each; named piece values and mirrored Soldier advance.
- Both-color Cannon capture through one screen gives590 remaining material;
  Horse leg tactic400; immediate rook stalemate gives native win distance1.
- C11 history changes Black's best action: known warning history `e8f8` awards
  the native repetition-fault win999999; same imported board `e8e9` captures
  the undefended rook and establishes the native bare-General draw0. The
  capture-to-draw witness is independently asserted by native submission.
- C10 complementary depth2 fixture avoids a locally legal continuation whose
  fourth cycle loses for the offender. This fixture alone is not claimed as
  proof of a changed best action; the C11 comparison supplies that evidence.
- Frozen NATURAL-11-CHECKS has Red99/Black100 effective plies at100; explicit
  policy does not claim for a nonmoving opponent. At101 Black's valid count
  claim is the first equal-valued0 choice. Automatic policy concludes natively.
- Native non-turn resignation winner and scoped no-win draw, complete record
  restoration after success/depth/nodes/cancel; no completed work means no
  fallback score or move.511-audit capacity failure is unscored/saveable.
- General+Advisor255-state exact closure remains read-only; a two-ply natural
  witness is not a forced-win score;0-state Unknown, cancellation and verifier
  failure retain their distinct carriers and unchanged source.

One initial fixture-author expectation incorrectly valued fresh C11 as-900.
Explicit geometry showed the Black General can capture the undefended rook;
that native capture establishes bare-General draw0. The test was corrected
with its independent capture witness; product rules and frozen fixtures were
unchanged. No expected WXF outcome was changed to match an implementation.

A separate isolated test author `engine97_tests` adapted the earlier routing
and handoff tests to real execution, preserving strict/capacity/history and
supervision coverage. Actual child fixtures exercise play, read-only analysis,
zero-work incomplete, same-board/different-history, native same-ply resignation,
unsupported Chess Full, wrong fields/telemetry and complete echo binding.
The integrator additionally exercised cancellation and replacement through the
normal executable, alongside the separately labeled sleeping/forged child
fixtures used for deterministic watchdog/cleanup/stale tests. Test-helper
binaries remain outside normal desktop packages. No GUI interaction is claimed
as newly verified by native/controller tests.

Final independent reviewer is the separate `console96_review` session, source
and test review plus its own executed checks/measurements, not author rereading.
Its final report and any resolved findings are recorded in
[the review](engine-reviews/msh-97.md) and MSH-97. Exact command results, source
integration and relevant native/desktop CI are recorded at completion. Earlier
M1/M2/M3 and accepted Chess/Shogi evidence remain valid for unchanged scope.
