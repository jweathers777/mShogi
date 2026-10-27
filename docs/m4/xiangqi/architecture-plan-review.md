# Independent MSH-80 architecture and task-plan review

Reviewer: separate bounded agent `/root/xiangqi_plan_review`, 2026-10-27.
Method: fresh read-only requirements/document/source review and an independent
frontmatter dependency/status/criterion-preservation check. Root remains the
sole integrator. No product code was changed, built or run by this reviewer.
This is agent review, not an association rules ruling or human visual review.
Baseline: `14b9643dac70f09394ccc60aea8b0f8c5bbd5b03`, primary `codex/rewrite`;
reviewed the uncommitted MSH-80 planning changes and new MSH-91–103 records.

## Reviewed boundary

Read AGENTS.md and Backlog overview, docs/design.md, current user scope,
`docs/m4/xiangqi/{profile-proposal,sources,fixtures,handoff,comparator-audit}.md`,
original/current MSH-45/76/80/84 and all new MSH-91–103 criteria/dependencies.
Inspected actual consumers:

- `crates/mshogi-core/src/native.rs`: explicit game/profile/schema variants,
  native-owned canonical payload, RawValue-preserving strict dispatch and
  complete-event versus board-action identities.
- `crates/mshogi-engine/src/shared.rs`: purpose and exact target/native context,
  operation capabilities, request/session/process identities and limits.
- `crates/mshogi-desktop-session/src/shared.rs`, `shared_inspection.rs` and
  `shared_archive.rs`: session/selection tokens, native read-only inspection,
  semantic board/pieces/choices, fresh runtime identities, encoded capacity and
  replay-work admission with reserved stop/close path.
- `desktop/src/product/ApplicationShell.tsx` and `desktop/src/NativeBoard.tsx`:
  accepted common dark composition and stable sidebar/control slots, contrasted
  with the current square-board adapter that does not itself represent Xiangqi.

No universal rules framework or abstract board rewrite is justified or demanded.
The handoff's tagged native extension, minimal topology metadata and separate
appropriate intersection rendering follow the concrete consumers. They preserve
Chess/Shogi authority rather than homogenizing their semantics.

## Findings and verdict

No blocking architecture, ownership, scope-loss or dependency finding in the
reviewed plan. One minor copied terminology finding: MSH-100 AC2 initially says
failures remain separate from “chess outcomes.” This should say “native game
outcomes” (or “Xiangqi outcomes”). Root was notified; final integration should
contain the corrected wording. It does not require a new owner decision.

The profile is correctly presented as an **unselected, incomplete candidate**.
MSH-80 AC1–3 are not satisfied by its existence: AXF/CXA acquisition/reconciliation,
owner choices, exact cycle/interval policy and neither-can-win semantics remain
explicit gates. MSH-80 must remain unfinished; do not infer acceptance of option A
or complete conformance from this review. AC4's planning/refinement scope has
supporting evidence. Rules interpretation itself is assigned separate source
review; this review does not certify Chinese translations or fixture facts.

The complete MSH-45 obligation remains intact: native state/local rules/history,
independently expected fixtures plus actually admitted reference, own standalone
console, own functional evaluator/search, protocol/match execution, canonical
records and actual desktop journeys all have explicit owners and final review.
MSH-54 retains measured strength. Full history and inability-to-win obligations
are not moved to M5. R-CLASS instantiation has a future owner without requiring
all history/protocol/archive/presentation fixtures before the state kernel.

Shared MSH-95/98/99 belong to MSH-46, while native MSH-94 belongs to MSH-45.
Native handoff84 has a real record consumer95 without depending on session76;
95 does not depend on84. Product101/102 follow76, avoiding a shared/full-UI
acceptance cycle. Native read-only enemy inspection and history provenance are
explicit; no TypeScript movement engine is introduced. The common dark shell,
stable document/actions/analysis/history/details hierarchy and footer remain
requirements. Xiangqi geometry includes intersections/river/palaces/native
pieces and full-board visibility at normal/minimum sizes without scroll/zoom.

MSH-61 is retained concretely in99: actual encoded byte/event/control/observation/
replay-work boundaries, pre-commit admission and reserved stop/close/save path,
including256→257 observations; no raising-constant-only or silent loss. Strict
RawValue dispatch, complete known histories, native terminal commands that retain
board ply, cancellation/replacement/undo/selection retirement, purpose separation
and truthful unsupported capabilities remain explicit across95–102.

## Independent structural checks

A Python read-only scan parsed all103 active task frontmatters and traversed the
complete dependency graph: **no missing dependency or cycle**. It compared the
original HEAD criteria/DoD of45/76/84 to the current records: every original item
is preserved verbatim; original dependency sets remain subsets of the current
sets.45 and84 original criteria are not silently replaced;76 has an additive
shared-presentation requirement with product rendering verified later.

Status assertions passed:41/75 Done,42–47 open,80 unfinished,91–103 To Do.
For80/84/76/45 and every91–103 ticket, ancestor traversal found no dependency on
81–83 research or42–44 other-game final delivery. No artificial delivery-order
chain was introduced. After80 is actually completed and further work authorized,
91+92 form the first independent bounded queue, then93; none is currently
executable through the unfinished80 gate.

Verification limit: this was documentation/plan review plus source inspection,
not runtime correctness, reference admission, packaged GUI, native CI or playing
strength evidence. Existing unchanged implementation evidence remains untouched.

## Final correction check

Re-read the finalized handoff, planning-review index, docs/design.md addition
and task-status snapshot after root's bounded corrections. MSH-100 AC2 now
correctly says **native game outcomes**; the minor wording finding is resolved.
D-14 records only actual owner acceptance of Standard Shogi and the common
future-game shell; it does not adopt a Xiangqi profile. No new material
architecture or task-boundary change was introduced.

Read-only assertions reconfirmed80 In Review with **only AC4 checked**,
81–83 To Do/unassigned,91–103 To Do/unassigned,41/75 Done, and only documentation/
Backlog/README tracked changes. R-CYCLE remains an explicit incomplete-profile
blocker rather than a claimed deterministic repetition policy. The finalized
review index accurately distinguishes source/plan reasoning from runtime tests,
reference admission and actual platform CI; this reviewer makes no runtime claim.
No unresolved architecture/planning finding remains, subject to the existing
explicit source/owner-decision blockers that keep80 unfinished.
