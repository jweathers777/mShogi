# MSH-94 completed native boundary review

Reviewer `/root/xq_history_reviewer`, independent of root builder, conduct author and integration-test author. Review method: bounded source/code review of new `assessment.rs`, `conduct.rs`, `game.rs`, `record.rs`, their module exposure and history integration tests; independent WXF bilingual pixel/source/fixture review is separately recorded in `independent-source-review.md`. No comparator adjudication accepted as authority.

## Actual independent execution so far

- `cargo test --locked -p mshogi-core --test xiangqi_history`:13 passed,0 failed (5.66s test run); real main checkout, not copied crate. This establishes concrete11-cycle history tests, exact event prefix/reversal/audit, claimant arithmetic, actor-terminal behavior, strict forged record refusals and fresh restoration identities at the inspected source state. Later draw-offer changes need focused rerun.
- `cargo test --locked -p mshogi-core --lib xiangqi::conduct`:6 passed,0 failed (0.04s), real main checkout. Inspected conduct source SHA256 `a0e5a762286e917f652a9ba36674502d971acf68995e1003a360839d11fa2d0e`; proposed source corpus SHA256 `35b3c938f7bc694c7e4361c298a417d965753d1e56930709f2f425e8635a8fdc`. Final closed-witness hardening is pending a fresh check.
- Independent count arithmetic script and source fixture corrections are in the separate source report.

## Findings and disposition

1. **Material: chase warning retired on an open response prefix.** Original game recomputed a closed-cycle classifier after every action, then treated any error as proven variation. After a Red chase warning, its next Red continuation had no opposing reply yet, deactivating the warning and permitting wrong fourth-return draw. Root corrected this to preserve uncertain/open warnings and reevaluate closed anchors, with explicit `WarningRetired` history. TTT and other cycle integration tests now pass.
2. **Material: multi-step semantic undo blocked by runtime revision.** Original GameUndo required its original post-runtime revision, making stack undo fail immediately after undoing the latest command. Root separated full semantic checkpoint/command/lineage guards from monotonic request revision. The actual13-test suite now reverses multiple actions, warning, rejected-claim audit and terminal command exactly while preserving stale request rejection; wrong-order/restored-token refusal is checked. Public `Game:Clone` was removed; transaction copies are private.
3. **Source/API: unsourced immediate WithdrawDraw and repeated offers.** Root agreed to remove withdrawal and enforce/document after-own-move/no outstanding offer/window admission. WXF17.3H ENPDF40/32 and ZHPDF25/24 give no exact numeric reset for 'keep offering'; any concrete window reset must be named software admission, not falsely asserted as exact source wording. Pending final implementation/tests review.
4. **Intermediate warning state:** after a chase warning, a responsible checking move immediately proves variation under this classifier's check/chase mixture semantics. Original correction delayed retirement until anchor return. Root notified to add TTC and exact event-prefix retirement at first checking move. Pending final check.
5. **Source fixture mistakes:** false blocked real root and published109/110 ownership errors caught before acceptance, corrected with retained old data/finding provenance. Detailed source report records exact facts. No test expectation was silently changed to match native adjudicator output.

## Reviewed boundaries without additional finding at current snapshot

- Root/canonical identity/profile/schema explicit; bare FEN never creates pre-root history. Stable piece IDs distinguish chase victims from square/type equality.
- Strict typed outer decoding precedes final map equality; native root/current payloads remain RawValue until their authoritative parser rejects duplicates. Final shape/equality comparison catches missing nullable metadata; replay validates each command result, event prefix and redundant conclusion/proof/pending state. No intermediate map erases malformed native input.
- Board move actor admission differs from resignation/count actor semantics; automatic terminal precedence cannot be overwritten by later commands. Illegal/stale/capacity refusal is distinct from retained invalid claim audit.
- Full candidate admission checks actual encoded complete canonical record as well as command/event limits before commit, reserving terminal headroom. No accepted history/observations are discarded.
- History selection returns immutable snapshots without usable command context. Reopen obtains fresh runtime identity; semantic undo does not rewind request identities. Old execution contexts cannot become current merely by returning to identical board/commands.
- Conduct uses native complete legal captures/recaptures and before/after/response facts, not geometric attacks alone. Stationary unchanged Cannon attack in110 is excluded; changed Cannon screens in109 and Soldier mount cases remain causal. Real-root exceptions, same-type reciprocal/pin/blocked-leg facts,20.9 actor/victim exceptions and two-to-one counterchase frequency have concrete independently reviewed seeds.
- No invented mate-search/evaluation label: permitted non-chase labels need not all be called Idle. Unknown/invariant classification stays explicit, never fallback draw.
- Fast no-win proof obeys controlling owner natural board-win goal; two bare Generals and real one-ply mate/stalemate are scope-limited. A miss is `UnknownExactUnavailable`, not win-capable or draw.105 backend/feasibility is not claimed.

## Interim verdict

No final acceptance yet. Resolve/recheck findings3/4 and final conduct hardening; then capture final hashes and run focused tests. Broader integration/CI is root-owned and must be recorded honestly. This review does not establish GUI/platform journeys or MSH105 exact verification.

## Final conduct review follow-up

