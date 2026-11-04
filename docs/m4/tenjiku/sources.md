# Tenjiku modern-source reconciliation — MSH-83

Source review4November2026, baseline8676cbb. This bounded review uses the already
identified modern descriptions. It starts no new historical acquisition campaign.
The earlier [research/provenance](../research/tenjiku.md) remains unchanged.

## Sources actually inspected

**A — Colin Adams, The Struggle for Survival: Testing Times for Tenjiku Shogi,
second edition14February1999.** [Published PDF mirror](http://history.chess.free.fr/papers/Tenjiku-Colin%20Adams.pdf),185pages,1000783bytes,
SHA256 `a189a4a83f6e9554c4e8443241a5ab7307d6937f46b81f6c5ae35dc5063aef91`.
Re-fetched successfully; the web reader502 and initial sandbox DNS failure do not
mean the source is unavailable. Relevant pages are rendered and visually read:
text extraction has an unusable character map. Printed page n corresponds to
PDF page n+11 in the rules/pieces and appendix. A is an author's modern playing
contract and attributed reconstruction/translation, not an association rulebook
or an independently verified historical facsimile.

**M — H.G. Muller, Tenjiku Shogi, the wild Chess game.**
[Author's page](https://hgm.nubati.net/tenjiku/tenjiku.html), read4November2026.
No immutable publication version is displayed. It explicitly provides a modern
movement/promotion table, ranked-jump explanation and burn footnotes, but refers
to Chu for many older pieces and does not supply a complete standalone history/
ending/promotion-rights contract. No engine source or binary supplies expectations.

**H — historical qualifications already recorded.** Adams AppendixA, printed
166–168/PDF177–179, attributes a translation to Peter Banaschak from an undated
Shogi zushiki without an identified held copy/folio. The National Archives1696
catalogue, proposed1694edition and Kyoto witness have not acquired a new textual
status in this assignment. They neither establish agreement with A/M nor block
a named modern profile. Appendix translation and Adams's main playing rules
must be distinguished, especially for Free Eagle and Fire Demon movement.

## Consequential differences and proposed coherent baseline

**Owner selection: support both option1 Adams Modern and option2 Muller, as two
whole immutable profiles.** Adams main chapters1/3 supply profile1; M's explicit
differences plus labelled Adams supplements supply profile2. Traditional differs in Lion Hawk; “modern” is the
author's label, not a claim of universal adoption. The Muller alternative is
coherent for its stated mechanisms but needs more general-game gap resolution.
No independent rule switches or unlabelled mixing is selected.

| Area | Adams main playing contract | Muller / other evidence | Independent distinguishing example |
| --- | --- | --- | --- |
| Fire Demon rays | §3.1 p12/PDF23 diagram: vertical and four diagonal rays, plus area3 | M table same six rays; AppendixA p167's buffalo-like orientation is a translation/reconstruction difference | Occupied orthogonal blocker: horizontal4-square move rejected; vertical4-square move admitted if clear. |
| Dying moving Demon | §1.2.4 p4/PDF15: enemy bystanders burn, opposing Demons survive, mover then dies | M burn footnote suppresses bystander burning when mover dies | SenteFD(8,10)→(8,8), GoteFD(9,8), GotePawn(8,7): A removes Pawn and mover; M retains Pawn. |
| Lion Hawk | §3.6 pp17–18/PDF28–29 names area2+Bishop Traditional and recommends fullLion+Bishop Modern | M grants fullLion+Bishop | Capture at(9,8), then(10,8) in same action: Modern admits both captures; Traditional stops at first. |
| Free Eagle | Legend p11/PDF22 + §3.9 p21/PDF32: Queen rays plus orthogonal two-square jumps. Diagonal open-circle markings are called redundant | M grants a two-diagonal-step double move; AppendixA p167 is different evidence, not a silent correction of main diagram | Orthogonal distance2 jump over friendly blocker succeeds under A; diagonal two-victim turning route belongs to M, not A main. |
| Heavenly Tetrarch | §3.11 p23/PDF34: skip adjacent square on diagonal rays; sideways distance2 jump or distance3 if2 is clear; adjacent stationary capture. No vertical movement in main diagram/prose | M describes broader skipped-neighbor ranging in non-sideways directions | Empty-board vertical distance4 is absent under A, present under M; sideways4 absent under both stated restricted tables. |
| Ranked general jumps | §1.2.5 p4 + §§3.2/3.4/3.5 pp13/15/16: capture-only jump; every crossed piece AND jumped-to victim must rank lower | M prose blocks equal/higher interveners; footnote omits equal wording and explicitly forbids royal jump victims, with no matching all-lower endpoint rule | BG jumps over Pawn to VG: rejected under A. Equal-rank intervening BG also blocks. Ordinary unobstructed capture remains distinct. |
| Promotion rights | §1.2.2 p4: zone5 ranks, any component contact; mandatory if otherwise no future movement; first refusal restricts that piece's next move unless capture, thereafter free | M table gives pairs but no complete competing general timing/rights rule | Refuse first eligible move, then noncapture zone move: no promotion; capture on that next move may promote. |
| Royals | §1.2.1 p4: capture King and any Prince; exposure permitted; no true check | Not a license to import Chu finite mate, Lion protection or material rules | Exposed King move remains admissible; geometric threat alone does not conclude. |
| Bare King | §1.2.6 p5: bareKing loss with actual immediate bare-reply exception | Forced-series extension is suggested, not the printed base rule | One-turn countercapture versus a merely provable multi-turn sequence must differ. |
| Repetition | §1.2.7 p5: forbid a move recreating earlier board+same turn |100-move/250move/points claims are explicitly proposed tournament alternatives | First recreation rejects atomically; no third/fourth-occurrence forfeit or silent automatic draw. |

Strict action spelling, canonical effect enumeration, imported-history provenance,
atomic publication and private representation are software contracts. They do
not establish source authority for a new movement or ending. Proposed rare-edge
completions must be labelled separately in the profile and fixture derivations.

## Independent source assessment

Separate read-only reviewers independently rendered the main rule/legend/special
piece/appendix pages and read M. Both confirmed the burn, Lion Hawk, Free Eagle,
Tetrarch and ranked-jump distinctions. The source review corrected the earlier
inventory's insufficient Free Eagle/Tetrarch distinction; no old evidence was
rewritten. Source agreement is recorded only for actual matching clauses.
