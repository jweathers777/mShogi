# Selected Xiangqi rules profile — XIANGQI-P1

**Selected by John on 2026-10-27: `mshogi.xiangqi.wxf2018.v1`.**
Game identity `xiangqi`; immutable governing edition **World Xiangqi Rules2018**,
published by WXF as an official **bilingual Chinese/English publication**.
The [owner decision record](owner-decisions.md) supersedes the
[historical unselected proposal](profile-proposal-before-xiangqi-p1.md), including
its fourth-occurrence warning and language-precedence suggestions. The filename
is retained so existing task/document references remain valid.

Selection itself was a planning/profile contract, not an implemented capability.
Current implementation: MSH-91/92/93 board rules and MSH-94 native history,
canonical records and sound fast assessment; see [history contract](history-contract.md)
and [evidence](history-evidence.md). MSH-105 now supplies the explicit bounded
read-only [native exact path](exact-verification.md); product consumers remain
outstanding, and this is not general ordinary-play WXF completeness. The selection-stage claims below are preserved history.
Native schema and transport versions remain separately identified. No public
profile constant, native rule implementation, runtime reference or UI support
is introduced by selection. Existing Chess/Shogi identities remain unchanged.

## Authority and explicit interpretation method

The [source reconciliation](sources.md) identifies exact official PDF hashes,
clauses/pages, differences and evidence limits. There is **no blanket Chinese
precedence**. Each discrepancy is resolved by comparing both wordings, surrounding
WXF clauses and official examples, with predecessor AXF or subsequent official
WXF interpretation where relevant. Selected repairs are recorded individually.
Unknown questions stay named and block their affected implementation; comparator
helpers do not settle them. AXF/CXA corpus acquisition is not a blanket gate to
this selected WXF profile or unaffected board work. Other profiles need separate
names, source editions and owner decisions.

## Repetition, warnings and continued conduct

1. Repetition equality is the same native piece types/owners on all intersections
   and the same side to move. Known root counts as occurrence1. Declared counters,
   warning events and runtime identifiers do not change this board equality;
   actual piece identity remains available to classify pursuit/history.
2. At the **third occurrence**, identify/classify the repeating cycle. If WXF's
   responsibility rules identify one side with a unilateral prohibited perpetual
   obligation, commit the native requirement/warning to vary with its witness,
   responsible side and classification. Warning does not consume board ply.
3. If that side continues the prohibited cycle and produces the **fourth
   occurrence**, conclude its loss. If no one-sided prohibited obligation exists,
   the fourth occurrence produces the applicable WXF draw, including properly
   classified equal prohibited conduct. Equal fault is a WXF category, not a
   Boolean test that both sides have some prohibited label: all-check versus
   opposing chase still has the WXF checking-side responsibility.
4. This selected software interpretation reconciles threefold prohibited-conduct
   classification/warning with the separate four-occurrence draw provision.
   Do not use a generic chess repetition rule or a program's repetition score.
5. **G-CYCLE**, now resolved in [MSH-94 contract](history-contract.md): the original
   gate required witness selection
   for differing/overlapping return paths, simultaneous anchors, retained/retired
   warnings and the same victim across paths using independently reviewed WXF
   examples. The third/fourth boundary is settled; these classification cases
   must not silently default to draw/loss. Reviewed full contiguous witness and
   warning-prefix fixtures now establish this boundary. They do not affect MSH-91's restricted
   board corpus or MSH-92's state/action/reversal scope. The preserved F002 seed
   supplies concrete C/Q overlapping histories for that gate.

## Untimed natural-movecount requests and event ordering

Initial local play is **untimed**. Advisory eligibility and committed requests
are separate. A native participant may request an explicit manual/desktop claim
at a serialized command boundary; terminal-command actor admission is distinct
from board-action turn validation. Under Art8 count100 total no-capture plies,
deducting only the requesting side's checks above10 in that capture-free window.
No pawn-progress reset, inferred pre-import checks or shared global check
subtraction. Supplied counters remain declared; insufficient known history cannot
prove eligibility. Exact100 is eligible under the individually reconciled source
interpretation. No untimed five-minute penalty or simulated technical foul.

Manual mode is `explicit`. Unattended matches **may select** the named recorded
`auto_valid_claims` policy: after each accepted nonterminal move, validate both
claimants natively and exercise a valid claim, recording claimant, witness and
policy; Red-first provides a stable tie-break if both qualify. Automatic exercise
is an explicit match policy, not a different core draw rule or a mandatory mode.

