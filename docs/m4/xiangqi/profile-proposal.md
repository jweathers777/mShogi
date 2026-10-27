# Proposed Xiangqi profile and decision boundary

**Status: unselected, incomplete pending the named decisions/research below.**
Candidate identity **`mshogi.xiangqi.wxf2018.v1`**, game `xiangqi`, governing
edition **WXF2018**, with separately documented software-event adaptations.
No profile constant, public schema or supported capability has been added.
An immutable rules-profile ID, native record schema and transport version will
remain different identities. Rules changes require a different profile/version;
a wire wrapper version cannot change rules. Current delivered identities for
Chess and Standard Shogi are untouched.

## Material owner choices — not implementation defaults

The recommendation is a coherent **option A** comprising XQ-P1–P3 below.
It is a proposal requiring explicit selection, not inferred approval from the
instruction to research MSH-80. Alternatives retain separate names and scope.

| Decision | Recommended option A | Material alternatives / reason |
| --- | --- | --- |
| **XQ-P1 authority and language** | WXF2018 international profile; use the Chinese clauses to resolve the specifically documented English discrepancies, with bilingual diagrams and explicit software supplements. Retain the whole applicable code, including check/chase and3.2A. | CXA2020 would require authoritative fulltext/amendments first; an exact AXF edition likewise remains unverified. An English-only threefold/two-pursuer interpretation contradicts the paired source. WXF publishes no located controlling-language ruling; choosing this precedence is a product convention. |
| **XQ-P2 repetition / referee adaptation** | On the **fourth occurrence including the known initial position** (three completed returns), classify the repeating interval. If both/neither offend, conclude the appropriate draw. If one side owes change, commit an explicit warning event identifying the side, anchor position and offending cycle/classification. If the next completed return to that warned anchor preserves that side's prohibited repetition, commit refusal and its loss; a break of that cycle retires that warning. Warning and refusal are canonical match events, not illegal board moves. No hidden frontend warning state. | A direct loss on the fourth occurrence would remove the warning step and is a different software adaptation. A three-occurrence rule would be another profile, not silently borrowed from an engine. The proposed full-extra-cycle refusal trigger is precise software policy, **not asserted verbatim tournament law**. Complex/overlapping cycles still require an exact interval/anchor tie-break (R-CYCLE) and independently reviewed fixtures before acceptance; this candidate is not yet a complete deterministic classifier. |
| **XQ-P3 natural-count and untimed event policy** | Initial local play untimed. Native/core and initial desktop use recorded `explicit` claims; advisory eligibility is separate from a committed request. A valid player request may be submitted by either named participant at a serialized command boundary; it need not consume a board ply. Count100 no-capture plies, subtract only that claimant's checks above10 since the last capture/known root. Core does not invent a claimant for arbiter initiative. Initial unattended runner uses recorded `auto_valid_claims`: after each accepted nonterminal move check both claimants and commit a valid claim (record claimant; stable Red-first order if both eligible). No automatic count outcome in `explicit` mode. Invalid/malformed/stale requests reject atomically; replace physical clock/foul penalties with an explicit request error. Accept immediate board terminal outcome before a later claim request. | Exact physical referee/clock sanctions require a separate timed/event profile. Automatic natural-count adjudication without a recorded claimant changes the claimant-specific rule. Source “regardless of position” does not itself settle software event ordering; board-terminal-first and removal of sanctions require approval. No evaluation-based draw or resignation policy is introduced. |

Recommendation A also includes the following necessary software boundaries:
imported diagrams begin known history, supplied counters remain declared rather
than proof of earlier checks/captures, missing context remains explicit, and a
history-dependent conclusion cannot be asserted from a FEN alone. A sound proof
may establish an outcome; incomplete/resource-limited adjudication is unknown
and unscored, distinct from illegal action and process failure. Full required
rule coverage remains a delivery obligation; a limitation is not evidence that
either side can win. Physical touch-move, arbiter appeals, tournament scoring,
event pairings and over-the-board penalties are outside this local software
profile. This does not delete their source provenance or turn other tournament
codes into equivalent profiles.

**Unresolved research, not an invitation to guess:** R-EDITION and R-NOWIN in
[sources](sources.md). R-NOWIN may reveal an additional material choice; bring
the concrete source/fixture conflict back then. No full-profile or conformance
claim until required gaps and choices are resolved. Executed implementation
fixtures are not required for the planning decision, but independently reviewed
expectations and complete normative coverage are.

## Proposed complete native contract / clause coverage

