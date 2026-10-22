# mShogi agent instructions

These instructions apply to Codex, Claude Code, and bounded review or QA sessions. The repository must be sufficient to resume work without private conversation history.

## Authority and scope

1. Follow the current user's instructions and explicitly recorded owner decisions.
2. Preserve the accepted product requirements and architectural boundaries in `docs/design.md`.
3. Follow the selected task's scope and acceptance criteria within that design.
4. Treat tool guidance, examples, older source, and implementation conventions as subordinate to the above.

Ordinary implementation choices belong to the implementer. A ticket, implementation plan, ADR, or tool-recommended review checkpoint does not by itself require another permission request. When the user asks to work on a task or the next available task, carry that bounded work through implementation, appropriate verification, review, and normal integration. Ask only about a material product/rules ambiguity or an action outside existing authorization. Continue unaffected work when possible.

Never change a game rule, remove a promised variant, weaken acceptance criteria, or reinterpret a public semantic contract merely to make code or tests pass. Record the issue and bring a concrete proposed change with evidence back to John.

## Session startup

- Inspect Git status, branch, remotes, worktrees, recent relevant commits, and active ownership before editing. Preserve existing dirty and untracked work.
- Read `docs/design.md`, the selected task, its dependencies, and the relevant implementation. Read only additional material needed for the work.
- Run `backlog instructions overview` and the relevant creation, execution, or finalization guide when using that operation.
- Check the installed Backlog.md version against the recorded project baseline. Resolve a compatibility difference before mutating records with an incompatible version.
- Resume your own active task when appropriate. Otherwise select a dependency-ready task from `backlog task list --status Ready --ready --plain`, respecting the user's stated priority.
- A session must not claim another active worker's task. Record the actual owner and branch/worktree before substantive implementation.

## One bounded work item

Use one primary builder responsible for integration. Delegate independent research, review, or clearly isolated implementation when useful; do not run competing builders over the same files or task.

Before coding, inspect the code and put a short current implementation plan in the active task. Future tasks contain intent, constraints, acceptance criteria, dependencies, and verification expectations, not speculative detailed implementation plans.

Work on one bounded task per session by default. An explicitly requested queue or batch may continue through multiple tasks while preserving a separate plan, review, evidence, and completion record for each. Record newly discovered work as separate tasks when it falls outside the active scope. Do not silently expand the task.

## Backlog.md usage

- Use the supported Backlog.md CLI for task creation and updates. MCP may be used if already configured and compatible; it is not a requirement for this project.
- Keep status, ownership, dependencies, acceptance criteria, implementation notes, and final summaries in the task records.
- Use `--append-notes` to preserve prior findings. Use `--notes` only when intentionally replacing notes.
- Editing dependencies sets the intended list; pass the complete dependency set, not just the newest dependency.
- Keep custom project instructions outside Backlog.md's managed instruction block.
- Do not create GitHub Issues as an additional task system. A PR may carry a concise summary and link to a repository ticket.
- Preserve IDs and evidence. Archive completed or cancelled work deliberately; do not delete task history as routine cleanup.

The configured statuses are `To Do`, `Ready`, `In Progress`, `In Review`, and `Done`, with `Done` last.

- `To Do` means the task still needs sufficient scoping or resolution of a recorded blocker.
- `Ready` means the task has actionable scope; dependencies may still prevent execution.
- `In Progress` means a named worker owns the active work.
- `In Review` means implementation evidence is ready for independent assessment.
- `Done` means the acceptance criteria and applicable completion checklist are satisfied, review findings are resolved, and the result is integrated into the recorded rewrite branch.

If blocked, record the specific blocker and the next action, leave the task unfinished, and take other authorized work if available. Do not treat a dependency-ready task as proof that a material unresolved design question has been answered.

## Verification and review

Write meaningful correctness tests with the behavior they exercise. Use independently established expected results; tests that merely repeat the implementation's assumptions are insufficient for rules conformance.

