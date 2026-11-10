# MSH-157 development diagnosis and frozen probe selection

The complete 160-root predecessor development pass finished successfully before this selection. No held-out fixtures/results were accessed. Source expectations remain frozen; no native or reference probes were run by this investigator. Detailed selection, panel hashes and baseline ledger hash: `/tmp/msh157-probe-plan.json`.

## What the baseline establishes

- All 32 exact tablebase root outcomes are preserved, and the one exact immediate-mate objective is met. There is no exact failure to prioritize in this development sample. These tests establish root decisions, not endgame conversion.
- 60 of 63 published puzzle preferences are selected. The three other moves remain unresolved, not proved failures. All three deserve investigation because the supplied forcing/quiet continuations offer concrete causal questions; none justifies changing an expected answer.
- STS: 17 maximum-point preferences, 11 listed lower-point choices, 12 unlisted/unresolved choices. These are published strategic preferences, not exact win/loss grades.
- Public finite evaluations: 17 choices within the frozen preferred set; seven unlisted/unresolved. No independent continuation judgment condemns those seven alternatives.

This baseline does not support calling the engine uniformly incapable of tactical or elementary exact-endgame decisions at five seconds. Its clearest potential development gap is selecting strategically effective quiet/defensive continuations rather than recognizing the already-passed exact controls. Selection does not establish that the eight proposed alternatives below are actual game-theoretic errors.

## Eight candidates, selected after the complete pass

No exact failures exist. Include all three published-puzzle mismatches, then choose five listed lower-point STS decisions by largest point deficit with distinct theme families and stable-ID tie breaking. Do not promote unlisted STS/evaluation alternatives into errors merely to fill the panel.

| Root | Baseline → published choice | Why this discriminates |
| --- | --- | --- |
| Puzzle 0UOyM, defensive/hard, rating 2756 | Kg6 → exf3+ | Pawn ending: published en-passant liquidation is followed by ...Kh6–g5. Baseline depth13/PV is quiet king maneuvering; only 45 LMR and one null cutoff occurred, so wholesale reduction failure is unlikely. Compare longer horizon and capture-frontier omissions before valuation. |
| Puzzle 0m5mI, quiet/hard, rating 2134 | Bd6 → Kc3 | Locked bishop ending: source line includes ...f4, exf4, ...e3 and a bishop blockade before king advance. Baseline depth10 with zero LMR/null events; LMR/null ablations cannot explain it. Compare 15s and SEE/delta frontier first. Source continuation is legal evidence, not all-defense proof. |
| Puzzle 28pJK, quiet/medium, rating 1957 | Qf4 → Qc8 | Quiet queen repositioning supports ...Rd2 and ...Qa6. Baseline depth7, heavy q work and active reductions/omissions. Compare LMR/quiet/SEE/delta independently and longer time. |
| STS Undermine.037 | Rg5 (2) → h4 (10) | Pawn lever versus rook activity; a quiet strategic move can be reduced/omitted or undervalued. Baseline continuation takes d5 and trades queens after ...e3. No exact outcome verdict. |
| STS Simplification.024 | Nxe4 (2) → Bxc5 (10) | Two materially forcing liquidation routes. Native PV follows Kxe4/Rxd3/Nxd3/Bxd6/Rxb1/Nc3+; compare exchange omission and continuation depth, not material count alone. |
| STS AT.065 | Qxa5 (3) → Qd6 (10) | Queen exchange versus retaining pressure. Baseline exchange continuation gains pawns; genuine compensation/initiative judgment needs same-parent independent analysis. |
| STS Recapturing.003 | Rxd8+ (3) → Rxe4 (10) | Capture order: native PV eventually also recaptures e4 but allows ...Bxf2. Compare SEE admission and quiet defenses; seeing the recapture later does not prove the same consequences. |
| STS Knight Outposts.076 | Be3 (4) → Nd5 (10) | Move-order/timing rather than absence of the theme: native PV plays Nd5 after ...Qc7. Trace the consequence difference and remaining horizon before any outpost bonus. |

Success controls: published quiet tactical resource Puzzle 0d58A and exact draw-preserving endgame syzygy-draw-0DSOJ. They are development controls, not fresh validation. Historical eight-root material remains regression-only.

## Bounded staged probes: maximum 230 nominal seconds, 70 seconds retained for attribution

Existing runner and frontier example suffice; no new framework or engine change is requested. Each group is a standalone immutable panel for the shared global registry, using the frozen predecessor binary, same full known histories, 4096TT, no visit ceiling and depth safeguard128. Group manifests are in `msh157-probe-plan.json`.

1. `long15`: all three puzzles plus Undermine.037, Simplification.024 and AT.065, six roots ×15s =90s. Question: does a practical longer horizon retain/change the candidate decision and reach the source consequence? Changed move/score alone is not improvement.
2. `no-lmr`: Puzzle28pJK plus five STS candidates and two controls, eight ×5s =40s. Keeps the same q frontier.
3. `no-quiet`: same eight ×5s =40s, disables only main quiet omissions.
4. `no-see`: three puzzles plus Simplification.024/Recapturing.003 and controls, seven ×5s =35s.
5. `no-delta`: three puzzles and controls, five ×5s =25s.

Total230s. Coordinator may stop or omit later groups when the preceding evidence no longer discriminates. Do not automatically spend the remaining70s. Use it only for a concrete matched-horizon trace or one mechanism-specific check that emerged from these results. No full null-ablation grid: few/zero null events in two hard endings and no observed null-specific causal signal justify prioritizing other factors.

Eight same-parent reference candidate/anchor pairs are prepared separately in `/tmp/msh157-probe-reference-pairs.json`: 16 queries ×3s =48s charged to the existing160s reference reservation, not the230s native probe allocation. Bound-aware scores, full legal continuations and source choice must be assessed equally. This can support finite deterioration/preference findings; it cannot turn a published strategic preference into exact WDL. If reference analysis supports both alternatives, retain that counterevidence and deprioritize the root.

## Correction gate

A changed ablation decision only identifies a candidate mechanism. Before changing production, record the actual defense/action admission, its search window and remaining horizon, and enough endpoint evidence to distinguish omitted continuation from wrong classical judgment. The existing runner is time-primary; if fixed-depth trace support is needed, add only that bounded diagnostic capability with independent review and charge it to the remaining allocation. Exact mode is not a pruning-only control.

No valuation correction, broad q expansion or reduction change is justified by this baseline alone. If the probes do not establish a general defect with practical benefit, keep Workstream B as an honest bounded negative/inconclusive finding and reconsider the diagnosis. Do not invent a bonus to force agreement. Combined candidate verification must retain all exact outcomes and report broad development plus once-sealed held-out changes independently of these motivating roots.
