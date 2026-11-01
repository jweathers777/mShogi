# MSH-128: complete native local legality and inspection

Baseline `4710a4d9c1c188293fc89870cef68b2d0232e6c7`, primary `codex/rewrite`,
2026-11-01. Root Codex is the sole builder/integrator. Pinned Backlog.md 1.55.1
matched the project baseline; no other task was In Progress or In Review at startup.
Origin matched the primary checkout. Existing111/100/98 worktrees were preserved.
No external engine, GUI application, Python engine runtime or comparator was used.

## Acceptance coverage

| Original MSH-128 criterion | Executed evidence |
| --- | --- |
| AC1 complete local choices/apply | One public LocalPosition boundary, existing structural geometry reused; all40 accepted board fixtures through checked apply/undo/redo, complete independently expected choice sets, action uniqueness and exhaustive membership consistency probes. |
| AC2 roots/status | Both-owner T07 root tests, pinned geometric threat map, nonmoving-royal checks, two reviewed terminal roots, actual Ongoing/Check/Checkmate/QuietNoAction observations. |
| AC3 Swallow rules/defenses | All original local cases and owner rotations, third-own-Swallow and last-rank rejection, Goose/enemy exclusions, direct mate versus king escape/king capture/nonroyal capture/full defender hand, other-kind mate and last-rank drops. |
| AC4 immutable inspection | Native board/hand origins and actions/effects on an owner's private turn; opponent/pinned/check/terminal cases, both owner orientations, forced faces and available hand resources; full logical/runtime nonmutation and retained reversal authority. |
| AC5 conformance/diagnostics/review | Actual483 native assertions, source-derived attack tables, deterministic legal sequences, material/reversal/rotation/adversarial checks, independent bounded perft counts, explicit incomplete diagnostics, separate source/test review; actual integration/CI recorded below. |

## Surviving native API and semantics

`mshogi-core::tori::LocalPosition` is the sole checked **position-local** boundary.
`admit(Position)` enforces local root constraints over MSH-127's already checked
structural root. `standard`, `position` (immutable), `legal_actions`, `is_legal`,
`apply`, `undo`, `redo`, `attackers`, `is_in_check`, `status`, `inspect`, `perft`
and `divide` are the concrete API. Structural Position still has no public mutation
or candidate-admission escape hatch. A successful local choice is not full-game
permission after a conclusion or repetition obligation; those are MSH-129.

No movement rule was copied: candidates, attack geometry, compulsory resulting
faces, capture/demotion/ownership effects and committed reversal use MSH-127.
Effects is now public native data; LocalUndo/LocalRedo privately wrap its existing
single-use position-bound tokens. Clone/order/reuse/retired-branch rejection is
verified through an actual external Rust consumer. No serialized record, game
command, repetition key/ledger, engine capability, selector or session was added.

The local threat map includes pinned attackers and friendly defended endpoints;
it does not require an attacker to preserve its own Phoenix in a hypothetical
capture. It handles all nine faces, blockers, rays, Eagle limited rays and jumps.
Own-Phoenix exposure and actual Phoenix captures reject atomically. Root admission
permits the moving side to be checked or locally terminal, but rejects nonmoving
Phoenix check, adjacent Phoenixes, stranded last-rank Swallows and more than two
own unpromoted Swallows/file. Structural root provenance, missing material and
immutable declared counters are preserved without claiming historical reachability.

Swallow drops count only that owner's unpromoted Swallows on the file, forbid the
last rank and forbid mate delivered by the newly dropped Swallow itself. The mate
predicate enumerates all safety-valid defender moves/captures/drops. An adjacent
direct Swallow check cannot be blocked: a drop neither captures the checker nor
moves the Phoenix, so every defender drop fails self-check before its own drop-mate
rule could matter. Thus all safety-valid replies in this particular predicate are
fully legal board moves; there is **no recursive mate test**, no pseudo-legal escape
shortcut and no repetition input. Tests include both royal and nonroyal capture,
escape and full defender hands. Other piece drops may give mate and may use the
last rank. Drops never promote or create a timer; later forced moves remain native.

LocalStatus is a board observation only. QuietNoAction means the selected T05
stalemate-loss condition, never a Chess draw; MSH-129 installs actual match results.
Counter exhaustion cannot fabricate mate: finite legal/inspection/status queries
use private counter-neutral copies. Checked **commits** preserve actual counters
and return explicit ActionError::Capacity when unrepresentable. In particular,
a last-representable checking Swallow drop still sees genuine defender choices.

InspectionOrigin names either a board square (owner native-derived) or a side and
hand face. InspectedChoice includes the exact origin, complete Action and native
Effects on a private hypothetical turn. The query does not re-admit that temporary
turn as a real root, mutate actual state/counters/provenance/token identity, confer
live permission or start work. Invalid/empty/unavailable origins have no choices.
MSH-77 will project this native data into the shared desktop interaction owner.

## Exact executable fixture accounting

The selected profile and all103 fixture/ownership bytes remain unchanged from
MSH-81/MSH-126/MSH-127. [Actual field-level evidence](evidence/native-local-fixtures.json)
records **483 executed /483 passed /244 pending /0 failed**:

- MSH-127:202 passed, including all14 primary rows; its evidence reruns unchanged.
- MSH-128:281 passed, covering all69 primary rows and applicable supplemental checks.
- MSH-129:230 pending; MSH-77:13 pending; MSH-42:1 pending.
- Four documentary proofs remain annotations and83 later-consumer requirements
  remain separately pending. No history/conclusion/record/process field is passed
  by a local observation. The whole native game corpus is not yet certified.

