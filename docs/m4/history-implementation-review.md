# MSH-67 fresh implementation review

Reviewed 2026-10-26 by a fresh, separate Codex AI reviewer. This is independent
AI assessment of another agent's implementation, not human approval or an
independent external rules engine. The reviewer made no product-code, fixture,
or task edits. The review is bounded to native MSH-67 history and records.

## Verdict and finding

**Affected behavior passes the checks below, but MSH-67 is not ready for complete
acceptance or integration while the mutual-continuous-check gate remains open.**
No additional material implementation defect was found in this bounded review.

**[P1] Resolve the mutual-check gate before accepting the legal-action contract.**
`crates/mshogi-core/src/shogi/game.rs:511–512` returns
`GameError::UnresolvedMutualCheck` after a locally legal board action produces a
fourth occurrence with both sides classified as continuous checkers. Because
`submit` works on a candidate and propagates this error before committing the
event, that branch discards the action and its history. The approved contract
requires admitted legal board actions to remain recorded with their governing
consequence. This review has not established that this branch is reachable; it
also has not established its impossibility. Therefore a sound impossibility
argument applicable to the admitted live-root domain, or a concrete legal
history plus sourced governing interpretation, is still required. The public
provisional rejection is not itself an accepted outcome. The implementation
and history-state document accurately disclose this limitation. Passing the
other tests does not close it. Separate proof research was outside this review.

## Inputs and exact revision

The reviewer read `AGENTS.md`, `docs/design.md` (especially A-02–A-06), the owner
instruction attachment `822c96fa-32d3-4486-bb35-a25038a483d1/Pasted text.txt`,
MSH-67, the selected `shogi-profile.md`, the fixture derivations, and the complete
game/record implementations and focused tests. Startup inspection confirmed
`codex/shogi-history`, remote `origin` pointing to `jweathers777/mShogi`, and the
primary checkout on `codex/rewrite`. No ownership or branch was changed.

Review base: `986a5277f26fe2f41c671bc0f14d2da5f749a156`, with the uncommitted
`mod.rs` export additions and new files below. SHA-256 values were unchanged
between the initial review fingerprint and the post-test fingerprint:

| File | SHA-256 |
| --- | --- |
| `crates/mshogi-core/src/shogi/mod.rs` | `e47143afcd4d011f048877664f176e7769659cd30f5b0f4c7925f1fca909b8f2` |
| `crates/mshogi-core/src/shogi/game.rs` | `a89d655e1b9a697eb79c7d1bc42c4e1c801fd4d96c1c0c113680e914b8bf73b8` |
| `crates/mshogi-core/src/shogi/record.rs` | `6a0d83575f0ee95b27c28c625b433f87f535b2631a34e3616ee5b1f41de97537` |
| `crates/mshogi-core/tests/shogi_history.rs` | `84c08d691930e8d068a96e3b1717094563721becd9716906ff12bdce26562cab` |
| `crates/mshogi-core/tests/shogi_record.rs` | `04208aaed6008b4c8478abbefbf18c92eee72254fbb37d3415c9f9c5a0364a63` |
| `docs/m4/history-state.md` | `22702d2b6c2f557de7958170982ec455f9fdac8eaf359676d0a13cf677a2e5a0` |
| `docs/m4/history-fixture-derivations.md` | `4fc092b4f803b5881fce4f9ef88fc7ac52d71d714cdb739229eb7496255ff03a` |
| `tests/fixtures/shogi/history-outcomes.json` | `07cd1f3892b372385ae2149a5766afc88f41d37b50d4f981813df17ddc5434ff` |

## Review method and behavior coverage

The reviewer traced root construction, request admission, event commit,
repetition/check indices, every conclusion branch, declaration arithmetic,
undo/prefix replay, and strict typed record decoding. Expectations were assessed
against the approved profile; native tests and native observations were not
treated as the authority for a governing outcome.

- `GameRoot::standard` is the exact initial root. Imported positions retain an
  explicit full/reduced category and supplied/defaulted counter provenance,
  rebuild diagram origin, and start at one known occurrence. A moved bare
  `Position` retaining `HistoryOrigin::Initial` cannot establish a complete
  initial history. Material categories and default-one provenance are checked.
- Live admission checks the nonmoving king and permits a checked moving king;
  the broad structural `Position` API remains available. This is documented
  as a local necessary condition rather than a reachability proof.
- `checks[first..fourth]` covers exactly actions following the first occurrence
  through the fourth, including the defender's final evasion where applicable.
  Each side's complete interval is tested, rather than only the latest cycle.
  Root, intermediate and shifted identities are retained. No-action loss,
  repetition/checking loss, and length use the selected order. The admitted
  single-checker losing move is retained with its event.
