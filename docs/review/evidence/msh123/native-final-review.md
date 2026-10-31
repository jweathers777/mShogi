# MSH-123 final bounded native checkpoint

Reviewer `/root/notation_sources`, read-only, 2026-10-31. Integrator's working source on HEAD `0b4218deaa8dbe705b882169d9e4cdab33333b0e`; final source commit not yet made. This follows the independent primary-source research and earlier18-test review. **PASS; no unresolved material finding within native notation/projection scope.** Whole-task/package/platform acceptance remains separate.

Inspected final formatted `moves.rs`, notation modules, projection tests, shared snapshot wiring, real shared-consumer tests, native Xiangqi range accessor, `docs/review/moves-123.md` and selected completion in `docs/desktop/post-r4-ux.md`. Removing the duplicated helper `event_index` parameter makes `PresentedMove` derive that identity from native geometry; all three producers supply the same exact native prefix as before. No semantic regression found. Public fields now document origin/order/event-prefix/command meaning. Core accessor correctly distinguishes event-array indices `[start,end)` from post-event prefix IDs `(start,end]`. FigureB comment correctly cites PDF24–25/printed16–17. Source and design distinguish owner-approved rare ordinal+file completion from literal WXF text.

Independently executed on macOS arm64:

- `cargo test --locked -p mshogi-desktop-session moves -- --nocapture`: **21 passed, 0 failed** (19 new notation/projection/consumer tests plus2 existing persistence matches).
- `cargo test --locked -p mshogi-desktop-session shared_moves -- --nocapture`: **3 passed, 0 failed**, subset of the above, deliberately rerun after reading actual controller/admission/Save/Open/Take back and historical-analysis tests.
- `git diff --check`: **passed**.

No native full suite, package, GUI or CI claim is inferred from these focused commands. No source/task edits were made by reviewer.

SHA256 final reviewed source/docs (prior future evidence-only append):

| File | SHA256 |
| --- | --- |
| moves.rs | e34e3c23059f7960d38ff7f280635de11efafbe4c2b8faeabeb1640d49f320d5 |
| moves/chess.rs | d8cefd81c46a30e97453a15943ad48c79cc4134f65f45d3d8698a50061d61a57 |
| moves/shogi.rs | 1e7a62ba90530a3713187b7f9db895d9250a1583d7ee313d63fd128c8ce790ba |
| moves/xiangqi.rs | b78ba3500dc5eaa86faa9312d94ac937bb356db5e8ea0a48f419af265e700274 |
| moves/tests.rs | 4bbf1278639b0c33118fe31e20cd62805f5882db8c34d56ec350f55a42da2e3e |
| shared.rs | a1a6dc4509f0b1141d59f3c96bb0223084afb6c5dc811f289c6035a6b74aadf6 |
| shared_xiangqi.rs | 3175f759af8282b2f6b50a965ffdae2cc3f6ce6196f00ef0ffcf911325f0390b |
| shared_moves_tests.rs | 92f4a18281e674a5d739c90a8e69a41c0b07225f7f23bc1d9a9a85286ac5098b |
| core xiangqi/game.rs | 8b06419d5abad306b9fb73a8075a186b6fe495444835a77a50f771593a8278ec |
| docs/review/moves-123.md | 7838c66c2999b946b20df0f309f6ac183de1ccb372b77c93415ecf0bdf782d44 |
| docs/desktop/post-r4-ux.md | 91ce15c7944f365ce040c6c535d1047e4e42f2e09d175e349dfcc7c2606ae8fb |
