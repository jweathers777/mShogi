# Two selected modern Tenjiku playing profiles

MSH-83,4November2026; selected under the owner's instruction to support choosing
options1or2. Ratified after independent source/rules/fixture and data/contract review. The two whole
profiles are selectable **when starting a new game** and immutable thereafter:

| Selection | Profile identity | Edition |
| --- | --- | --- |
|1 Adams1999 Modern | `mshogi.tenjiku.adams1999-modern.v1` | `ADAMS1999-MODERN+MSH83-2026-11-04` |
|2 Muller, explicitly completed | `mshogi.tenjiku.muller-completed.v1` | `MULLER+ADAMS-COMPLETION+MSH83-2026-11-04` |

These are not independent switches or claims of historical consensus. Profile2
uses Muller's Tenjiku movement/effect description, **explicitly supplemented** by
Adams's general setup/promotion/history/ending contract where M gives none. A
means Adams main chapters1/3; M means Muller's Tenjiku page/table; S identifies
our narrow deterministic completion. Sources/precise differences are in
[sources.md](sources.md). AppendixA does not silently override A's main rules.
[Machine-readable data](profile.json), [fixtures](fixtures.json), [handoff](handoff.md).
No native rules, engine, console or product implementation exists from83.

## T01 — board, sides, setup and identity

16files×16ranks of squares,156pieces/78per side,36 original kinds and44 movement
faces. Files16→1 left→right from Sente; ranks a→p top→bottom; Sente moves first
and toward decreasing rank. Gote rotates directions180°. Fixture coordinates are
[file,rank]1..16, not a promised Rust/public notation encoding. A §1.4 pp6–7 gives
the exact initial table; `profile.json.initial_board` lists all156 placements.

Sente's rows left→right, files16→1:

```
p L N FL I C S G K DE G S C I FL N L
o RC . ChS ChS . BT Ky Ln FK Ph BT . ChS ChS . RC
n SSo VSo B DH DK WBf FiD LHk FEg FiD WBf DK DH B VSo SSo
m SM VM R HF SE BGn RGn GGn VGn RGn BGn SE HF R VM SM
l P P P P P P P P P P P P P P P P
k . . . . Dg . . . . . . Dg . . . .
```

Gote is the complete180° rotation/owner exchange, not a horizontal mirror.
Original King9p/Elephant8p, Lion9o/FreeKing8o, LionHawk9n/FreeEagle8n,
GreatGeneral9m/ViceGeneral8m distinguish central asymmetry. All other squares empty.
Original/promoted identity remains separate from movement face; original
unpromotable FEg/LHk coexist with promoted FK/Ln of the same movement.

## T02 — every movement face

The44 source-diagram entries in `profile.json.pieces` are normative for profile1,
with the profile2 overrides below. Offsets are Sente-relative `(file_delta,
rank_delta)`; F=(0,-1), B=(0,+1), L/R=(±1,0), D=all4 diagonals, O=orthogonals,
A=all8 directions. Rotate for Gote. A steps/rays/jumps legend p11/PDF22 governs:

- Step reaches the listed adjacent square. A ray stops before friend or at first
  enemy capture; nothing beyond. A jump ignores intermediate occupancy without
  capturing interveners. Limited ray1..2 is straight, blocked at1 and captures
  only its chosen final victim, not a two-capture action.
- K/CP stepA; G stepsO+forwardD; DE stepsA exceptB; BT stepsA exceptF;
  FL stepsF/B+D; S stepsF+D; C stepsF/B+forwardD; I stepsF+forwardD;
  Dg stepsF+backwardD; P stepsF. N jumps(±1,-2).
- R raysO; B raysD; FK raysA; DK raysO+stepsD; DH raysD+stepsO;
  RC raysF/B; L raysF; VM raysF/B+stepsL/R; SM raysL/R+stepsF/B;
  FO raysF/B+D; FB raysL/R+D; WH raysF/B+forwardD; W raysF/B+backwardD;
  MGn raysF+backwardD; FS raysF/B+stepsL/R+D.
