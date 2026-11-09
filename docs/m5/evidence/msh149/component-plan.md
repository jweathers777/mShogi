# MSH-149 read-only component/fixture plan

No source edits, builds or engine searches. Read current evaluator, task and research.md; root owns baseline and timing panel. Additional independent legal enumeration is in additional-component-controls.json (python-chess1.11.2, no external engine).

## Minimal implementation candidate

Keep the current middle king_safety expression, shelter/storm and king placement unchanged. Reuse the current geometric safe-check bitset before its per-piece cap. Track whether a rook/queen actually touches the king ring or has a safe checking origin; also count its supported contact-check origins (`safe_checks & defending_king.attacks`). The existing near-king safety mask already demands a second attacking supporter and excludes enemy non-king defenders. Do not enumerate legal moves at evaluation leaves.

An original modest starting expression is end_danger = min(128, pressure + checks/2 + 24*min(major_contact_checks,2)), active only when an enemy major genuinely participates. Assign the negative to king_safety.end. This retains some remote-check danger for an unsupported attacker but separately rewards supported contact geometry that the current cap of two checks can hide. Coefficients are routine heuristic judgment; independent geometry establishes the distinctions, not an optimal centipawn value or playing-strength gain. Preserve all native tactical scoring and prior specialized ending precedence.

The cap bounds the added end component to128 per king; generic max-static bound has ample existing slack but document that the added term remains inside it. There is no reason to change public data shape: existing white/black king_safety fields expose the new component. No global phase change or king-centrality suppression is needed.

## Component checks with independent facts

1. Queen support pair from independent-controls.json: `7k/8/5KQ1/8/8/8/8/8 w - - 0 1` versus `7k/8/6Q1/5K2/8/8/8/8 w - - 0 1`. Qg7# only in the supported board; otherwise ...Kxg7 is legal. Both current middle values are -109, so do NOT claim the existing capped check total distinguishes these. Native attack facts: Qg6 touches three Kh8 ring squares (g8,g7,h7), giving pressure15; two capped checks give24. The additional supported g7 contact yields candidate end -51 versus -27. King placement/material/middle values stay exactly their prior values, though total scores of the two different king boards naturally differ.
2. Rook pair `7k/R7/6K1/8/8/8/8/8 w - - 0 1` versus `7k/R7/8/5K2/8/8/8/8 w - - 0 1`: Rh7 is protected only with Kg6; unsupported ...Kxh7 is legal. Ra8# is the supported immediate mate. **Rh7 is not mate**: ...Kg8 remains legal. Pressure6 is unchanged, capped checks32 versus16; one supported contact gives candidate end -46 versus -14. Current middle -90/-74 remains unchanged.
3. Non-king defence: `6rk/8/5KQ1/8/8/8/8/8 w - - 0 1`. Qg7 Rg7 is legal, so g7 must not count as a safe contact origin despite Kf6 supporting it. This position has OTHER immediate mates Qh6/Qh5, so do not assert globally safe king, zero danger, or no mate. Test the particular square exclusion and the surviving native tactical resources.
4. Blocked major: `qb5k/pp6/8/8/3K4/8/8/8 w - - 0 1`. Qa8 is blocked by a7/b7/b8 friendly pieces; Bb8 attacks the king ring at e5, but no major participates. New white end danger must remain0, despite queen material on the board. This is a component control, not a game-WDL assertion.
5. No-major central king: `7k/8/5n2/8/3K4/8/4P3/8 w - - 0 1`. New end danger is0; no penalty merely for an active king. Existing minor/pawn placement and danger-middle terms remain untouched.
6. Queen exchange: `7k/7q/5KQ1/8/8/8/8/8 w - - 0 1`, native legal `Qxh7+ Kxh7`. Both majors disappear; end-danger must disappear and the native bare-kings outcome remains authoritative. This is not permission to favor every queen exchange.
7. Existing varied quiet-threat/sacrifice/check-evasion/defensive resources stay required. In `r6k/8/5K2/8/6Q1/8/8/8 b - - 0 1`, only Ra7/Ra6+/Rf8+ avoid every immediate reply mate. No global draw inference. Keep mirrored cases and mate/claim priority.

## Additional bounded verification

Color/rank reflection must exchange both term sets and score sign; turn/counter changes without a history-sensitive specialization must not change this new geometric component. Incremental evaluation versus rebuild remains identical after moves/exchange/undo. Verify end component bound on a valid promoted-material diagram or across the existing legal-branch fixture. KQK/KRK king approach and KBN conversion tests guard against discouraging necessary king activity. Search cancellation/root restoration and native claimed/checkmate precedence remain existing product checks; do not duplicate broad platform campaigns.

Evaluate the fixed independent panel and historical queen/rook root under identical bounded time/source conditions after acceptance of145. Report native decision quality versus heuristic component changes separately; improvements in a prescribed component are not a strength claim.
