# Standard-shogi complete-delivery review — MSH-75

2026-10-27. Reviewed and freshly tested integration:
`80d6b5cc71c8ceb5bf782d93295797aed1e18d66` on `codex/rewrite`.
This task changes evidence, coverage and completion records only. It makes no
production, fixture-expectation, profile or presentation change.

**Delivery assessment: all eight original MSH-41 acceptance criteria are
supported.** Three fresh independent AI review tracks found no material defect.
The final acceptance reconciliation is independently reviewed in
[the closure review](evidence/msh75/closure-review.md). The native Backlog records
record the accepted disposition and integration reference; this document is its
evidence mapping, not a replacement task system. M2/M3 remain closed. M4 and
MSH-42–47 remain open; no other game was implemented by this review.

## Original acceptance mapping

| MSH-41 | Integrated obligation and assessment | Independent assessment and executed evidence |
| --- | --- | --- |
| AC1 | Complete sourced immutable `mshogi.shogi.csa36.v1`, edition `JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026`, owner-selected SHOGI-P1. Every required clause has implementation and concrete fixtures; no required rule is deferred. | [Selected profile](shogi-profile.md), [clause ledger](shogi-coverage.md), [independent rules review](evidence/msh75/rules-review.md). Retained ten primary-source fingerprints match. Independent history/point/geometry audit corroborates the accepted fixture facts; historical source interpretation and selection are preserved. |
| AC2 | Native shogi owns positions, complete moves/drops, derived captures/demotion, checked admission, history, terminal commands, exact reversal and distinct repetition/record/execution identities. Native session serializes clients; frontend has no second rules engine. | Rules review plus [archive/session review](evidence/msh75/archive-desktop-review.md); current checked-transition, history, exact-undo, command-context, cross-game/stale-result tests. Advisory eligibility never becomes a committed declaration merely by inspection. |
| AC3 | Independently expected legal/illegal choices, transitions and outcomes, sequence/reversal/record tests, admitted reference and perft/divide evidence. Independent expectations are not replaced by either implementation's observations. | 101 board +68 completeness fixtures; fresh admitted cshogi run: 169 fixtures,167 sequences,1,304 plies/1,471 prefixes, zero mismatches. Fresh independent 21-ledger/149-prefix and nine-declaration-diagram audit. Reused 169 depth-two perft/divides and longer4,017-prefix evidence with exact algorithm/tool applicability; only start depth2=900 and each divide30 were hand-derived. [Manifest](evidence/msh75/rules-verification-manifest.json), [validation](legality-validation.md). |
| AC4 | Standalone game-selected console, full choices/apply/undo/history/outcomes, human inspection, strict machine records and honest position-only diagnostics. | [Engine/console review](evidence/msh75/engine-console-review.md); fresh actual `mshogi --game shogi shell` capture/demotion/promotion, same-ply resignation, Save/Load/Undo/re-export, duplicate rejection and no-clobber journey. MSH-90 common console behavior is exercised, not inferred from child status. |
| AC5 | Own functional native search uses complete native history/legal choices and actual winner; declarations are terminal choices, not alternating moves. Supervised structured execution retains purpose, full context, identities, limits, cancellation and retirement. Real USI/selfplay/opponent consumer exists. | Fresh68 focused engine/console/runner tests, actual structured child (including continuous-check winner), six selfplay plus six pinned YaneuraOu games and exact replay, no failures. Two capped initial games per report remain unscored; decisive fixtures are artificial functional tests. [Actual reports and provenance](evidence/msh75/engine-console-review.md). No strength claim. |
| AC6 | Native canonical profile/edition/root/counter/known-history/actions/policies/events/conclusions survive strict replay; shared v2 archive preserves desktop settings, observations, control journal and document semantics. Duplicate fields reach strict native parsers intact. | Fresh native record/strict subprocess tests; 23 focused archive/capacity tests; fresh real normal Save/Open records below. 256→257 is rejected before accepted mutation; byte/control/replay-work/native-event bounds preserve complete Save/Save As and reserved stop/close. [Capacity measurements and source audit](evidence/msh75/archive-desktop-review.md). Common migration remains one shared owner under MSH-46, implemented for current chess/shogi by MSH-73. |
| AC7 | Normal product supports untimed human/engine play for both sides, complete move/drop/promotion, native outcomes/recovery, read-only inspection/analysis, Save/Open and exact history/Return/Resume. Full9×9 fits supported minimum760×650. | Fresh normal macOS journey, native/host/frontend regressions, owner-approved MSH-74 shell; source-equivalent actual Windows/Linux normal and separately labeled verification GUI evidence. [Fresh normal journey](evidence/msh75/macos/README.md), [platform applicability audit](evidence/msh75/archive-desktop-review.md). Complete platform acceptance for all six games remains MSH-88/46/47. |
| AC8 | Complete clause ledger and independent integrated review; required behavior remains in M4. Findings and remaining common integration obligations stay visible; aggregate child status alone does not establish delivery. | Three fresh integrated boundary reviews plus final reconciliation; exact evidence categories, limitations, source equivalence, command results and CI below. Required shared MSH-69/72/73 and console follow-up90 are integrated. MSH-46/47 retain all future games and final shared closure criteria. |

