# Proposed unrestricted Chess comparison (MSH-49.2)

The conventional engine is complete at MSH-49.1. This is the next major
measurement checkpoint, using the existing native UCI match runner. Formal
execution is **not approved or started**. The earlier depth-2 Stockfish study
remains stopped; its thresholds and budget are inactive.

## Concrete proposal for John

- **32 games:** 16 independently sampled opening histories, each played with
  mShogi White and Black. Use the existing six-family native sampler, with
  replacement and the fixed old development roots excluded. Freeze protocol,
  code and builds before drawing one fresh seed. Retain duplicates and every
  attempt. This measures that stated opening population, not every Chess position.
- **Equal 500 ms per move:** both receive only `go movetime 500`. No external
  depth/node limit. Each retains its own time-management/early-finish behavior;
  equal allowance does not promise identical actual CPU use or search depth.
- **Unrestricted Stockfish 19:** full default neural evaluator, skill 20,
  strength limiting off, one search thread, pondering off, no external opening
  book or tablebases for either participant. Pin executable, exported default
  network and every option. Preserve mShogi's intrinsic search bounds.
- **Same local machine:** Apple M1 Pro, 32 GiB, macOS 27; sequential games.
  Stockfish uses its modest default 16 MB Hash, while mShogi retains its
  4,096-entry native TT. Cache bytes and evaluator memory are explicitly unequal;
  this compares the implementations under the same thinking-time allowance,
  with one search thread each and ample system RAM.
- **Up to one hour of active local match invocation time**, including smoke,
  failures and cleanup. Stop starting pairs with under ten minutes left; reserve
  five seconds for bounded outer cleanup. Stop on an invalid/incomplete pair,
  with no silent replacement or budget extension. A game reaching 512 total
  plies remains unscored. No stopping based on a favorable or unfavorable score.
- **Cost:** local CPU time and electricity only; no paid cloud compute, vendor
  payment or new software purchase. One hour is the ceiling, not a promise to
  use it all. Builds, replay and review are separate engineering work.
- **Interpretation:** accept a complete, reproducible and independently reviewed
  measurement report regardless of the score. Report wins/draws/losses, all
  failures and a broad 95% interval using the 16 paired scores. The interval
  assumes independent sampled roots and stable conditions. No parity, human Elo
  or numerical release-strength threshold is proposed. A decisive defeat stays
  an honest finding for subsequent engineering.

This budget and interpretation are the material owner decision required by
AGENTS.md and MSH-49.2 AC2. Routine technical preparation is already authorized.
A partial, failed or budget-capped campaign does not complete the task.

## Prepared evidence

The [protocol](evidence/msh49-2/protocol.json),
[build pins](evidence/msh49-2/build.json) and
[verification/accounting record](evidence/msh49-2/verification.json) are the
canonical preparation. One deliberately four-ply, two-game transport smoke took
5.8404 seconds, with two unscored caps, clean process completion and successful
native replay. It is not a strength result. Offline controls test refusal before
approval, exact time/options/artifact identities, budget accounting and bounded
interruption cleanup. Production code is unchanged; prior native platform
acceptance remains at `b8cc851`.

The task-specific [launcher](evidence/msh49-2/study.py) uses the existing runner;
it does not add a product protocol, runtime, rules implementation or shipped
Python dependency. It refuses formal execution without recorded owner approval
of this exact protocol, clean committed static study inputs and unchanged frozen
production source. It freezes evidence before generating a seed, runs each pair
once, retains reports and validates them with native replay. Independent review
must reconcile the actual final experiment before acceptance.

Official sources: [Stockfish 19 release](https://github.com/official-stockfish/Stockfish/releases/tag/sf_19)
and [Stockfish UCI controls](https://official-stockfish.github.io/docs/stockfish-wiki/UCI-Protocol-and-Stockfish-Commands.html).
