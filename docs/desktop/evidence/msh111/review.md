# Independent review: MSH-111

Reviewer: independent `msh111_review` agent; root was sole builder/integrator.
Scope: revisions `b8beae4` and `2611d09`, task/design boundaries, external
Linux driver preparation and actual-proxy experiment, shared wait change,
mutation non-replay guardrail, CI wiring, diagnosis and retained failure index.

The reviewer found that the first pooling experiment used urllib's forced
`Connection: close`, so it could not deterministically establish reuse. Revision
`2611d09` resolves this with explicit keep-alive in the reuse experiment,
recorded/asserted headers and connection identity. Delivered-failure and
semantic-error experiments retain the actual verifier transport. Re-review
found no further implementation findings.

The reviewer confirmed that `retry_canceled_requests(false)` remains intact,
mutations remain single-attempt, semantic errors and exhausted observations
remain fatal, and no journey assertions or normal distributable inputs change.
The controlled mechanism is explicitly distinct from proving the historical
WebKit cause.

Independent command, macOS / Python 3.14.7:
`python3 -m unittest discover -s desktop/scripts -p 'test_gui*.py'` — 36 passed.
The reviewer independently verified all 117 originally retained files against
`prior/index.json`. Attempted local socket experiment was unavailable because
the reviewer sandbox denied bind; no local real-proxy success was claimed.

Documentation review identified a claimed retained build log without a durable
file reference. Root retained the original Linux workflow log separately with
its own SHA-256 and job ID in the index, and linked it from the diagnosis.
At that review stage the corrected CI and actual packaged GUI results were
pending; final assessments follow below.


## Final independent Linux evidence assessment

At tested revision `2611d0921dd3c637e76b7bb08bc0a23b49d84f33`, Desktop CI
40706737407 / Linux job122880769223 passed. The reviewer independently audited
all14 GUI phase verdicts with no failures, all223 GUI report file checksums,
all13 normal package manifest files, the verification Debian hash, normal
phase host/engine identity against package smoke, and executed driver identity
against provenance. All14 phases independently established native host and
engine absence before teardown and confirmed cleanup isolation.

All six actual-proxy cases matched the expected independent request receipt /
connection counts and fatal behavior. No ordinary observation/lookup retry
events occurred in the GUI campaign. Five final native guard transport closures
were qualified and followed by independent shutdown absence. One promotion
startup Connect/refused log corresponds to recorded driver-readiness polling,
not mutation replay. Selected native claim and reopened Shogi history/analysis
screenshots and DOM captures support their recorded results.

No actionable findings. This establishes one complete successful actual Linux
installed/packaged campaign, without claiming exclusive historical causation.
The final workspace-relative provenance-copy adjustment was separately reviewed
as mechanical: it preserves failure evidence and artifact roots, and changes no
execution, mutation, package or assertion behavior. No additional behavior test
run was requested for it.


## Final compatibility and acceptance assessment

The reviewer independently verified the Windows report's all14 passing phases,
196 evidence hashes using original Windows path semantics, normal package
identities, all14 pre-teardown native host/engine absence events and cleanup
isolation. No ordinary transport recovery events or final transport closures
occurred in that Windows campaign.

The reviewer also verified all142 selected successful-evidence index hashes and
CI metadata showing successful Desktop and Rust jobs on all three platforms at
`2611d09`. Final diagnosis claims match the evidence, including the package/native
checks-only limit for macOS. No code or evidence blockers remain. The reviewer
recommends acceptance and integration of MSH-111; root retains responsibility for
normal integration, preservation of MSH-99 work, completion records and cleanup.
