# Modern JCSA Chu Shogi playing profile

**`mshogi.chu.jcsa2019.v1`**, edition
**`JCSA-2019-09-14+2004-06-06+MSH82-2026-11-02`**.
Ratified modern rules under MSH-82; no native Chu implementation or executed
Chu conformance is claimed here.

John's 2026-11-02 direction selects faithful modern JCSA play, capture-based
admission, a narrow immediate royal-threat mate predicate, native enforcement
before fourth repetition, and the narrow material rule with immediate responses
and agreed draws. [Decision record](decisions.md) distinguishes that direction
from withdrawn proposals. No unrestricted forced-mate search, automatic
repetition forfeiture, general bare-king award or quiet immobility loss is added.

Authority: **C1**, JCSA general rules, 2019 edition corrected2019-09-14;
**C2**, JCSA supplement updated2004-06-06, for compatible detailed cases;
**C3**, movement/setup companion revised2004-04-22. [Sources](sources.md) retain
exact URLs, inspection and hashes. C2 supplies explicit exception precedence;
it does not silently supersede C1. H1 is the previously inspected1915 witness,
not an original1703 facsimile. T1 is the association's Okazaki transcription:
use compatible movement corroboration, not its superseded Lion interpretation.
Lishogi is a separately identified modern alternative, not an oracle. No new
historical acquisition or external-engine dependency is required.

## C01 — board and original identities

12×12 squares, files12→1 left→right from Sente's view, ranks1→12 top→bottom.
Sente starts and moves toward decreasing ranks; Gote is the180° owner-exchanged
position. Captures are discarded; there are no hands or drops. Forty-six pieces
per side,92 total. In the following Sente rows, columns are files12 through1:

```text
12 L  FL Cu Si G  K  DE G  Si Cu FL L
11 RC .  B  .  BT Ki Ph BT .  B  .  RC
10 SM VM R  DH DK Ln FK DK DH R  VM SM
 9 P  P  P  P  P  P  P  P  P  P  P  P
 8 .  .  .  GB .  .  .  .  GB .  .  .
```

Rows7/6 are empty; Gote occupies the rotation, including GB files9/4 on rank5.
In particular Sente King7十二/Elephant6十二 and Lion7十/FreeKing6十 are
asymmetric. [Machine setup and piece table](profile.json) specify all92 squares.
C1 I/III and actual C3 setup agree with H1p214.

Retain native original/base identity and a once-only promotion bit. Twenty-one
initial kinds plus18 promoted-origin states give39 identity states but only29
movement faces, as counted by C1. A Pawn promoted to Gold cannot become Rook;
native Gold can. Go-between→Elephant cannot become Prince. There is no ownership
change or demotion on capture; preserve the discarded victim's original identity
in effects/history. King/Prince are native royal identities, not generic Chess
king metadata. Imported diagrams establish structure, counters and known history,
not historical reachability; source-derived inventory bounds do not prove reachability.

## C02 — ordinary movement

Offsets `(x,f)` below mean sideways and mover-forward; sideways signs are paired.
A step is exactly one square. A ray requires an empty intervening path, stops
before friendly occupancy and may capture its first enemy. A jump ignores and
does not capture intermediate occupants. No friendly capture or off-board action.
Let O=orthogonal four, D=diagonal four, A=all eight adjacent directions.

| Face (native label) | Steps | Jumps | Rays | Promotion of native original |
| --- | --- | --- | --- | --- |
| P 歩兵 | forward | — | — | Gold |
| GB 仲人 | forward/backward | — | — | Elephant |
| L 香車 | — | — | forward | White horse |
| RC 反車 | — | — | vertical | Whale |
| Cu 銅将 | forward three, straight backward | — | — | Side mover |
| Si 銀将 | forward three, backward diagonals | — | — | Vertical mover |
| G 金将 | O + forward diagonals | — | — | Rook |
| FL 猛豹 | forward/backward three | — | — | Bishop |
| BT 盲虎 | A except forward | — | — | Flying stag |
| DE 酔象/醉象 | A except backward | — | — | Prince |
| K 玉将/王将, Pr 太子 | A | — | — | none |
| Ki 麒麟 | D | orthogonal two | — | Lion |
| Ph 鳳凰 | O | diagonal two | — | Free king |
| SM 横行 | vertical | — | horizontal | Free boar |
| VM 竪行 | horizontal | — | vertical | Flying ox |
| R 飛車 | — | — | O | Dragon king |
| B 角行 | — | — | D | Dragon horse |
| DK 龍王 | D | — | O | Soaring eagle |
| DH 龍馬 | O | — | D | Horned falcon |
| FK 奔王 | — | — | A | none |
| WH 白駒 | — | — | forward three + straight backward | none |
| W 鯨鯢 | — | — | backward three + straight forward | none |
| FS 飛鹿 | horizontal + D | — | vertical | none |
| FO 飛牛 | — | — | vertical + D | none |
| FB 奔猪 | — | — | horizontal + D | none |
| SE 飛鷲 | restricted Lion below | restricted Lion | six rays excluding forward diagonals | none |
| HF 角鷹 | restricted Lion below | restricted Lion | seven rays excluding straight forward | none |
| Ln 獅子 | full Lion below | full Lion | — | none |

