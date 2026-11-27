# Standalone shogi console (MSH-68, common interface MSH-90)

`mshogi --game shogi shell` selects the real untimed game, profile
`mshogi.shogi.csa36.v1`. The host adapter parses and presents; `Game::submit`
owns action/actor/context admission, adjudication and terminal commands.
`mshogi --game chess shell` uses the chess adapter; `mshogi shell` retains that
default.
`mshogi --game chess profile` and `mshogi --game shogi profile` identify actual
coverage; `--game <game> --help` and shell `help` explain capabilities.
Unknown games, missing selector values, repeated selectors and extra modes fail
with exit 2 and stderr only. Selectors may precede or follow the one mode.
`mshogi --game chess uci` and the default `mshogi uci` preserve UCI. An explicit
shogi/UCI combination rejects; [USI](usi.md) uses `--game shogi usi`. `mshogi internal-search`
accepts no `--game`: structured requests supply authoritative identity. Human
output never appears in protocol mode.

## Commands and semantics

`help` describes every command. `position startpos` creates a standard complete
history root. `position sfen full|reduced <four-field-SFEN>` starts a new known
segment with explicit material category and a **supplied** counter. The rules library owns
structural and live-root admission; necessary local conditions
do not prove historical reachability. Earlier checks and repetitions are unknown.
The counter is preserved, not expanded into invented events. Defaulted counter
provenance from an existing canonical record is retained on load.

`show` prints ranks a–i, files 9–1, case-qualified piece identities, both hands,
SFEN, current side, declared completed count, known event count and
conclusion. `hands`, `sfen` and `status` are shorter inspections. `history` prints
the immutable profile edition, original root/category/counter provenance, full
accepted actor/command/check/declaration/conclusion events, exact post-SFENs,
known board-action count and repetition indices.

`move <USI-action>...` parses existing USI board actions and submits the
entire sequence on private candidate state; failure commits none of it.
`undo [n]` uses reversal tokens and counts **events**, not board plies.
The loaded canonical ledger is replayed through game admission to recreate
those reversal tokens, with a complete-game equality check before replacement.
Undo can therefore reverse a loaded declaration/resignation as well as a move.

`eligibility sente|gote` only queries the evidence and creates no event.
`declare sente|gote` submits a formal entering-king declaration. Ineligibility,
including wrong turn, causes the approved false-declaration loss; it is not an
illegal board move. `resign sente|gote` preserves terminal semantics and
does not require own turn. Terminal commands may change history and conclusion
without changing SFEN, board turn or declared completed count. Further commands
are admitted/rejected by the concluded-game policy.

## Shared commands and meaning

Both selected shells expose `position`, `show`, `legal`, `move`, `undo`,
`history`, `status`, `record print/save/load-file/load-json`, `perft` and `divide`.
Position notation remains: chess `position fen <six fields>`, shogi
`position sfen full|reduced <four fields>`. Neither adapter implements rules.
Chess `status` is **position-only** quiet/check/checkmate/stalemate; its `history`
reports known move count, root origin, stored claim policy and repetitions. It
is not full match adjudication and has no hands or declaration commands. Shogi
`status`/`history` include accepted events and conclusions as above.
Perft is local legality in both games, capped at chess4 and shogi 2 (divide >=1).

## Compact board

Only `show` uses short tokens: uppercase `K R B G S N L P` for Sente,
lowercase `k r b g s n l p` for Gote, and a leading `+` for promotion (`+P`,
`+r`, `+B`). Two-character right-aligned cells reserve the promotion column
throughout the game; piece letters, empty dots and file labels remain aligned
at a three-character stride. Ranks run a–i, files 9–1. Hands and status
follow the diagram. This selected MSH-110 convention supersedes the
MSH-90 owner-colon display; its completed evidence remains historical.
See [common console presentation](../console-presentation.md) for the current
three-game policy and future multi-character abbreviation convention.

After `position startpos` and `move 7g7f 3c3d`, the actual `show` board is:

```text
a  l  n  s  g  k  g  s  n  l
b  .  r  .  .  .  .  .  b  .
c  p  p  p  p  p  p  .  p  p
d  .  .  .  .  .  .  p  .  .
e  .  .  .  .  .  .  .  .  .
f  .  .  P  .  .  .  .  .  .
g  P  P  .  P  P  P  P  P  P
h  .  B  .  .  .  .  .  R  .
i  L  N  S  G  K  G  S  N  L
   9  8  7  6  5  4  3  2  1
```

