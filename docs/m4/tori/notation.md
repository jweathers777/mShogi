# Human Tori action notation

The selected `mshogi.tori.modern.v1` profile's T08 permits an explicit readable
human adapter; it does not prescribe a universal Tori interchange protocol.
The mShogi human form below is a documented display/input convention. Native
complete actions, exact context and canonical records remain authoritative.
There is no SFEN/USI compatibility claim.

| Element | Convention | Independently checked example |
| --- | --- | --- |
| Coordinates | Files1–7, ranks a–g, fixed Sente perspective | `7c`, `1g` |
| Current face | `Ph Fa Ea Cr Pt LQ RQ Sw Go` | `Fa7c` identifies Falcon on7c |
| Quiet board move | current face + origin + `-` + destination | `Ph4g-4f` |
| Capture | current face + origin + `x` + destination | `Fa7cx7b=Ea` |
| Changed compulsory result | `=Ea` or `=Go`, after the destination | `Sw3d-3c` keeps Swallow; `Sw3c-3b=Go` becomes Goose |
| Already promoted face | explicit current face; no new promotion suffix | `Ea7b-7a` |
| Drop | exact hand identity + `*` + destination | `RQ*5e` |

`LQ` and `RQ` are distinct identities, independent of side. A capture does not
rename either quail. `Ph` is never a hand resource. The source square disambiguates
pieces sharing a destination. Both native sides use the same coordinate system;
the separately shown actor is Sente or Gote.

`Fa7cx7b=Ea` is an independently hand-checked example: a Sente Falcon on7c
captures an opposing Swallow on7b and must become Eagle because it enters the
two-rank promotion zone. Omitting `=Ea`, substituting quiet `-`, or using the
other hand's quail is rejected when it does not identify a native local choice.
T08's older `Fa4c-4b+` illustration is not a required literal interchange token
and does not override the position's actual capture or compulsory result.

`mshogi_core::tori::notation::format_action` uses the authoritative source face,
native complete action and occupied destination. Formatting alone grants no
permission. `parse_action` matches exact labels against native `LocalPosition`
choices; it does not implement movement or promotion. The console then submits
fresh `GameRequest`s to `Game`, so conclusions, T06 obligations and current
context remain authoritative. Display strings never replace canonical replay.
No case folding, missing-source abbreviation or optional promotion inference is
performed. A parser success cannot revive an old execution identity.
`format_history_action` formats against an admitted exact event prefix without
replay or local-rule work, so historical Moves uses the actual source face.

The console is available with `cargo run --release --locked --bin mshogi --
--game tori shell`. Type `help`. The standard root is `position startpos`;
`position record-file <path>` and `position record-json <JSON>` admit supported
custom roots or complete games through strict canonical replay. There is no
second text-only diagram grammar. Board cells use `s`/`g` owner prefixes without
colons and fixed widths (`sLQ`, `gRQ`, `sEa`). Hands retain separate native counts.

`move` batches are all-or-none. `resign` and `stop` retain their native event
meanings; Stop is incomplete/unscored, not a game result. `undo` reverses complete
events. `history` reports board actions in readable notation and non-board events
separately with exact event/board-ply indices. `perft` and `divide` are explicitly
bounded local diagnostics and do not replace history-aware game permission.
`analyze` is read-only history-aware native search; it never applies its action.

Canonical Save uses the existing bounded atomic new-file infrastructure and
refuses overwrite. Malformed, wrong-game/profile, oversized or replay-invalid
Load leaves the current session intact. Successful Load restores event undo.
Human shell output and errors remain separate from structured `internal-search`.

## Deterministic native consumer smoke

Run `cargo test --release --locked -p mshogi-console --test tori_selfplay --
--nocapture`. This invokes the real `mshogi internal-search` process for each
turn, with explicit depth1/nodes256, exact canonical history, fresh request/
session/process identities and an independent20-second process watchdog. The
two fixed scenarios each run twice and compare actions, complete observations
and canonical records:

- Standard-root self-play, bounded to eight board actions. It then records an
  explicit unscored WorkLimit Stop; this is a functional smoke, not a completed
  standard game or playing-strength result.
- A hand-reviewed predecessor of TORI-O-MATE. `Ea4c-4b` reaches the independently
  expected native checkmate; the next process reports that actual conclusion.

The test prints structured `TORI-SELFPLAY-STANDARD` and
`TORI-SELFPLAY-COMPLETE` records with limits, complete canonical games and actual
observations. Native replay validates the records. Deterministic reruns apply
to this engine/search version and fixed work limits; the watchdog is a failure
bound, not a time-control policy or promise about timed-run reproducibility.