C3's actual diagrams, not engine results, establish these categories. C3's
compound directions and H1 observations are cross-checked separately; no Tori
Phoenix, Eagle, quail or promotion rule carries over merely by an English name.

## C03 — complete compound actions

Lion may take one adjacent King step; jump directly to any of the16 outer-ring
squares at Chebyshev distance2; or take two adjacent King steps, each landing
legally under evolving occupancy. Either leg may capture; up to two distinct
victims. The second step may turn, finish adjacent to the start, or return to it.
The start is vacated, so capture-return removes the adjacent enemy and restores
the Lion to its original square. A quiet return requires an actually empty,
in-board first landing; it cannot pivot on a friendly/enemy piece without
capturing the enemy. A surrounded initial Lion has no quiet return. No null
zero-route pass, friendly overflight-and-return or off-board pivot. C1 VIII/C2§4.

SE has this power only along either single forward diagonal; HF only straight
forward. On that chosen direction it may step once, jump two, take both forward
steps or step and return. It cannot turn to another diagonal or sideways on the
second leg. Their other directions are the rays in C02. A direct jump over a
friendly **or enemy** intermediate square preserves it; the two-leg version
captures an enemy intermediate. C3 diagrams/prose and compatible T1 movement.

One choice commits one whole turn. Preserve origin, ordered actual landing route
and promotion decision; effects are native-derived ordered removals/movement.
Do not accept client victim lists. Two paths sharing endpoints can have different
victims; return-to-origin is not an empty action. Retain even effect-equivalent
chosen paths in records. Repetition uses its own semantic equivalence in C07.
All legality, effects, royal survival and conclusions are assessed on the
complete action; the frontend never commits a partial leg. Actual two-royal
capture can remove both royals in one action. No promotable compound mover
exists: initial Lion cannot promote; Kirin-Lion/SE/HF have already flipped once.

## C04 — promotion

Enemy camp is ranks1–4 for Sente,9–12 for Gote. Every promotion in C02 is optional
and once only. An original promotable piece may flip after entering from outside
to inside; or after capturing in camp or from camp while leaving. A quiet move
wholly inside, or quiet departure, does not renew promotion. Leave and re-enter
to regain an opportunity. An entirely out-of-camp capture grants none. Resolve
using the starting face, derive capture, then flip, then evaluate the final state.

Pawn has an additional optional last-rank opportunity after declining earlier;
Lance has **no** such ordinary-play rescue, and Go-between has none. C1 IV.5's
Lance rescue is correspondence-only, not automatically selected by software.
Unpromoted Pawn/Lance on their farthest rank remain dead and occupy/block their
squares. Promotion is not compulsory. Pawn/Go-between must be promoted before
they can be the qualifying material extra in C08. C1 IV/C2§1.

No persistent invented 'declined promotion' flag is needed: native base/promotion,
current origin/destination/capture and current camp determine these opportunities.
Imported in-camp identity does not invent a prior quiet-entry entitlement. Preserve
known history and provenance separately. The C1 VII Kirin capture flips only at
the end and can create a defended Lion's next-turn protection.

## C05 — Lion capture law

Only actual Lion movement faces (native Lion or promoted Kirin) receive the Lion
exchange rules; SE/HF Lion-like movement does not make them Lions.

1. Lion may capture an **initially adjacent** enemy Lion irrespective of defense.
2. A distance2 Lion cannot capture a defended Lion, including a sliding defense
   revealed when the attacking Lion vacates its origin. Evaluate the complete
   action, not a newly adjacent intermediate subturn.
3. A qualifying intermediate enemy capture, other than unpromoted Pawn or
   unpromoted Go-between, permits the accompanying defended-Lion capture. Another
   Lion qualifies. This exception outranks next-turn protection. After Lion takes
   Lion, an enemy non-Lion can immediately recapture; no new protection arises.
