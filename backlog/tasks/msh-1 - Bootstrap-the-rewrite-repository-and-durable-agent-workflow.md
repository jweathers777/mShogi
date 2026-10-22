---
id: MSH-1
title: Bootstrap the rewrite repository and durable agent workflow
status: Done
assignee:
  - '@codex'
created_date: '2026-10-22T13:26:00Z'
updated_date: '2026-10-22T22:49:00Z'
labels:
  - workflow
  - bootstrap
milestone: m-0
dependencies: []
documentation:
  - docs/design.md
  - AGENTS.md
  - START-HERE.md
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Create the real repository baseline from this reviewed handoff package so a fresh Codex or Claude Code session can continue without chat history.

## Design references

P-06; M0; D-03 through D-06

## Scope

Inspect the existing repository and worktrees; establish or reconcile an isolated rewrite branch; preserve legacy history and unrelated edits; import design and native Backlog records; validate the shared agent entry point; commit and normally push the dedicated branch when configured access is available.

## Verification expectations

Follow START-HERE.md. Record actual repository/branch state, CLI version, task and milestone reads, dependency-ready selection, doctor output, preserved instructions, and integration evidence. A package validation report is not a substitute for checking the real import.

## Out of scope

Rust product code, engine implementation, GUI scaffolding, default-branch changes, force pushes, and discarding existing work.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Actual repository, remote, legacy reference, rewrite branch/worktree, and existing-work handling are recorded without overwriting unrelated work.
- [x] #2 The design, README, shared AGENTS instructions, and one-line CLAUDE import are present and coherent in the target rewrite.
- [x] #3 Backlog.md reads the complete seed queue and milestones, doctor reports no unresolved structural issue, and dependency readiness is verified.
- [x] #4 No implementation task or criterion is marked complete solely because its record was imported; any seed-ID collision was reconciled with dependencies intact.
- [x] #5 The baseline is committed, normal push outcome is honestly recorded, and the next eligible tasks and any genuine access blocker are reported.
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
1. Inspect repository ownership, refs, worktrees and existing rewrite; preserve master and main_v2 including the local WIP commit.
2. Branch codex/rewrite from main_v2 in an isolated managed worktree; import the non-colliding starter, preserving existing source, notices and shared agent policy.
3. Reconcile README claims with the retained prototype; verify archive parity, CLI 1.55.1, all 14 tasks/7 milestones, board, doctor, dependencies and unchanged implementation.
4. Perform proportionate independent review, resolve findings, commit and normally push only codex/rewrite; record actual integration evidence and next ready tasks in MSH-1.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Repository inspection (2026-10-23 Europe/Paris): origin fetch/push git@github.com:jweathers777/mShogi.git. Original worktree /Users/johnweathers/Development/Personal/mShogi was clean on master, legacy/reference HEAD 941cd4dc37e6e6210d4f993c96a553753e228b19 (also origin/master and origin/HEAD). Initially one worktree, no attached app artifacts or active task ledger/policy. Existing main_v2 fd280dff02fa7ef73809e20b28bdec83fa9c955f is one local WIP commit ahead of origin/main_v2 90458437d8872d6264b5536ae6ab15142c11e357; inspected its manifest, main, rules/state/move types, README and recent history. Remote heads independently confirmed by git ls-remote --heads origin after retrying outside the network sandbox: master and main_v2 only.

Ownership: @codex owns MSH-1 only, branch codex/rewrite, isolated managed worktree /Users/johnweathers/.codex/worktrees/rewrite-bootstrap/mShogi. This is the designated rewrite integration branch. Existing rewrite takes precedence over an orphan import: branch directly from main_v2, retaining all its tracked content and ancestry; master/main_v2 refs remain unchanged. No orphan/reset/clean/deletion, remote/default-branch change or forced push.

Imported all 28 archive files into previously absent paths, with collision guards. Archive SHA-256 1c9431d527cf0530a81530c37f2f577e5f094cb7c64b2802ef9df0adb1ded532. No seed IDs collided; no task dependencies remapped. Existing README, BSD-LICENSE, Cargo files, src and variants retained. The starter README.md needs its initial empty-tree claim reconciled with the existing prototype; this does not certify that prototype against M1.

CLI: global backlog --version = 1.55.0; no global upgrade. Downloaded exact backlog.md@1.55.1 using npm exec --cache /private/tmp/mshogi-bootstrap-npm-cache --yes --package backlog.md@1.55.1 -- backlog --version (1.55.1). All import verification and mutations use that pinned CLI. Read its overview/execution guidance and edit help. Shared managed instruction block version remains 1.55.1, CLAUDE.md remains @AGENTS.md. Initial task list: 14 Ready tasks, zero completed acceptance criteria, only MSH-1 selected by --status Ready --ready; milestone list: 7, none complete. First doctor invocation hit sandbox EPERM while creating backlog/drafts; retry with worktree filesystem access is required, not treated as a passed check.

Verification of the imported candidate (2026-10-23 Europe/Paris), revision context: HEAD/base main_v2 fd280dff02fa7ef73809e20b28bdec83fa9c955f plus the 28-file bootstrap candidate; no product-source change. Environment: macOS 27.0.1 build 26A434, Darwin 27.0.0 arm64, Git 2.55.0, Python 3.14.7, explicit NVM Node v23.11.0 / npm 10.9.2, native Backlog.md 1.55.1.

