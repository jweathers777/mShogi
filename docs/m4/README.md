# M4 shogi-first plan and delivery evidence

Planning prepared 2026-10-26 under **MSH-62**; its reviews remain historical.
The authorized MSH-63–66 batch selects the profile and implements native board
state/local legality only. Current evidence is in [coverage](shogi-coverage.md). M2/M3 remain closed; their source/evidence, completed
task history, MSH-61 persistence admission and MSH-40 roadmap correction remain
intact. MSH-41–47 stay open with their original full delivery/closure criteria.

Standard shogi is first; provisional later order is xiangqi, tori, chu, tenjiku.
Independent rules research may proceed separately. The source inventory records
no evidence-based change to that provisional delivery order at present.

| Deliverable | Proposal and scope |
| --- | --- |
| Current-code implementation map | [implementation-map.md](implementation-map.md): actual chess boundaries and minimal explicit chess/shogi extensions, old-document compatibility, capabilities and preserved lossless admission |
| Complete named shogi profile | [shogi-profile.md](shogi-profile.md): selected `mshogi.shogi.csa36.v1`, primary editions/clauses, complete board/history/conclusion conventions and explicit software adaptations |
| Independent verification/reference/opponent audit | [shogi-verification.md](shogi-verification.md): immutable source pins and exact operation/semantic gaps; source audit is not runtime conformance |
| Selected contract coverage and current native evidence | [shogi-coverage.md](shogi-coverage.md), [legality-validation.md](legality-validation.md): bounded MSH-63–66 board implementation, reviewed fixtures/reference execution and exact limitations |
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

The first future implementation queue is MSH-64/65, then MSH-66; affected
history/outcome work MSH-67 also waits for SHOGI-P1/MSH-63. MSH-69/72/73 supply
early shared mechanisms without waiting for final MSH-41 or MSH-46. MSH-75
eventually verifies full shogi delivery; MSH-88/89 and MSH-47 retain final
all-game/platform and M4 closure obligations.

MSH-62 did not execute that queue, certify any shogi rule, run a
reference/opponent match, implement another game or close an M4 delivery.
M5/M6 obligations remain future work. Planning verification and independent
review are recorded in [validation.md](validation.md) when complete.
