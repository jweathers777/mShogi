# MSH-96 independent review and local evidence

Review method: separate Codex reviewer session `console96_review`, bounded
working diff from `fd5fe6f34bcff8299f0990fb4ea71ce1b194c1eb`, plus native
consumer requirements, task96 and final `console.md`. This is an independent
source/test review, not a rules-authority or external-engine comparison.

Verdict: no material findings. Reviewed command/native ownership, strict raw
canonical restoration, retained LIFO undo, candidate atomicity and actual
envelope saveability preflight, claims/terminal commands, fast/exact separation,
source provenance, one worker/one result slot, cancellation/cleanup and retirement.
No evaluator/search/external protocol scope was introduced.

Reviewer executed: `cargo test -p mshogi-console --test xiangqi_cli` (11 pass),
`--test chess_cli` (19), `--test shogi_cli` (8), `--test common_console` (9),
and the native reserved-capacity regression (1). Total48, zero failed. Initial
common-console obsolete unknown-game expectation was corrected to a genuinely
unknown spelling; reviewer rerun9/9. No remaining production findings.

Integrator on macOS arm64 / pinned workspace Rust: `cargo fmt --all`,
`cargo clippy --workspace --all-targets --locked -- -D warnings`,
`cargo test --workspace --locked` all passed. Existing intentionally ignored
reference/benchmark cases remain ignored; none is presented as newly executed.
The two initially incorrect history fixture assertions and compile borrow fix
are recorded in console.md; rules and existing fixture expectations unchanged.

Final source revision, native CI/platform results and integration are recorded
in the MSH-96 task. Earlier M1/M2/M3 and MSH-84 evidence is reused only for
unchanged behavior. Desktop GUI interactions were not rerun for this console task.
