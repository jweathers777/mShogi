# Main-study freeze review — MSH-161

Reviewer: execution_review; 2026-11-11. This records a practical qualification
recommendation and mechanical phase implementation. No engines ran during review.
It is not a final diagnosis or a claim that the main200-game study has completed.

## Opponent selection

Selecting **Oxidation0.6.0** is justified for the main study. On the four shared,
contrasting prepared roots with paired colors, mShogi scored1W/4D/3L (37.5%)
against Oxidation and4W/4D/0L (75%) against Goldfish. Oxidation supplies the more
useful demonstrated resistance while leaving draws and winning opportunities;
this is preferable to the earlier unaided-start sample alone. Eight games per
reference do not statistically establish population superiority, but such proof
is not required to make this practical fixed-reference selection. Report the
observed qualification conditions/sample and uncertainty without a rating claim.
Freeze Oxidation's existing source/binary/options, both participants' memory
policies and5-second practical move conditions. Keep all20 qualification games
separate from main-study results; do not switch opponents within the main sample.

The blind gameplay review remains an independent check of whether these games
provide recoverable positions and informative resistance. No mechanism-specific
engineering explanation is needed for this choice.

## Mechanical phases

`phase_tags.tag_history(initial_fen, history, history_origin='standard_start')`
returns positions indexed by ply: row p is the board **before** decision p;
row N is the final board. CLI `phase_tags.py REPORT OUTPUT_JSON` consumes the
canonical native histories, preserving report/script hashes and python-chess
version. Imported/unknown origins retain unresolved original-identity status;
no early history is invented.

Original starting minor identities are tracked through moves, captures and returns.
A captured minor is no longer awaiting development; a returned original piece on
its original home square counts as home. King disposition is mechanical:
castled in known history, no remaining castling rights, or both queens gone.
Opening boundary is the **first** of three consecutive full-move boundaries where
both sides have at most one original minor at home and both kings have resolved
disposition. Confirmation uses subsequent history; a short final tail remains
unresolved instead of being silently shortened. Per-position material<=26 using
Q9/R5/B3/N3, excluding pawns/kings, takes endgame precedence. Promotions can change
that material tag. None of these indicators asserts sound development or king
safety, and they are not production evaluation/rules changes.

Verification on macOS arm64 with python-chess1.11.2:
`/private/tmp/msh139/venv/bin/python -m unittest discover -s docs/m5/evidence/msh161 -p test_phase_tags.py -v`
passed7 offline tests. They cover three-boundary confirmation/tail refusal,
original-minor return resetting the pending boundary, captured identity,
king-rights disposition without development, the material threshold/precedence,
queenless/unknown histories and illegal-history refusal/input preservation.

All16 prepared qualification histories independently tagged successfully with
N+1 positions. Mechanical opening boundaries varied from ply16 to48; this is not
a fixed-move rule. Descriptive position counts (including prepared prefixes) were
372 opening,616 middlegame and1005 endgame; these are implementation checks, not
performance findings or candidate-only denominators. Late/unusual boundaries need
reasoned reviewer attention and boundary-sensitive event reporting, not silent
post-result relabeling. Prepared plies must stay excluded from competitor errors.

Source SHA256 `phase_tags.py`:
`51f687ff4c4c9fbfa3e39b06720145cc8f2bda84655ff2a99643aa2bc8ab1f7b`.
Test SHA256:
`03c7c3e8248de0bd041858e96d1e870883cb09d0eb39c0b2dacbf679bd50bc90`.
The parent should inspect this isolated new helper before relying on its tags;
this reviewer authored it and does not claim independent source review of it.

## Main procedure gates

The proposed main batch's20-game checkpoints and all-move500k-node coverage,
followed by5M-node matched singleton played/alternative searches for every v1
uncertain/negative/threshold row, address the pilot's selective missingness.
Stable nonflagged10% hash-selected auditing should stratify by game/player/phase
and include at least one per game. Preserve every row/disposition and all raw
v0/v1 interpretations. Initial-PV alternatives are a controlled comparison set,
not proof of global optimality; unresolved or changing alternatives need discovery
and stronger comparisons according to the frozen procedure.

