# M4 implementation map: chess and standard shogi first

Planning proposal for MSH-62, 2026-10-26. No game implementation is part of this
batch. Baseline is closed-M3 integration `6f29713795804eed3605114d90fed37aa5f554a0`.
The reused builder has identical product sources; it still excludes the preserved
prototype in its root workspace manifest, while integration has removed that
obsolete exclusion. That unrelated cleanup remains intact.

## Actual boundaries and the smallest proposed extensions

Paths below are repository-relative. Symbols are more stable than line numbers;
the baseline revision fixes what was inspected. Existing chess types and public
semantics remain game-specific. Add a shogi module and explicit two-game dispatch
only where an actual consumer needs it. There is no rules DSL, plugin ABI,
mandatory common position storage or requirement to make every game use negamax.

| Current code boundary | What is actually chess-specific | Proposed first extension and verification obligation |
| --- | --- | --- |
| `crates/mshogi-core/src/chess/mod.rs`: `Color`, `Square`, `Position`; `action.rs`: `Action`, `Promotion`, `Undo` | White/Black direction; 8×8 squares; four chess promotion kinds; action always has board `from`/`to`. | Add authoritative `shogi` state, Sente/Gote, 9×9 coordinates, base/promoted piece identity and hands. Its complete action is `Move {from,to,promote}` or `Drop {hand_kind,to}`. Preserve captures/demotion/hand effects and exact reversal in Rust. Keep chess `Action` unchanged. A declaration or resignation is a match command, not a fabricated drop/pass. |
| `chess/game.rs`, `adjudication.rs`, `matchplay.rs`, `record.rs` | `GameRecord`, `AdjudicatedGame`, `ChessMatch`, optional chess draw claims, chess modes/clock ordering and FEN root. | Add shogi-owned position history, exact repetition identity, check-event history and match commands/results under the selected shogi profile. Separate local legal board choices from history-dependent loss/draw and declaration eligibility. Do not reuse chess dead-position or resignation proofs as shogi rules. |
| `crates/mshogi-search/src/search.rs`: private `Tree`, `ChessTree`, `SearchChoice`, `SearchDecision`, `negamax` | Public entry accepts `AdjudicatedGame`; claim value zero; terminal `ChessOutcome`; UCI text tie order; material evaluator; `expect` assumes every nonterminal chess node has a move. | First supply a transparent shogi evaluator including hands/promoted pieces and a full-choice shogi search using its own adjudication. Reuse the small traversal only after removing concrete chess errors/claim assumptions at a real second consumer; two clear adapters are preferable to a public universal trait. Handle no-move loss and legal declaration choices before leaves. Never treat a reference engine's search-pruned moves as all legal choices. M5 owns measured strength. |
| `crates/mshogi-search/src/coordination.rs`: `RequestContext`, `SearchCompletion`, `SearchCoordinator` | Exact base is a chess `GameRecord`; completion is a chess `SearchResult`. | Retain the concurrency/identity pattern. A tagged game context and game-specific completion adapters may share cancellation/storage, without converting shogi into a chess record. Preserve exact history, parameters and purpose in equality; board hash equality is insufficient. |
| `crates/mshogi-engine/src/lib.rs`: internal protocol v2 `EngineRequest`, `EngineChoice`, `EngineResult`, `record_json`; `server.rs`, `analysis.rs` | Canonical chess record, `Action`, `Color`, `ChessOutcome`, chess claims; Full cooperative reachability is a chess operation. Strict raw-record parsing preserves duplicate-field rejection. | Introduce a versioned, tagged chess/shogi envelope with rules-profile ID/version, canonical-record version and typed payload. Preserve exact purpose/request/session/process generation and limits in every response. Board action, declaration/claim, concluded game, incomplete work and failure remain separate results. Route raw payload to its authoritative strict parser; do not first erase duplicate fields in an untyped JSON map. Keep explicit v2 chess decoding/behavior coverage; reject an unsupported version/profile instead of guessing. |
| `crates/mshogi-engine/src/bridge.rs`: `EngineBridge` | Scheduling/pipe bounds are reusable, request/result types are concrete; Windows launch deliberately avoids a console. | Keep one execution owner, bounded queue/results/stderr, off-path I/O, watchdog, cancellation and cleanup. Generalize its envelope, not process semantics. Test game switching and identical-looking positions across different profiles, alongside existing purpose/history/generation races. No engine failure awards a game result by itself. |
| `crates/mshogi-desktop-session/src/lib.rs`: `DesktopSession`, `DesktopController`, `new_match`, `accept`, `snapshot` | Owns `ChessMatch`; new games are chess; commands, claims, side and snapshots use chess types. `Snapshot` has `fen`, `rules_mode: m2`, chess outcome/check fields. | A small native session enum dispatches Chess/Shogi to their authoritative matches. Serialize common command admission and result acceptance once. Native adapters produce profile-specific legal choices and readable state; the frontend never adjudicates. A profile switch is protected document replacement, with fresh execution identity and no stale repaint. Shogi live outcomes follow its profile, not `rules_mode=m2`. |
| `desktop/src/native.ts`, `Board.tsx`, `App.tsx`, `presentation.ts` | `Color` means White/Black; chess glyphs and default geometry; `LegalChoice.from` requires a board square; main app hardcodes chess identity. The shared route component already handles complete multi-stage choices. | Profile-qualified side IDs and native display metadata separate player identity, first mover, piece appearance and bottom orientation. Sente is SFEN `b` and moves first; chess White moves first. Add a hand-origin selection alternative alongside a board origin. Native complete choices supply allowed routes and promote/decline choices. Never encode a drop with a fake source square or infer legal targets in JS. |
| `Board.tsx`: `BoardGeometry`, `displayedSquares`, `BoardChoice`, glyph fallback | Rectangular square board can already scale, but key construction concatenates file/rank strings, colors drive glyph classes, and geometry cannot describe intersections. | For shogi, use native opaque square IDs with ordered row/column metadata, a 9×9 square layout, owner-oriented pieces, accessible names and hand controls. Preserve route completion/cancel semantics. Later real xiangqi requires a named intersection extension; chu/tenjiku require actual native route/effect/royal consumers. The M3 synthetic fixture stays synthetic. |
| `Analysis.tsx`, `analysisPresentation.ts`, engine `RequestPurpose`/`Telemetry` | Score scaling uses chess pawn=100 and mate constants; move formatter recognizes only `[a-h][1-8]`; Full means chess cooperative reachability. | Advertise native per-profile analysis capabilities, operation limits and score metadata/perspective. First shogi supports bounded ordinary move analysis. Preserve chess Full; report unsupported shogi Full explicitly and do not relabel it as tsume or forced-mate analysis. Real mate scores need a game/evaluator convention, not the chess formatter. All results remain bound to the exact historical selection and read-only acceptance path. |
| `persistence.rs`: `Archive`, `preflight`, `validate_observation`, `view_prefix`, `atomic_write`; `capacity.rs`: candidate admission/receipts | Archive v1 stores chess record JSON strings, `Color`, chess results/modes and fixed caps. Full/timed live restoration is deliberately rejected. | Add a tagged document envelope and game-specific canonical payload while keeping the complete v1 chess reader and its semantics. A common worker dispatches strict validation/replay and history-prefix reconstruction to the right native implementation. Keep observations historical after Open, fresh runtime identities, explicit Resume and stale snapshot/save guards. Apply admission to the whole candidate, not only move bytes. |
| `crates/mshogi-console/src/main.rs`, `session.rs`, `uci.rs`; `mshogi-runner` | Single-argument chess shell entry; UCI/FEN/action text; runner uses `AdjudicatedGame`, chess clocks and claims. | Add explicit game selection and a shogi shell/notation adapter, keeping old chess commands/UCI stable. A separate USI adapter and shogi runner use shogi authority and record profile, commands, policies, failures and observations. Share bounded subprocess mechanisms only where useful; do not force USI shogi outcomes through a UCI chess runner. |

