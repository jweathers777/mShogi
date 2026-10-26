# MSH-67 independent history and outcome fixture derivations

Fixture source: [history-outcomes.json](../../tests/fixtures/shogi/history-outcomes.json).
Prepared 2026-10-26 by a separate Codex AI fixture/research agent from the native
history implementer. This is AI derivation and board corroboration, not a claim
of human rules review. The first fixture review is retained with the then-unmerged implementation at
`f259bb23834d6156f21aeec53db9ee00ef7242e4` on `codex/shogi-history`.
That partial implementation and its reviews remain historical evidence. The
subsequent [complete-chain and supplement resolution](mutual-check-resolution.md)
now discharges the mutual-check gate. Native final verification and integration
are recorded separately in MSH-67 and [history validation](history-validation.md).

## Authority, editions and method

The governing sources are [CSA WCSC36 rules](https://www.computer-shogi.org/wcsc36/rule.pdf),
effective 2023-11-27, [JSA rules](https://www.shogi.or.jp/match/taikyoku_rules/taikyoku_rules.pdf),
corrected 2025-10-01, and the approved [software import/mode contract](shogi-profile.md).
The existing [source manifest](reference/rule-source-manifest.json) remains intact.
The archived CSA/JSA PDF bytes were checked again with `shasum -a 256`:

- CSA: `9387db362c1eb1aa87b294c314c68be6f54d7f9a8165b8a3bb69e7b80a1b5bed`.
- JSA: `2f738555ebfc2f79f789b1464be97d2236fb50afd7bf77df0b30f0437fa5642f`.

The AI first constructed concrete square-by-square histories and hand-counted
points/occurrences/checks from those clauses. The expectations do not come from
native game adjudication, `cshogi.is_draw`, search-cycle helpers or opponent
results. The existing native `Position` probe was subsequently used solely to
corroborate action legality, check flags, zero legal actions and exact reversal.
That comparison cannot establish the governing outcome and is not presented as
an independent rules oracle. The JSON's check ledger has one entry after each
board action; occurrence indices are known-segment board plies with root zero.
Every diagram is explicitly a reduced-material imported root with unknown
prehistory. Local safety/material validity does not prove historical reachability.

Source-to-expectation map:

| Source clause | Applied expectation |
| --- | --- |
| JSA8(1),10(10); CSA27(2) | Exact board, owner-specific hands and turn identity; first-to-fourth interval, including checked and check-relieved endpoints. |
| CSA25;27(1)(iv/v) | Explicit declaration and each prerequisite; false formal declarations lose. |
| JSA6(1) | Named resigning side loses; no own-turn restriction appears in this clause. |
| CSA27(1)(i),27(3) | No-action loss and repetition precede the 320-action cap. |
| Approved software contract | Imported counter/provenance, unknown prehistory, untimed mode and atomic request rejection. |

## HIST-INTERVAL concrete ledgers

Take Sente king9i, Gote king5a and Sente rook4c. The four actions
`4c5c 5a4a 5c4c 4a5a` restore board, hands and turn. On Sente's actions
1 and3 the rook occupies the king's file on rankc with no intervening piece;
Gote's evasion removes that attack. Three cycles therefore produce root
occurrences0,4,8,12 and six Sente checks. The fourth root is check-relieved.
Sente loses even though Gote made the final action. Starting instead with rook5c,
Gote to move, and cycling `5a4a 5c4c 4a5a 4c5c` makes the fourth root checked,
with the same loser. The 180-degree owner-exchanged pairs prove Gote's checking
loss too. The nonmoving king is safe at all four roots.

The quiet alternative `4c3c 5a4a 3c4c 4a5a` returns to the same root but
its first Sente action does not check. Three quiet cycles are ordinary repetition.
One quiet cycle followed by two all-check cycles also draws: an implementation
must inspect the whole first-to-fourth interval instead of just the latest cycle.
Conversely, the prefix `4d4c 6a5a` reaches that root for its first occurrence
at2. A noncheck at1 is outside the selected interval2–14 and does not interrupt
the subsequent six checks. That fixture loses for Sente at14.

The overlapping fixture uses the eight-action walk
`4c5c 5a4a 5c4c 4a3a 4c3c 3a4a 3c4c 4a5a`.
The supplied root returns at8, but the intermediate checked state rook4c/king4a
occurs at3,7,11,15. It therefore concludes at15 before a fourth supplied-root
return. Other shifted identities also recur; the JSON records their partial
occurrence sets. Every Sente action in(3,15] checks. This prevents algorithms
that track only the supplied root or assume nonoverlapping uniform cycles.

The fresh imported root at SFEN counter13 has exactly the same board, hands,
turn and counter as the concluded12-action history. Its known occurrence list is
only[0]. Earlier checks/repetitions are unknown, so it is live; an SFEN counter
is not an event log. This is the same-position/different-history witness.

## Mutual continuous-check research history

**Historical blocked-stage result, superseded by the
[reviewed resolution](mutual-check-resolution.md):** no outcome was asserted for
a hypothetical mutual continuous-check cycle. Neither governing rule text supplies a separate mutual-case winner.
A concrete legal history plus sourced interpretation or a sound impossibility
argument is required to close this part of HIST-INTERVAL.

The obvious argument that every countercheck captures a piece is invalid:
interpositions and discovered checks need not capture. Nor does a drop/capture
pair monotonically reduce board material. The published
[2008 attempted proof](https://yaneurao.hatenadiary.com/entry/20080604/p1) and its
[part2](https://yaneurao.hatenadiary.com/entry/20080607/p1)/
[part3](https://yaneurao.hatenadiary.com/entry/20080608/p1) contain retracted
lemmas and expressly identify remaining cases. The author's
[2024-01-29 retrospective](https://yaneuraou.yaneu.com/2024/01/29/sennichite-involving-a-reverse-check/)
still described an incomplete proof. A
[2024-03-03 announcement](https://yaneuraou.yaneu.com/2024/03/03/proof-of-the-impossibility-of-perpetual-check-in-shogi/)
links a later Nanka proof; its 2024-10-02 comment raises a drop-related objection
to the asserted `|disc H|` invariant and bishop accounting. The announcement
alone is therefore insufficient; the underlying argument needs independent
inspection. Fairy-shogi/Queen demonstrations and engine enum priority are not
standard-shogi evidence.

A useful limited argument is sound: if a king moves while counterchecking, the
check must be discovered by vacating its ray to the enemy king, because adjacent
king checks violate king safety. A second king move cannot countercheck by
vacating a ray unless the kings are again aligned; nonking actions do not change
their alignment. This constrains king movement, but does not alone exclude all
fixed-king interposition/capture/drop cycles. It is explicitly not a completed
global impossibility proof.

## DECL-BOUNDARY arithmetic and isolated prerequisite failures

The Sente base diagram has K9a, R8b, B7b, and eight promoted pawns8c through1c.
There are ten other owned camp pieces. Their points are5+5+8=18. The hand B5P
adds5+5=10 for28; B4P instead gives27 and fails Sente28. Rotating and exchanging
owners gives Gote camp18; hand b4p gives27 and succeeds, while b3p gives26 and
fails. Promoting the rook/bishop retains five points each. Every diagram has
exactly two kings and no inventory exceeds the standard total; the nonmoving
king is unattacked. Hand pieces contribute points but do not count as camp pieces.

The independently isolated failures are:

- Move only K9a to9d: king camp condition fails; ten other camp pieces/28 points
  remain.
- Move +P1c to1d and add a hand pawn: camp pieces fall to9, while camp17 plus
  hand11 remains28. The outside promoted pawn contributes no declaration points.
- Add Gote rook9c: the clear9b line checks K9a; the other requirements remain.
  K9a can evade to8a, so this is a live checked root, not a terminal diagram.
- Change turn to Gote while retaining Sente as the formal declaring actor:
  own-turn fails; all board/material conditions remain.
- Remove one hand pawn at each side's threshold: only the point prerequisite
  fails, as above.

CSA25 places own-turn among the same prerequisites as material/check. Thus a
well-formed, properly addressed explicit Sente declaration on Gote's turn is an
admitted false declaration. A typed request has one actor in its envelope. A second conflicting actor
embedded in the command is malformed wire data and is a separate admission
error; this native layer does not claim caller authentication. The corpus also records
harmless eligibility queries and stale-command rejection. A query cannot end a
game; committed success/failure preserves the board and adds zero board actions.
Untimed mode satisfies remaining time while live. A request for an unsupported
timing policy is rejected; the fixtures do not invent a clock or simulate a
false-declaration timeout in untimed play.

Resignation fixtures name either the moving or nonmoving side. The outcome
belongs to that side and adds zero board actions. Subsequent-event rejection,
undo and same-board-ply event-prefix distinctions require native record tests.

## IMPORT-LENGTH counter and precedence constructions

Supplied n320 means319 declared completed actions. A legal rook move produces320
and a length draw. A checking move with legal king evasions still draws at320;
the selected CSA cap has no JSA checking extension. Supplied n321, n322 and n999
start terminal by length when legal board actions otherwise exist. Supplied n1
and explicitly defaulted n1 both mean zero completed actions but retain distinct
provenance. Strict SFEN remains four-field; defaulting is an adapter operation.

At n309, twelve known actions add to308 declared prior actions and reach320.
The quiet loop concludes with repetition reason; the continuous-check loop
concludes with Sente's loss. Neither is relabeled as a cap draw.

For no-check/no-action precedence, Sente has only K1i; Gote has k9a, r2a and r9h.
K1i is not attacked, but1h is attacked by r9h,2i by r2a, and2h by both.
There is no Sente hand or other piece. This is a loss at imported n321.
Replacing r9h by r1a checks K1i, while r2a denies2h/2i and r1a denies1h;
the checked no-action root at n322 loses too. Both defeat length precedence.

The action320 mate reuses the established SH-F017 geometry with an on-board
pawn, not a forbidden mating pawn drop. `5c5b` checks k5a, G4c protects5b,
and Gote's lances4a/6a and pawns4b/6b occupy the other king exits. No other Gote
action removes the pawn attack. The root is locally safe for the nonmoving king;
the resulting no-action loss precedes the cap. Reachability before the imported
root is not asserted.

## Board-only corroboration evidence

Environment: macOS27.0.1 build26A434; stable rustc1.89.0; Python3.14.7.
Board-core source revision: `986a5277f26fe2f41c671bc0f14d2da5f749a156`.
The outcome fixture expectations were prepared without reading native history
adjudication code. Commands used:

```text
cargo build -p mshogi-core --example shogi_probe --target-dir /private/tmp/msh67-fixture-target
python3 /private/tmp/msh67-create-fixtures.py
shasum -a 256 /private/tmp/msh64-rule-sources/CSA-36.pdf /private/tmp/msh64-rule-sources/JSA-R.pdf
```

The temporary script serialized the explicitly constructed diagrams and ledgers,
then submitted each of158 prefixes to the existing `shogi_probe`. All legal
transitions, declared check flags, specified zero-action finals and reversal
checks passed. Raw local corroboration output is
`/private/tmp/msh67-fixture-board-corroboration.json`. Temporary scripts/evidence
are developer-local corroboration, not product dependencies or an outcome oracle.
Native adjudication, strict replay, rejection and independent fixture review
results must be recorded separately; this document does not infer them.
