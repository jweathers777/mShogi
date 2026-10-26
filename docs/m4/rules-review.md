# MSH-62 independent rules and reference planning review

Date: 2026-10-26. Reviewer: fresh independent Codex review agent, separate from
the planning author. Worktree: `next-task`, branch `codex/m4-planning`, base
`2b9849c`. Reviewed the author's working planning documents before integration;
this report does not assert a future commit identity.

**Verdict: PASS for the bounded rules/reference planning scope. No material
findings remain.** This is not profile approval, fixture certification, runtime
reference admission, shogi conformance or M4 delivery closure. SHOGI-P1 remains
an owner decision in MSH-63; MSH-41–47 remain open.

## Scope and method

Read `AGENTS.md`, relevant design boundaries, MSH-62, and the complete
`shogi-profile.md`, `shogi-verification.md`, `shogi-fixtures.md` and
`later-game-sources.md`. Checked primary Japanese JSA and CSA texts through web
retrieval, and inspected the actual pinned cshogi and YaneuraOu source checkouts.
Spot-checked the later-game primary texts described below. Architecture/task-DAG
review is a separate review scope.

Read-only commands included `backlog instructions overview`, `backlog task view
MSH-62 --plain`, `git status --short --branch`, source `sed`/`rg`, and each
reference checkout's `git rev-parse HEAD`. A small inventory check found exactly
55 unique contiguous table-row IDs, SH-F001–SH-F055. No product code, task record
or author document was edited by this reviewer. No reference builds, external
matches, game implementation, Rust/frontend tests, packaged application checks
or M3 audits were run.

## Authority and complete-profile assessment

The recommendation explicitly allocates JSA board rules, CSA event conclusions
and separately identified software adaptations. It does not present the blend
as JSA professional match procedure. One named/versioned proposed bundle is
reviewable; alternatives do not silently become required first-batch features.