## Concrete first wire/presentation contract

The extension ticket should finalize concrete Rust/serde names when active, but
must preserve these proposed semantic fields:

* `game_kind`, `rules_profile` and its immutable edition, native record schema,
  and complete canonical game/match payload. Identity is not just `shogi`.
* Explicit execution purpose and analyzed history selection, request ID, session
  version, process generation, evaluator/settings identity and actual work limits.
  The entire accepted context must match; cancellation or another game/profile
  retires it even if squares and pieces happen to match.
* Native side identifiers with display names, bottom-side orientation and piece
  ownership/appearance metadata. Never rename chess Black into Sente implicitly.
* Native board descriptors, hand inventory, complete choice IDs, an origin of
  board square **or hand piece**, ordered route steps and optional completion
  alternatives. Piece labels/turn/result/available match commands come from Rust.
* Profile-specific conclusion reasons and optional match scoring/replay metadata.
  A declaration is not a played action; an advisory eligibility query is not a
  committed false declaration. An observation is not a resumable pending request.

This is a concrete boundary proposal for two real consumers, not permission to
publish a universal action schema for every future game. The existing chess
library action/record encodings need no replacement. Later consumer extensions
must retain complete routes, multiple royals and derived effects without making
frontend rules or inventing chess-style self-check for chu/tenjiku.

