# Native Xiangqi history and canonical game contract (MSH-94)

Selected rules: `mshogi.xiangqi.wxf2018.v1`, governing WXF2018 edition;
see [sources](sources.md), [owner decisions](owner-decisions.md) and
[executed evidence](history-evidence.md). This adds a game-owned Rust API to
the existing complete local board contract. It does not add a console, engine,
shared transport, archive or desktop consumer. Those existing task owners remain.

## Admission, history and outcomes

`Game::from_root` preserves the native `PositionRoot`, provenance and declared
counters. Imported/analysis roots establish the beginning of known history;
a diagram is not proof of prior reachability. Root-assigned persistent piece
identities survive moves and are removed on capture. Board repetition equality
uses placement, native owner/type and turn, not counters or interchangeable IDs.
Pursuit uses persistent IDs, so two equal pieces are not one victim.

`GameRequest` binds a complete expected `GameContext`, actor and `GameCommand`.
The context contains root, claim policy, all commands (including rejected claim
audit), and a fresh-instance/monotonic-revision token. Only `Play` requires the
actor to move. Resignation and claimant-specific natural-count requests need
not consume a board ply or belong to the side to move. No caller supplies a
warning or a conclusion. Stale, illegal, concluded, pending and resource-limited
admission are distinct refusals, with no partial mutation.

Events distinguish board actions, warnings/retirements, submitted/rejected/
accepted count claims, draw offers, resignation, agreement, pending adjudication
and conclusions. A native win names its winner and reason explicitly: local
no-action (checkmate or stalemate), continued repetition fault, or resignation.
Draw reasons are repetition, a valid count claim, agreement or a proven
neither-can-win result. Process failure is not a native win/draw reason.

An agreement offer is admitted after that actor's own move, with no outstanding
offer and at most once in that after-own-move window. The opponent may accept
or decline; a subsequent board action clears it. There is no immediate offeror
withdrawal. WXF17.3H prohibits repeated offering (EN PDF40/printed32, ZH
PDF25/printed24); this specific window is an explicit minimal untimed software
admission adaptation, not a source-prescribed numeric harassment rule. No clock,
foul or tournament penalty is invented.

## G-CYCLE witness and responsibility

The selected third-warning/fourth-continuation policy uses the entire contiguous
known interval from an anchor's first occurrence to its third/fourth occurrence.
It does not restart a count for each action word or cherry-pick matching cycles
from mixed return paths. This is the recorded software witness policy within
the selected profile, not a claim that WXF uniquely specifies an anchor algorithm.
Each board/turn anchor has its own warning. A fourth-occurrence loss needs the same
active side/fault/anchor obligation; otherwise the selected fourth-occurrence draw
applies. Actual earliest terminal events end play; a later anchor cannot overwrite
an earlier outcome. Captures make the old board anchors unreachable and retire
their obligations.

Responsibility is a consequence-relevant quotient of WXF19/20/chapter5, not a
public universal move-nature telemetry API. All-check takes priority; mutual
all-check is equal fault. Chase uses causal native legal capture/recapture
facts, opponent responses and persistent victims. A moved pursuer or newly
enabled discovered attack counts; an unchanged stationary attack does not.
Real/fake roots, reciprocal same-kind capture and pinned/blocked pieces,
Horse/Cannon versus protected Rook, General/Soldier combinations, uncrossed
Soldier targets, changing victims and actual two-to-one counter-chase frequency
retain their native consequences. Simultaneous mating-threat/chase uses chase;
alternation is different. Permitted mate-threat/offer/exchange/block/idle and
capture-after-check families have no independent prohibited consequence. Their
absence of fault is not mislabeled as an exact Idle or mate-search observation.

A changed action sequence alone does not prove variation. A check obligation
retires on its owner's nonchecking action. A chase obligation retires only on
proved broken pursuit; a missing opposing response cannot prove that. The
prefix helper retains an upper bound on possible continuing victims and never
invents an unanswered General/Soldier exemption. Same-type identity swaps use
current identities when diagnosing the cyclic response, without adding a future
accepted event. Original warnings and explicit retirement events remain in the
ledger. Invariant failures reject atomically; an unresolved result-affecting
classification pauses explicitly, unscored, rather than becoming a draw.