4. After a non-Lion captures an enemy Lion, its own defended Lion(s), including
   a capturing Kirin that promotes and is defended, cannot be captured on the
   opponent's immediately following turn. Undefended Lions gain no window.
   Protection expires after that actual opponent turn; a rejected request does
   not consume it. Qualifying extra capture can override it under point3.

Defense is actual capture influence in the relevant hypothetical final geometry,
including hidden sliding defense, not a Chess pinned-piece/self-check test. Retain
the original defended-distance restriction when a first-leg Pawn/Go-between is
the sole defender and is removed: this does not manufacture the exception.
The five C2 diagram examples, C1 hidden-defense pair, Kirin/new-Lion direct probe
and expiry fixtures disambiguate these clauses. Remote royals/pawns and promoted
Kirin origins added to complete source diagrams are labeled fixture construction.

## C06 — royals, immediate mate and no-action

Capture-based movement: a King/Prince may enter attack or ignore a threat.
There is no Chess self-check/pin filter, mandatory check announcement or twenty-
move immunity. Losing King alone does not lose while Prince remains, and vice
versa. After the complete action, loss of the last royal concludes the opponent's
win before any material or repetition processing. Further game actions reject.
C1 II.1–2/III and the actual C1 IX Prince-recapture diagram establish survival.

For C1's mate conclusion, use a deliberately narrow immediate test, not a
search-forced-win theorem. A side with **one** remaining royal must face an actual
admitted one-action capture threat. Enumerate its complete legal responses. A
response escapes if it wins by actual last-royal capture, obtains an independently
established non-mate terminal result, or leaves no single admitted next opponent
action capturing **all** its surviving royals. Thus promoting Elephant to Prince
may escape even if separate enemy moves attack each royal. Do not require that
one of two royals be individually unattacked. The predicate does not recursively
call mate on replies and never searches an unbounded mating tree. Existing
two-royal positions continue to actual capture/survival rather than being
declared mated by a one-king filter. C1 does not publish this algorithm; it is the
explicit finite implementation of its immediate mate wording, selected by the
owner's capture-based/narrow-mate direction. Complete capture/mate fixtures and
Prince rescue must independently verify it in86.

No actual royal threat plus no complete board action is **not** a sourced loss
or draw. Report native `no_board_actions` with unresolved/unscored status;
resignation and permitted agreement commands remain available. This records the
source's silence without inventing a Chess outcome or hiding a control failure.
A separately applicable sourced material result still applies. No engine fallback,
evaluation resignation, move-count draw or general insufficient-material proof.

## C07 — native must-vary before fourth recurrence

Use full known history, not a diagram counter. A repeated position includes native
placement/origin/promotion, owner, side to move and active Lion protection. Physical
piece serials, runtime IDs and declared move counter are not repetition identity.
For C1's same-sequence qualification, compare the intervening sequence of complete
**turn-boundary repetition-position transitions**, excluding history, derived
obligations and runtime state from that projection. Distinct effect-equivalent empty routes do
not escape repetition; both quiet returns remain `jitto`. Full selected routes
are still distinct and losslessly retained in actions/records. Captures and flips
are irreversible in Chu and cannot occur inside a repeated unchanged-material
cycle; no inter-leg board is a separate occurrence. C1 VI.1, VIII/C2§4.

At the third matching occurrence with two matching intervening cycles, derive a
prospective fourth-cycle obligation. Record its witness, responsible side(s),
expected continuation and progress. Responsibility follows C1:

- The express both-only-quiet-return case (VI.1d/VIII) requires the first passer
  to vary, even if a stationary threat persists; this is its specific exception.
  The first passer is the first actor in the actual continuous both-pass streak
  in known history, not the first actor of a rotated cycle anchor.
- Otherwise continuous checker(s) of a **sole surviving royal** must vary (VI.1a).
  An attack on only one of two royals is not this check category. Unlike(b), (a) does not
  require a sole side; if both continuously check, both are constrained.
- Otherwise, if only one side delivers attacks in the repeated variation, it
  must vary (VI.1b). A moved piece's resulting capture influence counts, including
  continued influence on the same victim; a newly revealed stationary attack
  also counts. Unchanged incidental influence of a piece not moved does not
  transform every unrelated quiet move into an attacking move.
- Otherwise the side to move at the repeated boundary must vary (VI.1c).

Preserve legal prefixes. Reject the responsible side's **last continuing action
before** the prospective fourth occurrence, including every effect-equivalent
route, not necessarily its first action after occurrence3. Rejecting only the
closing mover is wrong when responsibility belongs to the other player. An
actual deviation breaks that prospective cycle; independent overlapping cycle
obligations remain enforced. Do not commit the fourth occurrence, append a
forfeit, silently draw, or delete history. A rejected request changes nothing.
Quiet4-ply fixture: accept moves9/10, reject Sente's move11, before Gote would
close move12. Both-jitto: reject the first passer's next jitto, including an
alternative empty path. This is a bounded native history/admission algorithm,
not a new outcome rule. Legal-choice queries/search use the same obligations.

