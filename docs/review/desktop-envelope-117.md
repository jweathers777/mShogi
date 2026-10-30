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

Corrected production source is `eea8ce1c0e7b51bf02b42aa62f69d9c833c5735e`.
Desktop [41086133015](https://github.com/jweathers777/mShogi/actions/runs/41086133015)
passed all three systems, including actual Linux/Windows Chess/Shogi GUI steps.
macOS establishes package construction, native host checks and bundled-engine
execution, not new GUI interaction. Native
[41086132999](https://github.com/jweathers777/mShogi/actions/runs/41086132999)
passed macOS and Ubuntu; Windows passed the complete workspace suite, then
160/161 all-feature tests before the overall 15-minute CI job timeout cancelled
the unfinished Full byte-capacity stress. There was no new assertion failure.
The [exact selected timeout log](evidence/msh117/windows-native-corrected-timeout.txt)
and complete job metadata remain separate from the original failing Open.

Independent review approved a minimal CI-only job budget of25minutes at
`ad14caed788f105e27a95868646db8d60cebd310`. Product code is byte-identical to
eea8ce1; the native15-second validation budget, byte/count/work limits and every
test body/assertion remain unchanged. This allows additional time for the workspace
and full QA suites on hosted Windows; it is not a product-limit increase or a
claim that the unfinished test passed. Final
[native41088858739](https://github.com/jweathers777/mShogi/actions/runs/41088858739)
and [Desktop41088858764](https://github.com/jweathers777/mShogi/actions/runs/41088858764)
passed all three systems at ad14cae. Each native job passed149 default desktop
tests plus161 all-feature tests, including the complete Full byte-capacity case.
Windows job124177086439 finished in15m27s; its all-feature suite took399.00s,
with the unchanged stress test passing. Linux's all-feature suite took356.18s,
macOS248.63s. The [original selected results](evidence/msh117/native-final-results.txt)
and [final native](evidence/msh117/native-ci-final.json)/
[Desktop metadata](evidence/msh117/desktop-ci-final.json) retain exact provenance.
Final Desktop CI includes Linux/Windows actual GUI interaction and macOS package/
host/bundled-engine checks. No final-source GUI inference is made for Xiangqi or
macOS. There are no remaining required CI failures or unavailable R2 checks.

### Actual current-source Linux package evidence

Run41086133015/job124167653182 produced artifact12991683842,
`desktop-ubuntu-24.04`,73155706bytes, reported artifact digest
`sha256:8473001ba383e9db8a516a008a345820105372fb307bc4cae92cdf9f463440e8`.
The separately labeled QA Debian package and extracted host were used for
difficult states; the ordinary installed package exercised ordinary Save/Open
and historical analysis. Normal distributables exclude all16 tested QA markers.
Normal host SHA256 is
`80aba03ee087be87a9138ae70e184a6b10ced580dc02f070b41479ba4c2e0181`;
the unchanged bundled engine SHA256 is
`1e22e13216061d515b6214c7ef0e4917b99ee40ccd593d87bfcdc45f8e2ed3b2`.
Exact QA/package identities and original evidence bytes are
[indexed separately](evidence/msh117/linux-current/artifact-index.json).

The root integrator and separate read-only reviewer independently audited all
243 GUI file hashes,13 normal-package manifest hashes,14 passed phases and14
native pre-teardown shutdown proofs. Both256-observation documents are complete.
Actual native Save/Open retains Chess and Shogi selected-history targets;
Shogi declaration→Take back→Save/Open retains the complete abandoned lineage
while reopening at0 current native events with no conclusion, fresh document
identity1→2 and selection4→13, stopped until explicit Resume. Stopped/resumed
native window captures and DOM observations are retained. Ubuntu24.04/X11,
WebKitGTK2.52.6 and the recorded MSH-111 no-pool driver provenance identify this
environment; no driver fix or MSH-112 resolution is claimed.

Windows GUI success is recorded by the exact CI step, with no independent
Windows artifact-content audit claimed here. The audited Linux campaign meets
R2's identified-platform requirement. No new Xiangqi or macOS GUI campaign is
claimed; final integrated three-game/three-system GUI acceptance remains R6.

### Acceptance reconciliation

| AC | Evidence and disposition |
| --- | --- |
|1|One strict current envelope; all old desktop readers/writer switches deleted; raw duplicate-preserving parser and identity tests pass.|
|2|Native MatchRecord/journal replay and current roundtrips retain complete game-specific events, prefixes, selected history, settings and Take back branches.|
|3|R1 schema1 retained; ordinary/exact/Full observations remain historical with exact targets, native proof evidence and fresh stopped reopen. No graph search or pending execution restored.|
|4|Actual encoded measurements and byte/event/control/256-observation/replay/headroom tests; complete Full-proof candidate admission and operation-local native validation; unchanged product limits. Full byte stress passes on all three systems.|
|5|Atomic failed/interrupted writes and invalid/stale/resource-limited Opens preserve accepted state; actual GUI documents reopen stopped with fresh identities and explicit Resume.|
|6|Only current capture/install bridge; no extra live state or old reader; deletion endpoint explicitly owned by118.|
|7|Native semantic fixtures plus actual current-source Linux packaged Save/Open/history/Take back/failure journeys; normal/QA package separation audited.|
|8|Fresh independent source/test/package review; original failure and timeout preserved; final three-platform native/Desktop CI passes at ad14cae, whose product code is identical to eea8ce1. Normal integration/push recorded.|

No R3 stage starts. Legacy development files are intentionally unsupported;
Chess timed/live-Full/witnessed-match restoration remains unsupported while Full
analysis evidence is retained. MSH-112 remains separately owned. No unresolved
material source finding remains. All eight criteria and applicable completion
items are supported. MSH-117 is complete after final evidence/task integration;
the next dependency-ready bounded stage is MSH-118/R3, which remains unstarted.

The same separate read-only reviewer made a final fresh acceptance reconciliation
against the actual final CI JSON/logs, byte-identical production tree and audited
package artifacts: **all eight AC supported; no unresolved material finding**.
Its [recorded final disposition](evidence/msh117/final-independent-review.json)
distinguishes actual independent test execution, source/evidence review and GUI
artifact audit from an additional GUI campaign or human owner review.
