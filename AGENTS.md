# mShogi agent instructions

These instructions apply to Codex, Claude Code, and bounded review or QA sessions. The repository must be sufficient to resume work without private conversation history.

## Authority and scope

1. Follow the current user's instructions and explicitly recorded owner decisions.
2. Preserve the accepted product requirements and architectural boundaries in `docs/design.md`.
3. Follow the selected task's scope and acceptance criteria within that design.
4. Treat tool guidance, examples, older source, and implementation conventions as subordinate to the above.

Ordinary implementation choices belong to the implementer. A ticket, implementation plan, ADR, or tool-recommended review checkpoint does not by itself require another permission request. When the user asks to work on a task or the next available task, carry that bounded work through implementation, appropriate verification, review, and normal integration. Ask only about a material product/rules ambiguity or an action outside existing authorization. Continue unaffected work when possible.

Never change a game rule, remove a promised variant, weaken acceptance criteria, or reinterpret a public semantic contract merely to make code or tests pass. Record the issue and bring a concrete proposed change with evidence back to John.

## Communication preference

Owner preference, 2026-11-11: Use ASD-STE100 Simplified Technical English in all future communications with John.

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
- Structure new and active task descriptions as **Problem**, **Objective**, **Approach**, **Completion**. Keep them concise and understandable on their own; link detailed findings, conditions and receipts instead of accumulating chronology in the description. Preserve historical records and stable handles.
- Use `--append-notes` to preserve prior findings. Use `--notes` only when intentionally replacing notes.
- Editing dependencies sets the intended list; pass the complete dependency set, not just the newest dependency.
- Keep custom project instructions outside Backlog.md's managed instruction block.
- Do not create GitHub Issues as an additional task system. A PR may carry a concise summary and link to a repository ticket.
- Preserve IDs and evidence. Archive completed or cancelled work deliberately; do not delete task history as routine cleanup.
- Give every new task a concise, stable, descriptive handle (preferably two to five words). Record `Handle: ...` in its description and a `handle:<kebab-case>` label through the CLI. For useful existing tasks, append `Stable task handle: ...` without rewriting historical records. The handle remains stable if the longer title changes.
- In new conversational reports, development summaries, plans and task references, write `MSH-138 (Desktop Search Limits)`, `MSH-137 (Chess Search Refinement)` or `MSH-49.2 (Stockfish Baseline)`: always ID plus handle. CLI arguments, filenames, machine identities and the managed examples below retain their native syntax. Do not rewrite old evidence just to retrofit handles.

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

Follow [risk-based verification](docs/testing/verification-levels.md), the durable owner policy of 2026-11-09:

- **Level 1 — routine local (default):** platform-independent search/evaluation, pruning, ordering, algorithmic optimization and isolated native correctness. Run relevant local tests, independent regressions, profiling and review; broaden local coverage for shared rules/consumers. Applicable local checks and independent review permit completion and integration. Production Rust changes alone do not require three-platform CI.
- **Level 2 — targeted platforms:** material portability, process supervision, concurrency, filesystem/persistence, protocol, dependency/toolchain or platform-sensitive application changes. Select relevant operating systems and the smallest sufficient native/consumer suites from the actual risk; no automatic unrelated packaging or GUI campaign.
- **Level 3 — full integration/release:** significant integration milestones, major architectural migrations, release candidates and releases. Establish actual supported-platform native and desktop compatibility on Linux, macOS and Windows; reuse earlier valid source/scope evidence honestly.

Record the level, reason, actual environment/source/checks and explicitly deferred broader compatibility checkpoint in the active task. A Level 1 task is not blocked on unrelated hosted-runner capacity. Reconcile obsolete blanket platform obligations explicitly in unfinished task records; preserve historical failures and completed acceptance. Never present deferred or unperformed checks as passed. Ordinary verification selection needs no owner approval.

Use an independent reviewer or fresh review session for meaningful behavior changes and milestone closure. Give the reviewer a bounded diff or question, relevant requirements, and existing evidence. The author rereading their own work is useful but is not independent review. Documentation-only and mechanical changes need proportionate review, not invented test suites.

Record the actual commands, outcomes, environment, and code revision used for verification. Distinguish passed, failed, and not run. Never infer success on an operating system from a build on another operating system.

Backlog.md allows a task to be marked Done with unchecked criteria. Project policy does not. Check only criteria supported by evidence; leave the task unfinished if required verification is unavailable.

