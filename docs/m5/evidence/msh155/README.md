# MSH-155 (Chess Time Scaling): evidence

Diagnostic source d53f344; repository baseline dcc17e1. No production changes.

`panel-frozen.json` retains the exact pre-run input and hash. The stable benchmark
in docs/m5/chess-decision-benchmark.json changes only source/proof path references
to durable repository paths. Expectations, histories and positions are unchanged.
The original panel-design text contains a notation error: b6f2 is **Qf2+**, not
Qxf2; f2 is empty. UCI/proof/PV data are correct; final report notation was checked.

`runs/all-summary.json` is the complete derived82-cell diagnostic trajectory.
The baseline has72cells, selected30s6, intermediate2/3s4; smoke3 is separate.
`adjudication/findings.md` and summary/ledgers describe23successful independent
queries plus one excluded initialization failure. Its final paragraph predates
final30sselection; methodology.md records the actual Dedrle/STS tie-break decision.

Raw artifacts are losslessly bundled as runs-artifacts.tar.gz and
adjudication-artifacts.tar.gz. bundle-manifest.json fingerprints every entry.
Unpack into a scratch directory; entries use relative runs/ and adjudication/
prefixes. Original `/tmp/msh155/*` paths map to runs/* and
`/tmp/msh155-adjudication/*` to adjudication/*. Drivers retain actual execution
paths/provenance; adapt root, binary and output paths before reproducing elsewhere.
The convenient manifests/ledgers/drivers outside the bundles are byte-identical.
No binaries, build caches, virtual environments or private settings are included.

`panel/` retains puzzle/API responses, complete tablebase maps, bounded mate-proof
script/receipt and freshness check. Only the selected STS row is retained; its full
source URL/hash and line number are in sts-source.json. The source manifest retains
original full-source/unused-candidate hashes for provenance, not a claim that every
original scratch file is copied. Unused candidates were discarded before answers.

The adjudication initialization failure (SIGXFSZ beforego), reviewed retry and
old/new drivers/manifests are retained. Subsequent3squery configurations were equal.
All budget charges include failures; the inheritedcap change did not alter Chess
settings. No result from a failed query was used.

The external engines and APIs are optional diagnostic evidence, not normal test,
CI, product or conformance dependencies. Native roots assert unchanged state and
legal PV admission; independent proof/tablebase outcomes have their own authority.
The methodology review is independent of the driver author; harness-verification
is an author evidence recheck, not falsely labeled independent driver review.

Run `python3 verify_evidence.py` from any directory to check the committed bundle, source hashes, fixture equivalence and report links. `verify_harness.py` retains the original full scratch-path checker; adapt paths after unpacking. Current adjudication driver/collector/candidates are provided separately alongside archived historical driver evidence. `verification.json` is the consolidated verification record.
