# Actual normal macOS application — 19beb74

Source `19beb7437f11003734d65ee68f69a5f34bbdd02c`, macOS27 arm64, normal
uninstrumented bundle. External CUA used native accessibility, pointer/keyboard,
file pickers and Cmd+Q; no development server, injected game data or verification
entry point. [Package hashes/checks](../macos-package.json),
[fresh-launch process path](running-package.json),
[native quit observation](native-quit.json) and [file hashes](retained-files.json)
have separate scopes.

Both initial captures use the same 1180×820 **native window**, including macOS
title bar: 2360×1640 Retina pixels. These are native JPEG bytes; provisional
`.png` suffixes were corrected to `.jpg` without changing image contents. CI
reports separately measure the client viewport. No current Mac minimum-size
claim is made; both-game 760×650 client evidence comes from Windows/Linux.

| Observed state | Actual capture |
| --- | --- |
| Chess initial common shell | [Chess](chess-normal.jpg) |
| Shogi initial common shell | [Shogi](shogi-normal.jpg) |
| Chess friendly selection → opponent knight, read-only help in footer | [Inspection](chess-opponent-inspection.jpg) |
| Shogi opponent pawn / Escape | [Inspection](shogi-opponent-inspection.jpg) |
| Human7g7f and real engine4c4d response | [Play](shogi-play.jpg) |
| Exact historical event1 / common Return control | [History](shogi-history.jpg) |
| Real historical analysis: Gote, depth1,31nodes | [Analysis](shogi-historical-analysis.jpg) |
| Zero-node budget: incomplete, no completed evaluation | [Incomplete](shogi-incomplete-analysis.jpg) |
| Finite20,000-node budget: completed depth2, node limit reached | [Bounded result](shogi-node-limited-analysis.jpg) |
| Normal native archive Open / opponent pawn in hand | [Hand](shogi-opponent-hand.jpg) |
| Escape, Enter on opponent hand | [Keyboard inspection](shogi-opponent-hand-keyboard.jpg) |
| One-event prefix: Gote rook4e, occupied4g destination, no future Gote hand | [Historical inspection](shogi-historical-opponent-inspection.jpg) |
| Return to live and Flip / opponent hand | [Flipped inspection](shogi-opponent-hand-flipped.jpg) |
| Own complete Save/Open / stopped until Resume | [Reopened](shogi-reopened-stopped.jpg) |
| Saved-game native Quit confirmation | [Quit guard](native-saved-quit-guard.jpg) |

The pre-existing open session was preserved at
`/private/tmp/msh74-preserved-before-19beb74-20261027.mshogi` before quitting the
older process. The source capture archive was never overwritten. Separate QA
saves retain the [two-event/seven-observation play session](records/play-analysis.mshogi)
and [flipped inspection settings](records/inspection-flipped.mshogi). The first
was actually reopened by this normal package: two events, saved observations and
stopped execution were visible. The second derives from the existing native
reduced-material diagram; it makes no standard-opening reachability claim.

Rapid external submit/cancel attempts did **not** establish successful Mac
cancellation. Some observations preceded native work startup and others followed
completed node-limited searches. No screenshot is labeled as successful active
work or cancellation on that basis; accepted observations were preserved.
Current Windows/Linux normal GUI journeys establish actual cancellation and
replacement separately. Native Cmd+Q on the saved inspection session did reach
the common saved-game confirmation and exit; exact host/sidecar absence was
checked before relaunch. No test process was killed as quit evidence.

The final normal application is left running with a fresh untimed Sente game for
owner review. Independent actual screenshot review is recorded separately;
owner visual approval remains outstanding and MSH-74 remains In Review.
