# Standalone shogi console (MSH-68)

`mshogi shogi` selects the real untimed native game, profile
`mshogi.shogi.csa36.v1`. The host adapter parses and presents; `Game::submit`
owns action/actor/context admission, adjudication and terminal commands.
`mshogi shell`, `mshogi uci` and `mshogi internal-search` retain their separate
existing chess/protocol contracts. Human output never appears in protocol mode.

## Commands and semantics

`help` describes every command. `position startpos` creates a standard complete
history root. `position sfen full|reduced <four-field-SFEN>` starts a new known
segment with explicit material category and a **supplied** counter. Native
structural/live-root admission remains authoritative; necessary local conditions
do not prove historical reachability. Earlier checks and repetitions are unknown.
The counter is preserved, not expanded into invented events. Defaulted counter
provenance from an existing canonical record is retained on load.

`show` prints ranks a–i, files9–1, owner-qualified piece identities, both hands,
SFEN, current side, declared completed count, known event count and native
conclusion. `hands`, `sfen` and `status` are shorter inspections. `history` prints
the immutable profile edition, original root/category/counter provenance, full
accepted actor/command/check/declaration/conclusion events, exact post-SFENs,
known board-action count and repetition indices.

`move <USI-action>...` parses existing native USI board actions and submits the
entire sequence on private candidate state; failure commits none of it.
`undo [n]` uses native reversal tokens and counts **events**, not board plies.
The loaded canonical ledger is replayed through native admission to recreate
those reversal tokens, with a complete-game equality check before replacement.
Undo can therefore reverse a loaded declaration/resignation as well as a move.

`eligibility sente|gote` only queries the native evidence and creates no event.
`declare sente|gote` submits a formal entering-king declaration. Ineligibility,
including wrong turn, causes the approved false-declaration loss; it is not an
illegal board move. `resign sente|gote` preserves native terminal semantics and
does not require own turn. Terminal commands may change history and conclusion
without changing SFEN, board turn or declared completed count. Further commands
are admitted/rejected by the native concluded-game policy.

## Explicit readable notation

This is custom source-explicit long notation, **not** a claim to KIF, KI2 or a
standard western notation. Input remains exact existing USI: `7g7f`, `5c5b+`,
`P*5e`. Every `legal` row and accepted move pairs USI with a readable label:

| Meaning | Label |
| --- | --- |
| Sente silver4e to5d, retaining identity | `S:Silver[4e->5d;keep]` |
| Different silver6e to same5d | `S:Silver[6e->5d;keep]` |
| Optional/mandatory promotion selected | `S:Pawn[5c->5b;promote]` |
| Optional nonpromotion selected | `S:Pawn[5d->5c;keep]` |
| Already promoted pawn moving | `S:+Pawn[5e->5d;keep]` |
| Gote hand pawn dropped | `G:Pawn[hand->5e;drop]` |

`keep` means **no new promotion, retain the current identity**. It does not imply
demotion or that the mover is unpromoted. A mandatory nonpromotion alternative is
absent from the native complete legal list and rejects on submission. Owner,
source, destination, drop identity and promotion decision are always explicit;
two same-kind movers cannot collapse to one label. Captures and hand demotion are
derived natively; labels do not submit victim lists.

`legal` is explicitly labeled **local**. It lists complete position-level choices
in native deterministic USI order, including all drops and promotion choices.
An already concluded game can still have local actions; game admission rejects
further play. `perft`/`divide` are also position-level diagnostics, not search or
game-tree outcome counting. They ignore repetition, length and declarations,
normalize counters privately as documented by the native API, and never mutate
the live game. Perft0 is1 even at a terminal position; divide0 rejects.

## Bounded records and failure behavior

`record` prints a complete MSH-69 envelope with the native shogi record as raw
payload. `record save <path>` writes the same bytes. `record load <path>` checks
the complete envelope/profile/edition/schema and routes raw bytes to the native
strict parser, then restores the accepted game. This retains supplied/defaulted
counter provenance, root category, native policies, complete event history,
declaration evidence and conclusions. No FEN/SFEN-only or move-only fallback is
used. Wrong-game, malformed, duplicate/unknown, unsupported, tampered or oversized
input leaves the current session and its undo history intact.

