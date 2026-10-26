# MSH-67 independent native game review

Reviewed 2026-10-26 by the separate Codex AI fixture author, after independently
constructing the expected history/declaration/import corpus. The primary native
game implementer was a different agent. This is independent AI implementation
review, not a claim of human review. Record-code review and mutual-check proof
review are separate scopes.

Reviewed files: `crates/mshogi-core/src/shogi/game.rs` and
`crates/mshogi-core/tests/shogi_history.rs`, against the approved profile,
current owner instructions, and independently established
[fixture derivations](history-fixture-derivations.md).

Base revision: `986a5277f26fe2f41c671bc0f14d2da5f749a156` plus the uncommitted
implementation reviewed. SHA-256 fingerprints identify those exact sources:

```text
a89d655e1b9a697eb79c7d1bc42c4e1c801fd4d96c1c0c113680e914b8bf73b8  crates/mshogi-core/src/shogi/game.rs
84c08d691930e8d068a96e3b1717094563721becd9716906ff12bdce26562cab  crates/mshogi-core/tests/shogi_history.rs
07cd1f3892b372385ae2149a5766afc88f41d37b50d4f981813df17ddc5434ff  tests/fixtures/shogi/history-outcomes.json
```

## Findings and disposition

No material mismatch was found in the implemented unambiguous outcomes:

- Root admission keeps broad `Position` analysis available, rejects a checked
  nonmoving king, allows a checked moving king, verifies full/reduced category,
  and retains explicit counter provenance. Imported construction deliberately
  replaces a bare position's old origin with a new unknown-prehistory segment.
  Exact standard-root construction is the only complete-initial-history path.
- The complete identity/check ledgers and slice `checks[first..fourth]` align
  position index zero with action1. The slice includes all actions after the
  first occurrence through the fourth. It correctly handles both checkers,
  checked/check-relieved fourth occurrences, interrupted checking, nonchecks
  before the selected interval, and earlier fourth occurrences of intermediate
  identities in overlapping cycles.
- The legal final action remains recorded for continuous-check loss. The
  imported root/after-action precedence agrees with the independently expected
  no-action, repetition and cap fixtures. A supplied counter does not invent
  older occurrences/checks.
- Formal declaration preserves the board/counter, retains the actual actor
  and evidence, and treats own-turn as a losing prerequisite failure.
  Advisory probes do not commit events. Either live side can resign.
- Stale, wrong-board-actor, illegal and post-conclusion requests reject before
  mutating the authoritative state. Exact semantic undo and authoritative
  event-prefix reconstruction preserve same-board-ply terminal events and
  distinguish identical positions with different root/history contexts.

The minor advisory-eligibility documentation finding is **resolved**. Follow-up
inspection on 2026-10-26 confirms `DeclarationEligibility::eligible` now describes
whether the actor's declaration prerequisites hold and explicitly says that this
advisory evidence grants no admission: concluded games reject commands. This
matches the intended distinction between prerequisite evidence and command
admission. The clarification was already present in the fingerprinted reviewed
source above; rechecking the current file produced the same SHA-256
`a89d655e1b9a697eb79c7d1bc42c4e1c801fd4d96c1c0c113680e914b8bf73b8`.
No additional source or behavior change was made by this reviewer, and the
focused test result below remains applicable.

The provisional `UnresolvedMutualCheck` branch remains a separately recorded
material gate, not an accepted complete behavior. It rejects a candidate
locally legal action if both checking flags are true. This is acceptable only
as clearly unfinished work pending the independent impossibility proof or
source interpretation. It must not be represented as a complete MSH-67 result
or silently integrated under the losing-cycle acceptance requirement. The
implementer already records the same limitation in `history-state.md`.

## Executed verification

Command run by this reviewer:

```text
cargo test -p mshogi-core --test shogi_history --target-dir /private/tmp/msh67-independent-game-review
```

Result: **passed, 5 tests, 0 failed**, including corpus history/import ledgers,
all declaration arithmetic/prerequisite fixtures, root provenance/admission,
rejection/resignation and same-position/different-history cases. Environment:
macOS27.0.1 build26A434, stable rustc1.89.0. These tests verify native game
behavior against the separately established expectations; they do not establish
the still-pending mutual-cycle impossibility theorem, operating-system coverage
outside macOS, playing strength, clocks or application integration.

No product source or test code was modified by this reviewer. Fixture wire-actor
wording was aligned with the implementer's single-envelope-actor API before
this implementation review began. A second conflicting actor inside a command
is malformed wire data; this layer does not claim caller authentication.
