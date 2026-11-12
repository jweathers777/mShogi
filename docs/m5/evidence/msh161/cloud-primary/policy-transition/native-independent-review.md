# MSH-161 (Competitive Gameplay Diagnosis): independent native correction review

**PASS for the reviewed source checkpoint; Linux deployment/real-runner smoke remains required before batch03.** Reviewed all12files in the builder verification manifest, including production search/coordinator/UCI/runner paths, focused tests, example consumer and public documentation. Independently verified all12current file hashes match that manifest. No source edits or engine/match/cloud executions were performed by this reviewer.

The change is scoped correctly. Native automatic-outcome precedence and manual ExplicitOnly claims are untouched. SearchOptions defaults keep other native callers claim-capable; UCI always submits unavailable optional claims through immutable request context, including either legacy option value. Central claim admission suppresses root/interior/leaf/quiescent draw choices and floors; child critical guards use the same availability. Mandatory terminal checks precede claim handling and TT reuse. The TT is fresh per invocation, so fixed per-request availability cannot mix semantics in one table; no rules-history identity modification is needed. Coordinator context equality includes availability and the new forged-setting regression rejects a stale completion. Unexpected completed UCI Claim is an error; only the separate existing zero-iteration fallback remains.

Runner ExplicitOnly admission uses existing native outcome authority; historical AutoClaim remains supported. Generated UCI option replay now matches recorded configuration. The tampered opposite-policy command test is meaningful, and old runner tests retain historical replay coverage. No unrelated search heuristic, evaluation, variant or protocol change was identified.

Focused fixtures cover third/fourth repetition continuation and fifth mandatory termination;50-move continuation and75-move automatic stop; checkmate at the150-halfmove boundary; stalemate/dead positions; genuine legal PV-backed UCI moves at98/99/100half-moves; root/interior/quiescence unavailable claims with both native policies and exact/conventional TT-on/off variants; default manual claim preservation; request stale-identity; runner replay and tamper rejection. Existing native adjudication tests supply independently grounded mandatory-outcome expectations. Requirements/source review is recorded separately in independent-review-requirements.md, including the coordinator-verified FIDE Articles9.6.1–9.6.2 citation.

Builder evidence inspected: search library118passed/2existingignored; core adjudication9+records5+matchplay10; live UCI11; runner17; final TT-on/off focused2; final all-targets consumer compilation and formatting/diff checks pass. These are observed builder receipts, not independently rerun checks. Initial test/helper/example compile omissions are preserved in the builder record and corrected. Current source hashes match its final tested checkpoint; the final consumer log records successful compilation. No material finding remains.

Verification is targeted Level2 due protocol/process/request propagation. This receipt does not claim Linux or Windows execution, actual external-engine interoperability, deployed image identity or formal playing strength. Coordinator must complete the bounded Linux build and real paired-runner smoke, preserve old40AutoClaim games and source/policy strata, then verify future manifests before launching batch03.

Builder verification SHA256: `8c80c0491d230cba6d0cd2d0c75751d4d42a024a092cce73a1f013ba1637853a`

- `crates/mshogi-console/src/uci.rs`: `02c9c96ba2b4406d3cedb6bd40ee9a20e4baa0afb9614ce1ffb54be3ba6c6a0a`
- `crates/mshogi-console/tests/uci.rs`: `95558c49888db3a6e0fad7b000f5843df107700b7283e80326f794061974fe81`
- `crates/mshogi-runner/src/config.rs`: `9842455810b74c80d9e18b484862c967263be284b73ce0edc1e5429044079a80`
- `crates/mshogi-runner/src/report.rs`: `5bd2a0b32cc1c1c544b1252768c799786fdcdaf1383fc14b34d71883cdbbab5f`
- `crates/mshogi-runner/tests/runner.rs`: `5f607c6f8d1bde355e2679f858e5a487098a5361ac60e0dd17b6c6cdbccf3299`
- `crates/mshogi-runner/tests/support/uci_fixture.rs`: `f15dde6c111ce11704cabd5b044d97a91bbfff29b4b61f024e1116219e7ebcd8`
- `crates/mshogi-search/examples/chess_frontier_profile.rs`: `334ad9514ee5b5fa78013d2aaa1a9c99dfc838ed7838202d48099149d4e8527e`
- `crates/mshogi-search/src/chess_conventional.rs`: `3e97faf2ab9d3a7e843d6b65b92ac2daaae2c987c4039a071a204d984fddbe1a`
- `crates/mshogi-search/src/chess_conventional_tests.rs`: `994c6e47871b7959732147dab052edbe38c13ae221668bcc65ee11c74dbbd745`
- `crates/mshogi-search/src/coordination.rs`: `57c4a216456c5a373b08d10e5cece2550c46b8cb94ce1680e2ce79c7f995ec29`
- `docs/match-runner.md`: `207985e4e5e830fac6f7401d4e9e7db1f6edc6b18660aa5d23d1d6d85581c40a`
- `docs/uci.md`: `ce677e0e36dc7a5b5222efe04dae1a6a3934e717eecf8de2828da511ddfe82cd`
