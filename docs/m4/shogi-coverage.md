# Selected shogi contract: clause and fixture coverage

Current profile: **`mshogi.shogi.csa36.v1`**, option A, owner decision SHOGI-P1
2026-10-26 against `6db8500e92129b95e502e7b4d5c4a26072e38652`.
[The profile](shogi-profile.md) fixes editions and precedence. This ledger separates
accepted contract, concrete independently expected cases, reference observations
and implemented evidence. Historical MSH-62 reviews are retained unchanged.
A listed fixture is an obligation, not an assertion that it has executed.

| Selected clause / source | Stable fixture scope | Implementation owner / evidence boundary |
| --- | --- | --- |
| Setup, side-relative geometry, blockers, promoted movement; JSA-R2–6/B1–3 | SH-F001–004 | MSH-64 expected board corpus; MSH-65 state; MSH-66 complete legality |
| Optional/compulsory promotion, camp entry/exit, capture demotion; JSA-R5/10(1)(ii), B2/3/N | SH-F005–010,022–024 | MSH-64–66, both sides and evasions; LegalAll required |
| Hands, drops, nifu, dead ranks, pawn-drop mate and legal defenses; JSA-B2/B4/B5 | SH-F011–017 | MSH-64–66; existing-pawn mate and other-piece drop mate are allowed |
| King safety, pins, checker capture/interposition, no king capture; JSA-B3–5 | SH-F018–022,031/032 board-only | MSH-64–66; geometric influence is distinct from whether an attacker can legally move |
| Complete position equality and exact undo; APP and JSA-R8(1) | SH-F023/024/026/027 position-only | MSH-65/66: board, owners, promotion, hands, turn; counter/provenance retained but excluded from repetition identity |
| Four occurrences, checking interval first through fourth, checker's loss before draw; CSA27(2), JSA-R8/10(10) | SH-F025–030 | MSH-67 history and independently reviewed event ledger; not inferred from reference is_draw |
| No legal action loses with/without check; CSA27(1)(i) | SH-F031–033 | MSH-66 supplies exact empty board-choice set only; MSH-67 assigns reason/result |
| Advisory entry eligibility vs explicit declaration; camp, ten other pieces, point thresholds28/27, no check, own turn, time; CSA25 | SH-F034–037 | MSH-67 native untimed declarations. Invalid admitted formal declaration loses; malformed/stale request rejected before admission |
| JSA24/31 declaration/agreement/replay | SH-F038 and alternative portions035/042 | **Unselected alternative**, no selected-profile implementation obligation |
| Resignation, digital request rejection, external committed foul vs failure; JSA-R6, CSA25–28, APP | SH-F039/040 | MSH-67 untimed resignation/request semantics; MSH-71 named runner policy; timed/retrospective officiating outside initial mode |
| 320 completed plies; loss/repetition precede length; positive SFEN n means n−1 declared completed plies; CSA27(3)+approved APP | SH-F041/043/044 | MSH-65 preserves strict supplied counter/provenance. MSH-67 classifies imported roots n≥321, boundary n320, defaulted adapters and canonical reconstruction |
| Final draw and distinct 0.5-per-side score, no replay; CSA22/27 | SH-F042 | MSH-67 result/score metadata; MSH-71 export/runner |
| Strict SFEN/USI board-action primitives, no invented history | SH-F043–045 | MSH-65; protocol commands resign/win are not board actions |
| Full canonical game/command provenance, records, restoration | SH-F046–048 | MSH-67/68/69/73; MSH-61 complete-save-path guarantee preserved |
| Presentation, complete choices, capacity, protocol/search, actual application | SH-F049–055 | MSH-68–75; full MSH-41 and MSH-46 delivery remain open |

## Interpretation boundaries and named gates

**BOARD-PIN (MSH-64/66, SH-F016/020/021):** attacks used for king safety are
geometric with occupancy: an attacked king cannot be left capturable even when
the attacking piece is pinned to its own king. Test the pinned defender contrast
and king-origin x-ray on independently reviewed diagrams. A defender's proposed
capture in a pawn-drop-mate test must itself preserve its king. This is an
operational formalization of JSA-B4/B5's threatened-capture/self-check rules,
not permission to use an engine helper as authority.

**HIST-INTERVAL (MSH-67, SH-F028–030):** before accepting history adjudication,
instantiate and independently review checked/check-relieved fourth occurrences,
interrupted checking and overlapping candidate repeat windows. The contract is
all own moves between first/fourth occurrences, not a first prior hash hit. If a
mutual continuous-check cycle is structurally possible, establish the exact
interval/precedence from CSA27(2)/JSA-R10(10) with a concrete legal history before
assigning that case a result. Otherwise retain a reasoned impossibility proof.
This gates affected MSH-67 history cases only, not local board enumeration.

Current resolution: [the complete-chain proof and case accounting](mutual-check-resolution.md)
discharge the mutual-case obligation for every admitted live root. The original
incomplete proof review remains historical evidence. Checked/check-relieved,
interrupted and overlapping histories are instantiated in the independently
expected history corpus and native tests described below.

**DECL-BOUNDARY (MSH-67, SH-F034–037):** material-valid threshold diagrams and
one-negated-condition cases must distinguish eligibility from formal commitment.
Untimed live time eligibility is approved; no unnamed timed policy is implemented.

