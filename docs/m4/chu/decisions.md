# Chu profile decisions and faithful interpretation

Owner direction, 2026-11-02; MSH-82. Selected identity
**`mshogi.chu.jcsa2019.v1`**, edition
**`JCSA-2019-09-14+2004-06-06+MSH82-2026-11-02`**.
[Complete profile](profile.md), [sources](sources.md), [fixtures](fixtures.json).

The initial unintegrated proposal asked about automatic fourth-cycle forfeiture,
automatic bare-royal draw and quiet no-action loss. John rejected novel rules
arising merely from software underspecification. **Those proposals are withdrawn
and are not selected behavior.** No Chu implementation used them. Original
2026-10-27 research remains intact; its questions are reconciled here rather than
turned into historical-acquisition requirements.

| Area | Owner direction | Faithful deterministic implementation |
| --- | --- | --- |
| CHU-P1 | Capture-based movement, no Chess self-check, narrow actual-threat/complete-response mate, no unrestricted forced-mate search | C06 tests finite complete responses and next-turn captures. Prince promotion can escape if no one action captures both royals. Quiet no-action without a sourced ending stays unresolved/unscored, not a new loss/draw. |
| CHU-P2 | Responsible side varies **before** fourth occurrence through native admission | C07 derives prospective-cycle witnesses at occurrence3, admits still-changeable prefixes and rejects the culprit's last continuing opportunity. No fourth occurrence or forfeit. Both-jitto uses the actual first passer; alternative empty routes cannot evade it. |
| CHU-P3 | Narrow exhaustion, exclusions, immediate response and agreement-based draws | C08 counts only the express configuration, permits actual immediate countercapture, and requires consent for draw. The C1 IX Prince-recapture sequence fixes timing; imported roots use explicit current turn without invented earlier play. |

C1 VI.1a does not contain VI.1b's sole-side qualification. Applying(a) to both
continuous checkers is an explicit literal interpretation, not a claim that the
source discusses that rare diagram. VI.1d/VIII is the specific both-jitto case.
Repetition compares native semantic turn-boundary positions/sequence while
records retain every actual route. This separates repetition equivalence from
complete-action identity without discarding information.

Royal mate is a finite implementation of the immediate published condition,
not a claim JCSA printed pseudocode or a proof of a forced win several turns
away. Quiet no-action status records a source limit without manufacturing an
outcome. II.6's agreement procedure governs IX's exhausted-draw examples; an
illustration of a draw is not converted into an unstated automatic event.

C1 with compatible C2/C3 governs. Software does not select correspondence-only
Lance rescue. No Go-between rescue, mandatory promotion, Shogi drops or historical
twenty-move grace. Lishogi's different rescue, Lion-protection and neutral-repeat
conventions remain named alternatives, not unresolved conflicts in this profile.

Strict parsing, native complete actions/effects, atomic refusal, imported known
history, schema spelling, internal representation and bounded algorithms remain
implementer decisions. No further material owner question was identified after
source/fixture review. A later concrete contradiction must be investigated rather
than silently changing this identity.