- Ky stepsD+jumps2O; Ph stepsO+jumps2D. WBf raysL/R+D+limited2F/B;
  ChS raysF/B+D+limited2L/R; VSo rayF+limited2L/R+stepB;
  SSo raysL/R+limited2F+stepB.
- Ln and LHk, HF/SE, FiD/VGn, FEg, HT and ranked generals use T03–T05.

There are no hands or drops. Captured/burned pieces are permanently discarded;
royals are never transferred to a hand. No source supports importing Shogi drop,
self-check, impasse or pawn-drop-mate rules. These common clauses are A-sourced
supplements for profile2; M's linked Chu introduction does not import Chu's rules.

## T03 — complete routes and special movement

A complete choice includes original square, every **actual** visited landing,
any stationary capture target, and promotion choice. Native resolution supplies
captures/effects. No client victim/cause list. Distinguish a direct jump from a
two-leg path with the same endpoints; never conflate route identity with endpoint.
Repeated landings may be represented when these rules allow them. Partial UI
entry is not a partial native turn. Intermediate occupancy is evaluated after
preceding leg effects; origin is vacated while moving and can be revisited.

**Lion:** A §3.7 p19. One adjacent King step; direct jump to any Chebyshev-distance2
square; or two adjacent King steps with independent directions. Each actual
landing may capture one enemy. Friendly occupied landing rejects, including on
an intermediate leg. Return to origin is permitted; capture-return is igui.
Empty return is a complete pass-like route, subject to T09. There is no JCSA Chu
Lion-exchange/protected-Lion restriction. **Modern Lion Hawk**, both profiles,
adds Bishop rays to this complete Lion ability; A Traditional area2 is not selected.

**Horned Falcon:** ordinary raysB/L/R+D; restricted Lion movement only alongF:
one step, direct two-square jump, two forward steps, or first step then return.
**Soaring Eagle:** ordinary raysO+backwardD; restricted Lion movement along either
one forward diagonal with the same straight-two/return choices. Second leg does
not turn onto a different diagonal. Capture may occur on both landings. A pp24–25;
M describes these as Chu movements, with direction detail corroborated by its
linked movement explanation, not imported protected-Lion law.

**Area3:** FiD and VGn can take one, two or three adjacent King steps, changing
direction. Every nonfinal landing must be empty; capture ends the action. No
jump over an occupied intermediate square. Revisiting a vacated square, including an Area3 return to origin, is a
route, not an extra victim (**S**, explicit path interpretation where the source
does not separately discuss area passes). FiD also has ordinary vertical/diagonal rays; VGn
also has ranked diagonal capture-rays. A pp11–14, M area explanation. No burns
occur merely because an intermediate square is visited (T04 S).

**Free Eagle:** profile1 A p21: Queen rays plus direct orthogonal distance2 jumps;
its diagonal open-circle marks are redundant unobstructed travel, not Lion power.
Profile2 M: Queen rays plus one/two adjacent diagonal steps with free choice of
second diagonal and capture at each actual landing, including return/igui.
There is no added profile2 orthogonal jump through a blocker. Overlapping
single-landing identical choices are emitted once; distinct routes remain distinct.

**Heavenly Tetrarchs:** cannot land on any adjacent square. May instead capture
one adjacent enemy while staying put: stationary `igui` target, **not** an actual
landing on that forbidden square. Profile1 A p23: diagonal rays starting distance2,
ignoring only the adjacent square's occupant, plus sideways distance2 jump or
sideways distance3 if distance2 is empty; no vertical movement. Profile2 M:
the same sideways2/3 and igui, with skipped-adjacent rays in both vertical and
all diagonal directions. After the skipped square, ordinary ray blocking/capture
applies. The sideways3 path skips1 but cannot cross occupied2. A stationary
igui has no movement path or implicit promotion; HT cannot promote.

## T04 — Fire Demon and deterministic ordered effects