## C08 — narrow exhaustion and agreement

Apply after complete moves, following actual royal survival and the sound mate
predicate. For counting **only**, ignore dead last-rank unpromoted Pawn/Lance;
they remain on the board and can block movement. The express narrow award is
one side with a sole royal and the other with exactly two nondead pieces, at
least one royal and a qualifying extra that is not unpromoted Pawn/Go-between.
Additional King/Prince can be the small extra under C1 III. Live Pawn/Go-between
are not erased from the count: K+Gold+Pawn versus K is not the narrow three-piece
case. Larger armies continue ordinary play. C1 II.4–5, III, IX.

If the bare side is now to move and can immediately capture the stronger side's
last royal or equalize to one royal each, permit that actual complete reply
instead of awarding prematurely. Availability alone is neither draw nor win.
Equalizing capture leads to an agreement-eligible exhausted position. Choosing
a different reply which leaves the threshold awards the stronger side after
that move. With no exception reply, award immediately. A qualifying imported
root uses the same explicit current-turn test; do not invent a preceding move or
reply entitlement. The C1 IX sequence (Elephant→Prince; enemy Lion captures King;
Prince recaptures Lion) is the authoritative timing fixture.

Draw is by **mutual agreement** under II.6. IX's resulting bare-royal diagram
establishes the exhaustion case for that agreement, not an unstated automatic
draw event. Bare-royal status, offer, opposing acceptance, refusal and concluded
draw are distinct. Record each command and its actor/context; neither evaluation
nor an unsupported mating-possibility theorem supplies agreement. Agreement may
record both parties' material-exhaustion judgment; only their actual consent
concludes it. Resignation by either player while live loses without a board ply.
Physical touch-move, discretionary foul forgiveness and tournament procedures
are outside this untimed digital profile; invalid/stale/illegal inputs reject
atomically rather than manufacture a physical-event forfeiture.

## C09/C10 — state, reversal and canonical replay

Native Chu owns bounded structure validation, original identity and promotion,
ordered complete actions/effects, all royals, Lion window, complete known history,
cycle witnesses/progress, command actors/agreement and conclusions. Distinguish
diagram validity, declared counters, known history and proven reachability. No
FEN/SFEN/USI/CSA claim for this game. An imported counter supplies no earlier
occurrences or Lion protection; supplied explicit native root rights/provenance
must be validated, not guessed from a diagram.

Internal apply/undo restores all logical state/caches exactly. Semantic Undo,
Take back, history selection and reopened documents never revive retired execution
IDs. Canonical records contain explicit game/profile/edition/native schema,
root/provenance, commands and exact ordered routes. Recompute captures, Lion
rights, cycles and conclusions by native replay; duplicated serialized facts are
checked, never trusted. Duplicate fields must reach the strict native parser
through RawValue dispatch, before any map can erase them. Resource/parse/illegal/
unsupported/incomplete failures remain distinct and unscored.

## C11 — real consumers and verification boundary

[Handoff](handoff.md) preserves86(native),78(shared) and43(functional product)
ownership. Current execution envelope/context/controller/document/interaction
owners suffice with explicit Chu dispatch. Add only native-specific state,
actions/effects, royal/status metadata and a12×12 renderer; no universal rules
framework or second frontend rules engine. Functional search is full-history,
bounded and cancellable; playing strength remains52/M5. Exact-context, purpose,
session/request/process generation and work limits remain acceptance authority.

Console must preserve complete paths and native terminology. Use unambiguous
file1..12/rank a..l diagnostics; human notation can display JCSA numeric-file/
Japanese-rank and ordered captures, explicit promotion/decline and native piece
names. Canonical actions are authoritative; short notation never loses a path
or substitutes a rules parser. Finished UI uses actual traditional Chu kanji,
both orientations and complete board at normal/minimum supported sizes, without
board scrolling/zoom. Retain shared shell and exact two-player Moves/event mapping.

The [independently expected corpus](fixtures.json) precedes implementation.
Its validation/review is not execution of native rules.86 must provide actual
adapters, complete LegalAll comparisons, transition/replay/undo, properties,
valid180° transformations, adversarial checks and hand-established perft/divide.
78/43 must separately execute real consumer/product obligations. External engines
are optional investigations under [conformance policy](../../testing/conformance-policy.md).
