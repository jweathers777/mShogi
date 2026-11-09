# MSH-148 (Deferred Frontier Work)

Quiescence now projects a settled quiet candidate only when its already-ordered
move is reached. An earlier capture cutoff can avoid the projection entirely.
All ordering scores and capture exchange estimates are computed at node entry,
so the admitted subsequence retains its previous order. Unsettled evasions and
threat defenses, native admission, draw claims, cancellation and errors are
unchanged. This removes unused work without adding pruning.

At corrected source `0a941948a405c75a2473e737bc3a260032753db0`, 36 paired
fixed-work runs over 12 positions preserve exact completed decisions, scores,
PVs, visits and every existing counter except the two documented counters of
reached quiet projections/threat admissions. Across those three-repeat pairs,
1,572,132 quiet projections are avoided. Deterministic tests exercise a capture
cutoff with zero quiet projections, a wide window that reaches quiet work,
cancellation at projection boundaries and typed native counter overflow.
All 98 search-library tests pass, with two pre-existing ignored tests; formatting
and release search all-target clippy also pass. Independent source and final
measurement reconciliation approved the change.

**No robust overall speed gain is established.** The sum of per-position median
search times changes from 4,439.076 to 4,423.175 milliseconds, a 0.358% difference
within ordinary timing noise. Sorting additional unadmitted quiet candidates may
partly offset the saved projections; that explanation was not separately profiled.
All 16 paired timed conditions retain the same chosen move, and six supplementary
independent endgame choices preserve WDL. One open-files sample completes an extra
ply at one second; this is not playing-strength evidence. Total comparison work
was 96.26 seconds, with no opponent matches or full-platform campaign.

The [canonical evidence](evidence/msh148/verification.json) retains exact sources,
binary hashes, measurements, commands, local fixture correction and limitations.
Level 1 local acceptance is appropriate: this changes portable search scheduling,
not platform-sensitive interfaces. Existing platform evidence is not relabelled.

The next priority is MSH-150 (Lazy Native Move Queries): the native existence
query currently builds all candidates before checking its first legal witness.
The preceding CPU samples identify substantial native query and allocation cost.
A shared short-circuit traversal can address that cost inside native authority.
Then continue MSH-145 (Pawn Threat and Exchange Judgment) and MSH-149 (Classical
King Exposure), with fresh adaptive assessment after the coherent round rather
than a tournament for this neutral timing result.
