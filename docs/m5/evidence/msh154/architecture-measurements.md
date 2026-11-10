# Architecture audit — observed diagnostic results

This supplements the frozen `/tmp/msh-adversarial-architecture.md`; it does not rewrite those initial hypotheses. No production changes, matches, or strength measurement were performed. All artifacts are in `/tmp/msh-adversarial-architecture-inputs/`.

## Provenance and resource scope

Rebuilt the unchanged production and example source at `d53f34413d808dc2b727a06a770b92f24191eabf` from the isolated `codex/chess-adversarial-assessment` checkout using `cargo build --release --locked --offline -p mshogi-search --example chess_frontier_profile --target-dir /Users/johnweathers/Development/Personal/mShogi/target`. Build completed in11.23sec; exact binary SHA256 `55b38b7411eed17038c1426630fd34a22a4f31d67fd5c3f9ce74612b5fd58ebf`. Source hashes, compiler and command are recorded in `provenance.json`. Native search was sequential, separate from opponent/reference timing. Every timed search used fresh per-invocation TT, depth64 and no visit cap. Per-input hard/soft time was1sec or5sec, both equal; outer watchdogs are in `runs.jsonl`/`run.py`.

Initial run ledger totals70.690224seconds wall /69.844209seconds child CPU, including one failed sampling attempt's completed5second search. Build and offline replay bookkeeping are separate. First macOS sampling failed inside sandbox with process-inspection error; subsequent read-only unsandboxed calls were explicitly tool-approved and succeeded. No security policy or settings changed. All search processes exited0; no watchdog hit. All harness root-FEN, immutable-input and native PV-replay assertions passed. This is Level1 diagnostic evidence, not platform/release verification.

## Reproduction across the twelve historical roots

The current12-input one-second replay preserves initial standard position plus every historical move, checking the exact final FEN. Outcomes are current-source observations, not proof that the move is optimal:

|Panel|Move|Score for root side|Completed depth|Main+q visits|
|---|---|---:|---:|---:|
|00,22.Nf5 suspect|Nf5|45|5|240382|
|01 neutral middle game|exf5|95|7|195645|
|02 opposite-flank control|h4|77|7|176159|
|03,23...Rf6 suspect|Rf6|56|7|202418|
|04 live-repetition endgame|Ne2|-19|6|92816|
|05 rook ending|Rxh2|123|7|466204|
|06 compensation control|Bxg6|111|9|173271|
|07 late liquidation|Bxa4|-703|7|299626|
|08 bishop coordination|Ne2|275|8|239315|
|09 central-break history|b5|-1|6|209020|
|10 rook coordination|Rf4|303|6|231869|
|11 central passer history|Nd5|74|6|221592|

At5sec, panel00 changes to Bf5(+20,depth7,1222251visits), panel03 to Rad8(+33,depth8,1073984), while control06 keeps Bxg6(+108,depth10,759218). A changed choice is not demonstrated improvement. External reference scores were still provisional because final bound flags were being recovered; do not assign exact centipawn losses from them.

Every current panel root has76.6–94.8% q visits. Quiet projections alone range173786–791608 per second, against92816–466204 counted visits. Main exact-context TT hits are8.4–27.8% of actual probes; bound cutoffs4.0–8.2%. The first actually searched move causes89.8–95.7% of main move-loop cutoffs. This does **not** support diagnosing generally failed ordering from the source's coarse continuation-history index alone. The statistic excludes TT/null/terminal exits and does not prove the best quiet move was retained. Rook-only panel05 has zero null cutoffs and zero LMR reductions, yet highest visits/second; low pruning and expensive execution must be distinguished. Incomplete final iterations consume10.4–50.0% of1second budgets; work in a PV is not automatically work in the returned choice. Raw observations plus counter parsing are in `panel-summary.json`.

## Historical repetition: concrete cost defect, narrow prevalence

Offline independent python-chess1.11.2 replay counted legal-EP repetition identities for all605 mShogi decisions across12 full histories. The reversible summary resets after a pawn move, capture or castling-right loss, matching the native key's irreversible definition; full historical occurrence counts are retained. Global repeated identity applies57/605 decisions;13 have a still-reachable repeated identity;44 have **only obsolete repeated identities**. Those44 occur in2games:20 decisions from pair002/game0 starting ply106,24 from pair004/game0 starting ply150. The latter is a win. None of the12 panel roots has obsolete-only repetition; panel04 has a live repeat. Therefore this cliff is not a general explanation for the panel's decision weaknesses. Search-internal occurrence prevalence was not measured.

Synthetic controlled fixed-depth2 runs, three alternating repetitions, no node cap, gave exactly equal nodes/move/score across each valid comparison:

- Repeated knight cycle followed by e4: complete-history149visits,Nc6,-19,median1.714ms; exact-metadata irreversible suffix149visits,Nc6,-19,0.435ms. Prepared moves0versus154.
- Same with retained reversible Nc6,Nf3,Nf6 suffix:148visits,Nc3,-11;1.997ms versus0.553ms. Prepared moves0versus119.
- Nonrepeating Nf3,Nf6,e4 full history, exact suffix and one-step suffix:104visits,Nxe4,+88;0.358/0.322/0.327ms. Prepared moves95 in each.