All40 accepted board-transition specifications run through public local admission
with real exact snapshots, hand/material checks and undo/redo. The two accepted
resignation specifications stay129. Actual legal roots/rejections, full choice
observations, royal checks, declared native effects, inspection probes and rejection
snapshots come from native operations. One fixture's movement-probe dictionary
keys supply authored query inputs; its expected boolean answers are never echoed.
The standalone conformance CLI without a native adapter still reports727 pending.

## Tests, independent expectations and diagnostics

Reproduce on stable Rust1.99.1/cargo1.99.1 (local host macOS arm64):

```sh
cargo test --locked -p mshogi-core --lib tori:: -- --nocapture
cargo test --locked -p mshogi-core --test tori_local
cargo test --locked -p mshogi-core --doc tori
cargo test --locked -p mshogi-core -p mshogi-conformance
cargo clippy --locked -p mshogi-core -p mshogi-conformance --all-targets -- -D warnings
cargo fmt --all -- --check
MSH_TORI_128_REPORT=/tmp/tori128-report.json cargo test --locked -p mshogi-core --lib tori::local_conformance -- --nocapture
# Pinned Backlog.md1.55.1 and optional standard-library metadata check:
backlog doctor
python3 docs/m4/tori/evidence/verify-native-local-integrity.py
git diff --check
```

Local cached runs used `--offline`. **25 Tori unit/conformance tests passed,
1 external public-token test passed,2 Tori API/doctests passed**, no Tori ignores.
An initial unused test import warning was removed before final lint; no rule/test
expectation changed and no native assertion failed. Full core/conformance regressions passed 262 checks across 33 reported suites,
zero failed. Two existing release-only deep Chess checks were not run:
`deep_independent_divides_match_in_order` and `deep_fixed_seed_sequences`.
They remain opt-in, not new Tori ignores or claimed passes. Lint/format results
are separately recorded; no expensive completed-game campaign was repeated.

Independent complete-choice expectations are written in the Rust tests before
comparison: distant corner Phoenix has exactly3 moves; a long single check allows
four royal escapes plus five Crane blocking drops; adding a Goose jump removes
all blocking-drop defenses; a pinned Crane has only its vertical blocking move.
The original mate/stalemate roots and drop-mate/escape proof fixtures remain fixed.
Additional hand-derived capture defenses distinguish royal capture of an unprotected
checker, a Crane capturing a protected checker, and the prohibition of defensive
drops against direct adjacent check. Native all-face threats expand only the
independently authored fixture offsets, not a second rules engine.

Properties run192 deterministic **legal** actions twice, restore every reverse/
redo prefix exactly, preserve native base material, and verify inspection nonmutation.
Both-owner rotation compares complete native choices and effects. Exhaustive typed
request membership versus enumeration is a generator consistency check, supplementing
rather than replacing independent rules expectations. All40 accepted fixtures also
use the shared conformance apply/undo helper. MSH-127 structural regressions remain
separate from these complete-local checks.

Perft/divide are explicit read-only diagnostics, never routine rendering or search.
Depth is bounded0..8 (divide1..8), visits1..1,000,000, with one caller-supplied budget
shared by the whole traversal. The work unit is a visited board node, including
root; fixed-board legality/defense enumeration occurs inside that unit. There is no
wall-clock or playing-strength promise. Limits, exhausted work, count overflow and
unexpected native action/reversal failures remain distinct errors with exact native
paths; incomplete runs return no partial count as exact. Only independently
established counts are asserted: distant corner royals give depth1=3, depth2=3×3=9,
visited=1+3+9=13; terminal roots have depth1=0. No engine-generated perft expectation
was introduced. Original position/provenance and tokens survive diagnostic failure.

## Actual independent review

Separate read-only `tori128_review` inspected owner request, T02–T07, original
fixture/ownership definitions, production diff and test adapters. Method: manual
semantic source/API/fixture challenge plus independently executed tests and Clippy.
It verified attack/evasion rules, drop defenses/nonrecursion, root admission,
immutable hypothetical inspection, counter/query separation, public/structural
boundaries, observation independence and perft failure paths. No material finding.

The reviewer suggested direct public LocalUndo/LocalRedo stale/clone/branch tests.
This was implemented in an external-consumer Rust test and independently rerun;
both-owner last-rank/drop-effects/forced-face and shared40-fixture round-trip checks
were also independently verified. Final review **PASS, no remaining findings**:
25unit tests,1public integration test,2doctests and core all-target Clippy passed;
profile/corpus/manifest bytes unchanged. Exact reviewed file hashes are retained
in the execution evidence. No review agent edited source or task records.

## Integration and remaining scope

Actual integrated revision, native CI and final status are recorded below after
execution; no future commit or platform result is assumed. MSH-128 remains In Review
until all original criteria/completion items have supporting integration/CI evidence.
Existing Chess/Shogi/Xiangqi rules, consumers, current execution/archive/frontend
architecture and MSH-61 saveability guarantee are unchanged. Normal native CI is
required; no packaged GUI or strength campaign is required or claimed here.

MSH-129 owns repetition/departure obligations, full-game command permission,
conclusions/resignation, canonical records/strict replay, capacity measurement and
native dispatch/context. MSH-85 remains final native acceptance, MSH-77 shared
runtime/document/inspection presentation and MSH-42 complete engine/console/desktop
delivery. None is automatically activated; M4 remains open.
