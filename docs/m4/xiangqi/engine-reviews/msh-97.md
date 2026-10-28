# MSH-97 independent review

Method: separate Codex reviewer session `console96_review`, bounded source/test
and documentation diff based on completed MSH-96 integration `6c37398`, against
MSH-97, the authorized queue and MSH-84 native consumer contract. Independent
fixture authors were separate from both builder and final reviewer. No external
engine evaluation/adjudication was used as an expected-result authority.

Final verdict: **no unresolved findings**. Reviewed full-history minimax, native
terminal/claim/pending admission, balanced reversal and semantic root comparison,
recorded fast versus explicit root-exact provenance, unscored uncertainty/error,
bounded work and honest completed iterations, structured result matching,
capabilities, cancellation/replacement/retirement and compatibility boundaries.

Resolved verification gap: C10 avoidance alone did not establish that warning
history changed the best action. The independently justified C11 comparison
now proves known `e8f8` penalty win versus fresh `e8e9` capture-to-bare-General
draw, with explicit native witnesses. Existing rules/fixtures were unchanged.
The final documentation also corrects 'each iteration reconstructs' to one
private reconstruction with restored-root reuse.

Reviewer independently executed:

- `cargo test --locked -p mshogi-search --test xiangqi`:12 passing at first
  final review; separately reran the added deterministic default/tie test1/1.
- `cargo test --locked -p mshogi-engine --test xiangqi_handoff`:6 passing.
- `cargo test --locked -p mshogi-engine --test xiangqi_routing`:7 passing.
- `cargo test --locked -p mshogi-console --test xiangqi_internal`:7 passing,
  including normal executable cancellation and cross-game replacement.
- `cargo run --release --locked -p mshogi-search --example xiangqi_baseline`:
  success, standard about30ms default/301ms ceiling, exact255-state closure
  about0.7ms. Actual observations, not latency/strength guarantees.
- `git diff --check`:pass.

Total33 distinct focused tests passed. Source, engine.md, measurements and
scope claims were reviewed together after the final corrections. No GUI,
external protocol or strength delivery is inferred. Final source integration,
platform CI and task acceptance bookkeeping remain the integrator's recorded
responsibility in MSH-97.
