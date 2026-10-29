# Fresh native/source assessment

Reviewer: `/root/review103_rules`; revision541fc303,2026-10-29. Method: fresh
source/primary-source examination and actual test execution, not an author
reread or only prior ticket summaries. No code/task edits.

Examined current `crates/mshogi-core/src/xiangqi/{mod,codec,action,legal,conduct,game,record,assessment,exact,exact_request}.rs`:

- All seven piece kinds,90 intersections, complete local actions, royal safety,
  pins/evasions and exact checked transition/undo; structural admission versus
  local legality versus historical reachability remain distinct.
- WXF third-warning/fourth-continuation selected policy uses full contiguous
  histories and persistent victims, not a generic board-key draw. Source-derived
  C/Q/D/T mixed/overlapping histories, warnings/retirement and same-ply events
  cover G-CYCLE. Official37/38/108/109/110, real/apparent roots, pinned/blocked
  protection, General/Soldier exceptions and counter-chase frequency preserve
  the selected consequential classification. Permitted natures are not falsely
  advertised as a universal exact Idle/mating-threat classifier.
- Art8 claimant checks above10, capture reset,100 threshold, imported missing
  history, optional versus automatic claims and terminal actor/event ordering.
- Strict native RawValue parsing precedes map normalization; authoritative
  replay checks command result, full event prefix and final state. Semantic undo
  and committed Take back prefixes never rewind runtime freshness.
- Fast no-win is a narrow sound invariant/actual witness or Unknown. Explicit
  exact search uses complete ordinary native graph edges; only exhausted closure
  proves no-win, a replayable finite mate/stalemate path proves possible natural
  win, and unexplored frontier/cancellation remains Unknown. Goal excludes
  conduct/voluntary/administrative awards and is not forced-win minimax.

Commands, all PASS:

```sh
cargo test --locked -p mshogi-core --test xiangqi_board --test xiangqi_history --test xiangqi_game_boundary --test xiangqi_exact --test xiangqi_inspection_prefix
# 40 tests:5+16+9+6+4
cargo test --locked -p mshogi-core --lib xiangqi::
# 24 tests
```

The frozen board corpus contains105 cases, distinct from the admitted101
board-reference cases and520 sequence-prefix observations. Reused comparator
provenance was inspected; no new comparator build/differential campaign or WXF
outcome oracle is claimed. Exact tests independently enumerate108 bare-General
and510-per-Advisor-color palace universes, plus beyond-fast witnesses and all
resource/identity boundaries. Larger bounded work may honestly remain Unknown.

Fresh primary-source checks matched EN SHA256
`2b45534953bbfcca42dc2c52e06be48ff44ebe3bf7237248b12e39576c494cf3`
and ZH `f61f5c82ddaf2abb45eb25c24d58f2b278d2c150439f391d413c32c4ce275b3e`.
Reviewer read cached English Articles3/8/19–20 and visually inspected EN PDF
pages15/46/47 and ZH29/30. Historical bilingual/pixel diagram reviews remain
applicable; this was not a new complete116/92-page transcription. No new
source conflict was identified. Controlling owner G-NOWIN excludes the older
provisional penalty-inclusive proposal.

Disposition: no concrete native defect found. Whole-review synthesis supports
MSH-45 AC1–6 but requires AC7/full closure to remain blocked by F01. Reviewer
independently confirmed F01's shared Shogi/Xiangqi mechanism and P2 severity.
MSH-103 AC1/DoD2 cannot pass until correction. A misleading phrase suggesting
shared structured exact execution was corrected to native API/console only.
No new owner decision, rules change or broad architecture change is needed.
