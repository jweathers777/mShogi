# MSH-63 independent selected-profile review

Date: 2026-10-26. Reviewer: fresh Codex review agent, separate from the profile
author/integrator; this is an independent agent review, not a human review.
Worktree: `next-task`, branch `codex/shogi-board`, HEAD
`f63f2d2c929659ee0a9a7f7e0034f3a1b375fd4c`. The reviewed selection edits and new
coverage ledger were uncommitted working changes at review time. Concurrent
board/state/reference work was outside this review.

**Verdict: PASS for MSH-63's bounded selection/profile/ledger scope. No material
findings remain.** This does not certify board or history implementation,
executed fixtures, reference admission, complete shogi conformance or MSH-41
delivery. Integration and task acceptance remain the integrator's responsibility.

## Method and evidence

Read `AGENTS.md`, the Backlog overview, MSH-63's four acceptance criteria,
MSH-67's dependencies and acceptance criteria, the selected profile, the complete
new clause ledger, the fixture inventory, relevant design/D-13 and delivery-plan
sections, and the M4 README. Compared the selection diff against HEAD and read
the proposal at owner-approved integration
`6db8500e92129b95e502e7b4d5c4a26072e38652`. Read the existing MSH-62 planning
and rules reviews as historical evidence rather than presenting them as a fresh
review of this decision.

Read-only commands included `backlog instructions overview`, `git status
--short --branch`, `git rev-parse HEAD`, `git diff -- docs/design.md docs/m4/`,
`git show 6db8500e92129b95e502e7b4d5c4a26072e38652:docs/m4/shogi-profile.md`,
and targeted `cat`, `sed` and `rg` reads. Primary-source web retrieval rechecked
the JSA rules PDF, JSA check/mate and prohibited-play pages, and CSA36 rules PDF.
No product code, task record or author document was edited by this reviewer;
only this report was written. No test suite, reference engine, opponent match,
GUI or packaged build was run. Executed fixtures are not an MSH-63 acceptance
requirement.

## Selection and clause assessment

The integrator supplied the explicit owner approval of option A and its
software adaptations. The current profile, D-13, M4 README, delivery plan and
coverage ledger consistently record `SHOGI-P1`,
`mshogi.shogi.csa36.v1`, the date and approved integration revision. Untimed local
play, advisory versus committed declarations, atomic request rejection and
declared-counter/bounded-known-history semantics are all included. The document
does not silently incorporate professional JSA declaration/replay thresholds.

The rechecked [CSA36 rules](https://www.computer-shogi.org/wcsc36/rule.pdf),
Arts.22 and 25–27, support the declared 28/27 thresholds, camp/material/turn/check/
time conditions, false-declaration loss, no-legal-action loss, final repetition
draw with continuous-check exception, and 320-ply loss/repetition precedence.
The recorded 2023-11-27 edition date matches the PDF. The selected software
boundaries are explicitly identified adaptations rather than claims that this
application reproduces all tournament conduct, timing or adjudication.

The [JSA rules PDF](https://www.shogi.or.jp/match/taikyoku_rules/taikyoku_rules.pdf),
Arts.8–10, supports keeping four-occurrence identity, JSA replay procedures and
professional entering-king alternatives traceable. Current selected identity
includes both owners' hands, promotion/ownership and turn; counters and elapsed
time do not become repetition evidence. The imported-root policy consistently
uses supplied positive SFEN `n - 1` for declared length while counting its root
once in a new known history segment. Default/replay provenance is owned by
MSH-67; MSH-65 only preserves and verifies supplied position counters.

The outcome contract covers resignation, declaration commitment and failure,
no-action loss with or without check, whole checking intervals, priority over
ordinary repetition and length, and game reason separately from competition
score/replay. Declaration/resignation do not invent board plies. A move that
completes a history-dependent losing cycle is retained with its consequence.
No unnamed timed policy or retrospective officiating mode is promised.

## Board interpretation and deferred gates

The ledger's **BOARD-PIN** gate belongs to MSH-64/66, specifically
SH-F016/020/021. It distinguishes geometric influence used for king safety
from a defender's legal capture while pinned. The
[JSA check/mate explanation](https://www.shogi.or.jp/knowledge/shogi/04.php)
and [self-check prohibition](https://www.shogi.or.jp/knowledge/shogi/05.php)
support threat-based king safety and excluding a defense that exposes its king.
The pinned-attacker rule is an operational formalization/inference from those
rules, not a quotation explicitly addressing that rare configuration. Its
independently reasoned diagrams, pinned defender contrast and king-origin x-ray
must pass before affected board legality is accepted. An engine helper alone
cannot discharge this gate. This review does not claim those fixtures executed.

Remaining history/declaration/import research has exact names, fixture ranges
and an implementation owner in the ledger:

- **HIST-INTERVAL**, MSH-67, SH-F028–030: checked/check-relieved fourth
  occurrences, interrupted checks, overlapping windows, and a legal mutual-check
  example with source resolution or a reasoned impossibility proof.
- **DECL-BOUNDARY**, MSH-67, SH-F034–037: valid material/point constructions and
  each failed prerequisite, preserving probe/commit and untimed semantics.
- **IMPORT-LENGTH**, MSH-67, SH-F041: n320/n321/beyond, no-action/repetition
  precedence, known-history origins and unproven reachability.

These are explicit blocking fixture/research items before affected
implementation, as MSH-63 AC3 permits; they are not invented completed evidence
or new owner decisions. MSH-67's existing criteria require the corresponding
history, import and overlapping-boundary resolution. Unselected JSA-only
questions remain conditional and do not block option A.

## Current versus historical evidence

The MSH-62 reviews remain unchanged and accurately describe their preapproval
boundary. The fixture inventory now labels its constructions as historical
planning and directs current clause ownership/status to the new ledger. Thus
older conditional/proposal wording in individual rows does not assert that the
current profile is unapproved or make JSA alternatives selected obligations.
Neither the selection nor this review closes MSH-41, MSH-46 or M4; later native,
record, console, engine, desktop and independent delivery gates remain intact.
