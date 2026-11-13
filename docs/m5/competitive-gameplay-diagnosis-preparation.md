# MSH-161 (Competitive Gameplay Diagnosis)

Status: preparation; no gameplay conclusions. Production source frozen at
77101fe (retained MSH-158 engine following rejected MSH-159 candidate).
[Owner request](evidence/msh161/owner-request.txt) authorizes approximately200
complete games plus necessary independent analysis. This explicitly supersedes
routine15-minute and earlier2400-second allowances for this study only. No
production search/evaluation/rules changes, rating study or engineering follow-on.

## Protocol v0 — pilot, subject to documented validation

Use existing native UCI match runner and native replay. Root Codex is the sole
integrator on codex/rewrite. Independent protocol/source research precedes main
execution; a gameplay analyst receives games and reference analysis without prior
engineering explanations. Independent final review challenges conclusions.

Plan180 prepared games (90 histories, paired colors) and20 unaided start games
(ten per color). Select master-supported histories across approximately15–18
families without candidate-result selection. Audit open/closed centers,
symmetry, tension, activity, development and castling diversity. Source records,
licenses, selection seed, rejects and full prefixes remain reproducible. Screen
prepared roots independently for balance before playing. Supplied disadvantage
is separate from subsequent engine decisions. Repeated start trajectories remain
related evidence, never20 independent opening families. MSH-160 owns the opening
assessment questions within this study rather than a competing position corpus.

Qualify Goldfish1.13.0 and Oxidation0.6.0 with a small matched pilot only if needed;
prior1-second samples do not prove either stronger at5seconds. Freeze one
opponent before the main sample. Seek resistance plus real opportunities, not
all hopeless defenses. Qualification is separate from main results.

Both engines receive go movetime5000, one worker, no book/ponder/handicap or
external tablebases. Preserve existing engine memory policies explicitly. Fresh
processes per game, same host, serial competitive runs, no concurrent reference
search. Record binary/source hashes and actual response times. Native auto-claim
policy owns results. No evaluation resignation/adjudication.512 total plies is
an operational cap, never a draw. Each pair gets a6000-second wall watchdog;
per-response15seconds; bounded protocol/report output. Preserve failures, caps,
retries and cleanup. Analyze games outside competitive runs.

At100 searched plies/game,200 games imply27.8 engine-hours plus reference work.
This is an estimate, not a ceiling. Measure elapsed/CPU/search resource use and
artifact sizes per process, including failed work. Review after pilot and every
20–30 games. Complete broad coverage before prioritization; extensions must name
an unresolved coverage/uncertainty question. Never stop at the first plausible
weakness. No unreviewed entire-study batch.

## Reference method and events

Fixed Stockfish19, one thread, Hash64MiB, UCI_ShowWDL, no strength limit. Preserve
complete root histories, raw UCI, depth/nodes/time, scores/bounds and legal PVs.
Pilot initial work:500000nodes for unrestricted root and forced actual move,
independently cleared hash. Primary severity E=(W+D/2)/1000 from mover perspective;
this is Stockfish self-play expected-score proxy, not calibrated actual opponent
win probability. Centipawns/mates support interpretation separately. Keep negative
estimated loss as inconsistency, not silently clamped zero.

Provisional screening: E-loss>=.05 significant,>=.10 serious,>=.20 major.
Confirm significant events using5million-node same-root equal-work singleton
played/alternative comparisons, then20million where unstable or result-critical.
Discover alternatives unrestricted first. Preserve ambiguous judgments; bounded
scores do not count as precise estimates. Check selected legal continuations with
native authority. Exact tablebase outcomes require available licensed data or
public service with recorded provenance, halfmove and history qualifications.
No finite-engine preferred move is uniquely mandatory or game-theoretic proof.
Audit nonflagged moves and nearby threshold sensitivity before freezing method.

Per game record first confirmed significant candidate deterioration, largest
consequential mistake, linked opponent-created opportunity/missed opportunity,
recovery (opponent error then successful response), favorable-position preservation
and actual conversion episodes. Later errors in an already lost position are
separate diagnostics, not independent causes of defeat. Link correlated moves as
one episode and avoid summing duplicated category damage. Analyze both players.
Distinguish favorable proxy episodes from demonstrably winning positions.

Opening ends under a recorded development/king-disposition rule, not a fixed move
number. Pilot must settle the precise rule before ranked analysis. Preserve
boundary ambiguity and full-game classifications. Master popularity identifies
established/unusual moves only; independent continuation analysis judges quality.
Classify behavior before technical hypotheses: development, king safety, tactics,
quiet defense, coordination/activity, pawn breaks/structure, planning, exchanges,
opportunity response, preservation/conversion and endgames; allow new categories.

## Review and reporting

Freeze protocol after pilot validates game completion, time fairness, replay,
structural variety, telemetry, Stockfish stability and accepted alternatives.
Changes retain original analyses and state whether reanalysis is required.
Track category discovery/frequency/severity/family distribution per batch.
Rank affected-game prevalence, severity and consequential episodes, with both
move/game denominators, phase/family/color/result coverage and contrary successes.
Use opening-pair/family uncertainty, not independent-move confidence intervals.
Observed behavior, possible mechanism, discriminating investigation and potential
general engineering work remain separate. Existing position benchmark supports
regression context only. Update roadmap and stop for owner review before fixes.

Verification level1: offline analysis/helpers, native replay, meaningful parser/
score/event tests and independent review. No new cross-platform/GUI execution is
claimed. Broader compatibility remains deferred to an actual integration boundary.
