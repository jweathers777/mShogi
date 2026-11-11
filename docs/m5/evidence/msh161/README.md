# MSH-161 (Competitive Gameplay Diagnosis): active study handoff

The complete study is **not finished**. No competitive weakness has been ranked.
Owner/root Codex uses primary `codex/rewrite`; no isolated production changes or
new worktree. MSH-160 opening questions are integrated. Read the full owner request
and `../../competitive-gameplay-diagnosis.md` protocol v0 before resuming.

## Unattended execution

`run_batch.py qualification-schedule.json qualification` launched four qualification
games (one color pair per Oxidation0.6.0 and Goldfish1.13.0) at5s per move. It uses
native UCI runner/adjudication and complete native replay, copied frozen binaries
under `/private/tmp/msh161`, source77101fe. Individual pair watchdog6000s; no
aggregate scientific compute ceiling. Current engine options/hashes are in
`build.json` and `preflight.json`. No production engine changed.

`qualification/batch.json` is the native batch state; per-pair receipts record
failures, elapsed/CPU time, complete response durations and replay. `inflight` is
not permission to restart: inspect preserved process identity and reconcile any
orphan first. A technical cap/failure is never a Chess draw or loss. Do not read
partial game output to supervise moves or select fixes.

The launched `after_qualification.py` dependency chain waits on the shared OS lock,
then requires completed healthy native replay before running fixed Stockfish19
analysis of every accepted move from both players and supplied prefixes. It then
screens source candidates separately. The chain's actual launch/status is recorded
in `postbatch-status.json`; absence means it has not launched. `analysis.py` uses
fresh processes/cleared hash, Threads1/Hash64,500000nodes for unrestricted and
forced played searches,60s query watchdog, bounded rawUCI, full histories and
persistent query receipts. `--nodes`, `--game`, `--ply`, `--move` support later
controlled confirmations. Native and reference work share `.execution.lock` and
must never overlap. Python environment: `/private/tmp/msh139/venv/bin/python`.

The app has one deferred review named **Review completed Chess diagnosis batch**
(id `review-completed-chess-diagnosis-batch`). It resumes this chat once at the
scheduled checkpoint, then must arrange the next appropriate batch review while
work remains. Do not launch duplicate pipelines. Keep unattended software doing
the computation; use agents only on completed batches. No repeated unchanged
model polling. Notify only meaningful findings/failures/completion/user action.

## Next scientific work

1. Review qualification results AND all-move reference outputs. Four start games
   do not prove relative strength; use a small diverse qualification extension if
   suitability remains unclear. Freeze one opponent before main sample.
2. Complete pilot validation: timing fairness, histories/native replay, score
   perspectives, meaningful stable versus uncertain reference comparisons, at
   least two acceptable alternatives, sound prepared roots and diverse structures.
   Confirm serious screens at5millionnodes, selected20million if needed, with
   same-root equal-work alternatives and native legal continuation admission.
3. Source candidates in `sources/` retain actual master games, independent of
   candidate performance. Respect source review and revisions. Screen balance
   without mShogi results, freeze90 histories with explicit structure coverage,
   and keep all rejections. Raw source archives are reproducible local downloads;
   selected factual source records and exact hashes are in repository.
4. Define/freeze exact opening phase transition, event/episode criteria, category
   rubric, uncertainty/threshold sensitivity and source selection before main
   ranking. Protocol v0 is a pilot draft, not a falsely finished method freeze.
5. Execute180 prepared games +20 separate unaided games in meaningful20–30-game
   reviewed batches. Complete coverage before priorities; justified extensions
   answer specific missing evidence. All games fixed opponent/binaries/conditions.
6. Give a fresh gameplay analyst completed games/reference outputs without earlier
   engineering explanations. Challenge classifications and later final rankings
   independently. Separate observed behavior, possible mechanisms, discriminating
   investigations and proposed general engineering. Update roadmap; stop for John
   before changes. MSH-153 stays held.

## Evidence and integration

Level1 offline study/helpers: native replay, independent reviews and meaningful
parser/score/history tests. No new Linux/Windows or GUI claim. Predecessor production
verification is reused only for unchanged source.14 offline analysis tests passed on the independently reviewed source; see
`offline-tests.log` and `analysis-review.md`. Real reference smoke remains an
automatic gate after the competitive batch completes.

Use Backlog1.55.1 at
`/private/tmp/mshogi-bootstrap-npm-cache/_npx/749414256ca1c44f/node_modules/backlog.md-darwin-arm64/backlog`.
PATH1.55.0 is read-only. CLI mutations need normal `.git` lock access.
Record actual commits after creation. Omit live lock files/bytecode from Git;
compress completed raw analysis reproducibly with hashes if size warrants, retain
all findings, failures and provenance. Do not commit executables/build output.

