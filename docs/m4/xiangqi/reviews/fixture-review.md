# MSH-91 independent expected-fixture review

Verdict: PASS for the frozen independent board corpus, with the explicit structural-domain handling below. This review precedes reference differential execution. No move generator or reference output was used to verify expected facts.

Reviewer: `/root/xiangqi_boundary_reviewer`, distinct from fixture author and native builder. Date: 2026-10-27. Inspected corpus SHA-256 `bf945517e4ea8a3627d2914d7206ab80f1394c571a30c29f65b3db9b84f63ad9`; `docs/m4/xiangqi/board-fixture-design.md`. Baseline source `53e90a93aa8954965cd69e336a9806ea5f59acaa`, working tree contains uncommitted task-scoped additions.

## Method and evidence

Read selected profile, AGENTS/design/task91–93 criteria, source reconciliation and original F001/F002 derivations. Directly visually inspected hash-pinned official WXF2018 English PDF pp11–13 (printed3–5): setup and Articles2.1–2.10. Read every original Red-facing position's concrete FEN, expected actions/exclusions, and reasoning; manually checked all Black-facing transcriptions using explicit 180-degree coordinate rotation, owner exchange, decreasing-rank Black forward and its home palace/river. Used a small read-only JSON shape check for unique IDs, sorted duplicate-free action lists, origin consistency and no include/exclude contradictions. It does not calculate moves.

- 105 fixtures, 101 restricted-reference eligible, four deliberate structural General-threat/capture cases.
- F001 exact44 checked member by member against setup: 4R+4H+4E+2A+1G+24C+5S. Both cannon capture endpoints require exactly the opposing cannon as screen before the opposing Horse. Black set is the explicit reversed-side counterpart.
- B01–B09 rays/screens; B10–B18 horse legs/elephant eyes/river; B19–B28 palaces/soldiers; B29–B34 pins/facing-General and cannon-screen discovery; B35–B41 complete/selected evasions; B42/B43 no-action with distinct check state; B44–B50 all six capturable families and noncapturable General: reviewed as the named source distinctions. `moves_from` is exhaustive only for that origin, `must_include`/`must_exclude` remains partial, as documented.
- Both General roots retained and their unrelated attacks checked in each manual position. Capturing a checking line attacker may give a new check, which is permitted; it does not invalidate an otherwise safe action.
- P001 independently hand-enumerated without source code: d0/f9 root has2 actions; after d0d1 Black has2 and afterd0e0 Black has1. Red third-ply reply counts are3,2,2. Therefore d1=2,d2=3,d3=7; divide2 `{d0d1:2,d0e0:1}`, divide3 `{d0d1:5,d0e0:2}`. This deliberately ignores Art3.2A and all history outcomes; it establishes only local enumeration.
- F002 C/Q four-ply cycles manually checked: all moves legal geometrically and respect General safety; same root after each, checked versus quiet Red moves. No later repetition outcome was imported into this board corpus.

## Findings resolved before freeze

The provisional B15 used a Red Soldier b1 and B09 a Black Soldier e7. Although useful blockers in unconstrained structural diagrams, they violate the reference's source-audited reachable-square domain. Reported before any differential run. Fixture author replaced them with Horses (and explicit rotated counterparts), leaving the movement facts unchanged. Frozen file includes these source-driven changes; no engine observation caused them.

## Required execution interpretation

B28/B50 and their Black counterparts already threaten the non-moving General; they are deliberately excluded from reference live admission. Native93 may reject the entire live-root query or expose a clearly labeled diagnostic-only action helper; it must never admit General capture. Their partial structural movement facts must not be mislabeled normal complete legal-action expectations if live admission rejects the diagram. Native test implementation should explicitly demonstrate both structural diagnosis and live rejection rather than silently skip them.

This review establishes independent expected facts and scoped fixture coverage, not executed native conformance, reference admission, whole-game completeness or playing strength. Later MSH-94/105 history/proof obligations remain unaffected.

## Subsequent discrepancy — original PASS superseded for B38 only

The first frozen-corpus reference run reported `e0f0` in B38 and `e9d9` in its Black counterpart beyond the exhaustive expected lists. The fixture author and this reviewer independently reexamined the concrete diagram against WXF2.1/2.5/2.9. B38's Black Horse on f2 itself separates Red General f0 from Black General f9; no facing-General violation occurs, and Horse f2 does not attack f0. Thus e0f0 is a legal evasion. In the rotated position the Red Horse d7 blocks d9–d0 and e9d9 is legal.

The original expected lists and this initial review both overlooked that intervening Horse. This is a rule/position-derived correction after a useful reported disagreement, not automatic acceptance of engine output. Preserve the original frozen corpus hash and mismatch report; amend only these two exhaustive lists and their reasoning, then renew independent review/freeze before rerunning. The original PASS above is superseded for these two facts until the corrected corpus is verified. All other reviewed facts remain applicable.

## Renewed freeze — corrected corpus PASS

Inspected replacement SHA-256 `a0724cf66703a63256efd77fe67313259ca8ce0062095155d85286d1f1b4ed4d`. B38 now lists exactly `a1f1,e0d0,e0e1,e0f0`; rotated counterpart `e9d9,e9e8,e9f9,i8d8`, with explicit Horse interposition reasoning. The diagrams and governing scope are unchanged. Renewed independent acceptance permits rerunning the reference comparison. The original discrepancy report/hash remains evidence of actual disagreement and source-driven correction; the earlier broad PASS was not treated as infallible authority.
