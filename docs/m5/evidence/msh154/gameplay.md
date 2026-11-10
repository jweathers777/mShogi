# Independent gameplay audit — historical complete games

Bias firewall: the independent full-game opinions and planned selection were frozen in /tmp/msh-adversarial-gameplay-initial.md before usable reference queries. No engine source, roadmap, prior diagnosis/observations/analysis reports were read. Only raw pair configs, accepted match records and UCI transcripts were used initially. Later architecture supplied current replay moves only after the initial pass was frozen.

## Scope and denominator

All 12 completed games from docs/m5/evidence/msh151/pair-000-discovery-oxidation060 through pair-005-confirmation-goldfish113 were reconstructed from standard start, including their complete 16-ply book histories. Every move of all 12 games was replayed legally in independently installed python-chess1.11.2. There are 605 post-book mShogi decisions, six games each colour, six each opponent, five wins/five losses/two draws. This is balanced within this historical cohort; it is neither a random chess population nor evidence of current failure prevalence.

The mShogi executable identity in these raw records is source 639b6465be384330c9fd7d49580c13bb0c314bf2, SHA256 833b3ac570e51da47560875ab55dadd37292398555303cabd2dc6ee0229147de. Current audit revision d53f344 was not the player in those games. Every side received 1000ms movetime, no chess-clock time control, macOS/aarch64 release. Opponents were Oxidation0.6.0 and Goldfish1.13.0 with their version identities recorded in the pair configs. This audit makes no new Elo or cross-version strength claim. No new games were played.

## What the complete games show

| Raw pair/game | Result for mShogi | Whole-game reading and causal caution |
|---|---|---|
|000/0 White|Draw|Developed an attack against the Black king;22.Nf5 allowed ...Qg6, then queen/minor liquidation and pawn losses. Recovered enough activity for repetition. A drawn result conceals a plausible serious decision error.|
|000/1 Black|Win|Useful central play and compensation after White wins exchange; active bishops/knight and42...Bxg2+ combination, then queen/bishop conversion. Demonstrates attack/tactical capability; not proof all earlier play was optimal.|
|001/0 White|Win|Queenside space/pawn majority while Black storms kingside; liquidated to rook ending and supported connected passers.24.h4 appears purposeful. Opponent's later ...Re4 donation does not erase the deliberate earlier passer play and is not evidence of forcing perfection.|
|001/1 Black|Loss|King attack and23...Rf6 enable24.Rh4; actual line24...Rxh6 25.Bxh6 Kxh6 26.Qd2+ Kg7 27.Rxh5 loses net material. Later queen-to-pawn liquidation happened in an already difficult position; blaming only the late pawn ending misidentifies the issue.|
|002/0 White|Loss|Early rook excursion, then materially level rook/minor ending.52.Ne2 is not supported as the cause;55.Nf4 only a smaller/uncertain deterioration in probes. The h-passer and hanging knight after57.Rxc5 arise later. Exact first consequential error remains unresolved.|
|002/1 Black|Draw|Active king and h-passer in rook ending, then liquidation to bare rooks.48...Rxh2 was not refuted by probes. A visually dangerous passer is not proof of a missed win.|
|003/0 White|Loss|13.Bxg6 obtains pawns/exposes king; finite reference search prefers this over retreat and reports equality, so calling the sacrifice unsound is unsupported. Position already seriously bad before 27.e4; later queen-for-rook+bishop liquidation and dangerous d-passer are symptoms downstream. Earliest deterioration between sacrifice and move27 unresolved.|
|003/1 Black|Loss|Apparent attack/material gain after25...Rxe3 masks coordination/king-safety costs; reference already strongly disfavors Black before 35...Re4.66...Bxa4 was played in an already lost position. This is evidence against treating nominal material gain as success; not proof the late pawn race caused the loss.|
|004/0 White|Win|Handles opening sacrifice, queen trade, bishop pair, passer and material conversion over 99 moves. A positive endgame/conversion control, with no precision claim.|
|004/1 Black|Loss|Opening9...g5 itself survives reference probe; Black is already worse before21...d5. Heavy-piece invasion and king walk culminate in mate. The visually alarming pawn storm is not itself established as erroneous; first deterioration remains unresolved.|
|005/0 White|Win|Sharp queen simplification, rook ending, supported c-passer and promotion.34.Rc5 appears to spend some winning margin, while remaining favorable. A smaller quality issue differs from a losing blunder.|
|005/1 Black|Win|13...d3 central passer cramps the opponent and supports queen/bishop/rook coordination against the king. Reference strongly supports the move over the proposed human queen move. Positive control for compensation, attack and pawn threats.|

