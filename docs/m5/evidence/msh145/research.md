# MSH-145 (Pawn Threat and Exchange Judgment): bounded preparation

Research only; no source edits, builds, searches or matches. Read current task/evaluation source. Reuse MSH-147 diagnosis before choosing weights. New receipts: independent-nearby-oracles.json; previous six broad-family receipts: /private/tmp/msh147/additional-endgame-oracles.json. Both preserve full legal-move WDL, not only the best move. Query-time Python-chess validation checked FEN validity and exact equality of returned legal move sets. The first fetch script rejected a proposed rook diagram because the non-moving king was in check; that invalid diagram was never queried and is not evidence. The six already fetched responses were re-fetched when preserving the corrected receipt file; this did not run engines.

## Recommended coherent correction

Combine a modest general passer-tempo correction with a small advisory material-scaling layer. Do not make exchange-specific or game-specific bonuses: better values at the resulting leaf positions should improve exchange judgment through the unchanged native search. Preserve full history, draw claims, mate precedence and native terminal scoring. No new runtime tablebase, board search, move generator or quiet-frontier extension is needed.

### Passer runway versus interception

Current `clear && !unsafe_path` only describes occupancy/current attacks; a king can reach the promotion path on a later turn. The separate escort term is only four times the difference of king distances to the next square, while advanced runway bonuses grow quadratically. This makes a clear distant path look better than a later interception merits.

Use an original bounded geometric estimate to discount only the *extra runway premium* when an enemy king can reach a future landing square by the pawn's earliest arrival/capture tempo. For each remaining pawn landing square, compare king Chebyshev distance with pawn pushes plus a side-to-move offset; at most six squares. Count the initial legal-clear double push if included, otherwise conservatively restrict this estimate to pawns beyond their initial rank. This is a heuristic interception risk, never a win/draw/capture proof.

For a white a5 pawn and black Ke7: promotion takes three pushes. White to move gives Black only three king moves including a possible capture after promotion, insufficient for distance four. Black to move gives four, enough geometrically. Independent four-piece pawn-race fixtures below establish the resulting win/draw distinction. A side-to-move-independent evaluator cannot express this distinction unless search sees it.

Do not blanket-discount every geometrically catchable passer. Protect the relevant premium when the candidate interception square is controlled by the escorting king or safely supported. In particular a rook/queen behind its pawn can defend the *future* promotion square even though today's attack bitboard stops at the pawn: same-file rear piece attacking the current pawn is a cheap existing-geometry support signal. Current clear-path checks must remain; blockers should never be counted as a runway. King distance ignores route obstructions, checks and king opposition, so use it as a bounded discount, not a binary race result. Do not grow another per-leaf path search. First inspect component differences, then short native search WDL on varied fixtures.

Useful paired oracle rows (categories are side to move):
- `8/4k2p/8/P7/8/8/8/7K w - - 0 1`: WIN; **only a5a6** preserves the win, king moves draw.
- Same diagram Black to move: DRAW. Black ...Kd6/...Kd7/...Kd8 preserve draw; consult full response for all choices.
- `8/1K2k2p/8/P7/8/8/8/8 b - - 0 1`: LOSS for Black (White wins), despite the same raw black-king/pawn distance. White Kb7 controls the promotion approach; naive distance discount misses escort.
- `8/4k2p/8/P7/8/8/8/R6K b - - 0 1`: LOSS for Black (White wins); rear rook support defeats a bare king-distance conclusion. This is a non-discriminating whole-game WDL control for many moves because White has an extra rook; use it to guard component semantics rather than claim it proves a unique conversion choice.

### Advisory conversion scaling

Current generic score retains full material imbalance and has no general scaling stage. Specialized handling is limited to KPK, one wrong-bishop rook-pawn guard and KBNK. MSH-147 demonstrated large positive values in independently drawn KNNK/KQKP/KRPKR positions, so extending conversion understanding is supported beyond a single game.

Prefer three small, inspectable cases with guards over a general material polynomial or huge ending catalogue:
1. **KNNK:** drawish advisory static estimate for exact two knights versus bare king, but never native dead-position classification or a search-terminal shortcut. Negative control `7k/4NK2/5N2/8/8/8/8/8 w - - 0 1` is independently WIN: **only Ne7g6#** wins, every other legal move draws. This demonstrates why a material recognizer must not suppress mate search or claim KNNK universally drawn.
2. **KQKP:** attenuate generic queen advantage when the pawn is on its penultimate rank on rook/bishop files, defended by its adjacent king, with the attacking king remote. Keep near-king, central-file and directly capturable/tactical cases out where practical. This is a fortress/drawishness heuristic, not a proof; a modest finite score is safer than a hard zero based on unproved geometry. Preserved c2-pawn/far-white-king case draws, d2 central-pawn neighbor wins. New `8/8/8/8/3Q4/3K4/2p5/2k5 w - - 0 1` wins: the nearby white king defeats a blanket bishop-pawn scaling rule.
3. **KRPKR:** a narrowly guarded third-rank defence scaling case is an established starting point: defending king covers the queening area, pawn and attacking king have not advanced past the defensive barrier, defending rook is on its third rank. Avoid immediately hanging defensive rooks and preserve all native tactical handling. Existing Philidor-style row draws with either side to move; existing Lucena-style advanced row wins. The new nearby Kg7 defensive-king row ALSO draws, so do not mislabel it as a winning negative control. The Lucena row is the proven positive control. Do not claim a simple pattern exhausts rook endings.

A broad linear 'rook beats pawn' or material-only scaling fix would not address these distinctions. Do not scale real mate scores or claims. If a specialization replaces the score, expose its kind/strong side/raw and scaled values so diagnosis can see what happened. Exact KPK knowledge should remain first and untouched.

## Verification proportion

A focused core can use: two tempo rows; escort and rear-rook controls; KNNK draw plus forced-mate negative; KQKP far bishop-file draw, central-file win and near-king win; KRPKR third-rank draw with both turns plus Lucena win. That is twelve small independent rows, reusing five earlier preserved receipts. Add the original unique-Re1 KRKP and quiet-threat/sacrifice checks as existing guards, not another new campaign. Do not score agreement with one arbitrary tablebase move: invert the chosen child's WDL and compare with root WDL. DTZ is not nominal search depth or mate distance. Static evaluations are heuristic even where the WDL expectation is exact.

Mirror/color-swap cases and halfmove/repetition tests should verify signs and native claim precedence; they need not require new oracle downloads. Keep full-history Goldfish 44.Ne5/45.Kxe5 reproductions and the queen/rook case to detect an improvement confined to one family. No statistical matches are necessary for this increment. A broader king-danger/positional revision should remain separate if not needed to satisfy this bounded task.

## Primary classical references

Stockfish11 uses separate material/endgame scaling and specialized endgames, including KNNK/KQKP/KRPKR. Its rook recognizer is explicitly incomplete; these sources demonstrate established design ideas, not correctness of a copied heuristic:
- https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/material.cpp
- https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/endgame.cpp
Its passer evaluation distinguishes king proximity to the next/second push, block-square support and rear rook/queen support rather than a single current clear-path flag:
- https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/evaluate.cpp
Use original coefficients and simple guard logic; do not copy source, tables or weights. The small independent tablebase service is an offline diagnostic oracle only: https://tablebase.lichess.ovh/standard .
