# Chess dead-position and mating-feasibility coverage

Chess has one judging behavior and one record schema. The rules decide dead
positions and mating impossibility with a sound material subset. They do not
decide every dead position. There is no configurable judging mode.

## Rules interpretation and authority

The pinned authority is the [FIDE Laws effective 2023-01-01](https://handbook.fide.com/chapter/E012023): Article 3.10.1 defines legal moves through Articles 3.1–3.9; Article 5.2.2 asks whether either side has any legal mating continuation; Articles 5.1.2 and 6.9 ask the same question for the opponent of the resigning/flagged player. Both players cooperate in this existence question. A forced-win engine score, adversarial draw, failure to find a mate, and material advantage are not answers.

Hypothetical continuations disregard future optional claims, the selected auto-claim policy, clocks, and 50/75-move or repetition outcomes. Actual previously concluded results still take precedence. Counter storage limits do not affect hypothetical legality. Castling permissions, promotion choices, legal en passant, king safety, checkmate and stalemate do affect it. This is not a historical-reachability proof for imported FENs; the validated local-position contract remains authoritative.

## What the rules decide

- **Automatic dead-position draw.** `Position::dead_recognition()` returns `ProvenDead` only for kings alone, one bishop or knight in total, or bishops only with every bishop on one square color. Every other position is `Unknown`, and play continues. Fivefold repetition and the 75-move rule still end such a game. See [adjudication](chess-adjudication.md).
- **Resignation and flag fall.** `mating_feasibility` proves that the opponent cannot mate when the opponent has a bare king or the position is proven dead. A supplied witness proves that the opponent can mate when it is a legal action sequence that ends in checkmate by the opponent. Otherwise the result is `Unresolved`, with no chess score. See [match API](chess-match-api.md).

## Accepted incompleteness

Dead positions outside the material subset are not adjudicated as dead. An example is the blocked pawn wall `8/7k/8/p1p1p1p1/P1P1P1P1/8/K7/8 w - - 0 1`, where neither king can reach the other side. A resignation or flag fall without a material proof or a witness stays unresolved. This is an accepted limitation, not a draw. The rules do not search the reachable position graph.

## Records

Game history records and match envelopes each have one schema, version 1. A record with another schema version is rejected.

## Evidence

`cargo test -p mshogi-core --test adjudication --test matchplay --test game_record --locked` covers dead recognition, mating feasibility with and without witnesses, unresolved terminations and record schemas.