Conformance must follow `docs/testing/conformance-policy.md`: selected rules and
independently established expectations, meaningful property/metamorphic/negative
checks and independent review. External game engines are optional investigations,
never rules oracles or ordinary build/test/CI/conformance dependencies. An explicit
owner-approved playing-strength qualification may require independent opponents
for that measurement gate; it does not make them product or rules dependencies. Preserve
historical observations without requiring another engine for future variants.
Owner-requested opponent strength comparisons, such as MSH-49.2 (Stockfish Baseline), are separate
measurement tasks; they do not make an external engine a normal product, rules
conformance or CI dependency.

For engine-strength development, implement researched conventional techniques in
coherent increments using correctness/tactical tests, bounded profiling and small
development matches. Routine local engine-match experiments have a **15-minute
cumulative compute allowance per task** unless John explicitly authorizes more;
record aggregate active run time including failures and concurrent runs. Do not
reset the allowance by relabeling experiments or artificially splitting tasks.
Do not run multi-hour benchmarks, substantial statistical campaigns or formal
strength studies without explicit owner approval. Those studies belong at a
substantially complete conventional engine for a game, or a substantially complete
new evaluation approach integrated into it, not at individual optimizations or
small groups of changes. Routine algorithm choices remain the implementer's job.
Follow the owner-selected sequence in `docs/m5/strength-roadmap.md`.

Owner revision, 2026-11-09: ordinary desktop Chess is time-primary. MSH-139
(Chess Strength Assessment) is explicitly authorized to discover current strength
against full-strength independently rated engines under comparable thinking time.
There is no approved strength target, fixed tournament size or campaign-hour
allocation. Adapt opponents and fresh paired confirmation to observed scores and
uncertainty; use established statistics and stop when findings are useful for the
next development decision. Track actual resource use, bound individual processes
and artifacts, and escalate genuinely substantial additional resource needs.
This authorization supersedes the unapproved qualification proposal and the
routine 15-minute match allowance for this assessment only. Report findings and
STOP for John's review before selecting a target, further engine improvements or
Shogi/Xiangqi transfer (assessment-specific stopping point, subsequently satisfied). Completed engineering and historical evidence remain valid;
learned evaluation stays later. Player-configurable time is separate future work.

Owner continuation, 2026-11-09 after MSH-139 (Chess Strength Assessment):
continue conventional Chess engineering autonomously, beginning with the
completed-depth1/295113-visit incident in the Goldfish loss. Establish the work
and root cause before optimization, correct general defects with independent
regressions, then pursue substantial search/frontier/native-query/TT and classical
endgame/evaluation improvements in coherent tasks. No position special cases,
concealed limits, learned evaluation or arbitrary rating target. Routine match
allowance remains15minutes cumulatively per task; no substantial tournament after
each increment. Reassessment belongs after meaningful progress, with fresh
openings/adaptive references and proportionate approved resource scope. Shogi and
Xiangqi remain deferred until explicit owner approval. Routine engineering choices
do not require owner checkpoints.

Owner course-correction after MSH-144 (Chess Progress Assessment), 2026-11-09:
first diagnose broad conventional Chess effectiveness before automatically taking
MSH-145 (Pawn Threat and Exchange Judgment). Use representative independent tactical,
positional and endgame positions, preserved full histories, increasing thinking
time (for example 1/5/15/30 seconds), controlled search comparisons and profiling.
Distinguish overhead, useful search depth, frontier/selectivity/ordering/TT behavior,
evaluation quality and their interactions. Prioritize coherent general improvements
from findings, not ticket order or isolated historical failures. Later adaptive
strength reassessment follows a substantial coherent round, not each optimization.
No learned evaluation or Shogi/Xiangqi transfer without owner approval, and no
conventional-maturity claim from algorithm presence or speed alone.

Owner refinement after MSH-151 (Chess Coherent-Round Assessment), 2026-11-09:
prioritize systematic decision-quality diagnosis and coherent corrections. Define
measurable tactical/endgame/continuation outcomes and stopping criteria for bounded
investigations. Separate evaluation, horizon, selectivity and execution costs using
comparable conditions; seeing a continuation in a PV does not prove its consequences
were correctly searched or valued. Do not automatically repeat small adaptive
rating studies after MSH-152 (Classical Tactical Forecasting) or later increments.
When meaningful decision improvements justify strength measurement, recommend a
controlled earlier/current-version comparison with matched openings, conditions
and opponents where appropriate, and an appropriate sample for the question.
Substantial tournaments and formal studies require approval before execution;
routine bounded development experiments retain the existing resource policy.
No learned evaluation or Shogi/Xiangqi transfer is authorized.

For macOS GUI verification, reuse the fixed normal/QA/synthetic application slots
and identities documented in `docs/desktop/packaged-gui-verification.md`, using
the helper in the primary integration checkout even for worktree builds. Never
create a bundle identifier, app name or launch directory per fixture, task or
revision. Change a QA scenario through the external fixture selector while its
application is stopped; do not rewrite or re-sign the bundle per scenario.
Inspect existing saved approvals read-only when diagnosing access. Never edit
approval stores or security policy, self-grant access, or promise that an app
rebuild cannot trigger a platform-controlled prompt. Record actual identity,
signature and reuse observations.