Both profiles: vertical and diagonal rays (not horizontal rays) or Area3.
At completed-action final position, an **already-Demon mover** burns adjacent
enemies of all kinds except stationary enemy Fire Demons. Friendly pieces are
not burned. Capturing an enemy Demon normally on a landing is permitted; removal
of that Demon removes its aura before the final hazard test. Any mover ending
adjacent to a surviving enemy Demon dies. No global periodic burn of unmoved
pieces or recursive death-triggered explosion exists. An enemy Demon can thus
kill a moving Demon while surviving itself. More than one enemy Demon still
causes one mover-destruction event.

**Profile1 A §1.2.4:** active Demon burns enemy non-Demon bystanders **before**
its own demise beside an enemy Demon. **Profile2 M footnote:** if the moving
Demon dies, it produces no bystander burns. All such differences remain native,
not renderer or evaluation settings.

Resolve one atomic action using these phases; S labels serialization/underspecification
completion rather than claiming the source prints an algorithm:

1. Admit route/target and promotion choice; derive ordinary captures in actual
   landing order. Do not conclude when an intermediate royal disappears.
2. Determine final-position surviving enemy Demon hazards after direct captures.
   No intermediate-landing aura burn (**S**, complete-turn interpretation of end
   of move/destination; source is ambiguous on Lion component wording).
3. Active mover's burn batch: all enemy non-Demons adjacent to final square,
   using profile1 always/profile2 only if mover survives. Determine victims from
   one common phase snapshot, remove them simultaneously. Canonical listing sorts
   by absolute rank then file (**S**, serialization only, no gameplay priority).
4. Remove hazardous mover once. Stationary enemy Demons do not die or trigger
   additional victims. This phase exists even for a non-Demon mover.
5. Promote only a surviving mover, after movement/burn/suicide effects. A Water
   Buffalo becoming Demon burns **nothing on that turn**, A p4/M footnote. It
   cannot resurrect after dying next to an enemy Demon. Its new aura matters for
   later actions, not immediate removal of already adjacent unmoved opponents.
6. Update native history/rights and assess outcomes on the complete resulting
   state. Both final royal sets destroyed => draw by explicit simultaneous-royal
   convention (**S**); exactly one side retains royals => that side wins.

Effect records distinguish direct capture, simultaneous burn batch, mover demise
and surviving promotion, with actual identities/locations/causes. Victims never
depend on hash/board iteration order. A final royal may be burned. Multiple royal
losses in one batch remain simultaneous. Same-endpoint actions need not have the
same effects. Rejection exposes no intermediate state or partial effects.

## T05 — ranked jumping generals

Rank0 ordinary pieces (including Fire Demon/LionHawk/FreeEagle),1 BGn/RGn,
2 VGn,3 GGn,4 King/Prince. Prince rank4 is explicit royal completion **S**;
A's rank paragraph names King and M's footnote excludes royalty.
BGn/VGn use diagonal lines; RGn orthogonal; GGn all8. A jump-capture must cross
at least one piece, of either owner, and end at an enemy victim. Every intervening
piece must rank strictly below mover. Equal/higher pieces block in both profiles,
even if friendly or separated by ordinary pieces. Uncaptured interveners survive.
Quiet movement beyond any blocker is prohibited.

Profile1 A p4: jumped-to victim must also be strictly lower ranked.
Profile2 M: any **nonroyal** enemy may be the jumped-to victim. This is an explicit
interpretation **S-M** of M's separate blocker/royalty wording, not a claim M
expressly lists every higher-ranked general target. Both reject jumped King/Prince.
Ordinary unobstructed ray capture of a King/Prince or higher/equal general is
legal. VGn Area3 uses ordinary captures rather than ranked-jump restrictions.

## T06 — original/promoted identities and promotion rights

A §1.2.2 p4 and individual piece sections; explicit A supplement for profile2.
All30 promotion pairs are enumerated in `profile.json.promotion`; K,FiD,GGn,VGn,
FEg,LHk originals do not promote. Only once per original piece: promoted Ky moving
asLion cannot promote again toLionHawk; promoted HF moving asBGn cannot becomeVGn.
Preserve original identity even when its current face matches another original.

