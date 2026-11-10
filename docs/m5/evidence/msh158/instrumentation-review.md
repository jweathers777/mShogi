# MSH-158 (Chess Decision Corrections): temporary instrumentation review

Independent reviewer: native_independent_review. Scope: current temporary diff in chess_conventional.rs and chess_frontier_profile.rs; read owner instruction attachment a1c29243. No engine searches, builds, tests or held-out data reads performed by reviewer.

Verdict: no search decision-logic blocker found by source review; suitable for bounded diagnostics with limitations below and author validation. This is not approval of a search correction or evidence of decision improvement.

The normal configured API supplies None; traced API calls the same engine. Evaluation computes the same value once and only formats its components for selected events. TT cutoff values, move selection, reduction/re-search conditions, frontier admission, and native transitions are unchanged. Successful native push/preview/null transitions append the corresponding action (None for null), while existing pop restores trace path along with evaluation/native state. Invocation wrappers leave after both successful and error Results; ordinary cancellation and null error recovery retain cleanup. No new poll, native projection, or rule decision appears in the tracing path.

Interpretation requirements:

- Event node is the active main/q invocation, not a unique position identity. Child-search/child-return and pushed-pruning sites have the caller invocation ID but the currently pushed child FEN/path. Entry events establish invocation positions; do not join arbitrary event FENs as though invariant within node.
- Filters match action paths after flattening null markers. Null descendants can match ordinary move-prefix filters, but path None explicitly identifies the synthetic branch. They are not legal real continuations.
- q_generated with all_legal=false lists only captures/promotions; absence of a quiet action there is frontier exclusion, not illegality. Native legal replay from the event FEN is necessary to establish which quiet reply was available. q_quiet_excluded covers only explicitly projected quiet candidates.
- Return bound labels describe the returned value against the caller's original window, not a proof of the fully searched game-theoretic value. TT-cutoff events retain stored bound and effective window. Selective frontier and unfinished iterations remain relevant.
- Missing events in a capped trace (dropped > 0), outside selected prefixes, or before min_iteration cannot establish that an action was not searched. Main-to-q wrappers may create two invocation IDs at the same FEN/path.
- Trace collection, path maintenance, FEN/evaluation formatting and wrapper overhead change equal-time coverage. Traced 5s/15s are diagnostics, and require trace-off comparisons. A fixed-depth/node comparison excluding elapsed timing is the appropriate semantic-equivalence check.

Requested author checks: fixed-work trace-on/off result/statistics equality; cap/filter behavior; cancellation leaves collector stacks empty; null tagging. Parent owns resource accounting and experimental findings. Minor interface limitations (public mutable cap, unchecked diagnostic input narrowing) do not block trusted bounded experiment inputs but should not be represented as a hardened public diagnostics protocol.
