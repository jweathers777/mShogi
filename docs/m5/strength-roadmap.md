# Conventional engines first

Owner direction, 2026-11-08. This supersedes the initial MSH-49 weak-baseline /
depth-2 Stockfish acceptance study and its pending four-hour budget request.
That study is stopped, with no formal held-out roots or games generated. Keep
its code, baseline, development reports, frozen protocol and actual runtime as
historical evidence; they establish no release-strength acceptance.

## Engineering sequence

| Stage | Delivery and dependency |
| --- | --- |
| 1. Chess search foundation | Build on the current native engine: efficient make/unmake and incremental state; history-safe transposition bounds; iterative deepening, principal-variation search, aspiration re-search, staged move ordering, SEE/history heuristics and bounded quiescence with legal check evasions. Retain an exact diagnostic mode. |
| 2. Complete conventional Chess | Integrate guarded null-move pruning, late-move reductions/re-search, conservative futility/SEE pruning and bounded extensions. Assess ProbCut and singular extensions in that architecture. Complete tapered material/piece placement, pawn structure, mobility, king safety, threats and endgame evaluation, plus reliable thinking-time allocation and real product execution. These are coherent engineering increments within the conventional implementation task, not statistical milestones per technique. |
| 3. Initial Chess diagnostic baseline | Owner-authorized MSH-49.2: 8–12 games from paired openings, equal one-second thinking time against full-strength Stockfish 19, at most30 minutes total match work. Report results, depths, performance limitations and representative losses. No significance study, confidence interval or Elo estimate. This diagnostic is complete and remains historical evidence. It does not establish classical-engine maturity; continue the engineering stage below before cross-game transfer. No repeated expensive Chess studies. |
| 4. Other conventional engines | Adapt to Shogi and Xiangqi first, then Tori, Chu and both accepted Tenjiku profiles. Reuse proven techniques where their semantics apply, with game-specific evaluation, tactical ordering, quiescence, pruning guards and history identities. Each substantially complete engine can justify its own approved major evaluation; an unavailable compatible opponent remains an honest limitation. |
| 5. Chess learned evaluation | After the six conventional engines are established, research and integrate a complete learned approach in Chess, including data provenance, training, export, verified native inference and product integration. NNUE-style incremental evaluation is a researched starting candidate, not a predetermined shared model. Approve substantial training resources and the later strength study before running them. |
| 6. Extend validated learning | Adapt the validated approach to the other five games, preserving both Tenjiku profiles. Refine actual model/data requirements game by game. This is tracked follow-on work; it does not silently add all-game learned delivery to the existing M5 closure gate. |

The Chess implementation (MSH-49.1) and initial diagnostic (MSH-49.2) are two bounded
task records under MSH-49. MSH-135 tracks the later learned extensions. Existing MSH-50/54 own Shogi/Xiangqi, MSH-51/52/53 the historical variants,
and MSH-55 the initial Chess learned evaluator. MSH-104's distinct all-game
opening-knowledge research follows the conventional phase and initial Chess
learning result; it remains a separate experiment, not a shared-model dependency.
MSH-56 still requires all six strength deliveries, initial learned improvement
and the opening-knowledge experiment. No completed M4/M5 prerequisite is reopened.

Owner correction after MSH-136: conventional Chess is **not complete** because
major algorithms are present. Preserve completed MSH-49/49.1/49.2/136 and all
receipts. MSH-137 continues coherent practical Chess development before MSH-50
Shogi and MSH-54 Xiangqi; learned evaluation stays later. Its bounded increments
cover native overhead (137.1), collision-checked future-state TT reuse (137.2),
and integrated search/classical-evaluation refinement (137.3). These are engineering
checkpoints, not statistical studies. Reassess significant remaining opportunities
from current evidence; add justified work rather than treating this list as a
completion checklist. Transfer only after independent engineering assessment finds
remaining opportunities predominantly diminishing-return refinements, speculative
research or disproportionate effort. Routine research, algorithms, tests and
profiling need no owner approval; escalate actual major architectural/resource
tradeoffs. No new match study is planned. The existing 15-minute aggregate routine
match allowance applies across the coherent work, without subtask resets.

## Verification and measurements

During engineering, use independently expected search/tactical/endgame tests,
state/hash/evaluation restoration, history/claim/terminal regressions, selective
re-search tests, cancellation checks, short profiling and small development
matches. Exact-mode minimax tests do not prove selective search is exact, and
neither tactical tests nor throughput establish playing strength. Use independent
review and proportionate actual platform checks at coherent source checkpoints.

Routine local engine-match experiments are limited to **15 minutes cumulative
compute per task** unless John explicitly authorizes more. Record aggregate
active invocation runtime, including failed attempts and concurrent runs; do not
reset the allowance by relabeling matches or artificially splitting tasks.
Do not run multi-hour benchmarks, substantial statistical campaigns or formal
strength studies without explicit owner approval. A major milestone is a
substantially complete conventional engine for a game, or a substantially
complete new evaluation approach integrated into it. An optimization or small
group of techniques is not such a milestone. Ordinary correctness tests are
engineering verification, not grounds for another expensive match campaign.

## Selected references and technical boundaries

The search/evaluation selections above are engineering recommendations informed
by the [Stockfish 11 classical evaluator](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/evaluate.cpp),
[search](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/search.cpp)
and [move ordering](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/movepick.cpp).
Current Stockfish uses learned evaluation; its historical classical code supplies
architecture references, not promised mShogi strength or weights to transplant.
[Official UCI options](https://official-stockfish.github.io/docs/stockfish-wiki/UCI-Protocol-and-Stockfish-Commands.html)
define clock/resource controls and explicit strength limiting for the later
full-strength opponent setup.

[YaneuraOu](https://github.com/yaneurao/YaneuraOu/blob/master/source/engine/yaneuraou-engine/yaneuraou-search.cpp)
and [Pikafish](https://github.com/official-pikafish/Pikafish/blob/master/src/position.cpp)
provide concrete Shogi search/history and Xiangqi adjudication references.
Their profiles are not rules oracles for our accepted native profiles. Board-only
TT reuse cannot erase claims, counters, repetition or whole-profile identity.
Quiescence and SEE must respect drops, compound routes, royal survival and fire
effects where applicable; a search-only null branch never becomes played history.
Research and independently implement algorithms. Do not copy upstream code,
tables or weights without resolving the applicable source-license requirements.

## Decisions reserved for John

Routine algorithm choices, representations, guards, test cases and coherent
implementation sequencing belong to the implementer. Bring back only material
choices: the actual major-study resource/time budget and comparison/release
criteria when the engine is ready; substantial training/data/hardware commitments;
or an actual conflict in product/rules requirements. No new decision is needed
to begin conventional engineering under this direction.

MSH-138 follows the refined-engine checkpoint before cross-game transfer: calibrate
ordinary desktop Chess play limits within existing admission ceilings. Faster
fixed-visit execution may stop before the hard time limit; timed UCI capability
does not by itself establish ordinary GUI play strength. This is product-default
work, with no new Chess opponent study or unresolved search-architecture claim.

MSH-137 and its three increments are now complete: independently reviewed native
cost, history-safe reuse and coherent tactical/classical refinements, with actual
three-platform native CI at `2a2ff7c`. The [refinement record](chess-classical-refinement.md)
states measured gains, historical failures and remaining limits. This supports
practical engineering readiness, not a strength claim. MSH-138 is next;
MSH-50/54 remain dependent on its desktop-default calibration.
