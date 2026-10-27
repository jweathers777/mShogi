# Native Xiangqi position/action API

MSH-92 foundation for selected `mshogi.xiangqi.wxf2018.v1`, governing edition
`WXF-2018+XIANGQI-P1-2026-10-27`. Rust module `mshogi_core::xiangqi` is independent
of Chess/Shogi types, console I/O, Tauri, Python and comparator code. This
document distinguishes the state foundation from the later verified local
legal-action boundary; it is not complete-game delivery evidence.

## Native state and construction domains

`Intersection` is one of90 intersections: files a–i/ranks0–9 with Red at the
bottom. `Side::{Red,Black}` owns forward direction, palace membership and river
crossing. Palaces are d–f/ranks0–2 and7–9; the river separates4/5. These are native
rule geometry, not decorative chess metadata. The seven `PieceKind` identities
are General, Advisor, Elephant, Horse, Rook, Cannon and Soldier. `Piece` has only
owner/kind; no dummy promotion, hands, drops or declaration fields exist.

`Position::from_diagram` rejects duplicate intersections, missing/extra Generals
and excess original per-owner inventory. Sparse material and off-home placements
are useful **structural analysis diagrams** and are intentionally representable.
Exactly one General per side is required even for these diagrams. Construction
does not establish static live-root validity, legal preceding play or historical
reachability. The MSH-93 local layer must explicitly distinguish its admission
domain rather than manufacture a no-action outcome from an invalid diagram.

`Counters` contains explicitly known/unknown raw capture-free plies and fullmove
number. A supplied fullmove number is positive; values fitu32. These are declared
notation values, not evidence of omitted capture/check history, WXF Article8
eligibility or any repetition/outcome. Ordinary transitions increment raw
capture-free plies on every quiet action (including check and Soldier movement),
reset to0 on an observed capture and increment known fullmove number after
Black. Unknown values stay unknown unless an observed capture establishes0.
Overflow is an explicit representation failure and must not prune later board
legal-action queries into a false mate/stalemate.

## Scoped codecs and immutable roots

`Position::from_fen`/`to_fen` use a strict six-field dialect:

- Ranks9→0, files a→i; uppercase Red, lowercase Black.
- `k/a/b/n/r/c/p` mean General/Advisor/Elephant/Horse/Rook/Cannon/Soldier.
  The external letters do not rename native types; h/e aliases are rejected.
- Active `w` means **Red**, `b` Black; there is no native White side.
- Literal `- -` fields carry no castling/en-passant rules.
- Last fields are raw no-capture plies and positive fullmove number. Canonical
  single spaces/empty runs, bounded ASCII and strict unsigned syntax apply.

FEN is limited to256bytes (maximum valid board99bytes plus bounded fields is
well below that). Missing counters cannot be invented: `to_fen` returns
`UnknownCounters`; use native root JSON to preserve explicit unknown values.

`PositionRoot` retains an immutable root and `RootOrigin::{StandardInitial,
ImportedFen,AnalysisDiagram}`. Clone its position for private local work; moving
that clone does not mutate/relabel the root. Known history **starts at the root**;
no prior repetition/checks/actions are reconstructed. Standard provenance requires
the exact initial board/turn/counters; imported-FEN provenance requires supplied
counters; diagnostic roots may have explicit unknown counters. Root provenance
is construction metadata, not a claim that a structural diagram is playable.

`PositionRoot::to_json`/`from_json` scope native **root-position** documents under
`mshogi.xiangqi.position.v1`, with explicit profile, edition, origin, board, native
side, nullable counters and `history:"starts_at_this_position"`. This is not the
MSH-94 canonical full-game record, shared envelope, archive or saved observation.
Moves/events/claims are unknown fields and reject. Direct typed Serde parsing
preserves duplicates until rejection; no intermediate map erases malformed input.
All fields, including explicit-null counters, are required. Missing, duplicate,
unknown, conflicting identities, unsupported values and trailing JSON reject.
The4KiB byte bound covers valid documents (under512bytes) and bounds hostile input.

## Complete actions and exact local reversal

`Action { from,to }` is a complete ordinary board choice, encoded as four a0–i9
bytes. No partial route, victim list, suffix or terminal command is admitted.
`ActionEffects` is derived from native state: action, moving owned piece and
optional captured piece. General capture is not a playable continuation under
WXF2.8; captures never create a hand resource or change the mover's kind.

The MSH-92 foundation keeps `prepare_structural`/`commit_resolved` crate-private:
they check source/ownership/destination/royal capture/counter representation and
derive atomic state changes, but **do not establish movement legality**. There is
no public unchecked action preparation/commit API. MSH-93 supplies the complete
verified geometry/General-safety boundary before exposing playable actions.

`ResolvedAction` privately binds a before-state, after-state and native effects;
the commit guard rejects a different complete before-state. `Undo` stores exact
before/after values and only restores its complete matching post-state. Stale or
out-of-order rejection leaves the position untouched. This is local value-level
safety, not a game/session/process identity: equal values in another runtime
context are not distinguished. Future94/95/98 must retain full game history and
retired-execution protections; local undo cannot authorize reviving engine work.

No WXF third-warning/fourth adjudication, chase responsibility, natural-count
claim, impossible-to-win recognition, console, search, protocol, archive or desktop
consumer is implemented by this foundation. Those boundaries remain assigned
to their existing later tickets, especially94/105 and95–103.