Three alternating1second repetitions give median42688versus218519visits for the obsolete-prefix pair,37866versus210402 with retained reversible suffix. Negative controls give210584/213128/213398visits with the same depth9,Nxe4,+63. Shallow millisecond timings alone are noisy, but fixed work, gate counters and1second results agree on this mechanism. These are query-path controls, not deployable FEN truncation.

The semantic positive control is essential: after a live repeated cycle plus Nf3,Nf6,Ng1, native full history chooses the intended Ng8 threefold claim immediately, while an identical fresh FEN chooses Nc6,+61. A general history reset would change required behavior.

A real affected root (pair002/game0,ply112) preserves the exact last irreversible state at ply110 plus Rc8,Kd7. Complete history:98376visits,depth6,+46,Rxc5 in1000.291ms. Semantically equivalent suffix:423907visits,depth8,-39,Rxc5 in1000.210ms. This is one exploratory pair, not a repeated estimate, and the selected move is unchanged. A rules-owned reversible-summary optimization is justified for investigation; a strength claim is not.

## Sampling: actual costs vary by position and history

Three separate5second searches were sampled for3seconds at1ms; these are not timing comparison runs. The values below are **exclusive top-of-stack sample percentages**, not additive inclusive call-tree totals or precision timing. Native inlining, sampling bias and one trial per root limit attribution. Full call graphs retained.

|Exclusive function/family|Panel00 queen/tactical (2233samples)|Panel04 live repetition (2342)|Panel05 rook ending (2221)|
|---|---:|---:|---:|
|attacker_bits|14.2%|4.3%|6.3%|
|resolve|9.7%|4.9%|4.7%|
|dynamic eval body|9.5%|4.2%|9.5%|
|mach_absolute_time|6.6%|2.5%|8.1%|
|apply_resolved|5.4%|3.2%|3.5%|
|EvalState::push|3.3%|1.2%|4.6%|
|update_position_hash|3.2%|1.2%|3.3%|
|repetition_identity|3.0%|23.8%|6.6%|
|validate_structure|0.5%|17.2%|1.0%|
|claims_with_actions body|below5samples|9.1%|0.9%|

The live-repeat profile demonstrates that **necessary** history semantics can trigger expensive intended-claim validation too, not only the obsolete-repeat gate. Current validation repeatedly packs identities, fully validates structures and builds public Undo snapshots for already generated legal moves. Improvements should reuse native checked effects and exact occurrence lookup rather than drop claims. Queen/tactical and rook roots show distributed native geometry/transition/eval/hash/clock costs; no single evaluator or board-representation bottleneck explains all roots. The full-board hash/eval deltas have measurable self time, supporting a bounded changed-cell token investigation if pursued. Clock reading is nontrivial, but coarser polling must retain bounded cancellation and is not automatically the highest-value work.

## What counts as work, and why opponent NPS cannot normalize it

mShogi `Work::visit` (`chess_conventional.rs:408`) counts entered main/q calls. It excludes candidate generation/validation, lazy legal witnesses, preview construction/pruning, intended-claim catalog/validation, static feature construction, contact-mate/reply probes, each recursive SEE exchange, quiet-q projections, TT/key work, sorting and PV allocation. It includes revisits caused by aspiration/PVS/reduction re-search and hypothetical null branches. `see_probes` counts initial tactical ordering estimates, **not every recursive exchange step**; `q_quiet_projections` counts only quiet admission projections. Neither plus visits is a total primitive-operation count.

Local Goldfish1.13 source (`/private/tmp/msh139/goldfish113/src/search.cpp:55,301–351,589`) increments total_nodes at root, q entry, and certain main/TT paths; main abort/history-draw exits precede increment, whereas q increments before its abort/history checks. Its legal generation, make/unmake, evaluation and SEE are also uncounted primitives. It admits different frontier semantics, hash reuse and history handling. `position.cpp:192–213,265–364` maintains material/board hash directly at changed cells and saves compact prior metadata. These source facts provide architecture alternatives, **not evidence that Goldfish's history approximation is acceptable for mShogi**, nor a measured speed ratio. No opponent timing benchmark was added. Own native visit throughput and work composition are measured here; cross-engine normalized NPS or strength ratios cannot be inferred.

## Revised priorities from this evidence

1. Retain the two reproduced decision cases and a successful compensation control for bound-safe continuation analysis. Distinguish static valuation, q horizon and selective fail-low before choosing a search correction. Depth increase changed both suspects, but did not prove correction.
2. Native query/history efficiency is independently supported: obsolete repeated-history gating is a specific narrow defect; full intended-claim admission is expensive even with live repetitions. Preserve current/intended claims and full replay, optimize within core.
3. For general execution, prototype bounded shared native transition effects/change lists only if a coherent task's cost target is tied to these measured families. Already-existing bitboards rule out a simplistic “port to bitboards” recommendation.
4. TT capacity/history restrictions and frontier/quiet work remain credible interacting hypotheses. Current ordering first-cutoff results discourage broad heuristic tuning without discriminating outcome evidence. No broad TT sweep, architecture rewrite, learned evaluator, new matches or formal strength study was run.
