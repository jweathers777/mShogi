# Tori native playable-consumer checkpoint (MSH-42)

This checkpoint adds a functional computer opponent and its native consumers to
`mshogi.tori.modern.v1`. It does not complete MSH-42 or approve Tori desktop artwork.
The eight original delivery criteria remain intact. The normal player chooser,
host New Game and normal product Open gates still exclude Tori. MSH-77's accepted
shared source and the original native/profile fixture evidence remain historical
inputs; this checkpoint supersedes their explicit unsupported-engine handoff.

## Search and execution

The concrete `mshogi_search::tori` module performs deterministic iterative,
full-width minimax through the native `Game`. It has no board-only transposition
cache, pruning, learned evaluation or separate legality/adjudication model. A
private search owner reconstructs an admitted canonical record; native submit/undo
preserves full history and T06 departure obligations. Undo restores semantics while
retiring runtime tokens. Cancellation or a transition error unwinds each accepted
edge. A visit limit includes iterative revisits and discarded iterations.

The transparent `mshogi.tori.material.v1` evaluator totals current board faces and
native base pieces in hand, in hundredths of a Swallow: Sw100, Go300, Fa700,
Ea1000, Cr400, Pt300, LQ500, RQ500, Ph0. Positive means an advantage for the named
root side, Sente or Gote. Material never establishes a game outcome. Native
terminal scores use win/loss minus board-action distance, including stalemate or
repetition forfeits; they are not all checkmates. `mshogi.tori.full-width.v1`
identifies this search. There is no playing-strength claim; MSH-51 owns that work.

Default interactive native play is depth2/2,000 visits; supported maxima are
3/10,000. Local release probes completed depth2: starting position 17 choices,
324 visits, 9ms; drop-heavy position 281 choices, 1,847 visits, 71ms at
committed source `035c35c` (an earlier local probe was 62ms). These are
observations from this machine, not latency guarantees. The existing supervised
child watchdog/cleanup and independently terminable process remain authoritative.
No timed game or new search time-control protocol is introduced.

The existing execution protocol4 and observation schema1 gain an additive typed
`Tori` result: `Choice`, native `Finished`, or `Incomplete`. Failures remain a
separate unscored family. Only a completed full-width iteration can supply a
choice; a later visit/stop bound may preserve that completed iteration's actual
choice, score and PV. With no completed iteration there is no fallback or score.
Host cancellation retires the entire request, even if a child retained a usable
last-completed choice. Root-terminal results match the actual native conclusion,
including out-of-turn resignation. No CSA declaration, Chess Full or Xiangqi
exact operation is imported.

Request, process generation, session version, limits, purpose, exact root/history
and historical selection remain explicit. Bridge matching performs cheap bounded
identity/telemetry checks. Full PV and leaf-score consistency validation runs on
the existing bounded native document admission worker, not desktop polling.
Consistency validation does not establish that a heuristic choice was optimal.
Native capacity failures during search are unscored `FailureKind::Search` with
native diagnostics; typed document resource admission remains a separate owner.

## Native document and session

The one desktop controller/runtime schedules native engine turns after explicit
Resume and subsequent admitted human actions. A manual native verification move
while paused never authorizes engine work; reopening stays stopped until Resume.
Ordinary analysis is read-only and targets the exact selected event prefix. It shares the same replaceable execution resource;
starting/replacing/cancelling analysis retires prior work. Historical selection,
Return to live, Undo/Take back, replacement and close retire execution. Stopped
reopening assigns fresh runtime identities and never restarts saved work.

One immutable worker-owned candidate retains the observation **before** the exact
engine `Native Play` control, re-admits the complete action through native Game,
and checks the complete saveable document before installation. Archive validation
requires that action immediately after an engine Choice. An analysis Choice never
appends a game action. Rejected malformed or capacity-limited results preserve the
accepted document, retire consumed work and pause scheduling. Old unsupported,
unscored failure observations remain valid historical evidence, including their
previously admitted generic transport limits. Actual new Tori execution enforces
its smaller measured limits. No record, envelope, transport or observation version
is silently changed and no supported size constant is increased.

