# MSH-94 independent conduct source/fixture review

Research worker only until root's explicit isolated-module delegation; root remains integrator. No reference adjudication output used as authority. Date2026-10-28. Sources: selected bilingual WXF2018 PDFs already acquired under docs/m4/xiangqi/source-manifest.json, local /private/tmp/msh80-research/xiangqi. OCR locates clauses; material diagram/Chinese readings checked against rendered pixels, including independent review's300dpi Chinese85 image. English and Chinese are jointly governing, without blanket priority.

## Source-to-contract map

|Scope|English PDF/print|Chinese PDF/print|Implementable consequence|
|---|---|---|---|
|19.1-7 move nature|43-44/35-36|27/26|Check, mate threat, chase, exchange, block, offer and idle differ. Mate threat means next checking move or checking sequence forcing mate, not arbitrary evaluation. No geometric attack-only substitute.|
|19.8-12 history/response|44/36|28/27|Perpetual chase targets same persistent piece identity, with opposing response escaping or directly resolving chase. Counter-chase/check retains its nature even when responding.|
|19.13-15 roots|44-45/36-37|28/27|Real root requires actual immediate legal recapture after hypothetical capture. Pinned protector is fake; geometry insufficient.|
|19.16 mixtures|45/37|28/27|Alternating check/mate/chase/idle is different from simultaneous labels on one move.|
|19.17-19 frequency|45-46/37-38|29/28|Two-to-one refers counter-chase frequency, not number of pursuing pieces.|
|20.1 and3.1H|46/38 and14/6|29/28 and10/9|All-check prohibited; mutual all-check equal fault draw. All-check has responsibility priority over opposing chase.|
|20.2 mixtures|46/38|29/28|Mate threat and alternate check/chase/idle families permitted. Missing English listing of alternate mate/chase repaired using WXF's own diagram38, not old AXF import.|
|20.3-5 victim aggregation|46/38|29/28|Same persistent victim, including multiple pursuers, differs from alternately chasing several victims. Uncrossed Soldier victim and General/Soldier pursuer exceptions;2:1 frequency applies actual opposing replies.|
|20.6 roots/exception|46-47/38-39|29/28|Real-rooted targets permitted; fake-rooted prohibited. Horse/Cannon against protected Rook remains prohibited.|
|20.7 reciprocal constraints|47/39|29/28|Same-type free exchanges permitted; pinned Rook/Cannon or leg-blocked Horse may still be prohibited. Native reciprocal legal capture is material evidence.|
|20.8-10 labels|47/39|29-30/28-29|Chase outranks exchange for responsibility; General/Soldier pursuit and their qualifying combinations exempt. Block/offer/exchange/capture-after-check permitted.|
|chapter5 diagram37|66/58|49/48|Black Cannon check/chase mixture permitted; Red Rook chasing same unprotected Cannon prohibited.|
|diagram38|66/58|49/48|Black alternate mate/chase permitted; Red perpetual chase prohibited. Printed initial Rooki6 is setup, after first fourplies root is Rookh6. Use stableh6 root for repeated witness and retain original setup separately.|
|36.4 diagram109|106/98|85/84|Simultaneous mate-threat and chase resolves to chase. Red Horse movement enables Red Cannon capture of Black Elephantg9 by changing screens; this chase is causal although pursuer did not move.|
|36.5 diagram110|106-107/98-99|85/84|Red Cannonh8 continuously attacks Black Rookf8 via Red Horseg8. That persistent unchanged attack is expressly excluded from cycle; moving Red Rook alternates chase/idle and Black Horse is perpetual chaser.|
|chapter5 section20 stationary victim/screens|77-79/69-71|later diagram pages require corresponding exact review|Cannon may perpetually chase stationary target by moving mounts; actor moving mount is not automatically the pursuer. Uncrossed Soldier exception applies.|
|protected on alternating move|62-63/54-55|applicable chapter5 protected-Cannon family|A target real-rooted on one phase does not establish all-move prohibited chase; do not count every capture threat as a fault.|

## Independent proposals and correction ledger

`conduct-fixture-proposals.json` contains12 exact FEN/action witnesses. They were reasoned from rules/pixels; `fixture-local-observations[-v2].json` records native local legality only, not outcome authority. Every proposed action was locally executed on the existing MSH93 native probe; this establishes legal paths, never classifier correctness. Captures/root checks remain independently reasoned facts.