The [JSA PDF](https://www.shogi.or.jp/match/taikyoku_rules/taikyoku_rules.pdf)
footer confirms the recorded enactment/revision/correction dates. Its Arts.8–10
support the proposal's distinction between repetition/replay, professional
declaration and foul procedures. The [correction notice](https://www.shogi.or.jp/match/taikyoku_rules/taikyoku_rules_revision.pdf)
corrects the declaration cross-reference without replacing its thresholds.
The contextual 500-ply question remains expressly conditional on selecting the
JSA alternative; it is not imported into the CSA proposal.

The [CSA36 PDF](https://www.computer-shogi.org/wcsc36/rule.pdf), linked by the
[event index](https://www.computer-shogi.org/wcsc36/menu.html), has a 2023-11-27
revision/effective date. The proposal honestly distinguishes that edition from
the 2026 event. Arts.22 and 25–27 support the stated 28/27 declaration conditions
and false-declaration loss, no-action loss, final repetition draw, continuous
check exception, 320-ply draw and loss precedence. Competition scoring is kept
separate from game reason and JSA replay series.

Movement, capture/demotion, promotion choices, mandatory promotion, hands/drops,
nifu, uchifuzume, king safety, mate and non-board conclusions are all covered.
The [JSA movement explanation](https://www.shogi.or.jp/knowledge/shogi/02.php)
and [prohibited-play explanation](https://www.shogi.or.jp/knowledge/shogi/05.php)
support the cited educational clarifications. Ordinary invalid API requests
reject atomically; formal admitted declaration and runner foul events have
separate semantics. Untimed play, imported counters and unknown pre-root history
are identified software proposals within the single SHOGI-P1 bundle. These do
not create unnecessary representation approvals.

## Exact reference-operation assessment

Both local checkout HEADs match the documented full revisions. The cshogi
checkout is `6985fe5b516c34072e27c63cf429847f03193522`, version 1.1.0 in
`setup.py`; its license/header treatment and developer-only proposal are
accurately described. Source inspection corroborates the operation restrictions:

| Operation | Independent audit conclusion |
| --- | --- |
| Python `legal_moves` | `_cshogi.pyx` and `cshogi.h` construct `MoveList<LegalAll>`. The actual generation branches retain legal pawn/lance/knight/bishop/rook promotion alternatives, including evasions. Search-pruned `Legal` is correctly excluded as a substitute. |
| King safety/drop legality | Generator and position helpers support the documented bounded board comparison. Complete legal choices still need actual sentinel admission and independent expected fixtures. |
| `is_legal` / USI parsing / push | `moveIsPseudoLegal<false>` skips extra promotion validation gated on `Searching`; drop checks are not a complete hostile-input validator. Parser acceptance is correctly forbidden as the negative oracle. Push only a matching enumerated LegalAll action after independent grammar validation. |
| SFEN / `is_ok()` | Loading initializes state and clears history but is not full structural validation or reachability proof. `isOK()` permits a missing king. The proposed prevalidation and explicit admission categories are necessary and present. |
| Push/pop | Wrapper and state routines preserve the comparison-relevant board, hands, side, counter and history chain. The plan requires runtime snapshots and exact reversal; source reading does not claim those runs passed. |
| Repetition helpers | `isDraw` returns on its first earlier matching key, with search-oriented hand superiority and check-counter categories. `moveIsDraw` differs in bounds/orientation. Neither implements governing four-occurrence adjudication. The independent event ledger is correctly required. |
| Entering-king helpers | Arithmetic uses the current side, camp/hand material, ten other camp pieces and check status. LAW_24/LAW_27 and Python defaults differ. Explicit rule selection is required; clock, declaration event and result procedure remain outside the helper. |
| `is_game_over()` | Exactly zero LegalAll moves; it does not establish repetition, declaration or a complete game result. |

YaneuraOu checkout `0f421391ae311df9df57dcfcb2b4a78a1972bdf8` and its
material-edition/configuration source support its role as a proposed USI
opponent. The all-moves search option is not treated as a complete-choice oracle.
The plan records GPL licensing, shared Apery/YaneuraOu lineage, option admission,
policy compatibility, missing build/handshake/match evidence and required
transcripts. Agreement between these engines is not represented as two fully
independent rules authorities. The [USI specification](https://shogidokoro2.stars.ne.jp/usi.html)
is appropriately used for transport syntax rather than tournament adjudication.

## Fixture and later-game assessment

Independently checked the concrete standard-start count, promotion boundaries,
capture/demotion, nifu, pawn-drop-mate/escape, pin and repetition examples.
SH-F001's 30 choices follow from blocked starting paths. SH-F015 excludes the
dropped-pawn capture and all king escapes; removing the 6a lance in SH-F016
opens a safe escape. SH-F028's rook cycle checks on every Sente action and
returns to the root after each four actions. SH-F032's gold covers 1b/2b and
knight covers 2a without checking 1a, so its proposed no-action case is sound
within the declared sparse-position domain. No false expected fact was found.
Complex attack, declaration-material, overlapping-history and capacity cases
remain explicitly pending construction/review. These are appropriate future
work, not invented executed evidence or known-reachable full games.

The later-game inventory separates adopted association rules, historical
holdings/facsimiles, modern reconstructions and software conventions. Primary
spot-checks confirm the [WXF English PDF](https://www.wxf-xiangqi.org/images/wxf-rules/2018_World_XiangQi_Rules_English2018.pdf)
has the recorded four-occurrence language at printed p.7 and threefold wording
at printed p.36; the plan correctly requires reconciliation before selecting
policy. The [JCSA text](https://www.chushogi-renmei.com/kouza/rule2007.htm)
is labeled 2019 and distinguishes royal capture/mate and prince treatment.
The inventory does not infer universal self-check rules from a missing check
announcement. Tori and tenjiku missing primary passages remain named acquisition
gaps. This bounded review does not certify every later-game translation or
historical lead. Their unresolved profiles do not block unaffected standard
shogi planning or authorize implementation shortcuts.

The remaining delivery gates—owner ratification, archived source identity,
independently reviewed concrete corpus, actual reference/opponent execution and
complete product integration—are retained explicitly. MSH-62 may complete as
planning without claiming that any of those later gates has passed.

## Final narrow consistency check

The latest imported-root proposal explicitly uses declared SFEN `n - 1` for
the length policy while keeping prior repetition/check history unknown.
SH-F041's phrase “not automatically a completed known-history counter” is
compatible with that distinction, but its “interpretation pending” qualifier
is stale. Recommended a small author wording alignment: name the proposed
declared counter, the `n = 320`/`n >= 321` and default/replay provenance cases,
and leave **fixture construction/review**, rather than proposed semantics,
pending. This is a nonmaterial documentation clarification; expectations remain
conditional on SHOGI-P1. It does not change the PASS verdict or require another
owner decision beyond that bundle. The reviewer changed no author document.
