# M4 shogi-first plan and delivery evidence

Planning prepared 2026-10-26 under **MSH-62**; its reviews remain historical.
MSH-63–66 selected the profile and delivered native board state/local legality.
MSH-67 adds rules-owned untimed history, conclusions and strict native records.
MSH-68–74/90 deliver its real console, engine, interoperability and normal desktop
consumers. MSH-75 independently reconciles complete standard-shogi delivery in
[the acceptance review](shogi-delivery-review.md), supporting MSH-41 completion.
Current evidence is in [coverage](shogi-coverage.md). M2/M3 remain closed; their source/evidence, completed
task history, MSH-61 persistence admission and MSH-40 roadmap correction remain
intact. MSH-42–47 and M4 remain open with their original full delivery/closure criteria.

Standard shogi is first; provisional later order is xiangqi, tori, chu, tenjiku.
Independent rules research may proceed separately. The source inventory records
no evidence-based change to that provisional delivery order at present.

Current next-game handoff (MSH-80,2026-10-27): [Xiangqi planning dossier](xiangqi/README.md).
It proposes `mshogi.xiangqi.wxf2018.v1` without adoption, preserves unresolved
source/owner decisions and refines native/shared/console/engine/desktop work
into MSH-91–103 under the existing MSH-45/76/84 records. No Xiangqi implementation
has begun. Partial [Tori](research/tori.md), [Chu](research/chu.md) and
[Tenjiku](research/tenjiku.md) research remains independent and unfinished.

| Deliverable | Proposal and scope |
| --- | --- |
| Current-code implementation map | [implementation-map.md](implementation-map.md): actual chess boundaries and minimal explicit chess/shogi extensions, old-document compatibility, capabilities and preserved lossless admission |
| Complete named shogi profile | [shogi-profile.md](shogi-profile.md): selected `mshogi.shogi.csa36.v1`, primary editions/clauses, complete board/history/conclusion conventions and explicit software adaptations |
| Independent verification/reference/opponent audit | [shogi-verification.md](shogi-verification.md): immutable source pins and exact operation/semantic gaps; source audit is not runtime conformance |
| Selected contract coverage and current native evidence | [shogi-coverage.md](shogi-coverage.md), [legality-validation.md](legality-validation.md): bounded MSH-63–66 board implementation, reviewed fixtures/reference execution and exact limitations |
| Complete standard-shogi delivery review | [shogi-delivery-review.md](shogi-delivery-review.md): original MSH-41 AC1–8, complete selected profile, three independent review tracks, fresh reference/opponent/normal desktop execution and source-specific platform evidence |
| Native history, conclusions and canonical records | [history-state.md](history-state.md), [history-validation.md](history-validation.md), [mutual-check resolution](mutual-check-resolution.md): MSH-67's untimed native contract and independently reviewed proof, with original incomplete evidence retained |
| Initial independent fixture inventory | [shogi-fixtures.md](shogi-fixtures.md): 55 stable IDs, hand-derived examples and clearly pending instantiation/review; no invented test results |
| Native tasks, traceability and executable graph | [delivery-plan.md](delivery-plan.md): MSH-63–89, original MSH-41/46 criteria mapping, one common owner and no prerequisite/final-delivery cycles |
| Bounded later-game source/gap inventory | [later-game-sources.md](later-game-sources.md): adopted association editions, historical evidence, proposed revisions and software conventions kept distinct |

## The material owner choice

**SHOGI-P1 / MSH-63:** John selected option A, `mshogi.shogi.csa36.v1`,
on 2026-10-26 against integration `6db8500e92129b95e502e7b4d5c4a26072e38652`.
The [selected profile](shogi-profile.md) records the exact CSA bundle and approved
software adaptations. JSA professional and other event conventions remain
separately named alternatives; they are not additional required implementations.

Ordinary native API/representation details stay autonomous. Illegal/stale API
requests reject atomically; this does not simulate every physical tournament
foul. Reference-engine helper behavior does not decide the profile. In particular,
cshogi's complete `LegalAll` move enumeration is useful, but its repetition
helper recognizes an earlier cycle rather than governing fourfold adjudication.

## Completion boundary

Historical MSH-62 handoff: the first implementation queue was MSH-64/65, then MSH-66; affected
history/outcome work MSH-67 also waits for SHOGI-P1/MSH-63. MSH-69/72/73 supply
early shared mechanisms without waiting for final MSH-41 or MSH-46. MSH-75
now verifies full shogi delivery; MSH-88/89 and MSH-47 retain final
all-game/platform and M4 closure obligations.

MSH-62 did not execute that queue, certify any shogi rule, run a
reference/opponent match, implement another game or close an M4 delivery.
M5/M6 obligations remain future work. Planning verification and independent
review are recorded in [validation.md](validation.md) when complete.

The next dependency-ready M4 research scopes are MSH-80 xiangqi (provisional
next game), MSH-81 tori, MSH-82 chu and MSH-83 tenjiku. No other game or M4
closure is included in MSH-75. Strength remains MSH-50 in M5.