Review corrections preserved inside proposals:
- Horse-versus-protected-Rook first draft had Horse d7 blocking h7 Rook's supposed b7 recapture. Independent reviewer caught it. Replaced with Black protector Rook b9, defending targetb7/b6 across b8. No expected truth was changed to match adjudicator output.
- Diagram109 Rookf8 is Red/unfilled, essential to mate threat; initial draft accidentally Black.
- Diagram110 Cannonh8 and Horseg8 are Red/unfilled, Cannonc4 Black/filled. Initial reverse assignments produced apparent non-collinearity. Correct high-resolution ownership resolves it; no rulebook contradiction claimed.
- Pinned same-type constructive fixture uses RedGenerale0, avoiding unintended d-file check. Stable pin is RedRooka9→BlackRookc9/d9→BlackGeneralf9.

Source cases still requiring additional independent histories before claiming comprehensive classifier conformance: direct General/Soldier pursuer combinations, both-sides equal prohibited chase, multi-target alternation, unrestricted2:1counter-chase family, capture-after-check/exchange/block/offer boundaries and arbitrary concurrent threats. Twelve examples are not a proof of entire chapter5 completeness.

## Isolated implementation plan after delegation

Root explicitly delegated only `crates/mshogi-core/src/xiangqi/conduct.rs` after proposals/review; no task records/other product files edited by worker. Root owns game/record/public dispatch and integration.

1. Validate contiguous legal native Step histories and stable piece identity maps. Native local board code remains authoritative.
2. Establish legal capture pairs with normalized representation counters; compare complete attacker/victim identities before/after move and after opponent reply.
3. Pursuit must be causal (moved pursuer or newly enabled legal capture), and opponent reply must resolve it. Ignore unchanged stationary attacks, retain discovered Cannon/mount threats.
4. Hypothetical capture+native legal recapture determines true root. Reciprocal capture distinguishes free same-type piece from pin/blocked-leg exception. Apply General/Soldier/uncrossed victim and Horse/Cannon-versus-Rook clauses.
5. Intersect prohibited persistent victims across every own move. Any checking/nonchecking mixture fails the all-chase test; checks are not ignored. All-check priority separately; equal prohibited conduct has no one-sided responsibility. Multiple simultaneous persistent victims return explicit Unresolved until reviewed; do not choose arbitrary victim.
6. Permitted mate/block/offer/idle labels need not be mislabeled idle to establish invariant responsibility; no mate-search capability advertised. If a missing distinction could change responsibility, return Unresolved and root preserves unresolved/unscored state.
7. Twelve independent cases, F002 all-check priority, malformed history/id tests; independent completed-module review before acceptance.

Tests initially returned0 because root had not yet registered module; that is not a pass. Actual later execution/results belong in final worker handoff.

## G-NOWIN exact question — historical proposal superseded by owner

**Current owner decision received by integrator while research was underway: option2, existential natural board wins only under WXF3.1A (checkmate and stalemate-as-loss). Perpetual-fault penalty wins, resignation/agreement/clock/admin/invalid-claim/misconduct events are NOT positive proof witnesses. Full game history remains authoritative for actual gameplay, but penalty mechanisms do not create winning capability. MSH105 still blocked on94+95.** The following research proposal is historical evidence, not the accepted contract. F002 C^3 must NOT be used as a Black no-win positive witness.

### Superseded research proposal

The selected record currently expressly gates the quantifier/eligible wins. WXF3.2A EN15/7 and ZH10/9 say neither side can win, without an executable definition. Independent reviewer reads this as existential possibility, not forced minimax. Worker agrees existential is the strongest textual recommendation, while flagging that including history-fault wins needs explicit recording under existing gate.

Proposed definition: S includes board/turn, native ids, complete known history, active warnings, selected claim policy, and prior conclusions. W_R(S) iff some finite sequence of admitted native board actions reaches a Red native win by checkmate, stalemate or selected one-sided WXF continued-fault loss, without crossing an earlier automatic conclusion (including repetition or auto-valid claim). W_B analogous. No-win iff neither existential holds. Exclude voluntary resignations, agreement, administrative sanctions, and the no-win conclusion currently being decided. Optional claims do not generate wins and can be omitted from an existential sequence unless selected policy automatically exercises them.