Launch receipts: native batch unified-exec session74120; follow-on chain session78978
(under caffeinate idle-sleep prevention). Read JSON states rather than assuming a
session ID proves continued execution. Initial follow-on state verified waiting
for qualification. The app review is scheduled for01:55 Europe/Paris,11November.

## Latest checkpoint —11November01:55review

Qualification completed all4games with native replay. Candidate scored2wins
against Oxidation and1win/1draw against Goldfish; opponent selection remains open,
not a strength conclusion. See `smoke-recovery.md`: the follow-on gate stopped on
correctly flagged finite Stockfish uncertainty. Independently reviewed gate-only
correction preserves uncertainty/null losses;16 tests passed and original failed
chain receipts remain. Full all-move analysis and root screening restarted via
unattended chain(session90158), using existing smoke without engine rerun. Next
single review03:15Europe/Paris11November. Inspect status once; no move polling.

## Latest checkpoint —11November03:15review

Initial all-move640decision/1280query analysis and330root screen completed with
zero failed queries;978.664s reference wall/953.951s childCPU including original
smoke. Full1618artifacts preserved with hashes in `initial-reference-evidence.tar.gz`
and `initial-reference-bundle.json`; raw local analysis remains available.
Blind gameplay review `qualification-gameplay-review.md` identifies several
provisional questions and strong contrary examples, not engineering priorities.

Neither reference is established stronger by the four start games. Independently
chosen strict-balanced Spanish/French/Semi-Slav/Sicilian roots now drive16additional
qualification games (both references/both colors), separate from main200sample.
`qualification_extension.py` launched(session86454) runs25singleton5M queries on
14pilotdecisions, then eight native pairs and allmove analysis, fully unattended.
`extension-chain.json` is its stage authority; shared locks prevent overlap. It
stops on failure rather than discarding/replacing work. Next single review08:30
Europe/Paris11November. Do not alter frozen protocol/helpers until chain finishes.

Pilot v0 drops too many finite screens:328/640 usable versus637numeric pairs,
so method review introduces separate read-only `screen_v1.py`; original flags and
v0 results are retained. Earlier aligned nonbound UCI reports are finite estimates,
not proven completed iterations. Version1 retains last-two-depth spread and final
same-move bound conflicts; it identifies followup, never confirms a mistake.
Mainmethod/sample/opponent remain unfrozen. Strict source screen70accepted,
244uncertain,16outsidebalance cannot itself supply90roots. Review v1 source
judgments/37root confirmation proposal before further source work. Preserve
structural quotas and all source selection/rejection provenance.

## Owner update — completion-driven reviews

Timed automation `review-completed-chess-diagnosis-batch` was deleted at the
owner's request. Do not recreate it or guess review times. The extension completed
all42steps, including16games and allmove analysis; its substantive review is now
due. Main opponent/method/sample remain unfrozen.

Every future unattended computation chain must be launched through
`notify_exit.py --thread 01a1c734-6d54-75e1-80e5-58074ecdc4dd --codex ABSOLUTE_CODEX_CLI
--receipt UNIQUE_RECEIPT.json --label LABEL --state CHAIN_STATE.json -- COMMAND...`.
Use the installed CLI path returned by `command -v codex`; current verified path:
`/Applications/ChatGPT.app/Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codex`.
Use normal execution escalation for this launch so the exit notification can
access Codex's local database. Do not change security settings, approval stores,
sandbox flags or CLI configuration. The wrapper calls only
`codex queue --thread THREAD --message MESSAGE` on success or failure; existing
helpers retain process bounds and artifact/state supervision. Launch under
caffeinate if idle-sleep prevention remains useful.

The notification receipt records computation exit and queue acceptance separately.
A nonzero queue result or timeout remains visible; no automatic retry because
an ambiguous timeout might already have queued the message. Refuse reused receipt
paths to prevent duplicate computation. A process killed without cleanup (SIGKILL,
host crash) cannot emit an exit message; preserved chain state remains the recovery
authority. No polling watcher or timed fallback is installed.

Actual completion wake verified: installed CLI accepted message
`01a1c989-56b4-78d2-a9ef-41ce90a11784` for the owning chat after validating the
completed extension and every allmove summary. Receipt:
`notifications/qualification-extension-completed.json`. The scheduled review has
been queued immediately; next action is its substantive gameplay/method review,
not another queue test or scheduler. Four notifier tests pass; targeted macOS
verification, no Linux/Windows claim. Group probes denied by the OS remain unknown
and require cleanup review; failure notification still proceeds.