## Complete profile clause disposition

The [stable clause-to-fixture ledger](shogi-coverage.md) and independent rules
review jointly trace every selected clause; SH-F001–055 are scope identities,
not a count of tests. Board001–024 cover both sides, all base/promoted movement,
blockers, king safety/pins, optional/forced promotion including evasions,
capture/demotion/hands, all drop kinds, nifu, dead ranks and pawn-drop mate with
legal king/piece defenses. Checked transitions reject atomically and undo exactly.

History025–044 covers four *known* occurrences, every own check between first
and fourth, either checker losing, interrupted/overlapping intervals, no-action
loss with/without check, own-turn eligibility queries versus formal declarations,
28/27 thresholds, false declarations, resignation, imported counters/history,
and loss/repetition before the320-completed-ply limit. Exact record and historical
prefix045–048 and real consumer049–055 obligations are exercised by the console,
structured engine, USI, runner, session, archive and normal desktop paths above.
Same board with different history, and terminal events at unchanged board ply,
remain distinct. Imported roots never acquire invented earlier occurrences.

The mutual-check obligation is discharged by the accepted complete-chain proof
and admitted-root domain, not a guessed winner or silently excluded legal action.
Fresh review reran23 conditional geometry pairings and all seven supplement
fingerprints. This is a mathematical reduction with finite conditional geometry,
not exhaustive state enumeration or a proof-assistant certificate.

JSA24/31-point professional conventions, replay/agreement alternatives, timed
eligibility, handicaps, tsume/try-rule and physical/retrospective officiating are
explicitly outside this selected profile. None is presented as missing required
CSA36 behavior or deferred selected-profile correctness.

## Fresh final verification and provenance

[Machine-readable final verification](evidence/msh75/final-verification.json)
records source, environment, commands, exact counts, raw-log fingerprints and CI.
macOS27.0.1 arm64; workspace Rust1.89; native host Rust1.90; Node24.19.

