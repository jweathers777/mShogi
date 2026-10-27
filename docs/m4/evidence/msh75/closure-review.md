# Independent final MSH-75 / MSH-41 acceptance reconciliation

2026-10-27. Fresh delegated AI reviewer `msh75_closure_review`, independent of
the sole integrator and the three delivery review tracks. Reviewed product
revision: `80d6b5cc71c8ceb5bf782d93295797aed1e18d66`, primary `codex/rewrite`.

**Verdict: PASS. All eight original MSH-41 acceptance criteria are supported
for the complete selected standard-shogi profile. No material unresolved
correctness, completeness, persistence, interoperability, accessibility or
product-contract finding blocks MSH-75/MSH-41 closure.** The integrator may
complete MSH-75 and then MSH-41, recording and normally integrating/pushing this
evidence. This review itself changes no task status and does not claim the
subsequent evidence commit already exists. M4 and its remaining games/shared
final acceptance remain open.

## Actual method and independence

Read repository AGENTS.md, Backlog overview and finalization guidance, the
original MSH-41 AC1–8/DoD, MSH-75 AC1–4/DoD and active plan, the complete current
owner attachment `c9d5505e-91b8-409b-8893-33cd14a95d50`, design requirements and
architectural boundaries, selected profile and clause ledger. Read the full
three fresh independent reports and their machine evidence, the proposed
complete-delivery mapping, current README/delivery-plan/coverage diffs, normal
Mac journey/runtime/archive facts, current CI metadata and retained platform
applicability audit. Child task statuses were context, not proof of acceptance.

Spot-checked actual native outcome precedence/check intervals, search terminal
winner/declaration/restoration logic, and archive candidate admission, byte/
observation/control/replay-work bounds and stop reservation. Inspected actual
fresh archive envelopes, native events/roots, observations and historical target,
and selected native AX captures, rather than treating screenshots as canonical
records. I did not change product sources, expectations, profile, task records,
other evidence or commits. Independently checked `backlog --version` on the
default PATH:1.55.0; the explicit pinned
`/private/tmp/mshogi-bootstrap-npm-cache/_npx/749414256ca1c44f/node_modules/backlog.md-darwin-arm64/backlog --version`
returns1.55.1, matching the managed instruction baseline. The integrator uses
that compatible pinned CLI for mutations; no lifecycle mutation was attempted
by this reviewer.

Executed read-only Python SHA-256/size and report/log-counter audits plus Git
source-diff and whitespace checks. I did **not** repeat the fresh native,
reference, subprocess, opponent, host/frontend suites or live GUI operation:
those actual executions belong to the separately identified reviewers and
integrator. This is independent final source/evidence/visual reconciliation,
not another claimed execution of their commands or a human expert translation
or screen-reader certification.

## Original MSH-41 acceptance mapping