## Compatibility and capacity are admission requirements

Existing desktop archive v1, chess game-record schemas 1/2 and match-record modes
retain their identities. Read old documents with the old strict parser, validate
them under the recorded chess policy, and migrate only to a documented lossless
tagged envelope. Keep original observation protocol/purpose/context as historical
data, including chess Full observations; never turn them into a new request.
Known unsupported timed/Full live reopening stays explicit. Failed, stale,
unsupported or resource-limited Open leaves the current session intact.

The writer format/version can change in its active shared-storage ticket; an old
document must not silently acquire a different profile, claim policy, outcome,
limits or history. Compatibility tests should reuse unchanged M3 records with
claims, resignation, command/event history, historical observations and advanced
analysis, plus exact old rejection cases. Compare canonical semantics, not only
FEN/move text. Open installs fresh execution IDs while preserving monotonic
frontend protections and stops play until explicit Resume.

MSH-61 is a preserved invariant: validate an immutable complete candidate before
committing a move, observation, settings change or control. Refusal/resource pause
keeps the previous accepted session fully saveable. Keep reserved identity and
stop/save/close headroom, stable receipts, atomic writes and unsaved-change guards.
No truncation, rolling loss of observations, unscored-result invention or promise
that Save/Open resets counts is acceptable.

Chess's depth2/20,000-node play defaults, analysis caps and archive limits
(16 MiB/4,096 events/256 observations/8,192 controls) are existing defaults, not
universal rules. The shogi consumer tickets must measure representative complete
records, hand/choice branching and observation sizes, select bounded supported
limits and record their identities. A changed limit policy must still bound Open,
replay, encoding and cleanup and preserve a complete save path at every accepted
boundary. Verify exact observation/count/byte boundaries for each supported
policy and cross-profile switching; raising constants alone is insufficient.

## External protocol boundary

Use USI for the actual shogi consumer. The
[Shogidokoro specification](https://shogidokoro2.stars.ne.jp/usi.html)
defines SFEN, move/drop/promotion text and handshake/stop/quit. Its entering-king
extension makes `bestmove win` a CSA declaration validated by the host; it is
not a portable JSA professional result. `gameover draw` does not define rating
points or a replay series. The
[original USI draft with extensions](https://hgm.nubati.net/usi.html) also
describes depth/node limits; verify the selected opponent's supported subset.
The profile and adapter must explicitly record those differences.

Use the pinned opponent/options in [the verification proposal](shogi-verification.md).
Keep full root+move history, host-side declaration validation, actual output and
failure identity. Protocol `resign`, `win`, missing reply and invalid move are
different cases. Preserve cancelled/stale results without applying them. Bounded
reproducible self-play/opponent smoke is functional M4 evidence; it is not M5
strength evidence. No new protocol is required for a game without a real consumer.

## MSH-72 concrete desktop boundary

The [shared desktop session contract](desktop-session.md) records the typed
chess/shogi adapter, native presentation/capabilities and preserved chess
compatibility. [Verification](desktop-session-validation.md) distinguishes
native/controller, real subprocess, frontend and platform CI evidence. This
bounded boundary does not close MSH-73/74/75 or MSH-41–47.