## Claims and serialized precedence

For claimant C, with L known capture-free plies and Q actual checks by C in
that window, eligibility is `L - max(0, Q - 10) >= 100`. The opponent's checks
are not subtracted. Capture resets the known window; imported declared counters
do not establish missing checks. Eligibility queries are advisory. A manual
invalid claim preserves semantic state but, if admitted within capacity, records
its submission and rejection. No untimed penalty or forced board action follows.

`Explicit` preserves optional claims. `AutoValidClaims` records automatic
submissions/acceptance, checking Red then Black after board adjudication. Order:
local board terminal first, repetition second, proven neither-can-win third,
then automatic valid claims. External commands are serialized in receipt order.
No command overwrites a conclusion. A valid unexercised optional count claim
cannot displace a later repetition outcome retroactively.

## Owner-selected natural-win assessment

Goal: `mshogi.xiangqi.natural-board-win.exists.v1`. A positive witness consists
only of ordinary admitted board actions reaching WXF3.1A checkmate or
stalemate-as-loss. This is existential, not minimax. Repetition/conduct penalties,
resignation, agreement, time, misconduct, administrative failures, invalid claims
and the no-win draw itself are excluded. Full known history still governs actual
match events; it cannot create intrinsic natural winning capability through a
penalty. F002's Black repetition award is therefore not a Black capability proof.

MSH-94's `assess_natural_win` returns a sound bare-two-Generals impossibility
proof, a genuine zero/one-ply natural-win witness, `UnknownExactUnavailable`, or
failure. Bare Generals cannot check/capture across disjoint palaces; in every
admitted different-file placement there is a legal vertical exit. The exhaustive
108-position invariant test checks that scoped proof and successors. No larger
material heuristic is silently substituted. A miss is not proof of possible
mate/stalemate and does not conclude a draw. MSH-105 owns the required bounded/
terminable exact backend and feasibility evidence after MSH-94 and MSH-95;
it remains unstarted here. No general exact-proof performance claim is made.

## Canonical record, undo and bounded saveability

`GameRecord` schema1 is `mshogi.xiangqi.game`: explicit game/profile/edition,
`untimed`, claim policy, `sound_fast_only`, goal and fast witness limit1, complete
root, commands/results/event ends, every derived event, redundant current native
position, conclusion, pending state and target-specific assessment. Runtime
execution tokens are not canonical data. Unsupported modes, limits, identities
or schemas are refused, not downgraded.

Typed strict decoding rejects duplicates/unknown fields before normalization.
Both root and redundant current-position JSON remain `RawValue` through the
native parser. Every command/result/event prefix is authoritatively replayed.
A final equality/presence check (after those strict parsers) rejects omitted
nullable fields and forged derived metadata while allowing key order/whitespace.
FEN alone never restores this record. Reopen allocates fresh runtime identity;
a saved record is not resumable pending execution.

`event_prefix` is immutable historical inspection, including separate board and
same-ply warning/conclusion states. It is not a new resumable command boundary.
Semantic LIFO undo restores exact native state and canonical history, while
runtime revision never rewinds, and newly admitted branches receive fresh
command lineage IDs. Returning to the same board
or record cannot revive old requests or undo handles from a different reopen.

Admission constructs a private candidate and measures its complete encoded
record before committing. Bounds are512 commands,2048 events,1MiB encoded
native JSON. Nonterminal admission reserves one command, four events and4096
bytes for native closure. Audited rejected claims consume capacity too. Exhaustion
refuses new work while preserving the entire current saveable record and a close
path; no accepted history is dropped. At511 rejected claim commands, command512
cannot be another audit but can be resignation, and both records reopen.
Later shared envelope/archive owners must measure their own additional overhead
and preserve MSH-61; this native limit is not approval of future desktop limits.
