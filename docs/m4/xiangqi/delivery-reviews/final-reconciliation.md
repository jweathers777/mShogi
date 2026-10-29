# MSH-103 final independent acceptance reconciliation

Review date: 2026-10-29. Independent reviewer: fresh read-only Codex agent
`/root/finalize103_review`. Primary integrator alone edits durable records and
performs integration. This is the bounded resumption of the existing review,
not a replacement complete-game assessment.

Reconciliation baseline: `869bee3815750b44a80aa855cfc347e29724cd2e`.
Corrected production source: `1c6b97e2bf7ecf560be1902e01b4e95038ee3482`.
Original complete-game assessment: `541fc303fdeaf5017948d14ddf92bf5efee5ab3f`.

## Method and evidence inspected

The reviewer read MSH-103, MSH-114, the original eight MSH-45 criteria, all three
original domain reviews, the acceptance matrix and MSH-114's source delta,
regression tests, execution report, CI records and package/GUI manifest.
Retained PNG and accessibility evidence inspected included normal Xiangqi
control exhaustion, verification-build Shogi control exhaustion and Xiangqi's
257th-observation refusal. These are reviews of actual retained evidence, not
new independent GUI execution. No tests or GUI campaign were rerun.

The source comparisons covered the original blocked-review head through the
correction, and correction through reconciliation baseline. Native rules,
search, records, profile decisions and independent expectations remain
unchanged; native additions are QA-gated fixtures/tests. No production diff
exists after the correction. The reviewer also inspected the final current-
source verdict, eight-row matrix and finalization evidence JSON.

## Findings and disposition

**Approve MSH-103 final acceptance. No material game-specific finding remains.**

- F01 is resolved exactly: a control-only refusal no longer invents recorded
  observations. Generic byte/event/replay/ambiguous reasons stay truthful;
  genuine ordinary/exact observation exhaustion retains accurate accounting.
  Saveability and stop/close reservation remain intact.
- MSH-45 AC1–6 and the selected WXF 2018, G-CYCLE and G-NOWIN contracts remain
  supported by the existing independent assessment and execution evidence.
- Existing complete-game desktop evidence plus independently reviewed MSH-114
  supports AC7; the complete ledger, original domain reviews and this bounded
  reconciliation support AC8. All MSH-103 acceptance and completion items are
  evidenced, subject to the primary integrator's normal final integration.
- The final matrix preserves the historical blocked assessment and failed
  probe. It does not claim that those failures passed or repeat the full audit.
- The later documentation-head Ubuntu failure in run40961719798 matches the
  recorded MSH-112 runner classification race. It remains a real failure,
  alongside successful source-identical production CI, not a new Xiangqi
  defect or a claimed fix. MSH-111's Linux driver problem remains open.

The exact correction-source Rust40959072793 and Desktop40959072831 results
are successful on all three target systems. They are distinct from actual
macOS Xiangqi GUI evidence. No Windows/Linux Xiangqi GUI execution is claimed.
MSH-45 AC7 explicitly assigns final shared packaged three-platform acceptance
to MSH-46/MSH-47; the final matrix preserves that ownership.

## Boundaries and next assignment

MSH-45 and M4 remain open. MSH-106 remains unstarted, including the recorded
interaction duplication, archive lifecycle/dispatch and structured resource-
diagnostic questions. These are not deferrals of a required Xiangqi fix.

The primary integrator queried `backlog task list --status Ready --ready --plain`:
no tasks were returned before finalization. Completing MSH-103 satisfies the
remaining MSH-45 dependency; its separate acceptance reconciliation is the
next dependency-satisfied assignment, still To Do. This review does not change
its status, criteria or the downstream architecture review gate.

See the [final current-source matrix](../delivery-review.md) and
[exact finalization evidence](../delivery-finalization-evidence.json).
