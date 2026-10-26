# Selected standard shogi profile

Status: **approved profile contract; implementation coverage is tracked separately**.
Owner John explicitly selected option A, `SHOGI-P1`, on 2026-10-26 against
integration `6db8500e92129b95e502e7b4d5c4a26072e38652`. Selected immutable identity:
**`mshogi.shogi.csa36.v1`**. Approval includes initial untimed local play,
advisory eligibility versus committed declarations, atomic malformed/stale/illegal
request rejection, and the declared SFEN counter/bounded-known-history semantics.
Options B/C remain separately named unselected alternatives, outside this batch.

The original MSH-62 proposal and independent planning reviews remain in Git and
in their historical review records. Approval does not certify implementation or
close MSH-41. The [clause coverage ledger](shogi-coverage.md) identifies the exact
fixture owners and deferred research gates.

## Recommendation and authority

The selected contract uses standard even-game shogi movement under the dated Japan Shogi
Association (JSA) rules, with the explicit software competition conclusions of
the Computer Shogi Association's 36th World Computer Shogi Championship
(CSA WCSC36, 2026). This is a **CSA competition convention**, not a claim to
reproduce JSA professional match procedure. It gives an independently specified
answer for no legal move, fourfold repetition, declaration and a finite game
limit. Choosing it does not require a CSA network adapter, tournament manager or
all tournament administration.

All English descriptions below are translations/paraphrases or proposed
software semantics, not official English quotations. Japanese originals govern
source interpretation. Every source was inspected on **2026-10-26**; undated
webpages are identified as such. Source editions do not advance silently when a
URL changes.

