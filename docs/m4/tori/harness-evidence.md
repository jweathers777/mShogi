# MSH-126 executable harness evidence

Owner correction2026-11-01 supersedes the former mandatory comparator gate.
Baseline `04225043545a7219fbbad6024eb2398c5f68aaf2`; primary `codex/rewrite`.
Root Codex sole builder/integrator, pinned Backlog1.55.1, original unrelated
worktrees preserved. No native Tori kernel, other game behavior or desktop changed.
Current independent source/fixture expectations come from completed MSH-81, not
implementation or external-engine output. Exact profile,103-case corpus and
ownership bytes remain unchanged; their SHA is retained in the CLI and tests.

## Revised acceptance and actual evidence

| Revised126 criterion | Evidence |
| --- | --- |
| AC1 strict self-contained fixture admission | Recursive duplicate checks, typed definitions, fixed103-ID registry/hash, immutable fixtures/provenance/owner mapping; malformed/missing/unknown/conflicting input tests. |
| AC2 usable typed observations and honest accounting | CLI/library native adapters, six boundaries, owner-qualified field keys, supplemental structural/property facets, bounded expected/observed/input diagnostics; default103fixtures/0executed/0passed/727pending/0failed,4annotations and83consumerrequirements pending. |
| AC3 meaningful deliberate negatives | Missing/extra/duplicate actions, wrong destinations, capture effects, promoted/demoted/quail identities, hand owner/count and history; nested duplicate/unknown/conflicting input and nonzero CLI failure cases. Constructed observations only, no native conformance claim. |
| AC4 properties/metamorphisms | Actual closure-based round_trip/rejection helpers, snapshot/replay/material comparison, deterministic2000generated snapshot mutations,49coordinate transforms,103input involutions,35independently expected rotations, explicit owner exchange and bad undo/mutating rejection. Real native properties remain pending127–129. |
| AC5 policy/reconciliation/review/integration | Projectwide policy+design/AGENTS, current126scope and128comparison requirement reconciled via CLI, future Tori/Chu/Tenjiku notes current, old comparison evidence preserved; separate review PASS. Actual integration/CI references appended after execution. |

## Executed local checks

macOS arm64, rustc1.99.1 `58cf8f9c3`, cargo1.99.1 `5f94df478`:

```sh
cargo test --locked --offline -p mshogi-conformance
cargo clippy --locked --offline -p mshogi-conformance --all-targets -- -D warnings
cargo fmt --all -- --check
cargo run --locked --offline -q -p mshogi-conformance
# pinned Backlog1.55.1:
backlog doctor
python3 docs/m4/tori/evidence/verify-harness-integrity.py
git diff --check
git diff --cached --check
```

Final focused suite: **18 passed,0failed,0ignored**. CLI input errors return2,
wrong constructed observations return1, required-owner/no-observation and
all-complete guards return1; default data-validation run returns0 with native
work explicitly pending. No reference engine, Python engine module or external
process participates in the library/CLI tests (CLI tests launch only this binary).

Initial scaffolding compile failed because Corpus lacked Debug for unwrap_err;
corrected. An initial annotation-count assertion expected5 rather than the four
actual documentary proofs; corrected against the enumerated fixture fields.
The new terminal-layer regression initially exposed a missing guard on local
rejection assertions; fixed and rechecked. These earlier failures are not passed
native checks or hidden changes to independently expected rule data.

## Actual independent review

Separate read-only reviewer `conformance126_review` inspected owner direction,
revised task/policy, source/fixture independence, strict parsing, predicate and
property semantics, accounting, adapter usability, task ownership and dependency
wiring. Independently ran cargo test (18PASS), clippy (PASS), and default CLI
(103/0executed/0passed/727pending/0failed/4annotations/83consumerpending).
No repository edits by reviewer. Review passed after these findings were resolved:

1. Involution alone missed bare s/g owner exchange; transformer and explicit
   wrong-owner negative corrected.
2. Primary row labels omitted executable127 structural effects/reversal facets;
   owner-qualified supplementary projections and real snapshot checks added.
3. Game command fields for impasse/agreement needed History classification;
   terminal-command routing and regression added. The reviewer's initial suspicion
   that the immutable manifest owner was128 was incorrect; it was already129.
4. Minor test-count documentation17→18 corrected.

Review SHA256 for final `assertions.rs`:
`1c17a14bfd6d5d338f1a3120b51b2ec9ef542038172432e03411c12bb113cef6`.
Final integration references identify the actual source; no future hash fabricated.

