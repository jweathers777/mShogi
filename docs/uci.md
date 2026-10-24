# First UCI chess adapter

Run `cargo run -p mshogi-console --release --locked -- uci` (or the built `mshogi uci`). This mode starts silently and waits for protocol input. Human commands remain in `mshogi shell`. The compatibility reference is the original **April 2006 UCI specification**, `engine-interface.txt` in the [Shredder UCI download](https://www.shredderchess.com/download/div/uci.zip), inspected for MSH-22. The implementation is an explicitly bounded subset; external-opponent match evidence belongs to MSH-23.

Supported commands:

| Input | Behavior |
| --- | --- |
| `uci` | `id name`, `id author`, the custom ClaimPolicy option, then `uciok` |
| `isready` | Immediate `readyok`, including while search is running |
| `ucinewgame` | Cancel/supersede old work and reset to the standard position under the selected policy |
| `position startpos [moves ...]` | Replay complete UCI actions from the standard start |
| `position fen <six fields> [moves ...]` | Import supplied counters and start known repetition history there; replay following actions |
| `go depth N`, `go nodes N` | Deterministic full-width search limits; depth 1–128, nodes any u64 including zero; combined limits stop at the first reached |
| `go movetime MS` | Cooperative elapsed-time budget, measured from command acceptance |
| `go wtime MS btime MS [winc MS] [binc MS] [movestogo N]` | Basic clock allocation for the side to move; see below |
| `go infinite` | Search privately and retain any completed result until `stop`; even a mate/terminal root does not emit bestmove early |
| `stop` | Request cooperative stop; return the last completed iteration or a labeled legal fallback |
| `quit` | Request shutdown and exit without joining an active search |
| `setoption name ClaimPolicy value explicit` / `auto` | Set the custom claim policy, cancel current work and reset the game; send a fresh `position` afterward |
| `debug on` / `off` | Accepted; no extra diagnostic stream is produced |

ASCII whitespace and CR, LF and CRLF endings are accepted. Leading unknown tokens are skipped until a recognized command; other unknown commands are ignored. The adapter advertises only its implemented ClaimPolicy option. Pondering/ponderhit, searchmoves restrictions, mate-search limits, Chess960, opening books, tablebases, hash/thread controls and other options are unsupported. Recognized unsupported `go` controls are rejected rather than silently searched without the requested restriction. Malformed/duplicate supported numeric controls are also rejected. Such rejected commands produce stderr diagnostics and do not submit work. There is no invented bestmove for an unaccepted `go`. Depth values outside the supported range are rejected explicitly. Unspecified depth defaults to 128 as an implementation ceiling; if an unbounded request reaches that ceiling, it retains the completed result rather than pretending to search deeper.

`position` parses and replays into a temporary rules-owned `AdjudicatedGame`. A failure, including an indexed illegal action or a move after an automatic conclusion, leaves the old game and search unchanged. A successful position replaces the entire known context atomically. Castling, en passant and promotions use the shared complete-action parser and legal rules transitions. The adapter never authors capture/effect lists. Search uses the native private coordinator and exact request identity/game record/version checks before publishing a result.

A conventional controller sends `stop`, waits for its bestmove, then replaces the position or begins another search. Out-of-order successful `position`, `ucinewgame`, policy changes or a new `go` supersede current work; old results are suppressed, including held infinite-search completions. They must never publish a bestmove for the new position. This recovery behavior intentionally does not manufacture a response for a superseded `go`. One accepted, normally sequenced `go` produces one `bestmove`; `stop` when idle is ignored. EOF shuts the process down as quit does.

## Work, time and claim policy

Default ClaimPolicy is **explicit**, preserving optional core claims. The custom option `ClaimPolicy` advertises `explicit` and `auto`. Under `auto`, the same selected policy reaches every future search node, including leaf turn boundaries. This matters: an opponent with an automatic runner policy cannot decline its valid claim merely because static evaluation favors continuing.

UCI has no standard draw-claim command or intended-move claim channel. If core search selects a root claim, the adapter diagnoses that limitation on stderr and returns its first lexicographically sorted legal move as a fallback, without claiming or concluding a draw. The unattended mShogi runner must exercise valid root auto-claims **before** sending `go`; it sets `ClaimPolicy auto` and sends position history so future-node policy agrees with its adjudication. An external controller using explicit claims remains responsible for exercising or declining available claims. The adapter's fallback is neither an assertion that a claim was invalid nor a playing-strength promise. `bestmove 0000` represents a rules-concluded game, including automatic draws, rather than disguising a claim as a null move.

A search stopped before its first completed iteration also returns the first legal move, with an explicit stderr diagnostic and `info depth 0` without a fabricated score. Rules representation/search failures terminate the engine with a nonzero exit and an explicit stderr diagnostic. They emit neither bestmove nor fabricated info/statistics for that request. Infinite mode also reports accepted search failures immediately, without waiting for stop; only successful completions can be held. A controller must record the failed process/search as an engine failure, not a chess result. For an already concluded root no fallback is played and bestmove is `0000`.

For clock allocation, choose the moving side's remaining time `R` and increment `I` (missing increment means zero). With positive `movestogo G`, or default `G=30`, spend at most `min(R/G + I, R - min(R/20, 50ms))`, using integer milliseconds and saturating arithmetic. If own-side time is missing, an apparent clock-only request is rejected; it cannot silently become infinite. With an explicit depth/node limit (or explicit infinite), missing own-side time imposes no additional clock limit. `movetime` overrides clock allocation. A zero time budget imposes zero search nodes. Depth/node/time combinations stop at their earliest selected limit; `infinite` plus a wall-clock budget is rejected as conflicting. A `go` with no effective limit behaves like infinite and awaits stop. Search can complete early at its selected work limit or terminal position; a timed request does not delay an already completed finite result to consume the entire allocation.

Clock stopping is cooperative. The event loop checks deadlines at each command and at a 2ms idle tick, then the worker acknowledges at its next node boundary/unwind. OS scheduling, one node's rules work, and pipe I/O can add latency. This is basic time management, not a hard real-time deadline guarantee. Actual match clocks/adjudication live in the rules match layer, not the protocol parser. Timed runs preserve their settings and observed timing; identical reruns are not promised. Deterministic depth/node runs preserve engine/evaluator identity, game context and work limits as described in [search.md](search.md).

## Channels and bounds

Protocol stdout contains only `id`, `option`, `uciok`, `readyok`, one final bounded `info` line per completed search, and `bestmove`. Every message is newline terminated and flushed. Scores use UCI centipawns or mate distances in moves; a mate-distance search score is converted from plies. The final info preserves actual completed depth, attempted nodes, elapsed milliseconds and the board-action portion of the variation. Diagnostic errors and fallback explanations go to stderr. No human shell prompts, command echoes or rules dumps enter stdout.

One input-reader thread feeds a `sync_channel(32)`; lines are limited to 4096 bytes, and oversized lines are drained without retaining their remainder. Non-ASCII/control-byte input is rejected and the next line remains usable. One search worker has one pending and one result slot, with no progress stream. The event loop processes commands while thinking and owns all protocol output. A normal final variation contains at most 128 actions, bounding each final info line. As with any pipe protocol, the controller must drain stdout/stderr; a blocked OS pipe can block its writer. Quit/EOF do not wait for the stdin reader or search thread to join.

## Verification

`cargo test -p mshogi-console --locked` includes eleven live UCI process tests: handshake/readiness during search, legal/mating bestmoves, depth and node limits, private stop/quit, history-aware repetition versus the same imported current FEN, atomic rejected positions, explicit/auto claim behavior at root and future nodes, movetime and both-side increment-clock inputs, infinite terminal holding, superseded-result suppression, stdout purity, malformed/oversized input recovery and unsupported-control diagnostics, and a descendant counter-overflow search terminating nonzero without a fallback or fabricated statistics. Two focused unit tests independently assert basic-clock allocations/numeric bounds and CR/LF/oversized streaming behavior. Existing 19 human console process tests remain applicable. Independent review and native platform CI are recorded in MSH-22; this adapter's tests alone do not establish external-engine interoperability or playing strength.