- Declaration queries are side-effect free. Formal declarations retain the
  actor and evidence, including wrong-turn false declarations; malformed wire,
  stale requests and wrong board-action actors take separate rejection paths.
  Resignation accepts either named side. Terminal commands preserve board and
  counter, and every command rejects once concluded.
- Semantic context includes the exact root and accepted actor/command ledger;
  this is distinct from repetition identity and a host's future monotonic
  execution identities. Undo checks/restores the complete game. Event prefixes
  distinguish a live board from its same-ply declaration/resignation outcome.
- The record uses strict typed decoding directly, including required nullable
  fields and unknown/duplicate-field rejection. Root category, profile edition,
  mode, policies, counter provenance and history origin are validated. Every
  event's index, actor, command, canonical state, check/declaration evidence,
  conclusion and score is compared with authoritative replay. Root/final
  conclusions and post-terminal events are separately checked. No lossy field
  acceptance was found in this inspection and the focused rejection tests.
- Capacity is bounded before JSON parsing by bytes, then by events before
  replay. Accepted untimed games cannot exceed 320 events because action320
  concludes, while a non-board terminal command ends earlier. The documented
  conservative envelope/event bound fits the 1 MiB native codec. The 320-event
  replay/save test covers a long accepted ledger and a same-ply terminal branch.
  This review does not claim shared transport or desktop archive integration.

## Independent fixture audit

The reviewer independently inspected all 21 history cases and nine declaration
diagrams, hand-checked the following geometry/arithmetic, and ran a separate
inline Python audit using no native APIs. The audit parsed SFEN into coordinates,
implemented direct piece attacks/ray obstruction, applied each corpus move,
checked moving-side safety and resulting checks, grouped exact board/turn
occurrences, and checked `root n - 1 + recorded actions`. All history hands are
empty in this corpus. It separately counted declaration camp pieces and points
from board/hand symbols, ignoring promotion for value.

| Case family | Independently checked expectation |
| --- | --- |
| Both owners, checked/check-relieved fourth | Three four-action rook/king cycles return at0,4,8,12. All six checking-side actions check; the final defender evasion does not erase the interval. |
| Quiet/interrupted checking | The quiet first rook move fails the all-check condition even when later cycles all check. Conversely, the noncheck at action1 is outside the2–14 repeated interval and does not interrupt it. |
| Overlapping identities | Rook4c/king4a with Gote to move occurs at3,7,11,15. This fourth occurrence precedes a fourth supplied-root return. Every Sente action inside that interval checks. All seven recorded partial occurrence sets match the independent grouping. |
| Declaration thresholds | Camp R+B+eight promoted pawns is10 other pieces/18 points; hand B+5P gives Sente28, B+4P gives27. Rotated Gote camp18 plus b+4p gives27, b+3p gives26. Promoted majors remain5. |
| Isolated declaration failures | Moving only the king outside camp preserves28/10; moving one camp pawn outside plus adding a hand pawn preserves28 but lowers camp count to9; rook9c checks K9a with8a available; changing only turn fails own-turn. |
| Imported counter/length | n320 starts at319 and the next action reaches320; n321/322/999 starts at320/321/998. n309 plus12 actions reaches320 where repetition/checking loss retains priority. Fresh n13 contains only one known occurrence. |
| No-action priority | K1i is confined by rooks2a/9h without check, or2a/1a with check. The pawn5c–5b mate has gold4c guarding capture and occupied king exits4a/6a/4b/6b. These losses precede the cap. |

The independent audit passed all 21 histories and nine declaration diagrams.
It establishes corpus geometry/arithmetic and interval bookkeeping, not a
global mutual-check impossibility proof or a new independent movement oracle.

## Commands, environment and results

Environment observed directly: macOS27.0.1 build26A434, arm64; stable
`rustc 1.89.0 (29483883e 2025-08-04)` and
`cargo 1.89.0 (c24e10642 2025-06-23)`.

```text
backlog instructions overview
git status --short
git branch --show-current
git remote -v
git worktree list
git log -3 --oneline
git rev-parse HEAD
sw_vers
rustc --version
cargo --version
shasum -a 256 <the eight reviewed paths listed above>
CARGO_TARGET_DIR=/private/tmp/msh67-freshreview-target cargo test -p mshogi-core --test shogi_history --test shogi_record
CARGO_TARGET_DIR=/private/tmp/msh67-freshreview-target cargo test -p mshogi-core --test shogi_legal --lib
python3 - <<'PY'  [independent inline fixture audit described above]
```

The first focused command passed **5 history +10 record tests**. The second
passed **11 library +7 existing shogi legality tests**. No tests failed or were
ignored. The Python audit exited0. No other platform run, packaged GUI,
playing-strength test, opponent campaign, or mutual-check proof verification
was performed by this reviewer; none is implied by these results.
