# Independent deployment script review

Reviewer: execution_review, 2026-11-12. Bounded read-only review of `build_worker_image.py` and `run_cloud_validation.py`; no build, cloud call, engine execution or submission. This review does not establish Linux runtime success.

The proposed route is appropriately scoped: a clean source-commit identity is checked against packaged Python/study files; native binary hashes are checked by the existing minimal packager; unchanged opponent/reference binaries are reused; only the existing registry is used. Corrected manifests are staged, not activated. The validation request retains the existing service account/network and uses one C3 task, a 600-second limit and zero retries. The inherited container entrypoint is `python3`, so the direct validation helper command is correctly formed. Completion uses the existing Pub/Sub consumer, with no repeated status polling or automatic resubmission. Failure exceptions and retrieved artifacts remain evidence, and the eight intentionally capped played plies are validation-only, not scored study games.

Initial review requested these bounded fixes before admission:

1. Include Dockerfile and requirements.txt in the worker source-commit byte comparison, since package() copies them separately from source_files.
2. Bind the terminal resource name and UID to the saved submission response; the existing wait routine binds its own initial lookup, which alone does not prove submission identity.
3. Require the core receipt/build/report artifacts and match the returned helper/build hashes against the intended package/build receipts. Self-consistent downloaded files alone do not establish the intended packaged identity.
4. A Docker build/push CLI timeout must retain an explicit reconciliation-required state: daemon-owned work may outlive the CLI. Do not claim cleanup or automatically retry.

Final recheck: all four requested changes are present. **No deployment-script blocker remains in the reviewed source.** Packaging additionally checks Dockerfile/requirements.txt against the selected commit. Validation checks submission/terminal UID and name, requires unique core artifact names, verifies downloaded file hashes and binds returned build/helper/report identities to the intended local build/package records. Image failures retain an explicit reconciliation-required marker; no daemon cleanup or automatic retry is claimed.

Final reviewed SHA256 values:

- build_worker_image.py: `5ece0655a90eb7b1e1044d4eab2c3f6e1a589f21fcb7584ec6f4f693a72ad82e`
- run_cloud_validation.py: `ee73d15b88d646e0ed3acbc1ff04f45f9b67c7182153b18798b7cd612cbfbdea`

Both files passed offline Python AST parsing. Execution was not performed by this reviewer. Exact resource-name comparison is conservative: a project-ID/number alias variation between submission and terminal responses could cause a false refusal, while preserving evidence; it cannot accept the wrong UID. If encountered, reconcile using the existing alias-aware resource matcher and exact UID, without resubmission. Failures before cloud setup may leave the exclusive local state at starting, which prevents silent retry and requires inspection of the outer process receipt.

The native helper's accepted initial-start event adjustment is separately reviewed by its owner. Final worker commit/image must include its stable bytes; neither syntax checks nor this admission substitute for the single authorized Linux runtime validation.