## Scope, pending work and CI

No Fairy-Stockfish or any other external engine was built/run. No selected Tori
rule or expected data changed. Historical source audit/comparisons remain recorded
as optional/nonauthoritative observations. Existing external-engine interoperability
product capabilities were not removed. No packaged GUI campaign is required.

Normal Rust workspace CI includes the new crate automatically; no external engine
or workflow dependency added. Final native CI outcomes are recorded below by exact source. Desktop workflow construction, if automatically triggered, is supporting
unchanged-product evidence, not a new GUI verification claim or126requirement.
MSH-127 becomes dependency-ready only after126 closes.128/129/85kernel/77/42 and
other variants remain unstarted.83pending consumer obligations and --require-complete
fail-closed behavior are explicit limitations, not deferred harness correctness.

[Metadata integrity result](evidence/harness-integrity.json) checks129unique/resolved
task IDs/344edges/no cycles, unchanged dependency sets and future implementation
plans, unchanged completed tasks and selected profile/corpus/ownership, local
links and no changes to existing product crates/desktop/workflows. Reproduce with
`python3 docs/m4/tori/evidence/verify-harness-integrity.py`; this optional metadata
check uses standard Python only and is not a native conformance dependency.

## Actual first integration and Windows CI correction

Reviewed Rust source integrated and pushed at
`134f81ba67ef81918734ed1ed1acb5de7532fb99`. The first native CI run
[41314482706](https://github.com/jweathers777/mShogi/actions/runs/41314482706)
passed format/lint on all three systems, but Windows job124871524762 failed the
new harness tests (5passed/13failed): embedded corpus SHA differed from audited
LF bytes. This failure remains historical evidence, not a passed platform check.

The correction is two scoped `.gitattributes` entries enforcing `text eol=lf`
for the corpus and ownership JSON. No hash normalization, changed constant,
expected-data edit or test weakening. [Actual Git export probe](evidence/harness-eol-probe.json)
with `core.autocrlf=true`/`core.eol=crlf` retains exact141871-byte corpus/hash and
85731-byte ownership/hash. A separate reviewer repeated the export and compared
both against source134f81b Git blobs; PASS. That is a Git portability probe,
not Windows runtime verification. Actual final-source CI is recorded below.

[Executed CLI counterexamples](evidence/harness-negative-probes.json) at134f81b
return1 for a constructed wrong hand count and missing eight Phoenix destinations,
with input/expected/observed diagnostics. These are assertion-machine tests only.

[First CI dispositions](evidence/harness-first-ci.json) at134f81b: Linux workspace
passed; Windows failed the harness SHA check; macOS passed the new harness but
later failed unchanged `mshogi-runner` test
`missing_retirement_eof_cannot_hide_a_delayed_duplicate_bestmove` at
`shogi_runner.rs:355` (LateReply versus expected ResourceLimit). Runner sources
are unchanged. This differs from MSH-112's early-exit test; no common root cause
or runner fix is asserted. Required final-source outcomes remain separate below.

## Final integrated source and completion

Final tested source **`d5f98a2af9e38ae690e2fd5c4940a4d26d12407b`**, integrated
and pushed on `codex/rewrite`, includes the scoped LF checkout correction.
[Native workspace CI41315037374](https://github.com/jweathers777/mShogi/actions/runs/41315037374)
**PASS on Ubuntu24.04, macOS15 and Windows2022**, including formatting, lint,
workspace tests (all18 harness checks), isolated QA constructors, native snapshot
fixtures, build and console smoke. [Exact run/job/step evidence](evidence/harness-final-ci.json)
records actual source, environments and results. The prior Windows failure and
unchanged macOS runner failure above remain historical; the passing rerun is not
evidence of a runner fix.

MSH-126 is complete under its owner-revised five criteria and four completion
items. Final task/document-only bookkeeping introduces no code change and uses
the established `[skip ci]` convention rather than rerunning unchanged campaigns.
The automatically triggered Desktop packages workflow is outside this harness
boundary; no new packaged GUI verification is claimed. No external game engine
was built or invoked, and none is required for ordinary conformance, tests, CI,
packaging or release. MSH-127 is the next dependency-ready bounded task; it must
supply real native observations and properties. MSH-128/129, MSH-85 final native
acceptance, MSH-77/42 and other variant implementation remain unstarted. The103
specifications are admitted data, not executed native rules conformance; all727
native assertions and83 later-consumer requirements remain honestly pending.
