# MSH-117/R2 desktop envelope migration

Baseline `d6250cb71b7c498ddcdf0b82bb1041a4365c2c33`, primary `codex/rewrite`.
One root integrator; isolated consumer/fixture workers did not commit or integrate.
MSH-116 stays Done; no R3–R6, variant, rules or strength work starts here.

## Current boundary and deletion

The [current document contract](../desktop/document-v1.md) defines one
`mshogi.desktop.document.v1` envelope, version1, native-tagged raw payload and
strict game/profile/edition/native-schema identity. Every current writer uses
`document::encode`; every Open outer identity uses `DocumentEnvelope::parse`.
Native payloads and replay remain specialized.

Removed `ArchiveV2`, format/version probing, desktop1/2/3 acceptance switches,
Chess1/3 writer selection, `decode_chess_compatible` and `decode_chess_v1`.
Chess's native payload is `ChessDocument`, not a nested old desktop archive.
Take back entries are always a required vector; selected desktop event metadata
is explicit. There is no old-format reader or converter. Old-name occurrences
in current tests assert rejection; historical docs/evidence remain unchanged.

The finite bridge maps existing runtime captures to the one current writer and
validated new data to existing installation, with no independent live state.
MSH-118 deletes this bridge when taking active-document/metadata ownership.
R2 does not activate118 or change its dependency set.

Native evidence retained:

- Chess full MatchRecord policies/commands/actors/timestamps and final native
  GameRecord reconciliation, claims, conclusions, Full observations, original
  targets and abandoned Take back MatchRecord lineages. Historical selection
  now survives Save/Open separately from live state; fresh runtime selection
  identity and stopped work protect acceptance.
- Shogi drops/promotions/demotion into hands, declarations, native outcomes,
  control ordering and retired branch observations. The current ordinary GUI
  hand fixture is regenerated from native SFEN and the independently recorded
  two-action transcript, not converted from the historical desktop file.
- Xiangqi warnings/obligations/claimants/offers/conclusions and exact event
  prefixes, committed execution prefixes, ordinary/exact observations and
  proof/witness/Unknown/cancel/failure. Graph search is never rerun on Open;
  positive witnesses receive existing bounded native verification.

No core/search/engine rule or execution contract changed. Durable schema1 from
MSH-116 remains current; no request becomes pending merely by reopening evidence.

## Capacity and atomicity

Limits unchanged:16MiB actual encoded document,256 combined applicable
observations,8192controls, native event ceilings and game-specific replay costs.
All admission encodes the new full envelope with original payloads/branches,
maximum-width identity, historical-flag and stop/close reserve. Structural
refusal precedes commitment. Save and Save As retain complete evidence.

Chess now reserves an additional worst-selectable current native history prefix
in its existing retained-branch16,000-unit budget independent of current view;
otherwise selecting history after admission could consume unreserved Open work.
Ordinary branchless Chess retains native event/response/Full work limits and
15-second cancellable worker budget. Imposing the branch16kcap globally would
incorrectly reject supported40,001-action Full witnesses, so it was not done.
No constants were raised. Final branch-boundary probe admits115preferences,
120controls,315302bytes, then mandatory close and maximum history selection
both save/reopen successfully. The additional reserve accounts for real new
restoration work; it is not a generic replay policy across games.

Actual local debug fixture measurements (macOS27.0.1arm64, Rust1.99.1), current
working R2 code atop the baseline; full commands/diff provenance retained in
[local-measurements.txt](evidence/msh117/local-measurements.txt):

| Fixture | Current encoded bytes | Reopen |
| --- | ---: | --- |
| Shogi320actions |241130|passed|
| Shogi8192controls |698599|passed|
| Shogi256observations |581828|passed; reopened Save As581572bytes|
| Xiangqi89events/67controls/32observations |787976|286ms|
| Xiangqi103events/17controls/7observations |216405|501ms|
| Xiangqi1ordinary+255exact |603899|138ms|
| Xiangqi256ordinary |699949|261ms|

These are reproducible fixture observations, not universal maxima or performance
promises. The256ordinary test reuses one actual completed result under fresh
identities; it does not claim256searches. Exact16MiB Open padding/oversize,
actual admission byte refusal, native-event/control/replay thresholds,
256-to257, worst-width identities and stop reserves pass focused tests.
Interrupted writes preserve previous valid files; failed/stale Open leaves the
accepted document intact; reopening remains fresh, stopped and explicit Resume.

## Verification and independent review

Local focused passing checks before committed-source CI:

- 8current-envelope tests: all required/duplicate/unsupported/conflicting identities,
  raw nested dispatch, real native roundtrips, atomic malformed Open and bounded
  UTF8diagnostics.
- 12Take back tests; updated Chess selected-history claim/resignation roundtrips;
  focused persistence behaviors all pass, including actual byte-capacity stress.