## Execution efficiency within an assignment

Apply the owner-approved improvements in
`docs/review/tori-codex-execution-bottlenecks.md` without weakening correctness,
independent review, acceptance criteria or required platform evidence.

- Before broad tests or CI, inspect affected production consumers and separate
  workspaces. Check tagged dispatch exhaustiveness, the standalone Tauri host,
  relevant frontend consumers and changed test assertions early.
- Use focused checks during implementation, then broad relevant regressions at
  a coherent source checkpoint. Minor changes need only their affected checks
  unless they invalidate the broader evidence.
- Separate mandatory successful representative resource journeys from near-limit
  operations whose contract permits precise typed refusal. Always assert accepted
  state, history, identities, dirty state and prior-file integrity as applicable.
- Start independent review early when useful; re-review substantive changes and
  final source evidence while other required work proceeds.
- Declare the task's verification level and any risk-required CI jobs explicitly.
  Level 1 local checks/review can complete ordinary native engineering; preserve
  targeted Level 2 and full Level 3 obligations where their actual risks apply.
  Full packaged GUI campaigns belong at integration/release boundaries.
- Use one completion-aware CI monitor. Stop when the required condition is met;
  check available completion before another timed wait. Cancel superseded
  campaigns when appropriate and preserve their failures and provenance.
- Maintain one structured verification record per tested source revision and
  reference it from concise Backlog notes and acceptance summaries. Reuse prior
  records rather than copying their results into competing evidence ledgers.
- Before completion, establish that the exact tested revision passed its required
  checks. Reuse unchanged source/scope evidence; never infer unperformed checks.

## Current conventional Chess round

Owner authorization, 2026-11-10 after MSH-155 (Chess Time Scaling): establish
MSH-156 (Independent Chess Benchmark) from verified public sources before
substantial engine changes, then carry out MSH-157 (Conventional Decision
Improvement) autonomously as one coherent round. Use a few hundred diverse
positions, separate grouped development/held-out collections and frozen
expectations; the prior eight roots are regression-only. Five seconds is primary,
one second secondary, fifteen seconds focused. Protect heldout positions from
implementation decisions and promote/replace any exposed family before reuse.
Coordinate exact native execution improvements with controlled continuation
coverage/classical-judgment diagnosis. Preserve one rules/history/outcome authority;
no wholesale rewrite is presumed. MSH-153 (Classical Compensation Calibration)
remains held unless broader evidence selects valuation work. Compare substantial
stages to a frozen predecessor and report improvements, regressions and unresolved
judgments separately. Ordinary local engineering and related task transitions are
authorized; escalate major architecture, substantial resources and product choices.
No learned evaluation, Shogi/Xiangqi transfer or substantial tournament is
authorized. Eventual strength qualification needs separately approved clock-based
increment games and suitable independent opponents. Stop the round to reconsider
its diagnosis if useful benefits do not materialize.

Round outcome, 2026-11-10: MSH-156 (Independent Chess Benchmark) and MSH-157
(Conventional Decision Improvement) completed the authorized coherent round.
Exact native cost repairs were useful, but the broad decision-improvement gate
failed: 238/240 primary moves and all 80 held-out moves were unchanged. See
`docs/m5/chess-decision-round.md` and the revised roadmap. Preserve the frozen
corpus plus its conservative prior-exposure overlay and do not retune against
revealed held-out material. MSH-153 (Classical Compensation Calibration) remains
held; no competitive campaign is justified by this round's evidence. Revisit the
continuation-quality diagnosis before selecting further implementation work.

Owner correction, 2026-11-10: MSH-158 (Chess Decision Corrections) is the current
assignment. Improve actual conventional Chess decisions, beginning with development
roots 0UOyM, 0m5mI and 28pJK plus successes. Trace real interior search admission,
windows, TT, reductions/frontier and returned values; separate child searches are
not evidence of interior behavior. Test focused general search/evaluation hypotheses
without requiring perfect causal proof; retain only supported corrections. Do not
substitute native/history optimization, infrastructure, new benchmark construction
or another general assessment. Use the existing 160/80 split, five seconds primary,
selected fifteen seconds and fast regression checks; seal heldout until candidate
and procedure freeze, then no retuning. Reuse valid predecessor measurements. New
active engine computation is capped at 2400 seconds, including failures; no matches.
Stop after reasonable distinct experiments if none supports improvement, with the
specific unresolved obstacle. MSH-153 remains held unless valuation is implicated.
Ordinary implementation/review/integration within this assignment is authorized.

