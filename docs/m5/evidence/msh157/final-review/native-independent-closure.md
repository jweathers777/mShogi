# Final independent closure review

MSH-156 (Independent Chess Benchmark) / MSH-157 (Conventional Decision Improvement),2026-11-10.

**Approved for normal integration and closure of this bounded engineering round. The broad decision-improvement gate failed; this approval does not claim that substantially stronger Chess was achieved.** The native exact-cost fixes are justified on their own correctness/performance evidence. No further experiment, production change or tournament is needed to make the present result honest and reviewable.

## Scope and independence

Reviewer did not author the native changes, benchmark, runner, investigation, final comparisons or report. Earlier independent reviews covered the native diff/tests, cost evidence, reference drivers, matched-depth extension and stopping decision. Held-out positions/results did not inform this reviewer's production recommendations; final aggregate comparisons and confirmation records were inspected only after candidate freeze and completion. No engine search was run by this reviewer.

Reviewed final `docs/m5/chess-decision-round.md`, forward roadmap, comparison summaries, quiet confirmation, final reference findings/receipts, historical-exposure disclosure, consolidated audit/resource records and packaged evidence. Current native source/test hashes still match the reviewed passing local checkpoint, and Git diff against production3b52314 contains no subsequent Chess core/search change.

## Conclusions supported by the evidence

- The two native fixes preserve complete authoritative history and correct claim ordering/admission while removing demonstrated repeated work. Reviewed local checks passed;99 fixed-depth comparisons preserve decisions/scores/visits/depth. Selected affected histories gain3.86x/4.05x own visits at one second, with unchanged moves. The ordinary control's1.04x difference is appropriately treated cautiously. These are mechanism-specific cost observations, not general strength estimates.
- Original primary results contain158/160 unchanged development choices and80/80 unchanged held-out choices. The secondary40-root one-second screen has39 unchanged choices. Exact root objectives remain preserved; identical moves are not presented as universally correct. The primary broad-decision gate is unmet regardless of the interpretation of two development changes.
- Independently checked all four final reference records' artifact hashes and bestmove-aligned nonbound scores. OpenFiles Qb4+0cp versus Bb1+117cp, with the documented concrete continuation contrast, supports a local finite positional improvement. Outposts Rh8+17cp versus Ng5+48cp remains a small preference of unresolved practical significance. Neither establishes exact WDL, broad transfer or match strength. Source-tier automated grades remain unresolved, with adjudication separated rather than rewriting expected answers.
- The coverage investigation is a useful bounded negative/inconclusive localization result. Matched-depth controls expose horizon confounding; fresh-child tests disclose TT/window/order/check-extension differences. No safe general pruning or evaluator correction was established. The report avoids turning root/PV agreement or a score gap into causal proof. Keeping MSH-153 (Classical Compensation Calibration) held and declining a competitive campaign are proportionate.

## Timing, exposure and resource limitations

The historical-exposure audit overlapped19 held-out roots. Selection of their repeat was timestamp-based and frozen independently of repeat outcomes. All38 repeat choices match their original version choices; all19 version pairs are unchanged. This supports decision consistency but does not erase original work/depth timing uncertainty, replace the original primary sample or add independent validation cases. The report explicitly preserves that distinction.

The exposure audit identifies a conservative previously seen development board-family of two roots, with different side/counter context; both remain in development. Removing the family leaves the same two changed strategic decisions and all80 held-out decisions unchanged. The zero held-out-match statement is scoped to the audited files/encodings; excluded GUI/snapshot material and unknown latent families prevent a universal historical-novelty claim. The disclosed incomplete offline CPU accounting remains a real limitation, not silently inferred success:229.109known CPU seconds is a lower bound.

The consolidated engine audit records686 attempts, one retained/charged invalid-result cost attempt,96 source hashes and5,343 original artifact hashes. Actual active engine use2705.533657seconds and effective reservation charge2847.376833seconds remain below3300; recorded search intervals do not overlap. Offline data/audit work, builds and correctness tests are distinguished from this fully ledgered engine allowance. No matches or cross-platform/GUI success are invented.

## Package verification and final disposition

Independently verified the decision archive hash, all4,516 stored entry hashes, unique safe relative archive paths and exact manifest membership. There are851 explicitly omitted source artifacts and392 transformed entries. README/manifest clearly distinguish original execution fingerprints from archived bytes, especially STS source/reference-only treatment; they do not pretend omitted upstream inputs or redacted FEN fields were preserved. Report and roadmap relative links resolve. Original/frozen source identities, failed attempts, reconstruction paths and review records support resumption.

No unresolved material review blocker remains. Normal local commit/integration/push may proceed. The final summary should lead with the failed broad-decision gate alongside the useful exact cost repair and new independent measurement base. Further conventional decision-quality work should start from a concrete revised causal question, not an automatic sequence of bonuses or pruning changes. No learned evaluation, cross-game transfer or substantial competitive campaign is authorized by this closure.