**IMPORT-LENGTH (MSH-67, SH-F041):** review n320/n321/+ and no-action/repetition
precedence with explicit known-history origin. Sparse diagrams are structurally
admissible analysis inputs; material inventory and two kings do not prove reachability.
Missing earlier play is never inferred from the declared number.

These are named fixture/research gates already owned by existing tasks, not a
new owner-decision request. Source editions, the rejected event alternatives and
original planning constructions remain traceable in the profile/inventory and Git.

## Executed board evidence by bounded ticket

- **MSH-63:** explicit selection and author-independent AI profile/source review
  [PASS](profile-selection-review.md). This decision evidence claims no execution.
- **MSH-64:** [actual isolated reference admission](reference/README.md),
  [101 concrete independently expected cases](../../tests/fixtures/shogi/board-rules.json),
  four audited API-gap sentinels and four injected mismatch/failure demonstrations.
  [Independent AI fixture/harness review](reference-review.md) records the checked
  subset and resolved findings. Board inventory IDs001–024 are represented;
  representation of an ID does not certify every future completeness construction.
  Reference observations are separate from expectations; no native comparison is
  claimed by this ticket. The complete55-entry inventory is not a prerequisite.
- **MSH-65:** [state/notation/structural-transition tests](state-validation.md),
  [independent AI source/test review](state-review.md), source826cd2f integrated
  at ce42d06111a382436d2d853b5d61d4ac2196ac8b. Native CI40300895588 passed on
  Windows2022/macOS15/Ubuntu24.04. No full local legality or match outcome is claimed
  by that intermediate state ticket.

MSH-66 supplies the complete local legal API and native differential evidence;
MSH-67 and later consumers retain all history/outcome/protocol/archive/UI duties.

### MSH-66 execution and corrected reference domain

[Native legality validation](legality-validation.md) and its structured source
fingerprints record complete local legal choices, both-side movement/promotion,
drops, legal pawn-drop defenses, attacks/pins, atomic apply and exact undo.
The additional68 independently reviewed cases cover full central/edge movement
sets, bishop/lance blockers, empty local choice sets and independently derived
initial depth2=900/divide30. BOARD-PIN has concrete reviewed and executed coverage;
it is no longer an unresolved board interpretation gate.

Longer runs exposed two original MSH-64 bishop-evasion roots with both kings
already checked, outside cshogi's incremental-state domain. MSH-64 was reopened
and corrected with independently reviewed version2 diagrams and an audited domain
guard; v1 fixtures/report/manifest and the failed perft observation remain
preserved. [The domain review](reference-domain-review.md) supersedes that portion
of the initial admission review. Native structural admission was not narrowed:
the original diagram has its own independently expected native regression.

The actual admitted-reference/native evidence is167 deterministic sequences,
4,017 prefixes and169 depth2 perft/divide cases, with zero mismatches after the
domain correction. Counts beyond the explicitly hand-derived start expectations
remain differential observations. Separate AI fixture and code reviews identify
their actual methods; no human review or full game conformance is claimed.
HIST-INTERVAL, DECL-BOUNDARY and IMPORT-LENGTH remain MSH-67 gates, and all protocol,
archive, engine, application and complete-delivery duties remain open.

Final native source0845392 is integrated/pushed at
6f6301e5a6b90a1dca986e88c6ce6e022f07e5d7. Normal native CI40305615224 and
desktop regression CI40305615506 pass on Windows, macOS and Linux. Exact commands,
source fingerprints, review methods and limitations are in the validation records.
MSH-63–66 complete their bounded obligations; they do not close MSH-41 or M4.

### MSH-67 native history and canonical records

[Independent history fixture derivations](history-fixture-derivations.md) establish
21 history/import ledgers, nine declaration diagrams and seven command cases.
The current HIST-INTERVAL, DECL-BOUNDARY and IMPORT-LENGTH obligations have concrete
reviewed expectations, including checked/check-relieved fourth occurrences, either
checking side, interrupted checking, overlapping identities, exact28/27
declaration boundaries, false versus advisory declarations, supplied/defaulted
counters and loss/repetition/length precedence. Reference cycle helpers do not
define outcomes. The mutual-check proof uses23 conditional normalized pairings,
not an exhaustive search of legal states.

The [native contract](history-state.md) supplies explicit live roots and unknown
imported prehistory, complete rules-owned event/check/occurrence history, selected
untimed conclusions, atomic command admission, exact undo, same-ply terminal
prefixes and strict versioned canonical game/match records. Complete-save capacity
is bounded without discarding accepted history. Invariant diagnostics retain
candidate context through checked prefix and record replay; they never invent a
game outcome. [Validation and review](history-validation.md) distinguish executed
tests, earlier partial evidence and the final integration/CI record in MSH-67.
Engine, transport, console, shared archive, desktop and complete-delivery duties
remain with their existing later owners. MSH-41–47 and M4 remain open.

### Subsequent bounded consumer delivery

MSH-69 supplies the strict [typed shared context](shared-context.md), preserving
native record identity/history and one supervised execution owner. MSH-68
supplies the [standalone untimed console](shogi-console.md), complete native
choices/notation, canonical files and historical undo. MSH-70 supplies the
[bounded native evaluator/search and real structured consumer](shogi-engine.md),
with actual process evidence and game-specific scores; it does not establish
playing strength. Their task records identify exact reviews, tests and source/CI
revisions. Earlier milestone evidence above remains historical.

USI/external-opponent work, shared desktop sessions/archives/presentation and
complete-delivery closure remain with MSH-71 onward and MSH-41–47. M4 remains open.
