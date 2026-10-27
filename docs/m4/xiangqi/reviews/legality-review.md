# MSH-93 independent complete local board boundary review

Verdict: PASS for the bounded local legal-action contract. No remaining material review finding. Full Xiangqi match/history/adjudication remains94/105 and complete delivery45/M4 remains open.

Reviewer `/root/xiangqi_boundary_reviewer`, separate from native builder, independent fixture author and reference-harness author. Date2026-10-27. Prerequisites91/92 integrated at16042f2, closure22c10c1 before93implementation. Reviewed working diff rooted22c10c1: legal.rs/perft.rs, action/public API and module changes, unit/integration tests, separate developer probe, and local-legality.md. Reviewed source hashes are in `legality-reviewed-sha256.json`; final integrated commit assigned after review, not fabricated here.

## Review method and concrete boundaries

Line-by-line source review against selected profile, WXF2.1–2.10 page-image facts and frozen reviewed corpus. Inspected both actual capture geometry and post-action safety rather than treating comparator agreement as authority. Checked every public acceptance path and failure category, structural/static-live distinction, representation limits, counter normalization, exact undo ownership and bounded developer execution. No repository/task edits by reviewer.

- Live static admission requires both structurally present Generals in their palaces, Advisors on the actual five diagonal points, Elephants on seven own-side points and Soldiers on side-relative geography, with non-moving General not already geometrically attacked. Invalid diagrams return errors from pseudo/legal/status/perft rather than empty sets. This does not assert historical reachability.
- Complete native pseudo choices cover rook rays, cannon quiet/capture screen distinction, horse legs, elephant eyes/river, palace paths and soldier owner-relative movement. Own destinations and General captures are excluded. Every legal candidate is tested on the full resulting board for own-General geometric safety, so capture/blocker/screen insertion and removal are recomputed rather than reduced to chess ray-check rules.
- Attack queries are geometric and preserve pinned attackers. General-specific flying interaction is detected on actual royal target occupancy, not turned into an ordinary long move. Minor provisional empty-target documentation ambiguity was clarified explicitly; actual post-General-move safety already installed the new royal correctly.
- Reviewed independent pinned-attacker diagnostic: Red General e1/Rookd0, Black Generald9/Rookd2. Black rook d2 is pinned on its own General file but geometrically controls e2. Red e1e2 is pseudo-geometric yet unsafe; the native local contract does not recursively define attack through enemy legal choices. Reference agreement is separately supporting evidence.
- resolve_action validates static domain, structural ownership/capture checks with safe temporary counters, movement and resulting own-General safety, then prepares exact real counters. Public apply_resolved receives only a privately constructed native resolution and rejects stale complete before-state. No caller victim list/unchecked public mutation path.
- CounterOverflow never changes legal sets/status/perft, yet actual unrepresentable quiet transitions fail atomically. Captures reset to observed0, no WXF check subtraction or claim threshold leaks into counters.
- LocalStatus preserves checked versus unchecked no-action reason without committing a game conclusion. F002 histories preserve identical local sets and different native check observations, no repetition/claim/no-win result invented.
- perft/divide depth0..3 is bounded, checked-u64, outcome-free and uses a private normalized counter copy. Error propagation restores recursive state before returning; original position remains unchanged. Developer probe is a separately built Cargo example, bounded request/replay/depth/response with strict direct typed parse and native admission. No console/protocol/search/archive/desktop capability was added.

## Independently executed verification

macOS27.0.1 arm64, same working source captured in hash manifest:

1. `cargo test -p mshogi-core --lib xiangqi::tests`:16passed,0failed. Includes8100coordinate-pair public admissions, stale full-state resolution, explicit invalid-root errors, both-side max/unknown counters, raw24ply check cycle and immutable-root/provenance restoration.
2. `cargo test -p mshogi-core --test xiangqi_board`:5passed,0failed. Covers all105frozen cases (101live,4structural), every generated choice exact make/unmake, dedicated pin checks/no-action reasons, three deterministic64-ply all-choice/prefix restoration sequences and F002check/quiet prefixes.
3. Independently queried actual native and admitted reference P001 depth3 through compare.run_case: both7leaves, divide `{d0d1:5,d0e0:2}`, matching independently hand-derived expectation.
4. Independently queried24ply F002checking cycle: native fullFEN raw count24/fullmove13, reference adjustedcount20/fullmove13; exact board/side/fullmove and legal sets agree, no error after explicit foreign-clock exclusion. This reinforces native counter semantics rather than adopting the comparator's outcome logic.

## Broad differential artifact reviewed

`/private/tmp/msh91-reference/native-board-comparison.json` SHA256 `5fa7e14c9627730ee0a6c9af8a68b4d049b3513378f0ae8b486569bfaf5533b4`:

-105fixture records,4explicit structural exclusions,101admitted native/reference full-set comparisons at depth2.
-520sequence observations across8deterministic seeds,64plies each;0failures. Independently inspected all recorded request-prefix lengths, side/fullmove relationships and native undo_restored facts to exclude the earlier fixed mutable-prefix provenance issue.
-Native executable `target/debug/examples/xiangqi_probe` SHA256 `f9cc6f4277cb70d147cd55dfe0ad50daa61604feda4bdf72ea2fd321c0ab954c` independently matched on disk.
-Pinned reference9d7833aa113626b3ea63ad83f49fb9b456bd5839; strict board-only admission and limitations reviewed under91. Comparison tool SHA2562a1e5910250fd19ed7424b07eb78c15f69e9a3f72f50aa01052b534a616a692c, reference adapter843a804d64aa07df9756c107bb6a569a114be5ec4830b294747fc43843f4dece.

Initial44 and bare-General2/3/7 are independent expected facts. Larger opening perft/divide and random sequence totals remain differential observations, not promoted to authority. Both original B38 mismatch and source-reasoned correction remain preserved rather than hidden.

No altered Chess/Shogi rules/shared consumers or instrumentation in normal product packages were found. Normal workspace/native CI evidence is the integrator's separate obligation; this review does not infer other operating-system execution from macOS checks.

## Exact final-source provenance renewal

The integrator identified that the earlier f9cc…example binary preceded the final is_attacked documentation clarification in legal.rs. No logic changed, but that binary is not claimed as an exact build of final source. The example was rebuilt and frozen at `/private/tmp/msh93-native/xiangqi_probe`, SHA256 `aced652a098c28abf9d63a1842414a301a9bb45e5aab0f761965c36ab7acd7af`; full broad comparison, depth3 and raw refusal/counter evidence were rerun against that immutable executable. Earlier evidence and source-correction provenance are transparently retained as historical.

Independently verified final binary hash and all9source/build-input hashes in final provenance; source unchanged after execution. Final full `/private/tmp/msh91-reference/native-board-comparison-frozen.json` SHA256 `1802c7daa30b426d88f9b3537a1b74c878f89a6e1ac5a44f128f98217aef4185`:105fixtures,4excluded,520exact-prefix sequence observations,0failures, all check/undo fields and prefix side/fullmove relationships verified again. Final native-depth3.json retains Red/Black opening79666 observations and independent P0017/divide5+2, all complete divide arrays agree. Final native-probe-admission.json records15actual raw native refusal cases PASS, with final-binary identity and separate24raw-versus20foreign counter sentinel. Independently executed P001 depth3 again through the exact frozen binary:7leaves/divide5+2 PASS.

Final exact-source binding is recorded in `final-exact-source-review.json` beside this report. This renewal supersedes only the previous executable/report identity for final-source claims; completed source/semantic review remains PASS. No new logic review or unrelated platform campaign was needed.
