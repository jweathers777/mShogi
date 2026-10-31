# Candidate Tori rules, evidence and decision ledger

**Unratified dossier MSH81-DRAFT-2026-10-31.** Proposed historical-basis identity
`mshogi.tori.bunsei11.v1` is reserved only. No profile, schema or executable
capability has been added. A versioned *complete* profile is not yet established.
Source identifiers and read/access limits are defined in [sources](sources.md).

## Directly supported board facts

Coordinates below are a **proposed modern transcription convention**: lower
player faces upward; 1a upper-right, 7g lower-left; files decrease left-to-right.
They are not claimed to be T-H1's notation. For local geometry `(x,f)`, x increases
to the mover's right and f forward; rotate180° for the opposing owner.

| Clause | Direct evidence / candidate meaning | Fixture scope |
| --- | --- | --- |
| B01 board/inventory | T-H1 img4/8L:7×7,32 total,16 per side; one鵬,one鷹,two鶴,two雉,one each左鶉/右鶉,eight燕. Captured pieces reused. | Initial transcription, inventory only. |
| B02 setup | T-H1 img4; T-M3p2/T-M4 setup corroborate. Lower home rank files7→1:左鶉,雉,鶴,鵬,鶴,雉,右鶉;鷹4f;燕1e–7e plus3d. Upper arrangement is180° rotation, including extra燕5d. | Diagram, not invented perft/reachability proof. |
| B03鵬 | T-H1 img5R:8adjacent destinations. | Geometry, not complete royal legality. |
| B04鶴 | T-H1 img5L:6adjacent except horizontal. | Geometry. |
| B05雉 | T-H1 img5L:forward2 jump; backward diagonals1. Hollow intervening mark/text distinguishes jump. | Geometry and jump restriction. |
| B06左/右鶉 | T-H1 img6R:both forward ray;左 rear-right ray/rear-left1;右 rear-left ray/rear-right1. Labelled handed kinds are distinct; not promoted forms. | Labelled geometry; hand-type preservation interpretation separately noted below. |
| B07鷹 | T-H1 img6L:7adjacent except directly backward. | Geometry. |
| B08燕 | T-H1 img7L:forward1. | Geometry. |
| B09鵰 | T-H1 img7R:forward-diagonal and backward-straight rays; forward/sideways1; backward-diagonal1/2. T-M4 expressly disallows jumping. Historical diagram/text differs from explicit leap marks elsewhere. | Endpoints direct; blocker interpretation must be attributed, no lion-style double capture inferred. |
| B10鳫/鴈 | T-H1 img8R:jump2 diagonally forward or straight backward. | Geometry/jump, not shogi gold movement. |
| B11 capture/hands | T-H1 img8L:reuse captured pieces. T-M3/T-M4 give empty-square, unpromoted drops. Quail left/right labels are movement identifiers (T-H1img9R), not promotion. | Capture/demotion/drop seeds; exact quail transfer identity flagged for explicit profile text. |
| B12 promotion pair | T-H1imgs6–9:鷹→鵰,燕→鳫 only. Marks designate enemy zone. T-H1img9L forbids entering unpromoted; T-M4 explicitly compulsory two-rank entry. | Mandatory-entry candidate, with P02 timing gates. |
| B13 swallows | T-H1img8L permits two/limits two; img9R prohibits swallow drop-mate. T-M2 corroborates multiplicity; T-M4 Japanese specifies own swallows/on a file, T-M3 distinguishes unpromoted swallow/goose in drop restrictions. Last-rank prohibition has modern direct support. | Restriction-only seeds; full mate fixtures await royal-legality/precedence selection. |

These facts do not by themselves establish complete legal actions. In particular
source movement diagrams do not prove self-check rules, historical reachability,
all ending cases or legal action counts. An imported diagram/counter is not
proof of prior play. No engine-generated expected values are supplied.

## Complete-game coverage and unresolved obligations

