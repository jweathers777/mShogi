# MSH-66 independent native-legality review

Date: 2026-10-26. Reviewer: fresh Codex AI review agent, separate from the
implementation author and integrator; this is independent AI review, not human
review. Worktree `next-task`, branch `codex/shogi-board`; baseline HEAD
`368c613` with the MSH-66 working changes reviewed before integration.

**Verdict: PASS for the reviewed native local-legality implementation. No
material code/API findings remain.** Complete MSH-66 acceptance additionally
requires the separately recorded independent fixture review, actual admitted
reference comparisons and integration evidence. This verdict does not establish
history adjudication, match outcomes, protocol/search/UI delivery or MSH-41
completion.

## Method and scope

Read repository instructions, `docs/design.md`, MSH-66's acceptance criteria,
the approved `mshogi.shogi.csa36.v1` profile and clause coverage, MSH-64 reference
admission and corpus, and MSH-65's independently reviewed structural-state
boundary. Inspected the full new `legal.rs`, `perft.rs`, `shogi_probe.rs` and
`shogi_legal.rs`, plus the `mod.rs`/`action.rs` diff and the existing geometric
resolution/position invariants on which the public legal API relies.

This was a fresh source-and-test review, with independently executed focused
tests and additional manually constructed API assertions. Expected rules were
checked against the accepted profile and independently specified diagrams;
passing author tests alone was not treated as rule authority. The reviewer did
not copy cshogi code, execute the integrator's corpus/playout comparison run, or claim
independent human review of primary Japanese texts. The separate reference
worker owns actual comparison observations and their durable reports.
The reviewer separately executed the narrow intentional-failure harness check
recorded below using the admitted reference and native probe.

## Findings and reasoning

- **Attacks, candidates and legal actions:** geometric attacks include friendly
  defended target squares, stop at the first intervening occupant, and retain
  pinned-piece influence. Candidate enumeration applies structural movement,
  occupancy, promotion, resources, nifu and dead-rank checks. Legal membership
  then tests the fully resolved position for the moving king's safety, which
  handles pins, checker capture, king-origin x-rays and king adjacency without
  using pseudo-legal replies as king-safe defenses. King capture remains absent.
- **Complete choices:** both promotion flags are considered for each reachable
  destination, with structural resolution selecting available/forced choices.
  Already promoted pieces cannot promote again. Every held base kind is
  considered once per empty drop destination; identical hand resources do not
  duplicate actions. Sorting provides deterministic complete USI choices and
  does not prune nonpromotions as a search generator might.
- **Pawn-drop mate:** the newly placed pawn must itself attack the opposing
  king. A checking dropped pawn is adjacent, so it cannot be interposed against;
  a response must move the king or capture the pawn. Therefore no responding
  drop can resolve that pawn check, and structurally generated moves filtered
  by complete king safety exhaust the legal defenses without recursively
  invoking pawn-drop-mate detection. Defender captures keep both available
  promotion choices and compulsory promotion. Pinned defenders cannot supply
  a false defense. Existing-pawn mate and other-piece drop mate remain allowed.
- **State and diagnostics:** checked application resolves on an independent
  candidate and commits only after all legality checks. Snapshot undo matches
  the exact post-state, restoring owners, promotion, hands, side, counter and
  root provenance. Repetition identity explicitly includes board, both owners'
  hands and turn while excluding counters/provenance; it counts no occurrences
  and assigns no outcome. Query normalization prevents a large declared
  counter from changing local legality. Actual commit still reports counter
  overflow atomically.
- **Perft/divide:** depth zero is one; empty nodes contribute zero at positive
  depth. Traversal uses the same complete public legal actions and checked
  apply/undo, restores before propagating recursive count errors, and explicitly
  reports transition, restoration and count failures. The caller's position is
  untouched. These are local exact-depth sequence diagnostics, with exponential
  cost and no history/length stopping, as documented.
