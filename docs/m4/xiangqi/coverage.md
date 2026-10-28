# Selected Xiangqi profile coverage and implementation gates

Current profile: **`mshogi.xiangqi.wxf2018.v1`**, selected XIANGQI-P1 on
2026-10-27. Authority: official bilingual WXF2018, individual reconciliations
in [sources](sources.md). The table below preserves the selected complete-scope
planning ledger. Executed board foundations and local-rule conformance are now
recorded in [foundation evidence](board-foundation-evidence.md),
[local contract](local-legality.md) and the MSH-91/92/93 task records. The
restricted reference admission does not admit history helpers or playing strength;
the native history/game contract is now implemented in MSH-94 (see below).
MSH-95 adds [typed record/context routing](shared-routing.md) with explicit
unsupported execution. There is still no Xiangqi engine, console or desktop consumer.
All original MSH-45 requirements remain in M4. Profile selection is not delivery.

| Required scope / clause | Independent expectations / evidence plan | Delivery owner and gate |
| --- | --- | --- |
| Setup/intersections/Red-Black/pieces,1 | F001 exact44 opening actions; source setup transcription and side mirror |91 independent corpus;92 native state;93 complete legality |
| Every complete move/capture/derived effect,2 | F004–008 horse leg, elephant eye/river, palace, cannon screens, soldiers, kings, pins/evasions; all complete choices |92 transitions/reversal/codecs;93 full local conformance |
| Checkmate and stalemate both loss,2/3 | Separate in-check flag but same native no-action loss; no king-capture continuation |93 local/94 native outcomes |
| Native state versus history/provenance | F002 same target with checking/quiet prefixes; declared counters not proof; no inferred pre-import repetition |92 roots;94 exact history/native records;95 strict typed boundary |
| Third warning/fourth loss or WXF draw,3/19/20 | F022–026 independent boundary, equal-fault/responsibility and overlap expectations; native warnings/events preserve board ply |94 executed; **G-CYCLE** resolved with reviewed contiguous anchor fixtures; see history contract |
| All check/chase/mating-threat/exchange/offer/block/idle and exceptions,19/20/chapter5 | F009–015, WXF diagrams38/109 and legal/fake roots, victims/pins/river/same-type/frequency cases |94 full native adjudication; no comparator outcome oracle or ignored category |
| Natural-count Art8 and3.2D | F003 claimant98/100 ledger; F017/F027 complete legal history, manual rejection audit, optional policy, capture/reset/root and result ordering |94 native command/event admission;99 bounded audit/archive;102 actual interaction |
| Impossible for either side to win,3.2A | F016/F028 sound recognizer, exact proof/witness, cancellation/resource unknown; no inference from unrecognized material |94 executed assessment/fast recognizer; owner-selected natural-board-win existential goal;105 implements explicit bounded native exact assessment (see current status below) |
| Resignation, agreement, other concluded outcomes and ordering,3 | Serialized no-overwrite outcomes; board terminal then repetition then proven3.2A; admitted commands and automatic-claim policy explicit |94 native;97 engine;102 recovery/outcomes |
| Exact native action/event reversal and canonical records | F018 strict bounded duplicate-preserving native parse, complete commands/events/modes/policies/root; semantic undo never revives execution |92/94 native;95 tagged envelope;99 desktop archive |
| Own functional evaluator/search and actual external match | F019 context/purpose/history/limits/session/process identity, legal native results, cancellation; real independent opponent admission/config/record |97 own search;100 protocol/runner;54 measured M5 strength |
| Standalone native console and notation | Explicit native diagnostic dialect; no FEN-only restoration or lossy notation fallback |96 real console;100 external dialect adapters |
| Shared session, geometry and native read-only enemy inspection | Real intersection/river/palace/Red-Black metadata; complete native choices and selected history prefix |95/98 under76; existing RawValue/execution ownership retained |
| Complete lossless save path and actual desktop | F020/021; encoded bytes/events/controls/observations/replay work including rejection/proof observations, reserved stop/close; accepted common shell/full board visible |99/101/102;61 guarantee preserved; no Python or second frontend rules engine |
| Full complete-game independent verdict | Every original45 AC1–8 and all selected clauses traced to actual evidence, including both proof levels; unresolved required work blocks closure |84 and103 depend105;45 depends105/103; shared46/88/89 and47 remain final all-game/platform closure |

