# MSH-125 independent corrected-source native CI disposition

Reviewer `/root/workflow125_review`; read-only diagnosis, not implementation of MSH-112. Final production source `58a5d7ad95cba1724118a91d8ad4958fb1c287ed`, native run41214429663 Linux job124575870483 attempt1. Read actual log `/private/tmp/msh125-native-corrected-linux-failure.log`, SHA256 `a03442b92bacb3d6d435faa880cd6effc6fba3b0b62f016646a8dbbd18bf4552`, current MSH-112, prior MSH-124 independent disposition and unchanged runner test/source.

## Exact failure

Workspace runner test `process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored` failed at `crates/mshogi-runner/tests/runner.rs:172`. Mode=exit expected UnexpectedExit, actual MissingResponse; engine0, phase handshake, at_ns2065295, watchdog100ms, detail Broken pipe(os error32), exitNone, no output. Desktop-session200 tests already passed on this Linux job (192.00 seconds). Workspace runner9 PASS/1FAIL; failed job cannot establish later allfeatures/fixture steps as passed.

## Existing ownership and source causality

The exact signature matches current **MSH-112 To Do/unstarted** and preserved MSH-119/MSH-124 recurrences. Native runner write-error handling (`run.rs` around296–316) probes process exit once; only an observed nonzero exit is UnexpectedExit; an early write error with no status is MissingResponse. `process.rs:observe_exit` uses nonblocking try_wait. This is the already-recorded early-write versus process-status observation ordering, not the separate Shogi retired-duplicate/ResourceLimit race.

`git diff 07c0be6..58a5d7a -- crates/mshogi-runner crates/mshogi-engine crates/mshogi-core crates/mshogi-search Cargo.toml Cargo.lock` is empty. MSH-125 changes native desktop session/application workflow but does not change the subprocess runner, fixture, search, rules or workspace dependencies implicated here. The observation is a concrete documented unchanged runner-race recurrence, not evidence requiring weakening the MSH-125 browse/close contracts.

## Verification and honest disposition

Independently ran on local macOS: `cargo test --locked -p mshogi-runner --test runner process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored -- --exact`: 1 PASS, 9 filtered, 0.66 seconds. Local success is not Linux CI success and not a fix.

A normal failed-job retry on identical source after matrix completion is proportionate. Preserve attempt1 log and exact failure; record retry result separately. Do not change expected category to accept either outcome, extend timeouts, alter source or describe a successful retry as fixing MSH-112. MSH-112 remains To Do with its own focused diagnostic ownership. MSH-125 final CI criterion remains pending until actual required retry disposition; this report does not waive a check or mark task Done.

## Actual retry dispatch

After the original matrix completed, independently preserved full run/jobs/step metadata at `/private/tmp/msh125-native-attempt1-metadata.json`: attempt1 failure solely Linux runner Test; macOS and Windows all steps succeeded, including isolated QA constructors, native fixture verification, build and console smoke.

Executed exactly `gh run rerun 41214429663 --failed` (exit0) on 2026-10-31 around15:33UTC, ordinary authorized failed-job retry. Confirmed attempt2 has unchanged source `58a5d7ad95cba1724118a91d8ad4958fb1c287ed`, Linux retry job124578865084 running, retained Mac/Windows success. No source, test assertion, timeout or resource limit changed. Attempt2 actual completion pending; no fix claim.

## Actual unchanged-source retry completion

Native41214429663 **attempt2 completed SUCCESS** at exact58a5d7ad95cba1724118a91d8ad4958fb1c287ed. Independently fetched full final metadata and Linux job124578865084 log: original workspace runner assertion passed, desktop-session200 PASS/251.90s, allfeatures212 PASS/366.80s, fixture generation/comparison, build and console steps succeeded. Mac/Windows retained their original successful dispositions. Final metadata `/private/tmp/msh125-native-attempt2-final-metadata.json` SHA256 `1357b5d24260fea3b9deaa074ec192671eed7a836c8afcb2670df71170f7b24a`; Linux passing log `/private/tmp/msh125-native-linux-attempt2-passed.log` SHA256 `8bc67727bed70ca26a29b7f054cdc2ab1eeabb26387e75bd54b886f22eb469a9`. Attempt1 metadata SHA256 `6a77f9388a1e6f4b9898b78ae574d7b3552dc01cde5105fa73150602bf35af6e` retained alongside failed log.

Successful identical-source retry is valid CI execution evidence, not a fix or waiver of MSH-112. No assertion, source or timeout changed. MSH-112 remains unresolved with its established independent ownership.

Separately checked `git diff --exit-code 58a5d7a..8de41e4 -- crates desktop/src desktop/src-tauri Cargo.toml Cargo.lock .github/workflows/rust.yml`: empty. 8de41e4 adds only verification driver/actual diagnostic fixture/tests and task notes. Its automatically triggered native41215847540 remains separately pending at this checkpoint; no implicit green claim from source equivalence.

## Final verification-checkpoint native CI

Independently fetched native run41215847540: **attempt1 completed SUCCESS on all three platforms** at `8de41e46c892613c0a44d0d3991ffda0f88e8b5e`. Mac job124580079926, Linux124580080092, Windows124580080144 all complete success, including workspace tests, isolated QA constructors, native snapshot fixture verification, build and console checks. Full exact metadata `/private/tmp/msh125-native-final8de-metadata.json`, SHA256 `6ca5783e49f06610ee7130ec63facb7194a9b6fd83a0f86a8199736710053985`. This final native outcome supersedes pending status only; original58 Linux attempt1 MSH-112 failure remains historical and unresolved, not fixed by this task.
