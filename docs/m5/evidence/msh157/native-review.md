# Independent native review — MSH-157 (Conventional Decision Improvement)

Reviewer did not author the production changes and inspected no held-out positions or results. Scope: current changes to `crates/mshogi-core/src/chess/game.rs` and `adjudication.rs`, their native/search callers and relevant history/adjudication tests. Source base `da4010b` with the bounded native diff. No builds, tests, profiling or engine runs performed during the ongoing baseline measurement.

## Source disposition

No correctness blocker identified in the two production changes. This is an initial code review, not final verification approval; new tests and their actual execution evidence remain to be assessed.

The reversible-repeat cache preserves the complete archival occurrence map. A pawn move/capture reduces a monotonic position property, and castling permissions cannot return; hence a post-boundary repetition identity cannot equal an identity before the last such boundary. For a reversible move, the full map's new-identity count therefore supplies the relevant suffix count. The flag is initialized false for both known and imported roots, retained on later reversible moves, reset at each irreversible boundary and restored from the saved step on undo. Existing `Game::apply` and search prepared transitions both converge on `commit_after`; synthetic null branches use detached positions and do not mutate the actual Game. No alternate rules/history authority is introduced.

The cached predicate remains a necessary rather than sufficient condition for a claim. Its consumers are catalog gating, search claim-action requirements and checked preview eligibility. The preview proof still holds after old irreversible-prefix identities are ignored: a child can reach at most its second relevant occurrence, and the subsequent mover has the opposite side from that possible repeat. Complete public counts and canonical records remain intact. Search keys continue using their separate exact reversible context rather than this boolean as a bound identity.

The intended-claim catalog keeps current claims first, then all sorted intended threefold claims, then all sorted intended fifty-move claims. It normalizes only the unplayed fullmove counter exactly as the unchanged public single-request validator does. Each intended action still uses the authoritative checked native resolution path; failed projections are omitted just as failed public validation was omitted. Repetition packing is skipped for fifty-only candidates. Threefold uses the prior occurrence count >=2, equivalent to adding the intended one occurrence; fifty uses the projected halfmove threshold. The public validator remains unchanged as an independent full-validation comparison boundary.

## Verification required for final approval

- Independent boundary fixtures for pawn moves, captures and castling-right loss, quiet continuation/new live repetition after reset, undo across boundaries and preserved archival counts/record replay.
- Complete ordered catalog comparison against individual public validation on threshold98/99/100/149/150, combined live repetition and fifty-move eligibility, intended fullmove-overflow semantics, special moves and terminal precedence.
- Native cursor preview/push/pop and stale-token/null restoration checks; broader existing history/adjudication/search regressions after the coherent source checkpoint.
- Actual local commands/source identity and outcomes, with no cross-platform inference.

Performance and decision quality remain separate gates. O(1) prerequisite lookup and reused checked projections are plausible cost improvements, not proof of faster representative searches or better Chess decisions.

## Added test review

Reviewed new unit tests in game.rs and integration tests in chess_search_access.rs. Hand-established reset fixtures cover pawn advancement, actual capture and rook movement losing castling rights; each preserves old occurrence counts and restores the complete Game by undo. A second fixture verifies new repetitions after the reset, including an uncapturable raw EP field whose legal repetition identity is equivalent, plus invalid-action/fullmove-overflow atomicity. Integration checks compare ordered catalogs to the unchanged public validator across thresholds, special moves, terminal precedence and a simultaneous intended threefold/fifty case. The preview fixture exercises obsolete-prefix recovery, commit/pop equivalence and later live-claim protection. Existing broad tests supply additional special-effect, stale-token, detached-null and complete-mode coverage.

No actionable test-design defect identified. These are meaningful independent expectations plus differential checks, not assertions derived solely from the new cached predicate. They have not yet been compiled/executed in this review session; final disposition remains conditional on recorded successful local checks at the final source checkpoint.

## Final native correctness disposition

**Approved for the bounded native exact-fix commit and subsequent cost/decision measurement.** Reviewed the formatted final production/test diff and `/tmp/msh157-checks/receipts.json` plus all corresponding log summaries. Current three-file SHA256s exactly match `source-hashes.json`: game.rs `d06dc6e56a123526f66c43a0f16fa4ea8a835dc4dca655dc1b6e2cac82951f8b`; adjudication.rs `76c7f1acae0f11ab10107ae820a576d3512370f05a8c6203ec38fd70aa09915b`; chess_search_access.rs `9d0617e43130ef3f7018da3353b9d05a13fd454174147528bcb7f793ec549e87`.

Recorded local verification passed: rustfmt; core library plus adjudication/search-access/history/record/target-query/special-action suites (232 tests across seven binaries); Chess search library selection (79 passed, one ignored); console claim admission (one passed); optimized frontier-example build. `git diff --check` also passed in this review. These are inspected coordinator execution receipts, not falsely claimed independent reruns. No material finding remains in the reviewed native diff. Level 1 is proportionate; no Linux/Windows/GUI evidence is inferred.

Approval establishes local rules/history/search-regression support, not representative performance or better play. Those measurement gates remain with the coherent round. No held-out fixtures/results were inspected and this reviewer ran no engine or test suite.

## Cost driver pre-run note

Separately inspected `/tmp/msh157-cost.py` before execution. Found a concrete receipt-validation error: some existing fixed-depth inputs omit `expected_root_fen`, while the driver unconditionally indexed that field. Asked author to independently replay/normalize expected root FENs before launching cells and to assert default diagnostics. This is a diagnostic driver finding, not a product-code defect. Cost-driver approval awaits that correction; native correctness approval above is unaffected.

## Completed cost evidence review

Reviewed repaired `/tmp/msh157-cost.py`, retry manifest/ledger and summary. The driver now independently replays each full input history using Python-chess, supplies missing expected root FEN with raw EP and rejects conflicting existing fields; default diagnostic flags are asserted. Original source fields remain intact. The original invalid-result attempt is preserved in `/tmp/msh157-cost-run/runs.jsonl`, hash-guarded and charged 0.041283625 seconds. Retry accounting includes it rather than resetting the allowance.

Verified all 24 retry attempts succeeded, their recorded artifact hashes match, and the current driver hash equals its manifest. Ninety-nine predecessor/candidate fixed-depth comparisons preserve full decision, selected move, score, nodes and completed depth. Three alternated one-second pairs per selected timing root show median own-visit ratios candidate/predecessor of 3.8561 (obsolete-history real root), 4.0486 (live-repetition root) and 1.0367 (ordinary-history root). Every timed selected move remained unchanged across versions/repeats. Total active charge including the failed attempt is 18.147348376 seconds within the reserved60.

**Approved as bounded local cost evidence.** This strongly supports removal of the specific claim/history bottlenecks at the affected roots. It does not estimate general speedup or playing strength: roots were selected for mechanisms, only three pairs per root were run, fixed-depth durations are often submillisecond, and the ordinary-root 3.7% visit difference is small enough to avoid a broad performance claim. The two production changes were measured together, so the observations do not individually attribute all live/obsolete gains to one change. Use per-root fixed comparisons rather than treating the pooled median over heterogeneous33roots as a workload speedup. Better Chess decisions remain a separate benchmark gate.
