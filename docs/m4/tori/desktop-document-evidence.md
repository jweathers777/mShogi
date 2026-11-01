# MSH-130 — codec/admission evidence

Contract and API: [current Tori desktop document](desktop-document.md).
Planning baseline `1e51ea51b3ba6997b75a7c11a25df0c7f4eeaad5`; native production
`994521bec0637d6f8f17103f1106c0025d950753` is unchanged. Implementation source
and actual integration/CI references are appended after normal source commit;
precommit executions are identified by source hashes, not a fabricated commit.

## Acceptance evidence

| Original MSH-130 criterion | Concrete evidence |
| --- | --- |
|AC1 immutable lossless current payload |Initial/current native records, exact journal/settings/observations, retained complete branches and retired floor. Real8/16history, imported provenance/counter, capture/drop/promotion360history, rewind and all-prefix roundtrips. No in-flight request is encoded. |
|AC2 strict raw dispatch/native replay |Recursive duplicate check before enums, explicit current/inner identities and authoritative native parser plus journal replay. All seven TORI-REJECT inputs, missing/unknown/conflicting fields, forged prefix effects/conclusions/work/root metadata, timed mode, corrupt/truncated/oversized and cross-game negatives. |
|AC3 actual measured resources |[Machine-readable actual measurements](evidence/desktop-document-measurements.json), deterministic seed and test builder. Actual complete encoded bytes/events/controls/observations and repeated native replay charges; independent reproduction. Existing caps unchanged; new262,144whole-document policy is distinct from65,536per native record. |
|AC4 admission/reservation |Immutable precommit candidate validation, raw precharge before native parse, actual native/outer bounds,256→257observation refusal,4096retained events,8192controls, correct typed resource facts. Remaining-operation byte/work/control/native-event reservations verified with real ending deltas and boundary probes. |
|AC5 worker/atomicity and regressions |Existing FileWorker/atomic_write first and replacement Save, actual long-record Save, strict read-back, cancellation before rename, corrupt Save and real rename failure, preserved valid old file/no false Saved receipt, cancelled/replaced results and stale operation IDs. Current game's runtime clean-ack/installation tests reused in full session suite; no Tori live installation claim. Actual native child UnsupportedMode observation survives codec/reopen without result/score/conclusion. |
|AC6 independent review/current handoff |Separate read-only source/security/codec/capacity/persistence review and independently executed tests; findings resolved below. Native/controller installation remains explicitly unsupported.131/132/77/42retain their separate acceptance. CI disposition recorded separately from local and GUI evidence. |

## Actual capacity observations

Local macOS27.0.1 (26A434), Apple arm64, Rust1.99.1; frontend Node24.19.0.
Numbers are actual pretty desktop encoding; native records remain their native
canonical encoding. Timings are observations under that run's load, not a promise
of identical timed reruns or playing strength.

| Real case | Desktop bytes | Retained native events | Controls / observations | Document work |
| --- | ---: | ---: | ---: | ---: |
|360board history,104captures/103drops/16promotions |206,796 |360 |360 /0 |131,424 |
|Same history+one full-history observation |364,006 |720 |361 /1 |197,021 |
|Stop after observation refusal |364,323 |721 |362 /1 |197,023 |
|Resign then Close |364,641 |722 |364 /1 |197,026 |
|Retire branch at event360 and restore prefix180 |287,206 |540 |361 /0 |213,815 |
|Forced8repetition-forfeit |8,500 |8 |8 /0 |480 |
|Unequal16history/obligation |12,033 |16 |16 /0 |704 |
|256root-context observations |1,316,178 |0 |256 /256 |66,176 |

Second full-history observation refuses at262,618units plus1,040reserved,
above262,144. The previously accepted one-observation document remains exact;
Stop→Resign→Close and actual atomic Save succeed. Native360history is156,573B /
65,341native units. Local initial complete validation2,648ms, long stopped/ended
worker Save4,022ms and256root observations678ms. These measure distinct operations.
They support a conservative fixed128root charge plus repeated native charges;
the existing15second worker deadline remains the independent wall budget.

Exact controls: ordinary8,189accepted, next ordinary8,190refused with3controls
reserved; Stop+Resign+Close reaches8,192and still roundtrips. Exact retained events:
a real concluded16event target plus255observations reaches4,096; the next16event
observation refuses at4,112while below observation256. Exact observations:256root
observations accepted,257refused without dropping any. These are independent
reachable quota cases, not a claim all maxima can be reached simultaneously.

Native512KiB input/+1 and event/replay precharge checks preserve native caps.
Exact scalar byte/work ceilings use real ending growth to challenge consumed
reservations. A16MiB padded-input probe checks admission and plus-one refusal;
canonical output removes that padding. It is **not** a measured16MiB stored
semantic payload. Additional controls/byte/work/runtime limits remain independently
bounded; no structural failure is fixed by increasing a constant.