- 12actual console desktop consumer tests (four suites),5native macOS host tests,
 43GUI-verifier Python tests; relevant all-target/all-feature Clippy, rustfmt,
  diff check pass. Logs retained in `evidence/msh117/local-*.txt`.

Initial all-feature working-source test run:150passed/4failed. Four failures
were test-fixture assumptions exposed by the cutover: two new helper assumptions
(Chess Observe-plus-Resign event count and failed Open snapshot revision), duplicate-control JSON
whitespace and an oversized-wrapper probe that injected into the new outer
header instead of the native wrapper. Corrected tests use public snapshots,
actual compact field bytes and explicit native needles; all focused reruns pass.
Early compile errors during type removal and one needless-borrow Clippy finding
were resolved. A subsequent rustfmt check requested one-line reflow and passed
after formatting. These are not claimed as game-rule defects or ignored checks.

Fresh read-only reviewer `/root/r2_independent_review`, not a builder, inspected
all production/deletion/persistence changes, task/owner contract and native
validators. It independently executed57tests:8current-envelope,12Take back,
20Xiangqi archive,16Shogi archive and1expensive Full witness byte-boundary stress;
all pass. It found uncapped diagnostic text and stale v2success wording; both
were fixed with real long malformed payload/identity tests. It challenged selected
history reconstruction headroom; the added reserve and existing Full-budget
distinction resolve that finding. **No material source finding remains.**

## Initial committed-source CI and required correction

Reviewed source `8f4136961633659571f57fe1645a47fcfee52d45` integrated/pushed
normally. Desktop41080845341 passed all3jobs; Rust41080845090 passed Linux/macOS
but **Windows failed**:155/156all-feature checks passed, and the extreme Full
witness byte-boundary case saved successfully then returned Failed on Open.
The original assertion omitted the I/O diagnostic, so its exact failure cause
cannot be established from that log; no unsupported timeout/race classification
is claimed. The [failing probe](evidence/msh117/windows-native-initial-failure.txt)
and exact first-run CI metadata/provenance remain historical evidence.

The failure exposed a real preservation risk: compact encoding can retain many
large Full certificates, but Open revalidated every repeated proof while
admission previously validated only new evidence. The correction retains the
same limits and complete storage, and shares successful native proof validation
only within one operation. Exact history/goal/limits/result/work/witness form the
key; every observation still receives all native context/perspective/lineage/work
checks. A fresh Open starts untrusted. New observation admission for Full-proof
documents runs the actual complete encoded Open/replay path under the existing
15-second budget, including prior distinct proofs. Ordinary immutable-proof
commands retain existing reserves without repeating unchanged certificates.

Five new native regressions pass, independently rerun:40,001-action certificate
first/fresh-operation native verification, bounded repeated-proof reuse,
same-board/different-history and distinct-witness misses,12later corruptions,
failed-proof cache exclusion, previous distinct proofs checked beyond the
watermark and atomic cancellation/refusal. Stricter initial correction (full
candidate replay on every command too) passed the unchanged byte stress body in
249.37s locally. Final scope avoids repeating immutable proof work for ordinary
commands; final committed-source Windows execution remains required below.

Only the extremely expensive byte stress is placed under desktop-verification;
the **unchanged mandatory all-feature native CI step still executes its full
body on every OS**, rather than twice in the same job's default/all-feature
passes. No body/threshold/assertion was weakened; its failure now prints actual
IoState, and success records encoded count/bytes/Open time. The5new cache/admission
checks remain in the default tests. Independent review explicitly approved this
verification-cost placement and final correction; no source finding remains.

At initial8f41369, actual Linux14-phase installed normal/separately labeled QA
GUI evidence passed:243file hashes,13normal package hashes,QApackage/host,
16normal-host instrumentation exclusions and14native pre-teardown shutdown
proofs audited by root and the fresh reviewer. Current-format saved Chess/Shogi
histories and both256-observation archives are retained. The actual new
Shogi declaration→Take back→Save/Open→stopped reopen→explicit Resume journey
retains `resume,declare,take_back`,0current native events, fresh document1→2 and
selection4→13, with no declared outcome revived. Selected original reports,
archives/captures and package identities are [indexed here](evidence/msh117/linux/artifact-index.json).
This is qualified initial-source GUI evidence; no Xiangqi or macOS GUI claim.
Linux native logs establish156/156all-feature tests; Desktop logs241frontend,
43Python verifier and5host checks. No failed native run is hidden by these results.

## Corrected committed-source CI and final acceptance

Pending the reviewed correction commit and applicable CI/package reconciliation.
MSH-117 remains In Review with unchecked AC/DoD. No R3 stage starts. MSH-112
remains a separate historical issue and is not assumed to explain this failure.
