# MSH-95 delegated subprocess verification

Method: isolated test-only delegate, not independent product review. Root remains sole product builder/integrator; a separate reviewer owns completed-boundary review. Tests use the actual `CARGO_BIN_EXE_mshogi` internal-search child through the existing single `EngineBridge`. Adversarial timing/echo behavior uses only the separately labeled `mshogi-internal-fixture` executable; no instrumentation is added to normal product paths.

Environment: macOS27.0.1 build26A434, Rust1.89.0, current uncommitted MSH95 source based on integration0a90ec8e8669b5dff70f37b8ceaec48d1702be1b. Not Windows/Linux application evidence. Root must bind final reviewed source and normal CI to an actual integrated revision.

## Owned changes

- New `crates/mshogi-console/tests/xiangqi_internal.rs`, SHA256 `5697bff3f5b802ce2f521713eeabdee579b7c091bfeaa98859d9ba90842f3387`.
- Existing verification helper `crates/mshogi-console/tests/support/internal_fixture.rs`, SHA256 `b35e0bdaa36677329ff20d62b9c14f84699ed8f2634afc70e21210fb2ab6132d`: only new adversarial case27, which restores the submitted real native Xiangqi record and adds a rejected native natural-count claim audit. This preserves board/turn while changing authoritative history; the supervisor must reject the echo.
- No source, task, docs, ownership or dependency changes by this delegate; no commits.

## Actual commands and outcomes

`cargo test --locked -p mshogi-console --test xiangqi_internal -- --nocapture`: PASS5/5,0ignored,0filtered,0.44s runtime after compilation.

1. Real child echoes complete imported-root/warning history without mutation; EngineTurn,MoveAnalysis and ChessFull-shaped operations all explicitly return unscored UnsupportedMode. Real native resignation adds terminal events without board ply and remains an unsupported execution record. No move/action/score/conclusion is invented in the failure. Empty Xiangqi execution capabilities asserted. Exact opaque work settings retained; encoded request/response sizes remain below existing caps.
2. Separately labeled adversarial fixture20..27 mutates process/request/session identity,work limits,purpose,selection,game family,or same-board native history. Every publication is the original submitted context with ContextMismatch, not the forged response.
3. One existing supervisor replaces deliberately sleeping pending/active fixture work in both Xiangqi→Chess and Chess→Xiangqi directions, publishes only replacement, rejects reused retired identities, supports cancellation and bounded shutdown acknowledgment. The30ms wait is scheduling opportunity, not evidence of a guaranteed child-start timestamp. Shutdown acknowledgment is worker exit after cleanup attempt, not an independent OS process-reaping proof.
4. Cancelled Xiangqi analysis remains retired after native semantic resignation/undo,canonical reopen and historical selection identity changes. Fresh monotonic identities accept current work only; native reopening has a fresh runtime context even when canonical semantic data equals the saved record.
5. A100ms supervisor watchdog returns an exact-context bounded Timeout failure; canonical game/warnings remain unchanged and unconcluded. This is a process failure, not an unsupported execution result or chess outcome.

`cargo test --locked -p mshogi-console --test internal --test shared_internal --test shogi_engine -- --nocapture`: PASS11/11,0ignored,0filtered. Existing actual Chess move/incomplete,Shogi move/analysis/declaration/incomplete,history-context and cross-game lifecycle tests remain valid; existing malformed/oversized/missing/nonzero-exit/start/timeout fixture and retirement regressions pass.

`cargo clippy --locked -p mshogi-console --test xiangqi_internal -- -D warnings`: PASS.

`git diff --check`: PASS.

During development, the first compile caught two test-only API mistakes (`Game::board_ply` did not exist and `Submission.undo` was required), fixed before the passing run. One verification invocation named nonexistent target `shogi_internal`; it did not run tests and was corrected to existing `shogi_engine` above. No unavailable or failing check is counted as passed.

Limitations: these are console subprocess/supervision and exact-context tests, not Xiangqi evaluator/search,desktop integration,or G-NOWIN exact-backend evidence. They do not startMSH105/97. Existing normal packaging selects only `mshogi`, not the separate verification fixture. Final integrated workspace/native CI remains the integrator's responsibility.