Inspection changes no game state, SFEN/FEN, history or record. The formatter
is separate from verbose action labels and USI/record encodings.

## Explicit readable notation

This is custom source-explicit long notation, **not** a claim to KIF, KI2 or a
standard western notation. Input remains exact existing USI: `7g7f`, `5c5b+`,
`P*5e`. Every `legal` row and accepted move pairs USI with a readable label:

| Meaning | Label |
| --- | --- |
| Sente silver 4e to 5d, retaining identity | `S:Silver[4e->5d;keep]` |
| Different silver 6e to same 5d | `S:Silver[6e->5d;keep]` |
| Optional/mandatory promotion selected | `S:Pawn[5c->5b;promote]` |
| Optional nonpromotion selected | `S:Pawn[5d->5c;keep]` |
| Already promoted pawn moving | `S:+Pawn[5e->5d;keep]` |
| Gote hand pawn dropped | `G:Pawn[hand->5e;drop]` |

`keep` means **no new promotion, retain the current identity**. It does not imply
demotion or that the mover is unpromoted. A mandatory nonpromotion alternative is
absent from the complete legal list and rejects on submission. Owner,
source, destination, drop identity and promotion decision are always explicit;
two same-kind movers cannot collapse to one label. Captures and hand demotion are
derived by the rules library; labels do not submit victim lists.

`legal` is explicitly labeled **local**. It lists complete position-level choices
in deterministic USI order, including all drops and promotion choices.
An already concluded game can still have local actions; game admission rejects
further play. `perft`/`divide` are also position-level diagnostics, not search or
game-tree outcome counting. They ignore repetition, length and declarations,
normalize counters privately as documented by the API, and never mutate
the live game. Perft 0 is 1 even at a terminal position; divide 0 rejects.

## Bounded records and failure behavior

`record print` prints a complete MSH-69 envelope with the game record as raw
payload. `record save <path>` writes the same bytes. `record load-file <path>` checks
the complete envelope/profile/edition/schema and routes raw bytes to the
strict parser, then restores the accepted game. This retains supplied/defaulted
counter provenance, root category, policies, complete event history,
declaration evidence and conclusions. No FEN/SFEN-only or move-only fallback is
used. Wrong-game, malformed, duplicate/unknown, unsupported or tampered
input leaves the current session and its undo history intact. `record load-json
<JSON>` takes an explicit inline envelope; `load-file` always takes a path. Neither command guesses from its argument.
Both chess and shogi use these same canonical commands and the same strict
RawValue boundary, so duplicate nested fields remain visible to rejection.

The four `record` commands are the only forms, and they require the typed
envelope. Chess restoration accepts only M2 move/history state; match observations
and Full mode reject clearly, rather than being silently discarded. No desktop
archive capability is supplied here.

Record files and input lines have no byte limit (MSH-268). Paths consume
the remainder of the command (no shell quoting syntax) and may contain spaces or
UTF-8. Short text commands/SFEN/USI retain their ASCII grammars.

Save is an atomic **new-file publication**: write and sync a uniquely created
temporary sibling, create the final hard link without replacing any existing
entry, then remove the temporary entry. Unsupported link/filesystem behavior or
an existing destination produces a clear save failure, preserving the session
and prior file. Choose a fresh path; this command intentionally has no overwrite
mode. This is a bounded console export, not the later desktop archive feature or
a cross-filesystem power-loss durability claim. The output must be a normal
filesystem file. Results go to stdout, escaped errors to stderr with a line
number. Exit0 means every command succeeded,1 means any failed,2 means invalid
program arguments. Command lines have no byte limit.

To save/reopen a complete capture and terminal history (choose a new path):

```bash
cargo run --locked --release --bin mshogi -- --game shogi shell <<'EOF'
position sfen reduced 8k/9/4+p4/9/4R4/9/9/9/K8 b - 17
move 5e5c+
resign gote
record save /tmp/mshogi-complete.json
position startpos
record load-file /tmp/mshogi-complete.json
history
undo
status
EOF
```

After capture, Sente has a promoted rook5c and one unpromoted pawn in hand.
Resignation concludes a Sente win at event 2 with only one known board action;
undo restores the live post-capture prefix. Saving again to the same path
explicitly fails; use a new path.

## Perft and divide

Console perft supports depth 0–2. Divide supports depth 1–2. Both use
the game rules and known history. The [completeness derivations](shogi-completeness-derivations.md)
give the independently expected starting totals. The [USI guide](usi.md)
describes machine protocol play.
