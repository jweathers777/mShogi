# Owner direction: automatic-threshold-only competitive Chess

John requested this bounded correction on12November2026 before the next cloud
batch. It supersedes the frozen AutoClaim/no-production-change restriction only
for this correction and its necessary deployment. Continue MSH-161 normally after
verification without another owner checkpoint.

The controller must continue through threefold/fourfold repetition and the50-move
claim threshold; native fivefold/75-move automatic draws remain mandatory, along
with checkmate, stalemate and dead positions under the existing rules profile.
Neither engine receives an automatic optional claim from the controller. Search
must not assume an unavailable optional claim ends play. UCI must return an actual
searched legal move, not a claim-induced arbitrary fallback. Inspect native rules,
external match runner, UCI and conventional search together; no new claim protocol,
UCI redesign or unrelated search/evaluation changes.

Use focused native/search/UCI tests and one short actual-runner match validation
with both mShogi and Oxidation. No large validation study. Deploy corrected Linux
binaries and pinned worker image, update and verify future job manifests/identities
at a completed-batch boundary. Do not interrupt or modify running workers.

At receipt of this direction, batch02 and both followups have finished; independent
runtime evidence shows no remaining workers. Batch03 has NOT been submitted.
Primary batches01–02 comprise40preserved AutoClaim games. Flag their12optional-claim
draws as potentially affected; do not discard or replay them. Preserve the earlier
Mac/pilot strata separately. The remaining160primary games keep their original
opening pairs, equal practical time, opponent and analysis coverage, using the
corrected policy starting withbatch03 after deployment. Treat the policy/source
change as an explicit analysis stratum, especially for draw rates and conversion.

Report implementation, focused verification, short end-to-end validation, Linux
image/manifests deployment and the first corrected batch. Preserve all failures,
provenance and actual resource accounting. Exit notification remains required;
no periodic watcher or guessed wake time. The wider study still stops for John
before unrelated engine improvements or final roadmap implementation.
