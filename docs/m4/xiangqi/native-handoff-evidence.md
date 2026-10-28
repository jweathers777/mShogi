# MSH-84 native consumer-readiness evidence

Integration baseline `f548e8225807484bfd3eae1246c4fec0c2db1488`, tested production
source `601399502486a9143488cafe9bfa21dc3d24976d` from MSH-105. Root@codex is
sole writer/integrator in the primary `codex/rewrite` checkout. No isolation
worktree or competing builder is needed. All seven direct dependencies were
Done and independently evidenced before activation. No dependency/criterion is
removed, and no later consumer or architecture task is started.

## Scope verdict and acceptance map

MSH-84 verifies the native handoff as one system. No production refactor is
needed: the game-owned `mshogi_core::xiangqi` module and existing typed native
record/command adapter supply the coherent contract. This change adds only
consumer documentation, integration tests and task evidence. It does not add
an engine/search/console/UI, an exact-mode canonical game or a universal game
framework. [The contract](native-consumer-contract.md) is the consumer index.

| Original criterion | Concrete evidence |
| --- | --- |
|1 authoritative game-owned state/actions/effects/reversal/history/record boundary | New public-API `xiangqi_handoff` journeys below compose actual Game, local action resolution, events/warnings/claims, native records/envelopes, exact requests and shared context. Actual engine routing still refuses unsupported work. |
|2 independently expected native fixtures / applicable topology and terminal semantics | Reuse91 admitted board/reference corpus,93 full local legality,94 independently reviewed WXF histories and105 goal/proof fixtures. New cross-boundary expectations use F002, NATURAL-11-CHECKS, the reviewed mate/stalemate roots and105 bare-General/Advisor/two-ply seeds; no reference outcome oracle or changed expected data. Intersections/river/Red-Black semantics remain native; hands/promotions/compound routes are inapplicable, not replaced by dummy abstractions. |
|3 real shared consumption without duplicate ownership | Existing `CanonicalRecord`, `dispatch_command`, shared request/response and real internal-process regressions consume actual native payloads. New tests preserve typed replies, strict parsing, provenance, context matching and explicit unsupported/unscored execution.96/97/98/99/100–103 remain implementation owners. |
|4 prior native refinement / complete dependencies | Existing80 refinement split91fixtures,92state/codecs/reversal,93legality,94history/records/fastproof,95sharedrouting and105explicitexactwork before84 activation. Complete dependency set retained. CLI readiness note precedes active plan; no aggregate45 or later-consumer cycle. |

## Focused public-consumer journeys

New `crates/mshogi-engine/tests/xiangqi_handoff.rs` uses only public game-owned
APIs and the real native/shared adapters; no private helper access or replacement
rules engine. Its helpers forward requests and assert roundtrips, not adjudicate.

1. **Admission and ordinary capture:** structural facing-General FEN is
   representable but fails live-root admission. A Red Horse c3→e4 has empty d3
   leg and captures Black Soldier e4 in the sparse test root; mover/victim facts
   are explicit independently reasoned geometry. Local resolved effects/apply/
   undo agree with actual command/event state, canonical roundtrip and semantic
   undo. Wrong actor, illegal movement, equal-board foreign context and wrong-game
   dispatch reject without mutation.90 intersections/9×10/river4–5 stay explicit.
2. **History/warning/undo/fourth occurrence:** F002 check-cycle produces its
   third-occurrence warning at8plies/9events. Same-ply pre/post-warning snapshots
   differ in obligations, not board. A fresh same-board FEN has no known warning.
   Undo across the warning and reapply reproduces exact canonical data while
   changing runtime context and rejecting old exact work. Fourth occurrence
   awards Black the native penalty win; remaining local choices cannot bypass
   concluded-game admission. Warning and conclusion records roundtrip completely.
3. **Natural-count/audit/result:** independently counted11-check fixture at100
   plies leaves Red effective99/Black100. Red's rejected claim is retained audit,
   Black's accepted claim concludes at the same board ply. Canonical reopen and
   LIFO undo across conclusion/audit preserve the complete history and board.
4. **Native conclusions and terminal commands:** separately reviewed checkmate
   and stalemate roots both lose with distinct reasons. Initial44-action setup,
   ordinary play, invalid acceptance, valid agreement, undo to offer, decline and
   non-turn resignation cross canonical replay without duplicating native logic.
