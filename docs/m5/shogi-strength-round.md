# Shogi strength round

MSH-187 (Shogi Strength Round) tests a coherent classical search change against task-start main `1d19a544cc3e1ca3e92a0e8f53268758967329bf`. Native rules remain the authority for moves, declarations, history and results. No neural evaluator or external rating study is part of this work.

## First set: changes and intended benefit

This section describes the rejected first candidate, preserved at `e902c31d`. The revised set below removes its quiet-check and full-exchange changes.

The private Shogi position now uses precomputed ray masks and reverse attack lookup, plus incremental maintenance of the existing Pawn-advance and hand-flexibility terms. This removes repeated ray walks and evaluation scans without changing those weights. A swap-based exchange estimate counts the captured board piece and its unpromoted value transferred to hand, promotion, exposed rays and king safety. This is still a private static approximation.

The shared core has an opt-in checking stage at the first quiescence ply. Shogi supplies quiet board checks and checking drops; native-equivalent move admission still rejects self-check and Pawn-drop mate. Captures and promotions retain their existing stage. Quiet checks bypass material-only exchange/delta filters. Capture-only cache entries cannot cut off the fresh checking stage. Other games default to the prior policy and avoid the added classification work.

Shogi desktop play and bare USI `go` now use a one-second budget with depth 64 and five-million-node safety ceilings. Timed USI requests use the same retained history-prefix session policy as the internal protocol. Explicit fixed-work requests remain fresh and deterministic. `go movetime` is bounded; full USI game clocks, pondering and tsume remain unsupported. Old saved play observations remain readable, and new-game/retirement guards prevent stale results from changing the game.

The table remains 16 MiB per player, keyed by board, hands and side, with shared generation/replacement policy. Timed game processes retain it; fixed-work probes are fresh. Native history admission and private root-history construction remain in the per-request path. The first set targets repeated node work rather than rewriting that authority boundary. Pawn value remains 100, so this set does not rescale shared margins or transfer new Chess constants.

This round does not tune evaluation weights. The MSH-186 demonstration contains too few independent opening families and no validation/held-out partition. Tuning requires a separately bounded data and evaluation set; the 488 exposed demonstration positions are not held-out evidence.

## First-set before-and-after local probes

Frozen candidate product source: `e902c31d9445`. Cloud tooling source `ed92e6d28df59bc45836f108f24a3c8d9ff47e4e` has identical native source. Actual local host: Apple M1 Pro, arm64 macOS, Rust 1.99.1 release. Each timed root is a fresh native engine request at 1000 ms, with three repeats. Request times include startup and transport.

| Root | Predecessor median depth | Candidate median depth |
| --- | ---: | ---: |
| Initial position | 14 | 15 |
| Double static rook | 16 | 16 |
| Fourth-file rook | 11 | 12 |
| Third-file rook | 11 | 12 |
| Central rook | 10 | 10 |
| Active rooks, pieces in hand | 7 | 6 |
| Promoted-piece composition | 64 | 64 |
| Rook ending | 16 | 16 |
| Minor pieces in hand | 10 | 9 |

The promoted-piece composition reaches a mate/depth cap and is not representative ordinary depth. Both engines match 12/12 short checking-mate cases across six geometries; that exposed suite is ceiling-limited. The position-only fixed-work profile is about 1.67 times as fast, with identical 1,448,000 child visits and evaluation checksum. Full-search work changes: the fixed-depth total takes 1.8376 seconds before and 2.1575 after, with 1,594,093 versus 1,884,071 nodes. Timed node throughput rises about 16.5% in aggregate; eight roots improve, while minor pieces in hand decline about 13.9%. This is not evidence of stronger play.

Decision at this checkpoint: continue to the preplanned coherent-set game test. Three ordinary roots gain one ply, while two hand-heavy roots lose one. Added checking/exchange work changes the horizon. These mixed measurements justify testing decisions in games, not a strength claim or a sequence of position-specific fixes.

## Game protocol and resource limits

Use the same frozen native Shogi profile, six original opening prefixes and both colors. Control 5 seconds plus 0.2 seconds, unchanged across the comparison. SPRT H0=0/H1=15 Elo, alpha=beta=.05, minimum 30 and maximum 600 pairs. Check complete batches in seed order and retain every launched pair. Unfinished games are unscored and stop the campaign; no cutoff becomes a draw.

Execution reuses the existing Google Cloud Batch path: at most eight `c3-standard-4` VMs in `us-central1`, Intel Sapphire Rapids, no retries. The campaign has a 60-minute wall limit from first submission through cleanup, at most 8 VM-hours and US$2 estimated compute, with the final 300 seconds reserved for cleanup. Maximum 600 pairs is a ceiling, not guaranteed completion. A resource-limited result stays inconclusive. The pilot has eight pairs; later batches have at most 24. See the [frozen cloud plan](evidence/msh187/cloud-plan.md).

## First game-test checkpoint: stopped without acceptance

The pilot completed eight pairs. The second batch completed 22 of 24 pairs and stopped after two candidate requests returned `Incomplete/Stopped` at depth zero. The failed requests had 311 and 159 known history events and budgets of 115 and 177 ms. Native replay passed, and all cloud jobs, VMs and disks were removed. The failed games remain unscored. This campaign cannot establish acceptance or a strength gain.

The two batches used 1.0630691606 VM-hours by measured VM lifecycle envelopes, with estimated compute cost US$0.2143232473. These are allocation and compute estimates, not billed charges. See the [campaign evidence](evidence/msh187/cloud-round1/).