Zone is farthest5 ranks:1..5 Sente,12..16 Gote. A piece is eligible if a component
of its actual move begins or ends in zone; an actual Lion leg entering then returning
can qualify, a jump/ray merely passing across a zone square without landing cannot.
Resolve movement with old face, promotion after survival (T04). Never promote
an enemy victim or a stationary HT; neither it nor other unpromotables gains a choice.

Promotion is normally optional, except P/L/I on their farthest rank and N on its
farthest2 ranks, where otherwise it would have no possible geometric future move.
Occupancy/blockage does not by itself force promotion. Dead-mobility test ignores
other pieces. All four kinds must promote if eligible and surviving there.

At **that original piece's first eligible opportunity**, declining records one
restriction: on that piece's **next own move**, it may promote only if that move
captures an enemy and is otherwise eligible. Other pieces' moves do not consume
it. After this next own move, it may promote freely at later eligible opportunities,
even if it declines again. It is not a permanent in-zone restriction and not a
rule rearmed after every refusal. Mandatory promotion overrides an optional refusal;
a piece cannot choose to remain immobile. A captured/burned mover leaves no future
promotion right. This per-original-piece state is part of exact saved/search state.
Imported roots may declare rights per stable original-piece identity; absent entries
initialize to first opportunity unused at that root (**S**), not a claim about
unobserved pre-root play. Zone occupancy alone does not reconstruct prior refusals.

## T07 — legality, discarded pieces and turn progression

Movement domain, occupancy, complete choice, mandatory promotion, current actor,
royal/ending state and T09 history together define native Game LegalAll. No true
check/self-check filter exists (A §1.2.1): royal exposure and capturing an enemy
royal with one's own subsequently burned piece are permitted. Geometric threats
alone are advisory, not automatic mate. One complete accepted board action advances
turn once, irrespective of legs/captures/burns. No free general pass command;
source-authorized return routes remain complete board actions subject to history.

Local geometry/capture inspection can omit turn/history restrictions only when
explicitly labelled hypothetical/read-only. It cannot authorize actual play.
Invalid actor/profile/stale context/action/extra capture claim rejects atomically.
No source permission to infer Chi/JCSA Lion protection, Chu repetition obligation,
finite immediate-threat mate or material-exhaustion rule carries into these profiles.

## T08 — royals, endings and non-board commands

A §1.2.1/6: King and once-promoted Elephant/Prince are royals; capture/burn all
opponent royals wins, exposure is allowed. King/Prince identical movement and
rank do not erase their original identities. Assess T04 full effects before
royal outcome: one surviving side wins; simultaneous loss of both sets draws
(**S**, not a claim A/M discuss that edge). No checkmate-search or material score
creates an outcome. No general quiet-immobility/stalemate loss or automatic draw
is sourced: explicit `NoBoardActions` remains unscored/unconcluded unless a real
royal/bare condition applies; Resign/agreement/Stop remain available (**S**).

BareKing rule A §1.2.6, explicit completion for both: exactly **one remaining
royal and no other own piece** starts the immediate bare-response procedure;
apply equally to a sole Prince (**S**), while King+Prince is not bare. Newly
bared side may take an actual immediate legal reply that removes the opponent's
last nonroyal while retaining a royal; the opponent must then have exactly one
royal, not King+Prince. Both sole royals => draw. Capturing the
opponent's final royal instead is resolved first by normal royal loss. A completed immediate royal win or simultaneous-royal draw also qualifies as
an escape; a reply need not equalize nonroyal material if normal royal resolution
avoids the bare loss. If no such immediate escape reply is possible, bare side loses; if it plays another otherwise
legal board action, bare loss concludes after that action unless a higher-priority
royal result occurs. This is finite actual one-action admission, not proof of a
future forced series. A's proposed multi-turn extension is not adopted. Imported
sole-royal roots put the current bare side into this same response opportunity;
when exactly one side is bare but it is not to move, the root concludes a
win for the nonbare side; when both sides are already sole royals, the imported
root is drawn (**S**). No preceding command is invented. Structurally consistent
imported diagrams need not be proven reachable; dead unpromoted pieces are not
retroactively promoted. This permits honestly labelled no-action diagnostics.