Final consequential classification still requires batch review,20M-node escalation
where specified, legal continuation checks and retained uncertainty. Favorable
phase crossings, preservation/conversion episodes and strong-play examples require
their explicit follow-up; baseline500k analysis alone does not satisfy those
confirmation obligations. These later checks need not prevent launching the
reviewed main batch, provided they remain mandatory completion gates.

The final90-root balance and structural-coverage manifest is pending its focused
screens at this review checkpoint. Freeze it and the exact schedule before main
execution, preserve rejects/replacements and the separate20 unaided-start games,
and retain the phase-boundary convention plus ambiguity policy. Do not claim
this review clears unseen final balance results or the still-pending blind gameplay
assessment. No new production changes or competitive rating study are authorized.

## Final admission and launch clearance

The pending source/admission and blind-review gates above are now satisfied for
launch. Read `qualification-final-review.md`: its independent complete-game review
supports Oxidation's practical resistance across three families/both candidate
colors while retaining the level combined start/prepared score and imperfect
opponent play. No statistical superiority proof or rating claim is substituted.

Independently audited the final90 selected roots:

- 90 unique complete prefixes and22 families with at least four roots each;
- 16 exact pawn-symmetric,12 opposite-castled,15 closed and30 open centers,
 independently recomputed from boards/history;
- No qualification overlap by ID **or canonical board/turn/rights/legal-EP key**;
- Every selected source/FEN/prefix and exact source archive ordinal, headers and
 full game move list match; all reference PVs independently replay legally;
- Every chosen reference receipt hash, complete/exit/settled/untruncated/legal status,
 aligned nonbound cp/E window, distinct-depth stability and absence of conflicting
 final bound pass the frozen gates. Remaining stale-final-report flags are retained.

A disposable-directory execution of `prepare_main.py` independently produced10
schedules, each nine prepared pairs plus one unaided pair. All90 prepared roots
are consumed once, all references are Oxidation, and all schedule hashes match.
The actual `main-plan-checks.json` likewise records100 pairs/200 games and no
engine execution during preparation. No real schedule or frozen helper was edited
by this independent check.

Reviewed `main_batch.py`: all native games precede reference work under existing
helper locks; both-player played rows are eligible for the complete v1 follow-up
queue, stable nonflagged cells receive the fixed-hash audit, identical singleton
moves are deduplicated and missing alternatives remain unresolved in adjudication
metadata. Confirmation outputs explicitly do not certify events. Frozen file and
all schedule hashes are checked before each command. Native history origins now
reach the phase helper explicitly. Shared-host load is recorded without claiming
isolated throughput. Failures stop with retained stage/query evidence; future
20M consequential adjudication and episode/strong-play follow-up remain required.

Final frozen identities:

| Artifact | SHA256 |
| --- | --- |
| main-freeze.json | `67f923d6dda70e156a306279b3746ee223565d9c1bd5d8a805f6be7404cddff8` |
| main-method.md | `4aff77f5c0b5fb38dcae50528aa32573929067ab3689db78c7af4085fa0ebc7d` |
| main_batch.py | `495e7d294dd1d13b40e7f21365f2b88e9b9df165a066af94c911d48d42a3f2e4` |
| prepare_main.py | `3934381e7e733f631106a48abc26a0e694ee9073cecab0a1488946b4beea7fa1` |
| sources/main-roots.json | `8e2defc63c839d215ccbd4f1f667b777bfdfced74ef44a90a6698a8cca8de332` |
| sources/main-selection.json | `6e50ef5f46c7e782d32be6149dcbe0d14fc81af2b2cca311d46fba4a22bd8f4b` |

**No blocker remains to the first20-game main batch and its unattended analysis.**
This clears execution of the frozen method, not final findings, weakness rankings,
methodological saturation or engineering changes. Review the completed batch
before continuing the next scheduled batch; preserve the full planned coverage.