## Residual case ledger — not omitted behavior

- **G-CYCLE / MSH-94 — resolved:** the complete contiguous first→third/fourth
  interval, anchor-specific obligations, persistent victim IDs and proved warning
  retirement are defined in [native history contract](history-contract.md).
  The independent cycle/conduct/source and completed-boundary reviews, exact
  fixtures and executed tests are linked in [history evidence](history-evidence.md).
- **G-NOWIN semantics — resolved by owner:** existential natural board win only
  (WXF3.1A mate or stalemate); history-dependent penalties cannot create a
  positive witness. [Owner resolution](owner-decisions.md#g-nowin-resolution-during-msh-94-2026-10-27).
  MSH-94 supplies sound fast proof/witness/unknown and strict recorded modes.
  **MSH-105 exact computation and feasibility remain required**, unstarted until
  MSH-94 and MSH-95 are Done. No general proof-backend performance is claimed.
- **R-EDITION / scoped research:** undated AXF revision and complete CXA2020
  text/amendments remain unverified. Their absence is not claimed resolved and
  does not block91/92; use them only when an actual WXF question needs them.
- **R-CLASS /94 — executed:**20 source witness histories and additional independent
  warning-prefix regressions cover the consequence-relevant19/20/chapter5
  families. Native classification preserves permitted label distinctions without
  claiming a mate-threat search/telemetry API. Published37/38/108/109/110 ownership,
  root/pin and alternating/simultaneous cases were independently checked against
  both official texts/pixels. Actual corrections and review findings are retained.

MSH-80 completes a selected complete-scope planning contract with named gates,
not an implemented complete game or proof algorithm. The subsequent authorized
MSH-91/92/93 batch implements only the board foundation and local legal contract;
MSH-94 now adds the reviewed native history/record/fast-assessment boundary;
105 remains unstarted behind94/95. The exact path is not replaced by this work.
MSH-41/MSH-75 stay Done; MSH-42–47 and M4 remain open.

## MSH-105 current native exact assessment

The [exact-verification contract/evidence](exact-verification.md) implements the
owner-selected intrinsic board graph with bounded native BFS/DFS, replayable
positive witnesses, complete reachable-closure negative proofs and distinct
Unknown/failure. Independent palace-universe expectations and executable fixture
coverage support this boundary; raw measurements retain limits and observations.
The older unstarted105 statements above describe the preceding planning/history
stage, not the current source capability.

ExactAnalysis is an explicit read-only native assessment resource; accepting its
report does not overwrite Game history/fast-only canonical mode, conclude an
actual match or advertise shared-engine capabilities. MSH-84 handoff and later
MSH-97 consumers must retain the explicit unknown/unscored policy, full context,
limits and stale-result guards. Universal ordinary-play WXF completeness and
Xiangqi engine/console/desktop delivery are not inferred. All later delivery and
M4 gates remain open under their existing task ownership.

## MSH-84 consumer-readiness record

[The native consumer contract](native-consumer-contract.md) and
[handoff evidence](native-handoff-evidence.md) review91–95/105 together rather
than infer readiness from Done statuses. They distinguish local board choices,
actual native command authority, canonical/context identity and fast/exact
proof provenance/Unknown. Existing public game-owned APIs and shared typed
routing suffice; no production framework/refactor or consumer implementation
was required. Exact work stays explicit/read-only, and unresolved required
adjudication stays unscored. Later96/97/98/99/100–103 retain their implementation
and product integration responsibilities. M4/45 are not closed by this handoff.
