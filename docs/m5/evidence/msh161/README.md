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
