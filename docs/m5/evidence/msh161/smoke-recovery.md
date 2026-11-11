# Qualification checkpoint and reference gate recovery

Four qualification games completed normally and passed native replay. Candidate
won both Oxidation games and scored one win/one draw against Goldfish. These are
unaided starting-position observations, not proof of current relative strength.
Neither opponent yet meets the intended stronger-opponent evidence requirement;
review all-move analysis then qualify diverse prepared positions before selection.
Match wall3094.519s, childCPU3094.481s, response-duration3093.714s (includes transport).

The reference smoke ran two successful queries but the pipeline stopped because
it demanded zero finite-analysis uncertainty. Unrestricted Stockfish ended with
a depth20 upper bound, retaining an aligned depth19 nonbound score and explicit
last_score_update_not_selected_exact_score. That is a valid uncertain estimate,
not failed transport. Forced actual-move query was aligned/nonbound.

Independent execution_review approved permitting only that specific uncertainty
at the operational smoke gate. All legality, nonbound/alignment,WDL,history,
node-limit,raw-truncation and cleanup checks remain. Comparison/summarizer unchanged:
uncertain results retain null loss and cannot become severity findings.16 offline
tests pass, including real-receipt uncertainty and negative mutations.

Original failed-chain status/logs preserved under attempt0- names. Existing smoke
receipts reused without repeating engine work. Game outcomes and initial analysis
method unchanged; no production source changed. Full reference analysis restarts
unattended, followed by source balance screening. Smoke wall1.571s/chain1.801s
remains recorded in original receipts and is not reset out of resource accounting.