- **Verification probe:** the example is a development-only stdin/stdout bridge,
  with strict request fields, 64 KiB input, 512-move and depth-two limits. It
  retains prefixes and checks exact restoration after reversing the path. It
  does not add an engine/console protocol or bundle a reference implementation
  into the product.

No product code, fixture expectations, task records or commits were changed by
this reviewer. Only this report was added.

## Independently executed checks

Environment: macOS arm64 (`Darwin arm64`), Rust
`1.89.0 (29483883e 2025-08-04)`, repository-configured stable toolchain.

- `cargo test -p mshogi-core --test shogi_legal -- --nocapture`:
  **4 passed, 0 failed** at the initial review boundary, exercising the 101-case
  admitted board corpus, attacks/candidates/legal separation, counter-domain
  behavior and three deterministic 96-ply exact-restoration paths.
- `cargo test -p mshogi-core --lib shogi::tests -- --nocapture`:
  **11 passed, 0 failed**, covering the unchanged structural-state dependency.
- `git diff --check -- crates/mshogi-core/src/shogi
  crates/mshogi-core/tests/shogi_legal.rs
  crates/mshogi-core/examples/shogi_probe.rs`: **passed**.
- Independently built the core and compiled/executed a temporary external Rust
  API assertion program using `rustc --edition=2021`, the built core rlib and
  `target/debug/deps`: **passed**. Its manually constructed sparse root
  `9/9/9/9/9/3ppn3/3lkl3/3b1p3/4G3K b P 1` admits `P*5h`, after which the exact
  only legal defense is `4f5h+`. The pawn is protected by the gold on 5i, the
  defending king's neighbors are occupied, and the knight capture must promote
  on rank h. Nested apply/undo restored both exact prefixes. Further assertions
  checked atomic pawn-drop-mate rejection, a remote pawn drop when a structural
  imported diagram was already checked by a rook, friendly defended targets
  versus friendly landings, and identity differences for turn, promotion and
  hand ownership. The first draft of this temporary diagram had duplicate
  unpromoted pawns and was correctly rejected as `Nifu`; replacing the redundant
  blockers with distinct legal kinds produced the independently reasoned case
  above. This was a reviewer-diagram correction, not a product defect.

The temporary assertion source was `/private/tmp/msh66-review-edge.rs`, SHA-256
`d42300c820e29ef68e74380038d68dccb46150b4b07ac3bf6544ddf5c462e6be`.
It is supplemental local evidence, not a retained independent fixture corpus.
No Windows/Linux, GUI, packaged application or full-workspace tests were run by
this reviewer. Subsequent broader CI and reference evidence belong to the
integrator's validation record.

## Follow-up review boundary

After the author retained the reviewer's forced-promotion defense as a native
regression, reviewed the extended `shogi_legal.rs` and independently reran
`cargo test -p mshogi-core --test shogi_legal -- --nocapture`: **7 passed,
0 failed**. The added groups connect the separately reviewed completeness
corpus, retain the forced-knight-promotion defense and pre-existing-check
distinction, and preserve native behavior for the former SH-F022 v1 diagram
where both kings were checked. Manually confirmed the latter diagram's exact
three Gote replies after `9g8g`: `1a1b`, `1a2a`, `5c4d`. The nonmoving king was
initially checked in that structural diagram; reference limitations must not
replace the native geometric rule. MSH-64's independently reviewed v2 fixture
and reference-domain guard are recorded separately in `reference-domain-review.md`.

Reviewed the new development-only `tools/shogi/compare_perft.py`. One concrete
harness finding was reported and resolved: deduplicating equal roots before
checking every case's independent expectations could silently skip a later
start-position `900`/divide expectation when the earlier case had none. The
author removed that deduplication. Independently verified the fix with a
two-case temporary corpus using identical start roots, the second deliberately
claiming depth-two `901`: the admitted reference/native probe comparison
reported **two positions, one expected failure**, with the second case's
`independent_perft: 901` versus observed `900`, and exited 1. Thus the injected
bad expectation is detected after an earlier identical root. This intentional
failure is a passing harness sentinel, not a native product failure.

