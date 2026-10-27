# Independent fixture strategy and initial expectations

MSH-80 planning evidence, **not executed conformance tests**. Basis: official
WXF2018 Articles1–3,8,19–20 and chapter5, with [reconciliation](sources.md).
Freeze and independently review expectations **before** differential execution.
Reference output is stored separately, never substituted for expected facts.
Coordinates here are the proposed explicit Red-bottom a–i /0–9 dialect.
They are not an adopted canonical encoding. Side mirrors need independent
geometry checking rather than an assumption that every rule is symmetric.

## Concrete seed XQ-F001 — complete initial Red action set

The source setup is EN1.4 PDF11/printed3. Count by movement clauses2.1–2.7:
4 chariot +4 horse +4 elephant +2 advisor +1 general +24 cannon +5 soldier
= **44 complete actions**. Each cannon has11 quiet choices and one capture of
the opposing horse beyond the opposing cannon screen. This count was derived
from the board and movement clauses, not a program's perft result.

```
chariot: a0a1 a0a2 i0i1 i0i2
horse: b0a2 b0c2 h0g2 h0i2
elephant: c0a2 c0e2 g0e2 g0i2
advisor: d0e1 f0e1
general: e0e1
cannon: b2a2 b2c2 b2d2 b2e2 b2f2 b2g2 b2b1 b2b3 b2b4 b2b5 b2b6 b2b9
cannon: h2c2 h2d2 h2e2 h2f2 h2g2 h2i2 h2h1 h2h3 h2h4 h2h5 h2h6 h2h9
soldier: a3a4 c3c4 e3e4 g3g4 i3i4
```

Future corpus admission must independently verify exact setup transcription,
all44 members, exclusion of every other action, side mirror and shallow divide
expectations. No depth2+ total is asserted here without independent derivation.

## Concrete seed XQ-F002 — same board, different history

Only Red general d0, Red chariot e9, Black general f8; Red to move.

| History | Complete four-ply cycle | Independently expected fact |
| --- | --- | --- |
| H-check | `e9f9 f8e8 f9e9 e8f8` | Every Red move checks; Black resolves it. |
| H-quiet | `e9a9 f8f7 a9e9 f7f8` | No check or material chase. |

Both cycles return exactly the same board/side, keep generals inside their
palaces and never expose facing generals. Board-only action sets at that target
must agree. Native history classification/duty must differ: unilateral checking
versus neutral cycle. Repeat histories separately to exercise XQ-P2 after its
selection; **no immediate outcome ply is asserted before that decision**.
Retain root provenance and distinguish target history prefixes in requests.

## Concrete arithmetic seed XQ-F003 — claimant-specific natural count

A100-physical-ply no-capture ledger with Red12 checks/Black0 gives **Red98,
Black100** counted steps under Chinese Art8. Red10 versus11 checks isolates
the subtraction boundary; Black's11th check does not reduce Red's count.
At exactly100 counted steps Chinese threshold is reached, despite the English
continuation's inconsistent “did not exceed” phrasing. These are ledger
expectations, **not yet a full legal replay**; future history fixtures must
instantiate legal nonterminal sequences and approved claim/event ordering.

## Required clause-to-fixture families

Status **seed** means a concrete independently reasoned fact above; **family**
means boards/routes/history still need transcription and review. No family is
a passing test. Stable IDs are obligations to carry into the corpus ledger.

| ID / status | Expected distinction and source | Owning future boundary |
| --- | --- | --- |
| F001 seed | Initial complete44 set,1.4/2 | Fixture admission/local legality |
| F002 seed | Same board/different checks,19/20; exact target history | History/transport/session |
| F003 seed | Claimant counts10/11/100;3.2D/8 | History/adjudication |
| F004 family | Horse leg, elephant eye/river, palace/general/advisor constraints, both owners,2 | Fixture admission/local legality |
| F005 family | Soldier crossing river/edge/no retreat, both owners,2.7 | Local legality |
| F006 family | Cannon0/1/2 screens; mounts of either side; quiet/capture distinctions,2.6 | Local legality |
| F007 family | Facing generals, pinned blocker, horse-leg and cannon-screen check evasions; adding a screen may create check while removing one may cure it,2.9/2.10 | Local legality |
| F008 family | Checkmate versus stalemate both loss, no king-capture continuation; own-piece/off-board/self-check rejection,2.8/3 | Local legality/history |
| F009 family | Three completed returns/four occurrences, exact warning/refusal and neutral cycles under selected XQ-P2; incomplete EN/ZH diagram1/2 histories kept distinct | History; profile decision |
| F010 family | Real/fake roots (legal immediate recapture versus pin/immobility); protected chariot pursued by horse/cannon exception,19.10–15/20.6 | History |
| F011 family | Same mobile type versus constrained target; free horse versus leg-blocked horse,20.7 | History |
| F012 family | Same victim persistent identity versus changing victims; one/multiple pursuers; own general/soldier and uncrossed victim exemptions,20.3–5/9 | History |
| F013 family | All-check versus opposing chase; mutual-check legal witness or impossibility argument; alternating check/chase; simultaneous threats and two-to-one frequency,3.1H/19.18/20 | History |
| F014 family | WXF diagram38 alternating mating-threat/chase permitted; diagram109 simultaneous mate-threat/chase resolves to chase,19.16/20/36.4 | Fixture/history review; omitted20.2 enumeration resolved by these examples |
| F015 family | Block/offer/exchange/check-then-capture/idle and overlapping labels;20.8/10 plus applicable chapter5 examples | History |
| F016 family | Neither side can win3.2A, positive/negative proof, uncertainty and terminal precedence; R-NOWIN remains research gate | Profile/history |
| F017 family | Capture reset, starting known root, declared counters versus proven prefix; claim actor/advisory/committed/rejected requests, agreement/resignation;3/8 and selected software policy | History/native records |
| F018 family | Complete transition/undo identity, deterministic sequences, exact native record/event replay; invalid input atomicity, duplicate/unknown/profile/version conflicts | State/records/typed routing |
| F019 family | Wrong purpose/limits/history/session/generation; historical selection/undo/cancel/replacement across games cannot revive work | Transport/engine/session |
| F020 family | Encoded bytes/events/controls/observations/replay resource admission, reserved stop/close/save path, failed save/open atomicity, MSH-61 | Archive/shared session |
| F021 family | Actual intersection/palace/river/piece semantics, whole-board sizes, native enemy inspection, keyboard, common-shell control stability, actual packaged flows | Presentation/desktop/shared final acceptance |

## Reference comparison and failure evidence

The [source audit](comparator-audit.md) nominates two **unadmitted** restricted
board comparators. Future admission builds pinned sources in isolation, records
toolchain/flags/patches/binary hash and separately pinned weights if any, then
executes positive/negative sentinels. Require complete LegalAll-equivalent sets,
including all evasions; do not compare only a best move or attack map. Normalize
differing rank dialects explicitly. Demonstrate a deliberate expected-set
mismatch, malformed input, timeout and reference failure report with source IDs,
position/prefix, missing/extra actions and first divide divergence.

Shallow perft is board enumeration under a declared convention, not proof of
history conclusions. Imported root domain must be independently validated;
parser acceptance is neither reachability proof nor full-state restoration.
Pikafish/Fairy-Stockfish share ancestry; agreement is supporting evidence, not
independent authority. History outcomes use rule/diagram-derived expected facts,
not either engine's returned score/draw shortcut. Disagreements return to the
clause and independently reviewed position; expectations are never edited just
to match either implementation.
