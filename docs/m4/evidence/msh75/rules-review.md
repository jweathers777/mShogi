# MSH-75 independent complete native shogi rules review

Reviewed 2026-10-27 by a fresh Codex AI reviewer, separate from the original
builders and the MSH-75 integrator. Review boundary:
`80d6b5cc71c8ceb5bf782d93295797aed1e18d66`, primary `codex/rewrite`.

**Verdict: PASS for the complete selected native rules/history/record contract.
No unresolved material rules defect or required native coverage gap was found
that blocks MSH-41.** This supports original MSH-41 AC1–3, AC6's game-owned
encoding/replay obligations and the native-rule portion of AC8. The complete
MSH-41 verdict still requires the separate consumer, desktop, shared archive
and integrated evidence review. This report closes no task or milestone.

## Method and authority

I read AGENTS.md, the current user attachment, Backlog overview/execution,
MSH-75 and original MSH-41 criteria, the applicable design boundaries, selected
profile, clause/fixture/verification ledgers, native history contract, current
proof resolution and its two independent reviews. I inspected all eight native
shogi source modules, all three external shogi core test modules, current board,
completeness and history corpora, fixture derivations and relevant earlier
state/legality/history/reference-domain reviews. I traced actual native record,
console, search and desktop-session callers for legality/history ownership;
this was an integrated native contract review, not an inference from completed
child tickets.

The authority is approved `mshogi.shogi.csa36.v1`, edition
`JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026`. I did not make a new rule choice or
substitute another competition convention. All ten retained JSA/CSA/USI source
files still match the source manifest's exact SHA-256 values. This checks source
identity; it is not a fresh Japanese expert translation review. The historical
source interpretation and owner selection remain the accepted profile inputs.

Expected facts were distinguished from observations. The 101 board and 68
complete-choice fixtures carry independent derivation/review provenance, rather
than adopting the current native or cshogi outputs as their expected answers.
I independently checked movement/promotion/drop/king-safety logic against that
contract and recalculated all 21 history ledgers and nine declaration diagrams
using reviewer-authored Python SFEN replay, direct attack geometry, exact owner
hand identity and point arithmetic. The audit matches all 149 history prefixes
plus nine declaration roots. It reads neither implementation's outcome as an
oracle. Its result is [retained separately](rules-independent-ledger-audit.json).
It checks fixture facts, not exhaustive local legal membership.

Fresh native execution and fresh admitted-reference comparison corroborate those
facts below. cshogi's parser, `is_legal`, `is_draw`, search moves and entering-king
helpers were not used as the expected-rule or match-outcome authority. No fixture
expectations, product sources, Backlog records or commits were changed by this
reviewer. The CLI is installed at1.55.0 against the1.55.1 managed instruction
baseline; I made no record mutation, leaving compatibility handling with the
integrator.

## Complete selected-profile assessment

