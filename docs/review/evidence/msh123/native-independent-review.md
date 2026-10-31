# MSH-123 independent native notation/projection review

Reviewer `/root/notation_sources`; 2026-10-31. Read-only source review plus independently executed focused tests. Baseline HEAD `0b4218deaa8dbe705b882169d9e4cdab33333b0e` with integrator's uncommitted MSH-123 implementation. No repository/task edits by reviewer.

Scope: `moves.rs`, `moves/{chess,shogi,xiangqi,tests}.rs`, `shared.rs`/`shared_xiangqi.rs` snapshot wiring, native Xiangqi `command_event_ranges`, related native legal/action/history APIs and existing Chess timeline. Primary-source expectations independently researched in `/private/tmp/msh123-notation-research.md` before notation code review. Owner's separately approved rare-stack extension treated as display completion, not literal WXF text or rules change.

Verdict: **PASS; no unresolved material finding** in this bounded native boundary. This does not establish frontend layout, actual packaged GUI, all-platform behavior or whole-task acceptance.

- SAN uses native legal alternatives rather than attacks, correct file/rank/both preference, native post-state check/mate, EP/castles/promotion.
- JSA uses deduplicated native legal origins, movement-first then relative source, promoted identities, contextual drops/nonpromotion, both side perspectives, explicit no-直 Horse/Dragon exception and known previous destination only.
- WXF uses mover files/direction, straight distance versus diagonal file, front/rear and source FigureB; rare ordinal+file branch matches owner-approved completion. Candidate five-stack legality error from research was corrected rather than weakening native action admission.
- Only actual board actions become entries; selected cue resolves exact event-prefix counts. Xiangqi action prefix remains distinct from following derived warning and committed command-end; terminal/nonmove events retain prior cue. Canonical record is unchanged by projection.
- Root numbering exposes native counters/unknown history without fabricated moves. Imported second-mover roots align right column correctly.
- Secondary castling rook geometry comes from native transition delta. Shogi drop owns a hand origin. No frontend notation/action parser or rules duplication added here.
- Projection cache is transient and keyed by monotonically changing document/version; selection changes only select latest completed action. Xiangqi pre-action position uses immutable native cached snapshots; no Full/exact reconstruction path is entered by projection.

Actual command: `cargo test --locked -p mshogi-desktop-session moves -- --nocapture`. First checkpoint **11 passed**; final current checkpoint **18 passed, 0 failed**, comprising16 new notation/projection cases plus2 existing persistence tests selected by the text filter. Environment macOS arm64, repository native toolchain. No new public-field warnings on final checkpoint.

Final reviewed SHA256s before format-only changes:

- moves.rs `3077c26e459bc8183c13ed76a31f78dab65f4253fc33b1d99c42d8f167317263`
- chess.rs `c2d93988b508123a1e74f786cf9374efe4c52e53bd87147c125d10901a796ba4`
- shogi.rs `1e7a62ba90530a3713187b7f9db895d9250a1583d7ee313d63fd128c8ce790ba`
- xiangqi.rs `c4f2102abdbb25d543b33833c21786876b3f0f4cdc40d55a8b09486164ae93a0`
- tests.rs `3c3ad085762159a612f5bd2b2f23928e6e3c5bdfe8887bf0381832c05d2cbbbb`

Minor source precision communicated: FigureB fixture comment should cite PDF24–25/printed16–17 rather than only25/17. This is a comment-reference correction, not a behavioral blocker.