| Original criterion | Integrated obligation and evidence assessed | Result |
| --- | --- | --- |
| AC1: complete sourced identified/versioned profile | `mshogi.shogi.csa36.v1`, edition `JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026`, is explicitly selected by SHOGI-P1. Profile, clause ledger and [rules review](rules-review.md) jointly account for every required rule and approved software interpretation. Native board, history and command evidence below discharges the contract; no required clause is deferred. Unselected JSA conventions and timed/retrospective procedure are expressly outside this profile. | Satisfied |
| AC2: authoritative reusable state/actions/history and exact reversal | Native shogi owns board/hand identities, complete actions and derived effects, atomic rejection, checked transition, exact undo, native events/history/adjudication and canonical replay. Repetition identity excludes counter/origin; record identity retains them; execution additionally includes complete context and purpose/runtime identities. Search and session submit native commands. Frontend consumes native complete choices and does not duplicate legality/outcomes. Rules and [archive/session review](archive-desktop-review.md), exact reversal/sequence/strict-context tests and inspected source establish the composition. | Satisfied |
| AC3: independent corpus and reproducible conformance | Independent fixture derivations and prior independent reviews supply expected facts for 101 board and 68 completeness cases, history, declaration, sequence/reversal/record behavior. Fresh reference JSON actually contains 169 passing fixtures, 167 sequences, 1,304 plies and 1,471 prefixes with no mismatch. The independent ledger audit supplies 21 ledgers/149 prefixes and nine declaration diagrams. Applicable retained 169 depth-two perft/divides and 4,017-prefix evidence have explicit source/algorithm applicability. Only initial depth-two900/divide30 is independently hand-derived; other perft counts remain differential observations. Neither legacy agreement nor reference helpers define outcomes. | Satisfied |
| AC4: independently usable console and notation | [Engine/console review](engine-console-review.md), retained direct journey and actual subprocess tests exercise `mshogi --game shogi shell`, common game-selected commands, complete move/drop/promotion notation, inspection, apply/undo/history/outcomes and position-only diagnostics. Fresh capture/demotion/promotion, terminal same-ply event, strict Save/Load/replay/re-export, duplicate rejection and no-clobber behavior preserve native authority and separate human/machine output. | Satisfied |
| AC5: functional own engine and real smoke | Native search uses complete `Game`, native conclusions and every legal board alternative; actual winner scoring handles checking-side loss. Declarations submit terminal commands at roots/leaves without alternating-turn assumptions. Push/pop unwinds after limit/cancellation/failure; observations report completed work only. Exact context/purpose/limits and retirement protect structured consumers, including stale/history/cross-game results. Fresh 68 focused tests, real structured process evidence, finite supported USI operation, six selfplay and six actual pinned YaneuraOu games with replay establish functional play. Reports preserve action-limit cutoffs as null conclusion/score and do not imply strength. | Satisfied |
| AC6: complete strict canonical persistence | Native records retain profile/edition, standard/imported root and counter provenance, known-history scope, complete commands/events, policies and checked conclusions. Strict native replay validates them. Shared v2 archive retains native bytes, initial/current records, settings/control journal and observations, with exact history selection semantics. Duplicate nested fields reach strict parsing. Fresh strict subprocess/archive tests and real normal GUI Save/Open records corroborate this path. Reopen establishes fresh runtime tokens and stopped work. MSH-61 precommit capacity/reservation coverage is preserved, with no truncation of accepted work. Common migration retains its single shared owner; current chess/shogi consumers are actually integrated. | Satisfied |
| AC7: actual desktop play, analysis/history and recovery | [Fresh normal Mac journey](macos/README.md) supplies standard-start Sente and Gote human/engine play, capture/hand/demotion, optional promotion and cancellation, native Save/Open, stopped fresh reopen, history-prefix analysis, Return/Resume/Flip, keyboard and opponent inspection. Explicit imported reduced-material play supplies complete drop/cancellation and resignation events. Source-equivalent retained normal and separately labeled verification platform journeys cover forced promotion, declaration, stale/failure/recovery/capacity and minimum fit; native/frontend/host tests cover the same boundaries. I directly viewed actual product/minimum captures as described below. Final all-six-game packaged acceptance remains MSH-46/47/88, as the original criterion requires. | Satisfied |
| AC8: complete ledger and independent integrated assessment | Current clause ledger and [delivery review](../../shogi-delivery-review.md) map the complete profile through actual consumers. Three independent tracks plus this reconciliation record actual methods, failures/limitations and evidence categories. Current documentation reconciles stale earlier future-tense statements without erasing historical records. Required current shared/context/archive/console mechanisms are integrated, and required shogi correctness is not moved to M5. MSH-46/47 retain remaining six-game integration findings and final gates. | Satisfied |

## Complete selected-profile disposition

No selected rule remains an unnamed future requirement. SH-F001–024 and the
independently expected both-side corpora cover exact initial identities, all
base/promoted movement and blockers, captures/demotion/hands, every drop kind,
nifu/dead ranks, pawn-drop mate and legal king/piece defenses, optional/mandatory
promotion and king safety. Atomic admission and exact reversal remain native.

SH-F025–044 supplies four known occurrences and the whole first-to-fourth
checking interval, either checker's loss, interruption/overlap, no-action loss
with/without check, advisory versus formal entering-king declaration, 28/27
thresholds, false declarations, resignation, declared320-ply precedence and
imported counter/unknown-prehistory semantics. The inspected source prioritizes
no-action loss, then repetition/checker loss, then length; it does not erase a
legal action that causes checking-side loss. The mutual-check obligation has
the accepted complete-chain proof and independently reviewed conditional geometry
for the admitted-root domain, not a guessed winner or omitted legal action.
It is not exhaustive state enumeration or a proof-assistant result.

SH-F045–055 traces strict encoding, record/history/prefix restoration and real
console/engine/transport/runner/session/archive/normal-desktop consumers.
Same board/different history and same ply/different terminal events retain
distinct meaning. Imported roots start one known occurrence and never invent
earlier history. Unselected24/31-point/replay/agreement conventions, timed
eligibility, handicaps, tsume/try rules and physical/retrospective officiating
are outside the approved initial profile, not deferred required correctness.

## Verification and visual reconciliation

The independently reproduced checks in this final review were:

- `git status`, branch/remotes/worktrees/log inspection: primary `codex/rewrite`
  at80d6b5c; integrator-owned evidence/task/documentation edits preserved.
- `git diff --name-only 1b10277..80d6b5c` for production Rust/frontend/host,
  scripts, workflows, locks and package/toolchain settings: empty. The wider
  `desktop` diff contains only its README. Reusing final1b10277 Windows/Linux
  interaction is justified by unchanged affected production and driver bytes.
- Python size/SHA-256 audit: all **59** freshly indexed normal Mac artifacts
  and all **175** retained earlier platform originals match (73Linux,
  71Windows,31Mac). All28 immutable native/test/fixture/proof/tool/profile hashes
  in the rules manifest still match; its29th entry is the then-current coverage
  ledger, subsequently changed by this task's reviewed documentation-only
  reconciliation. That historical fingerprint is not rewritten as a current
  hash or used to assert unchanged coverage prose.
