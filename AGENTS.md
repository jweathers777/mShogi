# mShogi agent instructions

These instructions apply to every coding agent (Claude Code, Codex and others).
The repository and its Backlog are the complete project state. A new session
must be able to start the next task without chat history and without help.

## The project

- mShogi is a native Rust engine and desktop application for six games: Chess,
  Standard Shogi, Xiangqi, Tori Shogi, Chu Shogi and Tenjiku Shogi. It is not
  released. John is the sole owner.
- Product requirements: `docs/design.md`. Game behavior: the native rules in
  `mshogi-core`, following the selected rules profiles in `docs/rules/`.
- Never change a game rule, drop a game, or loosen a rules test to make code
  pass. Write the problem and a proposed decision in the task and ask John.

## Start a session

1. Run `git status`, `git branch`, `git worktree list`. Do not touch another
   worker's branch, worktree or uncommitted work.
2. Choose the work, in this order:
   - the task John named;
   - your own task that is In Progress;
   - the first task from `backlog task list --status Ready --ready --plain`
     that has no assignee and no `needs-john` label. Highest priority first.
3. Claim it before you change code: set yourself as assignee, set the status to
   In Progress, create the branch and push it. Skip any task that another worker
   claimed.
4. Read the task, the code that it changes, and the parts of `docs/design.md`
   that apply. Also read every document that John, the task or the design
   tells you to read. You do not have to read all of `docs/`.

## Do the work

- Branch from `main` as `<agent-name>/<task-id>-<short-name>`, for example
  `claude/msh-205-tori-selectivity` or `codex/msh-205-tori-selectivity`. A
  small documentation change can go directly on `main`.
- Make engineering decisions yourself. Use measurements to decide. Do not write
  option papers or wait for John on an engineering choice.
- Ask John only about: game rules or product behavior, spending above the cap
  below, destructive Git operations (history rewrite, force-push, deletion of a
  branch that is not yours), and work outside the task.
- How to ask John:
  - If John is active in your conversation, ask him there. Record the question
    and his answer in the task notes.
  - If he is not active, or does not answer: write the question and your
    recommendation in the task notes, add the label `needs-john`, push, and
    send a notification if your environment can. Then continue with other
    work.
- If you find work outside the task, create a new task. Do not expand the
  current task.

## Engineering quality

- Do not repeat yourself. This applies to all code, not only to code shared
  between games. When two places do the same thing, put it in one function,
  type or generic implementation behind a trait. When you touch duplicated
  code, prefer to merge it.
- Use judgment. Do not force an abstraction that makes a hot path slower or the
  code harder to read. Search code can keep a specialized form when a
  measurement shows that it matters.
- Write code that is consistent with the code around it: naming, module
  layout, error handling, test style and documentation comments. Read the
  nearby code before you write. Do not copy a pattern that this file says to
  remove, such as duplicated per-game modules.
- Build the simplest thing that works. Internal APIs have no compatibility duty.
  Saved-game formats keep their recorded compatibility.

## Checks

Every change:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Also, by type of change:

- **Rules (`mshogi-core`):** perft and tests whose expected results come from
  the rules, not from running the code. External engines are not rules oracles.
- **Search-side positions:** perft and differential tests against the native
  rules.
- **Engine strength:** the blunder suite and an SPRT match against the frozen
  predecessor at a fast time control. This match has standing authorization.
  Keep a change only when it wins, or when it is neutral and makes the code
  simpler. Faster search is not stronger play. Node counts and PVs do not have
  to stay the same.
- **Platform-sensitive code** (processes, files and persistence, Tauri,
  toolchain, dependencies): run CI on the affected operating systems. Full
  three-platform and packaged GUI checks are for releases only.
- **Review:** rules changes, saved-format changes and large refactors get one
  review by a fresh agent session or subagent, given the diff and the task. Fix
  the real findings. Other changes need no separate review.

Report results honestly: say what passed, what failed and what did not run.

## Heavy compute and run data

- John's Mac is not for heavy compute. A job that uses many cores for more than
  a few minutes runs on Google Cloud Batch through `tools/experiments/gcp.py`,
  with one machine type per comparison. Check that the VMs are deleted after.
- Spending cap: up to US$10 per task without asking. Ask John before more.
- Never commit run output: game records, logs, JSON reports, screenshots,
  archives. Write it under `runs/` (ignored by Git). Upload what can help a
  later diagnosis to `gs://mshogi-experiments-216675633801/runs/<task-id>/`.
  A lifecycle rule deletes objects under `runs/` after 60 days. Delete local
  run data when the task is integrated.
- In the task notes, record a short result: the commits compared, the command,
  the machine, the result (for example SPRT wins/losses/draws and Elo
  estimate), the cost and the GCS path.

## Finish

1. Merge `main` into the branch, run the checks again, then merge the branch
   into `main` and push.
2. Check the acceptance criteria that have evidence. Write a final summary of a
   few lines: what changed, why, results, follow-up tasks. Set the status to
   Done.
3. Clean up everything that you created: your feature branch, your worktree
   with its build output, temporary files, local run data and local caches.
   Disk space must not grow silently. Keep temporary files in one place that
   you can delete (`runs/` or your tool's scratch directory).

If you stop before the task is done, for any reason, write a handoff note:
what is done, what is next, and the branch name. Push the branch and leave the
task In Progress. A stop can come without warning (for example when your
context or usage capacity runs out), so update the handoff note at each
checkpoint, not only at the end.

## Tasks

- A new task has a title, a goal of one to three sentences, and acceptance
  criteria that say when it is done. Add notes only when they help the next
  session.
- Add a dependency only on a task that directly blocks it.
- Refer to a task by ID and title, for example "MSH-205 (Tori Selectivity
  Round)".

## Documentation

- `docs/` holds durable knowledge: the design, rules profiles, how-to guides,
  and a short design note when a technique is worth explaining (for example how
  the Lion or the Fire Demon is generated).
- `docs/` does not hold task reports, verification records, reviews of
  reviews, or run data.
- Only John can change the process. Do not change this file on your own
  authority. Propose a change to John; when he approves it, edit the rule in
  place. Do not add dated direction blocks. Git keeps the history.

## Engine strength program

Each game follows the same steps: strength harness, selectivity round,
evaluation round, reference-ladder anchor, blunder screen, then a stop
decision. The target is competent play without obvious blunders, from
classical techniques, as a baseline for later learned evaluation. See
`docs/m5/strength-roadmap.md`. Games after Xiangqi use no external opponents.
Texel tuning of evaluation weights is allowed. Neural-network evaluation is
future work and is out of scope now. Do not start it without John's approval.

## Code defaults

- Stable Rust. No `unsafe` in product code.
- Rules and search do not depend on UI, console I/O or Python.
- All six games stay in the design.

## Communication

- Use ASD-STE100 Simplified Technical English.
- End a session with: what changed, check results, and the next task.

<!-- BACKLOG.MD GUIDELINES START -->
<!-- backlog.md-instructions-version: 1.55.3 -->
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