## Commands, actual local results and limitations

[Actual local command output](evidence/desktop-document-local.txt) preserves the
full230test checkpoint, native38/713, final fast16, actual child and measured
worker outputs, with source-checkpoint distinctions.

Local executed commands (workspace root unless specified):

```text
cargo test --locked -p mshogi-desktop-session --all-features
cargo test --locked -p mshogi-core tori:: --lib -- --nocapture
cargo test --locked -p mshogi-console --test tori_internal
cargo test --locked -p mshogi-desktop-session tori_document --lib -- --skip measured_complete_long_documents
MSH_TORI_130_MEASUREMENTS=/tmp/msh130-measurements.json cargo test --locked -p mshogi-desktop-session tori_document::tests::measured_complete --lib -- --nocapture
cargo clippy --locked -p mshogi-desktop-session -p mshogi-console --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
cargo check --locked --manifest-path desktop/src-tauri/Cargo.toml --all-targets
# desktop/, bundled Node24.19.0 first on PATH:
npm run check
npm test -- src/product/resourcePresentation.test.ts src/product/NativeCapacity.test.tsx
```

- Full session all-features **230passed/0failed/0ignored** at the16-new-test
  checkpoint (175.07s), including existing current Chess/Shogi/Xiangqi archives,
  claims, exact/Full observations, Take back/retired lineages, resource facts,
  interrupted/stale receipts and lifecycle. Subsequent raw identity/per-record
  hardening and seventeenth focused test are independently checked separately;
  final exact-source CI supplies the integrated full run.
- Core Tori **38passed**, with **713executed/713passed/14pending/0failed** and
 83later consumer requirements. No native rule/source/fixture change.
- Real child→UnsupportedMode observation→desktop codec strict reopen **1passed**.
- Focused final tests and long-record worker experiment passed; their source and
  command results are recorded with the final review/CI section below.
- Clippy/format passed; standalone Tauri all-target check passed with20existing
  vendored Tao warnings. No Tao/signature/build-identity changes.
- TypeScript passed; frontend **2files/36tests passed**. Tori resource facts use
  the shared existing wording without a second frontend/presentation owner.

Historical intermediate failures, accurately distinguished:

1. Initial test compilation failed on a fixture-relative include path and wrong
   engine capability function name; corrected to actual paths/types before testing.
2. First native-byte probe placed whitespace **outside** its RawValue; the test
   incorrectly expected that padding to count as native payload bytes. It failed,
   then was corrected to put padding inside the native object; targeted rerun passed.
   Native parser and resource caps were not changed to accommodate the test.
3. Early test compilation had an unused-mut warning; removed. An early format
   invocation preceded creation of its declared test module; rerun passed after
   adding that module. Neither is a passing test claim.
4. System Homebrew Node26failed to start (missing libsimdjson.33.dylib). Reused
   the established bundled24.19runtime; TypeScript/frontend passed. No system
   installation or permission/security policy was changed.

No Tori GUI/app launch, final product renderer or live engine/session is claimed.
No external game engine was built or invoked. M1/M2/M3 and unchanged earlier
native/platform evidence is retained, not recast as this codec's runtime evidence.

## Independent method and resolved findings

Separate `tori130_review` agent, read-only source/diff/contract review, independent
command execution and measurement reproduction. Root is sole builder/integrator.
Reviewer independently executed16focused codec tests (16PASS/0failed/0ignored,
41.57s), the actual console subprocess test (1PASS), and last inner-identity/
per-record-precharge/unsupported-counter tests (PASS). Independently measured
bytes/work match the table exactly; timings differ legitimately. This is an actual
separate reviewer, not author's rereading or a tool-generated certification.

Material findings resolved before acceptance:

- Include historical selection IDs in safe retired execution floor/admission.
- Check already-cancelled candidate construction before native reconstruction;
  check cancellation between native record/observation parsing calls.
- Consume byte/work reservation per remaining ending operation; a fixed boolean
  allowance would refuse Stop at a ceiling that purportedly reserved it.
- Route all seven strict fixture cases through the desktop codec, and distinguish
  sole unsupported timed mode from merely duplicate mode-field rejection.

A proposed counter-test concern was checked against actual `WirePosition` and
withdrawn: `declared_total_plies` is a real checked native field. Extra
`declared_completed_plies` coverage was added without changing expectations.
Final source review PASS with no unresolved material findings. Final evidence/
CI metadata review and actual integration references follow below.

## MSH-131 handoff and preserved pending obligations

