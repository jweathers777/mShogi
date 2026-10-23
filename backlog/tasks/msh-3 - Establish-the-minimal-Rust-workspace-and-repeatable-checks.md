---
id: MSH-3
title: Establish the minimal Rust workspace and repeatable checks
status: Done
assignee:
  - '@codex'
created_date: '2026-10-22T13:26:00Z'
updated_date: '2026-10-23T11:45:00Z'
labels:
  - rust
  - build
  - ci
milestone: m-1
dependencies:
  - MSH-1
documentation:
  - docs/design.md
  - AGENTS.md
  - docs/development.md
  - legacy/rust-prototype/PRESERVATION.md
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Make a clean checkout build and run focused verification without a GUI or training environment.

## Design references

P-03; P-05; A-01; M1

## Scope

Create a small reusable Rust library and console binary, record a stable toolchain and dependency policy, define formatting/lint/test commands, and configure appropriate Windows/macOS/Linux CI. Add useful scaffolding only; no empty future subsystems.

## Verification expectations

Run the documented checks on the available host and inspect CI results for each claimed platform. Record unavailable platforms honestly. Confirm the library has no frontend, console-I/O, or Python dependency.

## Out of scope

Tauri/React scaffolding, database setup, Python training environment, generic plugin systems, and engine search.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A reusable library and headless console binary build from documented clean-checkout instructions.
- [x] #2 The stable Rust toolchain, dependency/lockfile policy, format, lint, and test commands are recorded and reproducible.
- [x] #3 CI defines relevant checks for Windows, macOS, and Linux; actual results are distinguished from configuration alone.
- [x] #4 Core library dependency boundaries are preserved and no speculative frontend, training, or plugin framework is introduced.
- [x] #5 README commands reflect operations that actually exist and have been verified on the stated environment.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Claim MSH-3 on clean codex/rewrite, inspect completed dependency and existing Rust prototype, current toolchain and GitHub Actions access.
2. Preserve the prototype's manifest, lockfile, source, variants and original README byte-for-byte under legacy/rust-prototype outside the active workspace; document concrete incompatibilities rather than repairing or certifying its game rules.
3. Establish a minimal two-crate Rust workspace (core library plus headless console), pin stable Rust 1.89.0 with rustfmt/clippy, and add only useful metadata/help/version scaffolding with focused console integration checks. Keep core free of console I/O and external dependencies.
4. Document clean-checkout build/run/format/lint/test commands, lockfile/dependency policy and preservation decision. Configure a pinned checkout action and native Windows/macOS/Linux CI with the same checks.
5. Run focused host and clean-checkout verification, obtain independent review, resolve findings, commit/push normally, inspect actual CI results per platform and record exact revision/evidence. Close MSH-3 only; report MSH-4 readiness.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup: codex/rewrite clean at 99adfa8fc580a46e0fd1f5b927ea336390ea1568, tracking origin/codex/rewrite; remote tip confirmed equal. Reuse attached isolated worktree /Users/johnweathers/.codex/worktrees/rewrite-bootstrap/mShogi. master remains 941cd4d and main_v2 retains local WIP fd280df (remote 9045843); original master checkout clean. MSH-3 is sole Ready+ready task; no In Progress owner. MSH-1 dependency is Done. @codex owns MSH-3 only. Read AGENTS/design/dependency/current task plus prototype source, manifest, lockfile and MSH-2 handoff. Pinned Backlog.md 1.55.1 used; global CLI unchanged; overview previously read this conversation and task-execution guide refreshed.

Prototype inspection: src/game/mod.rs has an unmatched closing brace; Variant::load_from_file constructs Variant without the added rules field; board methods print directly and main suppresses all warnings. Its architecture/rules are not adopted as the new conformance baseline. Initial cargo test --locked --offline --target-dir /private/tmp/mshogi-msh3-prototype-target failed before compiling because equivalent 1.0.1 was not cached; this is not a passed build nor a demonstrated compiler result. Preserve the prototype intact and separate it from the small library/console workspace rather than silently correcting/changing historical rule behavior.

