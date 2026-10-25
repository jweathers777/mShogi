# M3 completion batch evidence

M2 remains closed. MSH-27–35 and roadmap correction MSH-40 retain their
existing evidence. MSH-61 precedes MSH-36–39. M4–M6 are future work; this batch
does not implement them. M3 closure requires every acceptance criterion,
including actual Windows, macOS and Linux packaged GUI interactions.

## MSH-61: complete archive capacity

Native admission checks an immutable complete candidate before accepting an
ordinary document command or engine/analysis response. Separate bounded workers
perform document admission and filesystem operations. Runtime resource pauses
have no chess result. The current accepted document remains saveable, with
reserved stop/close and identity headroom. The format limits and archive schema
are unchanged. Saving and reopening retain all observations and canonical
history.

Independent review identified concurrent Save As bookkeeping replacement and
near-limit persisted identity exhaustion. Both were fixed with regression
coverage: document identity preserves the newer successful Save As metadata,
and persisted archive revision is separate from runtime-only snapshot progress.
Near-exhausted Open inputs are rejected before replacement, while ordinary
mutation refusal retains the save path. Host receipts identify the exact accepted command
even when a following engine-result admission advances the current admission.

Focused verification distinguishes native admission/controller tests, actual
console-child process tests, frontend component tests and packaged GUI evidence.
The first packaged macOS pass used the separate `mShogi Verification.app`,
fixture `capacity`: 255 native zero-node incomplete observations, then a real
bundled depth-1/20,000-node analysis accepted as observation 256 (21 visited
nodes). The next Start was refused before execution, keeping 256 observations
and zero moves. Save As succeeded; Open retained all 256 historical observations
with play stopped. Another Start was refused again. Explicit New game cleared
the old document from the live view; native Cmd-Q displayed the dirty-document
guard and Discard closed it. This first pass precedes the final identity fix;
final revision/package evidence is recorded separately below when available.

The application notice preserves Save and Save As, explains resource capacity
separately from illegal moves, failures and outcomes, and directs the user to
save then explicitly start another document. It does not suggest that a retry
or reopening replenishes structural capacity.

![Actual macOS capacity notice after successful complete Save As](evidence/m3-completion/capacity-limit-macos.png)

The temporary saved archive contained 256 observations and 256 controls,
385,760 bytes, SHA-256
`d88d54d9469a565d0a71cbe4ddd3a1f92962e181665a6d86f7bf21da34ab39f3`.
Local environment: macOS 27.0.1, arm64. The archive is test output, not a normal
package resource.

Final normal-package verification reopened that complete archive without QA
initializers, refused another analysis as an application resource limit and
successfully saved the entire document again. The final archive retains 256
observations/256 controls; historical flags explain the 256-byte reduction.
Package/engine hashes and final archive hash are in
[capacity-local-macos.json](evidence/m3-completion/capacity-local-macos.json).
The normal package inventory, UCI, engine-turn, move-analysis and bounded Full
protocol checks passed; fixture and native Cocoa probe markers are absent.

![Final normal package: complete save after capacity refusal](evidence/m3-completion/capacity-normal-macos.png)

Focused commands on the final source:

- `cargo test -p mshogi-desktop-session --lib --features desktop-verification`:
  **41 passed**, including 256/257 engine and analysis observations, actual
  serialized byte growth using legal Full witness evidence, 4,096/4,097 events,
  control/close reservations, cancellation, unchanged rejection, Save As races,
  reopening and identity headroom.
- `cargo test -p mshogi-console --test desktop_play --test desktop_analysis --test desktop_persistence`:
  **9 passed**, using actual native child processes.
- `cargo clippy -p mshogi-desktop-session --all-targets --features desktop-verification -- -D warnings`:
  passed. Repeated generation of 16 ordinary snapshots plus the isolated
  capacity snapshot was byte-identical.
- `npm test`: **56 passed**; strict TypeScript/Vite production build passed.
- `npm run desktop:build -- --bundles app`: normal macOS package passed.

Independent reviewer `capacity_review` examined the native/host/frontend delta
against `7c7df45`, reran 40 normal-feature native and 9 process tests, and
reproduced the formerly dangerous identity boundary: 17 legal Preferences
commands from a near-limit input, then capacity refusal, then successful Save.
No remaining material findings. This source review is separate from the actual
packaged GUI observations above. Hard runtime identity lifetime remains an
explicit guard; no archive caps were raised and no canonical evidence removed.

The exact normal GUI source is `3b427d49fb481c9391992d40f047941ae7f445c2`.
The host's two close/file serialization tests and both workspace formatting
checks also passed. Initial native CI passed on macOS and Windows; Linux found
a timing assumption in the new receipt/close test. The public command API polls
before returning, so a tiny candidate may already have been accepted. The test
now waits for completed validation without polling admission, then verifies
close supersedes that still-uncommitted candidate. This changes test setup only.
Independent review ran the corrected exact test plus **50/50 repetitions**.
The product GUI evidence remains applicable; replacement CI is recorded below.