This is the scope to implement after ratification, not a claim all definitions
are already fixture-complete. The fixture ledger distinguishes concrete seeds,
uninstantiated source families and decision-dependent expected outcomes.

| Area / governing clauses | Native obligation and consumer implication |
| --- | --- |
| Setup, board, sides —1 | Ninety **intersections**, nine files/ten ranks; river between ranks4/5, palaces d–f/ranks0–2 and7–9 in the proposed Red-bottom coordinate convention. Red/Black native sides, Red first,32 starting pieces. Geometry metadata represents lines, intersections, palace diagonals and river; no chess-square semantics. |
| Pieces and complete actions —2 | General/king orthogonal palace step; advisor diagonal palace step; elephant two-diagonal with eye blocker, own half only; horse orthogonal leg then diagonal destination; chariot orthogonal sliding; cannon quiet sliding or capture over exactly one screen; soldier owner-relative forward and sideways after crossing river, never backward. Capture removes one destination victim. No promotions, hands, drops, castling or declaration is imported. Complete action is native origin+destination; derived capture/check effects are rules-owned. |
| Board legality —2/3 | Reject own-piece landing, malformed/off-board requests, moving wrong side, exposing own general, and unobstructed facing generals. General capture is not a playable continuation API. Check evasions must enumerate **all** legal choices, including removing/adding cannon screens and horse-leg interpositions. Empty legal set loses whether checked or not; retain mate versus stalemate reason. |
| History classifications —19/20 and5 | Preserve complete known actions and identity of relevant pieces. Classify all-check, mating threat, chase, exchange, block, offer, idle and mixtures; same victim versus changing victims; real/fake protection, pinned/blocked same-type targets, cannon/horse against protected chariot, own king/soldier and uncrossed victim exceptions, two-to-one counter-chase frequency, overlapping labels and responsibility precedence. No attack-set-only substitute. |
| Match events —3/8/19/20 | Native resignation, agreement offer/accept, count claims, warning/refusal under XQ-P2, board-terminal outcomes and neither-can-win3.2A. Event actor/turn semantics belong to native adapter; board-ply checks must not reject legitimate terminal commands merely because actor is not side to move. Availability is not conclusion. Preserve claim denials, unresolved assessment, failures and cancellations as different categories; invalid commands do not mutate canonical history. |
| Imported roots and identities | Structural diagram admissibility differs from proven historical reachability. Preserve root provenance, declared counters, unavailable prefix, capture/check/pursuit history and warning state. Separate board equality, rules-history identity, saved-document identity and search reuse identity. Repetition starts with known root; no invented earlier occurrence. |
| Exact transition/reversal | Validate atomically, apply complete action/events once, undo exact board/material/side/counters/history/classification. Semantic undo changes game state, never revives old engine execution identities. Canonical commands and accepted events replay exactly. |
| Notation / records | Proposed internal diagnostic coordinate dialect: files a–i from Red's left, ranks0–9 from Red baseline, e.g. `a0a1`; name/pin dialect explicitly. ICCS/UCI/UCCI/Chinese notation are adapters, **not a native canonical record**. FEN-like import must pin piece alphabet, active-color meaning, unused fields/counters and strict parsing; preserve unsupported fields or reject, never guess. Pure board FEN cannot restore full history. Schema/version/strict duplicate rejection and encoded size limits use the existing native-envelope boundary. |
| Native engine and evaluation | M4 delivers own functional evaluator/search using native full context, local legality and selected history/event policy. Red/Black score perspective explicit; no borrowed Chess Full operation or Shogi declaration. Requests/results bind purpose, profile/native record, exact target prefix, request/session/process identity and limits. Board action, committed claim, conclusion, incomplete search and failure remain tagged distinct results. |
| Console, runner and desktop | Own console diagnostics and bounded interoperability; external reply checked by native adapter and runner owns selected adjudication. Common accepted mShogi dark shell and stable controls/sidebar. Xiangqi-specific intersection SVG/river/palaces/pieces, whole board visible at supported normal/minimum sizes, native read-only enemy inspection. Preserve click/keyboard/orientation, analysis provenance, cancellation, history/live distinction, Save/Open and lossless capacity admission. |

## Acceptance boundary

The profile can be selected only with recorded owner decisions and resolution
of required research semantics. Selection does not prove algorithms, references
or fixtures pass. MSH-80 is unfinished at this handoff; native implementation
is gated. MSH-45 closes only after every original criterion, including its own
functional engine, console, canonical replay and actual desktop, has evidence.
MSH-54 owns measured playing strength in M5. MSH-46/47 retain all-game and
three-platform closure; an extra Xiangqi task does not close them.
