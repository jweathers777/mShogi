# Conventional engines and demonstrated strength

Owner revision, 2026-11-09. This supersedes immediate cross-game transfer
based solely on algorithm presence, the initial Stockfish diagnostic, or an
engineering-readiness review. Completed work and original evidence remain valid
at their recorded sources; none establishes an official rating or present
playing-strength qualification.

| Stage | Outcome and gate |
| --- | --- |
| Continue conventional Chess | Research, profile and refine established search/classical evaluation as a coherent system. MSH-137 (Chess Search Refinement) is a completed engineering checkpoint, not strength qualification. Use observed important weaknesses to direct further conventional development. |
| Time-primary desktop play | MSH-138 (Desktop Search Limits): spend the allotted thinking time, retain the last completed iteration and preserve native rules/history, cancellation, resource protection, supervision and compatible documents. Only effectively unreachable technical visit/depth safeguards remain. |
| Assess current Chess | MSH-139 (Chess Strength Assessment): authorized adaptive full-strength rated comparisons, fresh independent-family confirmation, tactical/endgame checks and mistake analysis. [Current assessment](chess-strength-assessment.md) has no target, fixed tournament size or campaign-hour allocation. Report useful evidence and stop for owner review. |
| Continue from measured weaknesses | John reviewed MSH-139 (Chess Strength Assessment) and authorized further conventional Chess engineering. MSH-141 (Chess Frontier Reliability) first explains/corrects the depth1/295113-visit incident; then address substantial endgame/evaluation and search efficiency opportunities in coherent increments. No arbitrary rating target or maturity claim. |
| Other conventional engines | MSH-50 (Conventional Shogi) and MSH-54 (Conventional Xiangqi) remain To Do pending explicit owner approval of cross-game transfer, then Tori/Chu/both Tenjiku profiles under existing tasks. Adapt techniques with native history/action/effect proofs and game-specific evaluation/guards. Do not copy Chess repetition-reset formulas into capture/drop or chase games. |
| Learned evaluation later | After the conventional engines are established, research a substantially complete Chess learned approach, then extend a validated approach to the other games. Existing learned-evaluator, opening-knowledge and M5 closure requirements remain intact. |

MSH-140 (Player Thinking Time) records future ordinary Chess time configuration:
faster responses or stronger play through longer searches. Exposed choices must
agree with bounded process-supervision deadlines and exact saved/request identities.
It does not expand the immediate one-second default assignment or implicitly add
timed execution to every other game.

## Engineering and assessment are different

Routine correctness/tactical tests, short profiles and development matches remain
autonomous; local match work ordinarily has a 15-minute cumulative allowance per
task including failures/concurrent work. The current assessment has explicit owner
authorization beyond that allowance, without an arbitrary total game/hour budget.
Use adaptive information and reasonable independent confirmation to stop, track
actual work, and escalate genuinely substantial resource needs. No paid compute.
Do not conduct substantial studies after individual optimizations.

Use one frozen published pool where practical. Equal thinking time is principal;
record hardware, threads, memory, books/tablebases and elapsed work. Spend fewer
games at obvious mismatches and more near the estimate. Report uncertainty,
family disagreement and testing-condition limitations. No official CCRL, FIDE or
USCF rating follows. The rejected [qualification proposal](chess-strength-qualification.md)
remains historical only. No production engine improvement belongs in this task.

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
receipts and limitations. MSH-139 (Chess Strength Assessment) is complete. The owner selected further conventional Chess development, beginning with MSH-141 (Chess Frontier Reliability). Reassess with fresh adaptive opponents only after meaningful engineering progress; no per-optimization tournament. Cross-game transfer and learned evaluation remain deferred.


## Current conventional development queue

MSH-141 (Chess Frontier Reliability) owns the reproduced depth-1 incident and
its general correction. MSH-142 (Classical Endgame Judgment) follows the measured
conversion/scaling weaknesses; MSH-143 (Chess Search Efficiency) follows remaining
measured native, exchange and safe transposition costs. These are authorized
engineering work, with concise per-task plans based on then-current source.
Review practical playing strength with fresh adaptive reference openings after
meaningful progress across these increments, not after each technique. Neither
task completion nor higher reported depth establishes maturity or authorizes
cross-game transfer.

John's continuation after MSH-141 (Chess Frontier Reliability) explicitly assigns
its targeted macOS verification, then MSH-142 (Classical Endgame Judgment) and
MSH-143 (Chess Search Efficiency), followed by MSH-144 (Chess Progress Assessment).
The latter reuses the established adaptive methodology with fresh paired openings;
its [bounded scope](chess-progress-assessment.md) began with a 15-minute
scouting tranche. For this explicitly requested adaptive reassessment, the
implementer selected a 30-minute aggregate operational ceiling for proportionate
fresh confirmation, including scouting, failures and cleanup. Routine engineering retains its ordinary
15-minute allowance; a substantial further campaign requires a concrete resource
decision. Review actual combined results and remaining opportunities
before deciding readiness; no per-optimization tournament or automatic transfer.
