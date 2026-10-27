# MSH-95 completed-boundary independent review

Verdict: **PASS, no material unresolved finding** for the bounded routing-only implementation inspected. This does not approve Xiangqi execution, verifier105, evaluator/search97, desktop play/archive or milestone closure. Documentation/evidence consistency is reviewed separately when final draft exists; final integrated CI and commit verification remain integrator responsibilities.

## Actual review method and scope

Separate read-only Codex agent `/root/routing95_review`, independent of primary source builder `/root` and isolated console-test builder. I inspected baseline0a90ec8e8669b5dff70f37b8ceaec48d1702be1b through the current uncommitted bounded diff, task95 AC/DoD and explicit owner scope correction; read native envelope, native Xiangqi Game/record, shared request/result/server/wire and bridge, desktop new/replacement/archive admission and concrete canonical consumers. I examined every new focused test and test-only process-fixture change, including expected fixture facts and actual subprocess invocation. No product/task/repository documentation was edited by me. This is independent model/code/test review, not a human rules or GUI review.

## Findings resolved by the delivered design

The initial audit's concrete extension hazards are addressed:

- Core dispatch is explicit Xiangqi game/profile/edition/native-schema, using the unchanged strict outer RawValue envelope and selected authoritative native parser. No map erases malformed outer or nested input ahead of rejection, and no game fallback exists.
- Cached event count, known board-action count and native side derive solely from authoritative Game during validated record construction; private immutable fields cannot drift from canonical JSON. Reopening still establishes fresh runtime identities, while canonical equality remains semantic.
- Xiangqi ordinary settings are opaque u8/u64 wire values, not borrowed Chess/Shogi algorithm capabilities. Even zero/max values receive explicit unsupported output without work. Actual native side, complete event prefix/ply, purpose, target/session/selection and every echoed context/limit field remain exact. Chess-specific Full accepts only its bounded shape and sentinel, then explicitly rejects Xiangqi capability; it never runs Xiangqi verification or Chess reachability.
- Native command dispatch returns the Xiangqi Submission, preserving accepted versus rejected-natural-count audit result and undo; rules-owned actor admission remains authoritative. Nonturn Black resignation and claim admission work, wrong-turn board action and cross-game commands refuse atomically.
- Empty Xiangqi capabilities and explicit shared server branch prevent accidental Chess fallback. Existing native conclusions and unknown/fast-proven no-win assessments remain record data; unsupported execution never manufactures action/score/claim/conclusion.
- No bridge/wire/process/queue/search/rules changes were needed. Existing one-owner complete-context matching, cancellation, bounded reads/diagnostics, cleanup and monotonic request/process retirement continue across games and undo/reopen/historical reselection.
- Xiangqi desktop New and Replace are refused before capacity/admission/retirement mutations. Red/Black labels are exhaustive qualified-side compatibility arms only; no Xiangqi board/session/presentation/archive/analysis consumer was introduced. Current supported document, token, dirty state and pending work remain unchanged.
- Existing envelope1/shared3/legacy2 wire identities and native Chess/Shogi records remain unchanged; tests exercise actual supported child entry points. Native1MiB/envelope1MiB/request1MiB/response2MiB caps were retained, with actual composed-byte and exact-boundary evidence rather than constant increases.

## Independent focused execution

Commands executed locally against the reviewed production files:

- `cargo test --locked -p mshogi-core --test native_envelope --test native_xiangqi_envelope`: **10 PASS**.
- `cargo test --locked -p mshogi-engine --test shared --test xiangqi_routing`: **11 PASS**.
- `cargo test --locked -p mshogi-console --test xiangqi_internal --test internal --test shared_internal --test shogi_engine`: **16 PASS**; actual normal `mshogi internal-search` and separately labeled test-fixture subprocesses, including unsupported Xiangqi requests, real Chess/Shogi execution, forged exact echoes, cross-game replacement/cancellation, semantic undo/reopen/reselection, timeout and cleanup.
- `cargo test --locked -p mshogi-engine --test shogi_protocol`: **6 PASS**.
- `cargo test --locked -p mshogi-desktop-session xiangqi_tags_do_not_enable_desktop_creation_replacement_or_retire_live_work`: **1 PASS** (85 unrelated tests filtered).