| Gate | Evidence and exact unresolved contract | Affected work / next evidence |
| --- | --- | --- |
| P01 basis/edition | T-H1 actual text bears1828date but catalogue copy undated. T-H2 is separately catalogued1833/unread; T-H3 separately1828 metadata. No demonstrated revision relation. | Profile identity/edition and all conformance claims. Prefer T-H1 as inspectable basis; acquire T-H2/T-M1 before claiming edition equivalence. |
| P02 promotion/drop timing | T-H1img9L: unpromoted entry forbidden; capture/鵬-head phrase continues in zone-entry context, so no out-of-zone privilege is established. Drop inside marks is unpromoted, followed by promotion. Exact 'next' scope, moving within/out of zone after a drop, and king-head/drop-mate precedence need reconciliation. T-M3 *kann* and into/within/out wording is not proof of optional historical promotion; T-M4 explicit entry compulsion omits these exceptional cases. | Whole legal actions, undo/record state and completion choices. Independent Japanese passage review plus T-M1/T-H2; do not manufacture a promotion option or off-zone privilege. |
| P03 repetition/check | T-H1img8L→9R says same moves reaching three, initiator varies, with a千日手 gloss. It defines neither formal state equality nor third position occurrence vs three completed repetitions, cycle initiator in overlapping cycles, forced/no-variation cases, timing/sanction or check priority. T-M3 board/hands and perpetual-check prohibition are modern evidence, still incomplete. | History key/obligation/outcome, search legality, canonical replay, saved prefixes. Need explicit source-grounded formalization and owner decision; neither CSA fourth repetition nor WXF warning/loss is a default. |
| P04 royal/endings | T-H1 final rules refer remaining play to小将棋; this does not identify modern JSA2026. T-M3/M4 require check evasion/mate loss; M3 also states capture win and no agreed draw. No acquired complete code settles quiet no-legal-action, entering royals/impasse, material exhaustion, agreement, resignations, forbidden-action sanctions versus atomic software rejection, precedence. | Complete match/profile, terminal fixtures and engine scoring. Acquire T-M1/identified historical small-shogi interpretation; formulate explicit modern supplement only with owner approval. No chess/shogi draw/counter/declaration inherited. |
| P05 notation/root | Source diagrams and illustrative material do not define a current lossless canonical syntax, counters, import history or precise malformed-input contract. | Native game-specific record/notation design is ordinary implementation work after rule decisions, with explicit provenance; do not label7×7 payload ordinary SFEN or promise existing USI compatibility. |
| P06 movement/transfer | Explicit historical endpoint diagrams settle geometry. Two-step eagle blocking has modern explicit support and historical non-leap contrast; quail handed identity and exact captured-promoted demotion/hand effects must be stated across owner changes without confusing promotion or board-side labels. | Review blocker/capture/demotion fixtures and any extension from the explicit modern drop restriction to imported diagrams/other actions before whole-action reference admission. No compound capture invented from a range2 destination. |

## Owner checkpoint — alternatives, not adopted conventions

**TORI-P1: governing basis.** A: propose T-H1's internally dated1828 text as
the named basis, with every modern addition separately specified. B: propose a
separately named modern Kanare2022 reconstruction (T-M4), accepting its documented
scope and still resolving missing history/endings. Recommendation **A**: it is
now directly examinable, supplies original geometry/compulsory-promotion evidence,
and avoids presenting a derivative summary as historical authority. Limitation:
neither A nor B is yet a complete profile; approval of A alone cannot close81.
T-H2's unread1833 copy is an acquisition alternative, not an invented competing
ruleset. No owner selection is recorded in this pass.

**P02/P03/P04 are research/interpretation gates, not a menu of equally sourced
fully specified implementations.** Recommended continuation is targeted reading
of Okano241–265 and comparison of the exact T-H1 passages with Kagawa's copy,
then a concrete complete proposal for owner review. Ordinary API spelling,
representation and renderer choices need no owner vote. If the owner instead
chooses an explicitly modern reconstruction, its exact promotion timing, cycle
responsibility/sanctions and exceptional endings must be stated before adoption;
current sources do not justify making those decisions silently.

The distinction matters in fixtures: a swallow dropped into the zone can produce
the same diagram with a different interpretation of its next action; a repeated
board can have different hands, turn or cycle responsibility; a mate threat from
a drop is different from a promoted move. Unknown rules are not automatic draws,
illegal-action losses, proof of mating possibility or engine failure outcomes.

Kagawa/Okano are targeted acquisition paths, not a mandatory all-editions corpus
gate for every possible profile. They are not assumed to contain the missing
answers. A profile may limit its fidelity claim to an identified witness, while
remaining complete semantics must still be supported by additional evidence or
explicit, fully specified owner-approved software adaptations. This checkpoint
does not yet supply those adaptations.
