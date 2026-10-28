# Human console board presentation

Owner-selected convention (MSH-110,2026-10-28): the three implemented games
use the compact Chess-style case distinction for board ownership. These are
human `show` diagrams, independent of action labels, protocol tokens and
canonical records. Native game adapters still own piece/side identities,
coordinates, rules and complete actions.

| Game | Uppercase | Lowercase | Cell format |
| --- | --- | --- | --- |
| Chess | White | Black | One piece letter or dot, separated by spaces; unchanged. |
| Standard Shogi | Sente | Gote | Two-character right-aligned token, separated by spaces; leading `+` marks promotion. |
| Xiangqi | Red | Black | One piece letter or dot, separated by spaces; native river retained. |

Shogi examples: `P`, `p`, `+P`, `+p`. Every Shogi diagram reserves the promotion
column even if no piece is currently promoted. Piece letters, empty dots and
file labels therefore stay aligned when a promotion occurs. Shogi ranks a–i,
files9–1 and hands remain native. Xiangqi retains ranks9–0, files a–i,
intersections and the river; `B/b` means Elephant and `N/n` means Horse.
Chess remains byte-compatible with its existing board diagram.

For future variants whose native abbreviations require multiple characters,
such as `Lg` or `FK`, use the owner-preferred compact prefix convention:
a lowercase native owner marker immediately followed by the abbreviation,
**without a colon** (`sLg`, `gLg`, `sFK`, `gFK` where Sente/Gote apply).
Preserve the abbreviation's capitalization. Choose a stable cell width that
fits that game's longest token, including any native promotion notation;
align coordinates and empty dots consistently. Side markers, promotion and
piece abbreviations must be selected from the real game's semantics, not
assumed universal. This records presentation direction; no future variant or
universal renderer is implemented by this follow-up.

This replaces the human Shogi owner-colon format from MSH-90 and Xiangqi
owner-prefix format from MSH-96. Their task records, source revisions and
verification evidence remain preserved history. Verbose source-explicit Shogi
action labels still contain their existing separators; SFEN/FEN, USI/UCI,
structured engine requests, native records and desktop rendering are unchanged.