The sentinel command was:

```sh
/private/tmp/msh64-reference-venv/bin/python tools/shogi/compare_perft.py \
  --probe target/release/examples/shogi_probe \
  --fixtures /private/tmp/msh66-review-perft-cases.json \
  --report /private/tmp/msh66-review-perft-report.json
```

It is a bounded local diagnostic; the integrator retains actual full corpus,
perft/divide, deterministic path and mismatch-domain evidence. The product
`legal.rs` and `perft.rs` bytes were unchanged at this follow-up boundary.

## Reviewed source identity

SHA-256 at the initial four-test-group review boundary; paths are relative to
`crates/mshogi-core/`. Later fixture/test additions require a recorded follow-up
boundary rather than silently attributing their bytes to this review.

| File | SHA-256 |
| --- | --- |
| `src/shogi/legal.rs` | `a79edeef800bfc6dc316b6363cc408011189fd9b9018f05987b413ba3bea96cd` |
| `src/shogi/perft.rs` | `bc98a1ae0384ac3f1b53cc0bf0b5af49f3ef46f39f20cb42a0a6152da832f0fd` |
| `src/shogi/mod.rs` | `5d307c27662518e4b22b71b441ef449baac5cbd4771d35d8c41e38cae0de7ef8` |
| `src/shogi/action.rs` | `569fee1952af078f55aa192b864c4dc588ff1faf2a15af166bd637b7a2519235` |
| `examples/shogi_probe.rs` | `0af107c745054e2f891623d395afb441ac7a18e23c364fb1b0b377ef55dbed5d` |
| `tests/shogi_legal.rs` | `342fcbc1c42a7a2a381291c5e52ff16d4af84106171261d100671ca25d75eee2` |

Follow-up reviewed bytes:

| File | SHA-256 |
| --- | --- |
| `tests/shogi_legal.rs` | `976b0ad10513bf08f880c9a077caf8ec7ca819824337833efefba754aaf7b7c9` |
| repository `tools/shogi/compare_perft.py` | `70ec2a2b34fd22c4be643daf14bb17eae6f84f94a6fe75c17410faeed4108544` |

## Final integrated evidence boundary

Read `legality-validation.md`/`.json`, the final completeness/domain review
reports and all three retained native comparison reports after source
`0845392f9c9e5772ec19f281eee32f189ad84fa4` was integrated at
`6f6301e5a6b90a1dca986e88c6ce6e022f07e5d7`. Independently checked every one of
the validation manifest's ten source fingerprints against both working files
and `git show` of that integration. Product legality, perft and probe bytes,
and the seven-group test file, match the reviewed hashes above. The final
perft tool hash is
`8bfecc527dbe8e2615d9732d626452607500f7befffdb121d037fa53aac00fa8`;
its additional signed-counter traversal-headroom preflight was independently
reviewed in `reference-domain-review.md`.

Independently summed the actual retained observations: **167 sequences,
3,850 executed plies, 4,017 compared prefixes, zero mismatches**; all **169
depth-two perft/divide cases** have no errors. Both separately identified
Sente/Gote initial expectations record 900 total with every root divide 30.
Verified each report's fixture, admitted-manifest, harness/tool and probe hashes
against actual bytes. These report inspections validate the supplied evidence;
they do not claim this reviewer reran the integrator's full comparisons.

This supports AC1/2's complete local legality and atomic API, AC3's admitted
complete-set/transition and perft comparisons, and AC4's both-side edge fixtures
and exact restoration within the bounded task. Separate independent reviews
approve the 101 corrected board cases and 68 completeness cases. The reported
local workspace result is 260 passed, zero failed, two existing ignored deep
chess tests, with formatting and warnings-denied Clippy passing; this reviewer
did not repeat those unchanged broad checks. Remote CI run `40305615224` was
still running at this report boundary, so no remote CI pass is asserted here.
The independent native-legality verdict remains **PASS**, with no material
findings remaining and the history/product-delivery exclusions unchanged.