- Raw retained workspace/frontend log SHA-256 values match the final manifest;
  recounting56 Rust result groups gives **409 passed,0 failed,2 ignored**;
  frontend output gives **161 passed in21 files**. The ignored tests are the
  explicitly optional deep chess checks. Focused reviewer counts overlap these
  suites and are not added as unique coverage. Formatting, clippy and strict
  TypeScript are separately recorded PASS, not inferred from test counts.
- Both current CI JSON files bind **80d6b5c**, completed/success, with successful
  Rust and Desktop jobs on Windows2022/macOS15/Ubuntu24.04. Runs are
  40505357243 and40505357155. I inspected retained metadata, not fresh remote CI
  execution; prior platform interaction binds1b10277, not a new80 GUI run.
- Fresh reference report counters and all12 fresh runner records agree with
  the summaries. Each runner report contains two action-limit stops with null
  score/conclusion, two mating-drop native wins and two declaration native wins;
  every game has complete traffic and no execution failure. Opponent identity
  matches admitted YaneuraOu0f421391 executable hash74eded93…d3987f86.
- `git diff --check`: PASS before adding this report.

I directly viewed fresh normal Mac `capture-hand-engine-reply.jpg`,
`promotion-choice.jpg`, `historical-read-only-analysis.jpg`,
`fixture-complete-drop-engine-reply.jpg` and `final-normal-reopened-stopped.jpg`,
plus both retained normal Windows/Linux `normal-shogi-minimum-size.webview.png`.
These show the real shared application shell, native game/history/engine panels,
wood/kanji/ownership/hands, complete promotion modal, explicitly read-only event3
analysis, imported drop continuation and stopped reopened document. They are
visibly distinct from a development verification harness. Both760×650 client
captures show the entire81-square board, both player areas and interaction/footer
text without board scrolling/zoom/pan. Sidebar scrolling is separate. Current
presentation architecture introduces no mandatory scroll/zoom/pan policy for
future12×12/16×16 games, whose fit remains future work.

AX pairs explicitly show reopen document2→3, version26→27, revision27→28 and
selection26→27; stopped Resume and historical read-only labels are present.
Full native JSON is AX-truncated, so these captures alone are not claimed as a
complete runtime-state comparison. The retained complete archives independently
contain standard Sente8events/5observations/15controls, standard Gote3/2/4,
imported drop4/1/4 and imported resignation5/1/5. Imported roots preserve supplied
counter17 and unknown earlier history. The historical analysis targets exactly
three native events; the resignation is a fifth event at four board actions.

MSH-61 is specifically supported by candidate-before-commit admission and
fresh actual256→257 observation rejection,16MiB byte checks,8192 controls,
native321-event/16,000-work bounds, identity-width/historical-flag allowance and
reserved stop/close capacity. The reviewed archive tests retain all accepted
events/observations through Save/Open/Save As. A capacity refusal is an explicit
precommit limit, not accepted-work loss or a promise of unlimited retry.

Evidence categories remain separate: independent fixture facts, reference
comparisons, native tests, real subprocesses, actual external-opponent execution,
frontend/host tests, normal packaged GUI, verification GUI, platform execution,
AI review and owner visual approval. Windows/Linux reports are **3 normal plus
11 verification phases each**, not14 normal journeys. Current normal Mac work
does not claim fresh forced-promotion/declaration/recovery/minimum execution;
the source-applicable existing evidence covers those. Historical failed/cancelled
CI, wrong-domain diagrams, invocation errors and pre-fix retirement failures
remain failures or limitations in their retained records.

## Findings, limits and completion gate

There is no material finding and no required product repair or aesthetic
reopening. The earlier stale coverage prose has been reconciled with reviewed
documentation changes; its historical manifest fingerprint remains deliberately
historical. No additional rule/strength requirement was invented or narrowed.

MSH-75 AC1–3 and MSH-41 AC1–8 have evidence above; MSH-75 AC4 is satisfied by
closing its complete-delivery parent only after this independent result and
normal recorded integration. Applicable independent reviews are recorded,
material findings are absent, documentation/limitations/future gates are current.
Both tasks' final integration/summary DoD still requires the integrator's actual
completion commit/push and truthful references; this report does not fabricate
that future hash. The already tested product is integrated at80d6b5c, and the
remaining review change is evidence only.

The accepted delivery remains untimed, bounded, material-only full-width native
play, depth1–4/nodes0–20,000 (default2/5,000), with a documented finite USI subset.
No clocks/ponder/searchmoves/tsume, competitive strength, variation editing,
formal screen-reader certification or signed distribution is claimed. External
opponent execution here is macOS only; Linux GUI uses X11/Xvfb and Windows large
native captures retain the clipping caveat. Actual minimum/client images cover
fit. These are explicit accepted/future boundaries, not missing selected rules.

MSH-50 retains measured shogi strength. MSH-42–47 and76–89 retain other games and
shared six-game/platform closure; MSH-47/M4 must remain open. The documented next
dependency-ready M4 research is MSH-80 xiangqi (provisional next game), MSH-81
tori, MSH-82 chu and MSH-83 tenjiku, preceding their refinement/implementation.
No next-game work or M4 closure is part of this PASS.