Resource checks include actual encoded envelope bytes, retained native events,
controls, observations, aggregate replay work and reserved Stop/Resign/close
headroom. Real result validation now precharges context parse, target/prefix and
validation-branch reconstruction plus complete PV transitions. The 256-observation
boundary remains a total document limit. Saving/reopening does not reset it.
Existing failure-only documents keep their existing charge. The operational15s
worker deadline and pre-existing qualification for expensive near-limit opens
remain unchanged. Measured results and focused tests are indexed in the associated
structured evidence record; they are distinct from packaged GUI evidence.

## Console and notation

Launch the native console from the integration checkout:

```sh
cargo run --release --locked --bin mshogi -- --game tori shell
```

`help` documents standard/custom canonical roots, board/hands/status, complete
legal choices, atomic `move` batches and drops, event undo/history, resignation,
administrative Stop, bounded perft/divide, read-only bounded analysis and canonical
Save/Load. Human shell text is separate from `mshogi internal-search` JSON.
[Notation](notation.md) documents native `Ph4g-4f`, `Fa7cx7b=Ea`, `RQ*5e` and
handed identities. The same Rust formatter supplies native Moves and console
history. Labels never grant permission: parsing selects a native local complete
choice and Game still admits exact history, context and terminal semantics.

`TORI-C-NOTATION.expected.display_is_not_action_authority` and its single remaining
consumer requirement are covered by independent capture/promotion/drop examples,
wrong-label rejection, and a locally parseable but T06-forbidden departure refused
by Game. The corpus and ownership inventory are unchanged. The MSH-132 audit's
then-pending notation rows remain historical; this evidence is their implemented
consumer follow-up, not another count of its713 native assertions.

Actual subprocess smoke runs each fixed scenario twice with exact trace equality:
8 standard-root actions followed by an explicit unscored WorkLimit Stop, and an
independently reviewed mate-in-one predecessor ending in native checkmate plus an
actual Finished response. This demonstrates bounded functional play, not a full
standard starting-position game, external interoperability or competitive strength.

## Verification provenance

The [structured evidence index](evidence/native-playable-checkpoint.json) records
commands, counts, capacities, review method and failures. Production source is
`d4bfeeb491e8e7f5a731b3bf64e199cc744ce0bb`; the subsequent verification revision
`e60c2aafcc8f341ec27d7ea7c195d4e1a69b7105` changes an existing Shogi retirement
test to assert the actual authoritative refusal, with no production change.
The failed macOS assertion and superseded campaigns remain recorded separately.
Independent source/test review passed; it was read-only review, not a second
execution of tests. Required native CI `41469635243` passed all three platforms at
`e60c2aa`. Unchanged-production desktop CI `41465780619` passed all three platforms
at `d4bfeeb`, including normal package/bundled-engine checks and the existing-game
Windows/Linux GUI journeys. No new macOS or Tori GUI journey is inferred.
The [CI record](evidence/native-playable-ci.json) distinguishes those revisions
and checks. The redundant test-only package campaign was cancelled deliberately.

The [actual subprocess traces](evidence/native-playable-selfplay.json) preserve
both bounded scenarios, work limits, observations and complete native records.
Normal Tori product exposure and packaged Tori GUI verification remain pending.

## Remaining finished-product work

Continue under MSH-42 with the actual Tori board/piece artwork, integrated hands,
clear compulsory promotion and handed quail presentation in the accepted shell.
Keep all7×7 squares visible at supported sizes; preserve the one native controller,
one frontend session/choice owner, native inspection, exact Moves/history and
origin/destination cues. Obtain owner visual approval of the finished presentation.
The current technical renderer and any native fixture screenshots are not that
approval. Enable normal chooser/New Game/Open exposure only together with the
finished playable UI and matching native capabilities, limits and recovery.

Then verify actual packaged human/engine play, analysis/cancel/retry, outcomes,
Save/Load, history/Return/Resume, Take back, unsaved replacement and quit. Preserve
required platform-specific evidence and shared MSH-46/47/88/89 ownership. Finish
with independent complete-game acceptance against all eight original criteria.
No final desktop artwork or subsequent variant is started by this checkpoint.