## Current authoritative handoff — batch01 checkpoint complete;batch02 running

Main conditions remain frozen in `main-method.md`/`main-freeze.json`: Oxidation0.6.0,
production77101fe, equal5s/move,90master histories paired plus10unaided pairs,
200games total. No production changes or final priorities. Keep all10batches;
20completedgames do not establish population strength or a rating.

Batch01 native result is10W2D8L (prepared8W2D8L,unaided2W). The completed independent
execution and two gameplay reviews passed the continuation checkpoint. All2494played
moves retain their original dispositions;73queried decisions now have superseding20M
overlays. All7722unique reference queries succeeded (5564initial,2014at5M,144at20M).
The174nonflagged audit decisions revealed no stable significant missed-error trigger.
See `main/batch-01/{audit.json,verification-review.md}`, the two `review20m-*.md`
reviews and `adjudication20m/{audit.json,verification-review.md}`.

The73deeper comparisons comprise49supported finite deteriorations across BOTH
players,4subthreshold,14unresolved,4stable singleton controls and2claim-policy-limited
judgments. These are selected comparison dispositions, not independent causal errors
or prevalence estimates.20M supersedes5M even when it removes support. No automatic
fallback to convenient earlier scores; no absolute first/largest claims with earlier
uncertainty. `adjudicated20m-00-04.json` and `adjudicated20m-05-09.json` overlay the
unchanged complete ledgers (`decision-dispositions-00-04.json`,`adjudication-05-09.json`).
All legal histories/PVs, flags, sensitivity ranges and dependencies remain available.
Two literal stability flags at decimal.025 are identified as numerical-boundary
sensitivity, not silently waived or evidence of a large unstable swing.

Important interpretation limit: pair06/game1 is a native draw by White's automatic
threefold claim with intended UNPLAYED Bf3–g4 after50.Bf3 Bd5 (zero-basedp98/99).
Stockfish20M gives E1.0 to bothBf3/Bf5 atp98. Keep the native draw; its automatic
claim policy differs from Stockfish continuations in which a winning player may
decline a claim. Do not infer actual-result conversion failure/recovery there from
reference scores alone, or call this a rules/search defect. Both reviewers preserve
this limitation, with exact native event pointers. No mid-study policy change.

`main/cumulative-categories.json` now joins95selected examples and73overlays into
10descriptive parent categories, with actors/families/phases/support/severity separated;
`main/category-taxonomy.json` preserves every original analyst label. This is a
provisional discovery ledger, not a ranking. Retain successes, overlapping themes,
already-lost context and unqueried/linked uncertainty before final episode counting.
Further targeted work must answer a documented outstanding claim, never repeat until
a desired label appears. Complete all200games before final independent prioritization.

Actual batch01 processwall: native11929.047s, initialreference3264.835s,
5Mreference8980.574s,20Mreference2546.393s (20MchildCPU2540.266s). Chain elapsed
figures overlap and must not be added again. Audits retain exact accounting and
failure/cleanup provenance; no resource cap truncated scientific coverage.

Batch02 is RUNNING through `notify_exit.py`, execution session1756, state
`main/batch-02/chain.json`, notification `notifications/main-batch-02.json`.
Its frozen10pairs/20games are followed unattended by500k allmove analysis and5M
matched followups/audit. Normal local execution access permits exit notification;
no security settings changed. NO timer or periodic watcher. Do not poll or duplicate
this computation; its success/failure exit queues the owning chat.

NEXT on exit: independently validate/review completedbatch02, obtain consequential20M
checks, update cumulative categories/dispositions and then continuebatch03. Preserve
any failure state and reconcile before retry. Continue to200games, final independent
diagnosis and roadmap, then STOP for John before production engineering.

Verified archives: `main/batch-01-pre20m-evidence.tar.gz` (15792artifacts) and
`main/batch-01-20m-evidence.tar.gz` (723artifacts), each with its SHA/per-file
`*-bundle.json`. Raw local directories remain available. To restore a bundle in
another checkout, extract it from this evidence directory; archive paths are relative
to this directory. The20Mbundle includes the reviewed category snapshot and notification.
Earlier qualification/reference bundles remain separate. `verification.json` links
current stage evidence. No new Linux/Windows/GUI claim; study analysis is localLevel1,
with recorded localmacOSprocess/notification evidence. All final task criteria remain
unfinished. The separate committed read-only status tools are preserved unchanged;
this workflow does not launch a timed watcher.