Independently reread closed-witness validation, current-ID reply projection and prefix upper-bound logic, and executed `cargo test --locked -p mshogi-core --lib xiangqi::conduct`: **8 passed,0 failed**, real main checkout, final conduct SHA256 `082e720cdcbccfc22a2d759ffd8468043478140459163e453273728bd51dfecc`.

Additional material finding was identified before wiring the prefix helper: an unanswered General/Soldier attack cannot yet create a negative20.9 combination exemption. Reviewer independently constructed `5k3/9/9/9/9/3C5/9/4c4/4K4/3n5 w - - 0 1`, cycle `e1d1 e2d2 d1e1 d2e2`. Red Cannon d4 pursues stationary Black Horse d0 via alternating General d1 or Black Cannon d2 screens. The General's simultaneous direct attack after e1d1 is not resolved by Black's reply; it remains available and is excluded from completed pursuit. Thus completed witness is prohibited Cannon chase. An unanswered-prefix shortcut originally could invent a General exemption and retire that warning at9, although full cycle remained prohibited. Author confirmed and fixed: do not add an unproven exempt pursuer from an unanswered reply. Regression now preserves warning continuity at9/10 and full fault12. Native MSH93 probe independently accepted/reversed the four-ply route; a temporary reviewer Rust program exercised the actual public Game12-ply continuation. No future accepted event is invented.

Closed board repetition with identical-piece permutation is accepted using current stable IDs, while an open one-step checking prefix is rejected before all-check priority. Prefix continuation is an upper bound for an already established chase: a checking move or empty causal/prohibited victim intersection proves variation now; a genuinely unanswered qualifying pursuit remains live until actual response. These guards resolve the warning-state findings above. No further material conduct module finding remains at the recorded hash.

## Later game/boundary execution

After OfferDraw/DeclineDraw/source-admission and prefix wiring changes, independently ran:
`cargo test --locked -p mshogi-core --test xiangqi_history --test xiangqi_game_boundary`.
Actual outcome: boundary **7 passed**, history **12 passed/1 failed**. The remaining failure is a stale test that offers a draw at the initial root; current source-consistent after-own-move admission correctly rejects it. Root/test author were notified to update this scenario and rerun. This failure is not represented as a passing full run.

Inspected production snapshots for that run: assessment SHA256 `e27485c454d3f2091bb02fe853a287c9de6d9854027da8e0726eb52dd8551f2c`; game `fb0acbf15fbd4b2c6247f43065d209d2eca5c460a596532b90d0529d83b169e9`; record `02bdd18b77dc6351ed5cd9d9bc2fc5c5cfd5b14803fa6bb30838d59b18c90575`. Wire mode `sound_fast_only` and positive witness depth1 are explicit; altered exact mode/limits are rejected rather than downgraded. Agreement offers now follow documented source-consistent software window admission, no WithdrawDraw capability; decline belongs to the opponent. Both rulebook languages prohibit repeated offering but do not numerically define reset interval, so the software window must be documented as an adaptation.

## Final completed-boundary verdict

**PASS at the inspected source hashes below; all material review findings resolved.** This is independent source/fixture/code review with actual focused execution, not an author reread or a claim of external association certification.

Final independent real-checkout execution:
- `cargo test --locked -p mshogi-core --lib xiangqi::conduct`: **8 passed,0 failed**.
- `cargo test --locked -p mshogi-core --test xiangqi_history --test xiangqi_game_boundary`: **15 history +9 boundary tests passed,0 failed** (5.68s +5.23s). The stale initial draw-offer test was corrected to offer after the participant's own accepted move, preserving terminal actor and same-board-ply checks. The prior failing run remains accurately recorded above.

The15 history tests now include the independently reviewed actual mutual Cannon cross-check cycle and34-ply route precedence fixture. The9 boundary tests include immediate proven chase retirement, genuinely unanswered pursuit preservation, the pending-General negative-exemption regression, complete byte/command/event boundary saveability with terminal headroom, native RawValue strict parsing and explicit unsupported proof-mode/limit rejection.

Final reviewed production SHA256:
- assessment.rs: `e27485c454d3f2091bb02fe853a287c9de6d9854027da8e0726eb52dd8551f2c`
- conduct.rs: `082e720cdcbccfc22a2d759ffd8468043478140459163e453273728bd51dfecc`
- game.rs: `fb0acbf15fbd4b2c6247f43065d209d2eca5c460a596532b90d0529d83b169e9`
- record.rs: `176ba898195234075a4580f4fa61a38efc2fc62d79b894ba736527e05240a5f1`

Final record change separately reread: invariant/capacity replay errors retain their distinct typed reporting instead of all becoming command rejection; no parser or acceptance relaxation. No additional material finding.

Scope limits: this establishes the MSH94 native history/adjudication/canonical boundary, not a universal move-nature telemetry API, playing strength, Xiangqi product consumers or full exact no-win verification. The fast proof only covers its documented bare-General invariant and genuine zero/one-ply natural mate/stalemate witness; misses remain explicit UnknownExactUnavailable. MSH105 remains blocked by94/95 and no feasibility success is claimed. Normal workspace regressions, relevant native CI and final integration/push evidence remain the integrator's completion responsibility.
