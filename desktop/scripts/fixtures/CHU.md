# Chu ordinary-Load fixtures

These are development test inputs for the normal current desktop envelope and
ordinary native file dialog. They are not bundled into the application and do
not require the desktop-verification feature or external fixture selector.
The production engine, runtime, document admission and renderer remain the
normal application components.

Regenerate from the integration checkout:

```sh
cargo run --release --locked -p mshogi-desktop-session --example write_chu_gui_documents -- desktop/scripts/fixtures
cargo test --locked -p mshogi-desktop-session --example write_chu_gui_documents
```

Both engine scenarios use the same fixed probe limits (depth 1, 512 visits,
no time limit). The generator records these limits in the manifest and checks
the completed depth against the requested depth.

The generator creates strict `GameRecord`s, current-envelope
`ValidatedChuDocument`s, and a `chu-game-fixtures.json` manifest. It decodes
every complete archive and compares its exact bytes, game record and events.
The companion tests assert actual game effects against the independently
authored MSH-82/86 expectations in `docs/m4/chu/fixtures.json`; expected facts are
not inferred from an engine's choices. They also check atomic rejection of a
partial action, a forbidden pre-fourth repetition departure, quiet/capture-return
distinctions and the availability of both promotion choices.

Except for `chu-complete-resignation`, each root is an
independently authored diagram admitted by the kernel. Earlier history and historical
reachability from standard start are not claimed. Rank letters and file numbers
below name coordinates, independent of board orientation. A two-leg
action is one complete turn; a partially selected route is not a move.

| Archive suffix | Independent source | Normal GUI journey and expected fact |
|---|---|---|
| `contract` | CHU-RETURN-Ln-enemy | Lion `6fx7e-6f` captures one Pawn and returns; `6f-6e-6f` quietly returns. Distinct complete actions share their endpoint. Partial route can cancel without an event. |
| `captures` | CHU-LION-C2-1 | Lion `3cx2bx1b` captures Gold then Lion in that order; the first complete action does not conclude. |
| `promotion` | CHU-PROMOTE-DE-YES/NO | `6e-6d` offers both choices. Promoting the Drunk Elephant produces a Prince while its original King survives. |
| `royal-both` | CHU-ROYAL-BOTH-COMPOUND | Lion `6fx6ex5e` captures the enemy King and Prince in one complete action, concluding a Sente royal-capture win. |
| `royal-survivor` | CHU-ROYAL-SURVIVOR | Gote Rook `12jx12l` captures Sente's King, but Prince `8j` survives and the game remains ongoing. |
| `material-reply` | CHU-MATERIAL-DECLINE-COUNTERCAPTURE | Gote can equalize with `6dx6e`. Choosing `6d-5c` instead produces the narrow Sente material award after the complete action. |
| `mate` | CHU-MATE | Already concluded Gote finite-mate diagram; game board movement is not filtered by Chess self-check. |
| `agreement-root` | CHU-MATERIAL-AGREEMENT | Two Kings do not automatically draw. A formal offer alone does not conclude. |
| `repetition8` | CHU-REP-QUIET | Eight accepted actions, three root occurrences, Sente must vary. The next departure is still admitted. |
| `repetition10` | CHU-REP-QUIET | Ten accepted actions. Sente `1k-1l` is absent from game choices and rejected atomically. No fourth occurrence or automatic repetition loss is committed. |
| `agreement` | CHU-MATERIAL-AGREEMENT | Sente's offer is already recorded. Human Gote may accept even though board turn remains Sente. Acceptance concludes a draw without advancing board ply. |
| `agreed-draw` | CHU-MATERIAL-AGREEMENT | Exact two-event offer/accept history, zero board actions, agreed draw. |
| `royal-concluded` | CHU-ROYAL-BOTH-COMPOUND | Exact one-action naturally concluded royal-capture record. |
| `material-concluded` | CHU-MATERIAL-DECLINE-COUNTERCAPTURE | Exact one-action naturally concluded material record. |
| `complete-resignation` | Standard root and explicit scripted participant command | Twelve actual depth-1/512-visit mShogi engine turns, then Sente resigns. All 13 game events survive canonical replay; engine observations are not saved. |
| `complete-royal-engine` | CHU-ROYAL-BOTH-COMPOUND plus an explicitly added remote Gote Gold at `11a` | Actual engine Choice completes two royal captures and produces a Sente royal-capture win. A second actual engine request returns Finished; the exact complete action survives replay; engine observations are not saved. |

The standard-root resignation record is a complete **recorded game with an
explicit resignation ending**. It is not evidence that the engine naturally
completed a self-play game, nor evidence of competitive playing strength. Its
move choices are actual implementation observations; the expected final Gote
win follows from the independently specified Sente resignation, not its score.
It is separate from the earlier 32-ply bounded smoke. Natural-ending
scenarios above are complete from their explicitly limited diagram roots.

`complete-royal-engine` additionally establishes a natural engine-produced
ending, distinct from the explicit resignation. Its extra remote Gold prevents
the single-royal-capture material ending that is also legal in the unmodified
source diagram. The added piece cannot affect the Lion's two target squares or
attack the remote Sente King. Either remaining Gote royal would have a live
nonroyal companion, so the independently expected royal-capture result requires
removing both royals. The source fixture is unchanged. The manifest identifies
this derived root and records the actual engine-selected route; no expected
route is forced onto search and no heuristic score is converted into a result.

The minimal document holds the game record, human side and settings. Engine
results stay in memory for the open session. Load restores the live game with
empty results, stops work and allocates fresh runtime identities. The removed
capacity fixture tested a saved-observation limit that no longer exists.

These inputs do not establish arbitrary long-game capacity or a platform GUI
result. The packaged GUI campaign must perform the interactions; generating
these files alone does not establish GUI success.