Resign concludes loss for actual actor; draw requires real opposing offer and
acceptance, not self-consent or inferred exhaustion. Decline/ordinary play clears
the opposing offer. Administrative Stop records an interruption, not loss/draw.
These untimed command spellings/offer protocol are **S**, not a published A/M event
standard. No100-move/250move/points claim, evaluation resignation or automatic
agreement is selected. Concluded game rejects further board play.

## T09 — repetition, provenance and three distinct identities

A §1.2.7 forbids recreating any known complete board+next-side position. Native
admission rejects the **first** such continuation, preserving state/history;
no fourth-repeat forfeit/claim/draw and no Chu must-vary responsibility analysis.
Root counts once; imported diagrams fabricate no earlier positions. Administrative
commands do not append board repetition occurrences or consume promotion rights.

Repetition key (**S** exact spelling of source board position): immutable selected
profile, turn, each occupied square's side/originalkind/promoted state. Exclude
physical piece serial numbers, next-move refusal flags, offers, clocks/counters
and action-route spelling. Thus same board/turn with different refusal rights
still repeats; equal movement faces with different original/promoted identities
are not silently flattened. This exclusion follows source's board+turn rule,
not a claim saved/search identity is the same.

Saved-state identity includes complete original identities, rights, history,
commands/effects/outcomes and provenance. Search identity includes selected
profile and all history/rights relevant to future admission. Never cache outcome
or legal continuation by board alone. Complete routes remain distinct in canonical
records even when repetition treats their final boards as equal.

## T10 — reversal and canonical replay

Apply/undo must restore every logical state/cache/right/effect/history/conclusion;
private undo layout never becomes public save schema. Rewind/reopen assign fresh
runtime authority and cannot revive old requests. Native record fixes profile ID,
edition/schema, root/provenance, actual commands/complete actions, ordered effect
phases, per-original rights and derived outcomes. Strict parse rejects duplicate,
unknown, conflicting/missing identities and forged derived data; replay re-derives
and compares all semantic facts. One profile's record must never be interpreted
as the other merely because board/sides/topology match. Unsupported conversion
fails explicitly; switching profiles means a fresh root/game, not mutating history.

## T11 — independent expectations and pending implementation

`fixtures.json` contains source-diagram exact geometry sets, independently constructed
special-case/action/effect/history/negative cases and hand-counted4/16 perft/divide.
Geometry sets are not automatically full-game LegalAll: native87 must separately
admit turns, rights, history and endings. Source equations are finite empty-diagram
fixture authoring, not a prospective arbitrary-position move generator. Rotations
exchange owners/coordinates180°; no horizontal setup symmetry is presumed.

Strict data validation and review do not execute native Tenjiku. Native assertions
are **0 executed/0 passed**, with all relevant observations pending87. Consumer
requirements are separate79/44 obligations. Preserve independent expected values;
never populate observations by echoing expectations or another engine's answer.

## T12 — current shared architecture and selection

New Game exposes two meaningful named profiles under Tenjiku when finished44
makes it playable. Native creation chooses profile before root; records, requests,
observations, inspection contexts and presentation keys preserve that selection.
Existing `shared_new_game` infers profile from game and `side(NativeSide)` constructs
profile-qualified presentation keys from one profile per game:79 must extend these
real consumers for two profiles. It must not mutate existing games or introduce
separate controllers/envelopes/shells. Current document/version/selection tokens
remain safe through immutable binding and exact context admission; representation
of any new token field remains an implementer choice.

Detailed87/79/44 handoff and MSH-107 replay/cancellation/inspection measurement
obligations are in[handoff.md](handoff.md). No refactor, higher capacity constant,
competitive-strength optimization or implementation is authorized by83.