An invalid admitted manual claim is rejected atomically: board, side, move prefix,
counters, warnings and game conclusion stay unchanged. Retain the request and
rejection reason in a separate bounded authoritative command/audit record, not
as an accepted game action or a fictitious foul outcome. Pre-admission reserves
space for this result; resource-limit refusal stays distinct from invalid claim
and preserves the complete current save path (MSH-61/99). Repeated invalid
requests cannot consume unbounded memory or erase accepted observations.

Selected serialized ordering (software event policy, not an invented physical
arbiter procedure):

- A concluded game cannot be overwritten by a later resignation, claim or reply.
- Validate an incoming board action atomically, then resolve immediate native
  no-action outcome: checkmate or stalemate is a loss. This has precedence over
  a repetition/count request generated afterward at that boundary.
- Resolve the selected repetition result (one-sided continuation loss or WXF
  draw) next; a warning is an event, not a concluded game. Resolve a soundly
  proven3.2A impossibility draw next. A request requiring expensive/unresolved
  proof enters the bounded path; unknown alone never concludes a draw or loss.
- In still-live state process explicit commands in host receipt order:
  resignation, agreement offer/acceptance, manual natural-count claim or other
  supported native request. No event retroactively changes a prior conclusion.
  Apply `auto_valid_claims` only after the move's automatic conclusions, before
  admitting the next external command; no evaluation-based adjudication.
- If an experiment genuinely needs an incomplete exact result to adjudicate,
  retain an unresolved/unscored state. Cancellation, resource exhaustion and
  engine/process failure are execution observations, never invented game results.

Physical touch-move, arbiters/appeals, tournament clock/foul sanctions, pairings
and scoring remain outside this initial local software profile. Later timed/event
policies require explicit names; no runtime clock behavior is implemented here.

## WXF3.2A: required two-level verification

The impossible-for-either-side-to-win condition remains required M4 behavior:
**MSH-94** owns the native assessment contract and fast sound recognizer;
**MSH-105** owns the separately bounded or independently terminable exact path
beyond it. MSH-84 native handoff, MSH-103 full review and MSH-45 completion all
wait for105. Required completeness is not deferred to M5.