These are complete-game assessments, not a list of the largest final blunders. Tactical, strategic, pawn, compensation and endgame categories overlap. No rules violation was detected in these 12 histories; this does not establish general rules conformance.

## Reference method and limitations

Manual selection retained one primary per game, including wins/draws as controls, in /tmp/msh-gameplay-panel.json. Each contains the original full history and FEN. Twelve positions had unrestricted, played-move and human-alternative queries at identical 500k nodes, Threads1, Hash16MB, fresh ucinewgame/TT, no tablebases. Seven followup positions used2m nodes each: repeated22.Nf5,23...Rf6,34.Rc5, frozen55.Nf4/27.e4, and explicitly adaptive earlier35...Re4/9...g5. Thus16 distinct historical positions received usable exploratory reference probes, not 605-decision coverage. Every returned PV was legally replayed, and every constrained PV began with the requested root move. FEN/full-history identity is asserted in the primary script and bound-safe key recheck.

Important limitation: initial scripts used python-chess analyse merged dictionaries without preserving every per-info lowerbound/upperbound flag. Their numerical scores are exploratory reference indications; they cannot establish precise centipawn loss, exact final completed-depth comparisons or forced long continuations. Some returned PVs were only two plies. This limitation is explicitly retained rather than converting the numbers into truth. The two strongest cases receive a separate streaming, bound-preserving recheck (below). No tablebase was queried; no draw/win claim is tablebase-certified. Stockfish19 is a diagnostic reference, never a rules oracle.

Disconfirmations are central results: the initially suspicious13.Bxg6 and9...g5 survive;52.Ne2/48...Rxh2 were not refuted;27.e4,35...Re4,66...Bxa4 are unsuitable isolated first-error diagnoses because the position is already bad; several proposed human alternatives were much worse than the actual engine move. Conversely22.Nf5 and23...Rf6 remain the clearest reference-backed candidate quality failures and exist in different outcomes (draw/loss). No accurate failure-rate estimate follows from this sample.

## Excluded failed work and resource accounting

An initially authorized 20k-node whole-decision screen was launched just before the coordinator relayed a stricter methodology. Simbelmyne1.3.0 emitted an illegal-PV parse warning and then crashed while querying a terminal mate child. Only the first game checkpoint was saved. I did not inspect any score/candidate in its saved screen and excluded the entire run. Preserve /tmp/msh-gameplay-screen.py, .log and .json as failed/excluded provenance. Exact CPU was not captured because that script lacked a finally resource record. File birth/modification span is2seconds; conservatively charge 5 CPU-seconds to budget, explicitly an allowance charge rather than measured CPU.

Usable exploratory reference batch: 16.3178elapsed seconds,15.835663user+0.244219system childCPU. Followup: 34.6259elapsed,33.72875user+0.17975system childCPU. Measured reference child CPU 49.988382s plus conservative failed-run 5s =54.988382s charged before bound-safe recheck. The wrapper around the first completed batch returned1 only because macOS denied /usr/bin/time's trailing kern.clockrate sysctl; all 12 positions / 36 queries completed and the Python finally record was written. No match compute, no tablebase access, no new builds.

## Experiments that would discriminate causes

1. Replay the full-history panel on current d53f344 under matched 1s/5s conditions; retain the disconfirmed sacrifices/pawn storms and positive attack/passers controls. Improvements on two historical mistakes do not prove the whole historical game quality transferred.
2. For 22.Nf5, compare the root alternative's backed-up score and actual ...Qg6 continuation under common frontier/evaluation settings. For 23...Rf6 compare with ...Ne4 and current ...Rad8. Determine whether extra time repairs choice, whether all alternatives are searched, and whether the tactical consequences are valued consistently. A move appearing in PV is insufficient.
3. For 003/0 and003/1, use an explicitly fresh bounded phase search to locate when equality/favorable compensation becomes a reference-supported disadvantage, then test evaluation versus frontier/selectivity with identical histories and budget. Do not start from terminal lost positions or treat extra material as a winning outcome.
4. For 002/0, define the outcome as preserving the rook/minor defense against both flank passers over a legal continuation; test57.Rxc5 and preceding decisions. Keep drawn002/1 and winning004/0/005/0 as endgame controls. This audit does not justify a broad endgame-strength defect from one loss.

No product rule, source change, new strength target, tournament, learned evaluation or Shogi/Xiangqi transfer is recommended by this diagnostic report alone.