Do not substitute whether Red/Black can force a win against resistance. No-win false requires a concrete winning sequence, not failure of fast recognition. Unknown is not winnable or draw.

Independent negative: bare Generals d0/f9, FEN `5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1`. No legal action checks/captures an opposing General, no chase victims exist, and each live General retains a vertical move; thus no mate/stalemate/history-fault win. Do not infer a larger defensive-material class without proof.

Independent positive/history domain sentinel: F002 RedRooke9, BlackGeneralf8, RedGenerald0, Red turn, FEN `4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1`; C=`e9f9 f8e8 f9e9 e8f8`. C^2 warns Red; C^3 makes Black win at ply12 under already selected repetition policy, even though Black's lone General has no mating capability. This invalidates mating-only per-side win quantification if history fault is included. Voluntary resignation would make every live state win-capable and is therefore excluded.

MSH105 remains unstarted until task dependencies satisfied; this report is MSH94 source/assessment-contract research only, no exact algorithm/feasibility result claimed.

## Completed bounded module follow-up (supersedes earlier coverage gaps)

The separate reviewer supplied/source-reviewed further facts before execution. Current proposal corpus is **19 histories**, SHA256 `033f92cd2937bbec1f5a9eef4056eea543c372eef43ee8cd07ccd746a0cf6bc3`; native local replay of every prefix records **99 positions** in `fixture-local-observations-final.json`. Tests have19 expected responsibility cases; this corpus is separate from source observations/reference outcomes.

Added independently reviewed scope:
- Actual General-only and crossed-Soldier-only pursuit, plus General+Rook combined pursuit of one crossed Soldier (8plies), all20.9 exemptions.
- Cannon alternately chasing two stationary Horses while opponent moves blockers,20.4; multiple target identities explicitly prevent same-victim fault.
- Free RedHorse perpetually chasing BlackHorse whose reciprocal legs are blocked by two fixed Cannons,20.7; both actual move legs and capture legs individually reviewed.
- Real-rooted pinned Rook: added Advisor e8 can legally recapture on d9 while preserving rank9 pin screen. Source diagram54 EN76/68 and section22 EN81/73 independently confirm root allowance. Removed initial provisional Unresolved for this now settled case.
- WXF36.3 published diagram108 independently transcribed/reviewed equal prohibited chase: Red two Rooks alternate pursuit of same Black Cannon, Black unmoved Cannon uses alternating screens to pursue same stationary Red Rook. Both violate; no one-sided responsibility.
- Existing unrooted Rook/Horse case explicitly proves2:1 **counter-chase frequency**: Horsea6 cannot capture Rookc4, Horsec5 can capture Rooka4 via b5, while Red chases on both moves. This is not two pursuing pieces.

The preliminary speculative root-change-only causality idea was tested against independently fixed diagram108 and rejected: a stationary Rook's continuously available legal capture of the other Black Cannon a8 changes protection when a screen moves, but WXF only counts the actual newly enabled pursuit of i8 in that cycle. Capture-pair creation or movement of actual pursuer establishes the causal attack; changing root classification alone does not relabel a constant legal attack as a new chase. This was a code correction based on normative fixture, not an expected-data edit. Reply establishing a true root may still resolve pursuit, as19.10 and20.6 require.

Current implementation excludes Unresolved for the now source-settled root/pin and multiple-victim categories. It preserves explicit Unresolved only when the witness cannot supply its closing opposing reply or native identities differ across the proposed cyclic boundary. Invariants reject forged/noncontiguous actions/identities.

Actual product command after root registered modules and supplied record.rs:
`cargo test --locked -p mshogi-core --lib xiangqi::conduct -- --nocapture`
**PASS5 test functions**, including19case table, F002 all-check/mixed check+idle, real-root legal recapture, malformed actor/poststate/identities, and explicit2:1 phase assertions. No full workspace/platform/CI result is claimed by this worker; root owns integration checks. Earlier isolated-copy runs are supplementary development evidence, not substituted for this actual core run.