| Check | Actual outcome at reviewed source |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --locked --workspace --all-targets --all-features` |409 passed,0 failed,2 ignored across56 result groups. The two optional deep **chess** release perft/sequence checks were not rerun; existing evidence remains. Includes real console/protocol/subprocess and desktop-session tests. |
| `cd desktop && npm run check && npm test` | Strict TS PASS;161 tests/21 files PASS. |
| Independent focused native/consumer/archive/inspection checks |42 native rules,68 engine/console/runner,44 archive/session/inspection tests PASS; overlap with workspace is not additional unique coverage. |
| Independent product frontend and Tauri host |66 focused product tests and strict TS PASS;8 all-feature host tests PASS using explicit Rust1.90. |
| Fresh normal Mac package smoke + native CUA GUI | PASS, separate categories; package protocol smoke includes chess compatibility. Real Shogi GUI uses bundled native engine and native file dialogs. |
| [Rust CI40505357243](https://github.com/jweathers777/mShogi/actions/runs/40505357243) | Windows2022/macOS15/Ubuntu24.04 all success at80d6b5c. [Original metadata](evidence/msh75/rust-ci.json). |
| [Desktop CI40505357155](https://github.com/jweathers777/mShogi/actions/runs/40505357155) | Same three platforms all success at80d6b5c. [Original metadata](evidence/msh75/desktop-ci.json). |

The retained final MSH-74 Windows/Linux interaction source is
`1b10277436b94541235e15189a3af7fce867351c`; production, locks, package settings,
workflows and driver bytes have no delta to80d6b5c. Independent artifact audit
rehashes175 retained originals (73Linux,71Windows,31Mac). Each Windows/Linux
report has14 successful phases: **3 normal and11 separately labeled verification**,
not14 normal phases. All16 read-only inspection comparisons per platform preserve
complete state; native close cleanup is checked before driver teardown.
The fresh normal Mac host/engine hashes exactly match the final accepted package.
No new Windows/Linux GUI or external-opponent platform claim is made by this task.

Earlier failed/cancelled CI, wrong reference-domain diagrams, command invocation
mistakes and pre-fix runner retirement reports retain their historical outcomes.
Successful current evidence does not rewrite them. The Tao native-termination
patch's provenance and regression remain unchanged. Normal package inventory
excludes verification instrumentation; the retired MSH-72 harness is not the
product or owner-approved visual evidence.

## Findings and disposition

No material correctness, rule completeness, losslessness, interoperability,
accessibility or product-contract blocker was found. No production task was
reopened for appearance. The only reconciliation finding was stale planning-era
future-tense coverage text; current summaries now point to this complete review,
while historical planning and earlier reviews remain labeled and preserved.
There is no unresolved required clause and no new repair/strength task masquerading
as closure evidence. Independent reviews identify their actual methods; AI review,
independently expected fixture facts, differential observations, native tests,
actual processes/opponent games, packaged GUI and owner approval remain distinct.

### Post-integration CI follow-up

The evidence integration is `4c4145eaacf7fcaf89a338773aa110a3700e4c75`;
production/expected-fixture sources remain identical to80d6b5c. Its
[native CI40512188338](https://github.com/jweathers777/mShogi/actions/runs/40512188338)
passes all three platforms; [original metadata](evidence/msh75/integrated-rust-ci.json)
is retained. Desktop40512188317 attempt1 passes macOS/Windows, while Linux
passes13/14 phases and loses the response to its first Chess verification pointer
request. All normal-product and Shogi phases pass. The same session subsequently
returns the zero-ply application image and DOM; no product panic is evidenced.
The [independent incident review](evidence/msh75/ci-followup-review.md) distinguishes
the observed WebDriver peer reset from an unproven internal root cause.
[Original failed report and captures](evidence/msh75/ci-linux-attempt1/manifest.json)
remain failed evidence. No ambiguous pointer request is automatically retried,
no assertion is weakened, and no production fix is inferred. Only the Linux job
was rerun unchanged. Attempt2's [actual report](evidence/msh75/ci-linux-attempt2/gui-report.json)
passes14/14, including the complete genuine missing-engine/Retry/native-Quit
sequence that attempt1 did not reach. Independent audit verifies all223 retained
artifact fingerprints,16 full inspection comparisons and14 native absence
observations; host/engine hashes match attempt1. Its exact artifact ZIP is pinned
by ID12739498661 and SHA-256 in [the manifest](evidence/msh75/ci-linux-attempt2/manifest.json),
avoiding the two attempts' duplicate artifact names. The rerun's approximately
12-minute dependency-installation delay happened before application checks.
GitHub temporarily retained an in-progress overall status despite completed
successful steps; [the observation](evidence/msh75/ci-linux-attempt2/completed-steps-awaiting-status.json)
is kept separately. The final workflow concludes success; actual integration CI
and its three-platform results are retained in
[the final metadata](evidence/msh75/integrated-desktop-ci.json).

## Limits and next boundary

This is complete delivery of the selected initial **untimed** profile with a
functional material-only full-width engine, not a competitive-strength result.
Depth1–4/node0–20,000 search and its measured default2/5,000 remain explicit.
USI supports the documented finite subset: no clocks/ponder/searchmoves/tsume;
no completed iteration means incomplete/unscored, never an invented resignation.
Shogi does not inherit chess Full reachability. External-opponent execution here
is macOS only. Standard-start smokes and artificial decisive roots are not fair
match-strength measurements; MSH-50 owns those future criteria.

Persistence is bounded and lossless: precommit admission can refuse further work,
without discarding accepted events/observations or promising an impossible retry.
Desktop remains untimed, without variation editing, signed distribution or formal
screen-reader certification. Windows large native screenshots have the retained
virtual-desktop clipping caveat; complete client/minimum captures establish fit.
Linux GUI uses X11/Xvfb, not Wayland. Shared presentation introduces no mandatory
scroll/zoom/pan policy for future12×12/16×16 games.

Next dependency-ready M4 research is MSH-80 xiangqi (provisional next game),
MSH-81 tori, MSH-82 chu and MSH-83 tenjiku. Their source/owner decisions and native
refinement must precede implementation. MSH-42–47,76–89 keep their remaining
obligations; M4 stays open. This review starts none of that work.

Launch the existing normal macOS bundle with:

```sh
open /Users/johnweathers/Development/Personal/mShogi/desktop/src-tauri/target/release/bundle/macos/mShogi.app
```

Choose **New game → Shogi → Sente/Gote → Start game**. Opened games pause until
**Resume play**. Rebuild instructions remain in [desktop README](../../desktop/README.md).