Use this immutable payload and existing NativeWork/FileWorker family from the
single DesktopController and sole installation method. Runtime must admit the
complete candidate before accepting document changes; preserve current-token
checks, monotonic IDs, stopped reopen, unsaved guards and reserved close/stop.
Validate a Saved receipt against **current** content revision/document identity;
this codec never clears dirty state or installs a runtime. Saved observations are
historical, no request to resume. Retain complete branches when Undo/Take back
changes the native current prefix. Expensive preparation belongs off the UI path.

All14pending assertions and83consumer requirements in
[the supplemental map](shared-consumer-ownership.json) remain pending:131owns13/82
actual runtime verdicts;42retains notation1/1.130provides supporting strict/native
record/admission evidence, never falsely checks the actual live-document fields.
131must wire typed settings/commands to native authority and prove live rejection,
Open/Save/replacement/selection/Undo/Take back/retirement;132provides complete
semantic/frontend verification;77final integration and42actual playing delivery
stay open. Other variants, M5 and M4closure are untouched.

## Final pre-integration review checkpoint

Independent final source/API/security/capacity/persistence and evidence-document
review **PASS**, no unresolved material findings. Source hashes are in
[static validation](evidence/desktop-document-validation.json). Final author fast
codec run16PASS (the long measurement deliberately excluded), separate final
long-measurement/actual-worker run1PASS, actual child1PASS. Reviewer independently
checked the complete16test checkpoint plus added identity/precharge test and
changed counter/mode coverage. All17codec checks have supported focused evidence.
Final Clippy/format passed. Only normal exact-source integration/CI and completion
bookkeeping remain; this checkpoint itself does not mark the task Done.

## Exact source integration

Reviewed implementation is integrated and normally pushed to `codex/rewrite` at
`8fd12f953beeacac46fdd417b2054bedeafe2d6e`. Source hashes match the reviewed static
validation. [Rust workspace41355796434](https://github.com/jweathers777/mShogi/actions/runs/41355796434)
and [Desktop packages41355796473](https://github.com/jweathers777/mShogi/actions/runs/41355796473)
were dispatched for that exact revision. At dispatch they are pending, not passing;
final actual results are appended below. Task remains In Review until required
checks support all criteria. No additional Tori GUI campaign is inferred.

## Final exact-source acceptance and CI disposition

**MSH-130 acceptance PASS.** Native Rust workflow41355796434 completed SUCCESS on
Ubuntu24.04, macOS15 and Windows2022 at exact integrated source
`8fd12f953beeacac46fdd417b2054bedeafe2d6e`. Completed logs on **each** platform
establish218default session tests and231all-feature session tests, all17new
codec tests in both runs (34passing lines), and1actual Tori child→codec roundtrip.
Format/lint, native snapshots, build and console smoke also passed. The two older
deep Chess opt-ins are not newly claimed here.

[Actual CI metadata and verified log counts](evidence/desktop-document-ci.json)
preserve the captured state. Desktop workflow41355796473 has completed SUCCESS
on macOS and Linux. Windows locked frontend/native checks, normal package,
bundled-engine inspection and separate QA package all passed; **Windows's existing
Chess/Shogi/Xiangqi GUI stage is still running at this capture**, so neither its
GUI result nor the whole desktop workflow is described as passed. No Tori GUI
campaign or product acceptance is required or inferred for this codec-only task.

Separate final independent acceptance/CI reconciliation PASS: all six original
criteria and four completion requirements have evidence, no unresolved material
finding. The pending unrelated existing-game GUI stage is accurately preserved,
not added as a new Tori gate. Any subsequent concrete failure attributable to
this source must still be assessed; historical failures are not rewritten.
The final task/document bookkeeping is documentation-only and reuses this exact
reviewed production/CI evidence. All source hashes remain unchanged.

MSH-130 is Done;131is the next dependency-ready Ready slice and remains unassigned
and unstarted. Its required installation/runtime checks remain pending.132,77final
acceptance,42playing delivery, future variants and M4closure are not started or
closed by this task. Primary checkout remains `codex/rewrite`; no task worktree
was created and unrelated worktrees were preserved. Stop after130.

## Completed desktop CI follow-up

The final GitHub snapshot now establishes **SUCCESS for the entire Desktop
packages workflow41355796473**, including all macOS15, Ubuntu24.04 and
Windows2022 jobs at unchanged source`8fd12f953beeacac46fdd417b2054bedeafe2d6e`.
The Windows existing-game GUI stage completed successfully after the earlier
pending capture above. Updated actual CI metadata preserves that capture
provenance and the final completed jobs/steps. This is existing-game CI evidence,
not a claim that Tori live installation or Tori GUI play exists. No source change
or repeated campaign was needed.

Final bookkeeping checks: pinned Backlog1.55.1 doctor PASS; static scope audit
PASS (132tasks/353edges, unchanged dependencies and original criteria, matching
source hashes,23valid local links); git diff --check PASS. Ready dependency query
returns only MSH-131, which remains unassigned and unstarted.
