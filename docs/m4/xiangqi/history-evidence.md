# MSH-94 native history, adjudication and record evidence

Integration target: `codex/rewrite`, primary checkout. Baseline
`067d1767d06fa0e781aff2a325e4b21a9ecf1d75` (board source
`a71d27b1bca2d75d789ead1acc2f8a3132dcdc81`). No other active worker's task or
unrelated dirty files were claimed. Root@codex is the primary builder/integrator;
isolated conduct module and fixture work were delegated. No disposable worktree
was needed. Final integrated source and native CI references are recorded below
and in the MSH-94 task after actual completion.

## Scope and acceptance mapping

| MSH-94 criterion | Concrete evidence |
| --- | --- |
|1 complete history policy / native commands / classifications | [contract](history-contract.md), persistent IDs and full contiguous anchor witnesses;20 [conduct witnesses](../../../tests/fixtures/xiangqi/history-conduct.json), published37/38/108/109/110 source review,11 cycle histories, mutual-check and precedence fixtures;8 private conduct test functions +15 history integration tests |
|2 distinct opportunities/events/results/failures | Advisory count query versus accepted/rejected submission events; warning/retirement separate from board action; explicit winner/reason/pending/assessment; actor admission, board/event prefixes, no overwrite, no inferred pre-import evidence; focused integration/boundary tests |
|3 strict canonical replay / exact undo / identity | `xiangqi_game_boundary.rs` strict outer/native RawValue duplicate/unknown/missing/conflict/unsupported-mode cases; replay-derived command/event/metadata checks, full multi-step reversal, fresh reopen token, retired branch/request refusal; actual encoded byte/event/command boundaries and reserved native closure |
|4 independently reviewed histories / proof/unknown | Frozen source corpus and independent pixel review,10/11 checks at99/100/101, capture27 reset/import counters, 108 bare-General invariant placements/successors, actual0/1ply mate/stalemate witnesses, F002 repetition penalty excluded as natural capability; failed development runs/corrections retained |
|5 source/owner gates before affected acceptance | [G-NOWIN owner resolution](owner-decisions.md#g-nowin-resolution-during-msh-94-2026-10-27); source/fixture reviews resolve G-CYCLE and R-CLASS, exact backend remains105 with94/95 dependencies; no required proof scope shifted to M5 |
|6 manual/automatic claimant arithmetic / order / saveability |4 natural-count histories plus independent placement/check/count script; optional claims remain optional, rejected claims retain bounded audit; automatic Red-first named policy, board terminal→repetition→proven no-win→automatic claim ordering; repetition102 versus claim100 fixture; no untimed penalty; actual full record modes/limits measured before commit |

Links in the fixture ledger resolve from repository paths; source files live in
`crates/mshogi-core/src/xiangqi/{assessment,conduct,game,record}.rs`, with focused
integration suites `crates/mshogi-core/tests/xiangqi_{history,game_boundary}.rs`.
Existing Chess/Shogi native APIs and shared/desktop consumers were not changed.

## Independent research and review

Actual methods, not an author reread:

- `/root/xq_cycle_sources`: disjoint WXF cycle/count/precedence research,
  independently expected histories and isolated public-API fixture suite.
  [Cycle report](history-reviews/cycle-source-review.md),
  [count report](history-reviews/natural-count-review-notes.md).
- `/root/xq_conduct_sources`: independent bilingual source/diagram investigation,
  reviewed20-witness corpus and isolated conduct module only; root owns game,
  assessment, record and final integration.
  [Conduct report](history-reviews/conduct-source-review.md).
- `/root/xq_history_reviewer`: separate from those authors, independently read
  official bilingual PDF pixels, checked diagram ownership/geometry and count
  arithmetic, reviewed completed production boundary, constructed an additional
  unanswered-General exemption counterexample and actually ran focused tests.
  [Source review](history-reviews/independent-source-review.md),
  [completed boundary review](history-reviews/completed-boundary-review.md).
  Final verdict **PASS**,8 conduct +15 history +9 boundary tests executed by
  reviewer. Production SHA256s in that report bind the reviewed implementation.

Material findings resolved before acceptance: premature chase-warning retirement
on missing reply; an unproved General/Soldier exemption on an unanswered prefix;
semantic multi-undo conflicting with monotonic runtime revision; unsourced offer
withdrawal/repeated offer admission; protected-Rook geometry and published109/110
ownership transcription errors. Stale initial-root draw-offer test failed after
source-correct admission and was corrected to an after-own-move scenario. Reports
retain those failed/interim runs. No expected rule outcome was rewritten to match
a comparator. The initial proposed penalty-inclusive G-NOWIN goal was rejected
by the owner; historical proposal and final supersession both remain recorded.

## Local verification

Environment: macOS Darwin27.0.0 arm64, pinned stable Rust1.89.0
(`29483883e`, LLVM20.1.7), Cargo1.89.0. Actual commands on final source:

| Command | Outcome |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS:462 passed,0 failed,2 existing ignored deep-perft tests; includes actual Chess/Shogi subprocess/regression suites and32 new conduct/history/boundary test functions |
| `cargo test --locked -p mshogi-core --lib xiangqi::conduct` | PASS8, independent reviewer |
| `cargo test --locked -p mshogi-core --test xiangqi_history --test xiangqi_game_boundary` | PASS15+9, root and independent reviewer |
| `cargo clippy --locked -p mshogi-desktop-session --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --locked -p mshogi-desktop-session --all-features` | PASS92,0 failed |
| `cargo run --locked -p mshogi-desktop-session --example write_frontend_fixtures -- desktop/src/__fixtures__` then `git diff --exit-code -- desktop/src/__fixtures__` | PASS; unchanged native frontend fixtures |
| `cargo build --workspace --locked` | PASS |
| `python3 tools/xiangqi/review_count_history.py --output <temporary-report.json>` | PASS4 source-count fixtures; independent placement/Rook-check/capture/count arithmetic, no native/reference outcome calls |

The two ignored deep-perft cases remain intentionally opt-in; no deep-perft run
is newly claimed. Unchanged board conformance/reference build and M1/M2/M3/
Standard Shogi/desktop product evidence remains applicable. No new GUI journey,
playing-strength test, exact no-win feasibility result or Xiangqi engine consumer
is claimed by this native task.

## Board comparator observations versus history authority

The previously built/admitted Pikafish board-only probe remains pinned to
`9d7833aa113626b3ea63ad83f49fb9b456bd5839`; retained build/compiler/patch/binary
provenance is [admission](reference/pikafish-admission.json). Its admitted scope
is complete local legal choices, board/side/fullmove and check, **not** WXF
warnings, claims, conduct results or no-win proofs. The reference's adjusted
half-counter is explicitly excluded from equality, never substituted for native
claimant-specific history. Expected WXF outcomes come from sources/owner and
independently reviewed histories.

Original development observations:143 cycle native-board prefixes,108 conduct
native-board prefixes;233 natural-count,103 precedence and13 mutual-check paired
native/admitted-reference prefixes. These are supporting local-board evidence,
not differential adjudication. Reproducible collector/final-source paired summary
are retained below when completed, avoiding bulky generated raw transcripts.

## Native capacity and limitations

512 commands,2048 events,1MiB actual encoded canonical payload; nonterminal
admission reserves1 command/4 events/4096bytes. The focused511→512 test proves
an additional rejected-claim audit is refused before mutation, while resignation
still fits, preserves every audit/event and roundtrips. Byte/event/command parse
boundaries and unsupported restoration settings are also exercised. This does
not implement desktop archive99; its additional envelope/observation/control/
replay accounting must preserve MSH-61 independently.

Only untimed native play, `Explicit` or named `AutoValidClaims`, and recorded
`sound_fast_only`/one-ply witness assessment are supported. No clock, physical
procedures or evaluation-based result policy is added. Full exact natural-win
verification105 remains mandatory and unstarted until94/95 are Done. No later
Xiangqi implementation began. MSH-45, MSH-46/47 and M4 remain open.

## Final-source board refresh

Actual final-source `cargo build --locked -p mshogi-core --example xiangqi_probe`
followed by
`python3 tools/xiangqi/collect_history_boards.py --report <temporary-report.json>`:
**PASS17 histories,492 native prefixes,492 admitted-reference comparisons,0
native-only prefixes**. The [compact summary](history-reviews/board-observation-summary.json)
retains each fixture/input/source/binary/manifest hash, exact command/environment,
per-fixture counts and ordered raw-observation stream hashes. Expected corpus
files were unchanged. The report identifies baseline HEAD plus dirty-tree source
hashes because the executed final source was not yet committed; the integration
commit below contains those exact reviewed production files. The Cargo build
establishes native executable provenance; merely hashing working files would not.
Native probe SHA256 `c1a5a9953b9984fd4b71336a05c20aacfae18a6707cee139190143ac3c8e12f9`.
This refresh upgrades the earlier143 native-only cycle prefixes to paired board
observations; it does not upgrade Pikafish to a history/outcome oracle. Conduct
witness108 native prefixes and independent outcome fixtures remain distinct.

## Integrated production source and native CI

Implemented/reviewed/tested source, normally committed and pushed to
`codex/rewrite`: **`a822450d1618810e0ef37efb9ef05035939c5746`**.
Later evidence/task/documentary corrections do not change the production hashes
recorded in the independent review. No history rewrite or force push was used.

[Native CI40569025619](https://github.com/jweathers777/mShogi/actions/runs/40569025619)
completed **success** for that exact source. Actual log-derived results:

| Runner | Workspace tests | Isolated desktop QA | Other required workflow checks |
| --- | --- | --- | --- |
| `windows-2022` |459 passed,0 failed,2 intentionally ignored |92 passed,0 failed |fmt, workspace/all-feature clippy, native frontend fixture regeneration unchanged, build and console smoke PASS |
| `macos-15` |462 passed,0 failed,2 intentionally ignored |92 passed,0 failed |same workflow checks PASS |
| `ubuntu-24.04` |462 passed,0 failed,2 intentionally ignored |92 passed,0 failed |same workflow checks PASS |

All runners actually used pinned Rust1.89.0 (`29483883e`). Platform-specific
counts are retained rather than inferring Windows execution from Unix counts.
No new Xiangqi console/engine capability is implied by existing Chess/Shogi
console smoke. MSH-105 remains unstarted; MSH-95 is the next dependency-ready
native boundary after94 closes, and105 still waits95. MSH-45/M4 remain open.

The automatically triggered
[Desktop packages40569025438](https://github.com/jweathers777/mShogi/actions/runs/40569025438)
also completed **success on Windows2022/macOS15/Ubuntu24.04** at the same
production source. This establishes the workflow's locked frontend/native
checks, normal package construction/inspection and bundled-engine checks;
Linux and Windows additionally executed the existing installed/separately
packaged Chess/Shogi GUI regressions. Normal-versus-verification package
separation checks passed. The macOS job is package/bundled-engine evidence,
not a new macOS interactive GUI campaign. Earlier accepted macOS interaction
and native termination/Tao provenance evidence is reused unchanged.
No Xiangqi GUI or engine capability is implied. No unrelated product campaign
was manually repeated; these were the existing automatic integration workflows.

Final documentation follow-up only clarifies fourth-occurrence terminology and
retains the separate documentary PASS review; all reviewed production hashes
remain identical to tested integration `a822450d1618810e0ef37efb9ef05035939c5746`.
The final MSH-94 task summary records completion and unchanged future dependencies.
