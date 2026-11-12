# MSH-161 (Competitive Gameplay Diagnosis): automatic draw threshold transition

The owner authorized this bounded correction before primary cloud batch03. See
`owner-direction.md`. The existing primary40games remain AutoClaim evidence;
the remaining160games retain their frozen openings, opponent and5s/move conditions
under ExplicitOnly controller policy. Neither engine can claim through UCI.

Native automatic fivefold/75-move results and checkmate/stalemate/dead outcomes
remain the sole rules authority. Conventional search receives immutable optional
claim availability per request. UCI disables it throughout search; native/manual
callers retain their defaults. A completed unavailable Claim is an error rather
than an arbitrary legal fallback. The existing no-completed-iteration fallback
is unchanged. Runner configuration/replay accepts and binds ExplicitOnly.

`native-verification.json` records targeted Level2 local checks and preserved
failures. `adapter-verification.json` records the amended cloud adapter checks.
Broader GUI/release/Windows compatibility is deferred; no such pass is implied.
Native Linux build and one short actual runner/Oxidation validation are deployment
gates. The validation is not a primary game or a strength estimate.

`prepare_corrected_cohort.py` creates staging manifests only. It verifies all80
remaining pairs against original schedules/cohort hashes, preserves original
manifest bytes, binds new native and worker identities plus amendment, and keeps
Oxidation/Stockfish unchanged. Package-only jobs cannot execute. Final activation
must use the newly built image digest and a separately recorded admission check.
The existing GCS root ending `msh161-primary-v1` remains a storage namespace;
new experiment IDs are `msh161-primary-v2-batch-NN`. No IAM/network changes occur.

The original `main-freeze.json` and frozen helpers remain unchanged. The adapter
loads each original helper afresh and overrides only the emitted claim policy
when a valid, hash-bound amendment is present. Original jobs retain their original
semantics. Source/build/image references make the amended source stratum explicit.

`policy-strata.json` identifies the12earlier optional-claim draws and preserves
native conclusions/intended-but-unplayed claims. All40earlier games may be affected
by changed search assumptions, not only those draws. Do not replay or pool the
strata as one uniform draw/conversion measurement. External Oxidation/Stockfish
internal claim assumptions are unchanged; reference evaluations near optional
claim thresholds remain interpretation-limited and cannot establish an automatic
controller outcome. Retain unresolved earlier/linked events and correlated-event
limits. No final engineering priorities follow from this partial study.

Linux build succeeded from reviewed native commit `ab7ef8f10d9b9023dbe525fcbc47e54a40c423e0`; allfourbinary hashes and wrapper exit/notification were checked. See `linux-build-verification.json`. Image deployment and cloud validation remain pending. Do not launch batch03 until Linux provenance, short match
validation and final manifest/image admission are recorded. Long build and cloud
operations use the existing exit notifier; no periodic watcher or guessed wake.


Current execution: native build and wrapper succeeded; reviewed worker source is
`d9271a294cb92e1e87d366d49dfd409bbfd662b6`. `deployment-launch.json` records the
single running image-build/push then cloud-validation chain (wrapper PID96307).
It queues this chat on success or failure. Do not start another build, validation,
completion consumer, or batch03 while it runs. An older native-build completion
message is already reconciled and must not restart this chain.

On deployment exit, inspect `image-build-state.json`, `validation-state.json`,
`deployment-notification.json`, exact saved submission/cloud completion and bounded
validation artifacts. Check settled process state and cloud cleanup separately;
retain VM allocation failures and actual resource accounting. If image construction
fails before validation, the validation state may not exist: inspect image state
and wrapper stderr instead. Preserve all partial artifacts before a deliberate retry.

After success: verify staged batch03–10 identities against `amendment.json`, the
new registry digest, package receipt and original frozen schedule/spec hashes.
Preserve original unrun manifest bytes from staging/originals. Activate the amended
manifests in the existing primary locations only after this admission, record
new/old mappings and source/policy stratum, and submit batch03 under the same
8-worker C3 configuration. Use the existing wait/retrieve helper with exit notifier.
No new approval is needed. Keep the complete200game study and review gates; no
other engine engineering is authorized. Cleanup the temporary native source clone
once no build/process needs it, preserving reusable binaries/cache and receipts.
