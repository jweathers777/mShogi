# M2 first bounded batch review

Scope: MSH-15 (match specification), MSH-24 (boundary hardening), MSH-19 (static material evaluation), plus requested M2 task refinement. Base: integrated M1 revision `c4ed581`. This is not M2 closure; MSH-25 owns actual game/search/cancellation/UCI/runner integration and native platform closure evidence.

Independent read-only agents reviewed distinct bounded changes on `codex/m2-baseline` on 2026-10-24:

| Task | Reviewer | Findings and resolution |
| --- | --- | --- |
| MSH-15 | `review_profile` | Clarified intended claims at **at least** occurrence three; added explicit-only/auto-claim pairs and prospective mate claim; corrected fourth-occurrence current-claim expectations and policy activation after history reconstruction. Preparation also validates clock paths and mating witnesses. Follow-up confirmed no remaining material profile/planning finding. |
| MSH-24 | `review_hardening` | Found old Display-substring matching in sequence properties; replaced with `FenErrorReason` variants. Follow-up confirmed resolution and no remaining material finding. Reviewer independently ran 15 console and 9 FEN tests. |
| MSH-19 | `review_evaluator` | No material findings. Independently ran five integration tests and one doctest. Verified static-only boundary, signs, reflection, values and absence of strength claims. |

Local host: macOS 27.0.1 (26A434), arm64, Rust/Cargo 1.89.0. Formatting, workspace Clippy with `-D warnings`, locked workspace tests and build passed. Focused profile preparation and sequence checks passed after review corrections. The source commits and actual integrated CI revision/run/platform outcomes are preserved in each task's completion notes; configuration alone is not evidence.

The M1 deep perft/differential/property evidence remains applicable to unchanged rules. Existing opt-in deep tests were not rerun. Fixture preparation validates inputs and move paths; it does not implement future adjudication. Material scoring does not implement search. Full dead-position and side-specific possible-mate completeness remains MSH-26 (M4), and unknown timeout/resignation feasibility is unscored. No claim of complete chess, tournament compliance or playing strength is made.