| Label | Primary source and edition | Critical locators |
| --- | --- | --- |
| JSA-R | [JSA 対局規則, PDF](https://www.shogi.or.jp/match/taikyoku_rules/taikyoku_rules.pdf), enacted 2024-06-07, revised 2024-10-01 and 2025-01-01, corrected 2025-10-01; 14 PDF pages | Printed pp.1–7, Arts.2–6: board/play; p.8 Art.8: repetition; pp.8–9 Art.9: entering king; pp.9–11 Art.10: fouls; p.13: revision footer. PDF page index is printed page number here, because index0 is the unnumbered cover. |
| JSA-E | [2025-10-01 correction](https://www.shogi.or.jp/match/taikyoku_rules/taikyoku_rules_revision.pdf), one page | Corrects Art.9(5), condition4's cross-reference to Art.9(4)(i); retains 31/24 thresholds. |
| JSA-I | [Current official rules index](https://www.shogi.or.jp/match/taikyoku_rules/), retrieved edition headed 2025-10-01 | Entry point to JSA-R/JSA-E; HTML is incomplete, notably omitting Art.10(2–7). Prefer the dated PDF. |
| JSA-B1 | [JSA 本将棋: placement](https://www.shogi.or.jp/knowledge/shogi/01.php), undated | Initial-position diagram and coordinate explanation. |
| JSA-B2 | [JSA 本将棋: moving/capturing/dropping](https://www.shogi.or.jp/knowledge/shogi/02.php), undated | Five common principles; promotion on leaving the camp. |
| JSA-B3 | [JSA 本将棋: piece movements](https://www.shogi.or.jp/knowledge/shogi/03.php), undated | Eight piece headings and promoted movements. |
| JSA-B4 | [JSA 本将棋: check/mate](https://www.shogi.or.jp/knowledge/shogi/04.php), undated | Opening definition: play ends at mate; actual king capture is unnecessary. |
| JSA-B5 | [JSA 本将棋: prohibited play](https://www.shogi.or.jp/knowledge/shogi/05.php), undated | Nifu excludes promoted pawns; pawn-drop mate versus pawn-move mate; exposing one's king. |
| JSA-FAQ | [JSA rules FAQ](https://www.shogi.or.jp/faq/rules/), undated | Entering-king question: amateur 28/27 declaration; resignation/late-foul answers differ from current PDF default. |
| JSA-N | [JSA notation FAQ](https://www.shogi.or.jp/faq/kihuhyouki/), undated | Basic-notation paragraph distinguishes promotion/nonpromotion and explicitly illustrates forced pawn promotion. Not a canonical record format mandate. |
| CSA-36 | [CSA WCSC36 tournament rules, PDF](https://www.computer-shogi.org/wcsc36/rule.pdf), effective/revised **2023-11-27**, used by [WCSC36 (2026)](https://www.computer-shogi.org/wcsc36/menu.html); 12 pages | PDF pp.1,12: edition/effective date; p.8 Art.22: score; p.9 Art.24: clocks; p.10 Arts.25–27: declaration, finalization, losses/repetition; p.11 Art.27: 320-ply limit. Page references in this row are 1-based PDF pages. |

CSA Art.6 requires rule-following shogi, but does not restate every movement.
The selected profile explicitly names JSA-R and its beginner clarifications for that
part of the contract. CSA Arts.25–27 govern conclusions where they intentionally
differ from JSA-R. Software adaptations below are identified separately.

## Position and complete board actions

Coordinates here use files 1–9, ranks 1–9 from the sente diagram; sente moves
toward rank 1, gote toward rank 9. Initial state is 9×9, forty unpromoted pieces,
empty hands, sente to move. Sente: king 59, rook 28, bishop 88, gold 49/69,
silver 39/79, knight 29/89, lance 19/99, pawns 17,27,…,97. Gote is the 180-degree
rotation with ownership reversed. This is an even game; handicaps, tsume
problem conventions and try rules are outside this profile. See JSA-B1 and
JSA-R Art.3.

Movements relative to the moving side are:

| Piece | Movement | Promotion |
| --- | --- | --- |
| King | Any adjacent square | None |
| Gold | Adjacent forward, forward diagonals, sideways, backward straight | None |
| Silver | Adjacent forward and four diagonals | Gold movement |
| Knight | Two forward and one sideways; jumps | Gold movement |
| Lance | Forward ray | Gold movement |
| Pawn | One forward; captures the same way | Gold movement (tokin) |
| Rook | Orthogonal rays | Add adjacent diagonals (dragon) |
| Bishop | Diagonal rays | Add adjacent orthogonals (horse) |

Rays stop at the first occupied square; own occupancy prevents landing.
Only the knight jumps. A turn moves one owned board piece, optionally promoting
it, **or** drops one piece from the mover's hand onto an empty square. Capture
replaces an enemy nonking on the landing square and adds its unpromoted base
kind to the capturer's hand. Ownership and promoted state therefore matter
independently; capturing a tokin yields a pawn, not a gold. JSA-B2/B3 and JSA-R
Art.5 supply the movement/drop contract; the base-kind hand representation is
the explicit software interpretation of reuse without promoted drops.

For an unpromoted promotable piece, promotion is available when its move starts
or ends in the opponent's three-rank camp, including movement wholly inside
and movement out. Both promotion choices remain available where legal.
Pawn/lance on the last rank and knight on the last two ranks must promote;
unpromoted drops there are prohibited. This follows the movement geometry plus
JSA-R Art.10(1)(ii), with JSA-N's forced-promotion example. Drops cannot promote;
an already promoted piece stays promoted until captured. JSA-B2/JSA-R Art.5.

JSA-B5 makes these restrictions explicit: no second **unpromoted friendly pawn**
on a file (nifu); no pawn drop delivering immediate mate (uchifuzume); no move
leaving or placing one's king attacked. A pawn drop giving an escapable check
is legal; moving an existing pawn to mate is legal; mating drops of other
piece kinds are legal. Pawn-drop legality must consider legal defenses,
including captures/interpositions and their king safety. A nonchecking pawn
drop is not uchifuzume. No chess double pawn move, en passant, castling or
chess stalemate draw is imported.

King capture is absent from the legal action set and kings never enter hands.
Mate is check with no legal response; the last legal move concludes play.
JSA-B4 supplies the distinction between threatening capture and actually
capturing the king. Under the selected **CSA** outcome rule, a side with no
legal board action loses even if not checked (CSA-36 Art.27(1)(i)); retain a
diagnostic distinction between mate and other no-action loss. Position queries
alone do not establish repetition, declaration, clocks or a concluded match.

## History and game conclusions in the selected bundle

The following compact clause map is the source contract. A `ply` is one
completed board action, not a pair of moves. Custom imports follow the explicit
counter/history contract below; a supplied counter is a declared input, not
proof that its preceding moves occurred.

| Topic | Governing clause / selected application |
| --- | --- |
| Repetition identity | JSA-R8(1): board + **each owner's** hand + side-to-move. Include piece ownership/promotion; exclude elapsed time and move number. Initial state is an occurrence. |
| Four occurrences | CSA-36 27(2): final draw, no replay. A first prior occurrence is insufficient. |
| Continuous checking | CSA-36 27(2): checker's loss if all its moves between first and fourth occurrence give check. Store the whole interval; the repeating position may be checked or check-relieved (JSA-R10(10)). This overrides ordinary repetition draw. |
| No legal action | CSA-36 27(1)(i): loss, with or without check. |
| Resignation | JSA-R6(1): opponent wins. A match command, not a board action. |
| Declared entering king | CSA-36 25: own turn; king in enemy camp; ≥10 other owned camp pieces; no check; camp+hand points ≥28 sente / ≥27 gote; time remaining. Rook/bishop=5, others=1, king=0, regardless of promotion. |
| Declaration procedure | CSA-36 25(2): explicit declaration; not automatic on crossing a material threshold. A formal declaration failing any prerequisite loses. Advisory eligibility queries do not declare. |
| Length | CSA-36 27(3): 320 plies → draw only absent another loss or repetition; no-action loss on ply320 takes precedence. No JSA500 checking extension is added. |
| Scoring | CSA-36 22: draw=0.5 win/0.5 loss. Store the game reason independently of that competition score. |

Formal declaration and resignation preserve the board and finish the game.
They do not increment the board-action counter or invent a pass move. Undo in
analysis must restore history and match commands as well as pieces. Terminal
game records preserve profile identity, outcome reason, declaration evidence
and the history establishing repetition. A screenshot or SFEN board alone is
not sufficient match evidence.

### Imported roots and the 320-ply rule

The following deterministic software semantics are part of the SHOGI-P1 bundle
approved in **SHOGI-P1 / MSH-63**, not an additional open owner question:

- Preserve a supplied positive SFEN next-ply number `n` as declared input. The
  completed-action counter at that root is `n - 1`; each committed board action
  adds one. Store the exact origin and whether that number was supplied or
  defaulted. If a position-entry adapter permits an omitted counter, its explicit
  default is `n = 1`; this does not relax a strict SFEN parser's grammar.
- An imported diagram creates a new known history segment whose root counts
  once for repetition. Earlier occurrences and check history are **unknown**,
  not reconstructed from `n`. A continued diagram game records that limitation.
  A canonical saved-game load instead reconstructs and validates the recorded
  root, actions, commands and outcomes, including their counter provenance.
- Analysis may inspect a structurally valid diagram even at or beyond the game
  limit, while exposing its match classification. Starting/continuing live play
  from a diagram applies no-action loss first and then the length conclusion:
  `n >= 321` is already terminal by the declared counter and admits no further
  board action. At `n = 320`, the next action completes ply320 and triggers the
  normal loss/repetition/length ordering. Do not silently reset the number to
  gain playable turns. Resuming post-limit analysis as a new live game requires
  a separately recorded new root and counter.
- The counter establishes eligibility for this software length convention only;
  it establishes neither historical reachability nor whether an unrecorded
  earlier game should already have ended. Do not label a diagram continuation
  as a fully recovered original match. A missing prehistory also prevents any
  earlier repetition result from being claimed as verified.

MSH-65 can preserve the declared position counter without implementing game
adjudication. MSH-67 implements the above game/history semantics only after
the selected bundle has been ratified in MSH-63. MSH-65 verifies supplied position counters; MSH-67 owns defaulted/reconstructed
counter and known/missing game-history fixtures.

The source interval definition, not a reference engine's search-cycle
heuristic, governs continuous-check fixtures. If all own moves are not checks,
ordinary repetition applies. Boundary cases include the fourth occurrence
after the defender's evasion and a hand transfer that prevents equality.
A locally admissible board action completing continuous-check repetition is
recorded with the checking side's loss; generic invalid-request rejection must
not erase that history-dependent consequence. Label position-only move queries
accordingly, and test the game transition separately.
Possible overlapping repeat windows or a hypothetical cycle where both sides
continuously check require a separately reasoned fixture, rather than guessing
from a single hash match. Such fixture research does not change the selected
fourfold convention.

## Software application versus tournament conduct

These are approved software boundaries, not additional shogi moves:

- A malformed/stale request, illegal board choice, invalid imported position or
  eligibility probe is rejected atomically. It does not commit a tournament
  foul or award a win. This follows the accepted A-02 API contract and needs no
  new owner decision. Nifu/uchifuzume/self-check remain absent from legal moves.
- An explicitly committed **formal declaration** is a different command from a
  harmless eligibility probe. It has the CSA losing consequence if its rule
  conditions fail. Transport/schema/stale errors are rejected before admitting
  that command. The UI must make this distinction reviewable.
- A match runner may record an opponent's illegal committed engine action,
  formal false declaration or timeout as a competition loss under a named
  runner policy. An analysis failure, canceled search or invalid internal result
  is not automatically a player loss. Keep transport failures and playing
  results distinct; CSA's specific disconnection penalties are event policy.
- Initial local untimed play is an explicit software mode: no clock expires and
  the declaration time prerequisite is satisfied while the game is live.
  Timed matches must name their timing policy. CSA-36 Art.24's actual event is
  15 minutes plus 5 seconds per own turn, second truncation and timeout priority
  even over mate. Do not silently apply chess clocks or this event clock to
  untimed play. Timed shogi support requires separately scoped authorization of
  a named timing policy and its exact boundary fixtures; this profile does
  not promise or silently authorize an unidentified future clock task.
- No replay is scheduled for a CSA repetition/length draw. Analysis restart,
  rematch and tournament pairings are separate operations. A training or rating
  export must retain whether it used the CSA score convention.

CSA-36 Arts.26–28 distinguish referee decisions, server decisions, finalization
at announcement of pairings/results, and accident handling. Those are not
properties of a board position. Full human tournament officiating, appeals,
physical clock-button conduct and retrospective arbitral result changes are
not represented by legal move generation. An external competition result can
be attached with provenance without rewriting the accepted legal action path.
An implementation claiming a tournament-management mode must scope that work
explicitly rather than claim this playable profile simulates every official.

## Actual owner choice: SHOGI-P1

**Selected: option A**, by the explicit owner approval recorded above. The table
preserves the original alternatives for traceability; only A is this contract.

| Option | Meaning and consequence | Evidence |
| --- | --- | --- |
| **A — selected: `mshogi.shogi.csa36.v1`** | JSA board rules with CSA36 conclusions and the software boundaries above: 28/27 declaration, no-action loss, fourfold draw, 320-ply draw, distinct competition score. Practical fully specified computer-play baseline; does not claim JSA professional match procedure. | JSA-R/B1–5/N; CSA-36 Arts.22,24–28 |
| B — JSA professional convention, separate future profile ID | 24-point agreed impasse; declaration ≥31 wins /24–30 replay; fourfold and impasse generally require side-swapped replay. Requires the additional procedural questions below to be settled before accepting the match profile. Different outcomes from A are intentional. | JSA-R Arts.8–10; JSA-E |
| C — named amateur/other event convention | Valid only with a pinned organizer ruleset for conclusions, repetition and scoring. JSA-FAQ's 28/27 declaration is evidence that another convention exists, not a complete event profile. A try-rule service would need its own named profile and is not a substitute for standard declaration. | JSA-FAQ; JSA-R9(4)(iii) permits specified 27-point agreement variants |

Options B/C are recorded alternatives, not simultaneous first-batch features.
The selected complete profile supplies M4's standard-shogi closure contract;
M4 does not require every unselected tournament convention. Research or
implementation of another profile requires separate authorization. Any later
profile addition must preserve existing records' chosen convention.

## Conditional JSA-only questions and source conflicts

These do **not** block option A. They explain why copying a generic phrase such
as “official shogi rules” is insufficient:

1. **Repetition procedure:** JSA-R8(2–3) permits agreement before four and
   continuing after four, until return to a repeated state. An automatic
   fourth-occurrence immutable draw/replay result is a software adaptation,
   whereas CSA27(2) expressly uses final draw. A JSA profile must choose and
   document recognition/confirmation/continuation, replay linkage and clocks.
2. **Impasse procedure:** JSA-R9(3–6) has agreement, unilateral declaration and
   500-ply provisions in an entering-king context. It gives no universal
   position-local mating-impossibility algorithm. Research must resolve whether
   the 500 provision is global or contextual, its overlap precedence, and its
   continuous-check extension before coding an unconditional 500 draw. An agreed
   impasse counts all owned board+hand material; declaration counts only
   camp+hand. Neither is simply both kings entering the camp.
3. **Late fouls/finalization:** JSA-R10(2–6) permits retroactive loss, an event
   resignation-priority override, a next-game cutoff and exceptions for late
   time/takeback allegations. JSA-FAQ says resignation prevails. The dated PDF
   governs its baseline; do not silently merge the FAQ exception. A JSA
   tournament-management claim needs an explicit event and audit workflow.

The located JSA-R/B4 text does not explicitly settle the rare **no-check,
no-legal-action** case by a separate stalemate clause. CSA27(1)(i) does, which
is part of option A's recommendation. Do not fabricate a JSA citation or reuse
chess's draw rule. Captured promoted-piece base-kind normalization and attack
semantics for pinned defenders should receive independently reasoned fixtures;
these formalization checks do not authorize alternative rules by convenience.

## Acceptance handoff

MSH-63 records the actual SHOGI-P1 selection and focused independent review. MSH-64 pins retained
source bytes/checksums and independent expected positions; this research
inspected live primary sources but does not claim an archived byte snapshot.
MSH-64–66's source/board/state/legality work can proceed independently of the
bundle decision; outcome/history work in MSH-67 depends on MSH-63. Conformance
must separate board legality from
history/conclusion/command/timing/runner evidence. Pinned external engines are
bounded comparison tools, not rules authorities. The full standard-shogi scope
remains MSH-41; this profile and a passing kernel test cannot close it.

Current MSH-67 evidence: the [native game/history contract](history-state.md)
implements the selected untimed conclusions and strict canonical game/match
records. [The independently reviewed impossibility argument](mutual-check-resolution.md)
resolves the mutual-check research obligation for admitted live roots without
adding an outcome or changing this profile. The original incomplete review,
source research and failed proof repairs remain traceable. Exact implementation
verification/integration is recorded in [history validation](history-validation.md)
and the task record. Later engine, console, shared and desktop consumers remain
separate delivery obligations.