Native assessment distinguishes **proven impossible for both**, **proven at
least one side can win** (with the verifier's actual witness/proof), and
**unknown/incomplete**. Failure of the fast recognizer or exhaustion/cancellation
of exact work proves neither side of the question. Record mode, limits, exact
target/history/provenance and accepted proof/unknown observations. Bound replay
and reconstruction too; unsupported restoration is explicit.

**G-NOWIN resolved by John during MSH-94:** use the existential natural-course
board-win question, identifier `mshogi.xiangqi.natural-board-win.exists.v1`.
A positive witness is a finite admitted ordinary board-action continuation to
WXF3.1A checkmate or no-legal-action/stalemate loss. It is not a forced-win test.
Exclude repetition/prohibited-conduct penalties, resignation, agreement,
time/lateness, administrative/technical faults, invalid claims, misconduct and
the no-win draw itself. Article3 separates natural board losses from later
penalty clauses. Full history still governs actual game outcomes; a penalty
award cannot prove intrinsic natural winning capability. F002's possible Black
repetition award is specifically not a positive Black natural-win witness.
See the [explicit owner resolution](owner-decisions.md#g-nowin-resolution-during-msh-94-2026-10-27).
The earlier recommendation to include repetition awards was rejected. Exact
algorithm, proof/resource evidence and MSH-105 delivery remain required, not
claimed by this semantic decision. A missed fast proof remains unknown.


## Complete native contract / clause coverage


This is the selected complete delivery scope, not a claim of implemented or
executed conformance. The fixture ledger distinguishes concrete seeds,
uninstantiated source families and decision-dependent expected outcomes.

| Area / governing clauses | Native obligation and consumer implication |
| --- | --- |
| Setup, board, sides —1 | Ninety **intersections**, nine files/ten ranks; river between ranks4/5, palaces d–f/ranks0–2 and7–9 in the Red-bottom diagnostic coordinate convention. Red/Black native sides, Red first,32 starting pieces. Geometry metadata represents lines, intersections, palace diagonals and river; no chess-square semantics. |
| Pieces and complete actions —2 | General/king orthogonal palace step; advisor diagonal palace step; elephant two-diagonal with eye blocker, own half only; horse orthogonal leg then diagonal destination; chariot orthogonal sliding; cannon quiet sliding or capture over exactly one screen; soldier owner-relative forward and sideways after crossing river, never backward. Capture removes one destination victim. No promotions, hands, drops, castling or declaration is imported. Complete action is native origin+destination; derived capture/check effects are rules-owned. |
| Board legality —2/3 | Reject own-piece landing, malformed/off-board requests, moving wrong side, exposing own general, and unobstructed facing generals. General capture is not a playable continuation API. Check evasions must enumerate **all** legal choices, including removing/adding cannon screens and horse-leg interpositions. Empty legal set loses whether checked or not; retain mate versus stalemate reason. |
| History classifications —19/20 and5 | Preserve complete known actions and identity of relevant pieces. Classify all-check, mating threat, chase, exchange, block, offer, idle and mixtures; same victim versus changing victims; real/fake protection, pinned/blocked same-type targets, cannon/horse against protected chariot, own king/soldier and uncrossed victim exceptions, two-to-one counter-chase frequency, overlapping labels and responsibility precedence. No attack-set-only substitute. |
| Match events —3/8/19/20 | Native resignation, agreement offer/accept, count claims, warning/refusal under XIANGQI-P1, board-terminal outcomes and neither-can-win3.2A with sound fast recognition and the bounded exact path. Event actor/turn semantics belong to native adapter; board-ply checks must not reject legitimate terminal commands merely because actor is not side to move. Availability is not conclusion. Preserve claim denials, unresolved assessment, failures and cancellations as different categories; invalid claims preserve semantic game state while the separately admitted request/rejection audit is retained. |
| Imported roots and identities | Structural diagram admissibility differs from proven historical reachability. Preserve root provenance, declared counters, unavailable prefix, capture/check/pursuit history and warning state. Separate board equality, rules-history identity, saved-document identity and search reuse identity. Repetition starts with known root; no invented earlier occurrence. |
| Exact transition/reversal | Validate atomically, apply complete action/events once, undo exact board/material/side/counters/history/classification. Semantic undo changes game state, never revives old engine execution identities. Canonical commands and accepted events replay exactly. |
| Notation / records | Selected diagnostic coordinate dialect: files a–i from Red's left, ranks0–9 from Red baseline, e.g. `a0a1`; name/pin dialect explicitly. ICCS/UCI/UCCI/Chinese notation are adapters, **not a native canonical record**. FEN-like import must pin piece alphabet, active-color meaning, unused fields/counters and strict parsing; preserve unsupported fields or reject, never guess. Pure board FEN cannot restore full history. Schema/version/strict duplicate rejection and encoded size limits use the existing native-envelope boundary. |
| Native engine and evaluation | M4 delivers own functional evaluator/search using native full context, local legality and selected history/event policy. Red/Black score perspective explicit; no borrowed Chess Full operation or Shogi declaration. Requests/results bind purpose, profile/native record, exact target prefix, request/session/process identity and limits. Board action, committed claim, conclusion, incomplete search and failure remain tagged distinct results. |
| Console, runner and desktop | Own console diagnostics and bounded interoperability; external reply checked by native adapter and runner owns selected adjudication. Common accepted mShogi dark shell and stable controls/sidebar. Xiangqi-specific intersection SVG/river/palaces/pieces, whole board visible at supported normal/minimum sizes, native read-only enemy inspection. Preserve click/keyboard/orientation, analysis provenance, cancellation, history/live distinction, Save/Open and lossless capacity admission. |

## Planning completion and implementation boundary

XIANGQI-P1 settles the material profile choices for the first native queue.
The [coverage ledger](coverage.md) assigns every required area and residual gate;
the [fixture plan](fixtures.md) retains independently reasoned expectations and
separates executable future evidence from planning. G-CYCLE/G-NOWIN remain
blocking **affected history/adjudication work**, not hidden missing requirements
or blanket prerequisites for the board/kernel batch. No native rules begin in
MSH-80. After reviewed planning completion, MSH-91 and MSH-92 are Ready;93
follows both. Complete delivery remains MSH-45, shared all-game/platform closure
MSH-46/47, and measured strength MSH-54 in M5.


## MSH-94 implementation status

The current [native history contract](history-contract.md) resolves G-CYCLE and
records the consequence-relevant classification/admission software policy.
[History evidence](history-evidence.md) traces actual fixtures, review and tests.
The planning-boundary text above is preserved history; G-NOWIN semantics now have
the explicit owner resolution above. MSH-94 does not implement the required
exact backend105 or any future Xiangqi consumer. Complete M4 delivery still waits
for the recorded graph, including105; MSH-45/M4 remain open.