5. **Fast/exact uncertainty and provenance:** fast bare-General proof already
   draws; one-Advisor fast miss remains distinct from exact255-state closure.
   Accepted proof keeps its full source record, target/purpose/limits and work.
   Two-ply positive witness replays to native stalemate; exhausted/cancelled
   requests return distinct Unknown reasons, unsupported limits Failure. No
   report mutates Game or its complete save record. Selection/return and reopen
   invalidate old results. The existing105 tests additionally cover exact memory
   thresholds, every stop reason, counter overflow normalization, both native
   winners, independent exhaustive universe and actual same-board histories.
6. **Strict record/context/transport:** outer and nested duplicate fields,
   conflicting profiles, forged events and unsupported exact-mode restore reject.
   A real warned-game canonical payload crosses shared request/response; native
   history/request identity mismatches fail result matching. Actual server entry
   point returns unscored UnsupportedMode with empty Xiangqi capabilities.
   Canonical decode cannot resume old exact/runtime work or silently normalize
   malformed data before the authoritative native parser sees it.

These journeys check interactions among completed boundaries, rather than
re-proving every move/conduct category or measuring playing strength. Previous
independent expected data is unchanged.

## Reused evidence and exact limits

- [MSH-91/92 foundation](board-foundation-evidence.md),
  [MSH-93 local legality](local-legality-evidence.md): actual pinned/admitted
  Pikafish board comparator, independent choices/perft/sequence/apply-undo facts.
  No outcome helper becomes authority, and no reference rebuild is needed for
  unchanged production rules.
- [MSH-94 history evidence](history-evidence.md) and independent source/completed
  reviews: check/chase/warning/count/precedence/native replay/capacity facts.
  The reserved close/save path and exact original fixtures remain unchanged.
- [MSH-95 routing](routing-evidence.md): strict duplicate-preserving envelopes,
  exact encoded boundaries, native actor admission and real subprocess stale/
  cancellation/replacement/unsupported-execution evidence.
- [MSH-105 exact contract/reviews/measurements](exact-verification.md): reviewed
  existential board+turn proof domain, independently expected108+1020-state
  static universes and their reachable components, positive witness replay,
  limits/cancel/failure, capacity-versus-RSS measurements and Unknown classes.
  Defaults20kstates/200kedges/250ms/4MiB remain current measured settings, not
  permanent promises or universal practical feasibility.
- Tested production source6013995: [native CI40584949290](https://github.com/jweathers777/mShogi/actions/runs/40584949290)
  passed macOS/Linux/Windows,484/484/481workspace plus93desktopQA each;
  [package CI40584949296](https://github.com/jweathers777/mShogi/actions/runs/40584949296)
  passed all3, with actual installed/package GUI regressions on Linux/Windows
  and macOS package/bundled-engine checks. Unchanged M1/M2/M3/Chess/Shogi
  evidence is reused; this is not new Xiangqi GUI evidence or a macOS GUI rerun.

## Actual local checks and review

Environment: macOS arm64, pinned Rust1.89.0, Backlog.md1.55.1. PATHBacklog1.55.0
was read-only; all task mutations use the recorded pinned baseline.

- `cargo test --locked -p mshogi-engine --test xiangqi_handoff -- --nocapture`:
  PASS6journeys (1.11s initial execution).
- Separate initial readiness reviewer `/root/handoff84_review` read the real
  public APIs/implementations and selected profile/owner contracts, and ran
  `cargo test --locked -p mshogi-core --test xiangqi_exact --test native_xiangqi_envelope --test xiangqi_game_boundary`:
  PASS20(6+5+9), no ignored. Found no material production contract defect;
  requested current-status and provenance/uncertainty clarity, addressed here.
- [Final independent diff review](native-handoff-review.md) passed with no
  material findings; separate reviewer executed the new6journeys in addition to
  the20existing tests and inspected the complete handoff changes.
- Root normal native checks and final revision/CI are recorded in
  the task after their actual completion, without inventing a future source
  hash or claiming another platform from this local test.

Exact analysis remains explicit and read-only. Required unresolved adjudication
stays unscored; local `Ongoing` and absent Game conclusion are not a completed
no-win determination. Future consumers must retain original observation carriers,
native command authority, full context and runtime retirement. This handoff
closes neither complete Xiangqi45 nor M4, and does not start96/97/106.

## Completed root native verification

| Actual command | Outcome |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS490,0failed,2existing intentionally ignored deep-perft tests |
| `git diff --check` | PASS |

The workspace run includes the new six journeys and relevant existing native,
proof, strict envelope, shared-context, actual subprocess, search, match and
controller regressions. No production source changes, fixture changes, dependency
changes or extra package/GUI reruns are needed to establish unchanged behavior.
Normal triggered native CI on the integrated test/document revision is recorded
in MSH-84; unchanged production/package evidence above remains applicable.