Total at this point: **44 PASS**. The new maximum command-ledger test used all512 actual commands/1024events, retained full canonical payload and echoed response, accepted exact1MiB outer/request bytes and rejected +1. Test data uses native reviewed MSH94 histories rather than reference-engine adjudication as an oracle. Tests making malformed JSON inject duplicate raw members directly rather than first normalizing through a map.

An initial console command incorrectly named `shogi_protocol` in the console package and failed target selection before running that console selection; it was corrected to console `shogi_engine` plus the actual engine `shogi_protocol` command above. An initial desktop filter `unsupported_xiangqi` matched zero tests and provides no verification; the exact named1test command above corrected it. These attempts are not counted as passed evidence.

## Frozen production file hashes (SHA-256)

- `crates/mshogi-core/src/native.rs`: ffad877c7d36d05904632ff690d1d993645d5de10c75b009104dfc23c1d48d33
- `crates/mshogi-core/src/xiangqi/record.rs`: d44868f9ba7d6a254d40ea174c54e340e0bae9e6977c4a59b09819e2167005ca
- `crates/mshogi-engine/src/shared.rs`: db5b2c82c5a037211627f11954d82bf5f1a7331d30070bc2c946ce64bdc50f1f
- `crates/mshogi-engine/src/server.rs`: 0ecb90da29aa33f893a530719d067bd56cb6d60906a92c5e5a8544096ec4407b
- `crates/mshogi-desktop-session/src/shared.rs`: 620a5cbdda567ec7053ce7e659a111d3ec04aeabe51b91681122fcb6364ed64f
- `crates/mshogi-desktop-session/src/shared_archive.rs`: 681067a8addf86005016645ea704660438549a06d25bf16cb4dfd0913b817445

## Limits of the verdict

This review establishes routing/API/strict-parsing/process-boundary compatibility for the task's source changes. It does not independently re-prove WXF rule correctness from sources, measure playing strength, prove exact no-win feasibility, test an unsupported Xiangqi GUI, or establish another operating system from macOS local results. Final native CI/platform outcomes and integrated revisions must be recorded as actual results. The existing MSH95 scope/dependency plan remains intact;105 becomes dependency-ready only after95 is actually Done, and97 remains after its recorded native handoff prerequisites.

Additional independent command: `cargo test --locked -p mshogi-engine --lib cancel_and_replace_discard_even_an_already_published_completion` **1 PASS** (two unrelated tests filtered), confirming cancellation and replacement retire already published completions. **Final independent focused total:45 PASS**. Local environment: macOS27.0.1 build26A434, rustc1.89.0 (29483883e2025-08-04), cargo1.89.0 (c24e106422025-06-23).

## Documentary follow-up review

Verdict: **PASS**, no material overbroad scope, compatibility or evidence claim identified in the final draft inspected. Read `docs/m4/xiangqi/shared-routing.md`, `routing-evidence.md`, and the current-status diffs in `docs/m4/shared-context.md`, Xiangqi README/handoff/coverage. Confirmed the documented profile edition against the native constant, fixed wire/schema identities against production, opaque ordinary settings and bounded Full-shaped unsupported behavior, actual metadata/data-versus-execution separation, native command audit semantics, atomic desktop refusal and whole-encoded caps against the reviewed source/tests. The retained earlier narrative is explicitly historical, with accepted Shogi delivery and current Xiangqi status distinguished.

Verified the existing graph directly:95 depends94/69;105 depends94/95;84 depends80/91/92/93/94/95/105;96 and97 depend84/95. Documents correctly preserve this split and state105 becomes dependency-ready only after95Done, not automatically started. They do not assert a G-NOWIN operational algorithm/policy or general exact proof/feasibility result from routing. M4/45 remain open.

The local full-workspace478, all-features desktop93 and frontend161 results in routing-evidence are **integrator-reported actual execution**, not rerun independently by this reviewer; the document correctly separates them from my45focused independent tests and from future actual CI/platform evidence. Final tested/integrated source and native/package CI URLs are intentionally deferred to the task completion summary rather than fabricated. No new GUI/platform interaction is claimed.

All6 reviewed production SHA256 values were recomputed and unchanged. The added reverse-parser assertions (Chess and Shogi wrappers containing genuine Xiangqi payloads) were inspected and independently rerun with `cargo test --locked -p mshogi-core --test native_xiangqi_envelope raw_outer_and_nested_rejections_never_fall_through_to_another_game`: **1PASS**,4filtered. This reruns one existing test with two additional negative cases and is not counted as an extra distinct test beyond the45 total. No product edits were needed or made by reviewer.
