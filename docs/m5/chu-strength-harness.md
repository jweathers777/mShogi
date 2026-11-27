# Chu strength inputs

Use the [shared strength harness](strength-harness.md) with `--game chu`.
The frozen opening catalog `tools/strength/data/chu-openings-v1.json`
contains full native histories. The balance screen used depth 3, a 500,000-node
cap and an absolute score bound of 100 Pawn hundredths.

The frozen suites `chu-tactics-v1.json` and `chu-forcing-v1.json` in the same
directory contain native positions, complete independently proved answer sets
and explicit partitions. Use `probe --split development` or
`probe --split held-out` with either suite. The first is an elementary Lion
and royal capture screen. Its held-out positions share construction templates
with the development positions. The second asks for a win within three plies.
A slower forced win can fail its criterion.

The shared export preserves side to move and Lion protection. The Chu Texel
model filters threats to all surviving royals, positive static-exchange
captures and promotions, and unresolved exchanges. Freeze the fitting binary
with the corpus. Use the [strength roadmap](strength-roadmap.md) for judging
and time-odds comparisons.