Installed stable currently resolves to rustc 1.89.0 (29483883e 2025-08-04), cargo 1.89.0 (c24e10642 2025-06-23), aarch64-apple-darwin. Pin exact stable release 1.89.0 for reproducibility rather than silently floating with the stable alias. This is not a claim to use the newest Rust. GitHub CLI auth initially failed inside network sandbox; verified successfully with normal network/keychain access. Configured repository is public, push permission available, default branch master. Do not change repository/default-branch settings.

Implementation candidate: preserved all 19 original prototype files (manifest/lock, source, variants and README) byte-for-byte under legacy/rust-prototype, comparing each with 99adfa8. Root BSD-LICENSE, original branch refs and historical commits retained. Active Cargo workspace excludes the legacy directory. Root manifest/lock now contain only mshogi-core and mshogi-console. Library exports the already documented target-profile ID only; all process/I/O handling is in the small console, with no third-party active dependencies or speculative rule/API framework. publish=false preserves deferred release choices. Exact stable toolchain 1.89.0 with rustfmt/clippy installed without changing global default.

Baseline result correction: network-enabled cargo test --locked --target-dir /private/tmp/mshogi-msh3-prototype-target downloaded existing locked dependencies and failed with actual E0063 at variant.rs:118 (missing rules field). The earlier unmatched-brace observation was a misreading at a combined command-output boundary, not a verified source defect; it is withdrawn. No prototype source was changed. Initial new-workspace fmt --check found only formatting in the console integration test; cargo fmt --all corrected it, then checks passed.

Focused host checks on candidate (base99adfa8 plus MSH-3 diff): cargo fmt --all -- --check; cargo clippy --workspace --all-targets --locked -- -D warnings; cargo test --workspace --locked (5 real subprocess tests passed on macOS, zero artificial library/unit/doc tests); cargo build --workspace --locked; all three cargo run --locked --bin mshogi -- --help / --version / profile commands passed. CLI errors return2 with empty stdout/diagnostic stderr; headless tests run outside checkout with stdin closed, and Unix non-Unicode input is rejected without panic. Metadata/tree show exactly two active packages and no core dependencies. Relative docs links and git diff --check pass. No game/rules/parser/undo/search behavior, strength or GUI conformance is claimed.