Decision: stop this candidate comparison and inspect the full request path before another coherent round. Long histories reduce the time left for search in both players; the candidate also has more expensive first-iteration tactical work. Their separate contributions require measurement. Do not add time, invent a fallback move, or score an unfinished game to satisfy the acceptance criterion. No changed source or conditions will be pooled with this campaign.

## Revised set: speed, exact history identity and time

The bounded local ablation rejected the new quiet-check stage and full exchange ordering. On the shorter failed root, the predecessor completes depth 4 with a preloaded native game. The first candidate completes no iteration. Removing quiet checks reaches depth 1; an eight-ply tactical horizon still completes no iteration; removing quiet checks and restoring the predecessor exchange ordering reaches depth 4. On nine ordinary and hand-heavy roots, the speed-only variant has no lower nominal depth than the predecessor at these short budgets. These are development measurements, not a new strength result.

The revised set keeps faster attack lookup and incremental maintenance of unchanged evaluation terms. It restores the predecessor search policy and removes the unused experimental shared-core interface. It also replaces repeated native replay for retained-cache prefix matching with exact structural comparison of validated records. Root provenance, policies, every leading event and the prefix endpoint result must agree. Native request admission remains authoritative. Timed desktop/USI defaults remain in scope.

The prefix diagnosis measured 50.096 and 19.834 ms for the old prefix lookup at 311 and 159 events on the local Mac. This is separate from initial parsing and native reconstruction. It explains a real source of overhead in both players; it does not alone explain the separate expensive first-iteration search.

The revised source is frozen at `ef1c94a3536aeac2feaa85f1722968c9bf864c0c`. All 21 fixed-work roots match the predecessor exactly for moves, scores, principal variations, depths and nodes. Focused position tests and independent review pass. The failed first campaign is separate.

At 1000 ms over three repeats, median opening depths rise from 14/16/11/11/10 to 15/17/12/12/11. Hand-heavy roots remain at 7 and 10, and the rook ending remains at 16. The mate-cap composition remains at 64 and is not ordinary depth evidence. Fixed-depth work remains exactly 1,594,093 nodes; median time falls from 1.8376 to 1.3613 seconds (25.9% less). Aggregate timed throughput rises about 29.2%, although minor-pieces-in-hand throughput falls about 2.9%. Both candidates match all 12 exposed tactical cases. These measurements establish practical execution and depth gains, not game strength. Receipts are in [revised evidence](evidence/msh187/revised/verification.json).

An initial plan to fit this comparison into the original wall window was refused by its time-fit gate. The next resource checkpoint explicitly sets a new 60-minute window for this coherent set under the standing predecessor-test authorization, with at most six VMs. The aggregate 8-VM-hour and US$2 limits still include round 1. Each batch checks prior conservative receipts and reserves the remaining six-VM envelope. No old game is pooled or replaced. See the [revised plan](evidence/msh187/cloud-round2-plan.md), which also preserves the refused original-window proposal.

## Revised pilot: predecessor timing failures

The separate six-pair revised pilot stopped after three frozen-predecessor requests returned no completed iteration. Four pairs completed (candidate 5 wins, 0 draws, 3 losses); the other two pairs remain incomplete. The three failed games had 311, 285 and 308 known events, 114/115 ms request budgets, and zero/128/zero searched nodes. All failures belong to player 1, the frozen predecessor. They are not wins for the candidate. No strength acceptance is established.

All six VMs, the job and disks were removed. Measured VM lifecycle use was 0.2813104396 hours, with estimated compute US$0.0567144351. Both campaigns together used 1.3443796003 observed VM-hours and estimated US$0.2710376825. The conservative receipts charge 2.2220445130 VM-hours and US$0.4479819502 in aggregate. See [round 2 evidence](evidence/msh187/cloud-round2/README.md).

Decision: stop qualification at 5 s + 0.2 s. The new evidence concerns full-request timing in the predecessor, so it does not justify another candidate-only search change. Compare both engines on identical saved histories at original and safer budgets before choosing a separate control with sufficient pair and worker bounds. Preserve the native incomplete-result contract and every failed game.

## First-set verification and review

The following checks apply to the frozen first candidate. They do not establish acceptance of the revised source.

Local release checks passed 843 tests with 14 explicitly ignored. The standalone Tauri host compiles. All six games' search tests pass, including Shogi perft, randomized native action/state/hash comparisons, incremental evaluation restoration, hand-counted exchanges and quiet-check equivalence. All 18 fixed-depth Chess benchmark outputs are identical, including moves, scores, principal variations, nodes and depths. Actual release USI default play, continued history, new game and fixed probes passed with clean shutdown.

The frozen Linux build uses Rust 1.99.1, locked dependencies and portable `-C target-cpu=x86-64`. A one-move-per-color smoke run checked source provenance, native admission, replay and cleanup; its step-limited games are unscored and are not part of the competitive comparison. The initial pinned compiler image lacked Git; that failed build is preserved, and a derived local compiler image adds Git for observed build provenance.

Verification is Level 1 for private search and targeted Level 2 for desktop/USI process lifetime and persistence. Actual macOS consumer checks and Linux worker checks do not establish Windows or packaged-GUI compatibility; those remain for the applicable integration checkpoint.

Independent records cover the [core](evidence/msh187/core-review.md), [position and time integration](evidence/msh187/integration-position-review.md), and [cloud controller](evidence/msh187/cloud-review.md). The first candidate was rejected after the failed predecessor comparison. The [structured verification record](evidence/msh187/verification.json) links actual commands and receipts. Local probe receipts disclose pending cloud-tool/document changes as `harness_dirty:true`; the probe and measured native source were unchanged.
