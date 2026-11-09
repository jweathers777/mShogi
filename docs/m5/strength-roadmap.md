# Conventional engines and demonstrated strength

Owner consolidation, 2026-11-09. This supersedes immediate cross-game transfer
based solely on algorithm presence, the initial Stockfish diagnostic, or an
engineering-readiness review. Completed work and original evidence remain valid
at their recorded sources; none establishes an official rating or present
playing-strength qualification.

| Stage | Outcome and gate |
| --- | --- |
| Continue conventional Chess | Research, profile and refine established search/classical evaluation as a coherent system. MSH-137 (Chess Search Refinement) is a completed engineering checkpoint, not strength qualification. Use observed important weaknesses to direct further conventional development. |
| Time-primary desktop play | MSH-138 (Desktop Search Limits): spend the allotted thinking time, retain the last completed iteration and preserve native rules/history, cancellation, resource protection, supervision and compatible documents. Only effectively unreachable technical visit/depth safeguards remain. |
| Demonstrate strong Chess | MSH-139 (Chess Strength Qualification): meaningful, independently grounded full-strength opponent evidence, adaptive paired-opening estimation, tactical/endgame checks and mistake analysis. The [target, protocol and six-hour budget proposal](chess-strength-qualification.md) awaits John's approval; no formal campaign is authorized. Passing the agreed standard is a prerequisite to transfer. Falling short directs further conventional Chess work. |
| Other conventional engines | Only after qualification, MSH-50 (Conventional Shogi) and MSH-54 (Conventional Xiangqi), then Tori/Chu/both Tenjiku profiles under their existing game tasks. Adapt techniques with native history/action/effect proofs and game-specific evaluation/guards. Do not copy Chess repetition-reset formulas into capture/drop or chase games. |
| Learned evaluation later | After the conventional engines are established, research a substantially complete Chess learned approach, then extend a validated approach to the other games. Existing learned-evaluator, opening-knowledge and M5 closure requirements remain intact. |

MSH-140 (Player Thinking Time) records future ordinary Chess time configuration:
faster responses or stronger play through longer searches. Exposed choices must
agree with bounded process-supervision deadlines and exact saved/request identities.
It does not expand the immediate one-second default assignment or implicitly add
timed execution to every other game.

## Engineering and qualification are different

Correctness tests, independently expected tactics/endgames, short profiles and
small development matches are autonomous engineering. Routine local match work
retains the **15-minute cumulative compute allowance per task**, including failed
and concurrent runs; do not reset it through task splitting. No substantial
qualification tournament, multi-hour benchmark or additional paid compute may
run without John's approval of the target, protocol and total budget.

A major strength milestone is a substantially complete conventional engine for a
game or a substantially complete new evaluation approach integrated into it.
Individual optimizations are not invitations to repeat expensive studies. Keep
progress reports concise: accomplishments, material findings, blockers and actual
owner decisions. Ordinary algorithms, test cases, representations and routine
implementation choices remain the implementer's responsibility.

The qualification uses credible independently rated, unweakened opponents from
one frozen published pool where practical. Equal thinking time is the principal
comparison, with actual hardware, threads, memory, books/tablebases and elapsed
work recorded. Adapt the next opponent to current evidence and uncertainty;
do not treat a few paired games as a definitive rating or use naive bisection.
Reserve larger fresh samples for nearby independent families. No official CCRL,
FIDE or USCF rating follows from local tests under different conditions.

## Existing evidence and infrastructure

MSH-49.2 (Stockfish Baseline) remains the accepted eight-game initial diagnostic.
Its losses, depth/throughput observations and bounded budget remain historical;
its old immediate-transfer handoff is superseded. MSH-137 (Chess Search Refinement)
retains its actual native three-platform CI, measured costs and controlled tactical
corrections. Engineering readiness does not substitute for the new strength gate.

Reuse the [native runner and experiment tools](experiments.md), its UCI adapters,
paired outcome accounting, bounded artifacts and authoritative native replay.
No second rules engine, protocol, application runtime or experimentation platform
is needed. Rules conformance remains [independently established](../testing/conformance-policy.md);
opponent agreement is not a rules oracle.

The architecture references remain primary classical [Stockfish search](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/search.cpp),
[ordering](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/movepick.cpp)
and [evaluation](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/evaluate.cpp),
plus game-specific [YaneuraOu](https://github.com/yaneurao/YaneuraOu)
and [Pikafish](https://github.com/official-pikafish/Pikafish) research.
Independently implement techniques; resolve licensing before copying source,
tables or weights. No neural model is part of the current conventional candidate.

MSH-138 (Desktop Search Limits) is complete at tested source `fac34c9`, with
actual three-platform native/release-controller CI and the focused normal macOS
package journey. Its [engineering record](chess-desktop-time.md) preserves exact
receipts and limitations. The remaining transfer gate is MSH-139 (Chess Strength
Qualification), whose target, protocol and overall budget still await owner approval.
