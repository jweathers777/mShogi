# MSH-64 independent reference-domain correction review

Date: 2026-10-26. Reviewer: fresh Codex review agent, separate from the fixture,
reference harness and native implementation authors. This is an independent
agent review, not a human review. Worktree: `next-task`, branch
`codex/shogi-board`, review base `368c613348b07dd37f05415ded63bedb6ee26b29`.
The corrected files were working changes at review time. Only this supplement
and `completeness-fixture-review.md` were edited by this reviewer in this pass.

**Verdict: PASS for the corrected MSH-64 reference-root admission and current
101-case corpus. No material findings remain in this correction.** This
supplement explicitly supersedes the original `reference-review.md`'s broad
claim that all version-1 roots were admitted soundly. That earlier review and
its actual run remain historical evidence; their root-admission conclusion
was too strong. The reviewer missed two nonmoving-king attacks in that pass.
Their promotion-choice assertions were independently correct, but their roots
were unsuitable for this upstream reference's incremental-check assumptions.

## Late finding and independently approved correction

Longer MSH-66 traversal exposed the problem rather than merely a different
preferred move count. In `SH-F022:evasion-B-sente` version 1, B4d already
attacks the nonmoving K1a through 3c/2b while the opposing B5c checks the moving
K9g through 6d/7e/8f. The rotated version similarly has b6f attacking nonmoving
K9i. cshogi's cached/incremental check model is not admitted for such roots;
running deeper moves from them produced unsound reference observations and
eventually a king-capture/restoration failure. Agreement on selected first
choices did not establish that these whole roots were fit for random replay.

Before reference execution of the replacements, independently approved moving
only the nonmoving king from 1a to 1b, and 9i to 9h in the mirror. K1b is off
B4d's diagonals, while B5c still checks K9g along the same line. B4d can still
capture 5c with either promotion flag, removing the checker and preserving its
own king. Rotation gives the same proof for b6f–5g. The case IDs remain stable,
their versions increase to 2, the rationale identifies the previous versions,
and their expected membership/check assertions remain unchanged.

Compared all 101 records against the preserved version-1 JSON. The case IDs,
move histories and expected facts are identical. Exactly these two records
change `root_sfen` and `version`, plus their correction provenance. No expected
rule answer was rewritten to match engine output.

## Independent whole-corpus root audit

Ran a separate geometric audit using an independently written SFEN parser and
side-relative step/jump/ray rules with occupancy blockers. It did not import
cshogi, call the native product, use cached check status or infer expected
facts from either program. It included kings, golds, silvers, pawns, knights,
lances, bishops, rooks, all promoted minors, horse and dragon.

The historical 101 roots have exactly two nonmoving kings already attacked:
the two F022 bishop roots above. The remaining 99 historical roots pass this
condition. Both independently reviewed replacement roots pass. All 68 new
completeness roots also pass. Thus the correction addresses the entire
identified defect set rather than suppressing a disagreeing fixture by ID.
This audit is a local domain check, not proof of historical reachability.

## Audited admission mechanism

Read the complete revised `tools/shogi/reference.py` and the pinned upstream
implementation of the newly used public `cshogi.dlshogi.make_input_features`
wrapper. The wrapper calls the Cython `_dlshogi_make_input_features`, which
calls `src/cshogi.h`'s board method. That method computes occupied-board
`Position::attacksFrom` for every piece, merges attacks by owner/type and emits
raw geometric planes. Read the `attacksFrom` switch and associated position
declarations: all fourteen piece kinds, including king, are present. It is not
a search-pruned Legal list, parser-acceptance test or incremental check cache.

The plane indexing is correct at this exact pin: 14 placement planes, 14 attack
planes, three attack-count planes per owner, for 62 planes total. Piece-type
attack indices 14 through 27 inclusive therefore include every type. When
white moves, the upstream code reverses both owner and square orientation;
owner 0 remains the mover and `SQ99 = 80` makes `80 - king_square` the correct
target index. The guard allocates fresh C-contiguous float32 buffers at the
public API's dimensions, checks the expected 62-plane shape and reads only
the moving owner's raw attack planes at the opposing king. No model or
learned inference is involved.