## Bound-preserving confirmation and current-version replay

After initial independence was frozen, architecture supplied full-history current d53f344 replays. At 1 second, current selected the same suspect moves22.Nf5 and23...Rf6. At 5 seconds, it changed to22.Bf5 (completed depth7, own score+20) and23...Rad8 (depth8, own score+33). The separate gameplay reference recheck tested those actual choices alongside the historical moves at common 2m-node limits. Threads1/Hash16MB, freshTT per search, full original history and exact root-FEN equality. Raw UCI and every per-info score/PV/bound flag are retained. Every recorded PV was legally replayed and constrained roots verified. Select the LAST score-bearing info without lowerbound/upperbound, not a later partial bound.

| Historical root | Choice | Last exact Stockfish19 iteration, side-to-move cp | Interpretation |
|---|---|---|---|
|000/0 White22|historical/current1s Nf5|-290, depth25|Strongly disfavored by finite reference search.|
|same|current5s Bf5|+18, depth21|Near equality, substantial improvement in reference outcome.|
|same|Rab1 / human h3|+16d26 / -13d21|Several viable-looking alternatives; no unique-best contract.|
|same|unrestricted Bf5|+24, depth20|Independent root search agrees on acceptable alternative.|
|001/1 Black23|historical/current1s Rf6|-371, depth23|Strongly disfavored by finite reference search.|
|same|current5s Rad8|-37, depth21|Near equality, substantially preferable to Rf6 in this probe.|
|same|Ne4 / unrestricted Ne4|0d30 / 0d29|Another viable alternative; no proof that0 means forced draw.|

These are finite reference values at matched node ceilings, with different completed depths; they are not exact centipawn loss, tablebase WDL or a strength rating. Raw 2m-node searches may end during a later incomplete iteration; the reported exact iteration can precede the final node count. The chosen 5s moves improve both selected cases under this independent outcome check. Two cases do not establish general improvement, nor do they prove execution cost is the only cause. They do show that these two failures are time-sensitive rather than permanently fixed by current source at 1s.

Concrete continuation differences matter. The legally replayed reference line after22.Nf5 begins ...Qg6 23.Nd4 Qxh7 24.Bxh7 Nxb2 25.Be3 Nc4; the game used24...Bxd4 instead and also collected pawns. The Bf5 line instead begins22.Bf5 Qd6 23.Re2 Nxb2 24.Rae1 h5 25.c4, retaining coordination and changing the queen liquidation. After23...Rf6, the reference uses24.Rh4 Ne4 25.Nxf5 Qxf5 26.Qxe4 Qxe4+ 27.Rxe4; the game used ...Rxh6/Bxh6/Kxh6 and later lost the h5-knight. With23...Rad8 the reference begins24.Rd4 Rf6 25.Rh4 Nf7 26.Nxf7 Qc6+ 27.Qe4 Qxe4+ 28.Rxe4 Rxf7. Superficially similar ...Rf6 appears at a different moment with a different defensive continuation. These PVs are legally established candidate continuations, not exhaustive proofs against all replies.

Artifacts: /tmp/msh-gameplay-bound-safe.json (key000), /tmp/msh-gameplay-bound-safe-key03.json, corresponding scripts/logs and *raw-uci.log files. Original panel and full games: /tmp/msh-gameplay-panel.json and /tmp/msh-gameplay-games.json. Current replay provenance belongs to /tmp/msh-adversarial-architecture-inputs/provenance.json and panel-1s/selected-5s artifacts.

One recheck failure is preserved: I initially mapped ...Ne4 to an illegal h5e4 rather than d6e4. A legal-root assertion stopped the batch before that constrained probe, after key000 and one unrestricted key001 search; no illegal move was sent. Corrected key001 was run separately. Partial batch 13.1645elapsed /12.799427user+0.206961system childCPU; corrected key001 6.8780elapsed /6.701761user+0.127335system childCPU. This failure is counted, not hidden or subtracted. All measured reference childCPU including this partial/failure is 69.823866s; plus conservatively charged failed-screen 5s gives 74.823866s of the 180s allowance. Reference processes have exited; no further probes are running.

Final independent judgment: this cohort supports concrete time-sensitive tactical/continuation failures and preserves positive evidence for compensation, central passers, attacks and several conversions. It does not support blaming every defeat on pawn storms, nominal material loss or the last endgame blunder. The most useful next work is controlled current-version continuation diagnosis with positive/disconfirmed controls and measured execution/frontier/evaluation separation, not a broad feature checklist or a new rating campaign.