Distinguish rule correctness, search correctness, execution performance, playing strength, and application integration. Passing one category does not establish the others.

Run the smallest checks that address the changed behavior and concrete risks. Broaden checks for shared-boundary changes or an integration milestone. Do not run an expensive full platform or end-to-end suite repeatedly for every small task when current evidence already covers unchanged behavior.

Use an independent reviewer or fresh review session for meaningful behavior changes and milestone closure. Give the reviewer a bounded diff or question, relevant requirements, and existing evidence. The author rereading their own work is useful but is not independent review. Documentation-only and mechanical changes need proportionate review, not invented test suites.

Record the actual commands, outcomes, environment, and code revision used for verification. Distinguish passed, failed, and not run. Never infer success on an operating system from a build on another operating system.

Backlog.md allows a task to be marked Done with unchecked criteria. Project policy does not. Check only criteria supported by evidence; leave the task unfinished if required verification is unavailable.

## Git and routine autonomy

The established workflow permits ordinary local commits, non-destructive integration, normal pushes to the configured project remote, and cleanup of the agent's own completed disposable branches/worktrees. Use the designated rewrite branch as the integration target. Keep the original project history and default branch intact unless John explicitly changes that instruction.

Do not force-push, rewrite published history, discard unrelated edits, delete ambiguous worktrees, bypass hooks or protections, change repository access, or switch the remote/default branch as routine task work. Inspect existing state before integration. Resolve ordinary conflicts within scope; escalate a conflict in product intent with a concrete explanation.

Prefer coherent commits that include code, associated tests, and ticket evidence. Do not put credentials, local databases, large generated training sets, private machine settings, or build output into Git. Record reproducible references for large evidence instead.

## Completion and handoff

The active task's final summary must state what changed, why, acceptance/evidence coverage, important decisions or limitations, and linked follow-up work. Record the integration branch and relevant commit references without fabricating a future commit hash.

Before yielding, leave enough information in the task for a new session to continue. Report the completed task, meaningful verification, remaining blocker if any, and next eligible task. Do not maintain a second competing status spreadsheet or long chronological diary.

## Engineering defaults

- Keep rules and search independent of UI frameworks, console I/O, and Python.
- Use stable Rust for product code; separately scoped fuzzing tools may require a different recorded toolchain.
- Start with safe, understandable code and a representation appropriate to the implemented game. Add unsafe code, SIMD, or representation compression only for a measured need and with corresponding checks.
- Treat the old implementation as historical evidence and a source of regression ideas, not as the rules oracle.
- Preserve all six target games in the design. Chess-first sequencing does not authorize a chess-only product.
- Build the minimum implementation that satisfies the active contract. Do not prebuild a universal variant DSL, plugin ABI, distributed trainer, or speculative abstraction framework.

<!-- BACKLOG.MD GUIDELINES START -->
<!-- backlog.md-instructions-version: 1.55.1 -->
<CRITICAL_INSTRUCTION>

## Backlog.md Workflow

This project uses Backlog.md for task and project management.

**At the beginning of each conversation in this project, run `backlog instructions overview` before answering or taking action. Re-read it only if you have not read it yet in the current conversation.**

Use the overview to decide whether to search, read, create, or update Backlog tasks.

Before task lifecycle actions, read the matching detailed guide:
- `backlog instructions task-creation` before creating or splitting tasks
- `backlog instructions task-execution` before planning, changing status or assignee, adding a plan or implementation notes, or implementing task work
- `backlog instructions task-finalization` before checking acceptance criteria, writing final summaries, or moving tasks to terminal statuses

Use `backlog <command> --help` before running unfamiliar commands. Help shows options, fields, and examples.

Do not edit Backlog task, draft, document, decision, or milestone markdown files directly. Use the `backlog` CLI so metadata, relationships, and history stay consistent.

</CRITICAL_INSTRUCTION>
<!-- BACKLOG.MD GUIDELINES END -->
