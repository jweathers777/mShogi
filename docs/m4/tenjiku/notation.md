# Native Tenjiku human action notation

MSH-44 provides a compact **mShogi adapter**, not a claim of historical standard
notation. It serves both immutable whole profiles. Native complete actions,
ordered effects and canonical game history remain authoritative. Parsing selects
an actual local choice; `Game.submit` separately enforces full history, actor,
profile, terminal state and fresh context.

Travel grammar: `original[+]source(separator landing){1,3}[^]`, without spaces.
Stationary igui grammar: `original[+]source!xtarget`.

- `original` is the original native `Base.code()`. `+` means that original has
  already promoted. `ChS+` moves as Heavenly Tetrarch (`HT`); it is not an
  original HT. `WBf+` moves as Fire Demon and remains a promoted Water Buffalo.
- Coordinates are file 1–16 followed by rank a–p. Both owners use the same
  absolute coordinates; Sente/Gote comes from native position and event history.
- `-` marks an actual quiet landing and `x` an actual direct capture landing.
  All landings occur in order; crossed or jumped-over squares are absent.
  A return to the vacated origin is quiet. Different routes remain different
  complete choices even when their endpoints match.
- Final `^` requests this action's once-only promotion. Its absence selects no
  flip and does not assert promotion availability. Native effects determine
  whether the mover survives to promote.
- `!x` marks stationary igui with its exact target. The mover stays at the source;
  the target is not a travel landing. Igui has no route or promotion suffix.

| Native choice at an appropriate root | Label |
| --- | --- |
| Lion jumps and captures at 8f | `Ln8hx8f` |
| Lion captures at 8g then 8f | `Ln8hx8gx8f` |
| Lion captures at 8g and returns | `Ln8hx8g-8h` |
| Lion's quiet return through 7g | `Ln8h-7g-8h` |
| Vice General's three actual landings | `VGn4h-4g-5g-5f` |
| Promoted Chariot Soldier's stationary capture | `ChS+8h!x9h` |
| Pawn enters the promotion zone and requests a flip | `P12f-12e^` |
| Corresponding unflipped choice | `P12f-12e` |

Capture labels describe direct route occupancy. Fire collateral, mover death and
surviving promotion are **derived native phases**, separate from the player's
input. A Demon action keeps the same label when the two profiles produce different
burn/death effects. Console history and the native desktop completed-action
projection retain those exact phases; no client-supplied victim list is accepted.
Historical labels use each event's exact preceding native prefix.

Input is exact and case-sensitive. Wrong capture markers, wrong original identity,
incidental whitespace, unavailable promotion, malformed igui and incomplete or
unavailable routes cannot match a local choice. A geometrically valid label can
still be rejected by full Game history, including the forbidden first recurrence.

## Console

Select a whole profile explicitly:

```text
mshogi --game tenjiku --profile adams shell
mshogi --game tenjiku --profile muller shell
```

The exact canonical profile IDs are also accepted by `--profile`. Shell and
profile modes require explicit selection; there is no default Tenjiku profile.
`position startpos` keeps the current profile. Loading a strict canonical record
installs that record's profile without reinterpretation; a later fresh start
keeps the loaded profile. There is no rules switch within a game.

`help`, `show`, `status`, `legal`, `move`, `undo` and `history` expose complete
native choices, original/promoted identity, royal sets, root provenance, known
history, bare-reply state, actual conclusions and incomplete work annotations.
`move` batches commit only when every complete action is admitted. `undo` reverses
native events, including non-move events. `offer-draw`, `accept-draw`,
`decline-draw` and `resign` use an explicit Sente/Gote actor. `stop` records
unscored administrative interruption, without inventing a conclusion.

`position record-file`/`record-json` accept strict canonical standard/custom roots.
`record print`, `record save`, `record load-file` and `record load-json` share the
existing bounded file adapter. Save requires a new path and preserves an existing
file; strict complete replay succeeds before Load replaces the current session.
Records preserve profile/edition, original identities/rights, complete actions,
native effects and match history. Human labels never replace canonical semantics.

`perft` and `divide` use the **full native Game**, including history and endings:
depth 0–3 for perft, 1–3 for divide, at most 100,000 visited nodes across the whole
request. An incomplete traversal returns an explicit error rather than a partial
exact count. `analyze [depth nodes]` uses bounded native history-aware search on a
private copy; it leaves the live record unchanged. Zero work has no fallback move
or score. Current native search limits apply, independently of diagnostic bounds.

Board cells use lowercase `s`/`g` ownership prefixes and original native codes,
with `+` for already promoted originals. There are no captured-piece hands.
Human shell output is separate from structured `internal-search`; Tenjiku is not
advertised through Chess UCI, standard Shogi USI or SFEN.