Passed: backlog task list --json and all 14 backlog task view MSH-N --json reads; 7 milestone list --plain entries; initial Ready+ready = MSH-1 only; after ownership/In Progress, Ready+ready = empty. Every imported task initially had all AC/DoD unchecked. Only MSH-1 is being completed; MSH-2..14 retain seed status, identities, dependencies, and unchecked criteria. Confirmed MSH-13 depends on MSH-5 and MSH-14 waits on MSH-11/12/13. Parsed documentation targets and relative Markdown links resolve. No seed-ID collisions, no remapping.

Passed after filesystem-access retry: backlog doctor -> "No duplicate IDs, self-referential dependencies, or dependency cycles found." backlog board view shows MSH-1 In Progress/@codex and 13 Ready tasks, no Done tasks. This supersedes the earlier sandbox EPERM. Board timestamp uses the host timezone; this evidence date uses the requested Europe/Paris context.

Python zipfile/subprocess comparisons: 28 imports, 26 byte-identical to the archive; only README.md and CLI-updated MSH-1 intentionally differ. All 21 tracked main_v2 files are byte-identical, including source, manifest/lock, variants, original README, .gitignore and BSD-LICENSE. An earlier successful comparison printed a hard-coded count of 23; the actual enumerated count is 21, confirmed by rerun and reviewer. Shared AGENTS.md including 1.55.1 managed block is unchanged; CLAUDE.md is exactly @AGENTS.md plus newline. README.md now accurately describes the retained prototype, integration branch, pinned CLI and MSH-3 reconciliation; this supersedes the earlier pending README note. No design/acceptance weakening.

Environment-only failure: an unqualified node --version from Python resolved to broken Homebrew Node 26.9.0 (missing libsimdjson.33.dylib) after the substantive assertions passed. Explicit NVM Node v23.11.0/npm 10.9.2 reads pass; downloaded native pinned Backlog runs successfully without that Homebrew binary. No global runtime changes were made. Product builds, cargo tests, Windows/Linux execution, and rules/search/GUI verification were not run or claimed for this documentation/mechanical bootstrap; preserved prototype reconciliation belongs to MSH-3 and later tasks.

Independent review: separate bounded read-only reviewer /root/bootstrap_review compared the imported candidate with the archive and main_v2, inspected policy/design/task/README and existing refs/worktree. No actionable preservation, import or scope findings. Reviewer independently confirmed 26 exact imports, all 21 preserved base files, unchanged MSH-2..14 and milestones, coherent shared policy and readiness. Review limits: product behavior, future commits and push not assessed; author verified doctor and will verify integration. Review follow-up notes requesting current README/doctor outcomes are resolved here. Documentation/mechanical scope received proportionate review; no product milestone review is implied. Finalization guidance read.

Next dependency-ready tasks after MSH-1 reaches Done: MSH-2 (M1 chess rules profile and independent fixture corpus) and MSH-3 (minimal Rust workspace and repeatable checks). Neither is started. Preserve/reconcile the existing prototype when executing MSH-3 rather than assuming an empty repository.

Integration evidence: baseline commit d876a6190f5296f8a20b3e4ea529ec84d4211eef (Bootstrap rewrite design and native Backlog workflow (MSH-1)) is integrated on codex/rewrite, descendant of the preserved fd280df WIP and legacy 941cd4d. git diff --cached --check passed before commit; baseline includes 28 newly imported files and no changes to the 21 pre-existing files. git push -u origin codex/rewrite succeeded (new branch), with upstream origin/codex/rewrite established. git ls-remote --heads origin refs/heads/codex/rewrite refs/heads/master refs/heads/main_v2 confirmed rewrite = d876a6190f5296f8a20b3e4ea529ec84d4211eef, master = 941cd4dc37e6e6210d4f993c96a553753e228b19 and main_v2 = 90458437d8872d6264b5536ae6ab15142c11e357. No remote/default-branch changes, force-push, or remaining access blocker. This ticket completion/evidence is a separate follow-up commit on the same branch, identified by the commit containing this summary rather than an invented self-referential future hash.

Final closure check: backlog task list --status Ready --ready --json actually returns exactly MSH-2 and MSH-3 after MSH-1 is Done. doctor passes again; milestone list shows M0 1/1 done and M1 0/13 done. Completion evidence commit will be pushed normally to the established origin/codex/rewrite upstream; baseline push success above is already verified.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed MSH-1 only. Imported the reviewed design and native Backlog seed, shared AGENTS.md/CLAUDE.md policy, and handoff documents into codex/rewrite without overwriting the existing main_v2 Rust experiment. README.md now records the actual integration branch and prototype status. Preserved master, main_v2 (including local WIP), source, notices and history; chose an existing-rewrite descendant rather than an orphan tree.

Verified with pinned Backlog.md 1.55.1: 14 native task reads, 7 milestones, board, doctor, dependency integrity/readiness, archive parity, documentation links, exact shared policy import, and byte-identical existing files. Independent bounded review found no actionable issues. Baseline d876a6190f5296f8a20b3e4ea529ec84d4211eef was committed, normally pushed to origin/codex/rewrite, and confirmed with ls-remote. The commit containing this completion record carries the final MSH-1 evidence on the same integration branch.

Only MSH-1 acceptance/completion items are checked. No product builds, conformance or platform coverage is claimed. MSH-2 and MSH-3 are next eligible, unstarted; MSH-3 must reconcile the preserved prototype with the new workspace/check contract. No genuine access blocker remains.
<!-- SECTION:FINAL_SUMMARY:END -->