Files are limited to1,048,576 encoded input bytes, including whitespace and outer
metadata. Reading uses at most cap+1 bytes and rejects excess before parsing.
The native event/record limits remain in effect; none were increased. Complete
record sizes are independent of the4096-byte interactive line cap. Paths consume
the remainder of the command (no shell quoting syntax) and may contain spaces or
UTF-8. Short text commands/SFEN/USI retain their native ASCII grammars.

Save is an atomic **new-file publication**: write and sync a uniquely created
temporary sibling, create the final hard link without replacing any existing
entry, then remove the temporary entry. Unsupported link/filesystem behavior or
an existing destination produces a clear save failure, preserving the session
and prior file. Choose a fresh path; this command intentionally has no overwrite
mode. This is a bounded console export, not the later desktop archive feature or
a cross-filesystem power-loss durability claim. The output must be a normal
filesystem file. Results go to stdout, escaped errors to stderr with a line
number. Exit0 means every command succeeded,1 means any failed,2 means invalid
program arguments. Oversized command lines drain to LF and recover.

To save/reopen a complete capture and terminal history (choose a new path):

```bash
cargo run --locked --release --bin mshogi -- shogi <<'EOF'
position sfen reduced 8k/9/4+p4/9/4R4/9/9/9/K8 b - 17
move 5e5c+
resign gote
record save /tmp/mshogi-complete.json
position startpos
record load /tmp/mshogi-complete.json
history
undo
status
EOF
```

After capture, Sente has a promoted rook5c and one unpromoted pawn in hand.
Resignation concludes a Sente win at event2 with only one known board action;
undo restores the live post-capture prefix. Saving again to the same path
explicitly fails; use a new path.

## Diagnostic limit evidence and verification

The initial console cap is depth2 (divide1–2). Local release measurements on
macOS27 arm64/Rust1.89, three process launches each, include startup and native
root admission. The standard first launch was cold248.384ms; the table reports
the median rather than hiding it as warm search work. These are measurements,
not performance/strength guarantees or independent perft expectations for the
drop diagrams.

| Root | Perft1 observed | Perft2 observed | Median depth2 process ms |
| --- | ---: | ---: | ---: |
| Standard initial | 30 | 900 | 6.285 |
| Kings9i/1a, Sente hand `2RBGSNLP` | 523 | 1,446 | 4.950 |
| Same kings, both hands `RBGSNLPrbgsnlp` | 523 | 251,223 | 107.912 |

The drop roots are `8k/9/9/9/9/9/9/9/K8 b <hands> 1` and explicitly imported
reduced-material diagrams. Depth2 already demonstrates substantial branching;
deeper diagnostics stay in the native library rather than silently blocking an
interactive request for an exponential depth chosen for chess.

`crates/mshogi-console/tests/shogi_cli.rs` executes the actual binary. Facts reuse
the independently established SH-F003/005/010/011 board fixtures and SH-F028/029,
034/037 native history/declaration corpus. A separate read-only fixture auditor
supplied the new two-silver ambiguity diagram: silvers4e/6e each have five free
moves, king9i has three, giving13 legal actions; both can move5d with distinct
source labels. Expectations were derived before product execution. Tests cover
actual outputs, exact SFENs/hands, atomic rejection, harmless query vs admitted
false declaration, same-ply resignation, repetition/perpetual-check conclusions,
unknown imported prehistory, canonical byte-equivalent re-export, undo after
reopening, hostile replacement failure and a valid record larger than4KiB.
Existing chess, UCI, internal-process and desktop subprocess tests remain in the
console suite. No rule, fixture expectation, mutual-check proof, native record
encoding or MSH-61 desktop persistence guarantee changed.

MSH-70 owns native shogi search and actual structured execution; MSH-71 owns USI
protocol interoperability. This human console does not advertise either engine
or protocol support by itself. MSH-41–47 and M4 remain open.