Owner direction after MSH-158 (Chess Decision Corrections), 2026-11-10:
investigate and improve classical evaluation in MSH-159 (Classical Evaluation
Decisions). Establish meaningful intermediate-position rankings and consequences,
not Stockfish centipawn calibration. Evaluation-only experiments are authorized;
do not change search policy to force preferred moves. Reuse the existing160/80
benchmark, keep held-out sealed until freeze, retain5s primary/1s checks and selected15s
diagnostics. New active engine computation is capped at2400seconds; no matches,
new corpus, learned evaluation, cross-game work or unrelated optimization. Use
independent diagnosis/correction review and integrate supported changes normally.
Stop after reasonable distinct trials if unsupported; MSH-153 remains held unless
a relevant general valuation problem is established. Full owner request is retained
in docs/m5/evidence/msh159/owner-request.txt.

Owner direction, 2026-11-10: MSH-161 (Competitive Gameplay Diagnosis) is the
current assignment. Study approximately200 complete games first against one fixed
suitable stronger reference with equal practical thinking time (initial5s/move),
diverse master-supported paired openings plus separate unaided starts. Analyze
every move of both players with fixed Stockfish, confirm consequential events,
review pilot and completed batches, and rank observed competitive weaknesses
before hypothesizing mechanisms. Integrate MSH-160 opening questions; no competing
corpus. The full request in docs/m5/evidence/msh161/owner-request.txt authorizes the
broad study and necessary analysis beyond routine aggregate compute/match caps;
retain process bounds, failures, reproducibility and actual resource monitoring.
Use unattended software for games/reference analysis; agents inspect completed
batches, not individual moves or repeated unchanged polling. No production changes,
rating tournament, learned/cross-game work or speculative corrections. Stop for
John's review after independently reviewed diagnosis and updated roadmap.
Owner execution refinement,2026-11-11: computation wrappers must queue a message
on exit(success or failure) with `codex queue --thread THREAD_UUID --message TEXT`
to resume the owning idle chat. Do not use periodic watchers or guessed wake times.
Use normal execution escalation for Codex local database access; preserve all
security settings. Record computation status and notification acceptance separately.

Owner cloud-migration direction,2026-11-11: stop further local MSH-161 batches.
Preserve the active batch02; if the frozen runner cannot pause at a completed-pair
boundary, allow that existing batch to finish safely. Do NOT launch local batch03,
repeat searches or resume local execution from an older exit-notification message.
Prepare an interim competitive assessment from sufficiently analyzed Mac evidence;
no final engineering priorities or production corrections. The primary200-game
study should replay ALL frozen opening pairs on standardized cloud workers after
the coordinator reviews the separate cloud runner's validation. Mac evidence stays
exploratory and separate, not pooled as one uniform hardware sample. The coordinator
owns MSH-161/codex/rewrite and scientific interpretation; the cloud-runner agent owns
its execution implementation on an isolated worktree/branch. Coordinate ONLY through
repository records, not inter-chat messages. See the migration request and handoff
under docs/m5/evidence/msh161/migration; preserve frozen live inputs unchanged.


## Git and routine autonomy

The established workflow permits ordinary local commits, non-destructive integration, normal pushes to the configured project remote, and cleanup of the agent's own completed disposable branches/worktrees. Use the designated rewrite branch as the integration target. Keep the original project history and default branch intact unless John explicitly changes that instruction.

Do not force-push, rewrite published history, discard unrelated edits, delete ambiguous worktrees, bypass hooks or protections, change repository access, or switch the remote/default branch as routine task work. Inspect existing state before integration. Resolve ordinary conflicts within scope; escalate a conflict in product intent with a concrete explanation.

Prefer coherent commits that include code, associated tests, and ticket evidence. Do not put credentials, local databases, large generated training sets, private machine settings, or build output into Git. Record reproducible references for large evidence instead.

## Checkout and worktree lifecycle

Keep `codex/rewrite` checked out in the primary project directory. Use that
checkout for integration and normal application launches.
Keep the original `master` branch and history; a separate permanent checkout
is not required to preserve them.

Create a temporary worktree when a bounded task needs isolation or parallel
work. After the task is verified, independently reviewed where applicable,
integrated and pushed, remove its worktree before finishing. First confirm
that its commits are integrated, preserve any uncommitted or needed untracked
or ignored files, and check that no running task or process still needs it.
Archive app-managed worktrees with the app tool; remove ordinary Git worktrees
with Git. Leave a worktree only when it has an explicit ongoing purpose, and
record that purpose in the handoff. Reusable build caches may live in the main
checkout; completed task checkouts are not permanent build locations.

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