Added docs/development.md with native prerequisites, pin/override guidance, clean-checkout commands, lockfile/dependency policy, preservation rationale and CI evidence distinction; README now reflects actual scaffold. Core boundary and preserved snapshot verified programmatically. Workflow config uses native ubuntu-24.04/macos-15/windows-2022, same format/lint/test/build/smoke commands, fail-fast false, read-only contents, and official checkout v7.0.1 pinned to3d3c42e5aac5ba805825da76410c181273ba90b1 (tag/runtime fetched from official GitHub API; Node24). Pushes to codex/** and PRs into codex/rewrite trigger; backlog-only updates skip unnecessary reruns. CI is not yet claimed passed. Separate independent reviewer is examining bounded diff; clean remote checkout and actual CI evidence will follow the reviewed integration commit.

Independent review: separate read-only reviewer /root/msh3_review found no actionable findings. Read all scoped source/manifests/lock/toolchain/workflow and docs against AGENTS/design/task. Independently compared all 19 archived files with99adfa8 (byte-identical), ran cargo test --workspace --locked --offline --target-dir /private/tmp/msh3-review-target (all5 macOS subprocess tests passed), formatting check, Clippy all-targets -D warnings, dependency tree and diff checks. Thirteen additional real-process CLI argument/default/alias/error probes from temporary cwd with closed stdin passed. Reviewer confirmed core dependency/I/O boundary, proportionate scaffolding, pinned native CI and honest scope limits. Review does not substitute for committed clean-checkout or Windows/Linux CI evidence; those remain pending. GitHub Actions availability inspected read-only: enabled, allowed_actions all; no settings changed.

Committed/clean-checkout evidence, 2026-10-23 Europe/Paris: implementation commit 0dfb0259fc67b8e71f5b65b4a44e78d9c369e69d (Establish minimal Rust workspace and native CI (MSH-3)) integrated on codex/rewrite; git push origin codex/rewrite succeeded (99adfa8..0dfb025). Fresh remote clone: git clone --single-branch --branch codex/rewrite git@github.com:jweathers777/mShogi.git /private/tmp/mshogi-msh3-clean-checkout. HEAD confirmed0dfb025; rustup show confirmed repository pin1.89.0 active while global stable stayed default. With a fresh target directory, all documented cargo build --workspace --locked, three run/help/version/profile commands, fmt --all -- --check, clippy --workspace --all-targets --locked -- -D warnings, test --workspace --locked (5 subprocess tests) and dependency tree passed. Clean-checkout git status --porcelain was empty after checks. Host macOS27.0.1/Darwin27.0.0 arm64; rustc1.89.0 (29483883e), cargo1.89.0 (c24e10642). This supersedes the earlier pending clean-checkout statement.

Actual native CI evidence: https://github.com/jweathers777/mShogi/actions/runs/39948111367, push event, head_sha0dfb0259fc67b8e71f5b65b4a44e78d9c369e69d, completed/success. Inspected workflow result, every job/step result, and compiler/test/runner log output using gh api .../actions/runs/39948111367, .../jobs and gh run view39948111367 --repo jweathers777/mShogi --log. Local raw log is temporary, not committed; linked GitHub run is durable evidence.

- Linux: job120395650091, label ubuntu-24.04/image ubuntu-24.04, x86_64-unknown-linux-gnu. Format, Clippy, tests (5 integration tests), build and all3 console smoke commands succeeded.
- macOS: job120395650509, label macos-15/image macos-15-arm64, aarch64-apple-darwin. Same checks succeeded,5 integration tests.
- Windows: job120395650609, label/image windows-2022, x86_64-pc-windows-msvc. Same checks succeeded,4 integration tests; Unix non-Unicode argument case correctly inapplicable.

All three logs report rustc1.89.0 (29483883e2025-08-04), cargo1.89.0 (c24e106422025-06-23), Actions runner2.338.0. These are native executions, not cross-compilation or inferred OS support. Evidence establishes only current workspace/CLI scaffold behavior on those recorded hosts, not full rules, other architectures, packaged desktop, strength or M1 closure. No missing platform result remains for MSH-3. All independent review findings resolved (none raised).

AC mapping: #1 reusable library and headless binary built from fresh remote clone; #2 exact stable pin, committed active lockfile, dependency policy and verified documented commands; #3 native three-platform CI configured and each actual outcome inspected; #4 no core external dependencies/I/O, no frontend/Python/framework,19 prototype files preserved byte-identical; #5 README commands exist and pass host/clean checkout/native CI. Implementation is already integrated and pushed; this ticket-only completion record is a follow-up commit on the same branch. The workflow intentionally skips backlog-only pushes, so code revision0dfb025 remains the applicable tested revision. No invented future commit hash or default-branch/access/settings change.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed MSH-3 only. Established a minimal dependency-free core library plus headless console workspace, exact stable Rust1.89.0 toolchain/components, inherited safety/documentation lints, committed active lockfile, meaningful subprocess checks and native CI. Console exposes help/version/target-profile metadata and accurately states that rules are not implemented. Documented clean-checkout commands and boundaries in README/docs/development.md. Preserved all19 prototype files byte-for-byte under excluded legacy/rust-prototype rather than adopting its unfinished rules/console coupling; existing master/main_v2 history and notices remain intact.

Verified on host and fresh remote checkout: format, Clippy -D warnings, test, build, CLI smoke and dependency boundary. Independent reviewer found no actionable issues and reproduced checks plus13 extra CLI probes. Native CI run39948111367 succeeded at implementation revision0dfb0259fc67b8e71f5b65b4a44e78d9c369e69d on Linux x86_64, macOS arm64 and Windows MSVC x86_64 (5/5/4 integration tests respectively). Actual evidence is linked above; no game/rules or packaged desktop certification is claimed.

Implementation commit0dfb0259fc67b8e71f5b65b4a44e78d9c369e69d is normally pushed on origin/codex/rewrite; the commit containing this completion record carries the final evidence. No blocker remains. Next eligible task is MSH-4 (explicit chess position state and FEN boundary), unstarted; both MSH-2 and MSH-3 dependencies are now complete.
<!-- SECTION:FINAL_SUMMARY:END -->