Before calling the C++ constructor, local validation requires bounded ASCII
SFEN, canonical field spacing, nine well-formed ranks, one king per owner,
nonadjacent kings, no dead unpromoted pawn/lance/knight, no own-file duplicate
unpromoted pawn and standard physical material maxima. The material bounds
also protect the raw feature method's fixed hand-feature buffers. The supplied
next-ply and reserved continuation must fit cshogi's signed-int32 range.
These are explicitly named reference-domain restrictions; they do not change
the product's separately specified analysis-input/u32 contract or declare
unsupported diagrams to be native match outcomes.

`replay` invokes this admission before its first snapshot/LegalAll enumeration
or push; `seeded_path` uses it with the requested continuation budget. The
perft tool additionally preflights supplied prefix length **plus traversal
depth** before replay/enumeration. Legal replay from admitted roots preserves
the nonmoving-king safety invariant. Direct constant API-gap demonstrations
use independently checked safe roots; they remain diagnostics, not rule
oracles.

All eleven current source/license hashes matched the exact pinned checkout;
the installed public Python wrapper matched its newly retained source hash.
The loaded extension and successful build-log hashes are unchanged from the
actual isolated build. Extending the audited API surface does not fabricate a
new build or change the source pin. The manifest now explicitly names the raw
feature planes solely for root-domain admission.

## Actual verification

Independently reran:

```sh
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --diagnostics-demo --report /private/tmp/msh64-domain-review-current.json
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --fixtures tests/fixtures/shogi/legal-completeness.json \
  --report /private/tmp/msh66-completeness-review-current.json
```

Results: **101 current fixtures pass, zero fail, 13 domain-rejection sentinels,
four API-gap sentinels and four intentional mismatch/failure demonstrations**;
the separate completeness run passes all 68 fixtures, the same 13 domain
rejections and four API-gap sentinels. Both outputs are byte-identical to their
retained current reference reports. Native comparison sequences are zero in
these particular admission runs.

Also ran the revised harness directly against `board-rules-v1.json`. It exits
1 on the first unsupported bishop root with an explicit `reference-domain:
nonmoving king is already geometrically attacked` diagnostic, before
enumerating that root; no requested final report is created. Its mirror is
separately rejected by the automatic sentinels. The other eleven rejection
samples cover adjacent/missing kings, dead pawn/knight, nifu, excess material,
non-shogi piece syntax, counter overflow, reserved-prefix overflow, non-ASCII
and oversized input. Rejection is distinct from an empty legal move set.

During review, found that the initial perft comparison reserved replay length
but omitted its own traversal depth from signed-counter headroom. The root
author added the combined prefix-plus-depth preflight. Independently reran a
quiet root at next-ply 2147483647 through depth-2 comparison: it exits 1 with
the signed-int32 domain diagnostic, creates no report and never invokes the
sentinel native probe. Existing corpus counters were unaffected; this fixes
the generic tool-entry boundary. `git diff --check` passes.

## Preserved and current byte identities

| Artifact | SHA-256 |
| --- | --- |
| `board-rules-v1.json` | `579842bc0be0ab74567e1501100ef589be160c140c9371373768dc9fc43b6738` |
| `reference-run-v1.json` | `4b2bb754889cd12dee6ec32ef9ddd4c1d367ec515b7d717cbd2c9aaa4e1bb69c` |
| `cshogi-admission-v1.json` | `e38d6b6ae57e1a87256fb04638e97c691da932bbb670e5ed082beda79ed45427` |
| Current `board-rules.json` | `6a5356e77fe9508df27afcbb6ea66d2e3ea6397ed7f40994161b5583144dcbb1` |
| Current `reference-run.json` | `ce7b851ec9441063321be6be88ae940ce682fce4c8fae4120691416d1c153905` |
| Current `cshogi-admission.json` | `ce4769b55ab214d20540049647a91f7a066d32c5e6b235a6bb5677b75fe03e76` |
| `tools/shogi/reference.py` | `658f7a978c6678ca957115fa39a576ae056f299dcb414f89ca5a8c7f9f47aba1` |
| `tools/shogi/compare_perft.py` | `8bfecc527dbe8e2615d9732d626452607500f7befffdb121d037fa53aac00fa8` |

The first three preserved files match the hashes in the original independent
review. The current report binds the corrected fixture, expanded admission
manifest and revised harness hashes. Historical passing observations are not
relabelled as a sound admission of the old bad roots. The separately retained
initial native/perft domain-failure evidence remains diagnostic history.

This correction supports re-admitting and integrating the bounded MSH-64
deliverable. It does not itself close MSH-66, certify native conformance or
assign any history-dependent result. Those tasks retain their independent
runtime/implementation review and closure requirements.
