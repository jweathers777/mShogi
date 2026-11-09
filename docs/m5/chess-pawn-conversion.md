# MSH-145 (Pawn Threat and Exchange Judgment)

The broad MSH-147 diagnosis established two evaluation limitations: the clear-path
passer bonus considered current attacks but not future king interception, and
several independently drawn material families retained large conversion scores.
Historical Goldfish positions also changed with deeper search, but neither the
old exchange nor the newer engine choice is established as uniquely losing.

This increment discounts only the extra advanced-passer runway premium when
turn-aware king distance indicates interception risk or an enemy rook/queen
presses from behind. Escort control and friendly rear-heavy support protect the
premium. This bounded geometric estimate ignores future checks, opposition and
obstacles; it is a heuristic, never a race proof or a second move generator.

Narrow advisory scaling now covers two knights versus a bare king, a protected
penultimate rook/bishop-file pawn against a queen with a remote attacking king,
and an intact third-rank rook defence. Original coefficients preserve a small
gradient in the latter two cases. Nearby winning, hanging-piece, extra-material,
central-pawn and king-proximity controls limit applicability. Existing exact KPK,
wrong-bishop and bishop/knight handling remains. Native search still owns mates,
draw claims, complete history, cancellation and outcomes; a two-knight position
with the unique Ng6 mate demonstrates why static drawishness cannot adjudicate.

Tested source `846329fb5e4af855c53da83c3edbd4bb1cf5e1d6` passes 101 release search
library tests (two pre-existing ignored), workspace all-target clippy and
formatting. Independent complete tablebase move sets match native legal actions.
All 16 oracle choices preserve WDL, including the unique pawn-race push and rook
win. All three published tactical first moves remain selected. Every diagnostic
root is restored and every PV replays natively. Twenty-three conditions across
21 roots compare one-second play, plus five-second historical controls; both
binaries together used about 57 seconds, with no opponent matches.

The drawn queen/pawn search score falls from +714 to +95, two knights from +827
to zero, Philidor-style rook defence from +203 to +149, and its opposite-turn
score from −166 to −29. The narrow recognizer does not cover every subsequent
drawing rook position. The baseline already preserved all oracle WDL choices:
these results demonstrate better calibration and retained decisions, not newly
solved wins or a playing-strength gain. One tactical sample completes one fewer
ply while retaining its published move; no general speed improvement is claimed.

The [canonical evidence](evidence/msh145/verification.json) preserves independent
sources, complete oracle alternatives, baseline/candidate binaries and source
identities, terms, PVs, telemetry and actual local checks. Level 1 applies; no
other operating system or GUI campaign is claimed. Independent source and final
evidence reconciliation approved the increment with no material findings. Continue MSH-149 (Classical
King Exposure), then reassess the coherent round with fresh independent opponents.