Still do not advertise a mate-threat search classifier or a complete move-nature telemetry API. Responsibility treats permitted nonchase label alternatives conservatively: a nonchecking move with no qualifying chase may be a mate threat, exchange, offer, block or idle; it is not falsely labeled idle. Actual source fixture37/38/109 preserves combined/alternate distinctions relevant to responsibility. Full independent completed-module review remains required before acceptance.

## Final native boundary freeze and warning-prefix correction

Current corpus **20 witnesses /108 local-legal prefix observations**, SHA256 `35b3c938f7bc694c7e4361c298a417d965753d1e56930709f2f425e8635a8fdc`. Added independently paper-reviewed identical-Rook permutation: board/turn returns at8 but a4/a6 Rook ids exchange. Closed-witness classifier projects the already observed first complete action through the **current** identity map for a cyclic local diagnostic; it never restores old ids or appends an accepted future event. Native source validates board/turn closure before even all-check classification; a single ordinary checking prefix is an invariant error, not perpetual check.

Completed `continues_chase_prefix` preserves authoritative warning retirement at actual proof boundaries, not only return-to-anchor. It validates actual contiguous steps without requiring closure. For an unanswered newest pursuit, causal/prohibited victim candidates form an **upper bound**: empty persistent intersection proves variation immediately, nonempty means not yet disproved. No projected future opponent move is used in this prefix path. An actual reply which leaves the threat available also breaks continuous pursuit. Regression from root's T-cycle:
- T=`e9a9 a7e7 a9e9 e7a7`, root RedRooke9, BlackCannona7, BlackGeneralf8, RedGenerald0; FEN `4R4/5k3/c8/9/9/9/9/9/9/3K5 w - - 0 1`.
- T+T establishes Red chase warning at8.
- New Red `e9f9` checks instead: chase no longer continuous at9.
- New Red `e9d9` has no causal chase: continuity broken at9.
- New Red `e9a9` chases Cannon a7: continuity not yet disproved at9; Black `f8f7` at10 ignores that threat, so continuity broken at10 rather than waiting for anchor return12.

`cargo test --locked -p mshogi-core --lib xiangqi::conduct -- --nocapture`: **PASS7 test functions**, actual core, including all source cases and native identity/closure/2:1/prefix regressions. Module final freeze SHA256 `b8b0b01b9eba813205874f737a9b87b8c18408e8dcc67e53b6a01b6ca474de66`, supplied to separate reviewer. Clippy's last worker-run complaint is the newly added helper unused until integrator wires the Game call; no lint suppression was added. Prior owned collapsible-if lint fixed. No final whole-workspace/CI success is inferred.

## Final independent prefix finding resolved

Separate reviewer supplied a further independently constructed native-history counterexample **before** observing implementation behavior:
`5k3/9/9/9/9/3C5/9/4c4/4K4/3n5 w - - 0 1`, cycle `e1d1 e2d2 d1e1 d2e2`.
Red Cannon d4 perpetually chases stationary Black Horse d0 using alternating Red General d1 / Black Cannon d2 screens. The Red General's direct capture of d0 after moving to d1 is **not** resolved by Black's reply, so that incidental General attack does not create20.9's aggregate-pursuer exemption. The actual resolved pursuer is the Cannon.

The prefix upper-bound initially treated a pending General attack as though the future reply resolved it, allowing an unproved negative exemption to retire a valid Cannon-chase warning. Corrected: pending replies may upper-bound **positive** prohibited victim candidates, but may not establish **negative** General/Soldier aggregate exemptions. Those require an actual (or closed-cycle diagnostic) reply resolving that pursuer. Regression verifies the warning after two cycles, continuity retained at unanswered ply9, retained after actual reply10, and Red fault after completed cycle12. Pure General/Soldier and actual combined-pursuer exemption cases still pass.

Final completed module SHA256 **`082e720cdcbccfc22a2d759ffd8468043478140459163e453273728bd51dfecc`**. Actual product command `cargo test --locked -p mshogi-core --lib xiangqi::conduct -- --nocapture` **PASS8 test functions**, including19 category cycles, two-Rook identity-permutation,2:1phase facts, native true-root probes, malformed/nonclosed witness rejection, immediate warning-prefix breaks, and the pending-exemption counterexample. Previous freeze hashes above are superseded development evidence. Separate reviewer notified this exact hash for final assessment.