| Required behavior | Integrated implementation and independent evidence | Assessment |
| --- | --- | --- |
| Initial setup and identities | `mod.rs` exact40-piece initial SFEN; eight base kinds, owner and promotion separate; SH-F001 exact30 choices. `tests.rs` verifies placement and origin. | Satisfied. |
| Every unpromoted/promoted movement, side direction, blockers | `action.rs::geometrically_reaches`; all central/edge complete sets for both sides in `legal-completeness.json`, plus friendly/enemy bishop/lance/rook blockers and knight jumps. King/gold cannot promote; horse/dragon add only the specified steps. | Satisfied. |
| Captures, demotion, hands and turn | Checked/structural resolution derives victims from state, preserves the mover's base identity, credits each captured base kind to the correct owner and changes turn once. SH-F010 covers all six promoted captures in both directions; SH-F024 nested capture/drop sequences specify final SFEN. | Satisfied. |
| Drops, nifu and dead ranks | Every base hand kind is considered once per target. Resource/empty-square checks, unpromoted creation, last-rank pawn/lance and last-two-rank knight restrictions, and friendly unpromoted-pawn-only nifu are enforced. SH-F011–014 and both-side tests distinguish promoted/enemy pawn contrasts. | Satisfied. |
| Pawn-drop mate and legal defenses | Only the newly dropped pawn's own check triggers the test. An adjacent pawn check has no interposition square: a safe reply must capture that pawn or move the king. All structural replies with full resulting king safety therefore exhaust defenses, without recursive pawn-drop testing. Corpora cover mate, escape, king/piece capture and pinned capture; the independent forced-knight-promotion defense regression retains `4f5h+`. Existing-pawn mate and non-pawn mating drops remain legal. | Satisfied. |
| Optional/mandatory promotion | Both flags are enumerated, filtered by promotable kind, prior promotion, source/destination camp and dead-rank requirements. Entering, inside, leaving, capture and check-evasion choices are preserved for both sides, including nonpromotions omitted by search-pruned engines. | Satisfied. |
| Both-side king safety and no king capture | Geometric attacks include pinned influence and block correctly. Checked apply tests the resolved moving king; king capture/drop is absent. SH-F018–022, x-rays, pins and both-side fixtures exercise this. Broad structural diagrams retain independent native behavior outside cshogi's domain. | Satisfied. |
| Fourfold repetition and checker loss | Exact board ownership/promotion, both owners' hands and turn form identity; counter/origin do not. Root is occurrence0. `game.rs:544` checks all known repeated identities, and `[first..fourth]` check steps cover precisely every move after first through fourth. Checked and check-relieved endpoints, either checker, interrupted checks, a noncheck outside the interval and overlapping repeated identities have independently reconstructed ledgers. A legal action causing checker loss is committed and retained. | Satisfied. |
| No-legal-action loss, mate distinction | `game.rs:519` classifies an empty complete legal set as opponent win, retaining check/noncheck. No chess stalemate draw or king capture is used. Both-side board fixtures and game/record outcome fixtures exercise the semantics. | Satisfied. |
| Entering-king eligibility and formal/false declarations | Native arithmetic counts only own nonking camp pieces plus hand points, majors5/others1 regardless of promotion,10 other camp pieces,28 Sente/27 Gote, king camp, own turn, no check. Untimed time condition is explicitly true. Nine independently recalculated diagrams isolate the thresholds/prerequisites. Queries preserve state; admitted false declarations, including wrong-turn declarations, lose; stale/malformed admission does not. | Satisfied. |
| 320-action cap and precedence | `game.rs:530,541` applies no-action loss, then repetition/checker loss, then length. Imported n320→action320, n321/n322/n999 terminal roots, mate on320 and ordinary/checking repetition on320 are concrete fixtures. A nonmating check does not extend the cap. | Satisfied. |
| Resignation and terminal admission | Either named side may resign; declaration/resignation preserve board/counter and append distinct events. Terminal requests reject; malformed/illegal/stale requests preserve state. Board-action actors are checked separately. | Satisfied. |
| Imported roots and known history | `GameRoot::import` admits only an unattacked nonmoving king, validates full/reduced category and supplied/defaulted counter provenance, rebuilds diagram origin and starts exactly one known occurrence. Same-SFEN played history versus fresh import has distinct contexts/outcomes. Large declared counters do not invent prehistory. | Satisfied. |
| Exact reversal, prefixes and canonical replay | Snapshot undo checks exact complete post-state; game undo includes roots, events, checks, occurrences and conclusion. Prefixes reconstruct through native admission, including same-ply terminal commands. Strict records retain profile/edition/policies/root/history scope/commands/derived events/results/scores; replay checks each prefix and root/final classification. Forged, duplicate, missing, unknown, unsupported and post-terminal records reject. Full320-event and319+terminal-command records fit and replay. | Satisfied for native codec; shared application/archive capacity is separately reviewed. |

Two explicit geometric checks support the no-action boundary independently of a
native result. With Sente K1i and Gote k9a/r2a/r9h, K1i is unchecked but1h is
attacked along rankh and2i/2h along file2, exhausting its three destinations.
At imported n321 this remains a loss before length. For action320 mate, the
existing Sente pawn advances5c→5b, G4c protects5b, and Gote's own lances4a/6a
and pawns4b/6b occupy the other exits of k5a. No defender can capture/block the
adjacent pawn check; this is an allowed pawn move and no-action loss, ahead of
the cap. The native fixtures and replay tests produce those consequences.

## Mutual-check proof and admitted-domain boundary

I reused the already independently reviewed complete-chain and geometry
argument, checked its retained fingerprints, reread the reductions and mixed
radius/source-persistence reasoning, and reran the supplement calculation.
It agrees on23 normalized pairings over20 offsets,46 with reflections, and
side-exchange symmetry. All seven supplement checksums match. This is conditional
necessary geometry after mathematical reductions, not an exhaustive search of
all shogi states or a proof-assistant certificate.

