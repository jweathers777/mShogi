# Coherent-round evidence

See [the report](../../chess-decision-round.md) for interpretation and limitations.
Native production source is frozen at `3b52314`; binary and 96 source hashes are
in `native-build.json`, together with the separately frozen predecessor. The
original snapshot paths in receipts are execution provenance, not portable paths.

`decision-runs.tar.gz` contains the native and finite-reference attempt ledgers,
manifests, generated results and controls. `decision-bundle-manifest.json` maps
archived hashes to the original local execution hashes. Extract to a scratch
directory; native directories are under `native/`, reference runs under
`reference/`, and supervisor/selection controls under `control/`. The corpus
[README](../msh156/corpus/README.md) describes source retrieval and byte-identical
reconstruction. Remap original absolute paths to reconstructed inputs and retained
artifacts when auditing a portable extraction; do not silently edit original
receipts or assert that their hashes authenticate transformed files.

STS upstream data rights were not established. Its source rows, native input logs
and raw reference position commands are therefore omitted, and generated FEN fields
are removed from archived STS results. IDs, decisions, timings, scores, legal-PV
observations and original fingerprints remain. Archive hashes authenticate stored
bytes; original hashes document inspected execution bytes. Omitted artifacts are
listed explicitly. No STS source payload is claimed to be preserved by this bundle.

All native/reference searches were sequential. The additional historical-exposure
audit overlap and timestamp-selected quiet confirmations are retained separately;
original primary results were not replaced. Offline audit CPU is separate from
active engine charges. Global resource receipts include the first failed cost
validation attempt. Source hashes permit reuse of unchanged local correctness
checks; no cross-platform or GUI success is inferred.

`cost/` retains selected workloads, initial failure and retry evidence. Final
comparison, exposure, resource and independent-review records accompany this
bundle. Reference opinions remain finite analysis, not exact rules or strength
certificates. No tournament was run.