The theorem's actual assumptions are enforced by public native construction:
one king per side, valid promotion, combined stock bounds, admitted nonmoving
king safety and checked transitions. Captures/drops/promotions preserve stock
and the just-moving king's safety. Reduced material and unknown prehistory do
not violate the theorem's domain. Canonical records cannot inject check flags;
they rebuild the root and recompute every observation through `Game::submit`.
Thus the implementation does not exclude a required legal live action under a
new outcome guess. `MutualCheckInvariant` remains an atomic internal diagnostic
with the pending command/context; private corruption tests establish diagnostic
preservation, not a real legal mutual-check example.

## Executed checks and source identity

Environment: macOS27.0.1 arm64, rustc/cargo1.89.0, admitted reference Python3.11.0.
[The verification manifest](rules-verification-manifest.json) records exact
native/tool/fixture/profile/proof/reference SHA-256 values, raw source identity,
revision and fresh report bindings. Current native `game.rs` remains
`b5aaf5cada447bc1e6e57b409714f99af63b3a06706bc0c1cc7a26f2df68726d`.
Current `record.rs` is
`0d80be4a203ea6469dd2e22ea0a8e3e207519f995bac18836f7d3b7f6643431e`;
its added consumer accessors/schema constant do not bypass replay validation.

| Actual command | Result |
| --- | --- |
| `cargo build --locked --release -p mshogi-core --example shogi_probe` | PASS; fresh executable hash in manifest. |
| `CARGO_TARGET_DIR=/private/tmp/msh75-rules-target cargo test --locked -p mshogi-core --lib --test shogi_legal --test shogi_history --test shogi_record` |42 PASS,0 failed/ignored:20 library (including4 chess inspection tests),7 legality,5 history,10 record. |
| `/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py --probe target/release/examples/shogi_probe --seed 20261027 --playouts 1 --plies 8 --report docs/m4/evidence/msh75/rules-board-comparison.json` |101 fixture expectations passed;99 sequences,792 plies,891 prefixes,0 mismatches. |
| Same command with `--fixtures tests/fixtures/shogi/legal-completeness.json` and report `rules-completeness-comparison.json` |68 fixture expectations passed;68 sequences,512 plies,580 prefixes,0 mismatches. |
| Reviewer Python SFEN geometry/identity/point audit, described above |21 ledgers and9 declaration diagrams PASS; no expectations changed. |
| `python3 docs/m4/mutual-check-supplement/verify_geometry.py` |PASS, conditional geometry counts above. |
| `shasum -a 256 -c SHA256SUMS` from supplement directory |All seven PASS. An initial invocation from repository root resolved relative names incorrectly and failed; corrected working-directory check passed. This was a command-location error, not a product/proof-source change. |
| `git diff --check` |PASS. |

The two fresh reference reports total169 concrete fixtures,167 sequences,
1,304 plies and1,471 prefixes. Each run also passes13 reference-domain and4
actual API-gap sentinels. No diagnostic injection demonstration was requested
in these runs; the previous four deliberate failure demonstrations remain
retained historical harness evidence. The exact admitted cshogi revision is
`6985fe5b516c34072e27c63cf429847f03193522`; binary/manifest integrity is checked
by the harness. Neither repetition nor declaration outcomes are inferred from
its helper enums.

The prior [169 depth-two perft/divide comparisons](../../reference/native-perft-comparison.json)
and [4,017-prefix longer comparison evidence](../../legality-validation.md) are
reused, not reported as fresh executions. Their fixture, probe-source, perft and
comparison-tool bytes still match the recorded boundary. Since MSH-66, the
board `action.rs` addition is USI Serde transport; `legal.rs` adds hypothetical
side inspection and tests. The actual movement, structural/checked board
transition and perft algorithms are unchanged. I inspected those diffs, and
fresh current-source comparisons cover the integrated source. Only the initial
900-node depth-two totals/every divide30 were independently hand counted;
other retained perft numbers are differential observations.

## Findings, limitations and handoff

There are no unresolved material findings with a reproducible native rules or
record defect. Required selected-profile conclusions have not been relabeled
optional or deferred to strength work. Unselected JSA conventions, timed shogi,
retrospective officiating and competitive strength remain outside the approved
initial untimed profile; their absence is not an omitted selected rule.

The historical coverage ledger still contains earlier-stage statements that
MSH-71 onward remains future work. Those statements should be reconciled by the
MSH-75 integrator with the complete consumer and application evidence; they
are not a source-level rule failure. This review itself does not establish
shared save admission, UI/accessibility behavior, real opponent execution,
three-platform execution, strength or M4/all-game closure. Those stay with the
parallel independent tracks and integrator. No MSH-74 aesthetic work is reopened.
